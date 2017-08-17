use mems::Memory;
use util;

// TODO: Add Implied to Mcs6502 emulator.
#[derive(Debug, PartialEq)]
pub enum AddressMode {
    Implied,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,

    // Custom modes for the assembler.
    Label,
    LabelX,
    LabelY,
    ILabel,
    ILabelX,
    ILabelY,

    None
}

pub mod addr {
    use inst::mcs6502::ops;
    use inst::mcs6502::AddressMode;

    pub fn pc_offset(mode: &AddressMode) -> usize {
        match *mode {
            AddressMode::Label     |
            AddressMode::LabelX    |
            AddressMode::LabelY    |
            AddressMode::ILabel    |
            AddressMode::Absolute  |
            AddressMode::AbsoluteX |
            AddressMode::AbsoluteY |
            AddressMode::Indirect  => 3,
            AddressMode::Implied   => 1,
            AddressMode::None      => 0,
            _                      => 2,
        }
    }

    pub fn get_addr_mode(opcode: u8) -> AddressMode {
        match opcode {
            ops::ADC_IMMEDIATE   |
            ops::AND_IMMEDIATE   |
            ops::CMP_IMMEDIATE   |
            ops::CPX_IMMEDIATE   |
            ops::CPY_IMMEDIATE   |
            ops::EOR_IMMEDIATE   |
            ops::LDA_IMMEDIATE   |
            ops::LDX_IMMEDIATE   |
            ops::LDY_IMMEDIATE   |
            ops::ORA_IMMEDIATE   |
            ops::SBC_IMMEDIATE   => AddressMode::Immediate,

            ops::ADC_ZERO_PAGE   |
            ops::AND_ZERO_PAGE   |
            ops::ASL_ZERO_PAGE   |
            ops::BIT_ZERO_PAGE   |
            ops::CMP_ZERO_PAGE   |
            ops::CPX_ZERO_PAGE   |
            ops::CPY_ZERO_PAGE   |
            ops::DEC_ZERO_PAGE   |
            ops::EOR_ZERO_PAGE   |
            ops::INC_ZERO_PAGE   |
            ops::LDA_ZERO_PAGE   |
            ops::LDX_ZERO_PAGE   |
            ops::LDY_ZERO_PAGE   |
            ops::LSR_ZERO_PAGE   |
            ops::ORA_ZERO_PAGE   |
            ops::ROL_ZERO_PAGE   |
            ops::ROR_ZERO_PAGE   |
            ops::SBC_ZERO_PAGE   |
            ops::STA_ZERO_PAGE   |
            ops::STX_ZERO_PAGE   |
            ops::STY_ZERO_PAGE   => AddressMode::ZeroPage,

            ops::ADC_ZERO_PAGE_X |
            ops::AND_ZERO_PAGE_X |
            ops::ASL_ZERO_PAGE_X |
            ops::CMP_ZERO_PAGE_X |
            ops::DEC_ZERO_PAGE_X |
            ops::EOR_ZERO_PAGE_X |
            ops::INC_ZERO_PAGE_X |
            ops::LDA_ZERO_PAGE_X |
            ops::LDY_ZERO_PAGE_X |
            ops::LSR_ZERO_PAGE_X |
            ops::ORA_ZERO_PAGE_X |
            ops::ROL_ZERO_PAGE_X |
            ops::ROR_ZERO_PAGE_X |
            ops::SBC_ZERO_PAGE_X |
            ops::STA_ZERO_PAGE_X |
            ops::STY_ZERO_PAGE_X => AddressMode::ZeroPageX,

            ops::LDX_ZERO_PAGE_Y |
            ops::STX_ZERO_PAGE_Y => AddressMode::ZeroPageY,

            ops::ADC_ABSOLUTE    |
            ops::AND_ABSOLUTE    |
            ops::ASL_ABSOLUTE    |
            ops::BIT_ABSOLUTE    |
            ops::CMP_ABSOLUTE    |
            ops::CPX_ABSOLUTE    |
            ops::CPY_ABSOLUTE    |
            ops::DEC_ABSOLUTE    |
            ops::EOR_ABSOLUTE    |
            ops::INC_ABSOLUTE    |
            ops::JMP_ABSOLUTE    |
            ops::JSR_ABSOLUTE    |
            ops::LDA_ABSOLUTE    |
            ops::LDX_ABSOLUTE    |
            ops::LDY_ABSOLUTE    |
            ops::LSR_ABSOLUTE    |
            ops::ORA_ABSOLUTE    |
            ops::ROL_ABSOLUTE    |
            ops::ROR_ABSOLUTE    |
            ops::SBC_ABSOLUTE    |
            ops::STA_ABSOLUTE    |
            ops::STX_ABSOLUTE    |
            ops::STY_ABSOLUTE    => AddressMode::Absolute,

            ops::ADC_ABSOLUTE_X  |
            ops::AND_ABSOLUTE_X  |
            ops::ASL_ABSOLUTE_X  |
            ops::CMP_ABSOLUTE_X  |
            ops::DEC_ABSOLUTE_X  |
            ops::EOR_ABSOLUTE_X  |
            ops::INC_ABSOLUTE_X  |
            ops::LDA_ABSOLUTE_X  |
            ops::LDY_ABSOLUTE_X  |
            ops::LSR_ABSOLUTE_X  |
            ops::ORA_ABSOLUTE_X  |
            ops::ROL_ABSOLUTE_X  |
            ops::ROR_ABSOLUTE_X  |
            ops::SBC_ABSOLUTE_X  |
            ops::STA_ABSOLUTE_X  => AddressMode::AbsoluteX,

            ops::ADC_ABSOLUTE_Y  |
            ops::AND_ABSOLUTE_Y  |
            ops::CMP_ABSOLUTE_Y  |
            ops::EOR_ABSOLUTE_Y  |
            ops::LDA_ABSOLUTE_Y  |
            ops::LDX_ABSOLUTE_Y  |
            ops::ORA_ABSOLUTE_Y  |
            ops::SBC_ABSOLUTE_Y  |
            ops::STA_ABSOLUTE_Y  => AddressMode::AbsoluteY,

            ops::JMP_INDIRECT    => AddressMode::Indirect,

            ops::ADC_INDIRECT_X  |
            ops::AND_INDIRECT_X  |
            ops::CMP_INDIRECT_X  |
            ops::EOR_INDIRECT_X  |
            ops::LDA_INDIRECT_X  |
            ops::ORA_INDIRECT_X  |
            ops::SBC_INDIRECT_X  |
            ops::STA_INDIRECT_X  => AddressMode::IndirectX,

            ops::ADC_INDIRECT_Y  |
            ops::AND_INDIRECT_Y  |
            ops::CMP_INDIRECT_Y  |
            ops::EOR_INDIRECT_Y  |
            ops::LDA_INDIRECT_Y  |
            ops::ORA_INDIRECT_Y  |
            ops::SBC_INDIRECT_Y  |
            ops::STA_INDIRECT_Y  => AddressMode::IndirectY,

            ops::BCC_RELATIVE    |
            ops::BCS_RELATIVE    |
            ops::BEQ_RELATIVE    |
            ops::BMI_RELATIVE    |
            ops::BNE_RELATIVE    |
            ops::BPL_RELATIVE    |
            ops::BVC_RELATIVE    |
            ops::BVS_RELATIVE    => AddressMode::Relative,

            ops::ASL_ACCUMULATOR |
            ops::LSR_ACCUMULATOR |
            ops::ROL_ACCUMULATOR |
            ops::ROR_ACCUMULATOR => AddressMode::Accumulator,

            ops::BRK_IMPLIED     |
            ops::CLC_IMPLIED     |
            ops::CLD_IMPLIED     |
            ops::CLI_IMPLIED     |
            ops::CLV_IMPLIED     |
            ops::DEX_IMPLIED     |
            ops::DEY_IMPLIED     |
            ops::INX_IMPLIED     |
            ops::INY_IMPLIED     |
            ops::NOP_IMPLIED     |
            ops::PHA_IMPLIED     |
            ops::PHP_IMPLIED     |
            ops::PLA_IMPLIED     |
            ops::PLP_IMPLIED     |
            ops::RTI_IMPLIED     |
            ops::RTS_IMPLIED     |
            ops::SEC_IMPLIED     |
            ops::SED_IMPLIED     |
            ops::SEI_IMPLIED     |
            ops::TAX_IMPLIED     |
            ops::TAY_IMPLIED     |
            ops::TYA_IMPLIED     |
            ops::TSX_IMPLIED     |
            ops::TXA_IMPLIED     |
            ops::TXS_IMPLIED     => AddressMode::Implied,

            ops::custom::TOS_ABSOLUTE |
            ops::custom::PRT_ABSOLUTE => AddressMode::Absolute,

            ops::custom::VARIABLE     => AddressMode::Immediate,

            _ => AddressMode::None,
        }
    }
}

