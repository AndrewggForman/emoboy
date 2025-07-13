use crate::cartridge::Cartridge;

const CARTRIDGE_ROM_BANK_0_START: usize = 0x0000;
const CARTRIDGE_ROM_BANK_0_END: usize = 0x3FFF;
const CARTRIDGE_ROM_BANK_N_START: usize = 0x4000;
const CARTRIDGE_ROM_BANK_N_END: usize = 0x7FFF;
const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const CARTRIDGE_RAM_START: usize = 0xA000;
const CARTRIDGE_RAM_END: usize = 0xBFFF;
const WRAM_START: usize = 0xC000;
const WRAM_END: usize = 0xDFFF;
const PROHIBITED_ECHO_RAM_START: usize = 0xE000;
const PROHIBITED_ECHO_RAM_END: usize = 0xFDFF;
const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const PROHIBITED_RAM_START: usize = 0xFEA0;
const PROHIBITED_RAM_END: usize = 0xFEFF;
const IO_REGISTERS_START: usize = 0xFF00;
const IO_REGISTERS_END: usize = 0xFF7F;
const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const IE_REGISTER: usize = 0xFFFF;

pub struct Memory {
    // TODO probably a better way to handle segments of an array
    _memory: Vec<u8>,

    cartridge: Cartridge,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            _memory: vec![0; 0xFFFF],
            cartridge: Cartridge::new(),
        }
    }

    pub fn load_rom_file(&mut self, file_path: &str) {
        self.cartridge.load_rom_file(file_path);
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        match address {
            CARTRIDGE_ROM_BANK_0_START..=CARTRIDGE_ROM_BANK_N_END => {
                self.cartridge.read_byte(address)
            }
            VRAM_START..=VRAM_END => self._memory[address],
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.read_byte(address),
            WRAM_START..=WRAM_END => self._memory[address],
            PROHIBITED_ECHO_RAM_START..=PROHIBITED_ECHO_RAM_END => panic!("PROHIBITED_ECHO_RAM"),
            OAM_START..=OAM_END => self._memory[address],
            PROHIBITED_RAM_START..=PROHIBITED_RAM_END => panic!("PROHIBITED_RAM"),
            IO_REGISTERS_START..=IO_REGISTERS_END => self._memory[address],
            HRAM_START..=HRAM_END => self._memory[address],
            IE_REGISTER => self._memory[address],
            _ => panic!("Unexpected Memory::read_byte {}", address),
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        match address {
            CARTRIDGE_ROM_BANK_0_START..=CARTRIDGE_ROM_BANK_N_END => {
                self.cartridge.write_byte(address, value)
            }
            VRAM_START..=VRAM_END => self._memory[address] = value,
            CARTRIDGE_RAM_START..=CARTRIDGE_RAM_END => self.cartridge.write_byte(address, value),
            WRAM_START..=WRAM_END => self._memory[address] = value,
            PROHIBITED_ECHO_RAM_START..=PROHIBITED_ECHO_RAM_END => panic!("PROHIBITED_ECHO_RAM"),
            OAM_START..=OAM_END => self._memory[address] = value,
            PROHIBITED_RAM_START..=PROHIBITED_RAM_END => panic!("PROHIBITED_RAM"),
            IO_REGISTERS_START..=IO_REGISTERS_END => self._memory[address] = value,
            HRAM_START..=HRAM_END => self._memory[address] = value,
            IE_REGISTER => self._memory[address] = value,
            _ => panic!("Unexpected Memory::write_byte {} {}", address, value),
        };
    }
}
