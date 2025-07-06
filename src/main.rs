use crate::registers::{RegByte, Registers};

mod registers;

fn main() {
    println!("Hello, world!");
    println!("Hello from my-branch");
    
    let mut registers = Registers::new();
    registers.write_byte(RegByte::A, 0x1);
    println!("{}", registers.read_byte(RegByte::A));
}
