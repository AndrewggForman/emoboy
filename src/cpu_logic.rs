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
            increment_virtual_register_ignore_flags(cpu, &RegWord::BC);

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
            decrement_virtual_register_ignore_flags(cpu, &RegWord::BC);
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
            increment_virtual_register_ignore_flags(cpu, &RegWord::DE);
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
            increment_virtual_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_HL => {
            increment_virtual_register_ignore_flags(cpu, &RegWord::HL);

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
            let value = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::A, value);
            increment_virtual_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_HL => {
            decrement_virtual_register_ignore_flags(cpu, &RegWord::HL);
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
            let value: u8 = !(get_byte_from_8bit_register(cpu, &RegByte::A));
            cpu.registers.write_byte(&RegByte::A, value);

            cpu.registers.write_flag(RegFlag::Subtraction, true);
            cpu.registers.write_flag(RegFlag::HalfCarry, true);
            cpu.clock.cycle_clock(1);
        }

        // 3x
        OneByteOpCode::LD_HLdecrementedcontents_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(cpu, value, &RegWord::HL);
            decrement_virtual_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_SP => {
            increment_virtual_register_ignore_flags(cpu, &RegWord::SP);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_HLcontents => {
            let (result, zero, half_carry) =
                increment_8bit(get_byte_from_virtual_register(cpu, &RegWord::HL));

            cpu.registers.write_flag(RegFlag::Zero, zero);
            cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            load_byte_to_virtual_register_target(cpu, result, &RegWord::HL);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_HLcontents => {
            let (result, zero, half_borrow) =
                decrement_8bit(get_byte_from_virtual_register(cpu, &RegWord::HL));

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
            let value = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::A, value);
            decrement_virtual_register_ignore_flags(cpu, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_SP => {
            decrement_virtual_register_ignore_flags(cpu, &RegWord::SP);
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

        // 4x
        OneByteOpCode::LD_B_B => {
            cpu.registers
                .write_byte(&RegByte::B, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_C => {
            cpu.registers
                .write_byte(&RegByte::B, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_D => {
            cpu.registers
                .write_byte(&RegByte::B, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_E => {
            cpu.registers
                .write_byte(&RegByte::B, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_H => {
            cpu.registers
                .write_byte(&RegByte::B, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_L => {
            cpu.registers
                .write_byte(&RegByte::B, cpu.registers.read_byte(&RegByte::L));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::B, value);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_B_A => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_B => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_C => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_D => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_E => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_H => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_L => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::L));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::C, value);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_C_A => {
            cpu.registers
                .write_byte(&RegByte::C, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }

        // 5x
        OneByteOpCode::LD_D_B => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_C => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_D => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_E => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_H => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_L => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::D, value);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_D_A => {
            cpu.registers
                .write_byte(&RegByte::D, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_B => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_C => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_D => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_E => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_H => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_L => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::L));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::E, value);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_E_A => {
            cpu.registers
                .write_byte(&RegByte::E, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }

        // 6x
        OneByteOpCode::LD_H_B => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_C => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_D => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_E => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_H => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_L => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::L));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::H, value);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_H_A => {
            cpu.registers
                .write_byte(&RegByte::H, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_B => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_C => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_D => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_E => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_H => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_L => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::L));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::L, value);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_L_A => {
            cpu.registers
                .write_byte(&RegByte::L, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }

        // 7x
        OneByteOpCode::LD_HLcontents_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::HALT => {
            // TODO: Implement halt. Needs IME flag and state of pending interrupts ([IE] & [IF] status)
            // TODO: Talk with tint about how to handle unimplemnted instructions (panic fine?)
            panic!("ERROR::Attempted to perform a HALT. Not yet implemented. ");
        }
        OneByteOpCode::LD_HLcontents_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            load_byte_to_virtual_register_target(cpu, byte, &RegWord::HL);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_B => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::B));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_C => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::C));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_D => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::D));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_E => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::E));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_H => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::H));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_L => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::L));
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            cpu.registers.write_byte(&RegByte::A, byte);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_A => {
            cpu.registers
                .write_byte(&RegByte::A, cpu.registers.read_byte(&RegByte::A));
            cpu.clock.cycle_clock(1);
        }

        // 8x
        // add_byte_to8bit_register() also handles flag setting
        OneByteOpCode::ADD_A_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::ADD_A_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            add_byte_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::ADC_A_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            add_byte_and_carry_to_8bit_register(cpu, byte, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }

        // 9x
        // subtract_byte_from_8bit_register() also handles setting flags
        OneByteOpCode::SUB_A_B => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_C => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_D => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_E => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_H => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_L => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_HLcontents => {
            let subtrahend: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::SUB_A_A => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            subtract_byte_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_B => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_C => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_D => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_E => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_H => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_L => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_HLcontents => {
            let subtrahend: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::SBC_A_A => {
            let subtrahend: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            subtract_byte_and_carry_from_8bit_register(cpu, subtrahend, &RegByte::A);
            cpu.clock.cycle_clock(1);
        }

        // Ax
        // bitwise_byte_and_a() also handles flag setting
        OneByteOpCode::AND_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::AND_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            bitwise_byte_and_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        // bitwise_byte_xor_a() also handles flag setting
        OneByteOpCode::XOR_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::XOR_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            bitwise_byte_xor_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }

        // Bx
        OneByteOpCode::OR_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::OR_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            bitwise_byte_or_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_B => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::B);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_C => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::C);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_D => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::D);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_E => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::E);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_H => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::H);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_L => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::L);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(cpu, &RegWord::HL);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(2);
        }
        OneByteOpCode::CP_A => {
            let byte: u8 = get_byte_from_8bit_register(cpu, &RegByte::A);
            compare_byte_to_a(cpu, byte);
            cpu.clock.cycle_clock(1);
        }

        // Cx
        // Notes: Incrementing twice because stack pointer stores pointer to 8 byte segments.
        // Notes: -> Popping increments pointer because as the pointer grows it actually moves to lower addresses
        // TODO: I NEED TO LOOK INTO THIS -> how we store the stack pointer/program counter... and endianness
        // TODO:
        // z/nc/nz etc... info -> https://learn.cemetech.net/index.php?title=Z80:Opcodes:RET
        OneByteOpCode::RET_NZ => {
            let word: u16 = get_word_from_16bit_register(cpu, &RegWord::SP);
            load_word_to_16bit_register(cpu, word, &RegWord::PC);
            increment_virtual_register_ignore_flags(cpu, &RegWord::SP);
            increment_virtual_register_ignore_flags(cpu, &RegWord::SP);

            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.clock.cycle_clock(4);
        }
        OneByteOpCode::POP_BC => {}
        OneByteOpCode::PUSH_BC => {}
        OneByteOpCode::RST_00H => {}
        OneByteOpCode::RET_Z => {}
        OneByteOpCode::RET => {}
        OneByteOpCode::PREFIX_CB => {} // SPECIAL PREFIX
        OneByteOpCode::RST_08H => {}
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
// Load/Get Helper Functions
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

