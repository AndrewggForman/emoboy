use std::iter::Cycle;

use crate::clock;
use crate::cpu;
use crate::opcode::{OneByteOpCode, ThreeByteOpCode, TwoByteOpCode};
use crate::registers::{self, RegByte, RegFlag, RegWord};

// TODO: Add OpCodes. Refactor tests/build new ones. Implement clock cycles for each OpCode/fix clock cyles.

pub fn execute_one_byte_opcode(cpu: &mut cpu::Cpu, code: OneByteOpCode) {
    match code {
        // 0x
        OneByteOpCode::NOP => {
            // TODO:
            // CHECK LATER & ADD TESTS
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_BCcontents_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(cpu, value, &RegWord::BC);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_BC => {
            increment_16bit_register_ignore_flags(cpu, &RegWord::BC);

            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_B => {
            increment_8bit_register(cpu, &RegByte::B);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_B => {
            decrement_8bit_register(cpu, &RegByte::B);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::RLCA => {
            let byte = cpu.registers.read_byte(&RegByte::A);
            let bit_7: u8 = 0x80 & byte;
            let byte_rotated_left = byte.rotate_left(1);

            cpu.registers.write_flag(RegFlag::Carry, bit_7 == 0x80);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers.write_byte(&RegByte::A, byte_rotated_left);

            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_HL_BC => {
            add_virtual_register_from_virtual_register(cpu, &RegWord::HL, &RegWord::BC);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_BCcontents => {
            load_8bit_register_from_virtual_register(cpu, &RegByte::A, &RegWord::BC);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_BC => {
            decrement_16bit_register_ignore_flags(cpu, &RegWord::BC);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_C => {
            increment_8bit_register(cpu, &RegByte::C);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_C => {
            decrement_8bit_register(cpu, &RegByte::C);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::RRCA => {
            let byte = cpu.registers.read_byte(&RegByte::A);
            let bit_0: u8 = 0x01 & byte;
            let byte_rotated_right = byte.rotate_right(1);

            cpu.registers.write_flag(RegFlag::Carry, bit_0 == 0x01);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers.write_byte(&RegByte::A, byte_rotated_right);

            cpu.clock.cycle_clock(1);
        }

        // 1x
        OneByteOpCode::LD_DEcontents_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(cpu, value, &RegWord::DE);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_DE => {
            increment_16bit_register_ignore_flags(cpu, &RegWord::DE);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_D => {
            increment_8bit_register(cpu, &RegByte::D);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_D => {
            decrement_8bit_register(cpu, &RegByte::D);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::RLA => {
            let byte = cpu.registers.read_byte(&RegByte::A);
            let bit_7: u8 = 0x80 & byte;
            let carry_bit: u8 = if cpu.registers.read_flag(RegFlag::Carry) == true {
                1
            } else {
                0
            };

            let byte_rotated_left = byte.rotate_left(1);
            let byte_rotated_left_through_carry = { carry_bit | (byte_rotated_left & 0xFE) };

            cpu.registers.write_flag(RegFlag::Carry, bit_7 == 0x80);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers
                .write_byte(&RegByte::A, byte_rotated_left_through_carry);

            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_HL_DE => {
            add_virtual_register_from_virtual_register(cpu, &RegWord::HL, &RegWord::DE);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_DEcontents => {
            load_8bit_register_from_virtual_register(cpu, &RegByte::A, &RegWord::DE);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_DE => {
            let value = cpu.registers.read_word(&RegWord::DE);
            cpu.registers
                .write_word(&RegWord::BC, value.wrapping_sub(1));

            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_E => {
            increment_8bit_register(cpu, &RegByte::E);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_E => {
            decrement_8bit_register(cpu, &RegByte::E);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::RRA => {
            let byte = cpu.registers.read_byte(&RegByte::A);
            let bit_0: u8 = 0x01 & byte;
            let bit_mask: u8;
            let carry_bit: u8 = if cpu.registers.read_flag(RegFlag::Carry) == true {
                bit_mask = 0b1000_0000;
                1
            } else {
                bit_mask = 0b0000_0000;
                0
            };

            let byte_rotated_right = byte.rotate_right(1);
            let byte_rotated_right_through_carry =
                { (byte_rotated_right & 0b0111_1111) | bit_mask };

            cpu.registers.write_flag(RegFlag::Carry, bit_0 == 0x01);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers
                .write_byte(&RegByte::A, byte_rotated_right_through_carry);

            cpu.clock.cycle_clock(1);
        }

        // 2x
        OneByteOpCode::LD_HLincrementedcontents_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(cpu, value, &RegWord::HL);
            increment_16bit_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_HL => {
            increment_16bit_register_ignore_flags(cpu, &RegWord::HL);

            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_H => {
            increment_8bit_register(cpu, &RegByte::H);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_H => {
            decrement_8bit_register(cpu, &RegByte::H);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DAA => match cpu.registers.read_flag(RegFlag::Subtraction) {
            true => {
                let mut adjustment: u8 = 0;
                if cpu.registers.read_flag(RegFlag::HalfCarry) {
                    adjustment += 0x06;
                }
                if cpu.registers.read_flag(RegFlag::Carry) {
                    adjustment += 0x60;
                }
                let result = cpu
                    .registers
                    .read_byte(&RegByte::A)
                    .wrapping_sub(adjustment);
                let zero = result == 0;

                cpu.registers.write_byte(&RegByte::A, result);
                cpu.registers.write_flag(RegFlag::Zero, zero);
                cpu.registers.write_flag(RegFlag::HalfCarry, false);
                cpu.clock.cycle_clock(1);
            }
            false => {
                let mut adjustment: u8 = 0;
                let a_byte = cpu.registers.read_byte(&RegByte::A);
                let low_bit_mask: bool = (a_byte & 0x0f) > 0x09;
                let high_bit_mask: bool = a_byte > 0x99;
                if cpu.registers.read_flag(RegFlag::HalfCarry) || low_bit_mask {
                    adjustment += 0x06;
                }
                if cpu.registers.read_flag(RegFlag::Carry) || high_bit_mask {
                    adjustment += 0x60;
                    cpu.registers.write_flag(RegFlag::Carry, true);
                }
                let result = a_byte.wrapping_add(adjustment);
                let zero = result == 0;

                cpu.registers.write_byte(&RegByte::A, result);
                cpu.registers.write_flag(RegFlag::Zero, zero);
                cpu.registers.write_flag(RegFlag::HalfCarry, false);
                cpu.clock.cycle_clock(1);
            }
        },
        OneByteOpCode::ADD_HL_HL => {
            add_virtual_register_from_virtual_register(cpu, &RegWord::HL, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_HLincrementedcontents => {
            let value = get_8bit_value_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::A, value);
            increment_16bit_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_HL => {
            decrement_16bit_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }

        OneByteOpCode::INC_L => {
            increment_8bit_register(cpu, &RegByte::L);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_L => {
            decrement_8bit_register(cpu, &RegByte::L);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CPL => {
            // Complement Accumulator (bitwise not of A)
            let value: u8 = !(load_byte_from_8bit_register(cpu, &RegByte::A));
            cpu.registers.write_byte(&RegByte::A, value);

            cpu.registers.write_flag(RegFlag::Subtraction, true);
            cpu.registers.write_flag(RegFlag::HalfCarry, true);
            cpu.clock.cycle_clock(1);
        }

        // 3x
        OneByteOpCode::LD_HLdecrementedcontents_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(cpu, value, &RegWord::HL);
            decrement_16bit_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_SP => {
            increment_16bit_register_ignore_flags(cpu, &RegWord::SP);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_HLcontents => {
            let (result, zero, half_carry) =
                increment_8bit(get_8bit_value_from_virtual_register(cpu, &RegWord::HL));

            cpu.registers.write_flag(RegFlag::Zero, zero);
            cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            load_byte_to_virtual_register_target(cpu, result, &RegWord::HL);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_HLcontents => {
            let (result, zero, half_borrow) =
                decrement_8bit(get_8bit_value_from_virtual_register(cpu, &RegWord::HL));

            cpu.registers.write_flag(RegFlag::Zero, zero);
            cpu.registers.write_flag(RegFlag::HalfCarry, half_borrow);
            cpu.registers.write_flag(RegFlag::Subtraction, true);
            load_byte_to_virtual_register_target(cpu, result, &RegWord::HL);
            cpu.clock.cycle_clock(1);
        }
        //TODO
        OneByteOpCode::SCF => {
            cpu.registers.write_flag(RegFlag::Carry, true);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_HL_SP => {
            add_virtual_register_from_virtual_register(cpu, &RegWord::HL, &RegWord::SP);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_HLdecrementedcontents => {
            let value = get_8bit_value_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::A, value);
            decrement_16bit_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_SP => {
            decrement_16bit_register_ignore_flags(cpu, &RegWord::SP);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_A => {
            increment_8bit_register(cpu, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_A => {
            decrement_8bit_register(cpu, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CCF => {
            // Complement (inverse) of carry flag
            cpu.registers
                .write_flag(RegFlag::Carry, !(cpu.registers.read_flag(RegFlag::Carry)));
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.clock.cycle_clock(1);
        }

        _ => panic!("ERROR::Invalid One Byte OpCode! Yoinked by Jaguar Claw!"),
    }
}

pub fn execute_two_byte_opcode(cpu: &mut cpu::Cpu, code: TwoByteOpCode, byte1: u8) {
    match code {
        _ => panic!("ERROR::Invalid Two Byte OpCode! Yoinked by Lobster Claw!"),
    }
}

pub fn execute_three_byte_opcode(cpu: &mut cpu::Cpu, code: ThreeByteOpCode, byte1: u8, byte2: u8) {
    match code {
        _ => panic!("ERROR::Invalid Three Byte OpCode! Yoinked by Hawk Claw!"),
    }
}

// Helper Functions
// Load Helper Functions
pub fn load_byte_to_virtual_register_target(
    cpu: &mut cpu::Cpu,
    byte: u8,
    virtual_register: &RegWord,
) {
    let address = cpu.registers.read_word(virtual_register);
    cpu.memory.write_byte(address, byte);
}

pub fn load_8bit_register_from_virtual_register(
    cpu: &mut cpu::Cpu,
    register: &RegByte,
    virtual_register: &RegWord,
) {
    let value = cpu
        .memory
        .read_byte(cpu.registers.read_word(virtual_register));
    cpu.registers.write_byte(register, value);
}

pub fn get_8bit_value_from_virtual_register(cpu: &mut cpu::Cpu, virtual_register: &RegWord) -> u8 {
    let value = cpu
        .memory
        .read_byte(cpu.registers.read_word(virtual_register));
    return value;
}

pub fn load_byte_from_8bit_register(cpu: &mut cpu::Cpu, register: &RegByte) -> u8 {
    return cpu.registers.read_byte(register);
}

// Increment Helper Functions
pub fn increment_8bit_register(cpu: &mut cpu::Cpu, register: &RegByte) {
    let value = cpu.registers.read_byte(register);
    let (result, zero, half_carry) = increment_8bit(value);
    cpu.registers.write_flag(RegFlag::Zero, zero);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_byte(register, result);
}

pub fn increment_8bit(value: u8) -> (u8, bool, bool) {
    let result = value.wrapping_add(1);

    // check if we would have a carry
    let half_carry = calculate_8bit_half_carry(value, 1);

    let zero = result == 0;

    (result, zero, half_carry)
}

pub fn increment_16bit_register_ignore_flags(cpu: &mut cpu::Cpu, virtual_register: &RegWord) {
    let value: u16 = cpu.registers.read_word(virtual_register);
    let result: u16 = value.wrapping_add(1);
    cpu.registers.write_word(virtual_register, result);
}

//Decrement Helper Functions
pub fn decrement_8bit_register(cpu: &mut cpu::Cpu, register: &RegByte) {
    let value = cpu.registers.read_byte(register);
    let (result, zero, half_borrow) = decrement_8bit(value);
    cpu.registers.write_flag(RegFlag::Zero, zero);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_borrow);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_byte(register, result);
}

pub fn decrement_8bit(value: u8) -> (u8, bool, bool) {
    let (result, is_borrow) = value.overflowing_sub(1);
    let zero = result == 0;

    // TODO: THINK CORRECT -> talk to tint/look at it with someone
    let is_half_borrow =
        (((result & 0x10) == 0) && ((result & 0x08) == 0x08)) && ((value & 0x10) == 0x10);

    (result, zero, is_half_borrow)
}

pub fn decrement_16bit_register_ignore_flags(cpu: &mut cpu::Cpu, virtual_register: &RegWord) {
    let value: u16 = cpu.registers.read_word(virtual_register);
    let result: u16 = value.wrapping_sub(1);
    cpu.registers.write_word(virtual_register, result);
}

// Add Helper Functions
pub fn add_virtual_register_from_virtual_register(
    cpu: &mut cpu::Cpu,
    receiver: &RegWord,
    sender: &RegWord,
) {
    // TODO: Maybe look over later, seems fine
    let receiver_value = cpu.registers.read_word(receiver);
    let sender_value = cpu.registers.read_word(&sender);
    let (result, half_carry, carry) = add_16bit_and_16bit(receiver_value, sender_value);

    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
    cpu.registers.write_flag(RegFlag::Carry, carry);

    cpu.registers.write_word(receiver, result);
}

fn add_16bit_and_16bit(num1: u16, num2: u16) -> (u16, bool, bool) {
    let (result, is_carry) = num1.overflowing_add(num2);

    // check if we would have a carry
    let is_half_carry = calculate_16bit_half_carry(num1, num2);

    (result, is_half_carry, is_carry)
}

// Calculate Helper Functions
pub fn calculate_8bit_half_carry(byte1: u8, byte2: u8) -> bool {
    let half_carry = (((byte1 & 0xF) + (byte2 & 0xF)) & 0x10) == 0x10;
    half_carry
}

pub fn calculate_16bit_half_carry(word1: u16, word2: u16) -> bool {
    let half_carry = (((word1 & 0xFFF) + (word2 & 0xFFF)) & 0x1000) == 0x1000;
    half_carry
}