pub mod ops {
    // Add memory to accumulator with carry.
    pub const ADC_IMMEDIATE:   u8 = 0x69;
    pub const ADC_ZERO_PAGE:   u8 = 0x65;
    pub const ADC_ZERO_PAGE_X: u8 = 0x75;
    pub const ADC_ABSOLUTE:    u8 = 0x6D;
    pub const ADC_ABSOLUTE_X:  u8 = 0x7D;
    pub const ADC_ABSOLUTE_Y:  u8 = 0x79;
    pub const ADC_INDIRECT_X:  u8 = 0x61;
    pub const ADC_INDIRECT_Y:  u8 = 0x71;

    // "And" memory with accumulator.
    pub const AND_IMMEDIATE:   u8 = 0x29;
    pub const AND_ZERO_PAGE:   u8 = 0x25;
    pub const AND_ZERO_PAGE_X: u8 = 0x35;
    pub const AND_ABSOLUTE:    u8 = 0x2D;
    pub const AND_ABSOLUTE_X:  u8 = 0x3D;
    pub const AND_ABSOLUTE_Y:  u8 = 0x39;
    pub const AND_INDIRECT_X:  u8 = 0x21;
    pub const AND_INDIRECT_Y:  u8 = 0x31;

    // Shift left one bit (memory or accumulator).
    pub const ASL_ACCUMULATOR: u8 = 0x0A;
    pub const ASL_ZERO_PAGE:   u8 = 0x06;
    pub const ASL_ZERO_PAGE_X: u8 = 0x16;
    pub const ASL_ABSOLUTE:    u8 = 0x0E;
    pub const ASL_ABSOLUTE_X:  u8 = 0x1E;

    // Branch on carry clear.
    pub const BCC_RELATIVE:    u8 = 0x90;

    // Branch on carry set.
    pub const BCS_RELATIVE:    u8 = 0xB0;

    // Branch on result zero.
    pub const BEQ_RELATIVE:    u8 = 0xF0;

    // Test bits in memory with accumulator.
    pub const BIT_ZERO_PAGE:   u8 = 0x24;
    pub const BIT_ABSOLUTE:    u8 = 0x2C;

    // Branch on result minus.
    pub const BMI_RELATIVE:    u8 = 0x30;

    // Branch on result not zero.
    pub const BNE_RELATIVE:    u8 = 0xD0;

    // Branch on result plus.
    pub const BPL_RELATIVE:    u8 = 0x10;

    // Force break.
    pub const BRK_IMPLIED:     u8 = 0x00;

    // Branch on overflow clear.
    pub const BVC_RELATIVE:    u8 = 0x50;

    // Branch on overflow set.
    pub const BVS_RELATIVE:    u8 = 0x70;

    // Clear carry flag.
    pub const CLC_IMPLIED:     u8 = 0x18;

    // Clear decimal mode.
    pub const CLD_IMPLIED:     u8 = 0xD8;

    // Clear interrupt disable bit.
    pub const CLI_IMPLIED:     u8 = 0x58;

    // Clear overflow flag.
    pub const CLV_IMPLIED:     u8 = 0xB8;

    // Compare memory and accumulator.
    pub const CMP_IMMEDIATE:   u8 = 0xC9;
    pub const CMP_ZERO_PAGE:   u8 = 0xC5;
    pub const CMP_ZERO_PAGE_X: u8 = 0xD5;
    pub const CMP_ABSOLUTE:    u8 = 0xCD;
    pub const CMP_ABSOLUTE_X:  u8 = 0xDD;
    pub const CMP_ABSOLUTE_Y:  u8 = 0xD9;
    pub const CMP_INDIRECT_X:  u8 = 0xC1;
    pub const CMP_INDIRECT_Y:  u8 = 0xD1;

    // Compare memory to index X.
    pub const CPX_IMMEDIATE:   u8 = 0xE0;
    pub const CPX_ZERO_PAGE:   u8 = 0xE4;
    pub const CPX_ABSOLUTE:    u8 = 0xEC;

    // Compare memory to index Y.
    pub const CPY_IMMEDIATE:   u8 = 0xC0;
    pub const CPY_ZERO_PAGE:   u8 = 0xC4;
    pub const CPY_ABSOLUTE:    u8 = 0xCC;

    // Decrement memory by one.
    pub const DEC_ZERO_PAGE:   u8 = 0xC6;
    pub const DEC_ZERO_PAGE_X: u8 = 0xD6;
    pub const DEC_ABSOLUTE:    u8 = 0xCE;
    pub const DEC_ABSOLUTE_X:  u8 = 0xDE;

    // Decrement index X by one.
    pub const DEX_IMPLIED:     u8 = 0xCA;

    // Decrement index Y by one.
    pub const DEY_IMPLIED:     u8 = 0x88;

