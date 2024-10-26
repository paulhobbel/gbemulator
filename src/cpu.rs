use log::warn;

#[derive(Debug)]
pub struct CPU;

impl CPU {
    pub fn step(&self) -> bool {
        warn!("CPU step not implemented");

        false
    }
}
