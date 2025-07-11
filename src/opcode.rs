use crate::{cpu, registers::{self, RegByte}};

pub enum OpCode {
    ADD_A_B = 0x80,
    ADD_A_C = 0x81,
}

pub enum PrefixOpCode {
    RES_0_B = 0x80,

}


pub fn execute_opcode(code: OpCode, cpu: &mut cpu::Cpu)
{
    match code {
        OpCode::ADD_A_B => {
            let (result, overflowed) = 
                cpu.registers.read_byte(RegByte::A).overflowing_add
                (cpu.registers.read_byte(RegByte::B))
        },
        _ => panic!("Invalid OpCode!")
    }    
}

