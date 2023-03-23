use crate::context::Context;
use crate::utils::memory_access;

use std::io::prelude::*;

impl Context
{

    pub fn ldr_immediate_pool_thumb1(self: &mut Context, opcode: u16)
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

        let rm: usize = ((opcode >> 6) & 0b111) as usize;
        let rn: usize = ((opcode >> 3) & 0b111) as usize;
        let rt: usize = ((opcode) & 0b111) as usize;
        
        let m: u32 = self.r[rm];
        let n: u32 = self.r[rn];
        let t: u32 = self.r[rt];

        let offset_mem = n.wrapping_add(m);

        match opB
        {
            // STR Rt, [Rn, Rm]
            0b000 =>
            {
                println!("STR R{}, [R{},R{}]",rt,rn,rm);
                self.write_u32_to_mem(t,offset_mem);
            }
            // STRH Rt, [Rn, Rm]
            0b001 =>
            {
                println!("STRH R{}, [R{},R{}]",rt,rn,rm);
                self.write_u16_to_mem((t & 0xffff) as u16,offset_mem);
            }
            // STRB Rt, [Rn, Rm]
            0b010 =>
            {
                println!("STRB R{}, [R{},R{}]",rt,rn,rm);
                self.write_u8_to_mem((t & 0xff) as u8,offset_mem);
            }
            // LDRSB Rt, [Rn,Rm]
            0b011 =>
            {
                println!("LDRB R{}, [R{},R{}]",rt,rn,rm);

                let val: u8 = self.read_u8_from_mem(offset_mem);
                self.r[rt] = val as i8 as i32 as u32;  // Idiomatically sign extend 8 bit to 32 bit
            }
            // LDR Rt, [Rn,Rm]
            0b100 =>
            {
                println!("LDR R{}, [R{},R{}]",rt,rn,rm);
                let val: u32 = self.read_u32_from_mem(offset_mem);
                self.r[rt] = val;   
            }
            // LDRH Rt, [Rn,Rm]
            0b101 =>
            {
                println!("LDRH R{}, [R{},R{}]",rt,rn,rm);
                let val: u16 = self.read_u16_from_mem(offset_mem);
                self.r[rt] = val as u32;   
            }
            // LDRB Rt, [Rn,Rm]
            0b110 =>
            {
                println!("LDRB R{}, [R{},R{}]",rt,rn,rm);

                let val: u8 = self.read_u8_from_mem(offset_mem);
                self.r[rt] = val as u32;
            }
            // LDRSH Rt, [Rn,Rm]
            0b111 =>
            {
                println!("LDRSH R{}, [R{},R{}]",rt,rn,rm);
                let val: u16 = self.read_u16_from_mem(offset_mem);
                self.r[rt] = val as i16 as i32 as u32;   // Idiomatically sign extend 8 bit to 32 bit
            }
            
            _ =>
            {
                println!("What???");
            }
        }



    }

    // T1 encoding
    // 0 1 1 0 | 0/1 (bit 11)| imm5 (bits 10-6)| Rn (bits 5-3)| Rt (bits 2-0)
    // LDR/STR Rt, [Rn, #imm5]
    pub fn loadstore_imm_thumb1(self: &mut Context, opcode: u16)
    {
        let opA = opcode >> 12;
        let opB = (opcode >> 11) & 0b1;

        let imm5: u32 = ((opcode >> 6) & 0b11111) as u32;
        let imm32: u32 = imm5 << 2;    // Zero extend imm5 + '00'

        let rn: usize = ((opcode >> 3) & 0b111) as usize;
        let rt: usize = ((opcode) & 0b111) as usize;

        let t: u32 = self.r[rt];
        let n: u32 = self.r[rn];

        let offset_mem = n.wrapping_add(imm32);
        match opB
        {
            // STR Rt, [Rn, #imm5]
            0b0=>
            {

                println!("STR R{}, [R{},#{}]",rt,rn,imm32);
                self.write_u32_to_mem(t,offset_mem);
            }
            // LDR Rt, [Rn,#imm5]
            0b1 =>
            {
                println!("LDR R{}, [R{},#{}]",rt,rn,imm32);
                let val: u32 = self.read_u32_from_mem(offset_mem);
                self.r[rt] = val;   
            }
            _ =>
            {
                println!("How the hell did you even make it here?");
            }
        }
    }

    // T1 encoding
    // 0 1 1 1 | 0/1 (bit 11)| imm5 (bits 10-6)| Rn (bits 5-3)| Rt (bits 2-0)
    // LDRB/STRB Rt, [Rn, #imm5]
    pub fn loadstore_byte_imm_thumb1(self: &mut Context, opcode: u16)
    {
        let opA = opcode >> 12;
        let opB = (opcode >> 11) & 0b1;

        let imm5: u32 = ((opcode >> 6) & 0b11111) as u32;
        let imm32: u32 = imm5;    // Zero extend imm5 

        let rn: usize = ((opcode >> 3) & 0b111) as usize;
        let rt: usize = ((opcode) & 0b111) as usize;

        let t: u8 = (self.r[rt] & 0xff) as u8;
        let n: u32 = self.r[rn] ;

        let offset_mem = n.wrapping_add(imm32);
        match opB
        {
            // STR Rt, [Rn, #imm5]
            0b0=>
            {

                println!("STRB R{}, [R{},#{}]",rt,rn,imm32);
                self.write_u8_to_mem(t,offset_mem);
            }
            // LDR Rt, [Rn,#imm5]
            0b1 =>
            {
                println!("LDRB R{}, [R{},#{}]",rt,rn,imm32);
                let val: u32 = self.read_u8_from_mem(offset_mem) as u32;
                self.r[rt] = val;   
            }
            _ =>
            {
                println!("How the hell did you even make it here?");
            }
        }
    }

     // T1 encoding
    // 1 0 0 0 | 0/1 (bit 11)| imm5 (bits 10-6)| Rn (bits 5-3)| Rt (bits 2-0)
    // LDRB/STRB Rt, [Rn, #imm5]
    pub fn loadstore_halfword_imm_thumb1(self: &mut Context, opcode: u16)
    {
        let opA = opcode >> 12;
        let opB = (opcode >> 11) & 0b1;

        let imm5: u32 = ((opcode >> 6) & 0b11111) as u32;
        let imm32: u32 = imm5 << 1;    // Zero extend imm5 + '0'

        let rn: usize = ((opcode >> 3) & 0b111) as usize;
        let rt: usize = ((opcode) & 0b111) as usize;

        let t: u16 = (self.r[rt] & 0xffff) as u16;
        let n: u32 = self.r[rn];

        let offset_mem = n.wrapping_add(imm32);
        match opB
        {
            // STR Rt, [Rn, #imm5]
            0b0=>
            {

                println!("STRH R{}, [R{},#{}]",rt,rn,imm32);
                self.write_u16_to_mem(t,offset_mem);
            }
            // LDR Rt, [Rn,#imm5]
            0b1 =>
            {
                println!("LDRH R{}, [R{},#{}]",rt,rn,imm32);
                let val: u32 = self.read_u16_from_mem(offset_mem) as u32;
                self.r[rt] = val;   
            }
            _ =>
            {
                println!("How the hell did you even make it here?");
            }
        }
    }
}