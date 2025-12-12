use crate::clock;
use crate::cpu;
use crate::registers::{self, RegByte, RegFlag, RegWord};

// Load = LD, load right value into left, aka LD_B_C == Load C into B
pub enum OpCode {
    INC_B = 0x04,
    INC_C = 0x0C,
    INC_D = 0x14,
    INC_E = 0x1C,
    INC_H = 0x24,
    INC_L = 0x2C,
    INC_HL = 0x34,
    INC_A = 0x3C,

    INC_BC = 0x03,
    INC_DE = 0x13,
    INC_HL_REG = 0x23,
    INC_SP = 0x33,

    DEC_B = 0x05,
    DEC_C = 0x0D,
    DEC_D = 0x15,
    DEC_E = 0x1D,
    DEC_H = 0x25,
    DEC_L = 0x2D,
    DEC_HL_MEM = 0x35,
    DEC_A = 0x3D,

    DEC_BC = 0x0B,
    DEC_DE = 0x1B,
    DEC_HL = 0x2B,
    DEC_SP = 0x3B,

    RLCA = 0x07,
    RLA = 0x17,
    RRCA = 0x0F,
    RRA = 0x1F,
    //DAA = 0x27, TODO
    SCF = 0x37,

    // TODO: IMPLEMENT THESE: need to fetch next bit
    JR_S8 = 0x18,
    JRZ_S8 = 0x28,
    JRC_s8 = 0x38,

    ADD_HL_BC = 0x09,
    ADD_HL_DE = 0x19,
    ADD_HL_HL = 0x29,
    ADD_HL_SP = 0x39,

    LD_BC_D16 = 0x01,
    LD_DE_D16 = 0x11,
    LD_HL_D16 = 0x21,
    LD_SP_D16 = 0x31,

    LD_BC_A = 0x02,
    LD_DE_A = 0x12,
    LD_HLI_A = 0x22,
    LD_HLD_A = 0x32,

    LD_A_BC = 0x0A,
    LD_A_DE = 0x1A,
    LD_A_HLI = 0x2A,
    LD_A_HLD = 0x3A,

    LD_B_B = 0x40,
    LD_B_C = 0x41,
    LD_B_D = 0x42,
    LD_B_E = 0x43,
    LD_B_H = 0x44,
    LD_B_L = 0x45,
    LD_B_HL = 0x46,
    LD_B_A = 0x47,

    LD_C_B = 0x48,
    LD_C_C = 0x49,
    LD_C_D = 0x4A,
    LD_C_E = 0x4B,
    LD_C_H = 0x4C,
    LD_C_L = 0x4D,
    LD_C_HL = 0x4E,
    LD_C_A = 0x4F,

    LD_D_B = 0x50,
    LD_D_C = 0x51,
    LD_D_D = 0x52,
    LD_D_E = 0x53,
    LD_D_H = 0x54,
    LD_D_L = 0x55,
    LD_D_HL = 0x56,
    LD_D_A = 0x57,

    LD_E_B = 0x58,
    LD_E_C = 0x59,
    LD_E_D = 0x5A,
    LD_E_E = 0x5B,
    LD_E_H = 0x5C,
    LD_E_L = 0x5D,
    LD_E_HL = 0x5E,
    LD_E_A = 0x5F,

    LD_H_B = 0x60,
    LD_H_C = 0x61,
    LD_H_D = 0x62,
    LD_H_E = 0x63,
    LD_H_H = 0x64,
    LD_H_L = 0x65,
    LD_H_HL = 0x66,
    LD_H_A = 0x67,

    LD_L_B = 0x68,
    LD_L_C = 0x69,
    LD_L_D = 0x6A,
    LD_L_E = 0x6B,
    LD_L_H = 0x6C,
    LD_L_L = 0x6D,
    LD_L_HL = 0x6E,
    LD_L_A = 0x6F,

    //
    //LD_HL_X HERE!
    //
    LD_A_B = 0x78,
    LD_A_C = 0x79,
    LD_A_D = 0x7A,
    LD_A_E = 0x7B,
    LD_A_H = 0x7C,
    LD_A_L = 0x7D,
    LD_A_HL = 0x7E,
    LD_A_A = 0x7F,

    ADD_A_B = 0x80,
    ADD_A_C = 0x81,
    ADD_A_D = 0x82,
    ADD_A_E = 0x83,
    ADD_A_H = 0x84,
    ADD_A_L = 0x85,
    ADD_A_HL = 0x86,
    ADD_A_A = 0x87,

    ADC_A_B = 0x88,
    ADC_A_C = 0x89,
    ADC_A_D = 0x8A,
    ADC_A_E = 0x8B,
    ADC_A_H = 0x8C,
    ADC_A_L = 0x8D,
    ADC_A_HL = 0x8E,
    ADC_A_A = 0x8F,

    SUB_A_B = 0x90,
    SUB_A_C = 0x91,
    SUB_A_D = 0x92,
    SUB_A_E = 0x93,
    SUB_A_H = 0x94,
    SUB_A_L = 0x95,
    SUB_A_HL = 0x96,
    SUB_A_A = 0x97,

    SBC_A_B = 0x98,
    SBC_A_C = 0x99,
    SBC_A_D = 0x9A,
    SBC_A_E = 0x9B,
    SBC_A_H = 0x9C,
    SBC_A_L = 0x9D,
    SBC_A_HL = 0x9E,
    SBC_A_A = 0x9F,

    AND_A_B = 0xA0,
    AND_A_C = 0xA1,
    AND_A_D = 0xA2,
    AND_A_E = 0xA3,
    AND_A_H = 0xA4,
    AND_A_L = 0xA5,
    AND_A_HL = 0xA6,
    AND_A_A = 0xA7,

    XOR_A_B = 0xA8,
    XOR_A_C = 0xA9,
    XOR_A_D = 0xAA,
    XOR_A_E = 0xAB,
    XOR_A_H = 0xAC,
    XOR_A_L = 0xAD,
    XOR_A_HL = 0xAE,
    XOR_A_A = 0xAF,

    OR_A_B = 0xB0,
    OR_A_C = 0xB1,
    OR_A_D = 0xB2,
    OR_A_E = 0xB3,
    OR_A_H = 0xB4,
    OR_A_L = 0xB5,
    OR_A_HL = 0xB6,
    OR_A_A = 0xB7,

    CP_A_B = 0xB8,
    CP_A_C = 0xB9,
    CP_A_D = 0xBA,
    CP_A_E = 0xBB,
    CP_A_H = 0xBC,
    CP_A_L = 0xBD,
    CP_A_HL = 0xBE,
    CP_A_A = 0xBF,

    CD_B_C = 0xCD,
}

pub enum OneByteOpCode {
    // TODO: check if dec/incremented commands e..g LD (HL+), if they inc or def before or after command

    // 0x
    NOP = 0x00,
    LD_BCcontents_A = 0x02,
    INC_BC = 0x03,
    INC_B = 0x04,
    DEC_B = 0x05,
    RLCA = 0x07,
    ADD_HL_BC = 0x09,
    LD_A_BCcontents = 0x0A,
    DEC_BC = 0x0B,
    INC_C = 0x0C,
    DEC_C = 0x0D,
    RRCA = 0x0F,

