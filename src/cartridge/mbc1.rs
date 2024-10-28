use crate::cartridge::header::CartridgeHeader;
use crate::cartridge::{ram_banks, rom_banks, Cartridge};
use std::fmt::Debug;

#[derive(Debug)]
pub struct MBC1 {
    header: CartridgeHeader,
    rom: Vec<u8>,
    ram: Vec<u8>,
    rom_banks: usize,
    ram_banks: usize,
    has_battery: bool,
}

impl MBC1 {
    pub fn new(header: CartridgeHeader, rom: Vec<u8>) -> Self {
        let rom_banks = rom_banks(header.rom_size);
        let (has_battery, ram_banks) = match header.cartridge_type {
            0x02 => (false, ram_banks(header.ram_size)),
            0x03 => (true, ram_banks(header.ram_size)),
            _ => (false, 0),
        };

        let ram_size = ram_banks * 0x2000;

        Self {
            header,
            rom,
            ram: vec![0; ram_size],
            rom_banks,
            ram_banks,
            has_battery,
        }
    }
}

impl Cartridge for MBC1 {
    fn read_rom(&self, address: u16) -> u8 {
        // TODO: Implement bank switching
        if address < 0x4000 {
            self.rom[address as usize]
        } else {
            panic!("TODO: Implement bank switching");
        }
    }

    fn read_ram(&self, _: u16) -> u8 {
        // TODO: Implement bank switching
        0xFF
    }

    fn get_header(&self) -> CartridgeHeader {
        self.header
    }

    fn has_battery(&self) -> bool {
        self.has_battery
    }
}
