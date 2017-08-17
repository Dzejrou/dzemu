use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use asm::Assembler;
use inst::mcs6502;
use inst::mcs6502::AddressMode;
use util;

pub struct Assembler6502 {
    data:      Vec<u8>,
    files:     Vec<String>,
    labels:    HashMap<String, u16>,
    jumps:     HashMap<u16, String>,
    branches:  HashMap<u16, String>,
    vars:      HashMap<String, u16>,
    var_uses:  HashMap<u16, String>,
    values:    HashMap<String, Vec<u8>>
}

impl Assembler6502 {
    pub fn new() -> Assembler6502 {
        Assembler6502 {
            data:     Vec::new(),
            files:    Vec::new(),
            labels:   HashMap::new(),
            jumps:    HashMap::new(),
            branches: HashMap::new(),
            vars:     HashMap::new(),
            var_uses: HashMap::new(),
            values:   HashMap::new()
        }
    }
}

impl Assembler for Assembler6502 {
    fn assemble(&mut self, input: &str) {
        self.data.clear();
        self.labels.clear();
        self.jumps.clear();

        self.translate("JMP START");

        self.assemble_file(input);
    }

    fn link(&mut self) {
        // TODO: Implement shifting? Shift all bytes from
        //       address X and increment all labels by
        //       the shifted amount if they are after X?
        //       This would allow inserting code after
        //       assembling but before linking.
        if !self.labels.contains_key("START:") {
            panic!("Start label not defined.");
        }

        // Skip data section.
        self.translate("JMP END");

        // Calculate address and insert initial values.
        for (name, addr) in self.vars.iter_mut() {
            *addr = self.data.len() as u16;
            let name = name.clone();

            let default = vec![0];
            let values = self.values.get(&*name).unwrap_or(&default);
            for value in values.iter() {
                self.data.push(*value);
            }
        }

        for (&addr, var) in self.var_uses.iter() {
            match self.vars.get(var) {
                Some(&target) => {
                    self.data[(addr + 1) as usize] = util::lower(target);
                    self.data[(addr + 2) as usize] = util::upper(target);
                }
                None => {
                    panic!("Variable not defined: {}", var);
                }
            }
        }

        self.translate("END:");
        self.translate("NOP");

        for (&addr, label) in self.jumps.iter() {
            match self.labels.get(label) {
                Some(&target) => {
                    let lo = util::lower(target);
                    let hi = util::upper(target);
                    self.data[(addr + 1) as usize] = lo;
                    self.data[(addr + 2) as usize] = hi;
                }
                None => panic!("Label not defined: {}", label)
            }
        }

        for (&addr, label) in self.branches.iter() {
            match self.labels.get(label) {
                Some(&target) => {
                    let target = target as i16;
                    let addr = addr as i16;
                    let off = target - addr;

                    if off > 127i16 {
                        panic!("Branch label at 0x{:X} too far: {}", addr, label);
                    } else {
                        self.data[(addr + 1) as usize] = util::lower(off as u16);
                    }
                }
                None => panic!("Label not defined: {}", label)
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
    pub fn translate(&mut self, command: &str) {
        let data_end = self.data.len() as u16;

        if mcs6502::is_valid_label(command, true) {
            let mut label = String::from(command);
            if !label.ends_with(":") {
                label.push_str(":");
            }

            match self.labels.insert(label, data_end as u16) {
                Some(_) => panic!("Redefinition of label in {}", command),
                None    => ()
            }
            return;
        }

        let cmd_tmp;
        match command.find(";") {
            Some(num) => {
                let (cmd, _) = command.split_at(num);
                cmd_tmp = String::from(cmd);
            }
            None => {
                cmd_tmp = String::from(command);
            }
        }

        let command = cmd_tmp.trim();
        let mut space_idx;

        let op;
        let arg;

        if command.len() > 3 {
            match command.find(" ") {
                Some(num) => space_idx = num,
                None => panic!("Malformed command: {}", command)
            }

            if space_idx != 3 {
                let (label, rest) = command.split_at(space_idx);
                let rest = rest.trim();

                let mut label = String::from(label).to_lowercase();
                assert!(mcs6502::is_valid_label(&label, true), "Invalid label: {}.", label);
                if !label.ends_with(":") {
                    label.push_str(":");
                }
                match self.labels.insert(label, data_end) {
                    Some(_) => panic!("Redefinition of label in {}", command),
                    None    => ()
                }

                match rest.find(" ") {
                    Some(num) => space_idx = num,
                    None => panic!("Malformed command: {}", command)
                }

                let (op_, arg_) = rest.split_at(space_idx);
                op = op_;
                arg = arg_;
            } else {
                let (op_, arg_) = command.split_at(space_idx);
                op = op_;
                arg = arg_;
            }
        } else {
            op = command;
            arg = "";
        }

        let op = op.trim();
        let arg = arg.trim();
        let (addr_mode, operand) = mcs6502::parse_arguments(&arg);
        let mut addr_mode = addr_mode;

        if op.starts_with("B") && addr_mode == AddressMode::Absolute {
            // Relative and 2 digit absolute don't differ in assembly :/
            addr_mode = AddressMode::Relative;
        }

        let op = mcs6502::name_mode_to_opcode(op, &addr_mode);
        self.push_instruction(op, operand, addr_mode, &arg);
    }

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
                let file = String::from(file.to_str().unwrap());

                if !self.files.contains(&file) {
                    self.files.push(file.clone());
                    self.assemble_file(&file);
                }
            } else if upper_line.starts_with(".BYTE ") {
                // TODO: Special instruction + length before variable
                //       so that disassembler knows to skip it, maybe only
                //       in debug mode? Even with a name of the variable?
                self.declare_variable(&upper_line);
            } else if upper_line.starts_with(".WORD ") {
                // TODO: 16 bit variables, also add 16 bit
                //       custom instructions!
            } else if upper_line.starts_with(".MACRO ") {
                // TODO: Macros, probably add .ENDMACRO and change state,
                //       while in macro state, instead of translate below
                //       you define the macro until .ENDMACRO is found?
                // TODO: Do it before compilation like C does?
            } else if upper_line.starts_with(".STRING ") {
                // TODO: Add strings? Basically same as .BYTE array, but
                //       translates chars to bytes.
            } else if !line.is_empty() && !line.starts_with(";") {
                self.translate(&upper_line);
            }
        }
    }

