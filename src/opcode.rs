use crate::{registers::{self, RegByte, RegFlag}};
use crate::clock;
use crate::cpu;

pub enum OpCode {
    ADD_A_B = 0x80,
    ADD_A_C = 0x81,
}

pub enum PrefixOpCode {
    RES_0_B = 0x80,

}


pub fn execute_opcode(cpu: &mut cpu::Cpu, code: OpCode)
{
    match code {
        OpCode::ADD_A_B => {
            add_register_to_A(cpu, &RegByte::B);
        },
        OpCode::ADD_A_C => {
            add_register_to_A(cpu, &RegByte::C);
        }
        _ => panic!("Invalid OpCode!")
    }    
}

pub fn calculate_half_carry(byte1: u8, byte2: u8) -> bool {
    // mask top 4 on byte1 + byte2, add them together, check bit4 afterwards
    let hc = (((byte1 & 0xF) + ( byte2 & 0xF)) & 0x10) == 0x10;
    hc
}


pub fn update_zero_flag(cpu: &mut cpu::Cpu, result: u8) {
    if result == 0 {
        cpu.registers.write_flag(RegFlag::Zero, true)
    } 
    else {
        cpu.registers.write_flag(RegFlag::Zero, false)
    }
}

pub fn update_half_carry_flag(cpu: &mut cpu::Cpu, hc: bool, hc2: bool) {
    if hc || hc2 {
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
    let (result, overflowed) = 
            cpu.registers.read_byte(&RegByte::A).overflowing_add
                (cpu.registers.read_byte(register));               

        // Check if there is a half carry between A + B, and then the
           // > result + the carry 
        let hc = calculate_half_carry(cpu.registers.read_byte(&RegByte::A),
            cpu.registers.read_byte(register));
        let hc2 = calculate_half_carry(result,
            cpu.registers.read_flag(RegFlag::Carry) as u8);   

        // second addition, between original result and carry for final byte
        let (result2, overflowed2) = 
            result.overflowing_add(cpu.registers.add_carry());
        
        update_carry_flag(cpu, overflowed, overflowed2);

        update_zero_flag(cpu, result2);

        update_half_carry_flag(cpu, hc, hc2);

        cpu.registers.write_byte(RegByte::A, result2);

        cpu.clock.cycle_clock(1);    
}
#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn execute_op_code_ADD_A_B() {
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

        // Setting up new state
        cpu.registers.write_byte(RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_byte(RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        // Adding B = 25, to A = 200, should be A=226 due to carry preset to true
        execute_opcode(&mut cpu, OpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 226);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }
}