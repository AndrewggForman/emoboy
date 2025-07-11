pub struct Memory {
    // in the future, split this into the different regions and named better
    ram: Vec<u8>,
    // cartridge
}

impl Memory {
    pub fn new() -> Self {
        Self {
            ram: vec![0; 0xFFFF],
        }
    }
}
