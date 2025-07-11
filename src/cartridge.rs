use std::fs;

#[derive(Debug)]
pub struct Cartridge {
    // TODO segment into rom/ram and banks
    pub bytes: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            bytes: vec![0; 0xFFFF],
        }
    }

    // TODO validate that the rom should start at 0x0000
    pub fn load_rom_file(&mut self, file_path: &str) {
        self.bytes = fs::read(file_path).expect("Expected to be able to read from file. ");
        self.print_all_bytes();
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.bytes[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.bytes[address] = value;
    }

    // just for testing/reading purposes
    pub fn print_all_bytes(&self) {
        let mut i = 1;
        for byte in self.bytes.iter() {
            println!("byte {i}: {byte}");
            i += 1;
        }
    }
}
