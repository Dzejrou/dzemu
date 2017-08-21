use std::fs::File;
use std::io::Read;

use mems::Memory;
use inst::mcs6502;
use inst::mcs6502::ops;

pub fn read_rom(fname: &str) -> Vec<u8> {
    let rom_file = match File::open(fname) {
        Ok(file) => file,
        Err(error) => panic!("Error opening file: {:?}", error),
    };

    rom_file.bytes().map(|b| b.unwrap()).collect()
}

pub fn dump_rom(rom: &Memory) {
    println!("Rom contents:");
    let mut idx = 0;
    while idx < rom.size() {
        let opcode = rom.read_u8(idx);
        let mut offset = 0u8;

        if opcode == ops::custom::VARIABLE {
            offset = rom.read_u8(idx + 1);
        }

        let i = idx;
        println!("0x{:04X}: {}", i, mcs6502::op_to_str(rom, &mut idx));

        idx = idx + offset as usize;
    }
    println!("-------------");
}

pub fn extract_indirect_target(chars: &[char]) -> Vec<char> {
    let size = chars.len();

    let mut target: Vec<char> = Vec::new();
    for i in 1..size - 1 {
        if chars[i] != ')' {
            target.push(chars[i]);
        } else {
            break;
        }
    }

    target
}

#[inline]
pub fn lower(data: u16) -> u8 {
    (data & 0xFF) as u8
}

#[inline]
pub fn upper(data: u16) -> u8 {
    (data >> 8) as u8
}

#[inline]
pub fn to_u16(hi: u8, lo: u8) -> u16 {
    let hi = hi as u16;
    let lo = lo as u16;
    (hi << 8) | lo
}

fn is_valid_label_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'

}

pub fn is_valid_label(label: &str, decl: bool) -> bool {
    let chars: Vec<char> = label.chars().collect();
    let len = chars.len();

    if !chars[0].is_alphabetic() && chars[0] != '_' {
        return false;
    }

    for i in 1..len - 2 {
        if !is_valid_label_char(chars[i]) {
            return false;
        }
    }

    // Labels can optionally end with colon when declared.
    if decl {
        chars[len - 1] == ':' || is_valid_label_char(chars[len - 1])
    } else {
        is_valid_label_char(chars[len - 1])
    }
}
