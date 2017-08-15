use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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

        mcs6502::translate("JMP START", &mut self.data,
                           &mut self.labels, &mut self.jumps);

        self.assemble_file(input);
    }

    fn link(&mut self) {
        if !self.labels.contains_key("start:") {
            panic!("Start label not defined.");
        }

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

impl Assembler6502 {
    fn assemble_file(&mut self, input: &str) {
        let file = File::open(input).
            expect(&format!("Unable to open input file: {}", input));
        let reader = BufReader::new(&file);

        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();

            let upper_line = line.to_uppercase();
            if upper_line.starts_with(".INCLUDE ") {
                let file = self.get_include_file(&line, input);
                self.assemble_file(file.to_str().unwrap());
            } else if !line.is_empty() && !line.starts_with(";") {
                mcs6502::translate(&upper_line, &mut self.data,
                                   &mut self.labels, &mut self.jumps);
            }
        }
    }

    fn get_include_file(&self, line: &str, input: &str) -> PathBuf {
        let (_, rest) = line.split_at(8);
        let line = &rest[2..];
        let file_end = match line.rfind("\"") {
            Some(num) => num,
            None => 0
        };

        if file_end == 0 {
            panic!("Invalid file include: {}", line);
        }

        let (file, _) = line.split_at(file_end);

        let input_path = Path::new(input);
        match input_path.parent() {
            Some(parent) => parent.join(file),
            None => Path::new(file).to_path_buf()
        }
    }
}
