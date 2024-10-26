use crate::cartridge::Cartridge;
use crate::cpu::CPU;
use log::info;
use std::thread::sleep;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug)]
pub struct Emulator {
    cartridge: Cartridge,
    cpu: CPU,
    running: bool,
    paused: bool,
    ticks: u64,
}

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error("CPU error: {0}")]
    CPUError(&'static str),
}

impl Emulator {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            cpu: CPU,
            running: false,
            paused: false,
            ticks: 0,
        }
    }

    pub fn run(&mut self) -> Result<(), EmulatorError> {
        info!("Starting emulator with cartridge:\n{}", self.cartridge);

        self.running = true;
        self.paused = false;
        self.ticks = 0;

        while self.running {
            if self.paused {
                sleep(Duration::from_millis(10));
                continue;
            }

            if !self.cpu.step() {
                return Err(EmulatorError::CPUError("CPU stopped"));
            }

            self.ticks += 1;
        }

        Ok(())
    }
}
