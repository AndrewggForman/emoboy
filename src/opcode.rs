use crate::clock;
use crate::cpu;
use crate::registers::{self, RegByte, RegFlag};

// Load = LD, load right value into left, aka LD_B_C == Load C into B
pub enum OpCode {
    LD_B_B = 0x40,
    LD_B_C = 0x41,
    LD_B_D = 0x42,
    LD_B_E = 0x43,
    LD_B_H = 0x44,
    LD_B_L = 0x45,
    //LOAD_B_HL_BYTE = 0x46
    LD_B_A = 0x47,

    LD_C_B = 0x48,
    LD_C_C = 0x49,
    LD_C_D = 0x4A,
    LD_C_E = 0x4B,
    LD_C_H = 0x4C,
    LD_C_L = 0x4D,
    //LD_C_HL_BYTE = 0x4E,
    LD_C_A = 0x4F,

    LD_D_B = 0x50,
    LD_D_C = 0x51,
    LD_D_D = 0x52,
    LD_D_E = 0x53,
    LD_D_H = 0x54,
    LD_D_L = 0x55,
    //LD_D_HL_BYTE = 0x56,
    LD_D_A = 0x57,

    LD_E_B = 0x58,
    LD_E_C = 0x59,
    LD_E_D = 0x5A,
    LD_E_E = 0x5B,
    LD_E_H = 0x5C,
    LD_E_L = 0x5D,
    //LD_E_HL_BYTE = 0x5E,
    LD_E_A = 0x5F,

    LD_H_B = 0x60,
    LD_H_C = 0x61,
    LD_H_D = 0x62,
    LD_H_E = 0x63,
    LD_H_H = 0x64,
    LD_H_L = 0x65,
    //LD_H_HL_BYTE = 0x66,
    LD_H_A = 0x67,

    LD_L_B = 0x68,
    LD_L_C = 0x69,
    LD_L_D = 0x6A,
    LD_L_E = 0x6B,
    LD_L_H = 0x6C,
    LD_L_L = 0x6D,
    //LD_L_HL_BYTE = 0x6E,
    LD_L_A = 0x6F,

    ADD_A_B = 0x80,
    ADD_A_C = 0x81,
    ADD_A_D = 0x82,
    ADD_A_E = 0x83,
    ADD_A_H = 0x84,
    ADD_A_L = 0x85,
    //ADD_A_HL_BYTE = 0x86,
    ADD_A_A = 0x87,
    ADC_A_B = 0x88,
    ADC_A_C = 0x89,
    ADC_A_D = 0x8A,
    ADC_A_E = 0x8B,
    ADC_A_H = 0x8C,
    ADC_A_L = 0x8D,
    //ADC_A_HL_BYTE = 0x8E,
    ADC_A_A = 0x8F,
}

pub enum PrefixOpCode {
    RES_0_B = 0x80,
}

