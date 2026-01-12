#[cfg(test)]
mod tests {
    use crate::{cpu_logic::*, motherboard, opcode::*, registers::*};

    use super::*;

    // Testing RLCA (rotate register A to the left and set carry bit as well)
    #[test]
    fn rotate_register_a_left_producing_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1010_1010);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1010_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_left_without_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0010_1010);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0010_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn chain_rotate_register_a_left() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0010_1010);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0010_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1010_1000);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing RRCA (rotate register A to the right and set carry bit as well)
    #[test]
    fn rotate_register_a_right() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1010_1010);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1010_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_0101);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1010_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_almost_all_1() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1011_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing RLA (rotate register A to the left THROUGH carry bit)
    #[test]
    fn rotate_register_a_left_through_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1110);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1100);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_left_through_carry_starting_with_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1110);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing RRA (rotate register A to the right THROUGH carry bit)
    #[test]
    fn rotate_register_a_right_through_carry_with_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1011_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_through_carry_without_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0001_0011);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0001_0011);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0000_1001);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0100);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1100_0010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing INC 8-bit registers
    #[test]
    fn increment_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        motherboard.registers.write_byte(&RegByte::B, 0b1101_0111);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1101_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn increment_b_and_halfcarry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        motherboard.registers.write_byte(&RegByte::B, 0b1100_1110);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1100_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1101_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn increment_b_and_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard
            .registers
            .write_flag(RegFlag::Subtraction, false);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        motherboard.registers.write_byte(&RegByte::B, 0b1111_1110);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing incrementing 16-bit registers
    #[test]
    fn increment_bc_and_overflow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1110);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b1111_1111_1111_1111
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0000_0000_0000_0000
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing Incrementing the CONTENTS of a 16 bit register
    #[test]
    fn increment_contents_hl_register() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard
            .registers
            .write_word(&RegWord::HL, 0b1111_1111_1111_1110);

        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::HL), 0xFE);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_HLcontents);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::HL),
            0b1111_1111_1111_1110
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::HL)),
            0xFF
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_HLcontents);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::HL),
            0b1111_1111_1111_1110
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::HL)),
            0x00
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing load then increment 16-bit registers
    #[test]
    fn load_a_to_hl_then_increment_hl() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard
            .registers
            .write_word(&RegWord::HL, 0b1111_1111_1111_1111);

        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::HL), 0b0000_1101);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_0001);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_HLincrementedcontents_A);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0b0);
        assert_eq!(
            motherboard.memory.read_byte(0b1111_1111_1111_1111),
            motherboard.registers.read_byte(&RegByte::A)
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b001_0001);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_HLincrementedcontents_A);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0b1);
        assert_eq!(
            motherboard.memory.read_byte(0b0),
            motherboard.registers.read_byte(&RegByte::A)
        );
    }

    // Loading from virtual register to 8-bit register
    #[test]
    fn load_hl_to_a_then_increment() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard.registers.write_word(&RegWord::HL, 0xAAFF);

        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::HL), 0b0000_1101);

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1111);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_HLincrementedcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xAB00);
        assert_eq!(
            motherboard.memory.read_byte(0xAAFF),
            motherboard.registers.read_byte(&RegByte::A)
        );
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0000_1101);
    }

    // Testing DEC 8-bit registers

    #[test]
    fn decrement_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard
            .registers
            .write_flag(RegFlag::Subtraction, false);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        motherboard.registers.write_byte(&RegByte::B, 0b1111_1110);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1111_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1111_1100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn decrement_b_and_halfborrow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard
            .registers
            .write_flag(RegFlag::Subtraction, false);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        motherboard.registers.write_byte(&RegByte::B, 0b1111_0000);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1110_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1110_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing decrementing virtual 16-bit registers
    #[test]
    fn dec_bc() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b0000_0000_0000_0001);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_BC);

        assert_eq!(motherboard.registers.read_word(&RegWord::BC), 0b0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b1111_1111_1111_1111
        );
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing loading to 8bit register from virtual 16-bit registers
    #[test]
    fn load_bc_to_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1000_0001);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b0100_1001_0000_1111);
        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0100_1001_0000_1111
        );

        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::BC), 0b0000_0010);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_BCcontents);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0000_0010);
        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0100_1001_0000_1111
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );
        assert_eq!(
            motherboard.memory.read_byte(0b0100_1001_0000_1111),
            0b0000_0010
        );
    }

    // Testing loading to virtual 16-bit registers from 8bit register
    #[test]
    fn load_a_to_bc() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1000_0001);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b0100_1001_0000_1111);
        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0100_1001_0000_1111
        );

        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::BC), 0b0000_0010);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_BCcontents_A);

        motherboard.registers.pretty_print_word();

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0100_1001_0000_1111
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            0b1000_0001
        );
        assert_eq!(
            motherboard.memory.read_byte(0b0100_1001_0000_1111),
            0b1000_0001
        );
    }

    // Testing adding virtual register to virtual register
    #[test]
    fn add_bc_to_hl() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b0000_0000_1111_1111);

        motherboard
            .registers
            .write_word(&RegWord::HL, 0b0000_0000_0000_0001);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_HL_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0000_0000_1111_1111
        );
        assert_eq!(
            motherboard.registers.read_word(&RegWord::HL),
            0b0000_0001_0000_0000
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_bc_to_hl_and_halfoverflow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b0000_0000_0000_1111);

        motherboard
            .registers
            .write_word(&RegWord::HL, 0b0000_1111_1111_1111);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_HL_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0000_0000_0000_1111
        );
        assert_eq!(
            motherboard.registers.read_word(&RegWord::HL),
            0b0001_0000_0000_1110
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_bc_to_hl_and_overflow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1111);

        motherboard
            .registers
            .write_word(&RegWord::HL, 0b0000_0000_0000_0011);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_HL_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b1111_1111_1111_1111
        );
        assert_eq!(
            motherboard.registers.read_word(&RegWord::HL),
            0b0000_0000_0000_0010
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn add_hl_to_hl() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        motherboard
            .registers
            .write_word(&RegWord::HL, 0b0000_0000_0000_0011);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_HL_HL);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::HL),
            0b0000_0000_0000_0110
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing Decimal Adjust Accumulator
    #[test]
    fn decimal_adjust_accumulator_low_overflow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);

        motherboard.registers.write_byte(&RegByte::A, 0x12);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DAA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x18);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
    }

    #[test]
    fn decimal_adjust_accumulator_full_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard.registers.write_byte(&RegByte::A, 0xC0);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DAA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x20);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn decimal_adjust_accumulator_with_halfcarry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::HalfCarry, true);

        motherboard.registers.write_byte(&RegByte::A, 0x9E);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DAA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x04);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
    }

    #[test]
    fn decimal_adjust_accumulator_subtraction_with_halfborrow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        motherboard.registers.write_byte(&RegByte::A, 0x0F);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DAA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x09);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
    }

    #[test]
    fn decimal_adjust_accumulator_addition_no_change() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0x35);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DAA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x35);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
    }

    #[test]
    fn decimal_adjust_accumulator_addition_produce_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0x00);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DAA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x00);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
    }
    // Testing 8-Bit Register Inversing (complement)
    #[test]
    fn invert_a_register() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0x00);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::CPL);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0xFF);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);

        // Part2

        motherboard.registers.write_byte(&RegByte::A, 0b1001_0111);

        motherboard.registers.write_flag(RegFlag::Carry, true);
        motherboard.registers.write_flag(RegFlag::Zero, true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::CPL);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0110_1000);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
    }

    // Testing Flag Only Related Operations
    #[test]
    fn set_then_complement_carry_flag() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::Zero, true);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::SCF);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::CCF);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
    }

    // Testing Stack related operations
    #[test]
    fn pop_top2_off_stack_and_build_new_pc_retnz() {
        let mut motherboard = motherboard::Motherboard::new();

        // Setup PC
        motherboard.registers.write_word(&RegWord::PC, 0x0010);
        motherboard.memory.write_byte(0x0010, 0x88);

        motherboard.memory.write_byte(0xAA21, 0x07);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0x4000);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0x3FFF, 0xAA);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0x3FFE, 0x21);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RET_NZ);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x4000);
        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0xAA21);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::PC)),
            0x07
        );
    }

    #[test]
    fn ret_nz_but_z_is_set() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, true);

        // Setup PC
        motherboard.registers.write_word(&RegWord::PC, 0x0010);
        motherboard.memory.write_byte(0x0010, 0x88);

        motherboard.memory.write_byte(0xAA21, 0x07);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0x4000);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0x3FFF, 0xAA);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0x3FFE, 0x21);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RET_NZ);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x3FFE);
        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0x0010);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::PC)),
            0x88
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::SP)),
            0x21
        );
    }

    #[test]
    fn ret_nz_testing_stack_boundary() {
        let mut motherboard = motherboard::Motherboard::new();

        // Setup PC
        motherboard.registers.write_word(&RegWord::PC, 0x0040);
        motherboard.memory.write_byte(0x0040, 0x88);
        motherboard.memory.write_byte(0x8000, 0x12);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0x0000);
        motherboard.memory.write_byte(0x0000, 0x21);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0xFFFF, 0x80);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0xFFFE, 0x00);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RET_NZ);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x0000);
        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0x8000);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::PC)),
            0x12
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::SP)),
            0x21
        );
    }

    #[test]
    fn pop_bc() {
        let mut motherboard = motherboard::Motherboard::new();

        // Setup BC
        motherboard.registers.write_word(&RegWord::BC, 0x0040);
        motherboard.memory.write_byte(0x0040, 0x88);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0x00);
        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0x40);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0xFFEA);
        motherboard.memory.write_byte(0xFFEA, 0x21);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0xFFE9, 0x45);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0xFFE8, 0xAB);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::POP_BC);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0xFFEA);
        assert_eq!(motherboard.registers.read_word(&RegWord::BC), 0x45AB);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0x45);
        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0xAB);
    }

    #[test]
    fn push_bc() {
        let mut motherboard = motherboard::Motherboard::new();

        // Setup BC
        motherboard.registers.write_word(&RegWord::BC, 0x2AFF);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0x2A);
        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0xFF);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0x1F00);
        motherboard.memory.write_byte(0x1F00, 0x21);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0x1EFF, 0x45);
        motherboard.registers.decrement_sp();
        motherboard.memory.write_byte(0x1EFE, 0xBB);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::PUSH_BC);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x1EFC);
        assert_eq!(motherboard.registers.read_word(&RegWord::BC), 0x2AFF);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0x2A);
        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0xFF);

        assert_eq!(motherboard.memory.read_byte(0x1F00), 0x21);
        assert_eq!(motherboard.memory.read_byte(0x1EFF), 0x45);
        assert_eq!(motherboard.memory.read_byte(0x1EFE), 0xBB);
        assert_eq!(motherboard.memory.read_byte(0x1EFD), 0x2A);
        assert_eq!(motherboard.memory.read_byte(0x1EFC), 0xFF);
    }

    #[test]
    fn push_bc_and_underflow_stack() {
        let mut motherboard = motherboard::Motherboard::new();

        // Setup BC
        motherboard.registers.write_word(&RegWord::BC, 0xBB19);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0xBB);
        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0x19);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0x0001);
        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::SP), 0x22);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::PUSH_BC);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0xFFFF);
        assert_eq!(motherboard.registers.read_word(&RegWord::BC), 0xBB19);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0xBB);
        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0x19);

        assert_eq!(motherboard.memory.read_byte(0x0001), 0x22);
        assert_eq!(motherboard.memory.read_byte(0x0000), 0xBB);
        assert_eq!(motherboard.memory.read_byte(0xFFFF), 0x19);
    }

    #[test]
    fn fast_rst_to_address() {
        let mut motherboard = motherboard::Motherboard::new();

        // Setup BC
        motherboard.registers.write_word(&RegWord::PC, 0xABCD);

        // Setup stack
        motherboard.registers.write_word(&RegWord::SP, 0x1000);
        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::SP), 0x99);

        motherboard.registers.pretty_print_word();
        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RST_08H);
        motherboard.registers.pretty_print_word();

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x0FFE);
        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0x0008);

        assert_eq!(motherboard.memory.read_byte(0x1000), 0x99);
        assert_eq!(motherboard.memory.read_byte(0x0FFF), 0xAB);
        // 0xCD becomes 0xCE because it's the low byte and we have to increment program counter once
        // To push the address of the byte AFTER THIS OneByteOpCode unto the stack.
        assert_eq!(motherboard.memory.read_byte(0x0FFE), 0xCE);
    }

    // OLD TESTS CONVERTED TO NEW:
    // Add to A Tests
    #[test]
    fn execute_op_code_ADD_A_B_add_0_0() {
        // Adding 0 + 0, before
        let mut motherboard = motherboard::Motherboard::new();

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_B);

        // after, only Zero should be true
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_0_24() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        motherboard.registers.write_byte(&RegByte::B, 24);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 24);

        // Adding B = 24, to A = 0, should be A=0 + proper flags
        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 24);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 24);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_1_15() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 15);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 15);
        motherboard.registers.write_byte(&RegByte::B, 1);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 1);

        // Adding B = 1, to A = 15, should be A=16 + proper flags
        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 16);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 1);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_243_25() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 243);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 243);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);

        // Adding B = 25, to A = 243, should be A=12 with a overflow + proper flags
        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 12);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_200_25_and_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 200);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 200);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 225);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_hl_to_a() {
        let mut motherboard = motherboard::Motherboard::new();

        // Working in HRAM
        motherboard.registers.write_word(&RegWord::HL, 65410);
        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 65410);
        motherboard.memory.write_byte(65410, 99);
        let value = motherboard.memory.read_byte(65410);
        assert_eq!(value, 99);

        motherboard.registers.write_byte(&RegByte::A, 200);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 200);
        motherboard.registers.write_flag(RegFlag::Carry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_HLcontents);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 43);
        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 65410);
        assert_eq!(motherboard.memory.read_byte(65410), 99);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Add with Carry to A Tests
    #[test]
    fn execute_op_code_ADC_A_B_add_29_25_and_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 29);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 29);
        motherboard.registers.write_byte(&RegByte::B, 30);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 30);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADC_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 60);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 30);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADC_A_D_add_200_25_and_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 200);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 200);
        motherboard.registers.write_byte(&RegByte::D, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADC_A_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 226);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADC_A_A_add_100_100_and_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_byte(&RegByte::D, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADC_A_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 201);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_carry_to_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        motherboard.registers.write_byte(&RegByte::D, 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADC_A_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 1);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_carry_into_overflow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 254);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 254);
        motherboard.registers.write_byte(&RegByte::D, 1);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 1);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADC_A_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 1);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // LOAD tests
    #[test]
    fn load_b_to_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn load_e_to_d() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::E, 240);
        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 240);
        motherboard.registers.write_byte(&RegByte::D, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 25);
        motherboard.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_D_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 240);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 240);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn load_h_to_h() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::H, 240);
        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 240);
        motherboard.registers.write_byte(&RegByte::H, 240);
        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 240);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_D_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 240);
        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 240);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Bitwise AND Tests
    #[test]
    fn bitwise_and_of_a_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::AND_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_and_of_a_h() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 73);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 73);
        motherboard.registers.write_byte(&RegByte::H, 29);
        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 29);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::AND_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 9);
        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 29);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_and_of_a_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 73);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 73);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::AND_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 73);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 73);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Bitwise XOR Tests
    #[test]
    fn bitwise_xor_of_a_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::XOR_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 125);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_xor_of_a_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::XOR_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_xor_of_a_e() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 254);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 254);
        motherboard.registers.write_byte(&RegByte::E, 1);
        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 1);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::XOR_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 255);
        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 1);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Increment and Decrement 8bit Registers
    #[test]
    fn chain_increment_decrement_8bit_register() {
        let mut motherboard = motherboard::Motherboard::new();

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 255);
        motherboard.registers.write_byte(&RegByte::B, 24);
        motherboard.registers.write_byte(&RegByte::C, 33);
        motherboard.registers.write_byte(&RegByte::D, 1);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 255);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 34);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Bitwise OR Tests
    #[test]
    fn bitwise_or_of_a_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::OR_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 125);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_or_of_a_d_zeroed() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        motherboard.registers.write_byte(&RegByte::D, 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::OR_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Rotate A-Register left using RLCA or RLA
    #[test]
    fn old_rotate_register_a_left_producing_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b10101010);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b10101010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b01010101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn old_rotate_register_a_left_without_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0010_1010);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0010_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rotate_register_a_left_through_carry_without_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_left_through_carry_with_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0000_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0000_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0001_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RLA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0011_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Rotate Right A-register
    #[test]
    fn old_rotate_register_a_right_through_carry_without_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_without_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1110);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1110);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0011_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rotate_register_right_with_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1101);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1101);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRCA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1011_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn old_rotate_register_a_right_through_carry_with_carry() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b0000_1111);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0000_1111);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::RRA);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1100_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Load Virtual Registers to A
    #[test]
    fn chain_load_virtual_registers_to_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard.registers.write_word(&RegWord::HL, 9000);
        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::HL), 49);
        motherboard.memory.write_byte(8999, 200);
        motherboard.registers.write_byte(&RegByte::A, 10);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_HLdecrementedcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 8999);
        assert_eq!(motherboard.memory.read_byte(9000), 49);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 49);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_HLincrementedcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 9000);
        assert_eq!(motherboard.memory.read_byte(8999), 200);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 200);

        motherboard.registers.write_word(&RegWord::BC, 4000);
        motherboard.memory.write_byte(4000, 1);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_BCcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::BC), 4000);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 1);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);
    }

    // Compare to A tests
    #[test]
    fn compare_a_to_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        motherboard.registers.write_byte(&RegByte::B, 25);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::CP_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 100);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 25);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn compare_a_to_c_zeroed() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 99);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 99);
        motherboard.registers.write_byte(&RegByte::B, 99);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 99);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard
            .registers
            .write_flag(RegFlag::Subtraction, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::CP_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 99);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 99);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Test Virtual 16 bit register manipulations
    #[test]
    fn manipulate_hl_register() {
        let mut motherboard = motherboard::Motherboard::new();

        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);

        motherboard.registers.write_word(&RegWord::HL, 55151);
        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::HL), 241);

        motherboard.registers.write_word(&RegWord::BC, 2001);
        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::BC), 25);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 55151);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::HL)),
            241
        );

        assert_eq!(motherboard.registers.read_word(&RegWord::BC), 2001);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            25
        );

        // Where HL should point to after ADD_HL_BC
        motherboard.memory.write_byte(57152, 99);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_HL_BC);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 57152);
        assert_eq!(motherboard.memory.read_byte(55151), 241);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::HL)),
            99
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            25
        );

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_HL);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 57153);
        // assert_eq!(
        //     motherboard.memory.read_byte(motherboard.registers.read_word(&RegWord::HL)),
        //     100
        // );

        // Where HL should point after INC_HL_REG
        motherboard.memory.write_byte(57153, 4);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_HL);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 57154);
        // assert_eq!(
        //     motherboard.memory.read_byte(motherboard.registers.read_word(&RegWord::HL)),
        //     4
        // );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(48772, 29);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_HL_HL);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 48772);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::HL)),
            29
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // Test Decrementing and Incrementing Virtual Registers
    #[test]
    fn decrement_sp_below_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard
            .registers
            .write_word(&RegWord::SP, 0b0000_0000_0000_0001);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_SP);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::SP),
            0b0000_0000_0000_0000
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_SP);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::SP),
            0b1111_1111_1111_1111
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn old_increment_bc_and_overflow() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1110);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b1111_1111_1111_1111
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::INC_BC);

        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0000_0000_0000_0000
        );

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // TODO these names are getting a bit outta hand
    #[test]
    fn subtract_from_a_nop() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0x00);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::SUB_A_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 1);
        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing Loading to Virtual 16-bit Registers
    #[test]
    fn old_load_a_to_bc() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1000_0001);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard
            .registers
            .write_word(&RegWord::BC, 0b0100_1001_0000_1111);
        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0100_1001_0000_1111
        );

        motherboard
            .memory
            .write_byte(motherboard.registers.read_word(&RegWord::BC), 0b0000_0010);
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_BCcontents_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(
            motherboard.registers.read_word(&RegWord::BC),
            0b0100_1001_0000_1111
        );
        assert_eq!(
            motherboard
                .memory
                .read_byte(motherboard.registers.read_word(&RegWord::BC)),
            0b1000_0001
        );
        assert_eq!(
            motherboard.memory.read_byte(0b0100_1001_0000_1111),
            0b1000_0001
        );
    }

    // Testing Chaining OpCodes
    #[test]
    fn chaining_commands_on_register_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::D, 43);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 43);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0);

        motherboard.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        motherboard
            .registers
            .write_flag(RegFlag::Subtraction, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        motherboard.registers.write_flag(RegFlag::Carry, false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 43);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 43);

        // No flags change during load commands
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 86);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 43);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 172);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 43);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::DEC_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 171);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 43);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);

        motherboard.registers.write_byte(&RegByte::B, 91);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 91);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::XOR_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 240);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 91);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::ADD_A_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 75);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 91);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::SBC_A_D);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 31);
        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 43);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Load 0xFF00 + n8 commands
    #[test]
    fn load_a_to_0xff_plus_c() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::C, 0x33);
        motherboard.registers.write_byte(&RegByte::A, 7);
        motherboard.memory.write_byte(0xFF33, 0);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_Ccontents_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0x33);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 7);
        assert_eq!(motherboard.memory.read_byte(0xFF33), 7);
    }

    #[test]
    fn load_0xff_plus_c_to_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::C, 0x33);
        motherboard.registers.write_byte(&RegByte::A, 7);
        motherboard.memory.write_byte(0xFF33, 2);

        execute_one_byte_opcode(&mut motherboard, OneByteOpCode::LD_A_Ccontents);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0x33);
        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 2);
        assert_eq!(motherboard.memory.read_byte(0xFF33), 2);
    }

    // Two Byte OpCode Testing
    // Load byte into register
    #[test]
    fn load_byte_into_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::B, 0x43);
        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0x43);

        motherboard.registers.write_flag(RegFlag::Zero, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::LD_B_N8, 0x91);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0x91);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
    }

    // Jump Offset
    #[test]
    fn jump_offset_no_wrap() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::PC, 1200);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::JR_R8, 100);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 1300);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::JR_R8, 128);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 1172);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::JR_R8, 127);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 1299);
    }

    #[test]
    fn jump_offset_wrap() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::PC, 10);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::JR_R8, 240);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0xFFFA);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::JR_R8, 7);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 1);
    }

    // Addition with signed integer and stackpointer:
    #[test]
    fn positive_r8_added_to_sp() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::SP, 0x1000);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::ADD_SP_R8, 0x05);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x1005);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // TODO: Look over this test, negatives spooky
    #[test]
    fn negative_r8_added_to_sp() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::SP, 0x0010);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::ADD_SP_R8, 0xFF); // -1

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x000F);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn half_carry_from_r8_added_to_sp() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::SP, 0x000F);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::ADD_SP_R8, 0x01); // 1

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x0010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // TODO: Look over this test, negatives spooky
    #[test]
    fn half_carry_from_negative_r8_added_to_sp() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::SP, 0x001F);

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::ADD_SP_R8, 0xFF); // -1

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0x001E);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // LD FF?? Related tests
    #[test]
    fn loadhigh_a_a8() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.memory.write_byte(0xFFFE, 0x21);
        motherboard.memory.write_byte(0xFE, 0x99); // Shouldn't hit this.

        execute_two_byte_opcode(&mut motherboard, TwoByteOpCode::LDH_A_A8contents, 0xFE);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0x21);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Three Byte OpCode Tests:
    // Loading SP into a16 & 16+a1
    #[test]
    fn load_sp_into_memory() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::SP, 0xABCD);

        execute_three_byte_opcode(
            &mut motherboard,
            ThreeByteOpCode::LD_A16contents_SP,
            0xFF,
            0x10,
        );

        assert_eq!(motherboard.memory.read_byte(0xFF10), 0xCD);
        assert_eq!(motherboard.memory.read_byte(0xFF11), 0xAB);
    }

    // Loading a 16 bit num into SP
    #[test]
    fn load_d16_into_sp() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::SP, 0xABCD);

        execute_three_byte_opcode(&mut motherboard, ThreeByteOpCode::LD_SP_D16, 0xFA, 0x10);

        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0xFA10);
    }

    // Jumping to an n16
    #[test]
    fn jump_to_d16() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::PC, 0x0010);
        motherboard.registers.write_flag(RegFlag::Zero, true);

        execute_three_byte_opcode(&mut motherboard, ThreeByteOpCode::JP_NZ_A16, 0xFA, 0x10);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0x0010);

        motherboard.registers.write_flag(RegFlag::Zero, false);

        execute_three_byte_opcode(&mut motherboard, ThreeByteOpCode::JP_NZ_A16, 0xFA, 0x10);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0xFA10);
    }

    // TODO: test calls => ThreeByteOpCode::CALL_NZ_A16
    #[test]
    fn call_nz_to_a16() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::PC, 0x0010);
        motherboard.registers.write_word(&RegWord::SP, 0xFFF5);

        motherboard.registers.write_flag(RegFlag::Zero, true);

        execute_three_byte_opcode(&mut motherboard, ThreeByteOpCode::CALL_NZ_A16, 0xFA, 0x10);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0x0010);
        assert_eq!(motherboard.registers.read_word(&RegWord::SP), 0xFFF5);

        motherboard.registers.write_flag(RegFlag::Zero, false);

        execute_three_byte_opcode(&mut motherboard, ThreeByteOpCode::CALL_NZ_A16, 0xFA, 0x10);

        assert_eq!(motherboard.registers.read_word(&RegWord::PC), 0xFA10);
        assert_eq!(motherboard.memory.read_byte(0xFFF4), 0x00);
        assert_eq!(motherboard.memory.read_byte(0xFFF3), 0x11);
    }

    // Prefix Tests Section
    // RLC tests
    #[test]
    fn rlc_register_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::B, 0b1001_0001);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RLC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0010_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rlc_register_b_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::B, 0b0000_0000);

        motherboard.registers.write_flag(RegFlag::Carry, true);
        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RLC_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rlc_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RLC_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RLC_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1010_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // RRC Tests
    #[test]
    fn rrc_register_d() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::D, 0b1001_0001);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RRC_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b1100_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RRC_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0110_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rrc_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RRC_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0011_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RRC_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0001_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RRC_HLcontents);
        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RRC_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    // RL Tests
    #[test]
    fn rl_register_a() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::A, 0b1000_1111);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RL_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0001_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RL_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0011_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rl_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1010_0010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0100_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // RR Tests
    #[test]
    fn rr_register_e() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::E, 0b1000_1111);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 0b0100_0111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 0b1010_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 0b1101_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 0b1110_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_E);

        assert_eq!(motherboard.registers.read_byte(&RegByte::E), 0b1111_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rr_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1011_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0010_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0001_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RR_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // SLA Tests
    #[test]
    fn sla_register_l() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::L, 0b1000_1111);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0001_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0011_1100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0111_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1111_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1110_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1100_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn sla_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1010_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0100_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SLA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // SRA Tests
    #[test]
    fn sra_register_d_bit7_zero() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::D, 0b0101_1111);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0010_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0001_0111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_D);

        assert_eq!(motherboard.registers.read_byte(&RegByte::D), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn sra_register_h_bit7_one() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::H, 0b1101_1010);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1110_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn sra_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0011_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0001_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRA_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Swap tests
    #[test]
    fn swap_multitude_registers() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b1101_1010);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1010_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1110_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn swap_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b0110_1000);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SWAP_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0110_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // SRL Tests
    #[test]
    fn srl_register_b() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_byte(&RegByte::B, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0110_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0011_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0001_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn srl_register_hl_address() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0100_0010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0010_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0001_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SRL_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // BIT Check Tests
    #[test]
    fn bit0_checks() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_0_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_0_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_0_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit0_hl_address_check() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_0_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_0_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit1_checks() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_1_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register L
        motherboard.registers.write_byte(&RegByte::L, 0b0000_0010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_1_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0000_0010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::L, 0b1111_1101);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_1_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1111_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit2_checks() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1101);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register L
        motherboard.registers.write_byte(&RegByte::L, 0b0000_0100);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0000_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::L, 0b1111_1101);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1111_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit2_hl_address_check() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1011);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_2_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit5_checks() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1101);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register L
        motherboard.registers.write_byte(&RegByte::L, 0b0010_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0010_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::L, 0b1101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1101_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit5_hl_address_check() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0010_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0010_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit7_checks() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::A, 0b0111_1101);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        // Register L
        motherboard.registers.write_byte(&RegByte::L, 0b1000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b1000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.registers.write_byte(&RegByte::L, 0b0111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_L);

        assert_eq!(motherboard.registers.read_byte(&RegByte::L), 0b0111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit7_hl_address_check() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0010_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::BIT_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0010_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    // Bit RES (reset) tests
    #[test]
    fn bit0_resets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b1111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1110);
    }

    #[test]
    fn bit0_hl_address_reset() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0010_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_0_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0010_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit1_resets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1001);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0000_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0000_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b1111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1101);
    }

    #[test]
    fn bit1_hl_address_reset() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1000_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1100);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1010_0011);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1010_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit5_resets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0111_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0101_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b1111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1101_1111);
    }

    #[test]
    fn bit5_hl_address_reset() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1010_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1101_0011);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_5_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit7_resets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0101_1011);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b0111_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0111_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0111_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b1111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b0111_1111);

        motherboard.registers.write_byte(&RegByte::H, 0b1111_1100);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b0111_1100);
    }

    #[test]
    fn bit7_hl_address_reset() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1010_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0010_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0101_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1101_0011);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::RES_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit1_sets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1111_1000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0111_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0111_0011);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b1111_1100);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1111_1110);

        motherboard.registers.write_byte(&RegByte::H, 0b0111_1100);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b0111_1110);
    }

    #[test]
    fn bit1_hl_address_set() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1010_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1010_0111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0101_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1101_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_1_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit4_sets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b1101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1110_1000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1111_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0110_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b0111_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b0001_0000);

        motherboard.registers.write_byte(&RegByte::H, 0b0110_1100);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b0111_1100);

        // Register B
        motherboard.registers.write_byte(&RegByte::B, 0b1111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1111_1111);

        motherboard.registers.write_byte(&RegByte::B, 0b0110_0110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b0111_0110);
    }

    #[test]
    fn bit4_hl_address_set() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1010_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1011_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0100_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b0101_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1101_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_4_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bit7_sets() {
        let mut motherboard = motherboard::Motherboard::new();

        // Register A
        motherboard.registers.write_byte(&RegByte::A, 0b0101_1011);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, false);
        motherboard.registers.write_flag(RegFlag::Carry, true);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1101_1011);

        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        motherboard.registers.write_byte(&RegByte::A, 0b1110_1000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_A);

        assert_eq!(motherboard.registers.read_byte(&RegByte::A), 0b1110_1000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register C
        motherboard.registers.write_byte(&RegByte::C, 0b0110_0001);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_C);

        assert_eq!(motherboard.registers.read_byte(&RegByte::C), 0b1110_0001);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), true);

        // Register H
        motherboard.registers.write_byte(&RegByte::H, 0b0000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1000_0000);

        motherboard.registers.write_byte(&RegByte::H, 0b0110_1100);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_H);

        assert_eq!(motherboard.registers.read_byte(&RegByte::H), 0b1110_1100);

        // Register B
        motherboard.registers.write_byte(&RegByte::B, 0b1111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1111_1111);

        motherboard.registers.write_byte(&RegByte::B, 0b1000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_B);

        assert_eq!(motherboard.registers.read_byte(&RegByte::B), 0b1000_0000);
    }

    #[test]
    fn bit7_hl_address_set() {
        let mut motherboard = motherboard::Motherboard::new();

        motherboard.registers.write_word(&RegWord::HL, 0xFFAA);
        motherboard.memory.write_byte(0xFFAA, 0b1010_0101);

        motherboard.registers.write_flag(RegFlag::Zero, true);
        motherboard.registers.write_flag(RegFlag::Subtraction, true);
        motherboard.registers.write_flag(RegFlag::HalfCarry, true);
        motherboard.registers.write_flag(RegFlag::Carry, false);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1010_0101);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0111_1110);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1110);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0100_1010);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1100_1010);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1101_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1101_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b1000_0000);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1000_0000);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);

        motherboard.memory.write_byte(0xFFAA, 0b0111_1111);

        execute_prefix_opcode(&mut motherboard, PrefixOpCode::SET_7_HLcontents);

        assert_eq!(motherboard.registers.read_word(&RegWord::HL), 0xFFAA);
        assert_eq!(motherboard.memory.read_byte(0xFFAA), 0b1111_1111);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(motherboard.registers.read_flag(RegFlag::Carry), false);
    }
}
