#[cfg(test)]
mod mem_tests {
    use crate::memory::*;
    const CARTRIDGE_ROM_BANK_0_START: u16 = 0x0000;
    const CARTRIDGE_ROM_BANK_0_END: u16 = 0x3FFF;
    const CARTRIDGE_ROM_BANK_N_START: u16 = 0x4000;
    const CARTRIDGE_ROM_BANK_N_END: u16 = 0x7FFF;
    const VRAM_START: u16 = 0x8000;
    const VRAM_END: u16 = 0x9FFF;
    const CARTRIDGE_RAM_START: u16 = 0xA000;
    const CARTRIDGE_RAM_END: u16 = 0xBFFF;
    const WRAM_START: u16 = 0xC000;
    const WRAM_END: u16 = 0xDFFF;
    const PROHIBITED_ECHO_RAM_START: u16 = 0xE000;
    const PROHIBITED_ECHO_RAM_END: u16 = 0xFDFF;
    const OAM_START: u16 = 0xFE00;
    const OAM_END: u16 = 0xFE9F;
    const PROHIBITED_RAM_START: u16 = 0xFEA0;
    const PROHIBITED_RAM_END: u16 = 0xFEFF;
    const IO_REGISTERS_START: u16 = 0xFF00;
    const IO_REGISTERS_END: u16 = 0xFF7F;
    const HRAM_START: u16 = 0xFF80;
    const HRAM_END: u16 = 0xFFFE;
    const IE_REGISTER: u16 = 0xFFFF;

    #[test]
    fn number_sanity_check() {
        /* Sanity test for ranges, e.g. from a to b doesn't include b*/
        let mut memory = Memory::new();

        memory.write_byte(CARTRIDGE_ROM_BANK_0_END, 1);

        for address in CARTRIDGE_ROM_BANK_0_START..CARTRIDGE_ROM_BANK_0_END {
            assert_eq!(memory.read_byte(address), 0);
        }

        assert_eq!(memory.read_byte(CARTRIDGE_ROM_BANK_0_END), 1);
    }

    #[test]
    fn initialize_memory() {
        let memory = Memory::new();

        for address in CARTRIDGE_ROM_BANK_0_START..(CARTRIDGE_ROM_BANK_0_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in CARTRIDGE_ROM_BANK_N_START..(CARTRIDGE_ROM_BANK_N_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in VRAM_START..(VRAM_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in CARTRIDGE_RAM_START..(CARTRIDGE_RAM_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in WRAM_START..(WRAM_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in OAM_START..(OAM_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in IO_REGISTERS_START..(IO_REGISTERS_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        for address in HRAM_START..(HRAM_END + 1) {
            assert_eq!(memory.read_byte(address), 0);
        }

        assert_eq!(memory.read_byte(IE_REGISTER), 0);

        //PROHIBITED_ECHO_RAM_START: u16 = 0xE000;
        //PROHIBITED_ECHO_RAM_END: u16 = 0xFDFF;
        //PROHIBITED_RAM_START: u16 = 0xFEA0;
        //PROHIBITED_RAM_END: u16 = 0xFEFF;
    }

    #[test]
    #[should_panic]
    fn reading_prohibited_echo_ram_start() {
        let memory = Memory::new();
        assert_eq!(memory.read_byte(PROHIBITED_ECHO_RAM_START), 0);
    }

    #[test]
    #[should_panic]
    fn reading_prohibited_ram_start() {
        let memory = Memory::new();
        assert_eq!(memory.read_byte(PROHIBITED_RAM_START), 0);
    }

    #[test]
    #[should_panic]
    fn reading_prohibited_echo_ram_end() {
        let memory = Memory::new();
        assert_eq!(memory.read_byte(PROHIBITED_ECHO_RAM_END), 0);
    }

    #[test]
    #[should_panic]
    fn reading_prohibited_ram_end() {
        let memory = Memory::new();
        assert_eq!(memory.read_byte(PROHIBITED_RAM_END), 0);
    }

    #[test]
    #[should_panic]
    fn reading_prohibited_echo_ram_middle() {
        let memory = Memory::new();
        assert_eq!(memory.read_byte(PROHIBITED_ECHO_RAM_START + 10), 0);
    }

    #[test]
    #[should_panic]
    fn reading_prohibited_ram_middle() {
        let memory = Memory::new();
        assert_eq!(memory.read_byte(PROHIBITED_RAM_START + 10), 0);
    }

    #[test]
    #[should_panic]
    fn writing_prohibited_echo_ram_start() {
        let mut memory = Memory::new();
        memory.write_byte(PROHIBITED_ECHO_RAM_START, 22);
    }

    #[test]
    #[should_panic]
    fn writing_prohibited_ram_start() {
        let mut memory = Memory::new();
        memory.write_byte(PROHIBITED_RAM_START, 22);
    }

    #[test]
    #[should_panic]
    fn writing_prohibited_echo_ram_end() {
        let mut memory = Memory::new();
        memory.write_byte(PROHIBITED_ECHO_RAM_END, 22);
    }

    #[test]
    #[should_panic]
    fn writing_prohibited_ram_end() {
        let mut memory = Memory::new();
        memory.write_byte(PROHIBITED_RAM_END, 22);
    }

    #[test]
    #[should_panic]
    fn writing_prohibited_echo_ram_middle() {
        let mut memory = Memory::new();
        memory.write_byte(PROHIBITED_ECHO_RAM_START + 10, 22);
    }

    #[test]
    #[should_panic]
    fn writing_prohibited_ram_middle() {
        let mut memory = Memory::new();
        memory.write_byte(PROHIBITED_RAM_START + 10, 22);
    }
}