    // 1x
    LD_DEcontents_A = 0x12,
    INC_DE = 0x13,
    INC_D = 0x14,
    DEC_D = 0x15,
    RLA = 0x17,
    ADD_HL_DE = 0x19,
    LD_A_DEcontents = 0x1A,
    DEC_DE = 0x1B,
    INC_E = 0x1C,
    DEC_E = 0x1D,
    RRA = 0x1F,

    // 2x
    LD_HLincrementedcontents_A = 0x22,
    INC_HL = 0x23,
    INC_H = 0x24,
    DEC_H = 0x25,
    DAA = 0x27,
    ADD_HL_HL = 0x29,
    LD_A_HLincrementedcontents = 0x2A,
    DEC_HL = 0x2B,
    INC_L = 0x2C,
    DEC_L = 0x2D,
    CPL = 0x2F,

    // 3x
    LD_HLdecrementedcontents_A = 0x32,
    INC_SP = 0x33,
    INC_HLcontents = 0x34,
    DEC_HLcontents = 0x35,
    SCF = 0x37,
    ADD_HL_SP = 0x39,
    LD_A_HLdecrementedcontents = 0x3A,
    DEC_SP = 0x3B,
    INC_A = 0x3C,
    DEC_A = 0x3D,
    CCF = 0x3F,

    // 4x
    LD_B_B = 0x40,
    LD_B_C = 0x41,
    LD_B_D = 0x42,
    LD_B_E = 0x43,
    LD_B_H = 0x44,
    LD_B_L = 0x45,
    LD_B_HLcontents = 0x46,
    LD_B_A = 0x47,
    LD_C_B = 0x48,
    LD_C_C = 0x49,
    LD_C_D = 0x4A,
    LD_C_E = 0x4B,
    LD_C_H = 0x4C,
    LD_C_L = 0x4D,
    LD_C_HLcontents = 0x4E,
    LD_C_A = 0x4F,

    // 5x
    LD_D_B = 0x50,
    LD_D_C = 0x51,
    LD_D_D = 0x52,
    LD_D_E = 0x53,
    LD_D_H = 0x54,
    LD_D_L = 0x55,
    LD_D_HLcontents = 0x56,
    LD_D_A = 0x57,
    LD_E_B = 0x58,
    LD_E_C = 0x59,
    LD_E_D = 0x5A,
    LD_E_E = 0x5B,
    LD_E_H = 0x5C,
    LD_E_L = 0x5D,
    LD_E_HLcontents = 0x5E,
    LD_E_A = 0x5F,

    // 6x
    LD_H_B = 0x60,
    LD_H_C = 0x61,
    LD_H_D = 0x62,
    LD_H_E = 0x63,
    LD_H_H = 0x64,
    LD_H_L = 0x65,
    LD_H_HLcontents = 0x66,
    LD_H_A = 0x67,
    LD_L_B = 0x68,
    LD_L_C = 0x69,
    LD_L_D = 0x6A,
    LD_L_E = 0x6B,
    LD_L_H = 0x6C,
    LD_L_L = 0x6D,
    LD_L_HLcontents = 0x6E,
    LD_L_A = 0x6F,

    // 7x
    LD_HLcontents_B = 0x70,
    LD_HLcontents_C = 0x71,
    LD_HLcontents_D = 0x72,
    LD_HLcontents_E = 0x73,
    LD_HLcontents_H = 0x74,
    LD_HLcontents_L = 0x75,
    HALT = 0x76,
    LD_HLcontents_A = 0x77,
    LD_A_B = 0x78,
    LD_A_C = 0x79,
    LD_A_D = 0x7A,
    LD_A_E = 0x7B,
    LD_A_H = 0x7C,
    LD_A_L = 0x7D,
    LD_A_HLcontents = 0x7E,
    LD_A_A = 0x7F,

    // 8x
    ADD_A_B = 0x80,
    ADD_A_C = 0x81,
    ADD_A_D = 0x82,
    ADD_A_E = 0x83,
    ADD_A_H = 0x84,
    ADD_A_L = 0x85,
    ADD_A_HLcontents = 0x86,
    ADD_A_A = 0x87,
    ADC_A_B = 0x88,
    ADC_A_C = 0x89,
    ADC_A_D = 0x8A,
    ADC_A_E = 0x8B,
    ADC_A_H = 0x8C,
    ADC_A_L = 0x8D,
    ADC_A_HLcontents = 0x8E,
    ADC_A_A = 0x8F,

    // 9x
    SUB_A_B = 0x90,
    SUB_A_C = 0x91,
    SUB_A_D = 0x92,
    SUB_A_E = 0x93,
    SUB_A_H = 0x94,
    SUB_A_L = 0x95,
    SUB_A_HLcontents = 0x96,
    SUB_A_A = 0x97,
    SBC_A_B = 0x98,
    SBC_A_C = 0x99,
    SBC_A_D = 0x9A,
    SBC_A_E = 0x9B,
    SBC_A_H = 0x9C,
    SBC_A_L = 0x9D,
    SBC_A_HLcontents = 0x9E,
    SBC_A_A = 0x9F,

    // Ax
    AND_B = 0xA0,
    AND_C = 0xA1,
    AND_D = 0xA2,
    AND_E = 0xA3,
    AND_H = 0xA4,
    AND_L = 0xA5,
    AND_HLcontents = 0xA6,
    AND_A = 0xA7,
    XOR_B = 0xA8,
    XOR_C = 0xA9,
    XOR_D = 0xAA,
    XOR_E = 0xAB,
    XOR_H = 0xAC,
    XOR_L = 0xAD,
    XOR_HLcontents = 0xAE,
    XOR_A = 0xAF,

    // Bx
    OR_B = 0xB0,
    OR_C = 0xB1,
    OR_D = 0xB2,
    OR_E = 0xB3,
    OR_H = 0xB4,
    OR_L = 0xB5,
    OR_HLcontents = 0xB6,
    OR_A = 0xB7,
    CP_B = 0xB8,
    CP_C = 0xB9,
    CP_D = 0xBA,
    CP_E = 0xBB,
    CP_H = 0xBC,
    CP_L = 0xBD,
    CP_HLcontents = 0xBE,
    CP_A = 0xBF,

    // Cx
    RET_NZ = 0xC0,
    POP_BC = 0xC1,
    PUSH_BC = 0xC5,
    RST_00H = 0xC7,
    RET_Z = 0xC8,
    RET = 0xC9,
    PREFIX_CB = 0xCB, // SPECIAL PREFIX
    RST_08H = 0xCF,

    // Dx
    RET_NC = 0xD1,
    POP_DE = 0xD2,
    PUSH_DE = 0xD5,
    RST_10H = 0xD7,
    RET_C = 0xD8,
    RETI = 0xD9,
    RST_18H = 0xDF,

    // Ex
    POP_HL = 0xE1,
    PUSH_HL = 0xE5,
    RST_20H = 0xE7,
    JP_HLcontents = 0xE9,
    RST_28H = 0xEF,

    // Fx
    POP_AF = 0xF1,
    DI = 0xF3,
    PUSH_AF = 0xF5,
    RST_30H = 0xF7,
    LD_SP_HL = 0xF9,
    EI = 0xFB,
    RST_38H = 0xFF,
}

