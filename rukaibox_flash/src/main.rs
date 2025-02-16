use miette::{Result, miette};
use rkyv::rancor::Error;

pub mod config;
pub mod elf;
pub mod flash;

fn main() -> Result<()> {
    let config = config::load()?;
    let config = rkyv::to_bytes::<Error>(&config).map_err(|e| miette!(e))?;

    let firmware = elf::elf_to_bin(include_bytes!(env!(
        "CARGO_BIN_FILE_RUKAIBOX_FIRMWARE_rukaibox_firmware"
    )))?;

    flash::flash_device(&firmware, &config)?;

    println!("Succesfully flashed!");
    Ok(())
}
