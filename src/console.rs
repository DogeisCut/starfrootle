use crate::{bus::Bus, cpu::Cpu};


pub const CYCLES_PER_FRAME: u64 = 240_000;

pub struct Console {
    pub cpu: Cpu,
    pub bus: Bus,
}
impl Console {
    pub fn reset() {

    }
    pub fn run_frame(&mut self) {
        let mut spent = 0;
        while spent < CYCLES_PER_FRAME && !self.cpu.waiting {
            spent += self.cpu.step(&mut self.bus);
        }
        self.cpu.waiting = false;
        self.render();
    }
    pub fn render(&mut self) {
        
    }
}