pub enum TwoByteOpCode {
    // 0x
    LD_B_D8 = 0x06,
    LD_C_D8 = 0x08,

    // 1x
    STOP = 0x10,
    LD_D_D8 = 0x16,
    JR_R8 = 0x18,
    LD_E_D8 = 0x1E,

    // 2x
    JR_NZ_R8 = 0x20,
    LD_H_D8 = 0x26,
    JR_Z_R8 = 0x28,
    LD_L_D8 = 0x2E,

    // 3x
    JR_NC_R8 = 0x30,
    LD_HLcontents_D8 = 0x36,
    JR_C_R8 = 0x38,
    LD_A_D8 = 0x3E,

    // Cx
    ADD_A_D8 = 0xC6,
    ADC_A_D8 = 0xCE,

    // Dx
    SUB_D8 = 0xD6,
    SBC_A_D8 = 0xDE,

    // Ex
    LDH_A8contents_A = 0xE0, // TODO: ???
    LD_Ccontents_A = 0xE2,   // TODO: ???
    AND_D8 = 0xE6,
    ADD_SP_R8 = 0xE8,
    XOR_D8 = 0xEE,

    // Fx
    LDH_A_A8contents = 0xF0, // TODO: ???
    LD_A_Ccontents = 0xF2,
    OR_D8 = 0xF6,
    LD_HL_SPplusR8 = 0xF8,
    CP_D8 = 0xFE,
}

pub enum ThreeByteOpCode {
    // 0x
    LD_BC_D16 = 0x01,
    LD_A16contents_SP = 0x08, // TODO: ???

    // 1x
    LD_DE_D16 = 0x11,

    // 2x
    LD_HL_D16 = 0x21,

    // 3x
    LD_SP_D16 = 0x31,

    // Cx
    JP_NZ_A16 = 0xC2,
    JP_A16 = 0xC3,
    CALL_NZ_A16 = 0xC4,
    JP_Z_A16 = 0xCA,
    CALL_Z_A16 = 0xCC,
    CALL_A16 = 0xCD,

    // Dx
    JP_NC_A16 = 0xD2,
    CALL_NC_A16 = 0xD4,
    JP_C_16 = 0xDA,
    CALL_C_A16 = 0xDC,

    // Ex
    LD_A16contents_A = 0xEA, // TODO: ???

    // Fx
    LD_A_A16contents = 0xFA, // TODO: ???
}

pub enum PrefixOpCode {
    // 0x
    RLC_B = 0x00,
    RLC_C = 0x01,
    RLC_D = 0x02,
    RLC_E = 0x03,
    RLC_H = 0x04,
    RLC_L = 0x05,
    RLC_HLcontents = 0x06,
    RLC_A = 0x07,
    RRC_B = 0x08,
    RRC_C = 0x09,
    RRC_D = 0x0A,
    RRC_E = 0x0B,
    RRC_H = 0x0C,
    RRC_L = 0x0D,
    RRC_HLcontents = 0x0E,
    RRC_A = 0x0F,

    // 1x
    RL_B = 0x10,
    RL_C = 0x11,
    RL_D = 0x12,
    RL_E = 0x13,
    RL_H = 0x14,
    RL_L = 0x15,
    RL_HLcontents = 0x16,
    RL_A = 0x17,
    RR_B = 0x18,
    RR_C = 0x19,
    RR_D = 0x1A,
    RR_E = 0x1B,
    RR_H = 0x1C,
    RR_L = 0x1D,
    RR_HLcontents = 0x1E,
    RR_A = 0x1F,

    // 2x
    SLA_B = 0x20,
    SLA_C = 0x21,
    SLA_D = 0x22,
    SLA_E = 0x23,
    SLA_H = 0x24,
    SLA_L = 0x25,
    SLA_HLcontents = 0x26,
    SLA_A = 0x27,
    SRA_B = 0x28,
    SRA_C = 0x29,
    SRA_D = 0x2A,
    SRA_E = 0x2B,
    SRA_H = 0x2C,
    SRA_L = 0x2D,
    SRA_HLcontents = 0x2E,
    SRA_A = 0x2F,

    // 3x
    SWAP_B = 0x30,
    SWAP_C = 0x31,
    SWAP_D = 0x32,
    SWAP_E = 0x33,
    SWAP_H = 0x34,
    SWAP_L = 0x35,
    SWAP_HLcontents = 0x36,
    SWAP_A = 0x37,
    SRL_B = 0x38,
    SRL_C = 0x39,
    SRL_D = 0x3A,
    SRL_E = 0x3B,
    SRL_H = 0x3C,
    SRL_L = 0x3D,
    SRL_HLcontents = 0x3E,
    SRL_A = 0x3F,

    // 4x
    BIT_0_B = 0x40,
    BIT_0_C = 0x41,
    BIT_0_D = 0x42,
    BIT_0_E = 0x43,
    BIT_0_H = 0x44,
    BIT_0_L = 0x45,
    BIT_0_HLcontents = 0x46,
    BIT_0_A = 0x47,
    BIT_1_B = 0x48,
    BIT_1_C = 0x49,
    BIT_1_D = 0x4A,
    BIT_1_E = 0x4B,
    BIT_1_H = 0x4C,
    BIT_1_L = 0x4D,
    BIT_1_HLcontents = 0x4E,
    BIT_1_A = 0x4F,

    // 5x
    BIT_2_B = 0x50,
    BIT_2_C = 0x51,
    BIT_2_D = 0x52,
    BIT_2_E = 0x53,
    BIT_2_H = 0x54,
    BIT_2_L = 0x55,
    BIT_2_HLcontents = 0x56,
    BIT_2_A = 0x57,
    BIT_3_B = 0x58,
    BIT_3_C = 0x59,
    BIT_3_D = 0x5A,
    BIT_3_E = 0x5B,
    BIT_3_H = 0x5C,
    BIT_3_L = 0x5D,
    BIT_3_HLcontents = 0x5E,
    BIT_3_A = 0x5F,

    // 6x
    BIT_4_B = 0x60,
    BIT_4_C = 0x61,
    BIT_4_D = 0x62,
    BIT_4_E = 0x63,
    BIT_4_H = 0x64,
    BIT_4_L = 0x65,
    BIT_4_HLcontents = 0x66,
    BIT_4_A = 0x67,
    BIT_5_B = 0x68,
    BIT_5_C = 0x69,
    BIT_5_D = 0x6A,
    BIT_5_E = 0x6B,
    BIT_5_H = 0x6C,
    BIT_5_L = 0x6D,
    BIT_5_HLcontents = 0x6E,
    BIT_5_A = 0x6F,

    // 7x
    BIT_6_B = 0x70,
    BIT_6_C = 0x71,
    BIT_6_D = 0x72,
    BIT_6_E = 0x73,
    BIT_6_H = 0x74,
    BIT_6_L = 0x75,
    BIT_6_HLcontents = 0x76,
    BIT_6_A = 0x77,
    BIT_7_B = 0x78,
    BIT_7_C = 0x79,
    BIT_7_D = 0x7A,
    BIT_7_E = 0x7B,
    BIT_7_H = 0x7C,
    BIT_7_L = 0x7D,
    BIT_7_HLcontents = 0x7E,
    BIT_7_A = 0x7F,

