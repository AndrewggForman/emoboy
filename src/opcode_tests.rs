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
}
