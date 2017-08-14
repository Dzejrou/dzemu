extern crate dzemu;

use dzemu::asm::Assembler;
use dzemu::asm::mcs6502::Assembler6502;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("rom file not provided");
    }

    let mut asm = Assembler6502::new();
    asm.assemble(&args[1]);
    asm.link();
    asm.output("test.out");
}