    // "Exclusive-Or" memory with accumulator.
    pub const EOR_IMMEDIATE:   u8 = 0x49;
    pub const EOR_ZERO_PAGE:   u8 = 0x45;
    pub const EOR_ZERO_PAGE_X: u8 = 0x55;
    pub const EOR_ABSOLUTE:    u8 = 0x4D;
    pub const EOR_ABSOLUTE_X:  u8 = 0x5D;
    pub const EOR_ABSOLUTE_Y:  u8 = 0x59;
    pub const EOR_INDIRECT_X:  u8 = 0x41;
    pub const EOR_INDIRECT_Y:  u8 = 0x51;

    // Increment memory by one.
    pub const INC_ZERO_PAGE:   u8 = 0xE6;
    pub const INC_ZERO_PAGE_X: u8 = 0xF6;
    pub const INC_ABSOLUTE:    u8 = 0xEE;
    pub const INC_ABSOLUTE_X:  u8 = 0xFE;

    // Increment index X by one.
    pub const INX_IMPLIED:     u8 = 0xE8;

    // Increment index Y by one.
    pub const INY_IMPLIED:     u8 = 0xC8;

    // Jump to new location.
    pub const JMP_ABSOLUTE:    u8 = 0x4C;
    pub const JMP_INDIRECT:    u8 = 0x6C;

    // Jump to new location saving return address.
    pub const JSR_ABSOLUTE:    u8 = 0x20;

    // Load accumulator with memory.
    pub const LDA_IMMEDIATE:   u8 = 0xA9;
    pub const LDA_ZERO_PAGE:   u8 = 0xA5;
    pub const LDA_ZERO_PAGE_X: u8 = 0xB5;
    pub const LDA_ABSOLUTE:    u8 = 0xAD;
    pub const LDA_ABSOLUTE_X:  u8 = 0xBD;
    pub const LDA_ABSOLUTE_Y:  u8 = 0xB9;
    pub const LDA_INDIRECT_X:  u8 = 0xA1;
    pub const LDA_INDIRECT_Y:  u8 = 0xB1;

    // Load index X with memory.
    pub const LDX_IMMEDIATE:   u8 = 0xA2;
    pub const LDX_ZERO_PAGE:   u8 = 0xA6;
    pub const LDX_ZERO_PAGE_Y: u8 = 0xB6;
    pub const LDX_ABSOLUTE:    u8 = 0xAE;
    pub const LDX_ABSOLUTE_Y:  u8 = 0xBE;

    // Load index Y with memory.
    pub const LDY_IMMEDIATE:   u8 = 0xA0;
    pub const LDY_ZERO_PAGE:   u8 = 0xA4;
    pub const LDY_ZERO_PAGE_X: u8 = 0xB4;
    pub const LDY_ABSOLUTE:    u8 = 0xAC;
    pub const LDY_ABSOLUTE_X:  u8 = 0xBC;

    // Shift right one bit (memory or accumulator).
    pub const LSR_ACCUMULATOR: u8 = 0x4A;
    pub const LSR_ZERO_PAGE:   u8 = 0x46;
    pub const LSR_ZERO_PAGE_X: u8 = 0x56;
    pub const LSR_ABSOLUTE:    u8 = 0x4E;
    pub const LSR_ABSOLUTE_X:  u8 = 0x5E;

    // No operation.
    pub const NOP_IMPLIED:     u8 = 0xEA;

    // "OR" memory with accumulator.
    pub const ORA_IMMEDIATE:   u8 = 0x09;
    pub const ORA_ZERO_PAGE:   u8 = 0x05;
    pub const ORA_ZERO_PAGE_X: u8 = 0x15;
    pub const ORA_ABSOLUTE:    u8 = 0x0D;
    pub const ORA_ABSOLUTE_X:  u8 = 0x1D;
    pub const ORA_ABSOLUTE_Y:  u8 = 0x19;
    pub const ORA_INDIRECT_X:  u8 = 0x01;
    pub const ORA_INDIRECT_Y:  u8 = 0x11;

    // Push accumulator on stack.
    pub const PHA_IMPLIED:     u8 = 0x48;

    // Push processor status on stack.
    pub const PHP_IMPLIED:     u8 = 0x08;

    // Pull accumulator from stack.
    pub const PLA_IMPLIED:     u8 = 0x68;

    // Pull processor status from stack.
    pub const PLP_IMPLIED:     u8 = 0x28;

    // Rotate one bit left (memory or accumulator).
    pub const ROL_ACCUMULATOR: u8 = 0x2A;
    pub const ROL_ZERO_PAGE:   u8 = 0x26;
    pub const ROL_ZERO_PAGE_X: u8 = 0x36;
    pub const ROL_ABSOLUTE:    u8 = 0x2E;
    pub const ROL_ABSOLUTE_X:  u8 = 0x3E;

    // Rotate one bit right (memory or accumulator).
    pub const ROR_ACCUMULATOR: u8 = 0x6A;
    pub const ROR_ZERO_PAGE:   u8 = 0x66;
    pub const ROR_ZERO_PAGE_X: u8 = 0x76;
    pub const ROR_ABSOLUTE:    u8 = 0x6E;
    pub const ROR_ABSOLUTE_X:  u8 = 0x7E;

    // Return from interrupt.
    pub const RTI_IMPLIED:     u8 = 0x40;

    // Return from subroutine.
    pub const RTS_IMPLIED:     u8 = 0x60;

    // Subtract memory from accumulator with borrow.
    pub const SBC_IMMEDIATE:   u8 = 0xE9;
    pub const SBC_ZERO_PAGE:   u8 = 0xE5;
    pub const SBC_ZERO_PAGE_X: u8 = 0xF5;
    pub const SBC_ABSOLUTE:    u8 = 0xED;
    pub const SBC_ABSOLUTE_X:  u8 = 0xFD;
    pub const SBC_ABSOLUTE_Y:  u8 = 0xF9;
    pub const SBC_INDIRECT_X:  u8 = 0xE1;
    pub const SBC_INDIRECT_Y:  u8 = 0xF1;

    // Set carry flag.
    pub const SEC_IMPLIED:     u8 = 0x38;

    // Set decimal mode.
    pub const SED_IMPLIED:     u8 = 0xF8;

    // Set interrupt disable status.
    pub const SEI_IMPLIED:     u8 = 0x78;

    // Store accumulator in memory.
    pub const STA_ZERO_PAGE:   u8 = 0x85;
    pub const STA_ZERO_PAGE_X: u8 = 0x95;
    pub const STA_ABSOLUTE:    u8 = 0x8D;
    pub const STA_ABSOLUTE_X:  u8 = 0x9D;
    pub const STA_ABSOLUTE_Y:  u8 = 0x99;
    pub const STA_INDIRECT_X:  u8 = 0x81;
    pub const STA_INDIRECT_Y:  u8 = 0x91;