pub fn get_byte_from_virtual_register(cpu: &mut cpu::Cpu, virtual_register: &RegWord) -> u8 {
    let value = cpu
        .memory
        .read_byte(cpu.registers.read_word(virtual_register));
    return value;
}

pub fn get_byte_from_8bit_register(cpu: &mut cpu::Cpu, register: &RegByte) -> u8 {
    return cpu.registers.read_byte(register);
}

pub fn get_word_from_16bit_register(cpu: &mut cpu::Cpu, register: &RegWord) -> u16 {
    return cpu.registers.read_word(register);
}

pub fn load_word_to_16bit_register(cpu: &mut cpu::Cpu, word: u16, register: &RegWord) {
    cpu.registers.write_word(register, word);
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

pub fn increment_virtual_register_ignore_flags(cpu: &mut cpu::Cpu, virtual_register: &RegWord) {
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

pub fn decrement_virtual_register_ignore_flags(cpu: &mut cpu::Cpu, virtual_register: &RegWord) {
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

pub fn add_byte_to_8bit_register(cpu: &mut cpu::Cpu, new_value: u8, register: &RegByte) {
    let initial_value = cpu.registers.read_byte(register);
    let (result, carry) = initial_value.overflowing_add(new_value);

    // check if we would have initial_value
    let half_carry = calculate_8bit_half_carry(initial_value, new_value);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
    cpu.registers.write_flag(RegFlag::Carry, carry);

    cpu.registers.write_byte(register, result);
}

pub fn add_byte_and_carry_to_8bit_register(cpu: &mut cpu::Cpu, new_value: u8, register: &RegByte) {
    if !cpu.registers.read_flag(RegFlag::Carry) {
        add_byte_to_8bit_register(cpu, new_value, register);
        return;
    }

    let initial_value = cpu.registers.read_byte(register);
    let (result, is_overflow) = initial_value.overflowing_add(new_value);

    // Check for half carry between A + B, then result + carry
    let half_carry = calculate_8bit_half_carry(initial_value, new_value);
    let half_carry2 = calculate_8bit_half_carry(result, 1);

    let (result2, is_overflow2) = result.overflowing_add(1);

    cpu.registers.write_flag(RegFlag::Zero, result2 == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers
        .write_flag(RegFlag::HalfCarry, half_carry | half_carry2);
    cpu.registers
        .write_flag(RegFlag::Carry, is_overflow | is_overflow2);

    cpu.registers.write_byte(register, result2);
}

// Subtract Helper Functions
pub fn subtract_byte_from_8bit_register(cpu: &mut cpu::Cpu, subtrahend: u8, register: &RegByte) {
    let initial_value = cpu.registers.read_byte(register);
    let (result, borrow) = initial_value.overflowing_sub(subtrahend);

    // check if we would have to borrow from the 5th bit
    let half_borrow = (initial_value & 0xF) < (subtrahend & 0xF);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_borrow);
    cpu.registers.write_flag(RegFlag::Carry, borrow);

    cpu.registers.write_byte(register, result);
}

pub fn subtract_byte_and_carry_from_8bit_register(
    cpu: &mut cpu::Cpu,
    subtrahend: u8,
    register: &RegByte,
) {
    if !cpu.registers.read_flag(RegFlag::Carry) {
        subtract_byte_from_8bit_register(cpu, subtrahend, register);
        return;
    }

    let initial_value = cpu.registers.read_byte(&RegByte::A);
    let result = initial_value.wrapping_sub(subtrahend).wrapping_sub(1);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (initial_value & 0xF) < ((subtrahend & 0xF) + 1);
    let is_borrow = subtrahend == 0xFF || initial_value < (subtrahend + 1);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_flag(RegFlag::HalfCarry, is_half_borrow);
    cpu.registers.write_flag(RegFlag::Carry, is_borrow);

    cpu.registers.write_byte(register, result);
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

// Logical Helper Functions
// (Typically A is the targeted register implicitly)
pub fn bitwise_byte_and_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value & value;

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, true);
    cpu.registers.write_flag(RegFlag::Carry, false);

    cpu.registers.write_byte(&RegByte::A, result);
}

pub fn bitwise_byte_xor_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value ^ value;

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, false);
    cpu.registers.write_flag(RegFlag::Carry, false);

    cpu.registers.write_byte(&RegByte::A, result);
}

pub fn bitwise_byte_or_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value | value;

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, false);
    cpu.registers.write_flag(RegFlag::Carry, false);

    cpu.registers.write_byte(&RegByte::A, result);
}

pub fn compare_byte_to_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let (result, is_borrow) = a_value.overflowing_sub(value);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (a_value & 0xF) < (value & 0xF);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_flag(RegFlag::HalfCarry, is_half_borrow);
    cpu.registers.write_flag(RegFlag::Carry, is_borrow);
}

// Building helper function for words from bytes (16bit stuff made up of 2 8bit pieces)
// TODO: Finish after deciding how memory/endianness we want
// Todo: -> (keep our system big endian, or swap it to little endian to match gameboy?)
pub fn get_word_from_high_and_low_byte(high_byte: u8, low_byte: u8) -> u16 {
    let mut high_mask: u16 = high_byte.into();
    high_mask = high_mask << 8;
    let mut low_mask: u16 = low_byte.into();

    10
}
