use crate::cartridge::Cartridge;
use crate::registers::{RegByte, Registers};

mod cartridge;
mod clock;
mod cpu;
mod cpu_logic;
mod gpu;
mod memory;
mod motherboard;
mod opcode;
mod opcode_tests;
mod registers;

fn main() {
    println!("Hello, world!");

    // get a cartridge and pass it into the motherboard constructor

    let mut motherboard = motherboard::Motherboard::new();

    // TODO this feels wrong, why does motherboard load a rom. might need to add a motherboard/device type struct eventually
    motherboard.load_rom_file("assets/andy_test_rom.bin");

    // Experimental/testing purposes
    let mut cart = Cartridge::new();
    cart.load_rom_file("assets/andy_test_rom.bin");

    println!("ABOVE IS CARTRIDGE ----- BELOW IS MEMORY IN motherboard");

    let byte0 = motherboard.memory.read_byte(0);
    let byte1 = motherboard.memory.read_byte(1);
    let byte2 = motherboard.memory.read_byte(2);
    let byte3 = motherboard.memory.read_byte(3);
    let byte4 = motherboard.memory.read_byte(4);
    println!("Byte0: {}", byte0);
    println!("Byte1: {}", byte1);
    println!("Byte2: {}", byte2);
    println!("Byte3: {}", byte3);
    println!("Byte4: {}", byte4);
}
