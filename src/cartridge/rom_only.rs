use crate::cartridge::header::CartridgeHeader;
use crate::cartridge::Cartridge;

#[derive(Debug)]
pub struct RomOnly {
    header: CartridgeHeader,
    rom: Vec<u8>,
}

impl RomOnly {
    pub fn new(header: CartridgeHeader, rom: Vec<u8>) -> Self {
        Self { header, rom }
    }
}

impl Cartridge for RomOnly {
    fn read_rom(&self, address: u16) -> u8 {
        self.rom[address as usize]
    }

    fn read_ram(&self, _: u16) -> u8 {
        0
    }

    fn get_header(&self) -> CartridgeHeader {
        self.header
    }

    fn has_battery(&self) -> bool {
        false
    }
}