    // 8x
    RES_0_B = 0x80,
    RES_0_C = 0x81,
    RES_0_D = 0x82,
    RES_0_E = 0x83,
    RES_0_H = 0x84,
    RES_0_L = 0x85,
    RES_0_HLcontents = 0x86,
    RES_0_A = 0x87,
    RES_1_B = 0x88,
    RES_1_C = 0x89,
    RES_1_D = 0x8A,
    RES_1_E = 0x8B,
    RES_1_H = 0x8C,
    RES_1_L = 0x8D,
    RES_1_HLcontents = 0x8E,
    RES_1_A = 0x8F,

    // 9x
    RES_2_B = 0x90,
    RES_2_C = 0x91,
    RES_2_D = 0x92,
    RES_2_E = 0x93,
    RES_2_H = 0x94,
    RES_2_L = 0x95,
    RES_2_HLcontents = 0x96,
    RES_2_A = 0x97,
    RES_3_B = 0x98,
    RES_3_C = 0x99,
    RES_3_D = 0x9A,
    RES_3_E = 0x9B,
    RES_3_H = 0x9C,
    RES_3_L = 0x9D,
    RES_3_HLcontents = 0x9E,
    RES_3_A = 0x9F,

    // Ax
    RES_4_B = 0xA0,
    RES_4_C = 0xA1,
    RES_4_D = 0xA2,
    RES_4_E = 0xA3,
    RES_4_H = 0xA4,
    RES_4_L = 0xA5,
    RES_4_HLcontents = 0xA6,
    RES_4_A = 0xA7,
    RES_5_B = 0xA8,
    RES_5_C = 0xA9,
    RES_5_D = 0xAA,
    RES_5_E = 0xAB,
    RES_5_H = 0xAC,
    RES_5_L = 0xAD,
    RES_5_HLcontents = 0xAE,
    RES_5_A = 0xAF,

    // Bx
    RES_6_B = 0xB0,
    RES_6_C = 0xB1,
    RES_6_D = 0xB2,
    RES_6_E = 0xB3,
    RES_6_H = 0xB4,
    RES_6_L = 0xB5,
    RES_6_HLcontents = 0xB6,
    RES_6_A = 0xB7,
    RES_7_B = 0xB8,
    RES_7_C = 0xB9,
    RES_7_D = 0xBA,
    RES_7_E = 0xBB,
    RES_7_H = 0xBC,
    RES_7_L = 0xBD,
    RES_7_HLcontents = 0xBE,
    RES_7_A = 0xBF,

    // Cx
    SET_0_B = 0xC0,
    SET_0_C = 0xC1,
    SET_0_D = 0xC2,
    SET_0_E = 0xC3,
    SET_0_H = 0xC4,
    SET_0_L = 0xC5,
    SET_0_HLcontents = 0xC6,
    SET_0_A = 0xC7,
    SET_1_B = 0xC8,
    SET_1_C = 0xC9,
    SET_1_D = 0xCA,
    SET_1_E = 0xCB,
    SET_1_H = 0xCC,
    SET_1_L = 0xCD,
    SET_1_HLcontents = 0xCE,
    SET_1_A = 0xCF,

    // Dx
    SET_2_B = 0xD0,
    SET_2_C = 0xD1,
    SET_2_D = 0xD2,
    SET_2_E = 0xD3,
    SET_2_H = 0xD4,
    SET_2_L = 0xD5,
    SET_2_HLcontents = 0xD6,
    SET_2_A = 0xD7,
    SET_3_B = 0xD8,
    SET_3_C = 0xD9,
    SET_3_D = 0xDA,
    SET_3_E = 0xDB,
    SET_3_H = 0xDC,
    SET_3_L = 0xDD,
    SET_3_HLcontents = 0xDE,
    SET_3_A = 0xDF,

    // Ex
    SET_4_B = 0xE0,
    SET_4_C = 0xE1,
    SET_4_D = 0xE2,
    SET_4_E = 0xE3,
    SET_4_H = 0xE4,
    SET_4_L = 0xE5,
    SET_4_HLcontents = 0xE6,
    SET_4_A = 0xE7,
    SET_5_B = 0xE8,
    SET_5_C = 0xE9,
    SET_5_D = 0xEA,
    SET_5_E = 0xEB,
    SET_5_H = 0xEC,
    SET_5_L = 0xED,
    SET_5_HLcontents = 0xEE,
    SET_5_A = 0xEF,

    // Fx
    SET_6_B = 0xF0,
    SET_6_C = 0xF1,
    SET_6_D = 0xF2,
    SET_6_E = 0xF3,
    SET_6_H = 0xF4,
    SET_6_L = 0xF5,
    SET_6_HLcontents = 0xF6,
    SET_6_A = 0xF7,
    SET_7_B = 0xF8,
    SET_7_C = 0xF9,
    SET_7_D = 0xFA,
    SET_7_E = 0xFB,
    SET_7_H = 0xFC,
    SET_7_L = 0xFD,
    SET_7_HLcontents = 0xFE,
    SET_7_A = 0xFF,
}

