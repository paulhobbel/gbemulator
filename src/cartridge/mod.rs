use crate::cartridge::header::{load_header, CartridgeHeader};
use crate::cartridge::mbc1::MBC1;
use crate::cartridge::rom_only::RomOnly;
use log::debug;
use std::fmt::{Debug, Display};
use std::{fs, path};
use thiserror::Error;

mod header;
mod mbc1;
mod rom_only;

const HEADER_SIZE: usize = 0x150;

pub trait Cartridge: Send + Debug {
    fn read_rom(&self, address: u16) -> u8;
    fn read_ram(&self, address: u16) -> u8;
    fn get_header(&self) -> CartridgeHeader;
    fn has_battery(&self) -> bool;
}

impl Display for dyn Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let header = self.get_header();

        write!(f, "Title: {}\n", header.get_title())?;
        write!(
            f,
            "Type: 0x{:02X} ({})\n",
            header.cartridge_type,
            header.get_cartridge_type()
        )?;
        write!(
            f,
            "Entry: 0x{:02X}{:02X}{:02X}{:02X}\n",
            header.entry[0], header.entry[1], header.entry[2], header.entry[3]
        )?;
        write!(
            f,
            "ROM size: 0x{:02X} ({}KB)\n",
            header.rom_size,
            32 << header.rom_size
        )?;
        write!(
            f,
            "RAM size: 0x{:02X} ({})\n",
            header.ram_size,
            header.get_ram_size()
        )?;
        write!(f, "Licensee: {}\n", header.get_licensee_name())?;
        write!(f, "ROM version: {}\n", header.rom_version)?;
        write!(f, "Header checksum: 0x{:02X}\n", header.header_checksum)?;
        write!(f, "Has battery: {}\n", self.has_battery())
    }
}
#[derive(Debug, Error)]
pub enum Error {
    #[error("Read error: {0}")]
    Read(#[from] std::io::Error),
    #[error("Unsupported cartridge type: {0}")]
    UnsupportedCartridgeType(u8),
    #[error("Invalid ROM header")]
    InvalidHeader,
    #[error("Invalid checksum, expected: {0}, got: {0}")]
    InvalidChecksum(u8, u8),
}

pub fn load_cartridge(rom_path: path::PathBuf) -> Result<Box<dyn Cartridge>, Error> {
    debug!("Loading ROM from file: {:?}", rom_path.to_str());
    let rom = fs::read(rom_path)?;

    if rom.len() < HEADER_SIZE {
        return Err(Error::InvalidHeader);
    }

    let header = load_header(&rom);

    let calculated_checksum = calculate_checksum(&rom);
    debug!(
        "Calculated checksum: {:02X}, expected: {:02X}",
        calculated_checksum, header.header_checksum
    );
    if header.header_checksum != calculated_checksum {
        return Err(Error::InvalidChecksum(
            header.header_checksum,
            calculated_checksum,
        ));
    }

    match header.cartridge_type {
        0x00 => Ok(Box::new(RomOnly::new(header, rom))),
        0x01 => Ok(Box::new(MBC1::new(header, rom))),
        _ => Err(Error::UnsupportedCartridgeType(header.cartridge_type)),
    }
}

fn calculate_checksum(rom: &Vec<u8>) -> u8 {
    let mut calculated: u8 = 0;
    for i in 0x134..0x14D {
        calculated = calculated.wrapping_sub(rom[i]).wrapping_sub(1);
    }

    calculated
}

fn rom_banks(v: u8) -> usize {
    2 << v
}

fn ram_banks(v: u8) -> usize {
    match v {
        0x02 => 1,
        0x03 => 4,
        0x04 => 16,
        0x05 => 8,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_load_cartridge() {
        let cartridge = load_cartridge(PathBuf::from("roms/cpu_instrs.gb")).unwrap();
        let header = cartridge.get_header();

        assert_eq!(header.entry, [0x00, 0xC3, 0x37, 0x06]);
        assert_eq!(header.get_title(), "CPU_INSTRS".to_string());
        assert_eq!(header.get_cartridge_type(), "MBC1");
        assert_eq!(header.rom_size, 0x01);
        assert_eq!(header.get_ram_size(), "None");
        assert_eq!(header.get_licensee_name(), "None");
        assert_eq!(header.rom_version, 0x00);
        assert_eq!(header.header_checksum, 0x3B);
    }
}
