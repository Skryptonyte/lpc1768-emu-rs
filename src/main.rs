

mod context;
mod emu;
mod cpu;
mod utils;

use context::Context;
fn main() {
    println!("Start emulation!");
    let mut context: Context = Context::new();
    context.read_code_into_rom();

    
    while context.process_opcode()
    {
        context.dump_context();
    }
    context.dump_memory();
}
