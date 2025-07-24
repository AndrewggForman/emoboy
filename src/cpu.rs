use crate::memory::Memory;
use crate::{
    cartridge::Cartridge,
    clock::Clock,
    registers::{RegWord, Registers},
};

pub struct Cpu {
    pub registers: Registers,
    pub memory: Memory, // TODO-Talk with tint, was memory supposed to be private? I don't know how else to access it in op-codes to do HL commands
    pub clock: Clock,
    // graphics
    // sound
    // timers
    // inputs
    // IME flag

    // not exactly sure if it makes sense that a CPU has things like a graphic controller, or sound controller
    // but i think it works for a v1. We could always move stuff around down the line, perhaps with a
    // bus type structure that connects the different components organized within a device struct?

    // it makes sense that a cpu should be able to step in some way, but similar to above: not sure
    // if the cpu should be coordinating the steps for the other components or not. but i think the
    // it can also be pushed down the line a bit till more functionality is fleshed out
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            clock: Clock::new(),
        }
    }

    pub fn fetch_next_byte(&mut self, cartridge: &Cartridge) -> u8 {
        let next_byte = cartridge.bytes[self.registers.read_word(&RegWord::PC) as usize];
        self.registers.increment_pc();
        next_byte
    }

    pub fn load_rom_file(&mut self, file_path: &str) {
        self.memory.load_rom_file(file_path);
    }
}