    fn get_include_file(&self, line: &str, input: &str) -> PathBuf {
        let (_, rest) = line.split_at(8);
        let line = &rest[2..];
        let file_end = line.rfind("\"").unwrap_or(0);

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

    fn declare_variable(&mut self, line: &str) {
        let words: Vec<&str> = line.split(" ").collect();

        let count = words.len();
        let mut values: Vec<u8> = Vec::new();
        for i in 2..count {
            let chars: Vec<char> = words[i].chars().collect();
            let value = mcs6502::extract_operand_u8(&chars, 0);
            if let Some(value) = value {
                values.push(util::lower(value));
            } else {
                panic!("Invalid byte initializer: {}", words[i]);
            }
        }

        if !mcs6502::is_valid_label(words[1], false) {
            panic!("Invalid variable name: '{}'", words[1]);
        }

        let mut name = String::from(words[1]);
        name.push_str(":");
        self.vars.insert(name.clone(), 0u16);
        self.values.insert(name, values);
    }

    fn push_instruction(&mut self, op: u8, operand: u16, mode: AddressMode, arg: &str) {
        let data_end = self.data.len() as u16;

        match mode {
            AddressMode::Implied     => {
                self.push_one_byte(op);
            }

            AddressMode::Accumulator |
            AddressMode::Relative    |
            AddressMode::Immediate   |
            AddressMode::IndirectX   |
            AddressMode::IndirectY   |
            AddressMode::ZeroPageX   |
            AddressMode::ZeroPageY   |
            AddressMode::ZeroPage    => {
                self.push_two_byte(op, util::lower(operand));
            }

            AddressMode::Absolute    |
            AddressMode::AbsoluteX   |
            AddressMode::AbsoluteY   |
            AddressMode::Indirect    => {
                self.push_three_byte(op, operand);
            }

            AddressMode::Label       |
            AddressMode::LabelX      |
            AddressMode::LabelY      => {
                let mut label = String::from(arg).to_uppercase();
                label = self.strip_index(&label);
                label.push_str(":");

                if mcs6502::can_jump_to_label(op) {
                    self.jumps.insert(data_end , label);
                    self.push_three_byte(op, 0x00u16);
                } else if mcs6502::can_branch_to_label(op) {
                    self.branches.insert(data_end, label);
                    self.push_two_byte(op, 0x00u8);
                } else if mcs6502::can_use_variables(op) {
                    self.var_uses.insert(data_end, label);
                    self.push_three_byte(op, 0x0000u16);
                } else {
                    panic!("Instruction cannot use label: 0x{:X} {} ({:?})", op, label, mode);
                }
            }

            _                     => {
                panic!("Invalid address mode {:?} for opcode: {}", mode, op);
            }
        }
    }

    fn push_one_byte(&mut self, op: u8) {
        self.data.push(op);
    }

    fn push_two_byte(&mut self, op: u8, operand: u8) {
        self.data.push(op);
        self.data.push(operand);
    }

    fn push_three_byte(&mut self, op: u8, operand: u16) {
        self.data.push(op);
        self.data.push(util::lower(operand));
        self.data.push(util::upper(operand));
    }

    fn strip_index(&self, label: &str) -> String {
        let mut chars = label.chars();
        let mut res = String::new();

        let mut c = chars.next().unwrap_or(' ');
        while c.is_alphanumeric() || c == '_' {
            res.push(c);
            c = chars.next().unwrap_or(' ');
        }

        res
    }
}