    // Store index X in memory.
    pub const STX_ZERO_PAGE:   u8 = 0x86;
    pub const STX_ZERO_PAGE_Y: u8 = 0x96;
    pub const STX_ABSOLUTE:    u8 = 0x8E;

    // Store index Y in memory.
    pub const STY_ZERO_PAGE:   u8 = 0x84;
    pub const STY_ZERO_PAGE_X: u8 = 0x94;
    pub const STY_ABSOLUTE:    u8 = 0x8C;

    // Transfer accumulator to index X.
    pub const TAX_IMPLIED:     u8 = 0xAA;

    // Transfer accumulator to inxed Y.
    pub const TAY_IMPLIED:     u8 = 0xA8;

    // Transfer index Y to accumulator.
    pub const TYA_IMPLIED:     u8 = 0x98;

    // Transfer stack pointer to index X.
    pub const TSX_IMPLIED:     u8 = 0xBA;

    // Transfer index X to accumulator.
    pub const TXA_IMPLIED:     u8 = 0x8A;

    // Transfer index X to stack pointer.
    pub const TXS_IMPLIED:     u8 = 0x9A;

    pub mod custom {
        pub const VARIABLE:     u8 = 0xFB;
        pub const TOS_ABSOLUTE: u8 = 0xFC;
        pub const PRT_ABSOLUTE: u8 = 0xFF;
    }
}

pub fn extract_operand_u8(chars: &[char], off: u8) -> Option<u16> {
    let mut off = off as usize;

    let base;
    if chars.len() >= off + 3 && chars[off] == '$' {
        off += 1;
        base = 16;
    } else if chars.len() >= off + 2 {
        base = 10;
    } else {
        return None;
    }

    let digit1 = chars[off] as u8;
    let digit2 = chars[off + 1] as u8;
    util::u16_to_number(util::to_u16(digit1, digit2), base)
}

pub fn extract_operand_u16(chars: &[char], off: u8) -> Option<u16> {
    let mut off = off as usize;
    let base;

    if chars.len() >= off + 5 && chars[off] == '$' {
        off += 1;
        base = 16;
    } else if chars.len() >= off + 4 {
        base = 10
    } else if chars.len() >= off + 2 {
        return extract_operand_u8(chars, off as u8);
    } else {
        return None;
    }

    let digit1 = chars[off] as u8;
    let digit2 = chars[off + 1] as u8;
    let res1 = util::u16_to_number(util::to_u16(digit1, digit2), base);

    let digit1 = chars[off + 2] as u8;
    let digit2 = chars[off + 3] as u8;
    let res2 = util::u16_to_number(util::to_u16(digit1, digit2), base);

    let lower: u16;
    let upper: u16;

    match res1 {
        Some(num) => upper = num,
        None => return None
    }

    match res2 {
        Some(num) => lower = num,
        None => return None
    }

    if base == 16 {
        Some((upper << 8) + lower)
    } else {
        Some((upper * 100) + lower)
    }
}

pub fn parse_arguments(arguments: &str) -> (AddressMode, u16) {
    let mut chars: Vec<char> = arguments.chars().collect();
    chars.retain(|c: &char| *c != ' ');

    let mut addr_mode = AddressMode::None;
    let mut operand = 0u16;

    let size = chars.len();
    if size == 0 {
        return (AddressMode::Implied, 0u16);
    }

    // Indirect labels.
    if chars[0] == '(' {
        if chars[size - 1] == ')' {
            if chars[size - 2] == 'X' {
                addr_mode = AddressMode::ILabelX;
                chars[size - 3] = ')'; // Overwrite comma.
            } else {
                addr_mode = AddressMode::ILabel;
            }
        } else if chars[size - 1] == 'Y' {
            addr_mode = AddressMode::ILabelY;
        }

        let chars = util::extract_indirect_target(&chars);
        let argument: String = chars.into_iter().collect();
        if is_valid_label(&argument, false) {
            return (addr_mode, 0u16);
        }
    }

    // Indexed labels.
    if chars[size - 1] == 'X' || chars[size - 1] == 'Y' {
        let mut chars = chars.clone();
        let mut mode = AddressMode::None;

        if chars[size - 1] == 'X' {
            mode = AddressMode::LabelX;
        } else if chars[size - 1] == 'Y' {
            mode = AddressMode::LabelY;
        }

        chars.pop();
        if chars[size - 2] == ',' {
            chars.pop();
        }

        let label: String = chars.into_iter().collect();
        if is_valid_label(&label, false) {
            return (mode, 0u16);
        }
    }

    let size = chars.len();
    if size == 1 && chars[0] == 'A' {
        addr_mode = AddressMode::Accumulator;
    } else if is_valid_label(arguments, false) {
        addr_mode = AddressMode::Label;
    } else if chars[0] == '#' {
        // Implied
        let res = extract_operand_u8(&chars, 1);
        match res {
            Some(num) => {
                addr_mode = AddressMode::Immediate;
                operand = num;
            },
            None => ()
        }
    } else if chars[0] == '*' {
        // Zero page
        if size > 2 {
            let res = extract_operand_u8(&chars, 1);
            match res {
                Some(num) => {
                    addr_mode = AddressMode::ZeroPage;
                    operand = num;
                },
                None => ()
            }
        }

        if chars[size - 1] == 'X' {
            addr_mode = AddressMode::ZeroPageX;
        } else if chars[size - 1] == 'Y' {
            addr_mode = AddressMode::ZeroPageY;
        }
    } else if chars[0] == '(' {
        if chars[size - 1] == ')' {
            if chars[size - 2] == 'X' {
                addr_mode = AddressMode::IndirectX;
                chars[size - 3] = ')'; // Overwrite comma.
            } else {
                addr_mode = AddressMode::Indirect;
            }
        } else if chars[size - 1] == 'Y' {
            addr_mode = AddressMode::IndirectY;
        }

        let argument = util::extract_indirect_target(&chars);
        if argument.len() > 1 {
            let res = extract_operand_u8(&argument, 0);
            match res {
                Some(num) => {
                    operand = num;
                },
                None => addr_mode = AddressMode::None
            }
        }
    } else if chars.len() > 2 {
        // Absolute
        let res = extract_operand_u16(&chars, 0);
        match res {
            Some(num) => {
                addr_mode = AddressMode::Absolute;
                operand = num;
            },
            None => ()
        }

        if chars[size - 1] == 'X' {
            addr_mode = AddressMode::AbsoluteX;
        } else if chars[size - 1] == 'Y' {
            addr_mode = AddressMode::AbsoluteY;
        }
    }

    (addr_mode, operand)
}

