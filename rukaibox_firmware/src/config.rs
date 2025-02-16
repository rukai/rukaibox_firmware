use rkyv::{rancor::Failure, util::Align};
use rukaibox_config::{ArchivedConfig, CONFIG_OFFSET, CONFIG_SIZE, RP2040_FLASH_OFFSET};

// TODO: store in heap instead, apparently only 2kb of stack o.0
pub struct Config {
    pub bytes: Align<[u8; CONFIG_SIZE]>,
}

impl Config {
    pub fn load() -> Config {
        let bytes = load_config_bytes_from_flash();
        Config { bytes }
    }

    pub fn parse(&self) -> Result<&ArchivedConfig, Failure> {
        let size = u32::from_be_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]])
            as usize;
        rkyv::api::low::access::<ArchivedConfig, Failure>(&(&*self.bytes)[4..4 + size])
    }
}

fn load_config_bytes_from_flash() -> Align<[u8; CONFIG_SIZE]> {
    let mut data = Align([0; CONFIG_SIZE]);
    // Safety: This byte range is known to be valid flash memory on this device
    unsafe {
        for i in 0..CONFIG_SIZE {
            let address = (RP2040_FLASH_OFFSET + CONFIG_OFFSET + i) as *mut u8;
            data[i] = core::ptr::read_volatile(address);
        }
    }
    data
}
