
use crate::context::Context;
use crate::cpu::*;


use std::fs;
use std::io::prelude::*;


use std::fs::File;
impl Context
{
    pub fn dump_context(self: &mut Context)
    {
        println!("---------------");
        println!("R0: {:#08x}, R1: {:#08x}, R2: {:#08x}",self.r[0],self.r[1],self.r[2]);
        println!("R3: {:#08x}, R4: {:#08x}, R5: {:#08x}",self.r[3],self.r[4],self.r[5]);
        println!("R6: {:#08x}, R7: {:#08x}, R8: {:#08x}",self.r[6],self.r[7],self.r[8]);
        println!("R9: {:#08x}, R10: {:#08x}, R11: {:#08x}",self.r[9],self.r[10],self.r[11]);

        println!("PC: {:#08x}",self.r[15]);
        println!("CPSR: {:#08x}",self.x_psr);
        println!("---------------");
    }

    pub fn dump_memory(self: &mut Context)
    {
        for i in (0x10000000..0x10000100).step_by(8)
        {
            print!("{:#08x}: {:#02x} {:#02x} {:#02x} {:#02x}",i,self.read_u8_from_mem(i),
            self.read_u8_from_mem(i+0x1),self.read_u8_from_mem(i+0x2),self.read_u8_from_mem(i+0x3));

            println!(" {:#02x} {:#02x} {:#02x} {:#02x}",self.read_u8_from_mem(i+0x4),
            self.read_u8_from_mem(i+0x5),self.read_u8_from_mem(i+0x6),self.read_u8_from_mem(i+0x7));
        }
    }
    pub fn read_code_into_rom(self: &mut Context)
    {
        let mut f: File = std::fs::OpenOptions::new()
        .read(true).open("./obj").unwrap();

        f.read(&mut self.code).expect("Error reading object file into code!");
    }
    pub fn process_opcode(self: &mut Context) -> bool
    {
        let pc: usize = (self.r[15]) as usize;
        let code: &[u8] = &self.code;

        let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
        let thumbcode: u16 = opcode >> 6;
        // 16-bit Instructions with 7 bit prefix
        match (thumbcode){
            0b0100001010 =>
            {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.cmp_thumb1_opcode(opcode);
                return true;
            }
            _ =>
            {

            }
        }
        let thumbcode: u16 = opcode >> 9;

        match (thumbcode){
            0b0001100 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.adds_thumb1_opcode(opcode);
                return true;
            }
            0b0001101 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.subs_thumb1_opcode(opcode);
                return true;
            }
            0b0001110 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.adds_imm3_thumb1_opcode(opcode);
                return true;
            }
            0b0001111 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.subs_imm3_thumb1_opcode(opcode);
                return true;
            }
            _ => {
            }
        }

        // 16-bit Instructions with 5 bit prefix
        let thumbcode: u16 = opcode >> 11;
        
        match (thumbcode){
            // MOVS 
            0b00100 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.movs_thumb1_opcode(opcode);
                return true;
            }
            // ADDS imm8
            0b00110 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.adds_imm8_thumb1_opcode(opcode);
                return true;
            }
            // SUBS imm8
            0b00111 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.r[15] += 2;
                self.subs_imm8_thumb1_opcode(opcode);
                return true;
            }
            // LDR immediate
            0b01001 =>
            {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.ldr_immediate_thumb1(opcode);
                self.r[15] += 2;

                return true;
            }
            // Branch allways
            0b11100 =>
            {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.branch_t2_opcode(opcode);
                return true;
            }
            _ => {
            }
        }

        // 16-bit Instructions with 4 bit prefix
        let thumbcode: u16 = opcode >> 12;
        
        match (thumbcode){
            // Load/Store wrapper
            0b0101 =>
            {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.loadstore_register_thumb1(opcode);
                self.r[15] += 2;
                return true; 
            }
            // Branch wrapper
            0b1101 => {
                let opcode: u16 = (code[pc+1] as u16)<< 8 | (code[pc] as u16);
                self.branch_t1_opcode(opcode);
                return true;
            }
            _ => {
            }
        }

        
        println!("Unrecognized opcode: {}",thumbcode);
        return false;
    }
}