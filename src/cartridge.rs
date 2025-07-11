use std::fs;
use std::io::{self, BufReader, Read};

use crate::registers::{RegWord, Registers};

#[derive(Debug)]
pub struct Cartridge {
    pub bytes: Vec<u8>,
}

impl Cartridge {
    pub fn new(file_path: &str) -> Self {
        Self {
            bytes: fs::read(file_path)
            .expect("Expected to be able to read from file. "),
        }
    }

    // pub fn read_byte(self, registers: Registers) -> u8 {
    //     self.bytes[registers.read_word(RegWord::PC) as usize]
    // }

    // just for testing/reading purposes
    pub fn print_all_bytes(self) {
        let mut i = 1;
        for byte in self.bytes {
            println!("byte {i}: {byte}");
            i +=1;
        }   
    }
}