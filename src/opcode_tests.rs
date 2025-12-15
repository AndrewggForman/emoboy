// TODO: Import and modify/update all original tests from OneByteOpCode to OneByteOpCode (after finishing rest);
mod tests {
    use crate::{cpu, cpu_logic::*, opcode::*, registers::*};

    use super::*;

    // Testing RLCA (rotate register A to the left and set carry bit as well)
    #[test]
    fn rotate_register_a_left_producing_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1010_1010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1010_1010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0101_0101);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_left_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0010_1010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0010_1010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0101_0100);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn chain_rotate_register_a_left() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0010_1010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0010_1010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0101_0100);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1010_1000);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0101_0001);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing RRCA (rotate register A to the right and set carry bit as well)
    #[test]
    fn rotate_register_a_right() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1010_1010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1010_1010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0101_0101);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1010_1010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_almost_all_1() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1011_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1101_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing RLA (rotate register A to the left THROUGH carry bit)
    #[test]
    fn rotate_register_a_left_through_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1110);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1100);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_left_through_carry_starting_with_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1110);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing RRA (rotate register A to the right THROUGH carry bit)
    #[test]
    fn rotate_register_a_right_through_carry_with_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers.write_byte(&RegByte::A, 0b0111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1011_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1101_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_through_carry_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0001_0011);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0001_0011);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0000_1001);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0100);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1100_0010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing INC 8-bit registers
    #[test]
    fn increment_b() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        cpu.registers.write_flag(RegFlag::Carry, false);

        cpu.registers.write_byte(&RegByte::B, 0b1101_0111);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1101_1000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn increment_b_and_halfcarry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        cpu.registers.write_flag(RegFlag::Carry, false);

        cpu.registers.write_byte(&RegByte::B, 0b1100_1110);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1100_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1101_0000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn increment_b_and_zero() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, false);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        cpu.registers.write_flag(RegFlag::Carry, false);

        cpu.registers.write_byte(&RegByte::B, 0b1111_1110);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1111_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b0000_0000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing incrementing 16-bit registers
    #[test]
    fn increment_bc_and_overflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1110);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b1111_1111_1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0000_0000_0000_0000);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing Incrementing the CONTENTS of a 16 bit register
    #[test]
    fn increment_contents_hl_register() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers
            .write_word(&RegWord::HL, 0b1111_1111_1111_1110);

        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::HL), 0xFE);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_HLcontents);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b1111_1111_1111_1110);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            0xFF
        );

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_HLcontents);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b1111_1111_1111_1110);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            0x00
        );

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing load then increment 16-bit registers
    #[test]
    fn load_a_to_hl_then_increment_hl() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers
            .write_word(&RegWord::HL, 0b1111_1111_1111_1111);

        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::HL), 0b0000_1101);

        cpu.registers.write_byte(&RegByte::A, 0b1101_0001);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_HLincrementedcontents_A);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b0);
        assert_eq!(
            cpu.memory.read_byte(0b1111_1111_1111_1111),
            cpu.registers.read_byte(&RegByte::A)
        );

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        cpu.registers.write_byte(&RegByte::A, 0b001_0001);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_HLincrementedcontents_A);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b1);
        assert_eq!(
            cpu.memory.read_byte(0b0),
            cpu.registers.read_byte(&RegByte::A)
        );
    }

    // Loading from virtual register to 8-bit register
    #[test]
    fn load_hl_to_a_then_increment() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers.write_word(&RegWord::HL, 0xAAFF);

        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::HL), 0b0000_1101);

        cpu.registers.write_byte(&RegByte::A, 0b1111_1111);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_HLincrementedcontents);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0xAB00);
        assert_eq!(
            cpu.memory.read_byte(0xAAFF),
            cpu.registers.read_byte(&RegByte::A)
        );
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0000_1101);
    }

    // Testing DEC 8-bit registers

    #[test]
    fn decrement_b() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, false);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        cpu.registers.write_flag(RegFlag::Carry, false);

        cpu.registers.write_byte(&RegByte::B, 0b1111_1110);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1111_1101);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1111_1100);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn decrement_b_and_halfborrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, false);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        cpu.registers.write_flag(RegFlag::Carry, false);

        cpu.registers.write_byte(&RegByte::B, 0b1111_0000);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1110_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0b1110_1110);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing decrementing virtual 16-bit registers
    #[test]
    fn dec_bc() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b0000_0000_0000_0001);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b1111_1111_1111_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing loading to 8bit register from virtual 16-bit registers
    #[test]
    fn load_bc_to_a() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0001);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.registers
            .write_word(&RegWord::BC, 0b0100_1001_0000_1111);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0100_1001_0000_1111);

        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::BC), 0b0000_0010);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_BCcontents);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0000_0010);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0100_1001_0000_1111);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );
        assert_eq!(cpu.memory.read_byte(0b0100_1001_0000_1111), 0b0000_0010);
    }

    // Testing loading to virtual 16-bit registers from 8bit register
    #[test]
    fn load_a_to_bc() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0001);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.registers
            .write_word(&RegWord::BC, 0b0100_1001_0000_1111);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0100_1001_0000_1111);

        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::BC), 0b0000_0010);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_BCcontents_A);

        cpu.registers.pretty_print_word();

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0100_1001_0000_1111);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            0b1000_0001
        );
        assert_eq!(cpu.memory.read_byte(0b0100_1001_0000_1111), 0b1000_0001);
    }

    // Testing adding virtual register to virtual register
    #[test]
    fn add_bc_to_hl() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Subtraction, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b0000_0000_1111_1111);

        cpu.registers
            .write_word(&RegWord::HL, 0b0000_0000_0000_0001);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_HL_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0000_0000_1111_1111);
        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b0000_0001_0000_0000);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_bc_to_hl_and_halfoverflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Subtraction, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b0000_0000_0000_1111);

        cpu.registers
            .write_word(&RegWord::HL, 0b0000_1111_1111_1111);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_HL_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0000_0000_0000_1111);
        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b0001_0000_0000_1110);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_bc_to_hl_and_overflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Subtraction, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1111);

        cpu.registers
            .write_word(&RegWord::HL, 0b0000_0000_0000_0011);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_HL_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b1111_1111_1111_1111);
        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b0000_0000_0000_0010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn add_hl_to_hl() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Subtraction, true);

        cpu.registers
            .write_word(&RegWord::HL, 0b0000_0000_0000_0011);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_HL_HL);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 0b0000_0000_0000_0110);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Testing Decimal Adjust Accumulator
    #[test]
    fn decimal_adjust_accumulator_low_overflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);

        cpu.registers.write_byte(&RegByte::A, 0x12);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DAA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0x18);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
    }

    #[test]
    fn decimal_adjust_accumulator_full_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers.write_byte(&RegByte::A, 0xC0);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DAA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0x20);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn decimal_adjust_accumulator_with_halfcarry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::HalfCarry, true);

        cpu.registers.write_byte(&RegByte::A, 0x9E);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DAA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0x04);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
    }

    #[test]
    fn decimal_adjust_accumulator_subtraction_with_halfborrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);

        cpu.registers.write_byte(&RegByte::A, 0x0F);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DAA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0x09);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
    }

    #[test]
    fn decimal_adjust_accumulator_addition_no_change() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0x35);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DAA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0x35);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
    }

    #[test]
    fn decimal_adjust_accumulator_addition_produce_zero() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0x00);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DAA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0x00);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
    }
    // Testing 8-Bit Register Inversing (complement)
    #[test]
    fn invert_a_register() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0x00);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::CPL);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0xFF);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);

        // Part2

        cpu.registers.write_byte(&RegByte::A, 0b1001_0111);

        cpu.registers.write_flag(RegFlag::Carry, true);
        cpu.registers.write_flag(RegFlag::Zero, true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::CPL);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0110_1000);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
    }

    // Testing Flag Only Related Operations
    #[test]
    fn set_then_complement_carry_flag() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::Zero, true);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::SCF);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::CCF);

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
    }

    // Testing Stack related operations
    #[test]
    fn pop_top2_off_stack_and_build_new_pc_retnz() {
        let mut cpu = cpu::Cpu::new();

        // Setup PC
        cpu.registers.write_word(&RegWord::PC, 0x0010);
        cpu.memory.write_byte(0x0010, 0x88);

        cpu.memory.write_byte(0xAA21, 0x07);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0x4000);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0x3FFF, 0xAA);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0x3FFE, 0x21);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RET_NZ);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0x4000);
        assert_eq!(cpu.registers.read_word(&RegWord::PC), 0xAA21);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::PC)),
            0x07
        );
    }

    #[test]
    fn ret_nz_but_z_is_set() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, true);

        // Setup PC
        cpu.registers.write_word(&RegWord::PC, 0x0010);
        cpu.memory.write_byte(0x0010, 0x88);

        cpu.memory.write_byte(0xAA21, 0x07);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0x4000);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0x3FFF, 0xAA);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0x3FFE, 0x21);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RET_NZ);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0x3FFE);
        assert_eq!(cpu.registers.read_word(&RegWord::PC), 0x0010);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::PC)),
            0x88
        );
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::SP)),
            0x21
        );
    }

    #[test]
    fn ret_nz_testing_stack_boundary() {
        let mut cpu = cpu::Cpu::new();

        // Setup PC
        cpu.registers.write_word(&RegWord::PC, 0x0040);
        cpu.memory.write_byte(0x0040, 0x88);
        cpu.memory.write_byte(0x8000, 0x12);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0x0000);
        cpu.memory.write_byte(0x0000, 0x21);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0xFFFF, 0x80);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0xFFFE, 0x00);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RET_NZ);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0x0000);
        assert_eq!(cpu.registers.read_word(&RegWord::PC), 0x8000);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::PC)),
            0x12
        );
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::SP)),
            0x21
        );
    }

    #[test]
    fn pop_bc() {
        let mut cpu = cpu::Cpu::new();

        // Setup BC
        cpu.registers.write_word(&RegWord::BC, 0x0040);
        cpu.memory.write_byte(0x0040, 0x88);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0x00);
        assert_eq!(cpu.registers.read_byte(&RegByte::C), 0x40);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0xFFEA);
        cpu.memory.write_byte(0xFFEA, 0x21);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0xFFE9, 0x45);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0xFFE8, 0xAB);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::POP_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0xFFEA);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0x45AB);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0x45);
        assert_eq!(cpu.registers.read_byte(&RegByte::C), 0xAB);
    }

    #[test]
    fn push_bc() {
        let mut cpu = cpu::Cpu::new();

        // Setup BC
        cpu.registers.write_word(&RegWord::BC, 0x2AFF);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0x2A);
        assert_eq!(cpu.registers.read_byte(&RegByte::C), 0xFF);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0x1F00);
        cpu.memory.write_byte(0x1F00, 0x21);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0x1EFF, 0x45);
        cpu.registers.decrement_sp();
        cpu.memory.write_byte(0x1EFE, 0xBB);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::PUSH_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0x1EFC);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0x2AFF);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0x2A);
        assert_eq!(cpu.registers.read_byte(&RegByte::C), 0xFF);

        assert_eq!(cpu.memory.read_byte(0x1F00), 0x21);
        assert_eq!(cpu.memory.read_byte(0x1EFF), 0x45);
        assert_eq!(cpu.memory.read_byte(0x1EFE), 0xBB);
        assert_eq!(cpu.memory.read_byte(0x1EFD), 0x2A);
        assert_eq!(cpu.memory.read_byte(0x1EFC), 0xFF);
    }

    #[test]
    fn push_bc_and_underflow_stack() {
        let mut cpu = cpu::Cpu::new();

        // Setup BC
        cpu.registers.write_word(&RegWord::BC, 0xBB19);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0xBB);
        assert_eq!(cpu.registers.read_byte(&RegByte::C), 0x19);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0x0001);
        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::SP), 0x22);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::PUSH_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0xFFFF);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0xBB19);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0xBB);
        assert_eq!(cpu.registers.read_byte(&RegByte::C), 0x19);

        assert_eq!(cpu.memory.read_byte(0x0001), 0x22);
        assert_eq!(cpu.memory.read_byte(0x0000), 0xBB);
        assert_eq!(cpu.memory.read_byte(0xFFFF), 0x19);
    }

    #[test]
    fn fast_rst_to_address() {
        let mut cpu = cpu::Cpu::new();

        // Setup BC
        cpu.registers.write_word(&RegWord::PC, 0xABCD);

        // Setup stack
        cpu.registers.write_word(&RegWord::SP, 0x1000);
        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::SP), 0x99);

        cpu.registers.pretty_print_word();
        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RST_08H);
        cpu.registers.pretty_print_word();

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0x0FFE);
        assert_eq!(cpu.registers.read_word(&RegWord::PC), 0x0008);

        assert_eq!(cpu.memory.read_byte(0x1000), 0x99);
        assert_eq!(cpu.memory.read_byte(0x0FFF), 0xAB);
        // 0xCD becomes 0xCE because it's the low byte and we have to increment program counter once
        // To push the address of the byte AFTER THIS OneByteOpCode unto the stack.
        assert_eq!(cpu.memory.read_byte(0x0FFE), 0xCE);
    }

    // OLD TESTS CONVERTED TO NEW:
    // Add to A Tests
    #[test]
    fn execute_op_code_ADD_A_B_add_0_0() {
        // Adding 0 + 0, before
        let mut cpu = cpu::Cpu::new();

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_B);

        // after, only Zero should be true
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_0_24() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        cpu.registers.write_byte(&RegByte::B, 24);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 24);

        // Adding B = 24, to A = 0, should be A=0 + proper flags
        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 24);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 24);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_1_15() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 15);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 15);
        cpu.registers.write_byte(&RegByte::B, 1);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 1);

        // Adding B = 1, to A = 15, should be A=16 + proper flags
        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 16);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 1);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_243_25() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 243);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 243);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);

        // Adding B = 25, to A = 243, should be A=12 with a overflow + proper flags
        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 12);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn execute_op_code_ADD_A_B_add_200_25_and_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 225);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_hl_to_a() {
        let mut cpu = cpu::Cpu::new();

        // Working in HRAM
        cpu.registers.write_word(&RegWord::HL, 65410);
        assert_eq!(cpu.registers.read_word(&RegWord::HL), 65410);
        cpu.memory.write_byte(65410, 99);
        let value = cpu.memory.read_byte(65410);
        assert_eq!(value, 99);

        cpu.registers.write_byte(&RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_flag(RegFlag::Carry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_HLcontents);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 43);
        assert_eq!(cpu.registers.read_word(&RegWord::HL), 65410);
        assert_eq!(cpu.memory.read_byte(65410), 99);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Add with Carry to A Tests
    #[test]
    fn execute_op_code_ADC_A_B_add_29_25_and_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 29);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 29);
        cpu.registers.write_byte(&RegByte::B, 30);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 30);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADC_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 60);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 30);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADC_A_D_add_200_25_and_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_byte(&RegByte::D, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADC_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 226);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADC_A_A_add_100_100_and_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::D, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADC_A_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 201);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_carry_to_zero() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        cpu.registers.write_byte(&RegByte::D, 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 0);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADC_A_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 1);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn add_carry_into_overflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 254);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 254);
        cpu.registers.write_byte(&RegByte::D, 1);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 1);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADC_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 1);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // LOAD tests
    #[test]
    fn load_b_to_a() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn load_e_to_d() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::E, 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::E), 240);
        cpu.registers.write_byte(&RegByte::D, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        cpu.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_D_E);

        assert_eq!(cpu.registers.read_byte(&RegByte::E), 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 240);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn load_h_to_h() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::H, 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::H), 240);
        cpu.registers.write_byte(&RegByte::H, 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::H), 240);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_D_E);

        assert_eq!(cpu.registers.read_byte(&RegByte::H), 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::H), 240);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Bitwise AND Tests
    #[test]
    fn bitwise_and_of_a_b() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::AND_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_and_of_a_h() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 73);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 73);
        cpu.registers.write_byte(&RegByte::H, 29);
        assert_eq!(cpu.registers.read_byte(&RegByte::H), 29);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::AND_H);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 9);
        assert_eq!(cpu.registers.read_byte(&RegByte::H), 29);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_and_of_a_a() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 73);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 73);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::AND_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 73);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 73);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Bitwise XOR Tests
    #[test]
    fn bitwise_xor_of_a_b() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::XOR_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 125);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_xor_of_a_a() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::XOR_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_xor_of_a_e() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 254);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 254);
        cpu.registers.write_byte(&RegByte::E, 1);
        assert_eq!(cpu.registers.read_byte(&RegByte::E), 1);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::XOR_E);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 255);
        assert_eq!(cpu.registers.read_byte(&RegByte::E), 1);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Increment and Decrement 8bit Registers
    #[test]
    fn chain_increment_decrement_8bit_register() {
        let mut cpu = cpu::Cpu::new();

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.registers.write_byte(&RegByte::A, 255);
        cpu.registers.write_byte(&RegByte::B, 24);
        cpu.registers.write_byte(&RegByte::C, 33);
        cpu.registers.write_byte(&RegByte::D, 1);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 255);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_C);

        assert_eq!(cpu.registers.read_byte(&RegByte::C), 34);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::D), 0);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Bitwise OR Tests
    #[test]
    fn bitwise_or_of_a_b() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::OR_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 125);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn bitwise_or_of_a_d_zeroed() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        cpu.registers.write_byte(&RegByte::D, 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 0);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::OR_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Rotate A-Register left using RLCA or RLA
    #[test]
    fn old_rotate_register_a_left_producing_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b10101010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b10101010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b01010101);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn old_rotate_register_a_left_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0010_1010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0010_1010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0101_0100);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rotate_register_a_left_through_carry_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1110);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_left_through_carry_with_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0000_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0000_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0001_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0011_1110);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Rotate Right A-register
    #[test]
    fn old_rotate_register_a_right_through_carry_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0111_1110);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1110);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0011_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn rotate_register_right_with_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0111_1101);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0111_1101);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1011_1110);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn old_rotate_register_a_right_through_carry_with_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0000_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0000_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1100_0011);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Build 16-bit word out of 2 8-bit words
    #[test]
    fn build_2_byte_word_from_scratch() {
        let low_byte = 0b01101011;
        let high_byte = 0b11100001;
        let new_word = build_2_byte_word(low_byte, high_byte);

        assert_eq!(new_word, 0b1110000101101011);
        assert_eq!(new_word, 57707);
    }

    // Load Virtual Registers to A
    #[test]
    fn chain_load_virtual_registers_to_a() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers.write_word(&RegWord::HL, 9000);
        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::HL), 49);
        cpu.memory.write_byte(8999, 200);
        cpu.registers.write_byte(&RegByte::A, 10);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_HLdecrementedcontents);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 8999);
        assert_eq!(cpu.memory.read_byte(9000), 49);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 49);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_HLincrementedcontents);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 9000);
        assert_eq!(cpu.memory.read_byte(8999), 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);

        cpu.registers.write_word(&RegWord::BC, 4000);
        cpu.memory.write_byte(4000, 1);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_BCcontents);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 4000);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 1);

        cpu.registers.write_flag(RegFlag::Zero, true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);
    }

    // Compare to A tests
    #[test]
    fn compare_a_to_b() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::CP_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn compare_a_to_c_zeroed() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 99);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 99);
        cpu.registers.write_byte(&RegByte::B, 99);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 99);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::CP_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 99);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 99);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Test Virtual 16 bit register manipulations
    #[test]
    fn manipulate_hl_register() {
        let mut cpu = cpu::Cpu::new();

        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);

        cpu.registers.write_word(&RegWord::HL, 55151);
        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::HL), 241);

        cpu.registers.write_word(&RegWord::BC, 2001);
        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::BC), 25);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 55151);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            241
        );

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 2001);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            25
        );

        // Where HL should point to after ADD_HL_BC
        cpu.memory.write_byte(57152, 99);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_HL_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 57152);
        assert_eq!(cpu.memory.read_byte(55151), 241);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            99
        );
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            25
        );

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_HL);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 57153);
        // assert_eq!(
        //     cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
        //     100
        // );

        // Where HL should point after INC_HL_REG
        cpu.memory.write_byte(57153, 4);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_HL);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 57154);
        // assert_eq!(
        //     cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
        //     4
        // );

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.memory.write_byte(48772, 29);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_HL_HL);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 48772);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            29
        );

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Test Decrementing and Incrementing Virtual Registers
    #[test]
    fn decrement_sp_below_zero() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers
            .write_word(&RegWord::SP, 0b0000_0000_0000_0001);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_SP);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0b0000_0000_0000_0000);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_SP);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0b1111_1111_1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn old_increment_bc_and_overflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1110);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b1111_1111_1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::INC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0000_0000_0000_0000);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // TODO these names are getting a bit outta hand
    #[test]
    fn subtract_from_a_nop() {
        let mut cpu = cpu::Cpu::new();

        subtract_from_a(&mut cpu, 0);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_zero() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 1);
        subtract_from_a(&mut cpu, 1);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_half_borrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b10000);
        subtract_from_a(&mut cpu, 0b01111);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 1);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_borrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0000);
        subtract_from_a(&mut cpu, 0b1100_0000);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1100_0000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn subtract_from_a_borrow_half_borrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0000);
        subtract_from_a(&mut cpu, 0b1100_1000);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b10111000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn subtract_from_a_carry_nop() {
        let mut cpu = cpu::Cpu::new();

        subtract_from_a_carry(&mut cpu, 0);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_carry_nop_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);
        cpu.registers.write_byte(&RegByte::A, 1);
        subtract_from_a_carry(&mut cpu, 0);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_carry_zero() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 1);
        subtract_from_a_carry(&mut cpu, 1);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_carry_zero_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);
        cpu.registers.write_byte(&RegByte::A, 1);
        subtract_from_a_carry(&mut cpu, 0);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_carry_half_borrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b10000);
        subtract_from_a_carry(&mut cpu, 0b01111);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 1);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_carry_half_borrow_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);
        cpu.registers.write_byte(&RegByte::A, 0b10000);
        subtract_from_a_carry(&mut cpu, 0b01110);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 1);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn subtract_from_a_carry_borrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0000);
        subtract_from_a_carry(&mut cpu, 0b1100_0000);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1100_0000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn subtract_from_a_carry_borrow_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);
        cpu.registers.write_byte(&RegByte::A, 0b1000_0001);
        subtract_from_a_carry(&mut cpu, 0b1100_0000);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1100_0000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn subtract_from_a_carry_borrow_half_borrow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0000);
        subtract_from_a_carry(&mut cpu, 0b1100_1000);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b10111000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn subtract_from_a_carry_borrow_half_borrow_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Carry, true);
        cpu.registers.write_byte(&RegByte::A, 0b1000_0000);
        subtract_from_a_carry(&mut cpu, 0b1100_0111);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b10111000);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    // Testing Loading to Virtual 16-bit Registers
    #[test]
    fn old_load_a_to_bc() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1000_0001);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.registers
            .write_word(&RegWord::BC, 0b0100_1001_0000_1111);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0100_1001_0000_1111);

        cpu.memory
            .write_byte(cpu.registers.read_word(&RegWord::BC), 0b0000_0010);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            0b0000_0010
        );

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_BCcontents_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0001);
        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b0100_1001_0000_1111);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::BC)),
            0b1000_0001
        );
        assert_eq!(cpu.memory.read_byte(0b0100_1001_0000_1111), 0b1000_0001);
    }

    // Testing Chaining OpCodes
    #[test]
    fn chaining_commands_on_register_a() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::D, 43);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);

        cpu.registers.write_flag(RegFlag::Zero, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        cpu.registers.write_flag(RegFlag::Subtraction, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        cpu.registers.write_flag(RegFlag::Carry, false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::LD_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 43);

        // No flags change during load commands
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 86);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 172);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::DEC_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 171);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.registers.write_flag(RegFlag::Zero, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);

        cpu.registers.write_byte(&RegByte::B, 91);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 91);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::XOR_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 91);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 75);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 91);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_one_byte_opcode(&mut cpu, OneByteOpCode::SBC_A_D);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 31);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }
}
