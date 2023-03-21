use crate::context::Context;
use crate::utils::memory_access;

use std::io::prelude::*;

impl Context
{

    pub fn ldr_immediate_thumb1(self: &mut Context, opcode: u16)
    {
        
        let rt: usize = ((opcode >> 8) & 0b111) as usize;
        let imm8_shifted: usize = ((opcode & 0xff) << 2) as usize;
        let mut pc_aligned: u32 = (self.r[15]+4) & 0xFFFFFFFC;

        println!("LDR R{},[PC, {}]",rt,imm8_shifted);
        let offset_mem: u32 = (pc_aligned).wrapping_add((imm8_shifted) as u32);
        self.r[rt] = self.read_u32_from_mem(offset_mem);
    }

    pub fn loadstore_register_thumb1(self: &mut Context, opcode: u16)
    {
        let opA = opcode >> 12;
        let opB = (opcode >> 9) & 0b111;

        match opB
        {
            // STR Rt, [Rn, Rm]
            0b000 =>
            {

                let rm: usize = ((opcode >> 6) & 0b111) as usize;
                let rn: usize = ((opcode >> 3) & 0b111) as usize;
                let rt: usize = ((opcode) & 0b111) as usize;
                println!("STR R{}, [R{},R{}]",rt,rn,rm);

                let m: u32 = self.r[rm];
                let n: u32 = self.r[rn];
                let t: u32 = self.r[rt];

                let offset_mem = n.wrapping_add(m);
                self.write_u32_to_mem(t,offset_mem);
            }
            // STRB Rt, [Rn, Rm]
            0b010 =>
            {

                let rm: usize = ((opcode >> 6) & 0b111) as usize;
                let rn: usize = ((opcode >> 3) & 0b111) as usize;
                let rt: usize = ((opcode) & 0b111) as usize;
                println!("STRB R{}, [R{},R{}]",rt,rn,rm);

                let m: u32 = self.r[rm];
                let n: u32 = self.r[rn];
                let t: u32 = self.r[rt];

                let offset_mem = n.wrapping_add(m);
                self.write_u8_to_mem((t & 0xff) as u8,offset_mem);
            }
            // LDR Rt, [Rn,Rm]
            0b100 =>
            {
                let rm: usize = ((opcode >> 6) & 0b111) as usize;
                let rn: usize = ((opcode >> 3) & 0b111) as usize;
                let rt: usize = ((opcode ) & 0b111) as usize;

                println!("LDR R{}, [R{},R{}]",rt,rn,rm);
                let m: u32 = self.r[rm];
                let n: u32 = self.r[rn];

                let offset_mem = n.wrapping_add(m);
                let val: u32 = self.read_u32_from_mem(offset_mem);

                self.r[rt] = val;   
            }
            // LDRB Rt, [Rn,Rm]
            0b110 =>
            {
                let rm: usize = ((opcode >> 6) & 0b111) as usize;
                let rn: usize = ((opcode >> 3) & 0b111) as usize;
                let rt: usize = ((opcode ) & 0b111) as usize;

                println!("LDRB R{}, [R{},R{}]",rt,rn,rm);
                let m: u32 = self.r[rm];
                let n: u32 = self.r[rn];

                let offset_mem = n.wrapping_add(m);
                let val: u8 = self.read_u8_from_mem(offset_mem);

                self.r[rt] = val as u32;
            }
            _ =>
            {
                println!("What???");
            }
        }
    }
}