    use std::collections::HashMap;
    use crate::opcodes;
    
    #[allow(non_snake_case)]
    pub mod StatusFlags {
        pub const CARRY: u8 = 0b0000_0001;
        pub const ZERO: u8 = 0b0000_0010;
        pub const INTERRUPT_DISABLE: u8 = 0b0000_0100;
        pub const DECIMAL_MODE: u8 = 0b0000_1000; // not used in NES
        pub const BREAK: u8 = 0b0001_0000;
        pub const BREAK2: u8 = 0b0010_0000;
        pub const OVERFLOW: u8 = 0b0100_0000;
        pub const NEGATIVE: u8 = 0b1000_0000;
    }
    
    const STACK: u16 = 0x0100;
    const STACK_RESET: u8 = 0xfd;
    
    pub struct CPU {
        pub acc: u8,
        pub status: u8,
        pub index_x: u8,
        pub index_y: u8,
        pub sp: u8, // stack pointer
        pub pc: u16, // program counter
        memory: [u8; 0xFFFF]
    }
    
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    pub enum AddressingMode {
        Immediate,
        ZeroPage,
        ZeroPageX,
        ZeroPageY,
        Absolute,
        AbsoluteX,
        AbsoluteY,
        IndirectX,
        IndirectY,
        NonAddressing,
    }
    
    pub trait Memory {
        fn mem_read(&self, address: u16) -> u8;
        fn mem_write(&mut self, address: u16, value: u8);
    
        fn mem_read_u16(&self, pos: u16) -> u16 {
            let lo = self.mem_read(pos) as u16;
            let hi = self.mem_read(pos + 1) as u16;
            (hi << 8) | (lo as u16)
        }
        fn mem_write_u16(&mut self, pos: u16, data: u16) {
            let hi = (data >> 8) as u8;
            let lo = (data & 0xFF) as u8;
            self.mem_write(pos, lo);
            self.mem_write(pos + 1, hi);
        }
    }
    
    impl Memory for CPU {
        fn mem_read(&self, address: u16) -> u8 {
            self.memory[address as usize]
        }
    
        fn mem_write(&mut self, address: u16, value: u8) {
            self.memory[address as usize] = value;
        }
    }
    
    impl CPU {
        pub fn new() -> CPU {
            CPU {
                acc: 0,
                status: StatusFlags::INTERRUPT_DISABLE | StatusFlags::BREAK2,
                index_x: 0,
                index_y: 0,
                sp: STACK_RESET,
                pc: 0,
                memory: [0; 0xFFFF]
            }
        }
    
        fn set_flag(&mut self, flag: u8, value: bool) {
            if value {
                self.status = self.status | flag;
            } else {
                self.status = self.status & !flag;
            }
        }
    
        fn get_flag(&self, flag: u8) -> bool {
            (self.status & flag) > 0
        }
    
        fn lda(&mut self, mode:&AddressingMode) {
            let addr = self.get_operand_address(&mode);
            let value = self.mem_read(addr);
    
            self.acc = value;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn ldx(&mut self, mode:&AddressingMode) {
            let addr = self.get_operand_address(&mode);
            self.index_x = self.mem_read(addr);
            self.update_zero_and_negative_flags(self.index_x);
        }
    
        fn ldy(&mut self, mode:&AddressingMode) {
            let addr = self.get_operand_address(&mode);
            self.index_y = self.mem_read(addr);
            self.update_zero_and_negative_flags(self.index_y);
        }
    
        fn sta(&mut self, mode:&AddressingMode) {
            let addr = self.get_operand_address(&mode);
            self.mem_write(addr, self.acc);
        }
    
        fn stx(&mut self, mode:&AddressingMode) {
            let addr = self.get_operand_address(&mode);
            self.mem_write(addr, self.index_x);
        }
    
        fn sty(&mut self, mode:&AddressingMode) {
            let addr = self.get_operand_address(&mode);
            self.mem_write(addr, self.index_y);
        }
    
        fn tax(&mut self) {
            self.index_x = self.acc;
            self.update_zero_and_negative_flags(self.index_x);
        }
    
        fn tay(&mut self) {
            self.index_y = self.acc;
            self.update_zero_and_negative_flags(self.index_y);
        }
    
        fn txa(&mut self) {
            self.acc = self.index_x;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn tya(&mut self) {
            self.acc = self.index_y;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn tsx(&mut self) {
            self.index_x = self.sp;
            self.update_zero_and_negative_flags(self.index_x);
        }
    
        fn txs(&mut self) {
            self.sp = self.index_x;
        }
    
        fn inc(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let result = value.wrapping_add(1);
            self.mem_write(addr, result);
            self.update_zero_and_negative_flags(result);
        }
    
        fn inx(&mut self) {
            // use wrapping add to handle overflow from 255 to 0
            self.index_x = self.index_x.wrapping_add(1);
            self.update_zero_and_negative_flags(self.index_x);
        }
    
        fn iny(&mut self) {
            self.index_y = self.index_y.wrapping_add(1);
            self.update_zero_and_negative_flags(self.index_y);
        }
    
        fn dec(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let mut value = self.mem_read(addr);
            value = value.wrapping_sub(1);
            self.mem_write(addr, value);
            self.update_zero_and_negative_flags(value);
        }
    
        fn dex(&mut self) {
            self.index_x = self.index_x.wrapping_sub(1);
            self.update_zero_and_negative_flags(self.index_x);
        }
    
        fn dey(&mut self) {
            self.index_y = self.index_y.wrapping_sub(1);
            self.update_zero_and_negative_flags(self.index_y);
        }
    
        // addition with carry
        fn adc(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let carry_in = self.get_flag(StatusFlags::CARRY) as u8;
    
            let sum = self.acc as u16 + value as u16 + carry_in as u16;
    
            // set carry flag if sum > 255
            self.set_flag(StatusFlags::CARRY, sum > 0xFF);
            let result = sum as u8;
    
            // set overflow flag
            // overflow occurs when sign of inputs are the same, and result is different
            let overflow = (self.acc ^ result) & (value ^ result) & 0x80 != 0;
            self.set_flag(StatusFlags::OVERFLOW, overflow);
    
            self.acc = result;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        // subtract and carry
        fn sbc(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let inverted_value = (value as i8).wrapping_neg().wrapping_sub(1) as u8;
    
            // using ADC logic with inverted value
            // temp clone to call ADC on
            let mut temp_cpu = CPU {
                acc: self.acc,
                status: self.status,
                ..*self
            };
    
            temp_cpu.mem_write(0, inverted_value); // write value to dummy location
            temp_cpu.pc = 0; // point pc to dummy locaiton
            temp_cpu.adc(&AddressingMode::Immediate);
    
            // copy results back
            self.acc = temp_cpu.acc;
            self.status = temp_cpu.status;
        }
    
        fn and(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            self.acc = self.acc & value;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn eor(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            self.acc = self.acc ^ value;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn ora(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            self.acc = self.acc | value;
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn compare(&mut self, mode: &AddressingMode, reg_value: u8) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
            let result = reg_value.wrapping_sub(value);
    
            self.set_flag(StatusFlags::CARRY, reg_value > value);
            self.update_zero_and_negative_flags(result);
        }
    
        fn bit(&mut self, mode: &AddressingMode) {
            let addr = self.get_operand_address(mode);
            let value = self.mem_read(addr);
    
            self.set_flag(StatusFlags::ZERO, (self.acc & value) == 0);
            self.set_flag(StatusFlags::NEGATIVE, (value & StatusFlags::NEGATIVE) != 0);
            self.set_flag(StatusFlags::OVERFLOW, (value & StatusFlags::OVERFLOW) != 0);
        }
    
        fn update_zero_and_negative_flags(&mut self, value: u8) {
            self.set_flag(StatusFlags::ZERO, value == 0);
            self.set_flag(StatusFlags::NEGATIVE, (value & 0b1000_0000) != 0);
        }
    
        // --- Stack Functionality ---
        fn stack_push(&mut self, value: u8) {
            self.mem_write(STACK + self.sp as u16, value);
            self.sp = self.sp.wrapping_sub(1);
        }
    
        fn stack_pop(&mut self) -> u8 {
            self.sp = self.sp.wrapping_add(1);
            self.mem_read(STACK + self.sp as u16)
        }
    
        fn stack_push_u16(&mut self, value: u16) {
            let hi = (value >> 8) as u8;
            let lo = (value & 0xFF) as u8;
            self.stack_push(hi);
            self.stack_push(lo);
        }
    
        fn stack_pop_u16(&mut self) -> u16 {
            let lo = self.stack_pop() as u16;
            let hi = self.stack_pop() as u16;
            (hi << 8) | lo
        }
    
        fn pha(&mut self) {
            self.stack_push(self.acc);
        }
    
        fn pla(&mut self) {
            self.acc = self.stack_pop();
            self.update_zero_and_negative_flags(self.acc);
        }
    
        fn php(&mut self) {
            let mut flags = self.status;
            flags |= StatusFlags::BREAK;
            flags |= StatusFlags::BREAK2;
            self.stack_push(flags);
        }
    
        fn plp(&mut self) {
            self.status = self.stack_pop();
            self.set_flag(StatusFlags::BREAK, false);
            self.set_flag(StatusFlags::BREAK2, true);
        }
    
        // all branch instructions have same logic
        fn branch(&mut self, condition: bool) {
            if condition {
                let jump: i8 = self.mem_read(self.pc) as i8;
                let jump_addr = self.pc.wrapping_add(1).wrapping_add(jump as u16);
                self.pc = jump_addr;
            }
        }
    
        pub fn reset(&mut self) {
            self.acc = 0;
            self.status = StatusFlags::INTERRUPT_DISABLE | StatusFlags::BREAK2;
            self.index_x = 0;
            self.index_y = 0;
            self.sp = STACK_RESET;
    
            self.pc = self.mem_read_u16(0xFFFC);
        }
    
        pub fn load_and_run(&mut self, program: Vec<u8>) {
            self.load(program);
            self.reset();
            self.run();
        }
    
        pub fn load(&mut self, program: Vec<u8>) {
            self.memory[0x0600 .. (0x0600 + program.len())].copy_from_slice(&program[..]);
            self.mem_write_u16(0xFFFC, 0x0600);
        }
    
    
        fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
            match mode {
                AddressingMode::Immediate => self.pc,
    
                AddressingMode::ZeroPage => self.mem_read(self.pc) as u16,
    
                AddressingMode::Absolute => self.mem_read_u16(self.pc),
    
                AddressingMode::ZeroPageX => {
                    let pos = self.mem_read(self.pc);
                    let addr = pos.wrapping_add(self.index_x) as u16;
                    addr
                }
    
                AddressingMode::ZeroPageY => {
                    let pos = self.mem_read(self.pc);
                    let addr = pos.wrapping_add(self.index_y) as u16;
                    addr
                }
    
                AddressingMode::AbsoluteX => {
                    let base = self.mem_read_u16(self.pc);
                    let addr = base.wrapping_add(self.index_x as u16);
                    addr
                }
    
                AddressingMode::AbsoluteY => {
                    let base = self.mem_read_u16(self.pc);
                    let addr = base.wrapping_add(self.index_y as u16);
                    addr
                }
    
                AddressingMode::IndirectX => {
                    let base = self.mem_read(self.pc);
    
                    let ptr: u8 = (base as u8).wrapping_add(self.index_x);
                    let lo = self.mem_read(ptr as u16);
                    let hi = self.mem_read(ptr.wrapping_add(1) as u16);
                    (hi as u16) << 8 | (lo as u16)
                }
    
                AddressingMode::IndirectY => {
                    let base = self.mem_read(self.pc);
    
                    let lo = self.mem_read(base as u16);
                    let hi = self.mem_read((base as u8).wrapping_add(1) as u16);
                    let deref_base = (hi as u16) << 8 | (lo as u16);
                    let deref_addr = deref_base.wrapping_add(self.index_y as u16);
                    deref_addr
                }
    
                AddressingMode::NonAddressing => {
                    panic!("mode {:?} is not supported", mode);
                }
            }
        }
    
        pub fn run(&mut self) {
            self.run_with_callback(|_| {});
        }
    
        pub fn run_with_callback<F>(&mut self, mut callback: F)
        where
            F: FnMut(&mut CPU),
        {
                let ref opcodes: HashMap<u8, &'static opcodes::Instruction> = *opcodes::CPU_INSTRUCTIONS_MAP;
    
                loop {
                    let opcode = self.mem_read(self.pc);
                    self.pc += 1;
                    let pc_state = self.pc;
    
                    let instruction = opcodes.get(&opcode).expect("unknown opcode");
    
                    match opcode {
                        // --- LDA ---
                        0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => self.lda(&instruction.mode),
                        // --- LDX ---
                        0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => self.ldx(&instruction.mode),
                        // --- LDY ---
                        0xA0 | 0xA4 | 0xB4 | 0xAC | 0xBC => self.ldy(&instruction.mode),
                        // --- STA ---
                        0x85 | 0x8D | 0x95 | 0x9D | 0x99 | 0x81 | 0x91 => self.sta(&instruction.mode),
                        // --- STX ---
                        0x86 | 0x96 | 0x8E => self.stx(&instruction.mode),
                        // --- STY ---
                        0x84 | 0x94 | 0x8C => self.sty(&instruction.mode),
                        // --- ADC ---
                        0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => self.adc(&instruction.mode),
                        // --- SBC ---
                        0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => self.sbc(&instruction.mode),
                        // --- Compare Instructions ---
                        0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => self.compare(&instruction.mode, self.acc),
                        0xE0 | 0xE4 | 0xEC => self.compare(&instruction.mode, self.index_x),
                        0xC0 | 0xC4 | 0xCC => self.compare(&instruction.mode, self.index_y),
                        // --- AND ---
                        0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => self.and(&instruction.mode),
                        // --- EOR ---
                        0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51 => self.eor(&instruction.mode),
                        // --- ORA ---
                        0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => self.ora(&instruction.mode),
                        // --- BIT ---
                        0x24 | 0x2C => self.bit(&instruction.mode),
                        // --- INC ---
                        0xE6 | 0xEE | 0xF6 | 0xFE => self.inc(&instruction.mode),
                        // --- DEC ---
                        0xC6 | 0xCE | 0xD6 | 0xDE => self.dec(&instruction.mode),
                        // --- INX ---
                        0xE8 => self.inx(),
                        // --- INY ---
                        0xC8 => self.iny(),
                        // --- DEX ---
                        0xCA => self.dex(),
                        // --- DEY ---
                        0x88 => self.dey(),
    
                        // --- Stack Operations ---
                        0x48 => self.pha(),
                        0x68 => self.pla(),
                        0x08 => self.php(),
                        0x28 => self.plp(),
    
                        // --- Transfers ---
                        0xAA => self.tax(),
                        0xA8 => self.tay(),
                        0x8A => self.txa(),
                        0x98 => self.tya(),
                        0xBA => self.tsx(),
                        0x9A => self.txs(),
    
                        // --- Branch Instructions ---
                        0x90 => self.branch(!self.get_flag(StatusFlags::CARRY)),   // BCC
                        0xB0 => self.branch(self.get_flag(StatusFlags::CARRY)),    // BCS
                        0xF0 => self.branch(self.get_flag(StatusFlags::ZERO)),     // BEQ
                        0xD0 => self.branch(!self.get_flag(StatusFlags::ZERO)),    // BNE
                        0x30 => self.branch(self.get_flag(StatusFlags::NEGATIVE)), // BMI
                        0x10 => self.branch(!self.get_flag(StatusFlags::NEGATIVE)), // BPL
                        0x50 => self.branch(!self.get_flag(StatusFlags::OVERFLOW)), // BVC
                        0x70 => self.branch(self.get_flag(StatusFlags::OVERFLOW)), // BVS
    
                        // --- Status Flag Changes ---
                        0x18 => self.set_flag(StatusFlags::CARRY, false), // CLC
                        0x38 => self.set_flag(StatusFlags::CARRY, true),  // SEC
                        0x58 => self.set_flag(StatusFlags::INTERRUPT_DISABLE, false), // CLI
                        0x78 => self.set_flag(StatusFlags::INTERRUPT_DISABLE, true),  // SEI
                        0xD8 => self.set_flag(StatusFlags::DECIMAL_MODE, false), // CLD
                        0xF8 => self.set_flag(StatusFlags::DECIMAL_MODE, true),  // SED
                        0xB8 => self.set_flag(StatusFlags::OVERFLOW, false), //CLV
    
                        // --- JMP Absolute ---
                        0x4C => {
                            self.pc = self.mem_read_u16(self.pc);
                        }
    
                        // --- JMP Indirect ---
                        0x6C => {
                            let operand_addr = self.mem_read_u16(self.pc);
                            let target_addr = if operand_addr & 0x00FF == 0x00FF {
                                // 6502 bug case: page boundary crossing
                                let lo = self.mem_read(operand_addr);
                                let hi = self.mem_read(operand_addr & 0xFF00); // read from start of page
                                (hi as u16) << 8 | (lo as u16)
                            } else {
                                // normal case
                                self.mem_read_u16(operand_addr)
                            };
                            self.pc = target_addr;
                        }
    
                        // --- JSR ---
                        0x20 => {
                            self.stack_push_u16(self.pc + 1);
                            self.pc = self.mem_read_u16(self.pc);
                        }
    
                        // --- RTS ---
                        0x60 => {
                            self.pc = self.stack_pop_u16() + 1;
                        }
    
                        // --- ASL ACC ---
                        0x0A => {
                            let mut value = self.acc;
                            self.set_flag(StatusFlags::CARRY, (value & 0x80) > 0);
                            value <<= 1;
                            self.update_zero_and_negative_flags(value);
                            self.acc = value;
                        }
                        // --- ASL Mem ---
                        0x06 | 0x16 | 0x0E | 0x1E => {
                            let addr = self.get_operand_address(&instruction.mode);
                            let mut value = self.mem_read(addr);
                            self.set_flag(StatusFlags::CARRY, (value & 0x80) > 0);
                            value <<= 1;
                            self.update_zero_and_negative_flags(value);
                            self.mem_write(addr, value);
                        }
    
                        // --- LSR ACC ---
                        0x4A => {
                            let mut value = self.acc;
                            self.set_flag(StatusFlags::CARRY, (value & 0x01) > 0);
                            value >>= 1;
                            self.update_zero_and_negative_flags(value);
                            self.acc = value;
                        }
                        // --- LSR Mem ---
                        0x46 | 0x56 | 0x4E | 0x5E => {
                            let addr = self.get_operand_address(&instruction.mode);
                            let mut value = self.mem_read(addr);
                            self.set_flag(StatusFlags::CARRY, (value & 0x01) > 0);
                            value >>= 1;
                            self.update_zero_and_negative_flags(value);
                            self.mem_write(addr, value);
                        }
    
                        // --- RTI ---
                        0x40 => {
                            self.status = self.stack_pop();
                            self.set_flag(StatusFlags::BREAK, false);
                            self.set_flag(StatusFlags::BREAK2, true);
                            self.pc = self.stack_pop_u16();
                        }
    
                        // --- ROL ACC ---
                        0x2A => {
                            let mut value = self.acc;
                            let old_carry = self.get_flag(StatusFlags::CARRY);
                            self.set_flag(StatusFlags::CARRY, (value & 0x80) > 0);
                            value <<= 1;
                            if old_carry {
                                value |= 0x01;
                            }
                            self.update_zero_and_negative_flags(value);
                            self.acc = value;
                        }
                        // --- ROL Mem ---
                        0x26 | 0x36 | 0x2E | 0x3E => {
                            let addr = self.get_operand_address(&instruction.mode);
                            let mut value = self.mem_read(addr);
                            let old_carry = self.get_flag(StatusFlags::CARRY);
                            self.set_flag(StatusFlags::CARRY, (value & 0x80) > 0);
                            value <<= 1;
                            if old_carry {
                                value |= 0x01;
                            }
                            self.update_zero_and_negative_flags(value);
                            self.mem_write(addr, value);
                        }
    
                        // --- ROR ACC ---
                        0x6A => {
                            let mut value = self.acc;
                            let old_carry = self.get_flag(StatusFlags::CARRY);
                            self.set_flag(StatusFlags::CARRY, (value & 0x01) > 0);
                            value >>= 1;
                            if old_carry {
                                value |= 0x80;
                            }
                            self.update_zero_and_negative_flags(value);
                            self.acc = value;
                        }
                        // --- ROR Mem ---
                        0x66 | 0x76 | 0x6E | 0x7E => {
                            let addr = self.get_operand_address(&instruction.mode);
                            let mut value = self.mem_read(addr);
                            let old_carry = self.get_flag(StatusFlags::CARRY);
                            self.set_flag(StatusFlags::CARRY, (value & 0x01) > 0);
                            value >>= 1;
                            if old_carry {
                                value |= 0x80;
                            }
                            self.update_zero_and_negative_flags(value);
                            self.mem_write(addr, value);
                        }
    
                        // --- NOP ---
                        0xEA => {/* do nothing */},
    
                        // --- BRK ---
                        0x00 => return,
    
                        _ => todo!(),
                    }
    
                    // handle setting pc for everything that isnt jumps and branches
                    if pc_state == self.pc {
                        self.pc += (instruction.len -1) as u16;
                    }
    
                    callback(self);
                }
        }
    }
    
    
    #[cfg(test)]
    mod test {
        use super::*;
    
        #[test]
        fn test_0xa9_lda_immediate_load_data() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
            assert_eq!(cpu.acc, 5);
            assert!(cpu.status & 0b0000_0010 == 0);
            assert!(cpu.status & 0b1000_0000 == 0);
        }
    
        #[test]
        fn test_0xa9_lda_zero_flag() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
            assert!(cpu.status & 0b0000_0010 == 0b10);
        }
    
        #[test]
        fn test_0xaa_tax_move_a_to_x() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0x0A,0xaa, 0x00]);
    
            assert_eq!(cpu.index_x, 10)
        }
    
        #[test]
        fn test_5_ops_working_together() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    
            assert_eq!(cpu.index_x, 0xc1)
        }
    
        #[test]
        fn test_inx_overflow() {
            let mut cpu = CPU::new();
            cpu.load_and_run(vec![0xa9, 0xff, 0xaa,0xe8, 0xe8, 0x00]);
    
            assert_eq!(cpu.index_x, 1)
        }
    
        #[test]
        fn test_lda_from_memory() {
            let mut cpu = CPU::new();
            cpu.mem_write(0x10, 0x55);
    
            cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
    
            assert_eq!(cpu.acc, 0x55);
        }
    }