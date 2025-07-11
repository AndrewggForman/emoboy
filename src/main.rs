use crate::{cpu::Cpu, registers::{RegByte, Registers}};
use crate::cartridge::{Cartridge};

mod registers;
mod cpu;
mod clock;
mod cartridge;
mod opcode;

fn main() {
    println!("Hello, world!");
    
    // get a cartridge and pass it into the cpu constructor

    let cpu = Cpu::new();
    let cartridge = Cartridge::new("dmg_boot.bin");
    //println!("rom_reader: {rom_reader:#?}");
    cartridge.print_all_bytes();
}
