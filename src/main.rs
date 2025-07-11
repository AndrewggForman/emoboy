use crate::cartridge::Cartridge;
use crate::{
    cpu::Cpu,
    registers::{RegByte, Registers},
};

mod cartridge;
mod clock;
mod cpu;
mod memory;
mod opcode;
mod registers;

fn main() {
    println!("Hello, world!");

    // get a cartridge and pass it into the cpu constructor

    let cpu = Cpu::new();
    let cartridge = Cartridge::new("dmg_boot.bin");
    //println!("rom_reader: {rom_reader:#?}");
    cartridge.print_all_bytes();
}
