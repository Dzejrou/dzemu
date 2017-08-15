use std::fs::File;
use std::io::Read;

use mems::Memory;
use inst::mcs6502;

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
        println!("{}", mcs6502::op_to_str(rom, &mut idx));
    }
    println!("-------------");
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

#[inline]
fn is_ascii_number(byte: u8) -> bool {
    byte >= 48 && byte <= 57
}

#[inline]
fn is_ascii_capital_alpha(byte: u8) -> bool {
    byte >= 65 && byte <=70
}

#[inline]
fn is_ascii_alpha(byte: u8) -> bool {
    byte >= 97 && byte <= 102
}

pub fn u8_to_number(byte: u8, base: u8) -> Option<u8> {
    match base {
        10 => {
            if is_ascii_number(byte) {
                Some(byte - 48)
            } else {
                None
            }
        },

        16 => {
            if is_ascii_number(byte) {
                Some(byte - 48)
            } else if is_ascii_alpha(byte) {
                Some(byte - 97 + 10)
            } else if is_ascii_capital_alpha(byte) {
                Some(byte - 65 + 10)
            } else {
                None
            }
        },

        _ => None
    }
}

pub fn u16_to_number(word: u16, base: u8) -> Option<u16> {
    // Each byte is a digit.
    let lo = u8_to_number(lower(word), base);
    let hi = u8_to_number(upper(word), base);

    let lo_byte: u16;
    let hi_byte: u16;
    let base = base as u16;

    match lo {
        Some(num) => lo_byte = num as u16,
        None => return None
    }

    match hi {
        Some(num) => hi_byte = num as u16,
        None => return None
    }

    Some(hi_byte * base + lo_byte)
}

pub fn is_valid_label(label: &str, decl: bool) -> bool {
    let chars: Vec<char> = label.chars().collect();
    let len = chars.len();
    for i in 0..len - 2 {
        if !chars[i].is_alphabetic() && chars[i] != '_' {
            return false;
        }
    }

    // Labels can optionally end with colon when declared.
    if decl {
        chars[len - 1] == ':' || chars[len - 1].is_alphabetic()
    } else {
        chars[len - 1].is_alphabetic()
    }
}

#[cfg(test)]
mod tests {
    use util::*;

    #[test]
    fn u8_number_conversion() {
        let c = 'C' as u8;
        let d = 'd' as u8;
        let x = 'x' as u8;
        let two = '2' as u8;

        assert_eq!(u8_to_number(c, 16), Some(12));
        assert_eq!(u8_to_number(d, 16), Some(13));
        assert_eq!(u8_to_number(two, 10), Some(2));
        assert_eq!(u8_to_number(two, 16), Some(0x2));
        assert_eq!(u8_to_number(x, 16), None);
        assert_eq!(u8_to_number(c, 10), None);
    }

    #[test]
    fn u16_number_conversion() {
        let num1 = to_u16('A' as u8, '2' as u8);
        let num2 = to_u16('6' as u8, '5' as u8);
        let not_num = to_u16('G' as u8, '2' as u8);

        assert_eq!(u16_to_number(num1, 16), Some(0xA2));
        assert_eq!(u16_to_number(num2, 10), Some(65));
        assert_eq!(u16_to_number(num2, 16), Some(0x65));
        assert_eq!(u16_to_number(not_num, 10), None);
        assert_eq!(u16_to_number(not_num, 16), None);
        assert_eq!(u16_to_number(num1, 10), None);
    }
}