pub fn execute_opcode(cpu: &mut cpu::Cpu, code: OpCode) {
    match code {
        // Increment Register
        OpCode::INC_B => {
            increment_8bit_register(cpu, &RegByte::B);
        }
        OpCode::INC_C => {
            increment_8bit_register(cpu, &RegByte::C);
        }
        OpCode::INC_D => {
            increment_8bit_register(cpu, &RegByte::D);
        }
        OpCode::INC_E => {
            increment_8bit_register(cpu, &RegByte::E);
        }
        OpCode::INC_H => {
            increment_8bit_register(cpu, &RegByte::H);
        }
        OpCode::INC_L => {
            increment_8bit_register(cpu, &RegByte::L);
        }
        OpCode::INC_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            let (result, zero, half_carry) = increment_8bit(value);
            cpu.registers.write_flag(RegFlag::Zero, zero);
            cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.memory.write_byte(address, result);

            cpu.clock.cycle_clock(3);
        }
        OpCode::INC_A => {
            increment_8bit_register(cpu, &RegByte::A);
        }

        // Decrement 8 bit value
        OpCode::DEC_B => {
            decrement_8bit_register(cpu, &RegByte::B);
        }
        OpCode::DEC_C => {
            decrement_8bit_register(cpu, &RegByte::C);
        }
        OpCode::DEC_D => {
            decrement_8bit_register(cpu, &RegByte::D);
        }
        OpCode::DEC_E => {
            decrement_8bit_register(cpu, &RegByte::E);
        }
        OpCode::DEC_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            let (result, zero, half_borrow) = decrement_8bit(value);
            cpu.registers.write_flag(RegFlag::Zero, zero);
            cpu.registers.write_flag(RegFlag::HalfCarry, half_borrow);
            cpu.registers.write_flag(RegFlag::Subtraction, true);
            cpu.registers.write_byte(&RegByte::H, result);

            cpu.clock.cycle_clock(1);
        }
        OpCode::DEC_L => {
            decrement_8bit_register(cpu, &RegByte::L);
        }
        OpCode::DEC_HL_MEM => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            let (result, zero, half_borrow) = decrement_8bit(value);
            cpu.registers.write_flag(RegFlag::Zero, zero);
            cpu.registers.write_flag(RegFlag::HalfCarry, half_borrow);
            cpu.registers.write_flag(RegFlag::Subtraction, true);
            cpu.memory.write_byte(address, result);

            cpu.clock.cycle_clock(3);
        }
        OpCode::DEC_A => {
            decrement_8bit_register(cpu, &RegByte::A);
        }

        // Decrement virtual_registers_contents
        OpCode::DEC_BC => {
            let value = cpu.registers.read_word(&RegWord::BC);
            cpu.registers
                .write_word(&RegWord::BC, value.wrapping_sub(1));

            cpu.clock.cycle_clock(2);
        }
        OpCode::DEC_DE => {
            let value = cpu.registers.read_word(&RegWord::DE);
            cpu.registers
                .write_word(&RegWord::DE, value.wrapping_sub(1));

            cpu.clock.cycle_clock(2);
        }
        OpCode::DEC_HL => {
            let value = cpu.registers.read_word(&RegWord::HL);
            cpu.registers
                .write_word(&RegWord::HL, value.wrapping_sub(1));

            cpu.clock.cycle_clock(2);
        }
        OpCode::DEC_SP => {
            let value = cpu.registers.read_word(&RegWord::SP);
            cpu.registers
                .write_word(&RegWord::SP, value.wrapping_sub(1));

            cpu.clock.cycle_clock(2);
        }

        // Increment virtual registers contents
        OpCode::INC_BC => {
            let value = cpu.registers.read_word(&RegWord::BC);
            cpu.registers
                .write_word(&RegWord::BC, value.wrapping_add(1));

            cpu.clock.cycle_clock(2);
        }
        OpCode::INC_DE => {
            let value = cpu.registers.read_word(&RegWord::DE);
            cpu.registers
                .write_word(&RegWord::DE, value.wrapping_add(1));

            cpu.clock.cycle_clock(2);
        }
        OpCode::INC_HL_REG => {
            let value = cpu.registers.read_word(&RegWord::HL);
            cpu.registers
                .write_word(&RegWord::HL, value.wrapping_add(1));

            cpu.clock.cycle_clock(2);
        }
        OpCode::INC_SP => {
            let value = cpu.registers.read_word(&RegWord::SP);
            cpu.registers
                .write_word(&RegWord::SP, value.wrapping_add(1));

            cpu.clock.cycle_clock(2);
        }

        // Load next two data-bytes into Virtual Register
        OpCode::LD_BC_D16 => {
            let low_byte = fetch_byte();
            let high_byte = fetch_byte();
            let word = build_2_byte_word(low_byte, high_byte);
            cpu.registers.write_word(&RegWord::BC, word);
            cpu.clock.cycle_clock(3);
        }
        OpCode::LD_DE_D16 => {
            let low_byte = fetch_byte();
            let high_byte = fetch_byte();
            let word = build_2_byte_word(low_byte, high_byte);
            cpu.registers.write_word(&RegWord::DE, word);
            cpu.clock.cycle_clock(3);
        }
        OpCode::LD_HL_D16 => {
            let low_byte = fetch_byte();
            let high_byte = fetch_byte();
            let word = build_2_byte_word(low_byte, high_byte);
            cpu.registers.write_word(&RegWord::HL, word);
            cpu.clock.cycle_clock(3);
        }
        OpCode::LD_SP_D16 => {
            let low_byte = fetch_byte();
            let high_byte = fetch_byte();
            let word = build_2_byte_word(low_byte, high_byte);
            cpu.registers.write_word(&RegWord::SP, word);
            cpu.clock.cycle_clock(3);
        }

        // Load Virtual Registers contents-memory to A
        OpCode::LD_A_BC => {
            load_virtual_register_to_a(cpu, &RegWord::BC);
        }
        OpCode::LD_A_DE => {
            load_virtual_register_to_a(cpu, &RegWord::DE);
        }
        OpCode::LD_A_HLI => {
            load_virtual_register_to_a(cpu, &RegWord::HL);
            cpu.registers
                .write_word(&RegWord::HL, (cpu.registers.read_word(&RegWord::HL) + 1));
        }
        OpCode::LD_A_HLD => {
            load_virtual_register_to_a(cpu, &RegWord::HL);
            cpu.registers
                .write_word(&RegWord::HL, (cpu.registers.read_word(&RegWord::HL) - 1));
        }

        // Store register A's contents inside memory location
        OpCode::LD_BC_A => load_a_to_virtual_registers_byte(cpu, &RegWord::BC),
        OpCode::LD_DE_A => load_a_to_virtual_registers_byte(cpu, &RegWord::DE),
        OpCode::LD_HLI_A => {
            load_a_to_virtual_registers_byte(cpu, &RegWord::HL);
            cpu.registers
                .write_word(&RegWord::HL, (1 + cpu.registers.read_word(&RegWord::HL)));
        }
        OpCode::LD_HLD_A => {
            load_a_to_virtual_registers_byte(cpu, &RegWord::HL);
            cpu.registers
                .write_word(&RegWord::HL, (cpu.registers.read_word(&RegWord::HL) - 1));
        }

        // Rotate contents of A register
        OpCode::RLCA => {
            // TODO test this
            let word = cpu.registers.read_byte(&RegByte::A);
            let bit_7: u8 = 0x80 & word;
            let mut word_shifted = word << 1;
            if (bit_7 == 0x80) {
                word_shifted = word_shifted | 0x01
            }
            cpu.registers.write_flag(RegFlag::Carry, bit_7 == 0x80);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers.write_byte(&RegByte::A, word_shifted);

            cpu.clock.cycle_clock(1);
        }
        OpCode::RLA => {
            // TODO test this
            let word = cpu.registers.read_byte(&RegByte::A);
            let bit_7: u8 = 0x80 & word;
            let word_shifted = word << 1;
            let carry_value: u8;
            let mask;
            if (cpu.registers.read_flag(RegFlag::Carry)) {
                carry_value = 1;
                mask = 0x01;
            } else {
                carry_value = 0;
                mask = 0x00;
            }
            // Set bit0 to carry_value
            let word_shifted = word_shifted | mask;

            cpu.registers.write_flag(RegFlag::Carry, bit_7 == 0x80);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers.write_byte(&RegByte::A, word_shifted);

            cpu.clock.cycle_clock(1);
        }
        OpCode::RRCA => {
            // TODO test this
            let word = cpu.registers.read_byte(&RegByte::A);
            let bit_0: u8 = 0x01 & word;
            let mut word_shifted = word >> 1;
            if (bit_0 == 0x01) {
                word_shifted = word_shifted | 0x80
            }
            cpu.registers.write_flag(RegFlag::Carry, bit_0 == 0x01);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers.write_byte(&RegByte::A, word_shifted);

            cpu.clock.cycle_clock(1);
        }
        OpCode::RRA => {
            // TODO test this
            let word = cpu.registers.read_byte(&RegByte::A);
            let bit_0: u8 = 0x01 & word;
            let word_shifted = word >> 1;
            let carry_value: u8;
            let mask;
            if (cpu.registers.read_flag(RegFlag::Carry)) {
                carry_value = 1;
                mask = 0x80;
            } else {
                carry_value = 0;
                mask = 0x00;
            }
            // Set bit7 to carry_value
            let word_shifted = word_shifted | mask;

            cpu.registers.write_flag(RegFlag::Carry, bit_0 == 0x01);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::Zero, false);
            cpu.registers.write_byte(&RegByte::A, word_shifted);

            cpu.clock.cycle_clock(1);
        }

        // Set Carry Flag (and halfcarry/subtraction)
        OpCode::SCF => {
            cpu.registers.write_flag(RegFlag::Subtraction, false);
            cpu.registers.write_flag(RegFlag::HalfCarry, false);
            cpu.registers.write_flag(RegFlag::Carry, true);
        }

        // Add 16-bit register to HL
        OpCode::ADD_HL_BC => {
            add_contents_virtual_register_to_virtual_register(cpu, &RegWord::HL, &RegWord::BC);
        }
        OpCode::ADD_HL_DE => {
            add_contents_virtual_register_to_virtual_register(cpu, &RegWord::HL, &RegWord::DE);
        }
        OpCode::ADD_HL_HL => {
            add_contents_virtual_register_to_virtual_register(cpu, &RegWord::HL, &RegWord::HL);
        }
        OpCode::ADD_HL_SP => {
            add_contents_virtual_register_to_virtual_register(cpu, &RegWord::HL, &RegWord::SP);
        }

        // Load Register into B
        OpCode::LD_B_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::B);
        }
        OpCode::LD_B_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::B);
        }
        OpCode::LD_B_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::B);
        }
        OpCode::LD_B_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::B);
        }
        OpCode::LD_B_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::B);
        }
        OpCode::LD_B_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::B);
        }
        OpCode::LD_B_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1); // TODO - Talk with tint, kind of hacky...
            cpu.registers.write_byte(&RegByte::B, value);
        }
        OpCode::LD_B_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::B);
        }

        // Load Register Into C
        OpCode::LD_C_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::C);
        }
        OpCode::LD_C_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::C);
        }
        OpCode::LD_C_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::C);
        }
        OpCode::LD_C_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::C);
        }
        OpCode::LD_C_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::C);
        }
        OpCode::LD_C_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::C);
        }
        OpCode::LD_C_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            cpu.registers.write_byte(&RegByte::C, value);
        }
        OpCode::LD_C_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::C);
        }

        // Load Register into D
        OpCode::LD_D_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::D);
        }
        OpCode::LD_D_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::D);
        }
        OpCode::LD_D_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::D);
        }
        OpCode::LD_D_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::D);
        }
        OpCode::LD_D_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::D);
        }
        OpCode::LD_D_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::D);
        }
        OpCode::LD_D_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            cpu.registers.write_byte(&RegByte::D, value);
        }
        OpCode::LD_D_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::D);
        }

        // Load Register into E
        OpCode::LD_E_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::E);
        }
        OpCode::LD_E_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::E);
        }
        OpCode::LD_E_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::E);
        }
        OpCode::LD_E_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::E);
        }
        OpCode::LD_E_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::E);
        }
        OpCode::LD_E_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::E);
        }
        OpCode::LD_E_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            cpu.registers.write_byte(&RegByte::E, value);
        }
        OpCode::LD_E_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::E);
        }

        // Load Register into H
        OpCode::LD_H_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::H);
        }
        OpCode::LD_H_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::H);
        }
        OpCode::LD_H_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::H);
        }
        OpCode::LD_H_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::H);
        }
        OpCode::LD_H_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::H);
        }
        OpCode::LD_H_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::H);
        }
        OpCode::LD_H_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            cpu.registers.write_byte(&RegByte::H, value);
        }
        OpCode::LD_H_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::H);
        }

        // Load Register into L
        OpCode::LD_L_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::L);
        }
        OpCode::LD_L_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::L);
        }
        OpCode::LD_L_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::L);
        }
        OpCode::LD_L_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::L);
        }
        OpCode::LD_L_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::L);
        }
        OpCode::LD_L_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::L);
        }
        OpCode::LD_L_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            cpu.registers.write_byte(&RegByte::L, value);
        }
        OpCode::LD_L_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::L);
        }

        // Load Register into A
        OpCode::LD_A_B => {
            load_register_to_register(cpu, &RegByte::B, &RegByte::A);
        }
        OpCode::LD_A_C => {
            load_register_to_register(cpu, &RegByte::C, &RegByte::A);
        }
        OpCode::LD_A_D => {
            load_register_to_register(cpu, &RegByte::D, &RegByte::A);
        }
        OpCode::LD_A_E => {
            load_register_to_register(cpu, &RegByte::E, &RegByte::A);
        }
        OpCode::LD_A_H => {
            load_register_to_register(cpu, &RegByte::H, &RegByte::A);
        }
        OpCode::LD_A_L => {
            load_register_to_register(cpu, &RegByte::L, &RegByte::A);
        }
        OpCode::LD_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            cpu.registers.write_byte(&RegByte::A, value);
        }
        OpCode::LD_A_A => {
            load_register_to_register(cpu, &RegByte::A, &RegByte::A);
        }

        // Add Register to A
        OpCode::ADD_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1); // TODO - Talk with tint, kind of hacky...
            add_to_a(cpu, value);
        }
        OpCode::ADD_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            add_to_a(cpu, value);
        }

        // Add Register with carry to A
        OpCode::ADC_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            add_to_a_carry(cpu, value);
        }
        OpCode::ADC_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            add_to_a_carry(cpu, value);
        }

        // Subtract Register from A
        OpCode::SUB_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            subtract_from_a(cpu, value);
        }
        OpCode::SUB_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            subtract_from_a(cpu, value);
        }

        // Subtract (Register + Carry) from A
        OpCode::SBC_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            subtract_from_a_carry(cpu, value);
        }
        OpCode::SBC_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            subtract_from_a_carry(cpu, value);
        }

        // Set A to Bitwise AND from register
        OpCode::AND_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            bitwise_and_a(cpu, value);
        }
        OpCode::AND_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            bitwise_and_a(cpu, value);
        }

        // Set A to Bitwise XOR from register
        OpCode::XOR_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            bitwise_xor_a(cpu, value);
        }
        OpCode::XOR_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            bitwise_xor_a(cpu, value);
        }

        // Set A to Bitwise OR from register
        OpCode::OR_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            cpu.clock.cycle_clock(1);
            bitwise_or_a(cpu, value);
        }
        OpCode::OR_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            bitwise_or_a(cpu, value);
        }

        // Compare A to register
        OpCode::CP_A_B => {
            let value = cpu.registers.read_byte(&RegByte::B);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_C => {
            let value = cpu.registers.read_byte(&RegByte::C);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_D => {
            let value = cpu.registers.read_byte(&RegByte::D);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_E => {
            let value = cpu.registers.read_byte(&RegByte::E);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_H => {
            let value = cpu.registers.read_byte(&RegByte::H);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_L => {
            let value = cpu.registers.read_byte(&RegByte::L);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_HL => {
            let address = cpu.registers.read_word(&RegWord::HL);
            let value = cpu.memory.read_byte(address);
            compare_to_a(cpu, value);
        }
        OpCode::CP_A_A => {
            let value = cpu.registers.read_byte(&RegByte::A);
            compare_to_a(cpu, value);
        }
        _ => panic!("Invalid One Byte OpCode! Yoinked by Crab Claw!"),
    }
}

pub fn execute_two_byte_opcode(cpu: &mut cpu::Cpu, code: OpCode, byte1: u8) {
    match code {
        _ => panic!("Invalid Two Byte OpCode! Yoinked by Lobster Claw!"),
    }
}

// TODO - Name change, will be used soon for a lot.
// pub fn get_8bit_stored_in_16bit(cpu: &mut cpu::Cpu, address: u16) -> u8 {
//     let value = cpu.memory.read_byte(address);
//     value
// }

// TODO - PLACEHOLDER FUNCTIONALITY
pub fn fetch_byte() -> u8 {
    13
}

pub fn increment_8bit_register(cpu: &mut cpu::Cpu, register: &RegByte) {
    let value = cpu.registers.read_byte(register);
    let (result, zero, half_carry) = increment_8bit(value);
    cpu.registers.write_flag(RegFlag::Zero, zero);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_byte(register, result);
}

pub fn decrement_8bit_register(cpu: &mut cpu::Cpu, register: &RegByte) {
    let value = cpu.registers.read_byte(register);
    let (result, zero, half_borrow) = decrement_8bit(value);
    cpu.registers.write_flag(RegFlag::Zero, zero);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_borrow);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_byte(register, result);

    cpu.clock.cycle_clock(1);
}

pub fn add_contents_virtual_register_to_virtual_register(
    cpu: &mut cpu::Cpu,
    receiver: &RegWord,
    sender: &RegWord,
) {
    // TODO - finish after fixing references in registers.rs
    let receiver_value = cpu.registers.read_word(receiver);
    let sender_value = cpu.registers.read_word(&sender);
    let (result, half_carry, carry) = add_16_bit_and_16_bit(receiver_value, sender_value);

    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
    cpu.registers.write_flag(RegFlag::Carry, carry);

    cpu.registers.write_word(receiver, result);

    cpu.clock.cycle_clock(2);
}

pub fn load_virtual_register_to_a(cpu: &mut cpu::Cpu, register: &RegWord) {
    let value = cpu.memory.read_byte(cpu.registers.read_word(register));
    cpu.registers.write_byte(&RegByte::A, value);
}

pub fn load_a_to_virtual_registers_byte(cpu: &mut cpu::Cpu, v_register: &RegWord) {
    let value = cpu.registers.read_byte(&RegByte::A);
    let address = cpu.registers.read_word(v_register);
    cpu.memory.write_byte(address, value);
    cpu.clock.cycle_clock(2);
}

fn add_16_bit_and_16_bit(num1: u16, num2: u16) -> (u16, bool, bool) {
    let (result, is_carry) = num1.overflowing_add(num2);

    // check if we would have a carry
    let is_half_carry = calculate_16_bit_half_carry(num1, num2);

    (result, is_half_carry, is_carry)
}

pub fn increment_8bit(value: u8) -> (u8, bool, bool) {
    let result = value.wrapping_add(1);

    // check if we would have a carry
    let half_carry = calculate_half_carry(value, 1);

    let zero = result == 0;

    (result, zero, half_carry)
}

pub fn decrement_8bit(value: u8) -> (u8, bool, bool) {
    let (result, is_borrow) = value.overflowing_sub(1);
    let zero = result == 0;

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (value & 0xF) < (1 & 0xF);

    (result, zero, is_half_borrow)
}

pub fn build_2_byte_word(lower_byte: u8, upper_byte: u8) -> u16 {
    // Left shifting our upper_byte into the 8 most important bits, then build 2-byte word
    let new_word = ((upper_byte as u16) << 8) | lower_byte as u16;
    new_word
}

pub fn calculate_half_carry(byte1: u8, byte2: u8) -> bool {
    let half_carry = (((byte1 & 0xF) + (byte2 & 0xF)) & 0x10) == 0x10;
    half_carry
}

pub fn calculate_16_bit_half_carry(word1: u16, word2: u16) -> bool {
    let half_carry = (((word1 & 0xFFF) + (word2 & 0xFFF)) & 0x1000) == 0x1000;
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

pub fn add_to_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let (result, is_carry) = a_value.overflowing_add(value);

    // check if we would have a carry
    let half_carry = calculate_half_carry(a_value, value);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, half_carry);
    cpu.registers.write_flag(RegFlag::Carry, is_carry);

    cpu.registers.write_byte(&RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

pub fn add_to_a_carry(cpu: &mut cpu::Cpu, value: u8) {
    if !cpu.registers.read_flag(RegFlag::Carry) {
        return add_to_a(cpu, value);
    }

    let a_value = cpu.registers.read_byte(&RegByte::A);
    let (result, is_overflow) = a_value.overflowing_add(value);

    // Check for half carry between A + B, then result + carry
    let half_carry = calculate_half_carry(a_value, value);
    let half_carry2 = calculate_half_carry(result, 1);

    let (result2, is_overflow2) = result.overflowing_add(1);

    cpu.registers.write_flag(RegFlag::Zero, result2 == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers
        .write_flag(RegFlag::HalfCarry, half_carry | half_carry2);
    cpu.registers
        .write_flag(RegFlag::Carry, is_overflow | is_overflow2);

    cpu.registers.write_byte(&RegByte::A, result2);

    cpu.clock.cycle_clock(1);
}

pub fn load_register_to_register(
    cpu: &mut cpu::Cpu,
    sending_register: &RegByte,
    receiving_register: &RegByte,
) {
    cpu.registers.write_byte(
        receiving_register,
        cpu.registers.read_byte(sending_register),
    );

    cpu.clock.cycle_clock(1);
}

pub fn subtract_from_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let (result, is_borrow) = a_value.overflowing_sub(value);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (a_value & 0xF) < (value & 0xF);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_flag(RegFlag::HalfCarry, is_half_borrow);
    cpu.registers.write_flag(RegFlag::Carry, is_borrow);

    cpu.registers.write_byte(&RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

// TODO could maybe condense with subtract_from_a?
pub fn subtract_from_a_carry(cpu: &mut cpu::Cpu, value: u8) {
    if !cpu.registers.read_flag(RegFlag::Carry) {
        return subtract_from_a(cpu, value);
    }

    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value.wrapping_sub(value).wrapping_sub(1);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (a_value & 0xF) < ((value & 0xF) + 1);
    let is_borrow = value == 0xFF || a_value < (value + 1);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_flag(RegFlag::HalfCarry, is_half_borrow);
    cpu.registers.write_flag(RegFlag::Carry, is_borrow);

    cpu.registers.write_byte(&RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

pub fn bitwise_and_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value & value;

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, true);
    cpu.registers.write_flag(RegFlag::Carry, false);

    cpu.registers.write_byte(&RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

pub fn bitwise_xor_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value ^ value;

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, false);
    cpu.registers.write_flag(RegFlag::Carry, false);

    cpu.registers.write_byte(&RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

pub fn bitwise_or_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let result = a_value | value;

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, false);
    cpu.registers.write_flag(RegFlag::HalfCarry, false);
    cpu.registers.write_flag(RegFlag::Carry, false);

    cpu.registers.write_byte(&RegByte::A, result);

    cpu.clock.cycle_clock(1);
}

pub fn compare_to_a(cpu: &mut cpu::Cpu, value: u8) {
    let a_value = cpu.registers.read_byte(&RegByte::A);
    let (result, is_borrow) = a_value.overflowing_sub(value);

    // check if we would have to borrow from the 5th bit
    let is_half_borrow = (a_value & 0xF) < (value & 0xF);

    cpu.registers.write_flag(RegFlag::Zero, result == 0);
    cpu.registers.write_flag(RegFlag::Subtraction, true);
    cpu.registers.write_flag(RegFlag::HalfCarry, is_half_borrow);
    cpu.registers.write_flag(RegFlag::Carry, is_borrow);

    cpu.clock.cycle_clock(1);
}

#[cfg(test)]
mod tests {
    use super::*;

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

        cpu.registers.write_byte(&RegByte::A, 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        cpu.registers.write_byte(&RegByte::B, 24);
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

        cpu.registers.write_byte(&RegByte::A, 15);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 15);
        cpu.registers.write_byte(&RegByte::B, 1);
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

        cpu.registers.write_byte(&RegByte::A, 243);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 243);
        cpu.registers.write_byte(&RegByte::B, 25);
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

        cpu.registers.write_byte(&RegByte::A, 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);
        cpu.registers.write_byte(&RegByte::B, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::ADD_A_B);

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

        execute_opcode(&mut cpu, OpCode::ADD_A_HL);

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

        execute_opcode(&mut cpu, OpCode::ADC_A_B);

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

        cpu.registers.write_byte(&RegByte::A, 100);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 100);
        cpu.registers.write_byte(&RegByte::D, 25);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 25);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::ADC_A_A);

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

        execute_opcode(&mut cpu, OpCode::ADC_A_A);

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

        execute_opcode(&mut cpu, OpCode::ADC_A_D);

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

        execute_opcode(&mut cpu, OpCode::LD_A_B);

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

        execute_opcode(&mut cpu, OpCode::LD_D_E);

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

        execute_opcode(&mut cpu, OpCode::LD_D_E);

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

        execute_opcode(&mut cpu, OpCode::AND_A_B);

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

        execute_opcode(&mut cpu, OpCode::AND_A_H);

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

        execute_opcode(&mut cpu, OpCode::AND_A_A);

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

        execute_opcode(&mut cpu, OpCode::XOR_A_B);

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

        execute_opcode(&mut cpu, OpCode::XOR_A_A);

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

        execute_opcode(&mut cpu, OpCode::XOR_A_E);

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

        execute_opcode(&mut cpu, OpCode::INC_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::DEC_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 255);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::INC_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::B), 25);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::INC_C);

        assert_eq!(cpu.registers.read_byte(&RegByte::C), 34);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::DEC_D);

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

        execute_opcode(&mut cpu, OpCode::OR_A_B);

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

        execute_opcode(&mut cpu, OpCode::OR_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 0);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Rotate A-Register left using RLCA or RLA
    #[test]
    fn rotate_register_a_left_producing_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b10101010);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b10101010);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::RLCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b01010101);
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

        execute_opcode(&mut cpu, OpCode::RLCA);

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

        execute_opcode(&mut cpu, OpCode::RLA);

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

        execute_opcode(&mut cpu, OpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0001_1111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::RLA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0011_1110);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }

    // Rotate Right A-register
    #[test]
    fn rotate_register_a_right_through_carry_without_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b1111_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::RRA);

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

        execute_opcode(&mut cpu, OpCode::RRCA);

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

        execute_opcode(&mut cpu, OpCode::RRCA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1011_1110);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn rotate_register_a_right_through_carry_with_carry() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_byte(&RegByte::A, 0b0000_1111);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b0000_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        cpu.registers.write_flag(RegFlag::Carry, true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::RRA);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 0b1000_0111);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::RRA);

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

        execute_opcode(&mut cpu, OpCode::LD_A_HLD);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 8999);
        assert_eq!(cpu.memory.read_byte(9000), 49);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 49);

        execute_opcode(&mut cpu, OpCode::LD_A_HLI);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 9000);
        assert_eq!(cpu.memory.read_byte(8999), 200);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 200);

        cpu.registers.write_word(&RegWord::BC, 4000);
        cpu.memory.write_byte(4000, 1);

        execute_opcode(&mut cpu, OpCode::LD_A_BC);

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

        execute_opcode(&mut cpu, OpCode::CP_A_B);

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

        execute_opcode(&mut cpu, OpCode::CP_A_B);

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

        execute_opcode(&mut cpu, OpCode::ADD_HL_BC);

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

        execute_opcode(&mut cpu, OpCode::INC_HL);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 57152);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            100
        );

        // Where HL should point after INC_HL_REG
        cpu.memory.write_byte(57153, 4);

        execute_opcode(&mut cpu, OpCode::INC_HL_REG);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 57153);
        assert_eq!(
            cpu.memory.read_byte(cpu.registers.read_word(&RegWord::HL)),
            4
        );

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        cpu.memory.write_byte(48770, 29);

        execute_opcode(&mut cpu, OpCode::ADD_HL_HL);

        assert_eq!(cpu.registers.read_word(&RegWord::HL), 48770);
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

        execute_opcode(&mut cpu, OpCode::DEC_SP);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0b0000_0000_0000_0000);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::DEC_SP);

        assert_eq!(cpu.registers.read_word(&RegWord::SP), 0b1111_1111_1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);
    }

    #[test]
    fn increment_bc_and_overflow() {
        let mut cpu = cpu::Cpu::new();

        cpu.registers.write_flag(RegFlag::Zero, false);
        cpu.registers.write_flag(RegFlag::Subtraction, true);
        cpu.registers.write_flag(RegFlag::HalfCarry, true);
        cpu.registers.write_flag(RegFlag::Carry, true);

        cpu.registers
            .write_word(&RegWord::BC, 0b1111_1111_1111_1110);

        execute_opcode(&mut cpu, OpCode::INC_BC);

        assert_eq!(cpu.registers.read_word(&RegWord::BC), 0b1111_1111_1111_1111);

        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::INC_BC);

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

        execute_opcode(&mut cpu, OpCode::LD_BC_A);

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

        execute_opcode(&mut cpu, OpCode::LD_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 43);

        // No flags change during load commands
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::ADD_A_D);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 86);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::ADD_A_A);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 172);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::DEC_A);

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

        execute_opcode(&mut cpu, OpCode::XOR_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 240);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 91);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);

        execute_opcode(&mut cpu, OpCode::ADD_A_B);

        assert_eq!(cpu.registers.read_byte(&RegByte::A), 75);
        assert_eq!(cpu.registers.read_byte(&RegByte::B), 91);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), true);

        execute_opcode(&mut cpu, OpCode::SBC_A_D);
        assert_eq!(cpu.registers.read_byte(&RegByte::A), 31);
        assert_eq!(cpu.registers.read_byte(&RegByte::D), 43);
        assert_eq!(cpu.registers.read_flag(RegFlag::Zero), false);
        assert_eq!(cpu.registers.read_flag(RegFlag::Subtraction), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::HalfCarry), true);
        assert_eq!(cpu.registers.read_flag(RegFlag::Carry), false);
    }
}