pub fn execute_opcode(cpu: &mut cpu::Cpu, code: OpCode) {
    match code {
        // Load Register into B
        OpCode::LD_B_B => {
            load_register_to_register(cpu, RegByte::B, RegByte::B);
        }
        OpCode::LD_B_C => {
            load_register_to_register(cpu, RegByte::C, RegByte::B);
        }
        OpCode::LD_B_D => {
            load_register_to_register(cpu, RegByte::D, RegByte::B);
        }
        OpCode::LD_B_E => {
            load_register_to_register(cpu, RegByte::E, RegByte::B);
        }
        OpCode::LD_B_H => {
            load_register_to_register(cpu, RegByte::H, RegByte::B);
        }
        OpCode::LD_B_L => {
            load_register_to_register(cpu, RegByte::L, RegByte::B);
        }
        OpCode::LD_B_A => {
            load_register_to_register(cpu, RegByte::A, RegByte::B);
        }

        // Load Register Into C
        OpCode::LD_C_B => {
            load_register_to_register(cpu, RegByte::B, RegByte::C);
        }
        OpCode::LD_C_C => {
            load_register_to_register(cpu, RegByte::C, RegByte::C);
        }
        OpCode::LD_C_D => {
            load_register_to_register(cpu, RegByte::D, RegByte::C);
        }
        OpCode::LD_C_E => {
            load_register_to_register(cpu, RegByte::E, RegByte::C);
        }
        OpCode::LD_C_H => {
            load_register_to_register(cpu, RegByte::H, RegByte::C);
        }
        OpCode::LD_C_L => {
            load_register_to_register(cpu, RegByte::L, RegByte::C);
        }
        OpCode::LD_C_A => {
            load_register_to_register(cpu, RegByte::A, RegByte::C);
        }

        // Load Register into D
        OpCode::LD_D_B => {
            load_register_to_register(cpu, RegByte::B, RegByte::D);
        }
        OpCode::LD_D_C => {
            load_register_to_register(cpu, RegByte::C, RegByte::D);
        }
        OpCode::LD_D_D => {
            load_register_to_register(cpu, RegByte::D, RegByte::D);
        }
        OpCode::LD_D_E => {
            load_register_to_register(cpu, RegByte::E, RegByte::D);
        }
        OpCode::LD_D_H => {
            load_register_to_register(cpu, RegByte::H, RegByte::D);
        }
        OpCode::LD_D_L => {
            load_register_to_register(cpu, RegByte::L, RegByte::D);
        }
        OpCode::LD_D_A => {
            load_register_to_register(cpu, RegByte::A, RegByte::D);
        }

        // Load Register into E
        OpCode::LD_E_B => {
            load_register_to_register(cpu, RegByte::B, RegByte::E);
        }
        OpCode::LD_E_C => {
            load_register_to_register(cpu, RegByte::C, RegByte::E);
        }
        OpCode::LD_E_D => {
            load_register_to_register(cpu, RegByte::D, RegByte::E);
        }
        OpCode::LD_E_E => {
            load_register_to_register(cpu, RegByte::E, RegByte::E);
        }
        OpCode::LD_E_H => {
            load_register_to_register(cpu, RegByte::H, RegByte::E);
        }
        OpCode::LD_E_L => {
            load_register_to_register(cpu, RegByte::L, RegByte::E);
        }
        OpCode::LD_E_A => {
            load_register_to_register(cpu, RegByte::A, RegByte::E);
        }

        // Load Register into H
        OpCode::LD_H_B => {
            load_register_to_register(cpu, RegByte::B, RegByte::H);
        }
        OpCode::LD_H_C => {
            load_register_to_register(cpu, RegByte::C, RegByte::H);
        }
        OpCode::LD_H_D => {
            load_register_to_register(cpu, RegByte::D, RegByte::H);
        }
        OpCode::LD_H_E => {
            load_register_to_register(cpu, RegByte::E, RegByte::H);
        }
        OpCode::LD_H_H => {
            load_register_to_register(cpu, RegByte::H, RegByte::H);
        }
        OpCode::LD_H_L => {
            load_register_to_register(cpu, RegByte::L, RegByte::H);
        }
        OpCode::LD_H_A => {
            load_register_to_register(cpu, RegByte::A, RegByte::H);
        }

        // Load Register into L
        OpCode::LD_L_B => {
            load_register_to_register(cpu, RegByte::B, RegByte::L);
        }
        OpCode::LD_L_C => {
            load_register_to_register(cpu, RegByte::C, RegByte::L);
        }
        OpCode::LD_L_D => {
            load_register_to_register(cpu, RegByte::D, RegByte::L);
        }
        OpCode::LD_L_E => {
            load_register_to_register(cpu, RegByte::E, RegByte::L);
        }
        OpCode::LD_L_H => {
            load_register_to_register(cpu, RegByte::H, RegByte::L);
        }
        OpCode::LD_L_L => {
            load_register_to_register(cpu, RegByte::L, RegByte::L);
        }
        OpCode::LD_L_A => {
            load_register_to_register(cpu, RegByte::A, RegByte::L);
        }

        // Add Register to A
        OpCode::ADD_A_B => {
            add_register_to_A(cpu, &RegByte::B);
        }
        OpCode::ADD_A_C => {
            add_register_to_A(cpu, &RegByte::C);
        }
        OpCode::ADD_A_D => {
            add_register_to_A(cpu, &RegByte::D);
        }
        OpCode::ADD_A_E => {
            add_register_to_A(cpu, &RegByte::E);
        }
        OpCode::ADD_A_H => {
            add_register_to_A(cpu, &RegByte::H);
        }
        OpCode::ADD_A_L => {
            add_register_to_A(cpu, &RegByte::L);
        }
        OpCode::ADD_A_A => {
            add_register_to_A(cpu, &RegByte::A);
        }

        // Add Register with carry to A
        OpCode::ADC_A_B => {
            add_register_and_carry_to_A(cpu, &RegByte::B);
        }
        OpCode::ADC_A_C => {
            add_register_and_carry_to_A(cpu, &RegByte::C);
        }
        OpCode::ADC_A_D => {
            add_register_and_carry_to_A(cpu, &RegByte::D);
        }
        OpCode::ADC_A_E => {
            add_register_and_carry_to_A(cpu, &RegByte::E);
        }
        OpCode::ADC_A_H => {
            add_register_and_carry_to_A(cpu, &RegByte::H);
        }
        OpCode::ADC_A_L => {
            add_register_and_carry_to_A(cpu, &RegByte::L);
        }
        OpCode::ADC_A_A => {
            add_register_and_carry_to_A(cpu, &RegByte::A);
        }
        _ => panic!("Invalid OpCode!"),
    }
}

