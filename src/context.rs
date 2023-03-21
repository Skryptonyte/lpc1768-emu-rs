use std::io::Cursor;

pub struct Context
{
    pub r: [u32;16],
    pub x_psr: u32,
    pub code: Vec<u8>,
    pub data: Vec<u8> 
}

impl Context
{
    pub fn new() -> Context
    {
        Context{r:[0;16],x_psr:0x0,code:vec![0;0x80000],data:vec![0;0x8000]}
    }

    
}