pub fn name_mode_to_opcode(op: &str, mode: &AddressMode) -> u8 {
    match op {
        "ADC" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::ADC_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::ADC_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::ADC_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::ADC_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::ADC_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::ADC_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::ADC_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::ADC_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "AND" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::AND_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::AND_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::AND_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::AND_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::AND_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::AND_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::AND_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::AND_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "ASL" => {
            match *mode {
                AddressMode::Accumulator => {
                    ops::ASL_ACCUMULATOR
                }
                AddressMode::ZeroPage    => {
                    ops::ASL_ZERO_PAGE
                }
                AddressMode::ZeroPageX   => {
                    ops::ASL_ZERO_PAGE_X
                }
                AddressMode::Label       |
                AddressMode::Absolute    => {
                    ops::ASL_ABSOLUTE
                }
                AddressMode::LabelX      |
                AddressMode::AbsoluteX   => {
                    ops::ASL_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BCC" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BCC_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BCS" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BCS_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BEQ" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BEQ_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BIT" => {
            match *mode {
                AddressMode::ZeroPage => {
                    ops::BIT_ZERO_PAGE
                }
                AddressMode::Label    |
                AddressMode::Absolute => {
                    ops::BIT_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BMI" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BMI_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BNE" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BNE_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BPL" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BPL_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BRK" => {
            match *mode {
                AddressMode::Implied => {
                    ops::BRK_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BVC" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BVC_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "BVS" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Relative => {
                    ops::BVS_RELATIVE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CLC" => {
            match *mode {
                AddressMode::Implied => {
                    ops::CLC_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CLD" => {
            match *mode {
                AddressMode::Implied => {
                    ops::CLD_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CLI" => {
            match *mode {
                AddressMode::Implied => {
                    ops::CLI_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CLV" => {
            match *mode {
                AddressMode::Implied => {
                    ops::CLV_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CMP" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::CMP_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::CMP_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::CMP_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::CMP_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::CMP_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::CMP_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::CMP_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::CMP_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CPX" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::CPX_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::CPX_ZERO_PAGE
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::CPX_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "CPY" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::CPY_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::CPY_ZERO_PAGE
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::CPY_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "DEC" => {
            match *mode {
                AddressMode::ZeroPage  => {
                    ops::DEC_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::DEC_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::DEC_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::DEC_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "DEX" => {
            match *mode {
                AddressMode::Implied => {
                    ops::DEX_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "DEY" => {
            match *mode {
                AddressMode::Implied => {
                    ops::DEY_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "EOR" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::EOR_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::EOR_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::EOR_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::EOR_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::EOR_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::EOR_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::EOR_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::EOR_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "INC" => {
            match *mode {
                AddressMode::ZeroPage  => {
                    ops::INC_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::INC_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::INC_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::INC_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "INX" => {
            match *mode {
                AddressMode::Implied => {
                    ops::INX_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "INY" => {
            match *mode {
                AddressMode::Implied => {
                    ops::INY_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "JMP" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Absolute => {
                    ops::JMP_ABSOLUTE
                }
                AddressMode::ILabel   |
                AddressMode::Indirect => {
                    ops::JMP_INDIRECT
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "JSR" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Absolute => {
                    ops::JSR_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "LDA" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::LDA_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::LDA_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::LDA_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::LDA_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::LDA_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::LDA_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::LDA_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::LDA_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "LDX" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::LDX_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::LDX_ZERO_PAGE
                }
                AddressMode::ZeroPageY => {
                    ops::LDX_ZERO_PAGE_Y
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::LDX_ABSOLUTE
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::LDX_ABSOLUTE_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "LDY" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::LDY_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::LDY_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::LDY_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::LDY_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::LDY_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "LSR" => {
            match *mode {
                AddressMode::Accumulator => {
                    ops::LSR_ACCUMULATOR
                }
                AddressMode::ZeroPage    => {
                    ops::LSR_ZERO_PAGE
                }
                AddressMode::ZeroPageX   => {
                    ops::LSR_ZERO_PAGE_X
                }
                AddressMode::Label       |
                AddressMode::Absolute    => {
                    ops::LSR_ABSOLUTE
                }
                AddressMode::LabelX      |
                AddressMode::AbsoluteX   => {
                    ops::LSR_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "NOP" => {
            match *mode {
                AddressMode::Implied => {
                    ops::NOP_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "ORA" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::ORA_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::ORA_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::ORA_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::ORA_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::ORA_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::ORA_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::ORA_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::ORA_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "PHA" => {
            match *mode {
                AddressMode::Implied => {
                    ops::PHA_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "PHP" => {
            match *mode {
                AddressMode::Implied => {
                    ops::PHP_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "PLA" => {
            match *mode {
                AddressMode::Implied => {
                    ops::PLA_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "PLP" => {
            match *mode {
                AddressMode::Implied => {
                    ops::PLP_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "ROL" => {
            match *mode {
                AddressMode::Accumulator => {
                    ops::ROL_ACCUMULATOR
                }
                AddressMode::ZeroPage    => {
                    ops::ROL_ZERO_PAGE
                }
                AddressMode::ZeroPageX   => {
                    ops::ROL_ZERO_PAGE_X
                }
                AddressMode::Label       |
                AddressMode::Absolute    => {
                    ops::ROL_ABSOLUTE
                }
                AddressMode::LabelX      |
                AddressMode::AbsoluteX   => {
                    ops::ROL_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "ROR" => {
            match *mode {
                AddressMode::Accumulator => {
                    ops::ROR_ACCUMULATOR
                }
                AddressMode::ZeroPage    => {
                    ops::ROR_ZERO_PAGE
                }
                AddressMode::ZeroPageX   => {
                    ops::ROR_ZERO_PAGE_X
                }
                AddressMode::Label       |
                AddressMode::Absolute    => {
                    ops::ROR_ABSOLUTE
                }
                AddressMode::LabelX      |
                AddressMode::AbsoluteX   => {
                    ops::ROR_ABSOLUTE_X
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "RTI" => {
            match *mode {
                AddressMode::Implied => {
                    ops::RTI_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "RTS" => {
            match *mode {
                AddressMode::Implied => {
                    ops::RTS_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "SBC" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::SBC_IMMEDIATE
                }
                AddressMode::ZeroPage  => {
                    ops::SBC_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::SBC_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::SBC_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::SBC_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::SBC_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::SBC_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::SBC_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "SEC" => {
            match *mode {
                AddressMode::Implied => {
                    ops::SEC_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "SED" => {
            match *mode {
                AddressMode::Implied => {
                    ops::SED_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "SEI" => {
            match *mode {
                AddressMode::Implied => {
                    ops::SEI_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "STA" => {
            match *mode {
                AddressMode::ZeroPage  => {
                    ops::STA_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::STA_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::STA_ABSOLUTE
                }
                AddressMode::LabelX    |
                AddressMode::AbsoluteX => {
                    ops::STA_ABSOLUTE_X
                }
                AddressMode::LabelY    |
                AddressMode::AbsoluteY => {
                    ops::STA_ABSOLUTE_Y
                }
                AddressMode::ILabelX   |
                AddressMode::IndirectX => {
                    ops::STA_INDIRECT_X
                }
                AddressMode::ILabelY   |
                AddressMode::IndirectY => {
                    ops::STA_INDIRECT_Y
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "STX" => {
            match *mode {
                AddressMode::ZeroPage  => {
                    ops::STX_ZERO_PAGE
                }
                AddressMode::ZeroPageY => {
                    ops::STX_ZERO_PAGE_Y
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::STX_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "STY" => {
            match *mode {
                AddressMode::ZeroPage  => {
                    ops::STY_ZERO_PAGE
                }
                AddressMode::ZeroPageX => {
                    ops::STY_ZERO_PAGE_X
                }
                AddressMode::Label     |
                AddressMode::Absolute  => {
                    ops::STY_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TAX" => {
            match *mode {
                AddressMode::Implied => {
                    ops::TAX_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TAY" => {
            match *mode {
                AddressMode::Implied => {
                    ops::TAY_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TYA" => {
            match *mode {
                AddressMode::Implied => {
                    ops::TYA_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TSX" => {
            match *mode {
                AddressMode::Implied => {
                    ops::TXS_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TXA" => {
            match *mode {
                AddressMode::Implied => {
                    ops::TXA_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TXS" => {
            match *mode {
                AddressMode::Implied => {
                    ops::TXS_IMPLIED
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "VAR" => {
            match *mode {
                AddressMode::Immediate => {
                    ops::custom::VARIABLE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "TOS" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Absolute => {
                    ops::custom::TOS_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        "PRT" => {
            match *mode {
                AddressMode::Label    |
                AddressMode::Absolute => {
                    ops::custom::PRT_ABSOLUTE
                }
                _ => panic!("Unknown address mode for instruction {}: {:?}", op, mode)
            }
        }
        &_    => panic!("Unknown instruction: {}", op)
    }
}

pub fn is_valid_instruction(op: &str) -> bool {
    match op {
        "ADC" | "AND" | "ASL" | "BCC" | "BCS" |
        "BEQ" | "BIT" | "BMI" | "BNE" | "BPL" |
        "BRK" | "BVC" | "BVS" | "CLC" | "CLD" |
        "CLI" | "CLV" | "CMP" | "CPX" | "CPY" |
        "DEC" | "DEX" | "DEY" | "EOR" | "INC" |
        "INX" | "INY" | "JMP" | "JSR" | "LDA" |
        "LDX" | "LDY" | "LSR" | "NOP" | "ORA" |
        "PHA" | "PHP" | "PLA" | "PLP" | "ROL" |
        "ROR" | "RTI" | "RTS" | "SBC" | "SEC" |
        "SED" | "SEI" | "STA" | "STX" | "STY" |
        "TAX" | "TAY" | "TYA" | "TSX" | "TXA" |
        "TXS" | "PRT" | "TOS" | "VAR" => true,
        &_    => false
    }
}

pub fn is_valid_label(label: &str, decl: bool) -> bool {
    !is_valid_instruction(label) && util::is_valid_label(label, decl)
}

pub fn can_jump_to_label(op: u8) -> bool {
    op == ops::JMP_ABSOLUTE || op == ops::JSR_ABSOLUTE
}

pub fn can_branch_to_label(op: u8) -> bool {
    match op {
        ops::BCC_RELATIVE |
        ops::BCS_RELATIVE |
        ops::BEQ_RELATIVE |
        ops::BMI_RELATIVE |
        ops::BNE_RELATIVE |
        ops::BPL_RELATIVE |
        ops::BVC_RELATIVE |
        ops::BVS_RELATIVE => true,
        _                 => false
    }
}

pub fn can_use_variables(op: u8) -> bool {
    match op {
        ops::ADC_ABSOLUTE   |
        ops::AND_ABSOLUTE   |
        ops::ASL_ABSOLUTE   |
        ops::BIT_ABSOLUTE   |
        ops::CMP_ABSOLUTE   |
        ops::CPX_ABSOLUTE   |
        ops::CPY_ABSOLUTE   |
        ops::DEC_ABSOLUTE   |
        ops::EOR_ABSOLUTE   |
        ops::INC_ABSOLUTE   |
        ops::LDA_ABSOLUTE   |
        ops::LDX_ABSOLUTE   |
        ops::LDY_ABSOLUTE   |
        ops::LSR_ABSOLUTE   |
        ops::ORA_ABSOLUTE   |
        ops::ROL_ABSOLUTE   |
        ops::ROR_ABSOLUTE   |
        ops::SBC_ABSOLUTE   |
        ops::STA_ABSOLUTE   |
        ops::STX_ABSOLUTE   |
        ops::STY_ABSOLUTE   |
        ops::ADC_ABSOLUTE_X |
        ops::AND_ABSOLUTE_X |
        ops::ASL_ABSOLUTE_X |
        ops::CMP_ABSOLUTE_X |
        ops::DEC_ABSOLUTE_X |
        ops::EOR_ABSOLUTE_X |
        ops::INC_ABSOLUTE_X |
        ops::LDA_ABSOLUTE_X |
        ops::LDY_ABSOLUTE_X |
        ops::LSR_ABSOLUTE_X |
        ops::ORA_ABSOLUTE_X |
        ops::ROL_ABSOLUTE_X |
        ops::ROR_ABSOLUTE_X |
        ops::SBC_ABSOLUTE_X |
        ops::STA_ABSOLUTE_X |
        ops::ADC_ABSOLUTE_Y |
        ops::AND_ABSOLUTE_Y |
        ops::CMP_ABSOLUTE_Y |
        ops::EOR_ABSOLUTE_Y |
        ops::LDA_ABSOLUTE_Y |
        ops::LDX_ABSOLUTE_Y |
        ops::ORA_ABSOLUTE_Y |
        ops::SBC_ABSOLUTE_Y |
        ops::STA_ABSOLUTE_Y |
        ops::JMP_INDIRECT   |
        ops::ADC_INDIRECT_X |
        ops::AND_INDIRECT_X |
        ops::CMP_INDIRECT_X |
        ops::EOR_INDIRECT_X |
        ops::LDA_INDIRECT_X |
        ops::ORA_INDIRECT_X |
        ops::SBC_INDIRECT_X |
        ops::STA_INDIRECT_X |
        ops::ADC_INDIRECT_Y |
        ops::AND_INDIRECT_Y |
        ops::CMP_INDIRECT_Y |
        ops::EOR_INDIRECT_Y |
        ops::LDA_INDIRECT_Y |
        ops::ORA_INDIRECT_Y |
        ops::SBC_INDIRECT_Y |
        ops::STA_INDIRECT_Y => true,

        ops::custom::TOS_ABSOLUTE |
        ops::custom::PRT_ABSOLUTE => true,

        _                 => false
    }

}

pub fn op_name(opcode: u8) -> String {
    let str = match opcode {
        ops::ADC_IMMEDIATE        |
        ops::ADC_ZERO_PAGE        |
        ops::ADC_ZERO_PAGE_X      |
        ops::ADC_ABSOLUTE         |
        ops::ADC_ABSOLUTE_X       |
        ops::ADC_ABSOLUTE_Y       |
        ops::ADC_INDIRECT_X       |
        ops::ADC_INDIRECT_Y       => "ADC",

        ops::AND_IMMEDIATE        |
        ops::AND_ZERO_PAGE        |
        ops::AND_ZERO_PAGE_X      |
        ops::AND_ABSOLUTE         |
        ops::AND_ABSOLUTE_X       |
        ops::AND_ABSOLUTE_Y       |
        ops::AND_INDIRECT_X       |
        ops::AND_INDIRECT_Y       => "AND",

        ops::ASL_ACCUMULATOR      |
        ops::ASL_ZERO_PAGE        |
        ops::ASL_ZERO_PAGE_X      |
        ops::ASL_ABSOLUTE         |
        ops::ASL_ABSOLUTE_X       => "ASL",

        ops::BCC_RELATIVE         => "BCC",

        ops::BCS_RELATIVE         => "BCS",

        ops::BEQ_RELATIVE         => "BEQ",

        ops::BIT_ZERO_PAGE        |
        ops::BIT_ABSOLUTE         => "BIT",

        ops::BMI_RELATIVE         => "BMI",

        ops::BNE_RELATIVE         => "BNE",

        ops::BPL_RELATIVE         => "BPL",

        ops::BRK_IMPLIED          => "BRK",

        ops::BVC_RELATIVE         => "BVC",

        ops::BVS_RELATIVE         => "BVS",

        ops::CLC_IMPLIED          => "CLC",

        ops::CLD_IMPLIED          => "CLD",

        ops::CLI_IMPLIED          => "CLI",

        ops::CLV_IMPLIED          => "CLV",

        ops::CMP_IMMEDIATE        |
        ops::CMP_ZERO_PAGE        |
        ops::CMP_ZERO_PAGE_X      |
        ops::CMP_ABSOLUTE         |
        ops::CMP_ABSOLUTE_X       |
        ops::CMP_ABSOLUTE_Y       |
        ops::CMP_INDIRECT_X       |
        ops::CMP_INDIRECT_Y       => "CMP",

        ops::CPX_IMMEDIATE        |
        ops::CPX_ZERO_PAGE        |
        ops::CPX_ABSOLUTE         => "CPX",

        ops::CPY_IMMEDIATE        |
        ops::CPY_ZERO_PAGE        |
        ops::CPY_ABSOLUTE         => "CPY",

        ops::DEC_ZERO_PAGE        |
        ops::DEC_ZERO_PAGE_X      |
        ops::DEC_ABSOLUTE         |
        ops::DEC_ABSOLUTE_X       => "DEC",

        ops::DEX_IMPLIED          => "DEX",

        ops::DEY_IMPLIED          => "DEY",

        ops::EOR_IMMEDIATE        |
        ops::EOR_ZERO_PAGE        |
        ops::EOR_ZERO_PAGE_X      |
        ops::EOR_ABSOLUTE         |
        ops::EOR_ABSOLUTE_X       |
        ops::EOR_ABSOLUTE_Y       |
        ops::EOR_INDIRECT_X       |
        ops::EOR_INDIRECT_Y       => "EOR",

        ops::INC_ZERO_PAGE        |
        ops::INC_ZERO_PAGE_X      |
        ops::INC_ABSOLUTE         |
        ops::INC_ABSOLUTE_X       => "INC",

        ops::INX_IMPLIED          => "INX",

        ops::INY_IMPLIED          => "INY",

        ops::JMP_ABSOLUTE         |
        ops::JMP_INDIRECT         => "JMP",

        ops::JSR_ABSOLUTE         => "JSR",

        ops::LDA_IMMEDIATE        |
        ops::LDA_ZERO_PAGE        |
        ops::LDA_ZERO_PAGE_X      |
        ops::LDA_ABSOLUTE         |
        ops::LDA_ABSOLUTE_X       |
        ops::LDA_ABSOLUTE_Y       |
        ops::LDA_INDIRECT_X       |
        ops::LDA_INDIRECT_Y       => "LDA",

        ops::LDX_IMMEDIATE        |
        ops::LDX_ZERO_PAGE        |
        ops::LDX_ZERO_PAGE_Y      |
        ops::LDX_ABSOLUTE         |
        ops::LDX_ABSOLUTE_Y       => "LDX",

        ops::LDY_IMMEDIATE        |
        ops::LDY_ZERO_PAGE        |
        ops::LDY_ZERO_PAGE_X      |
        ops::LDY_ABSOLUTE         |
        ops::LDY_ABSOLUTE_X       => "LDY",

        ops::LSR_ACCUMULATOR      |
        ops::LSR_ZERO_PAGE        |
        ops::LSR_ZERO_PAGE_X      |
        ops::LSR_ABSOLUTE         |
        ops::LSR_ABSOLUTE_X       => "LSR",

        ops::NOP_IMPLIED          => "NOP",

        ops::ORA_IMMEDIATE        |
        ops::ORA_ZERO_PAGE        |
        ops::ORA_ZERO_PAGE_X      |
        ops::ORA_ABSOLUTE         |
        ops::ORA_ABSOLUTE_X       |
        ops::ORA_ABSOLUTE_Y       |
        ops::ORA_INDIRECT_X       |
        ops::ORA_INDIRECT_Y       => "ORA",

        ops::PHA_IMPLIED          => "PHA",

        ops::PHP_IMPLIED          => "PHP",

        ops::PLA_IMPLIED          => "PLA",

        ops::PLP_IMPLIED          => "PLP",

        ops::ROL_ACCUMULATOR      |
        ops::ROL_ZERO_PAGE        |
        ops::ROL_ZERO_PAGE_X      |
        ops::ROL_ABSOLUTE         |
        ops::ROL_ABSOLUTE_X       => "ROL",

        ops::ROR_ACCUMULATOR      |
        ops::ROR_ZERO_PAGE        |
        ops::ROR_ZERO_PAGE_X      |
        ops::ROR_ABSOLUTE         |
        ops::ROR_ABSOLUTE_X       => "ROR",

        ops::RTI_IMPLIED          => "RTI",

        ops::RTS_IMPLIED          => "RTS",

        ops::SBC_IMMEDIATE        |
        ops::SBC_ZERO_PAGE        |
        ops::SBC_ZERO_PAGE_X      |
        ops::SBC_ABSOLUTE         |
        ops::SBC_ABSOLUTE_X       |
        ops::SBC_ABSOLUTE_Y       |
        ops::SBC_INDIRECT_X       |
        ops::SBC_INDIRECT_Y       => "SBC",

        ops::SEC_IMPLIED          => "SEC",

        ops::SED_IMPLIED          => "SED",

        ops::SEI_IMPLIED          => "SEI",

        ops::STA_ZERO_PAGE        |
        ops::STA_ZERO_PAGE_X      |
        ops::STA_ABSOLUTE         |
        ops::STA_ABSOLUTE_X       |
        ops::STA_ABSOLUTE_Y       |
        ops::STA_INDIRECT_X       |
        ops::STA_INDIRECT_Y       => "STA",

        ops::STX_ZERO_PAGE        |
        ops::STX_ZERO_PAGE_Y      |
        ops::STX_ABSOLUTE         => "STX",

        ops::STY_ZERO_PAGE        |
        ops::STY_ZERO_PAGE_X      |
        ops::STY_ABSOLUTE         => "STY",

        ops::TAX_IMPLIED          => "TAX",

        ops::TAY_IMPLIED          => "TAY",

        ops::TYA_IMPLIED          => "TYA",

        ops::TSX_IMPLIED          => "TSX",

        ops::TXA_IMPLIED          => "TXA",

        ops::TXS_IMPLIED          => "TXS",

        ops::custom::VARIABLE     => "VAR",

        ops::custom::TOS_ABSOLUTE => "TOS",

        ops::custom::PRT_ABSOLUTE => "PRT",

        _                         => "UNKNOWN"
    };

    String::from(str)
}

pub fn addr_mode_to_operand(mode: &AddressMode, op8: u8, op16: u16) -> String {
    match *mode {
        AddressMode::Immediate   => format!("#${:02X}", op8),
        AddressMode::Absolute    => format!("${:04X}", op16),
        AddressMode::ZeroPage    => format!("*${:02X}", op8),
        AddressMode::AbsoluteX   => format!("${:04X}, X", op16),
        AddressMode::ZeroPageX   => format!("*${:02X}, X", op8),
        AddressMode::AbsoluteY   => format!("${:04X}, Y", op16),
        AddressMode::ZeroPageY   => format!("*${:02X}, Y", op8),
        AddressMode::Indirect    => format!("(${:04X})", op16),
        AddressMode::IndirectX   => format!("(${:02X}, X)", op8),
        AddressMode::IndirectY   => format!("(${:02X}), Y", op8),
        AddressMode::Relative    => format!("${:02X}", op8),
        AddressMode::Accumulator => String::from("A"),
        _                        => String::from("")
    }
}

pub fn op_to_str(cart: &Memory, idx: &mut usize) -> String {
    let opcode = cart.read_u8(*idx);
    let operand_u8;
    let operand_u16;

    if *idx + 1 < cart.size() {
        operand_u8 = cart.read_u8(*idx + 1);
    } else {
        operand_u8 = 0;
    }

    if *idx + 2 < cart.size() {
        operand_u16 = cart.read_u16(*idx + 1);
    } else {
        operand_u16 = 0;
    }
    let addr_mode = addr::get_addr_mode(opcode);

    let mut off = addr::pc_offset(&addr_mode);
    if off == 0 {
        off = 1;
    }
    *idx = idx.wrapping_add(off);

    let arg = addr_mode_to_operand(&addr_mode, operand_u8, operand_u16);
    let name = op_name(opcode);

    if arg.is_empty() {
        format!("{}", name)
    } else {
        format!("{} {}", name, arg)
    }
}

#[cfg(test)]
pub mod tests {
    use inst::mcs6502::*;

    #[test]
    fn argument_parsing_implied() {
        let args_good = String::from("");

        let (addr_mode, operand) = parse_arguments(&args_good);
        assert_eq!(addr_mode, AddressMode::Implied);
        assert_eq!(operand, 0);
    }

    #[test]
    fn argument_parsing_accumulator() {
        let args_good = String::from("A");

        let (addr_mode, operand) = parse_arguments(&args_good);
        assert_eq!(addr_mode, AddressMode::Accumulator);
        assert_eq!(operand, 0);
    }

    #[test]
    fn argument_parsing_immediate() {
        let args_hex = String::from("#$5A");
        let args_dec = String::from("#49");
        let args_bad = String::from("Hello!");
        let args_hex_small = String::from("#$5a");

        let (addr_mode, operand) = parse_arguments(&args_hex);
        assert_eq!(addr_mode, AddressMode::Immediate);
        assert_eq!(operand, 0x5A);

        let (addr_mode, operand) = parse_arguments(&args_dec);
        assert_eq!(addr_mode, AddressMode::Immediate);
        assert_eq!(operand, 49);

        let (addr_mode, operand) = parse_arguments(&args_bad);
        assert_eq!(addr_mode, AddressMode::None);
        assert_eq!(operand, 0);

        let (addr_mode, operand) = parse_arguments(&args_hex_small);
        assert_eq!(addr_mode, AddressMode::Immediate);
        assert_eq!(operand, 0x5A);
    }

    #[test]
    fn argument_parsing_zero_page() {
        let args_good1 = String::from("*$A3");
        let args_good2 = String::from("*63");
        let args_x = String::from("*$C9, X");
        let args_y = String::from("*17, Y");

        let (addr_mode, operand) = parse_arguments(&args_good1);
        assert_eq!(addr_mode, AddressMode::ZeroPage);
        assert_eq!(operand, 0xA3);

        let (addr_mode, operand) = parse_arguments(&args_good2);
        assert_eq!(addr_mode, AddressMode::ZeroPage);
        assert_eq!(operand, 63);

        let (addr_mode, operand) = parse_arguments(&args_x);
        assert_eq!(addr_mode, AddressMode::ZeroPageX);
        assert_eq!(operand, 0xC9);

        let (addr_mode, operand) = parse_arguments(&args_y);
        assert_eq!(addr_mode, AddressMode::ZeroPageY);
        assert_eq!(operand, 17);
    }

    #[test]
    fn argument_parsing_absolute() {
        let args_good1 = String::from("$12A3");
        let args_good2 = String::from("4321"); // TODO: Max 64k?
        let args_x = String::from("$32C9, X");
        let args_y = String::from("1017, Y");

        let (addr_mode, operand) = parse_arguments(&args_good1);
        assert_eq!(addr_mode, AddressMode::Absolute);
        assert_eq!(operand, 0x12A3);

        let (addr_mode, operand) = parse_arguments(&args_good2);
        assert_eq!(addr_mode, AddressMode::Absolute);
        assert_eq!(operand, 4321);

        let (addr_mode, operand) = parse_arguments(&args_x);
        assert_eq!(addr_mode, AddressMode::AbsoluteX);
        assert_eq!(operand, 0x32C9);

        let (addr_mode, operand) = parse_arguments(&args_y);
        assert_eq!(addr_mode, AddressMode::AbsoluteY);
        assert_eq!(operand, 1017);
    }
}
