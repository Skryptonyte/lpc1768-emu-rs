
use crate::context::Context;
use std::io::prelude::*;

impl Context
{
    // T1 encoding: 0 0 1 | 0 0 | Rd (bits 10-8) | imm8 (bits 7-0) 
    // MOVS Rd, #imm8
    pub fn movs_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let imm8: u8 = (opcode & 0xff).try_into().unwrap();
        let rd: usize = ((opcode >> 8) & 0x7) as usize;

        println!("MOVS R{},#{}",rd,imm8);
        self.r[rd] = imm8 as u32;

        // Set Z flag
        if imm8 == 0x0{
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        // Set CPSR
        if (( imm8 as u32) >> 31) & 1 == 1
        {
            self.x_psr |= 1 << 31
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
        // Retain C flag
    }

    // T1 encoding: 0 0 0 | 1 1 | 0 | 0 | Rm (bits 8-6) | Rn (bits 5-3) | Rd (2-0)
    // ADDS <Rd>, <Rn>, <Rm>
    pub fn adds_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let rm: usize = ((opcode >> 6) & 0x7) as usize;
        let rn: usize = ((opcode >> 3) & 0x7) as usize;
        let rd: usize = ((opcode >> 0) & 0x7) as usize;

        println!("ADDS R{},R{},R{}",rd,rn,rm);
        let temp: u64 = (self.r[rn] as u64) + (self.r[rm] as u64);

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }


        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        self.r[rd] = (temp & 0xffffffff) as u32;

        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }

    // T1 encoding: 0 0 0 | 1 1 | 0 | 1 | Rm (bits 8-6) | Rn (bits 5-3) | Rd (2-0)
    // SUBS <Rd>, <Rn>, <Rm>
    pub fn subs_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let rm: usize = ((opcode >> 6) & 0x7) as usize;
        let rn: usize = ((opcode >> 3) & 0x7) as usize;
        let rd: usize = ((opcode >> 0) & 0x7) as usize;

        println!("SUBS R{},R{},R{}",rd,rn,rm);

        let temp: u64 = (self.r[rn] as u64) + ( (!self.r[rm]) as u64) + 1 ;

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }

        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        self.r[rd] = (temp & 0xffffffff) as u32;

        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }

    // T1 encoding: 0 0 0 | 1 1 | 1 | 0 | imm3 (bits 8-6) | Rn (bits 5-3) | Rd (2-0)
    // ADDS <Rd>, <Rn>, #imm3
    pub fn adds_imm3_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let imm3: usize = ((opcode >> 6) & 0x7) as usize;
        let rn: usize = ((opcode >> 3) & 0x7) as usize;
        let rd: usize = ((opcode >> 0) & 0x7) as usize;

        println!("ADDS R{},R{},#{}",rd,rn,imm3);
        let temp: u64 = (self.r[rn] as u64) + (imm3 as u64);

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }


        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        self.r[rd] = (temp & 0xffffffff) as u32;

        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }

    // T1 encoding: 0 0 0 | 1 1 | 1 | 1 | imm3 (bits 8-6) | Rn (bits 5-3) | Rd (2-0)
    // SUBS <Rd>, <Rn>, #imm3
    pub fn subs_imm3_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let imm3: usize = ((opcode >> 6) & 0x7) as usize;
        let rn: usize = ((opcode >> 3) & 0x7) as usize;
        let rd: usize = ((opcode >> 0) & 0x7) as usize;

        println!("SUBS R{},R{},#{}",rd,rn,imm3);

        let temp: u64 = (self.r[rn] as u64) + ( (!(imm3 as u32)) as u64) + 1 ;

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }

        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        self.r[rd] = (temp & 0xffffffff) as u32;

        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }

    // ADDS <Rd>, #imm8
    pub fn adds_imm8_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let imm8: u8 = (opcode & 0xff).try_into().unwrap();
        let rd: usize = ((opcode >> 8) & 0x7) as usize;

        println!("ADDS R{},#{} (imm8)",rd,imm8);

        let temp: u64 = (self.r[rd] as u64) + (imm8 as u64);

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }

        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        self.r[rd] = (temp & 0xffffffff) as u32;

        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }

    // SUBS <Rd>, #imm8

    pub fn subs_imm8_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let imm8: u8 = (opcode & 0xff).try_into().unwrap();
        let rd: usize = ((opcode >> 8) & 0x7) as usize;

        println!("SUBS R{},#{} (imm8)",rd,imm8);

        let temp: u64 = (self.r[rd] as u64) + ( (!(imm8 as u32)) as u64) + 1 ;

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }

        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }

        self.r[rd] = (temp & 0xffffffff) as u32;

        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }

    // T1 encoding: 0 1 0 0 0 0 | 1 0 1 0 | Rm (bits 5-3) | Rn (2-0)
    // CMP Rn, Rm
    pub fn cmp_thumb1_opcode(self: &mut Context, opcode: u16)
    {
        
        let rm: usize = ((opcode >> 3) & 0x7) as usize;
        let rn: usize = ((opcode >> 0) & 0x7) as usize;

        println!("CMP R{},R{}",rn,rm);

        let temp: u64 = (self.r[rn] as u64) + ( (!self.r[rm]) as u64) + 1 ;

        // Set C flag
        if temp > 0xffffffff
        {
            self.x_psr |= 1 << 29
        }
        else
        {
            self.x_psr &= !(1 << 29);
        }

        // Set Z flag
        if temp & 0xffffffff == 0
        {
            self.x_psr |= 1 << 30;
        }
        else
        {
            self.x_psr &= !(1 << 30);
        }
        // Set N flag

        if (temp >> 31) & 1== 1
        {
            self.x_psr |= 1 << 31;
        }
        else
        {
            self.x_psr &= !(1 << 31);
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_movs_thumb1(){
        let mut c: Context = Context::new();
        println!("MOVS test");

        let prevcarry: u32 = (c.x_psr >> 29) & 1;
        for i in 0..=7
        {
            // Test MOVS R[0-7], #0
            c.movs_thumb1_opcode( (0b00100 << 11) | (i as u16) << 8 | 0x0 );
            assert_eq!(c.r[i],0x0);            // R[i] = 0x0
            assert_eq!((c.x_psr >> 30) & 1, 1); // Z = 1
            assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

            c.movs_thumb1_opcode( (0b00100 << 11) | (i as u16) << 8 | 0x0f );
            assert_eq!(c.r[i],0x0f);            // R[i] = 0xf
            assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
            assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

            c.movs_thumb1_opcode( (0b00100 << 11) | (i as u16) << 8 | 0xff );
            assert_eq!(c.r[i],0xff);            // R[i] = 0xff
            assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
            assert_eq!((c.x_psr >> 31) & 1, 0); // N = 1
        }

        assert_eq!(prevcarry ,(c.x_psr >> 29) & 1);  // Ensure carry is retained
    }

    #[test]
    fn test_adds_thumb1(){
        let mut c: Context = Context::new();
        println!("ADDS test");

        c.r[1] = 0x10;
        c.r[2] = 0x20;
        c.adds_thumb1_opcode( (0b0001111 << 9) | 1 << 6 | 2 << 3 | 0);

        assert_eq!(c.r[0],0x10 + 0x20);
        assert_eq!((c.x_psr >> 29) & 1, 0); // C = 0
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

        c.r[1] = 0xf739;
        c.r[2] = 0x2182;
        c.adds_thumb1_opcode( (0b0001111 << 9) | 1 << 6 | 2 << 3 | 0);

        assert_eq!(c.r[0],0xf739 + 0x2182);
        assert_eq!((c.x_psr >> 29) & 1, 0); // C = 0
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

        c.r[1] = 0xffffffff;
        c.r[2] = 0x1;
        c.adds_thumb1_opcode( (0b0001100 << 9) | 1 << 6 | 2 << 3 | 0);

        assert_eq!(c.r[0],0);
        assert_eq!((c.x_psr >> 29) & 1, 1); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 1); // Z = 1
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

        c.r[1] = 0xffffffff;
        c.r[2] = 0x2;
        c.adds_thumb1_opcode( (0b0001100 << 9) | 1 << 6 | 2 << 3 | 0);

        assert_eq!(c.r[0],0x1);
        assert_eq!((c.x_psr >> 29) & 1, 1); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0


        c.r[1] = 0x7fffffff;
        c.r[2] = 0x2;
        c.adds_thumb1_opcode( (0b0001100 << 9) | 1 << 6 | 2 << 3 | 0);

        assert_eq!(c.r[0],0x7fffffff+2);
        assert_eq!((c.x_psr >> 29) & 1, 0); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 1); // N = 1
    }

    #[test]
    fn test_subs_thumb1(){
        let mut c: Context = Context::new();
        println!("SUBS test");

        c.r[1] = 0x20;
        c.r[2] = 0x10;
        c.subs_thumb1_opcode( (0b0001101 << 9) | 2 << 6 | 1 << 3 | 0);

        assert_eq!(c.r[0],0x20 - 0x10);
        assert_eq!((c.x_psr >> 29) & 1, 1); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

        c.r[1] = 0xf739;
        c.r[2] = 0x2182;
        c.subs_thumb1_opcode( (0b0001111 << 9) | 2 << 6 | 1 << 3 | 0);

        assert_eq!(c.r[0],0xf739 - 0x2182);
        assert_eq!((c.x_psr >> 29) & 1, 1); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0

        c.r[1] = 0xffffffff;
        c.r[2] = 0x1;
        c.subs_thumb1_opcode( (0b0001101 << 9) | 2 << 6 | 1 << 3 | 0);

        assert_eq!(c.r[0],0xfffffffe);
        assert_eq!((c.x_psr >> 29) & 1, 1); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 1); // N = 1

        c.r[1] = 0x1;
        c.r[2] = 0x2;
        c.subs_thumb1_opcode( (0b0001101 << 9) | 2 << 6 | 1 << 3 | 0);

        assert_eq!(c.r[0],0xffffffff);
        assert_eq!((c.x_psr >> 29) & 1, 0); // C = 0
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 1); // N = 1


        c.r[1] = 0x00;
        c.r[2] = 0xff;
        c.subs_thumb1_opcode( (0b0001101 << 9) | 2 << 6 | 1 << 3 | 0);

        assert_eq!(c.r[0],(!0xff + 1) as u32);
        assert_eq!((c.x_psr >> 29) & 1, 0); // C = 0
        assert_eq!((c.x_psr >> 30) & 1, 0); // Z = 0
        assert_eq!((c.x_psr >> 31) & 1, 1); // N = 1

        c.r[1] = 0x2;
        c.r[2] = 0x2;
        c.subs_thumb1_opcode( (0b0001101 << 9) | 2 << 6 | 1 << 3 | 0);

        assert_eq!(c.r[0],0);
        assert_eq!((c.x_psr >> 29) & 1, 1); // C = 1
        assert_eq!((c.x_psr >> 30) & 1, 1); // Z = 1
        assert_eq!((c.x_psr >> 31) & 1, 0); // N = 0
    }
}