pub fn calculate_half_carry(byte1: u8, byte2: u8) -> bool {
    let half_carry = (((byte1 & 0xF) + (byte2 & 0xF)) & 0x10) == 0x10;
    half_carry
}

pub fn update_zero_flag(cpu: &mut cpu::Cpu, result: u8) {
    if result == 0 {
        cpu.registers.write_flag(RegFlag::Zero, true)
    } else {
        cpu.registers.write_flag(RegFlag::Zero, false)
    }
}

pub fn update_half_carry_flag(cpu: &mut cpu::Cpu, half_carry: bool, half_carry2: bool) {
    if half_carry || half_carry2 {
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
    } else {
        cpu.registers.write_flag(RegFlag::HalfCarry, false);
    }
}

pub fn update_carry_flag(cpu: &mut cpu::Cpu, carry: bool, carry2: bool) {
    if carry || carry2 {
        cpu.registers.write_flag(RegFlag::Carry, true);
    } else {
        cpu.registers.write_flag(RegFlag::Carry, false);
    }
}

pub fn add_register_to_A(cpu: &mut cpu::Cpu, register: &RegByte) {
    let (result, overflowed) = cpu
        .registers
        .read_byte(&RegByte::A)
        .overflowing_add(cpu.registers.read_byte(register));

    // Check if there is a half carry between A + B, and then the
    // > result + the carry
    let half_carry = calculate_half_carry(
        cpu.registers.read_byte(&RegByte::A),
        cpu.registers.read_byte(register),
    );

    // TODO - talk about carry_flag issue? maybe rework base functions
    update_carry_flag(cpu, overflowed, false);
    update_zero_flag(cpu, result);
    update_half_carry_flag(cpu, half_carry, false);

    cpu.registers.write_byte(RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

pub fn add_register_and_carry_to_A(cpu: &mut cpu::Cpu, register: &RegByte) {
    let (result, overflowed) = cpu
        .registers
        .read_byte(&RegByte::A)
        .overflowing_add(cpu.registers.read_byte(register));

    // Check if there is a half carry between A + B, and then the
    // > result + the carry
    let half_carry = calculate_half_carry(
        cpu.registers.read_byte(&RegByte::A),
        cpu.registers.read_byte(register),
    );
    let half_carry2 = calculate_half_carry(result, cpu.registers.read_flag(RegFlag::Carry) as u8);

    // second addition, between original result and carry for final byte
    let (result2, overflowed2) = result.overflowing_add(cpu.registers.add_carry());

    update_carry_flag(cpu, overflowed, overflowed2);
    update_zero_flag(cpu, result2);
    update_half_carry_flag(cpu, half_carry, half_carry2);

    cpu.registers.write_byte(RegByte::A, result2);

    cpu.clock.cycle_clock(1);
}

pub fn load_register_to_register(
    cpu: &mut cpu::Cpu,
    sending_register: RegByte,
    receiving_register: RegByte,
) {
    // When The registers are the same, aka Load A to A, nothing should happen except the clock
    if std::ptr::eq(&sending_register, &receiving_register) {
        cpu.clock.cycle_clock(1);
        return;
    }
    cpu.registers.write_byte(
        receiving_register,
        cpu.registers.read_byte(&sending_register),
    );

    cpu.clock.cycle_clock(1);
}

// TODO-FIX THIS
// pub fn add_HL_byte_to_A(cpu: &mut cpu::Cpu, register: &RegByte) {
//     let (result, overflowed) = cpu
//         .registers
//         .read_byte(&RegByte::A)
//         .overflowing_add(cpu.registers.read_byte(register));

//     // Check if there is a half carry between A + B, and then the
//     // > result + the carry
//     let half_carry = calculate_half_carry(
//         cpu.registers.read_byte(&RegByte::A),
//         cpu.registers.read_byte(register),
//     );

//     // TODO - talk about carry_flag issue? maybe rework base functions
//     update_carry_flag(cpu, overflowed, false);
//     update_zero_flag(cpu, result);
//     update_half_carry_flag(cpu, half_carry, false);

//     cpu.registers.write_byte(RegByte::A, result);

//     cpu.clock.cycle_clock(1);
// }

#[cfg(test)]
mod tests {
    use super::*;

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

        execute_opcode(&mut cpu, OpCode::ADD_A_B);

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

        cpu.registers.write_byte(RegByte::B, 24);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 24);

        // Adding B = 24, to A = 0, should be A=0 + proper flags
        execute_opcode(&mut cpu, OpCode::ADD_A_B);

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

        // Setting up new state
        cpu.registers.write_byte(RegByte::A, 15);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 15);
        cpu.registers.write_byte(RegByte::B, 1);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 1);

        // Adding B = 1, to A = 15, should be A=16 + proper flags
        execute_opcode(&mut cpu, OpCode::ADD_A_B);

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

        // Setting up new state
        cpu.registers.write_byte(RegByte::A, 243);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 243);
        cpu.registers.write_byte(RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);

        // Adding B = 25, to A = 243, should be A=12 with a overflow + proper flags
        execute_opcode(&mut cpu, OpCode::ADD_A_B);

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

        // Setting up new state
        cpu.registers.write_byte(RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_byte(RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        // Adding B = 25, to A = 200, should be A=226 due to carry preset to true
        execute_opcode(&mut cpu, OpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 225);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    #[test]
    fn execute_op_code_ADC_A_D_add_200_25_and_carry() {
        let mut cpu = cpu::Cpu::new();

        // Setting up new state
        cpu.registers.write_byte(RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_byte(RegByte::D, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        // Adding B = 25, to A = 200, should be A=226 due to carry preset to true
        execute_opcode(&mut cpu, OpCode::ADC_A_D);

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

        // Setting up new state
        cpu.registers.write_byte(RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(RegByte::D, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        // Adding B = 25, to A = 200, should be A=226 due to carry preset to true
        execute_opcode(&mut cpu, OpCode::ADC_A_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 201);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }
}
