use crate::cartridge::load_cartridge;
use crate::emulator::Emulator;
use std::process::exit;
use std::{env, path};

mod cartridge;
mod cpu;
mod emulator;

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <ROM file>", args[0]);
        exit(1);
    }

    let rom_path = path::PathBuf::from(args[1].clone());
    let cartridge = load_cartridge(rom_path).expect("Failed to load cartridge");
    let mut emulator = Emulator::new(cartridge);

    emulator.run().expect("Emulator error");
}
