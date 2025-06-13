use crate::cartridge::Rom;
use crate::cpu::Memory;

const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_MIRRORS_END: u16 = 0x3FFF;

impl Memory for Bus {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            RAM ..= RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b00000111_11111111;
                self.cpu_vram[mirror_down_address as usize]
            }
            PPU_REGISTERS ..= PPU_MIRRORS_END => {
                let _mirror_down_address = address & 0b00100000_00000111;
                todo!("PPU not implemented")
            }
            0x8000..=0xFFFF => {
                self.read_rpg_rom(address)
            }
            _ => {
                print!("Ignoring memory access at {}", address);
                0
            }
        }
    }

    fn mem_write(&mut self, address: u16, value: u8) {
        match address {
            RAM ..=RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b11111111111;
                self.cpu_vram[mirror_down_address as usize] = value;
            }
            PPU_REGISTERS ..= PPU_MIRRORS_END => {
                let _mirror_down_address = address & 0b00100000_00000111;
                todo!("PPU not implemented")
            }
            0x8000..=0xFFFF => {
                panic!("Attempting to write to ROM")
            }
            _ => {
                print!("Ignoring memory access at {}", address);
            }
        }
    }
}


pub struct Bus {
    cpu_vram: [u8; 2048],
    rom: Rom
}

impl Bus {
    pub fn new(rom: Rom) -> Self {
        Bus {
            cpu_vram: [0; 2048],
            rom: rom,
        }
    }

    fn read_rpg_rom(&self, mut addr: u16) -> u8 {
        addr -= 0x8000;
        if (self.rom.prg_rom.len() == 0x4000 && addr >= 0x4000) {
            // mirror
            addr = addr % 0x4000;
        }
        self.rom.prg_rom[addr as usize]
    }
}