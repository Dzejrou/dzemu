use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;

use asm::Assembler;
use inst::mcs6502;
use util;

pub struct Assembler6502 {
    data:   Vec<u8>,
    labels: HashMap<String, u16>,
    jumps:  HashMap<u16, String>
}

impl Assembler6502 {
    pub fn new() -> Assembler6502 {
        Assembler6502 {
            data:   Vec::new(),
            labels: HashMap::new(),
            jumps:  HashMap::new()
        }
    }
}

impl Assembler for Assembler6502 {
    fn assemble(&mut self, input: &str) {
        self.data.clear();
        self.labels.clear();
        self.jumps.clear();

        let file = File::open(input).
            expect("Unable to open input file.");
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            mcs6502::translate(line.unwrap(), &mut self.data,
                               &mut self.labels, &mut self.jumps);
        }
    }

    fn link(&mut self) {
        for (&addr, label) in self.jumps.iter() {
            match self.labels.get(label) {
                Some(&target) => {
                    let lo = util::lower(target);
                    let hi = util::upper(target);
                    self.data[(addr + 1) as usize] = lo;
                    self.data[(addr + 2) as usize] = hi;
                }
                None => panic!("Label not defined: {}.", label)
            }
        }
    }

    fn output(&mut self, output: &str) {
        let mut writer = File::create(output)
            .expect("Cannot create output file.");
        writer.write_all(&self.data)
            .expect("Cannot write to output file.");
    }
}
