use std::fs::File;
use std::io::Read;

pub fn read_rom(fname: &str) -> Vec<u8> {
    let rom_file = match File::open(fname) {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {:?}", error),
    };

    rom_file.bytes().map(|b| b.unwrap()).collect()
}

pub fn dump_rom(rom: &Vec<u8>) {
    for instr in rom.iter() {
        println!("{}", instr);
    }
}
