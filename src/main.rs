use crate::{cpu::Cpu, registers::{RegByte, Registers}};

mod registers;
mod cpu;
mod instructions;

fn main() {
    println!("Hello, world!");
    
    // get a cartridge and pass it into the cpu constructor

    let cpu = Cpu::new();
}
