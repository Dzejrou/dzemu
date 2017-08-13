extern crate dzemu;

use std::{thread, time};

use dzemu::util;
use dzemu::inst::mcs6502;
use dzemu::cpus::Cpu;
use dzemu::cpus::mcs6502::Mcs6502;
use dzemu::mems::Memory;
use dzemu::mems::ram::Ram8b;
use dzemu::mems::rom::Rom8b;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("rom file not provided");
    }

    let rom = Rom8b::from_vec(util::read_rom(&args[1]));

    println!("Rom contents:");
    let mut idx = 0;
    while idx < rom.size() {
        println!("{}", mcs6502::op_to_str(&rom, &mut idx));
    }
    println!("-------------");

    let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));
    cpu.boot(&rom);

    loop {
        cpu.execute();
        thread::sleep(time::Duration::from_secs(1));
    }
}
