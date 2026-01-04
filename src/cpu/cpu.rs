use crate::cpu::CpuMemory;

pub struct Cpu {
    pc: usize,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            pc: 0xC000,
        }
    }

    pub fn next_instruction(&mut self) {

    }
}
