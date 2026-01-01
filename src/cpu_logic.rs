// use std::iter::Cycle;

use crate::clock;
use crate::motherboard;
use crate::opcode::{OneByteOpCode, ThreeByteOpCode, TwoByteOpCode};
use crate::registers::{RegByte, RegFlag, RegWord};

// TODO: Add OpCodes. Refactor tests/build new ones. Implement clock cycles for each OpCode/fix clock cyles.
// TODO: Increment Program Counter properly!!! Only stack related operations currently interact with program counter
// TODO: Might need to fix ime related commands because they are supposed to go into effect: AFTER the NEXT instruction
// TODO: !!!Rewrite with motherboard instead of motherboard.!!!

pub fn execute_one_byte_opcode(motherboard: &mut motherboard::Motherboard, code: OneByteOpCode) {
    match code {
        // 0x
        OneByteOpCode::NOP => {
            // TODO:
            // CHECK LATER & ADD TESTS
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_BCcontents_A => {
            let value = motherboard.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(motherboard, value, &RegWord::BC);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_BC => {
            increment_virtual_register_ignore_flags(motherboard, &RegWord::BC);

            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_B => {
            increment_8bit_register(motherboard, &RegByte::B);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_B => {
            decrement_8bit_register(motherboard, &RegByte::B);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::RLCA => {
            let byte = motherboard.registers.read_byte(&RegByte::A);
            let bit_7: u8 = 0x80 & byte;
            let byte_rotated_left = byte.rotate_left(1);

            motherboard
                .registers
                .write_flag(RegFlag::Carry, bit_7 == 0x80);
            motherboard.registers.write_flag(RegFlag::HalfCarry, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.registers.write_flag(RegFlag::Zero, false);
            motherboard
                .registers
                .write_byte(&RegByte::A, byte_rotated_left);

            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_HL_BC => {
            add_virtual_register_from_virtual_register(motherboard, &RegWord::HL, &RegWord::BC);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_BCcontents => {
            load_8bit_register_from_virtual_register(motherboard, &RegByte::A, &RegWord::BC);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_BC => {
            decrement_virtual_register_ignore_flags(motherboard, &RegWord::BC);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_C => {
            increment_8bit_register(motherboard, &RegByte::C);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_C => {
            decrement_8bit_register(motherboard, &RegByte::C);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::RRCA => {
            let byte = motherboard.registers.read_byte(&RegByte::A);
            let bit_0: u8 = 0x01 & byte;
            let byte_rotated_right = byte.rotate_right(1);

            motherboard
                .registers
                .write_flag(RegFlag::Carry, bit_0 == 0x01);
            motherboard.registers.write_flag(RegFlag::HalfCarry, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.registers.write_flag(RegFlag::Zero, false);
            motherboard
                .registers
                .write_byte(&RegByte::A, byte_rotated_right);

            motherboard.clock.cycle_clock(1);
        }

        // 1x
        OneByteOpCode::LD_DEcontents_A => {
            let value = motherboard.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(motherboard, value, &RegWord::DE);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_DE => {
            increment_virtual_register_ignore_flags(motherboard, &RegWord::DE);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_D => {
            increment_8bit_register(motherboard, &RegByte::D);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_D => {
            decrement_8bit_register(motherboard, &RegByte::D);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::RLA => {
            let byte = motherboard.registers.read_byte(&RegByte::A);
            let bit_7: u8 = 0x80 & byte;
            let carry_bit: u8 = if motherboard.registers.read_flag(RegFlag::Carry) == true {
                1
            } else {
                0
            };

            let byte_rotated_left = byte.rotate_left(1);
            let byte_rotated_left_through_carry = { carry_bit | (byte_rotated_left & 0xFE) };

            motherboard
                .registers
                .write_flag(RegFlag::Carry, bit_7 == 0x80);
            motherboard.registers.write_flag(RegFlag::HalfCarry, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.registers.write_flag(RegFlag::Zero, false);
            motherboard
                .registers
                .write_byte(&RegByte::A, byte_rotated_left_through_carry);

            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_HL_DE => {
            add_virtual_register_from_virtual_register(motherboard, &RegWord::HL, &RegWord::DE);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_DEcontents => {
            load_8bit_register_from_virtual_register(motherboard, &RegByte::A, &RegWord::DE);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_DE => {
            let value = motherboard.registers.read_word(&RegWord::DE);
            motherboard
                .registers
                .write_word(&RegWord::BC, value.wrapping_sub(1));

            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_E => {
            increment_8bit_register(motherboard, &RegByte::E);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_E => {
            decrement_8bit_register(motherboard, &RegByte::E);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::RRA => {
            let byte = motherboard.registers.read_byte(&RegByte::A);
            let bit_0: u8 = 0x01 & byte;
            let bit_mask: u8;
            let carry_bit: u8 = if motherboard.registers.read_flag(RegFlag::Carry) == true {
                bit_mask = 0b1000_0000;
                1
            } else {
                bit_mask = 0b0000_0000;
                0
            };

            let byte_rotated_right = byte.rotate_right(1);
            let byte_rotated_right_through_carry =
                { (byte_rotated_right & 0b0111_1111) | bit_mask };

            motherboard
                .registers
                .write_flag(RegFlag::Carry, bit_0 == 0x01);
            motherboard.registers.write_flag(RegFlag::HalfCarry, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.registers.write_flag(RegFlag::Zero, false);
            motherboard
                .registers
                .write_byte(&RegByte::A, byte_rotated_right_through_carry);

            motherboard.clock.cycle_clock(1);
        }

        // 2x
        OneByteOpCode::LD_HLincrementedcontents_A => {
            let value = motherboard.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(motherboard, value, &RegWord::HL);
            increment_virtual_register_ignore_flags(motherboard, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_HL => {
            increment_virtual_register_ignore_flags(motherboard, &RegWord::HL);

            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_H => {
            increment_8bit_register(motherboard, &RegByte::H);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_H => {
            decrement_8bit_register(motherboard, &RegByte::H);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DAA => match motherboard.registers.read_flag(RegFlag::Subtraction) {
            true => {
                let mut adjustment: u8 = 0;
                if motherboard.registers.read_flag(RegFlag::HalfCarry) {
                    adjustment += 0x06;
                }
                if motherboard.registers.read_flag(RegFlag::Carry) {
                    adjustment += 0x60;
                }
                let result = motherboard
                    .registers
                    .read_byte(&RegByte::A)
                    .wrapping_sub(adjustment);
                let zero = result == 0;

                motherboard.registers.write_byte(&RegByte::A, result);
                motherboard.registers.write_flag(RegFlag::Zero, zero);
                motherboard.registers.write_flag(RegFlag::HalfCarry, false);
                motherboard.clock.cycle_clock(1);
            }
            false => {
                let mut adjustment: u8 = 0;
                let a_byte = motherboard.registers.read_byte(&RegByte::A);
                let low_bit_mask: bool = (a_byte & 0x0f) > 0x09;
                let high_bit_mask: bool = a_byte > 0x99;
                if motherboard.registers.read_flag(RegFlag::HalfCarry) || low_bit_mask {
                    adjustment += 0x06;
                }
                if motherboard.registers.read_flag(RegFlag::Carry) || high_bit_mask {
                    adjustment += 0x60;
                    motherboard.registers.write_flag(RegFlag::Carry, true);
                }
                let result = a_byte.wrapping_add(adjustment);
                let zero = result == 0;

                motherboard.registers.write_byte(&RegByte::A, result);
                motherboard.registers.write_flag(RegFlag::Zero, zero);
                motherboard.registers.write_flag(RegFlag::HalfCarry, false);
                motherboard.clock.cycle_clock(1);
            }
        },
        OneByteOpCode::ADD_HL_HL => {
            add_virtual_register_from_virtual_register(motherboard, &RegWord::HL, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_HLincrementedcontents => {
            let value = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::A, value);
            increment_virtual_register_ignore_flags(motherboard, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_HL => {
            decrement_virtual_register_ignore_flags(motherboard, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }

        OneByteOpCode::INC_L => {
            increment_8bit_register(motherboard, &RegByte::L);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_L => {
            decrement_8bit_register(motherboard, &RegByte::L);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CPL => {
            // Complement Accumulator (bitwise not of A)
            let value: u8 = !(get_byte_from_8bit_register(motherboard, &RegByte::A));
            motherboard.registers.write_byte(&RegByte::A, value);

            motherboard.registers.write_flag(RegFlag::Subtraction, true);
            motherboard.registers.write_flag(RegFlag::HalfCarry, true);
            motherboard.clock.cycle_clock(1);
        }

        // 3x
        OneByteOpCode::LD_HLdecrementedcontents_A => {
            let value = motherboard.registers.read_byte(&RegByte::A);
            load_byte_to_virtual_register_target(motherboard, value, &RegWord::HL);
            decrement_virtual_register_ignore_flags(motherboard, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_SP => {
            increment_virtual_register_ignore_flags(motherboard, &RegWord::SP);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_HLcontents => {
            let (result, zero, half_carry) =
                increment_8bit(get_byte_from_virtual_register(motherboard, &RegWord::HL));

            motherboard.registers.write_flag(RegFlag::Zero, zero);
            motherboard
                .registers
                .write_flag(RegFlag::HalfCarry, half_carry);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            load_byte_to_virtual_register_target(motherboard, result, &RegWord::HL);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_HLcontents => {
            let (result, zero, half_borrow) =
                decrement_8bit(get_byte_from_virtual_register(motherboard, &RegWord::HL));

            motherboard.registers.write_flag(RegFlag::Zero, zero);
            motherboard
                .registers
                .write_flag(RegFlag::HalfCarry, half_borrow);
            motherboard.registers.write_flag(RegFlag::Subtraction, true);
            load_byte_to_virtual_register_target(motherboard, result, &RegWord::HL);
            motherboard.clock.cycle_clock(1);
        }
        //TODO
        OneByteOpCode::SCF => {
            motherboard.registers.write_flag(RegFlag::Carry, true);
            motherboard.registers.write_flag(RegFlag::HalfCarry, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_HL_SP => {
            add_virtual_register_from_virtual_register(motherboard, &RegWord::HL, &RegWord::SP);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_HLdecrementedcontents => {
            let value = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::A, value);
            decrement_virtual_register_ignore_flags(motherboard, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::DEC_SP => {
            decrement_virtual_register_ignore_flags(motherboard, &RegWord::SP);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::INC_A => {
            increment_8bit_register(motherboard, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::DEC_A => {
            decrement_8bit_register(motherboard, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CCF => {
            // Complement (inverse) of carry flag
            motherboard.registers.write_flag(
                RegFlag::Carry,
                !(motherboard.registers.read_flag(RegFlag::Carry)),
            );
            motherboard.registers.write_flag(RegFlag::HalfCarry, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.clock.cycle_clock(1);
        }

        // 4x
        OneByteOpCode::LD_B_B => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_C => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_D => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_E => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_H => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_L => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::L));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_B_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::B, value);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_B_A => {
            motherboard
                .registers
                .write_byte(&RegByte::B, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_B => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_C => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_D => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_E => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_H => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_L => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::L));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_C_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::C, value);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_C_A => {
            motherboard
                .registers
                .write_byte(&RegByte::C, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }

        // 5x
        OneByteOpCode::LD_D_B => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_C => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_D => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_E => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_H => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_L => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_D_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::D, value);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_D_A => {
            motherboard
                .registers
                .write_byte(&RegByte::D, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_B => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_C => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_D => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_E => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_H => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_L => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::L));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_E_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::E, value);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_E_A => {
            motherboard
                .registers
                .write_byte(&RegByte::E, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }

        // 6x
        OneByteOpCode::LD_H_B => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_C => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_D => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_E => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_H => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_L => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::L));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_H_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::H, value);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_H_A => {
            motherboard
                .registers
                .write_byte(&RegByte::H, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_B => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_C => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_D => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_E => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_H => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_L => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::L));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_L_HLcontents => {
            let value: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::L, value);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_L_A => {
            motherboard
                .registers
                .write_byte(&RegByte::L, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }

        // 7x
        OneByteOpCode::LD_HLcontents_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_HLcontents_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::HALT => {
            // TODO: Implement halt. Needs IME flag and state of pending interrupts ([IE] & [IF] status)
            // TODO: Talk with tint about how to handle unimplemnted instructions (panic fine?)
            panic!("ERROR::Attempted to perform a HALT. Not yet implemented. ");
        }
        OneByteOpCode::LD_HLcontents_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            load_byte_to_virtual_register_target(motherboard, byte, &RegWord::HL);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_B => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::B));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_C => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::C));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_D => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::D));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_E => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::E));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_H => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::H));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_L => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::L));
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::LD_A_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            motherboard.registers.write_byte(&RegByte::A, byte);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::LD_A_A => {
            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(1);
        }

        // 8x
        // add_byte_to8bit_register() also handles flag setting
        OneByteOpCode::ADD_A_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADD_A_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::ADD_A_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            add_byte_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::ADC_A_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::ADC_A_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            add_byte_and_carry_to_8bit_register(motherboard, byte, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }

        // 9x
        // subtract_byte_from_8bit_register() also handles setting flags
        OneByteOpCode::SUB_A_B => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_C => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_D => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_E => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_H => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_L => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SUB_A_HLcontents => {
            let subtrahend: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::SUB_A_A => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            subtract_byte_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_B => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_C => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_D => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_E => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_H => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_L => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::SBC_A_HLcontents => {
            let subtrahend: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::SBC_A_A => {
            let subtrahend: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            subtract_byte_and_carry_from_8bit_register(motherboard, subtrahend, &RegByte::A);
            motherboard.clock.cycle_clock(1);
        }

        // Ax
        // bitwise_byte_and_a() also handles flag setting
        OneByteOpCode::AND_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::AND_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::AND_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            bitwise_byte_and_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        // bitwise_byte_xor_a() also handles flag setting
        OneByteOpCode::XOR_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::XOR_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::XOR_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            bitwise_byte_xor_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }

        // Bx
        OneByteOpCode::OR_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::OR_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::OR_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            bitwise_byte_or_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_B => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::B);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_C => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::C);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_D => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::D);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_E => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::E);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_H => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::H);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_L => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::L);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::CP_HLcontents => {
            let byte: u8 = get_byte_from_virtual_register(motherboard, &RegWord::HL);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::CP_A => {
            let byte: u8 = get_byte_from_8bit_register(motherboard, &RegByte::A);
            compare_byte_to_a(motherboard, byte);
            motherboard.clock.cycle_clock(1);
        }

        // Cx
        // Notes: Incrementing twice because stack pointer stores pointer to 8 byte segments.
        // Notes: -> Popping increments pointer because as the pointer grows it actually moves to lower addresses
        // TODO: I NEED TO LOOK INTO THIS -> how we store the stack pointer/program counter... and endianness
        // TODO: Make sure this is a conditional return when return if NOT ZERO
        // z/nc/nz etc... info -> https://learn.cemetech.net/index.php?title=Z80:Opcodes:RET
        // other info -> https://rgbds.gbdev.io/docs/v1.0.0/gbz80.7#RET_cc
        OneByteOpCode::RET_NZ => {
            if motherboard.registers.read_flag(RegFlag::Zero) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            return_call(motherboard);

            motherboard.clock.cycle_clock(5);
        }
        OneByteOpCode::POP_BC => {
            let low_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            let high_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            motherboard.registers.write_byte(&RegByte::B, high_byte);
            motherboard.registers.write_byte(&RegByte::C, low_byte);

            motherboard.clock.cycle_clock(3);
        }
        OneByteOpCode::PUSH_BC => {
            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::B),
                &RegWord::SP,
            );

            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::C),
                &RegWord::SP,
            );

            motherboard.clock.cycle_clock(4);
        }
        OneByteOpCode::RST_00H => {
            fast_reset_to_address(motherboard, 0x0000);

            motherboard.clock.cycle_clock(4)
        }
        OneByteOpCode::RET_Z => {
            if !motherboard.registers.read_flag(RegFlag::Zero) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            return_call(motherboard);

            motherboard.clock.cycle_clock(5);
        }
        OneByteOpCode::RET => {
            return_call(motherboard);

            motherboard.clock.cycle_clock(4);
        }
        OneByteOpCode::RST_08H => {
            fast_reset_to_address(motherboard, 0x0008);

            motherboard.clock.cycle_clock(4)
        }

        // Dx
        OneByteOpCode::RET_NC => {
            if motherboard.registers.read_flag(RegFlag::Carry) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            return_call(motherboard);

            motherboard.clock.cycle_clock(5);
        }
        OneByteOpCode::POP_DE => {
            let low_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            let high_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            motherboard.registers.write_byte(&RegByte::D, high_byte);
            motherboard.registers.write_byte(&RegByte::E, low_byte);

            motherboard.clock.cycle_clock(3);
        }
        OneByteOpCode::PUSH_DE => {
            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::D),
                &RegWord::SP,
            );

            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::E),
                &RegWord::SP,
            );

            motherboard.clock.cycle_clock(4);
        }
        OneByteOpCode::RST_10H => {
            fast_reset_to_address(motherboard, 0x0010);

            motherboard.clock.cycle_clock(4)
        }
        OneByteOpCode::RET_C => {
            if !motherboard.registers.read_flag(RegFlag::Carry) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            return_call(motherboard);

            motherboard.clock.cycle_clock(5);
        }
        OneByteOpCode::RETI => {
            return_call(motherboard);

            motherboard.registers.write_ime(true);

            motherboard.clock.cycle_clock(4);
        }
        OneByteOpCode::RST_18H => {
            fast_reset_to_address(motherboard, 0x0018);

            motherboard.clock.cycle_clock(4)
        }

        // Ex
        OneByteOpCode::POP_HL => {
            let low_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            let high_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            motherboard.registers.write_byte(&RegByte::H, high_byte);
            motherboard.registers.write_byte(&RegByte::L, low_byte);

            motherboard.clock.cycle_clock(3);
        }
        OneByteOpCode::LD_Ccontents_A => {
            let address: u16 = 0xFF00 | motherboard.registers.read_byte(&RegByte::C) as u16;

            motherboard
                .memory
                .write_byte(address, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::PUSH_HL => {
            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::H),
                &RegWord::SP,
            );

            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::L),
                &RegWord::SP,
            );

            motherboard.clock.cycle_clock(4);
        }
        OneByteOpCode::RST_20H => {
            fast_reset_to_address(motherboard, 0x0020);

            motherboard.clock.cycle_clock(4)
        }
        OneByteOpCode::JP_HLcontents => {
            load_word_to_16bit_register(
                motherboard,
                motherboard.registers.read_word(&RegWord::HL),
                &RegWord::PC,
            );
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::RST_28H => {
            fast_reset_to_address(motherboard, 0x0028);

            motherboard.clock.cycle_clock(4)
        }

        // Fx
        OneByteOpCode::POP_AF => {
            let low_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            let high_byte: u8 = get_byte_from_stackpointer_dont_increment(motherboard);
            motherboard.registers.increment_sp();

            motherboard.registers.write_byte(&RegByte::A, high_byte);
            motherboard.registers.write_byte(&RegByte::F, low_byte);

            motherboard.clock.cycle_clock(3);
        }
        OneByteOpCode::LD_A_Ccontents => {
            let address: u16 = 0xFF00 | motherboard.registers.read_byte(&RegByte::C) as u16;

            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.memory.read_byte(address));
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::DI => {
            motherboard.registers.write_ime(false);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::PUSH_AF => {
            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::A),
                &RegWord::SP,
            );

            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(
                motherboard,
                motherboard.registers.read_byte(&RegByte::F),
                &RegWord::SP,
            );

            motherboard.clock.cycle_clock(4);
        }
        OneByteOpCode::RST_30H => {
            fast_reset_to_address(motherboard, 0x0030);

            motherboard.clock.cycle_clock(4)
        }
        OneByteOpCode::LD_SP_HL => {
            load_word_to_16bit_register(
                motherboard,
                motherboard.registers.read_word(&RegWord::HL),
                &RegWord::SP,
            );
            motherboard.clock.cycle_clock(2);
        }
        OneByteOpCode::EI => {
            motherboard.registers.write_ime(true);
            motherboard.clock.cycle_clock(1);
        }
        OneByteOpCode::RST_38H => {
            fast_reset_to_address(motherboard, 0x0038);

            motherboard.clock.cycle_clock(4)
        } // _ => panic!("ERROR::Invalid One Byte OpCode! Yoinked by Jaguar Claw!"),
    }
}

pub fn execute_two_byte_opcode(
    motherboard: &mut motherboard::Motherboard,
    code: TwoByteOpCode,
    byte1: u8,
) {
    match code {
        // 0x
        TwoByteOpCode::LD_B_N8 => {
            motherboard.registers.write_byte(&RegByte::B, byte1);
            motherboard.clock.cycle_clock(2);
        }
        TwoByteOpCode::LD_C_N8 => {
            motherboard.registers.write_byte(&RegByte::C, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // 1x
        TwoByteOpCode::STOP => {
            // TODO: Look into more later
        }
        TwoByteOpCode::LD_D_N8 => {
            motherboard.registers.write_byte(&RegByte::D, byte1);
            motherboard.clock.cycle_clock(2);
        }
        TwoByteOpCode::JR_R8 => {
            let signed_byte1: i8 = byte1 as i8;
            let current_address = motherboard.registers.read_word(&RegWord::PC);
            let signed_address: i16 = current_address as i16;
            let new_address = signed_address.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::PC, new_address as u16);

            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::LD_E_N8 => {
            motherboard.registers.write_byte(&RegByte::E, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // 2x
        TwoByteOpCode::JR_NZ_R8 => {
            if motherboard.registers.read_flag(RegFlag::Zero) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            let signed_byte1: i8 = byte1 as i8;
            let current_address = motherboard.registers.read_word(&RegWord::PC);
            let signed_address: i16 = current_address as i16;
            let new_address = signed_address.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::PC, new_address as u16);
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::LD_H_N8 => {
            motherboard.registers.write_byte(&RegByte::H, byte1);
            motherboard.clock.cycle_clock(2);
        }
        TwoByteOpCode::JR_Z_R8 => {
            if !motherboard.registers.read_flag(RegFlag::Zero) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            let signed_byte1: i8 = byte1 as i8;
            let current_address = motherboard.registers.read_word(&RegWord::PC);
            let signed_address: i16 = current_address as i16;
            let new_address = signed_address.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::PC, new_address as u16);
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::LD_L_N8 => {
            motherboard.registers.write_byte(&RegByte::L, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // 3x
        TwoByteOpCode::JR_NC_R8 => {
            if motherboard.registers.read_flag(RegFlag::Carry) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            let signed_byte1: i8 = byte1 as i8;
            let current_address = motherboard.registers.read_word(&RegWord::PC);
            let signed_address: i16 = current_address as i16;
            let new_address = signed_address.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::PC, new_address as u16);
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::LD_HLcontents_N8 => {
            let hl_byte_address: u16 = motherboard.registers.read_word(&RegWord::HL);
            motherboard.memory.write_byte(hl_byte_address, byte1);
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::JR_C_R8 => {
            if !motherboard.registers.read_flag(RegFlag::Carry) {
                motherboard.clock.cycle_clock(2);
                return;
            }

            let signed_byte1: i8 = byte1 as i8;
            let current_address = motherboard.registers.read_word(&RegWord::PC);
            let signed_address: i16 = current_address as i16;
            let new_address = signed_address.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::PC, new_address as u16);
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::LD_A_N8 => {
            motherboard.registers.write_byte(&RegByte::A, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // Cx
        TwoByteOpCode::ADD_A_N8 => {
            add_byte_to_8bit_register(motherboard, byte1, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        TwoByteOpCode::ADC_A_N8 => {
            add_byte_and_carry_to_8bit_register(motherboard, byte1, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        // Dx
        TwoByteOpCode::SUB_N8 => {
            subtract_byte_from_8bit_register(motherboard, byte1, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        TwoByteOpCode::SBC_A_N8 => {
            subtract_byte_and_carry_from_8bit_register(motherboard, byte1, &RegByte::A);
            motherboard.clock.cycle_clock(2);
        }
        // Ex
        TwoByteOpCode::LDH_A8contents_A => {
            // TODO: Make sure this is correct
            // 0b1111_1111_0000_0000 | 0b0000_0000_????_???? => 0b1111_1111_????_????
            let address: u16 = 0xFF00 | byte1 as u16;

            motherboard
                .memory
                .write_byte(address, motherboard.registers.read_byte(&RegByte::A));
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::AND_N8 => {
            bitwise_byte_and_a(motherboard, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // TODO: Need to double check this down the line. Something about flags not caring about it being signed addition?
        TwoByteOpCode::ADD_SP_R8 => {
            let signed_byte1: i8 = byte1 as i8;
            let current_sp = motherboard.registers.read_word(&RegWord::SP);
            let signed_sp: i16 = current_sp as i16;

            let half_carry = ((current_sp & 0xF) + (byte1 & 0xF) as u16) > 0xF;
            let full_carry = ((current_sp & 0xFF) + (byte1 & 0xFF) as u16) > 0xFF;

            let new_sp = signed_sp.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::SP, new_sp as u16);

            motherboard
                .registers
                .write_flag(RegFlag::HalfCarry, half_carry);
            motherboard.registers.write_flag(RegFlag::Carry, full_carry);
            motherboard.registers.write_flag(RegFlag::Zero, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.clock.cycle_clock(4);
        }
        TwoByteOpCode::XOR_N8 => {
            bitwise_byte_xor_a(motherboard, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // Fx
        TwoByteOpCode::LDH_A_A8contents => {
            let address: u16 = 0xFF00 | byte1 as u16;

            motherboard
                .registers
                .write_byte(&RegByte::A, motherboard.memory.read_byte(address));

            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::OR_N8 => {
            bitwise_byte_or_a(motherboard, byte1);
            motherboard.clock.cycle_clock(2);
        }
        // TODO: Check later. same as ADD SP r8, but also check if SP should be updated here or just
        TwoByteOpCode::LD_HL_SPplusR8 => {
            let signed_byte1: i8 = byte1 as i8;
            let current_sp = motherboard.registers.read_word(&RegWord::SP);
            let signed_sp: i16 = current_sp as i16;

            let half_carry = ((current_sp & 0xF) + (byte1 & 0xF) as u16) > 0xF;
            let full_carry = ((current_sp & 0xFF) + (byte1 & 0xFF) as u16) > 0xFF;

            let new_hl = signed_sp.wrapping_add(signed_byte1.into());

            motherboard
                .registers
                .write_word(&RegWord::HL, new_hl as u16);

            motherboard
                .registers
                .write_flag(RegFlag::HalfCarry, half_carry);
            motherboard.registers.write_flag(RegFlag::Carry, full_carry);
            motherboard.registers.write_flag(RegFlag::Zero, false);
            motherboard
                .registers
                .write_flag(RegFlag::Subtraction, false);
            motherboard.clock.cycle_clock(3);
        }
        TwoByteOpCode::CP_N8 => {
            compare_byte_to_a(motherboard, byte1);
            motherboard.clock.cycle_clock(2);
        }
    }
}

pub fn execute_three_byte_opcode(
    motherboard: &mut motherboard::Motherboard,
    code: ThreeByteOpCode,
    high_byte: u8,
    low_byte: u8,
) {
    match code {
        // 0x
        ThreeByteOpCode::LD_BC_D16 => {
            motherboard.registers.write_byte(&RegByte::B, high_byte);
            motherboard.registers.write_byte(&RegByte::C, low_byte);
            motherboard.clock.cycle_clock(3);
        }
        ThreeByteOpCode::LD_A16contents_SP => {
            // Load low byte of SP into a16, load high byte of SP into a16+1
            let low_byte_sp = (motherboard.registers.read_word(&RegWord::SP) & 0xFF) as u8;
            let high_byte_sp = ((motherboard.registers.read_word(&RegWord::SP) >> 8) & 0xFF) as u8;
            let address = ((high_byte as u16) << 8) | (low_byte as u16);

            motherboard.memory.write_byte(address, low_byte_sp);
            motherboard.memory.write_byte(address + 1, high_byte_sp);

            motherboard.clock.cycle_clock(5);
        }
        // 1x
        ThreeByteOpCode::LD_DE_D16 => {
            motherboard.registers.write_byte(&RegByte::D, high_byte);
            motherboard.registers.write_byte(&RegByte::E, low_byte);
            motherboard.clock.cycle_clock(3);
        }
        // 2x
        ThreeByteOpCode::LD_HL_D16 => {
            motherboard.registers.write_byte(&RegByte::H, high_byte);
            motherboard.registers.write_byte(&RegByte::L, low_byte);
            motherboard.clock.cycle_clock(3);
        }
        // 3x
        ThreeByteOpCode::LD_SP_D16 => {
            let low_byte_sp = (motherboard.registers.read_word(&RegWord::SP) & 0xFF) as u8;
            let high_byte_sp = ((motherboard.registers.read_word(&RegWord::SP) >> 8) & 0xFF) as u8;
            let sp_word = ((high_byte as u16) << 8) | (low_byte as u16);

            motherboard.registers.write_word(&RegWord::SP, sp_word);
            motherboard.clock.cycle_clock(3);
        }
        // Cx
        ThreeByteOpCode::JP_NZ_A16 => {
            if motherboard.registers.read_flag(RegFlag::Zero) {
                motherboard.clock.cycle_clock(3);
                return;
            }

            let address = ((high_byte as u16) << 8) | (low_byte as u16);
            motherboard.registers.write_word(&RegWord::PC, address);
            motherboard.clock.cycle_clock(4);
        }
        ThreeByteOpCode::JP_A16 => {
            let address = ((high_byte as u16) << 8) | (low_byte as u16);
            motherboard.registers.write_word(&RegWord::PC, address);
            motherboard.clock.cycle_clock(4);
        }
        // TODO: Test for this
        ThreeByteOpCode::CALL_NZ_A16 => {
            if motherboard.registers.read_flag(RegFlag::Zero) {
                motherboard.clock.cycle_clock(3);
                return;
            }

            let ret_address = motherboard
                .registers
                .read_word(&RegWord::SP)
                .wrapping_add(1);

            let ret_address_high = ((ret_address >> 8) & 0xFF) as u8;
            let ret_address_low = (ret_address & 0xFF) as u8;

            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(motherboard, ret_address_high, &RegWord::SP);

            motherboard.registers.decrement_sp();
            load_byte_to_virtual_register_target(motherboard, ret_address_low, &RegWord::SP);

            let new_address = ((high_byte as u16) << 8) | (low_byte as u16);
            motherboard.registers.write_word(&RegWord::PC, new_address);
            motherboard.clock.cycle_clock(6);
        }
        ThreeByteOpCode::JP_Z_A16 => {}
        ThreeByteOpCode::CALL_Z_A16 => {}
        ThreeByteOpCode::CALL_A16 => {}
        _ => panic!("ERROR::Invalid Three Byte OpCode! Yoinked by Hawk Claw!"),
    }
}

// Helper Functions
// Load/Get Helper Functions
pub fn load_byte_to_virtual_register_target(
    motherboard: &mut motherboard::Motherboard,
    byte: u8,
    virtual_register: &RegWord,
) {
    let address = motherboard.registers.read_word(virtual_register);
    motherboard.memory.write_byte(address, byte);
}

pub fn load_8bit_register_from_virtual_register(
    motherboard: &mut motherboard::Motherboard,
    register: &RegByte,
    virtual_register: &RegWord,
) {
    let value = motherboard
        .memory
        .read_byte(motherboard.registers.read_word(virtual_register));
    motherboard.registers.write_byte(register, value);
}

pub fn get_byte_from_virtual_register(
    motherboard: &mut motherboard::Motherboard,
    virtual_register: &RegWord,
) -> u8 {
    let value = motherboard
        .memory
        .read_byte(motherboard.registers.read_word(virtual_register));
    return value;
}

pub fn get_byte_from_8bit_register(
    motherboard: &mut motherboard::Motherboard,
    register: &RegByte,
) -> u8 {
    return motherboard.registers.read_byte(register);
}

pub fn get_word_from_16bit_register(
    motherboard: &mut motherboard::Motherboard,
    register: &RegWord,
) -> u16 {
    return motherboard.registers.read_word(register);
}

pub fn load_word_to_16bit_register(
    motherboard: &mut motherboard::Motherboard,
    word: u16,
    register: &RegWord,
) {
    motherboard.registers.write_word(register, word);
}

pub fn get_byte_from_stackpointer_dont_increment(motherboard: &mut motherboard::Motherboard) -> u8 {
    return motherboard
        .memory
        .read_byte(motherboard.registers.read_word(&RegWord::SP));
}

pub fn load_byte_into_stack_after_decrement_stack_pointer(
    motherboard: &mut motherboard::Motherboard,
    byte: u8,
) {
    decrement_virtual_register_ignore_flags(motherboard, &RegWord::SP);
    load_byte_to_virtual_register_target(motherboard, byte, &RegWord::SP);
}

// Increment Helper Functions
pub fn increment_8bit_register(motherboard: &mut motherboard::Motherboard, register: &RegByte) {
    let value = motherboard.registers.read_byte(register);
    let (result, zero, half_carry) = increment_8bit(value);
    motherboard.registers.write_flag(RegFlag::Zero, zero);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, half_carry);
    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard.registers.write_byte(register, result);
}

pub fn increment_8bit(value: u8) -> (u8, bool, bool) {
    let result = value.wrapping_add(1);

    // check if we would have a carry
    let half_carry = calculate_8bit_half_carry(value, 1);

    let zero = result == 0;

    (result, zero, half_carry)
}

pub fn increment_virtual_register_ignore_flags(
    motherboard: &mut motherboard::Motherboard,
    virtual_register: &RegWord,
) {
    let value: u16 = motherboard.registers.read_word(virtual_register);
    let result: u16 = value.wrapping_add(1);
    motherboard.registers.write_word(virtual_register, result);
}

//Decrement Helper Functions
pub fn decrement_8bit_register(motherboard: &mut motherboard::Motherboard, register: &RegByte) {
    let value = motherboard.registers.read_byte(register);
    let (result, zero, half_borrow) = decrement_8bit(value);
    motherboard.registers.write_flag(RegFlag::Zero, zero);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, half_borrow);
    motherboard.registers.write_flag(RegFlag::Subtraction, true);
    motherboard.registers.write_byte(register, result);
}

pub fn decrement_8bit(value: u8) -> (u8, bool, bool) {
    let (result, is_borrow) = value.overflowing_sub(1);
    let zero = result == 0;

    // TODO: THINK CORRECT -> talk to tint/look at it with someone

    let is_half_borrow = (value & 0x0f) < (1 & 0x0f);

    (result, zero, is_half_borrow)
}

pub fn decrement_virtual_register_ignore_flags(
    motherboard: &mut motherboard::Motherboard,
    virtual_register: &RegWord,
) {
    let value: u16 = motherboard.registers.read_word(virtual_register);
    let result: u16 = value.wrapping_sub(1);
    motherboard.registers.write_word(virtual_register, result);
}

// Add Helper Functions
pub fn add_virtual_register_from_virtual_register(
    motherboard: &mut motherboard::Motherboard,
    receiver: &RegWord,
    sender: &RegWord,
) {
    // TODO: Maybe look over later, seems fine
    let receiver_value = motherboard.registers.read_word(receiver);
    let sender_value = motherboard.registers.read_word(&sender);
    let (result, half_carry, carry) = add_16bit_and_16bit(receiver_value, sender_value);

    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, half_carry);
    motherboard.registers.write_flag(RegFlag::Carry, carry);

    motherboard.registers.write_word(receiver, result);
}

fn add_16bit_and_16bit(num1: u16, num2: u16) -> (u16, bool, bool) {
    let (result, is_carry) = num1.overflowing_add(num2);

    // check if we would have a carry
    let is_half_carry = calculate_16bit_half_carry(num1, num2);

    (result, is_half_carry, is_carry)
}

pub fn add_byte_to_8bit_register(
    motherboard: &mut motherboard::Motherboard,
    new_value: u8,
    register: &RegByte,
) {
    let initial_value = motherboard.registers.read_byte(register);
    let (result, carry) = initial_value.overflowing_add(new_value);

    // check if we would have initial_value
    let half_carry = calculate_8bit_half_carry(initial_value, new_value);

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, half_carry);
    motherboard.registers.write_flag(RegFlag::Carry, carry);

    motherboard.registers.write_byte(register, result);
}

pub fn add_byte_and_carry_to_8bit_register(
    motherboard: &mut motherboard::Motherboard,
    new_value: u8,
    register: &RegByte,
) {
    if !motherboard.registers.read_flag(RegFlag::Carry) {
        add_byte_to_8bit_register(motherboard, new_value, register);
        return;
    }

    let initial_value = motherboard.registers.read_byte(register);
    let (result, is_overflow) = initial_value.overflowing_add(new_value);

    // Check for half carry between A + B, then result + carry
    let half_carry = calculate_8bit_half_carry(initial_value, new_value);
    let half_carry2 = calculate_8bit_half_carry(result, 1);

    let (result2, is_overflow2) = result.overflowing_add(1);

    motherboard
        .registers
        .write_flag(RegFlag::Zero, result2 == 0);
    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, half_carry | half_carry2);
    motherboard
        .registers
        .write_flag(RegFlag::Carry, is_overflow | is_overflow2);

    motherboard.registers.write_byte(register, result2);
}

// Subtract Helper Functions
pub fn subtract_byte_from_8bit_register(
    motherboard: &mut motherboard::Motherboard,
    subtrahend: u8,
    register: &RegByte,
) {
    let initial_value = motherboard.registers.read_byte(register);
    let (result, borrow) = initial_value.overflowing_sub(subtrahend);

    // check if we would have to borrow from the 5th bit
    let half_borrow = (initial_value & 0xF) < (subtrahend & 0xF);

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard.registers.write_flag(RegFlag::Subtraction, true);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, half_borrow);
    motherboard.registers.write_flag(RegFlag::Carry, borrow);

    motherboard.registers.write_byte(register, result);
}

pub fn subtract_byte_and_carry_from_8bit_register(
    motherboard: &mut motherboard::Motherboard,
    subtrahend: u8,
    register: &RegByte,
) {
    if !motherboard.registers.read_flag(RegFlag::Carry) {
        subtract_byte_from_8bit_register(motherboard, subtrahend, register);
        return;
    }

    let initial_value = motherboard.registers.read_byte(&RegByte::A);
    let result = initial_value.wrapping_sub(subtrahend).wrapping_sub(1);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (initial_value & 0xF) < ((subtrahend & 0xF) + 1);
    let is_borrow = subtrahend == 0xFF || initial_value < (subtrahend + 1);

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard.registers.write_flag(RegFlag::Subtraction, true);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, is_half_borrow);
    motherboard.registers.write_flag(RegFlag::Carry, is_borrow);

    motherboard.registers.write_byte(register, result);
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
pub fn bitwise_byte_and_a(motherboard: &mut motherboard::Motherboard, value: u8) {
    let a_value = motherboard.registers.read_byte(&RegByte::A);
    let result = a_value & value;

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard.registers.write_flag(RegFlag::HalfCarry, true);
    motherboard.registers.write_flag(RegFlag::Carry, false);

    motherboard.registers.write_byte(&RegByte::A, result);
}

pub fn bitwise_byte_xor_a(motherboard: &mut motherboard::Motherboard, value: u8) {
    let a_value = motherboard.registers.read_byte(&RegByte::A);
    let result = a_value ^ value;

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard.registers.write_flag(RegFlag::HalfCarry, false);
    motherboard.registers.write_flag(RegFlag::Carry, false);

    motherboard.registers.write_byte(&RegByte::A, result);
}

pub fn bitwise_byte_or_a(motherboard: &mut motherboard::Motherboard, value: u8) {
    let a_value = motherboard.registers.read_byte(&RegByte::A);
    let result = a_value | value;

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard
        .registers
        .write_flag(RegFlag::Subtraction, false);
    motherboard.registers.write_flag(RegFlag::HalfCarry, false);
    motherboard.registers.write_flag(RegFlag::Carry, false);

    motherboard.registers.write_byte(&RegByte::A, result);
}

pub fn compare_byte_to_a(motherboard: &mut motherboard::Motherboard, value: u8) {
    let a_value = motherboard.registers.read_byte(&RegByte::A);
    let (result, is_borrow) = a_value.overflowing_sub(value);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (a_value & 0xF) < (value & 0xF);

    motherboard.registers.write_flag(RegFlag::Zero, result == 0);
    motherboard.registers.write_flag(RegFlag::Subtraction, true);
    motherboard
        .registers
        .write_flag(RegFlag::HalfCarry, is_half_borrow);
    motherboard.registers.write_flag(RegFlag::Carry, is_borrow);
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

// Stack Related Helper Functions
pub fn return_call(motherboard: &mut motherboard::Motherboard) {
    let low_word: u16 = get_byte_from_stackpointer_dont_increment(motherboard).into();
    motherboard.registers.increment_sp();

    let mut high_word: u16 = get_byte_from_stackpointer_dont_increment(motherboard).into();
    motherboard.registers.increment_sp();

    // 0x00FF becomes -> 0xFF00
    high_word = high_word << 8;

    // 0xFF00 & 0x00AB becomes -> 0xFFAB
    let full_word = high_word | low_word;

    motherboard.registers.write_word(&RegWord::PC, full_word);
}

pub fn fast_reset_to_address(motherboard: &mut motherboard::Motherboard, address: u16) {
    motherboard.registers.increment_pc();

    let [msb, lsb] = motherboard.registers.read_word(&RegWord::PC).to_be_bytes();

    // Load High then Low Byte into stack
    load_byte_into_stack_after_decrement_stack_pointer(motherboard, msb);
    load_byte_into_stack_after_decrement_stack_pointer(motherboard, lsb);

    // Fast Jump (aka reset) to PC == 0x0008
    load_word_to_16bit_register(motherboard, address, &RegWord::PC);
}
