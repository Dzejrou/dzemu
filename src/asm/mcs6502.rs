use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use asm::Assembler;
use asm::VariableSize;
use inst::mcs6502;
use inst::mcs6502::ops;
use inst::mcs6502::addr;
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
    prep:      Preprocessor,
    debug:     bool
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
            prep:     Preprocessor::new(),
            debug:    false
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

        for (&addr, var) in self.var_uses.iter() {
            match self.vars.get(var) {
                Some(&target) => {
                    self.data[(addr + 1) as usize] = util::lower(target);

                    // IndirectX and IndirectY only have 1 byte operand.
                    let mode = addr::get_addr_mode(self.data[addr as usize]);
                    if addr::pc_offset(&mode)  == 3 {
                        self.data[(addr + 2) as usize] = util::upper(target);
                    }
                }
                None => {
                    panic!("Variable not defined: {}", var);
                }
            }
        }

        self.translate("END:");

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

    fn debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}

impl Assembler6502 {
    pub fn translate(&mut self, command: &str) {
        let data_end = self.data.len() as u16;
        let mut command = command;

        // Label definition on its own line.
        if mcs6502::is_valid_identifier(command, true) {
            let mut label = command.to_string();
            if !label.ends_with(":") {
                label.push_str(":");
            }

            match self.labels.insert(label, data_end as u16) {
                Some(_) => panic!("Redefinition of label in {}", command),
                None    => ()
            }
            return;
        }

        let tokens: Vec<&str> = command.split(";").collect();
        if tokens.len() == 0 || tokens[0] == "" || tokens[0].starts_with(";") {
            return; // Commented line.
        } else {
            command = tokens[0];
        }

        let mut tokens: Vec<&str> = command.split_whitespace().collect();
        assert!(tokens.len() > 0);

        if mcs6502::is_valid_identifier(tokens[0], true) {
            let mut label = tokens[0].to_string();
            if !label.ends_with(":") {
                label.push_str(":");
            }

            match self.labels.insert(label, data_end) {
                Some(_) => panic!("Redefinition of label in {}", command),
                None    => ()
            }

            tokens.remove(0);
        }

        let op = tokens[0];

        let mut arg = if tokens.len() > 1 {
            tokens[1].to_string()
        } else {
            "".to_string()
        };
        if tokens.len() > 2 && arg.ends_with(',') {
            arg.push_str(tokens[2]);
        }

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
        let mut lines: Vec<String> = reader.lines()
                                        .map(|s| s.unwrap().to_string())
                                        .collect();

        lines = self.prep.process(lines);

        for line in lines.iter() {
            let line = line.trim();

            if line.starts_with(".INCLUDE ") {
                // This is a problem, includes will always be changed to lowercase :/
                let file = self.get_include_file(&line.to_lowercase(), input);
                let file = String::from(file.to_str().unwrap());

                if !self.files.contains(&file) {
                    self.files.push(file.clone());
                    self.assemble_file(&file);
                }
            } else if line.starts_with(".BYTE ") {
                self.declare_variable(&line, VariableSize::Byte);
            } else if line.starts_with(".WORD ") {
                // TODO: 16bit instructions?
                self.declare_variable(&line, VariableSize::Word);
            } else if !line.is_empty() && !line.starts_with(";") {
                self.translate(&line);
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

    fn declare_variable(&mut self, line: &str, size: VariableSize) {
        // TODO: If we declare an array and place ; in the
        //       middle of the init values, the assembler will
        //       flip out.
        let words: Vec<&str> = line.split(" ").collect();

        let count = words.len();
        let mut values: Vec<u8> = Vec::new();

        // Marker for dissasembler.
        if self.debug {
            values.push(ops::custom::VARIABLE);
            values.push(((count - 2) as u8) * size.bytes());
        }

        // Default value.
        if count == 2 {
            for _ in 0 .. size.bytes() {
                values.push(0x00u8);
            }
        } else {
            for i in 2..count {
                let chars: Vec<char> = words[i].chars().collect();
                match size {
                    VariableSize::Byte => {
                        let value = mcs6502::extract_operand(&chars);
                        if let Some(value) = value {
                            values.push(util::lower(value));
                        } else {
                            panic!("Invalid byte initializer: {}", words[i]);
                        }
                    }
                    VariableSize::Word => {
                        let value = mcs6502::extract_operand(&chars);
                        if let Some(value) = value {
                            values.push(util::lower(value));
                            values.push(util::upper(value));
                        } else {
                            panic!("Invalid word initializer: {}", words[i]);
                        }
                    }
                }
            }
        }

        if !mcs6502::is_valid_identifier(words[1], false) {
            panic!("Invalid variable name: '{}'", words[1]);
        }

        let mut name = String::from(words[1]);
        name.push_str(":");

        // Variables are where they were declared.
        self.vars.insert(name.clone(), (self.data.len() + 2) as u16);
        for value in values.iter() {
            self.data.push(*value);
        }
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

            AddressMode::ILabel      |
            AddressMode::ILabelX     |
            AddressMode::ILabelY     |
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
                    match mode {
                        AddressMode::ILabel      |
                        AddressMode::Label       |
                        AddressMode::LabelX      |
                        AddressMode::LabelY      => {
                            self.push_three_byte(op, 0x0000u16);
                        }
                        AddressMode::ILabelX     |
                        AddressMode::ILabelY     => {
                            self.push_two_byte(op, 0x00u8);
                        }
                        _                        => ()
                    }
                } else {
                    panic!("Instruction cannot use label: 0x{:X} {} ({:?})", op, label, mode);
                }
            }

            _                     => {
                panic!("Invalid address mode {:?} for opcode: 0x{:X}", mode, op);
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
        if c == '(' {
            // Indirect, ignore this.
            c = chars.next().unwrap_or(' ');
        }

        while c.is_alphanumeric() || c == '_' {
            res.push(c);
            c = chars.next().unwrap_or(' ');
        }

        res
    }
}

#[derive(Debug)]
struct Macro {
    pub name: String,
    pub args: Vec<String>,
    pub code: Vec<String>
}

pub struct Preprocessor {
    macros:      HashMap<String, Macro>,
    aux_defined: bool
}

impl Preprocessor {
    pub fn new() -> Preprocessor {
        Preprocessor {
            macros:      HashMap::new(),
            aux_defined: false
        }
    }
    pub fn register_macro(&mut self, code: Vec<String>) {
        if code.len() > 1 {
            let mut iter = code.iter();
            let declaration = iter.next().unwrap();
            if declaration == "" {
                return;
            }

            let mut args: Vec<String> = Vec::new();
            let mut code: Vec<String> = Vec::new();

            let mut tokens = declaration.split_whitespace();

            // Skip the .macro part.
            tokens.next();
            let mut name = String::from("$");
            name.push_str(tokens.next().unwrap());

            // TODO: Validate args to be valid labels.
            while let Some(arg) = tokens.next() {
                args.push(String::from(arg));
            }

            while let Some(line) = iter.next() {
                code.push(line.trim().to_uppercase());
            }

            // Allow redefinitions.
            self.macros.insert(name.clone(), Macro { name, args, code });
        }
    }

    pub fn process(&mut self, code: Vec<String>) -> Vec<String> {
        let mut expanded = true;
        let mut iteration = 0u8;
        let mut input: Vec<String> = code;
        let mut output: Vec<String> = Vec::new();

        // Auxiliary variables.
        if !self.aux_defined {
            // No aux atm, accu backup was unnecessarily expensive.
            self.aux_defined = true;
        }

        while expanded && iteration < 4 {
            expanded = false;
            iteration += 1;

            // Auxiliary block that limits iter's lifetime.
            {
                let mut iter = input.iter().peekable();
                while let Some(line) = iter.next() {
                    let line = line.to_uppercase();

                    if line.starts_with(".MACRO ") {
                        let mut macro_code: Vec<String> = Vec::new();
                        macro_code.push(line.clone());

                        while let Some(&line) = iter.peek() {
                            if line.starts_with(" ") {
                                macro_code.push(line.clone());
                            } else {
                                break;
                            }
                            iter.next();
                        }

                        self.register_macro(macro_code);
                    } else if line.starts_with("$PUSH ") {
                        self.macro_push(&line, &mut output);
                        expanded = true;
                    } else if line.starts_with("$POP ") {
                        self.macro_pop(&line, &mut output);
                        expanded = true;
                    } else if line.starts_with("$STRING ") {
                        self.macro_string(&line, &mut output);
                        expanded = true;
                    } else if line.starts_with("$MOV ") {
                        self.macro_mov(&line, &mut output);
                        expanded = true;
                    } else if line.starts_with("$") {
                        self.expand_macro(&line, &mut output);
                        expanded = true;
                    } else {
                        output.push(line.clone());
                    }
                }
            }

            if expanded {
                input = output;
                output = Vec::new();
            }
        }

        output
    }

    fn expand_macro(&mut self, line: &str, output: &mut Vec<String>) {
        let mut tokens = line.split_whitespace();

        if let Some(name) = tokens.next() {
            if let Some(mac) = self.macros.get(name) {
                let mut mapping: HashMap<String, String> = HashMap::new();
                if mac.args.len() > 0 {
                    for i in 0..mac.args.len() {
                        if let Some(arg) = tokens.next() {
                            mapping.insert(mac.args[i].clone(), arg.to_string());
                        } else {
                            panic!("Not enough arguments for macro {}: {}", name, i);
                        }
                    }

                    for line in mac.code.iter() {
                        let tokens = line.split_whitespace();
                        let mut line: Vec<String> = Vec::new();

                        for token in tokens {
                            let token = token.to_string();
                            // TODO: Fix indexed access!
                            // TODO: Fix indirect parens!

                            let matches = mapping.contains_key(&token);
                            if matches {
                                line.push(mapping.get(&token).unwrap().clone());
                            } else {
                                line.push(token.clone());
                            }
                        }

                        let mut out_line = String::new();
                        for word in line.iter() {
                            out_line.push_str(word);
                            out_line.push_str(" ");
                        }
                        output.push(out_line.trim_right().to_string());
                    }
                } else {
                    for i in 0 .. mac.code.len() {
                        output.push(mac.code[i].clone());
                    }
                }

            } else {
                panic!("Unknown macro: {}", name);
            }
        }
    }

    fn macro_push(&mut self, line: &str, output: &mut Vec<String>) {
        let mut tokens = line.split_whitespace();
        tokens.next();

        let arguments: Vec<&str> = tokens.collect();

        if arguments.len() != 1 {
            panic!("The $push macro supports only one argument, {} given {:?}",
                   arguments.len(), arguments);
        }

        output.push(format!("LDA {}", arguments[0]));
        output.push(String::from("PHA"));
    }

    fn macro_pop(&mut self, line: &str, output: &mut Vec<String>) {
        let mut tokens = line.split_whitespace();
        tokens.next();

        let arguments: Vec<&str> = tokens.collect();

        if arguments.len() != 1 {
            panic!("The $pop macro supports only one argument, {} given {:?}",
                   arguments.len(), arguments);
        }

        output.push(String::from("PLA"));
        output.push(format!("STA {}", arguments[0]));
    }

    fn macro_string(&mut self, line: &str, output: &mut Vec<String>) {
        let mut tokens = line.split_whitespace();
        tokens.next();

        let arguments: Vec<&str> = tokens.collect();

        let mut string = String::new();
        string.push_str(arguments[1]);

        if string.find('"') == string.rfind('"') {
            for i in 2 .. arguments.len() {
                string.push_str(" ");
                if arguments[i].contains('"') {
                    string.push_str(arguments[i].trim_right_matches(|c| c != '"'));
                    break;
                } else {
                    string.push_str(arguments[i]);
                }
            }
        }

        if string.find('"') == string.rfind('"') {
            panic!("String not misses quotes: {}", string);
        }

        let mut out_string = String::from(".BYTE ");
        out_string.push_str(arguments[0]);
        for c in string.chars() {
            if c != '"' {
                out_string.push_str(" $");
                out_string.push_str(&format!("{:X}", c as u8));
            }
        }
        out_string.push_str(" $00");

        output.push(out_string);
    }

    fn macro_mov(&mut self, line: &str, output: &mut Vec<String>) {
        let mut tokens = line.split_whitespace();
        tokens.next();

        let arguments: Vec<&str> = tokens.collect();

        if arguments.len() != 2 {
            panic!("The $mov macro requires two arguments.")
        }

        output.push(format!("LDA {}", arguments[1]));
        output.push(format!("STA {}", arguments[0]));
    }
}
