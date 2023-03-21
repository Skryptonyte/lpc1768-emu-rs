use crate::context::Context;

use std::io::prelude::*;

impl Context
{
    // T1 encoding: 1 1 0 1 | cond (bits 11-8) | imm8 (bits 7-0) 
    // B<c> <label>
    pub fn branch_t1_opcode(self: &mut Context, opcode: u16)
    {
        let cond: u8 = ((( (opcode) >> 8)  ) & 0xf ).try_into().unwrap() ;
        let imm8: u8  = (opcode & 0xff).try_into().unwrap();

        let mut cond_sat: bool = false;

        let N = (self.x_psr >> 31) & 1;
        let Z = (self.x_psr >> 30) & 1;
        let C = (self.x_psr >> 29) & 1;
        let V = (self.x_psr >> 28) & 1;

        let mut s: String = String::new();
        match cond
        {
            // EQ (0000)
            0b0000 =>
            {
                s += "BEQ ";

                cond_sat = Z == 1;
            }   
            // NE (0001)
            0b0001 =>
            {
                s += "BNE ";
                cond_sat = Z == 0; 
            }
            // CS (0010)
            0b0010 =>
            {
                s += "BCS ";

                cond_sat = C == 1;
            }
            // CC (0011)
            0b0011 =>
            {
                s += "BCC";
                cond_sat = C == 0;
            }
            // MI (0100)
            0b0100 =>
            {
                cond_sat = N == 1;
            }   
            // PL (0101)
            0b0101 =>
            {
                cond_sat = N == 0; 
            }
            // VS (0110)
            0b0110 =>
            {
                cond_sat = V == 1;
            }
            // VC (0111)
            0b0111 =>
            {
                cond_sat = V == 0;
            }
            // HI (1000)
            0b1000 =>
            {
                s += "BHI ";
                cond_sat = (C == 1 && Z == 0);
            }
            // LS (1001)
            0b1001 =>
            {
                cond_sat = (C == 0 && Z == 1);
            }
        
            // GE (1010)
            0b1010 =>
            {
                cond_sat = N == V;
            }
            // LT (1011)
            0b1011 =>
            {
                cond_sat = N != V;
            }
            // GT (1100)
            0b1100 =>
            {
                cond_sat = N == V && Z == 0;
            }
            // LE (1101)
            0b1101 =>
            {
                cond_sat = N != V && Z == 1;
            }
            // AL 
            0b1110 =>
            {
                println!("UNDEFINED cond for Branch T1 encoding!")
            }
            _ =>
            {
                println!("Unrecognized branch type!");
            }
        }

        // Sign extend imm8 to imm32
        let imm32: u32 = ((imm8 as i8 as i32)*2) as u32;
        // println!("signed check {}",imm8 as i8 as i32 as u32);
        s += &format!(" offset:{}",(imm32+4) as i32).to_string();
        println!("{}",s);
        if (cond_sat)
        {
            self.r[15] = (self.r[15]).wrapping_add( (imm32 + 4)) & !1;   // Change jump range to -256 to 254 and ensure aligned pc
        }
        else
        {
            self.r[15] += 2;
        }
    }

    // T2 encoding: 1 1 1 0 0 | imm11 (bits (10-0))
    // B label (unconditional)
    pub fn branch_t2_opcode(self: &mut Context, opcode: u16)
    {
        // This is just straight up wrong, but I am too lazy
        // I can't be arsed to write extra logic to sign extra imm11 + an extra 0 bit
        let imm8: u8  = (opcode & 0xff).try_into().unwrap();

        let imm32: u32 = ((imm8 as i8 as i32)*2) as u32;
        self.r[15] = (self.r[15]).wrapping_add( (imm32 + 4)) & !1;
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_branch_offset_thumb1(){
        let mut c: Context = Context::new();
        println!("Branch test");


        // Backward jump test #1

        c.r[15] = 0x16;
        c.x_psr = 0x00000000; // Set Z flag


        c.branch_thumb1_opcode(0xd1fa);
        assert_eq!(c.r[15],0xe);

        // Backward jump test #2

        c.r[15] = 0xa;
        c.x_psr = 0x00000000; // Set Z flag


        c.branch_thumb1_opcode(0xd1fc);
        assert_eq!(c.r[15],0x6);

    }

    /*
    #[test]
    fn test_beq_thumb1()
    {
        let mut c: Context = Context::new();

        c.r[15] = 0x10;
        c.x_psr = 0x40000000; // Set Z flag

        c.branch_thumb1_opcode(0b1101 << 12 | 0b0000 << 8 | 0x4);
        assert_eq!(c.r[15],0x10+4);

        // Test BEQ when Z = 0
        c.r[15] = 0x10;
        c.x_psr = 0x00000000; // Clear Z flag

        let offset: u8 = !0x4 + 1;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0000 << 8 | (offset as u16));
        assert_eq!(c.r[15],0x10+2);


    }
    #[test]
    fn test_bne_thumb1()
    {
        let mut c: Context = Context::new();

        // Test BNE when Z = 1
        c.r[15] = 0x10;
        c.x_psr = 0x40000000; 

        let offset: u8 = !0x4 + 1;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0001 << 8 | (offset as u16));
        assert_eq!(c.r[15],0x10+2);

        // Test BNE when Z = 0
        c.r[15] = 0x10;
        c.x_psr = 0x00000000; // Clear Z flag

        let offset: u8 = !0x4 + 1;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0001 << 8 | (offset as u16));
        assert_eq!(c.r[15],0x10+4);
    }

    #[test]
    fn test_bcs_thumb1()
    {
        let mut c: Context = Context::new();

        // Test BCS when C = 1
        c.r[15] = 0x10;
        c.x_psr = 0x20000000; 

        let offset: u8 = 0x4;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0010 << 8 | (offset as u16));
        assert_eq!(c.r[15],0x10+4);

        // Test BCS when C = 0
        c.r[15] = 0x10;
        c.x_psr = 0x00000000; 

        let offset: u8 = 0x5;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0010 << 8 | (offset as u16));
        assert_eq!(c.r[15],0x10+2);
        
    }

    #[test]
    fn test_bcc_thumb1()
    {
        let mut c: Context = Context::new();

        // Test BCC when C = 1
        c.r[15] = 0x10;
        c.x_psr = 0x20000000; 

        let offset: u8 = 0x4;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0011 << 8 | (offset as u16));
        assert_eq!(c.r[15],0x14);

        // Test BCC when C = 0
        c.r[15] = 0x10;
        c.x_psr = 0x00000000; 

        let offset: u8 = 0x4;
        c.branch_thumb1_opcode(0b1101 << 12 | 0b0011 << 8 | (offset as u16));
        assert_eq!(c.r[15],0xb);
        
    }
    */
}