pub struct CPU {
    pub acc: u8,
    pub status: u8,
    pub index: u8,
    pub pc: u16,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            acc: 0,
            status: 0,
            index: 0,
            pc: 0,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opscode = program[self.pc as usize];
            self.pc += 1;

            match opscode {
                0xA9 => { // LDA opcode
                    let param = program[self.pc as usize];
                    self.pc += 1;

                    self.lda(param);

                }

                0xAA => self.tax(),

                0xE8 => self.inx(),

                0x00 => return, // BRK opcode

                _ => todo!(),
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.acc = value;
        self.update_zero_and_negative_flags(self.acc);
    }

    fn tax(&mut self) {
        self.index = self.acc;
        self.update_zero_and_negative_flags(self.index);
    }

    fn inx(&mut self) {
        // use wrapping add to handle overflow from 255 to 0
        self.index = self.index.wrapping_add(1);
        self.update_zero_and_negative_flags(self.index);
    }

    fn update_zero_and_negative_flags(&mut self, value: u8) {
        if value == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if (value & 0b1000_0000) != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.acc, 0x05);
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }
    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.acc = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.index, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.index, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.index = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.index, 1)
    }
}