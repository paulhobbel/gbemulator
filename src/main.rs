use crate::cartridge::load_cartridge;
use crate::emulator::Emulator;
use std::env;
use std::process::exit;

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

    let rom_path = args[1].clone();
    let cartridge = load_cartridge(rom_path).expect("Failed to load cartridge");
    let mut emulator = Emulator::new(cartridge);

    emulator.run().expect("Emulator error");

    // load_cartridge("roms/Tetris.gb").expect("TODO: panic message");
    // load_cartridge("roms/Legend of Zelda, The - Link's Awakening (V1.2).gb")
    //     .expect("TODO: panic message");
}
