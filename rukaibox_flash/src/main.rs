use goblin::elf::program_header::PT_LOAD;
use miette::{IntoDiagnostic, Result};
use picoboot_rs::{
    PICO_FLASH_START, PICO_PAGE_SIZE, PICO_SECTOR_SIZE, PICO_STACK_POINTER, PicobootConnection,
    TargetID,
};
use rusb::Context;

mod config;

fn bin_pages(fw: &[u8]) -> Vec<Vec<u8>> {
    let mut fw_pages: Vec<Vec<u8>> = vec![];
    let len = fw.len();

    // splits the binary into sequential pages
    for i in (0..len).step_by(PICO_PAGE_SIZE as usize) {
        let size = std::cmp::min(len - i, PICO_PAGE_SIZE as usize);
        let mut page = fw[i..i + size].to_vec();
        page.resize(PICO_PAGE_SIZE as usize, 0);
        fw_pages.push(page);
    }

    fw_pages
}

fn main() -> Result<()> {
    let config = config::load()?;
    println!("{config:#?}");

    match Context::new() {
        Ok(ctx) => {
            // create connection object
            let mut conn = PicobootConnection::new(ctx, None)
                .expect("failed to connect to PICOBOOT interface");

            conn.reset_interface().expect("failed to reset interface");
            conn.access_exclusive_eject()
                .expect("failed to claim access");
            conn.exit_xip().expect("failed to exit from xip mode");

            // firmware in a big vector of u8's
            let fw_pages = bin_pages(
                &elf_to_bin(include_bytes!(env!(
                    "CARGO_BIN_FILE_RUKAIBOX_FIRMWARE_rukaibox_firmware"
                )))
                .unwrap(),
            );
            // erase space on flash
            for (i, _) in fw_pages.iter().enumerate() {
                let addr = (i as u32) * PICO_PAGE_SIZE + PICO_FLASH_START;
                if (addr % PICO_SECTOR_SIZE) == 0 {
                    conn.flash_erase(addr, PICO_SECTOR_SIZE)
                        .expect("failed to erase flash");
                }
            }

            for (i, page) in fw_pages.iter().enumerate() {
                let addr = (i as u32) * PICO_PAGE_SIZE + PICO_FLASH_START;

                // write page to flash
                conn.flash_write(addr, page).expect("failed to write flash");

                // confirm flash write was successful
                let read = conn
                    .flash_read(addr, PICO_PAGE_SIZE)
                    .expect("failed to read flash");
                let matching = page.iter().zip(&read).all(|(&a, &b)| a == b);
                assert!(matching, "page does not match flash");
            }

            // reboot device to start firmware
            let delay = 500; // in milliseconds
            match conn.get_device_type() {
                TargetID::Rp2040 => {
                    conn.reboot(0x0, PICO_STACK_POINTER, delay)
                        .expect("failed to reboot device");
                }
                TargetID::Rp2350 => conn.reboot2_normal(delay).expect("failed to reboot device"),
            }
        }
        Err(e) => panic!("Could not initialize libusb: {}", e),
    }

    println!("Succesfully flashed!");
    Ok(())
}

pub fn elf_to_bin(bytes: &[u8]) -> Result<Vec<u8>> {
    let mut binary = goblin::elf::Elf::parse(bytes).into_diagnostic()?;
    binary.program_headers.sort_by_key(|x| x.p_paddr);

    let mut last_address: u64 = 0;

    let mut data = vec![];
    for (i, ph) in binary
        .program_headers
        .iter()
        .filter(|ph| {
            ph.p_type == PT_LOAD
                && ph.p_filesz > 0
                && ph.p_offset >= binary.header.e_ehsize as u64
                && ph.is_read()
        })
        .enumerate()
    {
        // on subsequent passes, if there's a gap between this section and the
        // previous one, fill it with zeros
        if i != 0 {
            let difference = (ph.p_paddr - last_address) as usize;
            data.resize(data.len() + difference, 0x0);
        }

        data.extend_from_slice(&bytes[ph.p_offset as usize..][..ph.p_filesz as usize]);

        last_address = ph.p_paddr + ph.p_filesz;
    }

    Ok(data)
}
