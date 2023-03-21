use crate::context::Context;
use std::io::prelude::*;

impl Context
{ 
    pub fn read_u32_from_mem(self: &mut Context, addr: u32) -> u32
    {
        match (addr)
        {
            0x10000000..=0x10008000 =>
            {
                let index: usize = (addr-0x10000000) as usize;
                let mut val: u32 = self.code[index+3] as u32;
                val = val << 8 | ((self.code[index+2] & 0xff) as u32);
                val = val << 8 | ((self.code[index+1] & 0xff) as u32);
                val = val << 8 | ((self.code[index] & 0xff) as u32);

                return val;
            }
            0x00000000..=0x00080000 =>
            {
                let index: usize = (addr) as usize;
                let mut val: u32 = self.code[index+3] as u32;
                val = val << 8 | ((self.code[index+2] & 0xff) as u32);
                val = val << 8 | ((self.code[index+1] & 0xff) as u32);
                val = val << 8 | ((self.code[index] & 0xff) as u32);
                return val;
            }
            _ =>
            {
                return 0;
            }
        }
    }
    pub fn write_u32_to_mem(self: &mut Context, val: u32, addr: u32)
    {
        match(addr)
        {
            0x10000000..=0x10008000 =>
            {
                let index: usize = (addr-0x10000000) as usize;
                self.data[index] = (val & 0xff) as u8;
                self.data[index+1] = ((val>>8) & 0xff) as u8;
                self.data[index+2] = ((val>>16) & 0xff) as u8;
                self.data[index+3] = ((val>>24) & 0xff) as u8;
            }
            _ =>
            {
                println!("[WARN] No write permissions for address: {:#08x}",addr);
            }
        }
    }
    pub fn write_u8_to_mem(self: &mut Context, val: u8, addr: u32)
    {
        match(addr)
        {
            0x10000000..=0x10008000 =>
            {
                let index: usize = (addr-0x10000000) as usize;
                self.data[index] = val;
            }
            _ =>
            {

            }
        }
    }

    pub fn read_u8_from_mem(self: &mut Context, addr: u32) -> u8
    {
        match (addr)
        {
            0x10000000..=0x10008000 =>
            {
                let index: usize = (addr-0x10000000) as usize;
                return self.data[index];
            }
            0x00000000..=0x00080000 =>
            {
                let index: usize = (addr) as usize;
                return self.code[index];
            }
            _ =>
            {
                return 0;
            }
        }
    }
}