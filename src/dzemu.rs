extern crate dzemu;

use dzemu::util;
use dzemu::cpus::Cpu;
use dzemu::cpus::mcs6502::Mcs6502;
use dzemu::mems::ram::Ram8b;
use dzemu::mems::rom::Rom8b;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("rom file not provided");
    }

    let rom = Rom8b::from_vec(util::read_rom(&args[1]));
    util::dump_rom(&rom);

    let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));
    cpu.boot(&rom);

    while cpu.running() {
        cpu.execute();
    }
}
