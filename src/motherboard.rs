use crate::{
    cartridge::Cartridge,
    clock::Clock,
    cpu::Cpu,
    memory::Memory,
    registers::{RegWord, Registers},
};

// TODO: General note/concern, making everything private, will fix/make public
// > After further along, wanted to try habit of everything private until needing to
// > turn it into public/learning experience.

pub struct Motherboard {
    registers: Registers,
    memory: Memory,
    cartridge: Cartridge,
    clock: Clock,
    cpu: Cpu,
}

impl Motherboard {
    fn new() -> Self {
        Self {
            registers: Registers::new(),
            memory: Memory::new(),
            // TODO: Should the motherboard only be built with an existing cartridge?
            // > Or maybe having an empty aka nonexistent cartridge is fine on startup?
            cartridge: Cartridge::new(),
            clock: Clock::new(),
            cpu: Cpu::new(),
        }
    }

    // Get next instruction from memory by reading program counter
    // > and increment program counter
    fn fetch_next_byte(&mut self) -> u8 {
        let byte = self
            .memory
            .read_byte(self.registers.read_word(&RegWord::PC));
        self.registers.increment_pc();
        byte
    }

    // Get length of instruction (How many bytes of data needed, always between 1 (the initial bit) and 3 (two additional immediate bytes))
    fn get_instruction_length(instruction: u8) -> u8 {
        // TEMP: Working glossary for comments following instructions
        // Add/sub/load X Z = perform operation with X as destination/data performed UPON by z, Z is the data performing UNTO x
        // e.g. load x z = load z into x
        // e.g. sub x z = subtract z from x
        // (word) = 8bit contents of 16 bit address *(so far this pattern accurate, might need TODO: Fix)
        // TODO: Double check this: [d16 = immediate data value 16 bit, a16 = 16 bit address in memory, r8 = 8 bit signed data (added to PC?)]
        // TODO: What the heck is JR on here: https://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html (Side Note: this is table I'm currently using >
        // > in conjunction with: https://rgbds.gbdev.io/docs/v0.9.4/gbz80.7#ADD_HL,r16
        match instruction {
            // 0x
            0x00 => 1, // NOP
            0x01 => 3, // LD BC d16
            0x02 => 1, // LD (BC) A
            0x03 => 1, // INC BC
            0x04 => 1, // INC B
            0x05 => 1, // DEC B
            0x06 => 2, // LD B d8
            0x07 => 1, // RLCA
            0x08 => 3, // LD (a16) SP
            0x09 => 1, // ADD HL BC
            0x0A => 1, // LD A (BC)
            0x0B => 1, // DEC BC
            0x0C => 1, // INC C
            0x0D => 1, // DEC C
            0x0E => 2, // LD C d8
            0x0F => 1, // RRCA

            // 1x
            0x10 => 2, // STOP 0
            0x11 => 3, // LD DE d16
            0x12 => 1, // LD (DE) A
            0x13 => 1, // INC DE
            0x14 => 1, // INC D
            0x15 => 1, // DEC D
            0x16 => 2, // LD D d8
            0x17 => 1, // RLA
            0x18 => 2, // JR r8
            0x19 => 1, // ADD HL DE
            0x1A => 1, // LD A (DE)
            0x1B => 1, // DEC DE
            0x1C => 1, // INC E
            0x1D => 1, // DEC E
            0x1E => 2, // LD E d8
            0x1F => 1, // RRA

            // 2x
            0x20 => 2, // JR NZ r8
            0x21 => 3, // LD HL d16
            0x22 => 1, // LD (HL+) A
            0x23 => 1, // INC HL
            0x24 => 1, // INC H
            0x25 => 1, // DEC H
            0x26 => 2, // LD H d8
            0x27 => 1, // DAA
            0x28 => 2, // JR Z r8
            0x29 => 1, // ADD HL HL
            0x2A => 1, // LD A (HL+)
            0x2B => 1, // DEC HL
            0x2C => 1, // INC L
            0x2D => 1, // DEC L
            0x2E => 2, // LD L d8
            0x2F => 1, // CPL

            // 3x
            0x30 => 2, // JR NC r8
            0x31 => 3, // LD SP d16
            0x32 => 1, // LD (HL-) A
            0x33 => 1, // INC SP
            0x34 => 1, // INC (HL)
            0x35 => 1, // DEC (HL)
            0x36 => 2, // LD (HL) d8
            0x37 => 1, // SCF
            0x38 => 2, // JR C r8
            0x39 => 1, // ADD HL SP
            0x3A => 1, // LD A (HL-)
            0x3B => 1, // DEC SP
            0x3C => 1, // INC A
            0x3D => 1, // DEC A
            0x3E => 2, // LD A d8
            0x3F => 1, // CCF

            // 4x
            0x40 => 1, // LD B B
            0x41 => 1, // LD B C
            0x42 => 1, // LD B D
            0x43 => 1, // LD B E
            0x44 => 1, // LD B H
            0x45 => 1, // LD B L
            0x46 => 1, // LD B (HL)
            0x47 => 1, // LD B A
            0x48 => 1, // LD C B
            0x49 => 1, // LD C C
            0x4A => 1, // LD C D
            0x4B => 1, // LD C E
            0x4C => 1, // LD C H
            0x4D => 1, // LD C L
            0x4E => 1, // LD C (HL)
            0x4F => 1, // LD C A

            // 5x
            0x50 => 1, // LD D B
            0x51 => 1, // LD D C
            0x52 => 1, // LD D D
            0x53 => 1, // LD D E
            0x54 => 1, // LD D H
            0x55 => 1, // LD D L
            0x56 => 1, // LD D (HL)
            0x57 => 1, // LD D A
            0x58 => 1, // LD E B
            0x59 => 1, // LD E C
            0x5A => 1, // LD E D
            0x5B => 1, // LD E E
            0x5C => 1, // LD E H
            0x5D => 1, // LD E L
            0x5E => 1, // LD E (HL)
            0x5F => 1, // LD E A

            // 6x
            0x60 => 1, // LD H B
            0x61 => 1, // LD H C
            0x62 => 1, // LD H D
            0x63 => 1, // LD H E
            0x64 => 1, // LD H H
            0x65 => 1, // LD H L
            0x66 => 1, // LD H (HL)
            0x67 => 1, // LD H A
            0x68 => 1, // LD L B
            0x69 => 1, // LD L C
            0x6A => 1, // LD L D
            0x6B => 1, // LD L E
            0x6C => 1, // LD L H
            0x6D => 1, // LD L L
            0x6E => 1, // LD L (HL)
            0x6F => 1, // LD L A

            // 7x
            0x70 => 1, // LD (HL) B
            0x71 => 1, // LD (HL) C
            0x72 => 1, // LD (HL) D
            0x73 => 1, // LD (HL) E
            0x74 => 1, // LD (HL) H
            0x75 => 1, // LD (HL) L
            0x76 => 1, // HALT
            0x77 => 1, // LD (HL) A
            0x78 => 1, // LD A B
            0x79 => 1, // LD A C
            0x7A => 1, // LD A D
            0x7B => 1, // LD A E
            0x7C => 1, // LD A H
            0x7D => 1, // LD A L
            0x7E => 1, // LD A (HL)
            0x7F => 1, // LD A A

            // 8x
            0x80 => 1, // ADD A B
            0x81 => 1, // ADD A C
            0x82 => 1, // ADD A D
            0x83 => 1, // ADD A E
            0x84 => 1, // ADD A H
            0x85 => 1, // ADD A L
            0x86 => 1, // ADD A (HL)
            0x87 => 1, // ADD A A
            0x88 => 1, // ADC A B
            0x89 => 1, // ADC A C
            0x8A => 1, // ADC A D
            0x8B => 1, // ADC A E
            0x8C => 1, // ADC A H
            0x8D => 1, // ADC A L
            0x8E => 1, // ADC A (HL)
            0x8F => 1, // ADC A A

            // 9x
            // NOTE: [SUB A B] can also be written as [SUB B] with the A register as the target
            // > being explicit (Same is true for AND, XOR, OR, COMPARE (CP))
            0x90 => 1, // SUB A B
            0x91 => 1, // SUB A C
            0x92 => 1, // SUB A D
            0x93 => 1, // SUB A E
            0x94 => 1, // SUB A H
            0x95 => 1, // SUB A L
            0x96 => 1, // SUB A (HL)
            0x97 => 1, // SUB A A
            0x98 => 1, // SBC A B
            0x99 => 1, // SBC A C
            0x9A => 1, // SBC A D
            0x9B => 1, // SBC A E
            0x9C => 1, // SBC A H
            0x9D => 1, // SBC A L
            0x9E => 1, // SBC A (HL)
            0x9F => 1, // SBC A A

            // Ax
            0xA0 => 1, // AND A B
            0xA1 => 1, // AND A C
            0xA2 => 1, // AND A D
            0xA3 => 1, // AND A E
            0xA4 => 1, // AND A H
            0xA5 => 1, // AND A L
            0xA6 => 1, // AND A (HL)
            0xA7 => 1, // AND A A
            0xA8 => 1, // XOR A B
            0xA9 => 1, // XOR A C
            0xAA => 1, // XOR A D
            0xAB => 1, // XOR A E
            0xAC => 1, // XOR A H
            0xAD => 1, // XOR A L
            0xAE => 1, // XOR A (HL)
            0xAF => 1, // XOR A A

            // Bx
            0xB0 => 1, // OR A B
            0xB1 => 1, // OR A C
            0xB2 => 1, // OR A D
            0xB3 => 1, // OR A E
            0xB4 => 1, // OR A H
            0xB5 => 1, // OR A L
            0xB6 => 1, // OR A (HL)
            0xB7 => 1, // OR A A
            0xB8 => 1, // CP A B
            0xB9 => 1, // CP A C
            0xBA => 1, // CP A D
            0xBB => 1, // CP A E
            0xBC => 1, // CP A H
            0xBD => 1, // CP A L
            0xBE => 1, // CP A (HL)
            0xBF => 1, // CP A A

            // Cx
            // TODO: Figure out any question mark spots (unsure if implicit or complete command)
            0xC0 => 1, // RET NZ
            0xC1 => 1, // POP BC
            0xC2 => 3, // JP NZ a16
            0xC3 => 3, // JP (TODO: ?) a16
            0xC4 => 3, // CALL NZ a16
            0xC5 => 1, // PUSH BC (TODO: ? contents or value)
            0xC6 => 2, // ADD A d8
            0xC7 => 1, // RST 00H (TODO: ? no clue what this is)
            0xC8 => 1, // RET (TODO: ?) Z
            0xC9 => 1, // RET (TODO: ?)
            0xCA => 3, // JP Z a16
            // SPECIAL BELOW: PREFIX TO OTHER OPCODES
            0xCB => 1, // PREFIX CB (TODO: technically a 2/3? since swaps opcodes and does with it)
            0xCC => 3, // CALL Z a16
            0xCD => 3, // CALL (TODO: ?) a16
            0xCE => 2, // ADC A d8
            0xCF => 1, // RST 08H (TODO: I don't understand this one at all)

            // Dx
            0xD0 => 1, // RET NC
            0xD1 => 1, // POP DE
            0xD2 => 3, // JP NC a16
            0xD3 => 1, // BLANK (TODO: How to handle blanks/nothingburgers? Also what to return?)
            0xD4 => 3, // CALL NC a16
            0xD5 => 1, // PUSH DE
            0xD6 => 2, // SUB d8
            0xD7 => 1, // RST 10H (TODO: ? no clue what this is)
            0xD8 => 1, // RET (TODO: ?) C
            0xD9 => 1, // RETI (TODO: ?)
            0xDA => 3, // JP C a16
            0xDB => 1, // BLANK
            0xDC => 3, // CALL C a16
            0xDD => 1, // BLANK
            0xDE => 2, // SBC a d8
            0xDF => 1, // RST 18H (TODO: I don't understand this one at all)

            // Ex
            0xE0 => 2, // LDH (a8) A
            0xE1 => 1, // POP HL
            0xE2 => 2, // LD (C) A
            0xE3 => 1, // BLANK (TODO: How to handle blanks/nothingburgers? Also what to return?)
            0xE4 => 1, // BLANK
            0xE5 => 1, // PUSH HL
            0xE6 => 2, // AND d8
            0xE7 => 1, // RST 20H (TODO: ? no clue what this is)
            0xE8 => 2, // ADD SP r8
            0xE9 => 1, // JP (HL)
            0xEA => 3, // LD (a16) A
            0xEB => 1, // BLANK
            0xEC => 1, // BLANK
            0xED => 1, // BLANK
            0xEE => 2, // XOR (TODO: ? A) d8
            0xEF => 1, // RST 28H (TODO: I don't understand this one at all)

            // Fx
            0xF0 => 2, // LDH A (a8)
            0xF1 => 1, // POP AF
            0xF2 => 2, // LD A (C)
            0xF3 => 1, // DI (TODO: Direct interrupt maybe?)
            0xF4 => 1, // BLANK
            0xF5 => 1, // PUSH AF
            0xF6 => 2, // OR d8
            0xF7 => 1, // RST 30H (TODO: ? no clue what this is)
            0xF8 => 2, // LD HL SP+r8
            0xF9 => 1, // LD SP HL
            0xFA => 3, // LD A (a16)
            0xFB => 1, // EI (TODO: ? Emergency interrupt?)
            0xFC => 1, // BLANK
            0xFD => 1, // BLANK
            0xFE => 2, // CP (TODO: ? A) d8
            0xFF => 1, // RST 38H (TODO: I don't understand this one at all)

            // Panic!
            _ => panic!("Unsupported instruction byte. Get the lobster claw!"),
        }

        // TODO: Handle prefix opcode table memes.
    }

    fn execute_opcode(instruction: u8, instruction_length: u8) {
        // TODO, add first pass over to check if prefixed instruction before handoff
        // OR can do after handoff INSIDE the cpu/after passing it along

        match instruction_length {
            1 => println!("placeholder!"), // TODO: Pass opcode to CPU directly
            2 => println!("placeholder!"), // TODO: Pass array with opcode + next byte
            3 => println!("placeholder!"), // TODO: Pass array with opcode + next two bytes
            _ => panic!("Unsupported instruction length. Get the lobster hammer!"),
        }
    }
}
