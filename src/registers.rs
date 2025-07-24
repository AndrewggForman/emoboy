#[derive(PartialEq, Eq)]
pub enum RegByte {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
}

pub enum RegWord {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

pub enum RegFlag {
    Zero = 0x80,        // 0b1000_0000
    Subtraction = 0x40, // 0b0100_0000
    HalfCarry = 0x20,   // 0b0010_0000
    Carry = 0x10,       // 0b0001_0000
}

pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Registers {
    pub fn new() -> Self {
        // TODO check if these should be initialized to certain values
        Self {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: 0x00,
            h: 0x00,
            l: 0x00,
            sp: 0x0000,
            pc: 0x0000,
        }
    }

    pub fn read_byte(&self, register: &RegByte) -> u8 {
        match register {
            RegByte::A => self.a,
            RegByte::B => self.b,
            RegByte::C => self.c,
            RegByte::D => self.d,
            RegByte::E => self.e,
            RegByte::F => self.f,
            RegByte::H => self.h,
            RegByte::L => self.l,
        }
    }

    pub fn write_byte(&mut self, register: &RegByte, value: u8) {
        match register {
            RegByte::A => self.a = value,
            RegByte::B => self.b = value,
            RegByte::C => self.c = value,
            RegByte::D => self.d = value,
            RegByte::E => self.e = value,
            RegByte::F => self.f = value & 0xF0, // only top half used in f
            RegByte::H => self.h = value,
            RegByte::L => self.l = value,
        }
    }

    pub fn read_word(&self, register: &RegWord) -> u16 {
        match register {
            RegWord::AF => u16::from_be_bytes([self.a, self.f]),
            RegWord::BC => u16::from_be_bytes([self.b, self.c]),
            RegWord::DE => u16::from_be_bytes([self.d, self.e]),
            RegWord::HL => u16::from_be_bytes([self.h, self.l]),
            RegWord::SP => self.sp,
            RegWord::PC => self.pc,
        }
    }

    pub fn write_word(&mut self, register: &RegWord, value: u16) {
        let bytes = value.to_be_bytes();

        match register {
            RegWord::AF => {
                self.a = bytes[0];
                self.f = bytes[1] & 0xF0; // only top half used in f
            }
            RegWord::BC => {
                self.b = bytes[0];
                self.c = bytes[1];
            }
            RegWord::DE => {
                self.d = bytes[0];
                self.e = bytes[1];
            }
            RegWord::HL => {
                self.h = bytes[0];
                self.l = bytes[1];
            }
            RegWord::SP => self.sp = value,
            RegWord::PC => self.pc = value,
        }
    }

    // TODO - Not sure how to change this to a reference to RegFlag w/o breaking?
    // > Seems like we can't cast as u8 if we reference it/deference with raw pointer
    // > and I don't get how the read_flag even works
    pub fn read_flag(&self, register_flag: RegFlag) -> bool {
        self.f & (register_flag as u8) > 0
    }

    // TODO not sure if should be `set: bool`, `value: int` ...etc
    pub fn write_flag(&mut self, register_flag: RegFlag, value: bool) {
        if value {
            let mask = register_flag as u8;
            self.f = self.f | mask;
        } else {
            let mask = !(register_flag as u8);
            self.f = self.f & mask;
        }
    }

    // Move to OpCode, rename to get_carry_and_update_flag
    pub fn add_carry(&mut self) -> u8 {
        let carry = self.read_flag(RegFlag::Carry);
        if carry {
            // Update carry flag has been used
            self.write_flag(RegFlag::Carry, false);
            return 1;
        }
        0
    }

    // Should be called everytime we do something like fetch_byte(),
    pub fn increment_pc(&mut self) {
        self.pc += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_byte_registers() {
        // not much point to this test since read logic is so simple
        let mut registers = Registers::new();
        registers.a = 0x1;
        registers.f = 0x2;
        registers.b = 0x3;
        registers.c = 0x4;
        registers.d = 0x5;
        registers.e = 0x6;
        registers.h = 0x7;
        registers.l = 0x8;
        // TODO TALK TO TINT about &RegByte
        assert_eq!(registers.read_byte(&RegByte::A), 0x1);
        assert_eq!(registers.read_byte(&RegByte::F), 0x2);
        assert_eq!(registers.read_byte(&RegByte::B), 0x3);
        assert_eq!(registers.read_byte(&RegByte::C), 0x4);
        assert_eq!(registers.read_byte(&RegByte::D), 0x5);
        assert_eq!(registers.read_byte(&RegByte::E), 0x6);
        assert_eq!(registers.read_byte(&RegByte::H), 0x7);
        assert_eq!(registers.read_byte(&RegByte::L), 0x8);
    }

    #[test]
    fn write_byte_registers() {
        // not much point to this test since read logic is so simple
        let mut registers = Registers::new();
        registers.write_byte(&RegByte::A, 0x1);
        registers.write_byte(&RegByte::F, 0x22);
        registers.write_byte(&RegByte::B, 0x3);
        registers.write_byte(&RegByte::C, 0x4);
        registers.write_byte(&RegByte::D, 0x5);
        registers.write_byte(&RegByte::E, 0x6);
        registers.write_byte(&RegByte::H, 0x7);
        registers.write_byte(&RegByte::L, 0x8);
        assert_eq!(registers.a, 0x1);
        assert_eq!(registers.f, 0x20); // lower half should be zeroed out 0x22 -> 0x20
        assert_eq!(registers.b, 0x3);
        assert_eq!(registers.c, 0x4);
        assert_eq!(registers.d, 0x5);
        assert_eq!(registers.e, 0x6);
        assert_eq!(registers.h, 0x7);
        assert_eq!(registers.l, 0x8);
    }

    #[test]
    fn read_word_registers() {
        let mut registers = Registers::new();
        registers.a = 0x1;
        registers.f = 0x2;
        registers.b = 0x3;
        registers.c = 0x4;
        registers.d = 0x5;
        registers.e = 0x6;
        registers.h = 0x7;
        registers.l = 0x8;
        assert_eq!(registers.read_word(&RegWord::AF), 0x102);
        assert_eq!(registers.read_word(&RegWord::BC), 0x304);
        assert_eq!(registers.read_word(&RegWord::DE), 0x506);
        assert_eq!(registers.read_word(&RegWord::HL), 0x708);

        // not much point to these tests
        registers.sp = 0x111;
        registers.pc = 0x222;
        assert_eq!(registers.read_word(&RegWord::SP), 0x111);
        assert_eq!(registers.read_word(&RegWord::PC), 0x222);
    }

    #[test]
    fn write_word_registers() {
        let mut registers = Registers::new();

        registers.write_word(&RegWord::AF, 0x120);
        registers.write_word(&RegWord::BC, 0x304);
        registers.write_word(&RegWord::DE, 0x506);
        registers.write_word(&RegWord::HL, 0x708);

        assert_eq!(registers.a, 0x1);
        assert_eq!(registers.f, 0x20);
        assert_eq!(registers.b, 0x3);
        assert_eq!(registers.c, 0x4);
        assert_eq!(registers.d, 0x5);
        assert_eq!(registers.e, 0x6);
        assert_eq!(registers.h, 0x7);
        assert_eq!(registers.l, 0x8);

        // not much point to these tests
        registers.write_word(&RegWord::SP, 0x111);
        registers.write_word(&RegWord::PC, 0x222);
        registers.sp = 0x111;
        registers.pc = 0x222;
    }

    #[test]
    fn read_flag_registers() {
        let mut registers = Registers::new();

        registers.f = 0xF0; // 0b1111_0000
        assert_eq!(registers.read_flag(RegFlag::Zero), true);
        assert_eq!(registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(registers.read_flag(RegFlag::Carry), true);

        registers.f = 0x0; // 0b0000_0000
        assert_eq!(registers.read_flag(RegFlag::Zero), false);
        assert_eq!(registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn write_flag_registers() {
        let mut registers = Registers::new();
        registers.f = 0x0; // 0b0000_0000

        // setting
        registers.write_flag(RegFlag::Zero, true);
        assert_eq!(registers.f, 0x80); // 0b1000_0000

        registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(registers.f, 0xC0); // 0b1100_0000

        registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(registers.f, 0xE0); // 0b1110_0000

        registers.write_flag(RegFlag::Carry, true);
        assert_eq!(registers.f, 0xF0); // 0b1111_0000

        // clearing
        registers.write_flag(RegFlag::Zero, false);
        assert_eq!(registers.f, 0x70); // 0b0111_0000

        registers.write_flag(RegFlag::Subtraction, false);
        assert_eq!(registers.f, 0x30); // 0b0011_0000

        registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(registers.f, 0x10); // 0b0001_0000

        registers.write_flag(RegFlag::Carry, false);
        assert_eq!(registers.f, 0x0); // 0b0000_0000
    }
}
