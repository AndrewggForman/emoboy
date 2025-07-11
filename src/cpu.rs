use crate::memory::Memory;
use crate::registers::Registers;
use crate::instructions::Instruction;

pub struct Cpu {
    registers: Registers,
    memory: Memory,
    // graphics
    // sound
    // timers
    // inputs

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
        }
    }

    pub fn step() {
        // get next instruction
        // execute instruction
    }

    pub fn get_next_instruction() {
        // read pc
        // let instruction = Instruction::create(pc)
    }

    pub fn execute_instruction(instruction: Instruction) {
        // match instruction
    }
}