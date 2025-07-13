use crate::cartridge::Cartridge;
use crate::cpu::Cpu;
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

    let mut cpu = Cpu::new();

    // TODO this feels wrong, why does cpu load a rom. might need to add a motherboard/device type struct eventually
    cpu.load_rom_file("assets/dmg_boot.bin");
}
