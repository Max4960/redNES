use crate::cpu::AddressingMode;
use std::collections::HashMap;
use lazy_static::lazy_static;

pub struct Instruction {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub mode: AddressingMode,
    pub cycles: u8,
}

impl Instruction {
    fn new(code: u8, mnemonic: &'static str, len: u8, mode: AddressingMode, cycles: u8) -> Self {
        Instruction {
            code,
            mnemonic,
            len,
            mode,
            cycles,
        }
    }
}

lazy_static! {
    pub static ref CPU_INSTRUCTIONS: Vec<Instruction> = vec![
        // --- System Functions ---
        Instruction::new(0x00, "BRK", 1, AddressingMode::NonAddressing, 7),
        Instruction::new(0xEA, "NOP", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x40, "RTI", 1, AddressingMode::NonAddressing, 6),

        // --- Jumps & Subroutines ---
        Instruction::new(0x4C, "JMP", 3, AddressingMode::Absolute, 3),
        Instruction::new(0x6C, "JMP", 3, AddressingMode::NonAddressing, 5), // Indirect
        Instruction::new(0x20, "JSR", 3, AddressingMode::Absolute, 6),
        Instruction::new(0x60, "RTS", 1, AddressingMode::NonAddressing, 6),

        // --- Load/Store Operations ---
        Instruction::new(0xA9, "LDA", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xA5, "LDA", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xB5, "LDA", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0xAD, "LDA", 3, AddressingMode::Absolute, 4),
        Instruction::new(0xBD, "LDA", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0xB9, "LDA", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0xA1, "LDA", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0xB1, "LDA", 2, AddressingMode::IndirectY, 5),
        Instruction::new(0xA2, "LDX", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xA6, "LDX", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xB6, "LDX", 2, AddressingMode::ZeroPageY, 4),
        Instruction::new(0xAE, "LDX", 3, AddressingMode::Absolute, 4),
        Instruction::new(0xBE, "LDX", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0xA0, "LDY", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xA4, "LDY", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xB4, "LDY", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0xAC, "LDY", 3, AddressingMode::Absolute, 4),
        Instruction::new(0xBC, "LDY", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0x85, "STA", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x95, "STA", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0x8D, "STA", 3, AddressingMode::Absolute, 4),
        Instruction::new(0x9D, "STA", 3, AddressingMode::AbsoluteX, 5),
        Instruction::new(0x99, "STA", 3, AddressingMode::AbsoluteY, 5),
        Instruction::new(0x81, "STA", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0x91, "STA", 2, AddressingMode::IndirectY, 6),
        Instruction::new(0x86, "STX", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x96, "STX", 2, AddressingMode::ZeroPageY, 4),
        Instruction::new(0x8E, "STX", 3, AddressingMode::Absolute, 4),
        Instruction::new(0x84, "STY", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x94, "STY", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0x8C, "STY", 3, AddressingMode::Absolute, 4),

        // --- Register Transfers ---
        Instruction::new(0xAA, "TAX", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xA8, "TAY", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xBA, "TSX", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x8A, "TXA", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x9A, "TXS", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x98, "TYA", 1, AddressingMode::NonAddressing, 2),

        // --- Stack Operations ---
        Instruction::new(0x48, "PHA", 1, AddressingMode::NonAddressing, 3),
        Instruction::new(0x08, "PHP", 1, AddressingMode::NonAddressing, 3),
        Instruction::new(0x68, "PLA", 1, AddressingMode::NonAddressing, 4),
        Instruction::new(0x28, "PLP", 1, AddressingMode::NonAddressing, 4),

        // --- Logical ---
        Instruction::new(0x29, "AND", 2, AddressingMode::Immediate, 2),
        Instruction::new(0x25, "AND", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x35, "AND", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0x2D, "AND", 3, AddressingMode::Absolute, 4),
        Instruction::new(0x3D, "AND", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0x39, "AND", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0x21, "AND", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0x31, "AND", 2, AddressingMode::IndirectY, 5),
        Instruction::new(0x49, "EOR", 2, AddressingMode::Immediate, 2),
        Instruction::new(0x45, "EOR", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x55, "EOR", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0x4D, "EOR", 3, AddressingMode::Absolute, 4),
        Instruction::new(0x5D, "EOR", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0x59, "EOR", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0x41, "EOR", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0x51, "EOR", 2, AddressingMode::IndirectY, 5),
        Instruction::new(0x09, "ORA", 2, AddressingMode::Immediate, 2),
        Instruction::new(0x05, "ORA", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x15, "ORA", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0x0D, "ORA", 3, AddressingMode::Absolute, 4),
        Instruction::new(0x1D, "ORA", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0x19, "ORA", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0x01, "ORA", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0x11, "ORA", 2, AddressingMode::IndirectY, 5),
        Instruction::new(0x24, "BIT", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x2C, "BIT", 3, AddressingMode::Absolute, 4),

        // --- Arithmetic ---
        Instruction::new(0x69, "ADC", 2, AddressingMode::Immediate, 2),
        Instruction::new(0x65, "ADC", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0x75, "ADC", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0x6D, "ADC", 3, AddressingMode::Absolute, 4),
        Instruction::new(0x7D, "ADC", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0x79, "ADC", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0x61, "ADC", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0x71, "ADC", 2, AddressingMode::IndirectY, 5),
        Instruction::new(0xE9, "SBC", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xE5, "SBC", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xF5, "SBC", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0xED, "SBC", 3, AddressingMode::Absolute, 4),
        Instruction::new(0xFD, "SBC", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0xF9, "SBC", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0xE1, "SBC", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0xF1, "SBC", 2, AddressingMode::IndirectY, 5),

        // --- Comparisons ---
        Instruction::new(0xC9, "CMP", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xC5, "CMP", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xD5, "CMP", 2, AddressingMode::ZeroPageX, 4),
        Instruction::new(0xCD, "CMP", 3, AddressingMode::Absolute, 4),
        Instruction::new(0xDD, "CMP", 3, AddressingMode::AbsoluteX, 4),
        Instruction::new(0xD9, "CMP", 3, AddressingMode::AbsoluteY, 4),
        Instruction::new(0xC1, "CMP", 2, AddressingMode::IndirectX, 6),
        Instruction::new(0xD1, "CMP", 2, AddressingMode::IndirectY, 5),
        Instruction::new(0xE0, "CPX", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xE4, "CPX", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xEC, "CPX", 3, AddressingMode::Absolute, 4),
        Instruction::new(0xC0, "CPY", 2, AddressingMode::Immediate, 2),
        Instruction::new(0xC4, "CPY", 2, AddressingMode::ZeroPage, 3),
        Instruction::new(0xCC, "CPY", 3, AddressingMode::Absolute, 4),

        // --- Increments & Decrements ---
        Instruction::new(0xE6, "INC", 2, AddressingMode::ZeroPage, 5),
        Instruction::new(0xF6, "INC", 2, AddressingMode::ZeroPageX, 6),
        Instruction::new(0xEE, "INC", 3, AddressingMode::Absolute, 6),
        Instruction::new(0xFE, "INC", 3, AddressingMode::AbsoluteX, 7),
        Instruction::new(0xE8, "INX", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xC8, "INY", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xC6, "DEC", 2, AddressingMode::ZeroPage, 5),
        Instruction::new(0xD6, "DEC", 2, AddressingMode::ZeroPageX, 6),
        Instruction::new(0xCE, "DEC", 3, AddressingMode::Absolute, 6),
        Instruction::new(0xDE, "DEC", 3, AddressingMode::AbsoluteX, 7),
        Instruction::new(0xCA, "DEX", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x88, "DEY", 1, AddressingMode::NonAddressing, 2),

        // --- Shifts ---
        Instruction::new(0x0A, "ASL", 1, AddressingMode::NonAddressing, 2), // Accumulator
        Instruction::new(0x06, "ASL", 2, AddressingMode::ZeroPage, 5),
        Instruction::new(0x16, "ASL", 2, AddressingMode::ZeroPageX, 6),
        Instruction::new(0x0E, "ASL", 3, AddressingMode::Absolute, 6),
        Instruction::new(0x1E, "ASL", 3, AddressingMode::AbsoluteX, 7),
        Instruction::new(0x4A, "LSR", 1, AddressingMode::NonAddressing, 2), // Accumulator
        Instruction::new(0x46, "LSR", 2, AddressingMode::ZeroPage, 5),
        Instruction::new(0x56, "LSR", 2, AddressingMode::ZeroPageX, 6),
        Instruction::new(0x4E, "LSR", 3, AddressingMode::Absolute, 6),
        Instruction::new(0x5E, "LSR", 3, AddressingMode::AbsoluteX, 7),
        Instruction::new(0x2A, "ROL", 1, AddressingMode::NonAddressing, 2), // Accumulator
        Instruction::new(0x26, "ROL", 2, AddressingMode::ZeroPage, 5),
        Instruction::new(0x36, "ROL", 2, AddressingMode::ZeroPageX, 6),
        Instruction::new(0x2E, "ROL", 3, AddressingMode::Absolute, 6),
        Instruction::new(0x3E, "ROL", 3, AddressingMode::AbsoluteX, 7),
        Instruction::new(0x6A, "ROR", 1, AddressingMode::NonAddressing, 2), // Accumulator
        Instruction::new(0x66, "ROR", 2, AddressingMode::ZeroPage, 5),
        Instruction::new(0x76, "ROR", 2, AddressingMode::ZeroPageX, 6),
        Instruction::new(0x6E, "ROR", 3, AddressingMode::Absolute, 6),
        Instruction::new(0x7E, "ROR", 3, AddressingMode::AbsoluteX, 7),

        // --- Branches ---
        Instruction::new(0x90, "BCC", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0xB0, "BCS", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0xF0, "BEQ", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0xD0, "BNE", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0x30, "BMI", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0x10, "BPL", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0x50, "BVC", 2, AddressingMode::NonAddressing, 2),
        Instruction::new(0x70, "BVS", 2, AddressingMode::NonAddressing, 2),

        // --- Status Flag Changes ---
        Instruction::new(0x18, "CLC", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x38, "SEC", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x58, "CLI", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0x78, "SEI", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xD8, "CLD", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xF8, "SED", 1, AddressingMode::NonAddressing, 2),
        Instruction::new(0xB8, "CLV", 1, AddressingMode::NonAddressing, 2),
    ];
    
    pub static ref CPU_INSTRUCTIONS_MAP: HashMap<u8, &'static Instruction> = {
        let mut map = HashMap::new();
        for instruction in &*CPU_INSTRUCTIONS {
            map.insert(instruction.code, instruction);
        }
        map
    };
}