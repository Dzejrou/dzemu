use cpus::Cpu;
use mems::Memory;
use regs::Registers;

// Add memory to accumulator with carry.
const OP_ADC_IMMEDIATE = 0x69;
const OP_ADC_ZERO_PAGE = 0x65;
const OP_ADC_ZERO_PAGE_X = 0x75;
const OP_ADC_ABSOLUTE = 0x6D;
const OP_ADC_ABSOLUTE_X = 0x7D;
const OP_ADC_ABSOLUTE_Y = 0x79;
const OP_ADC_INDIRECT_X = 0x61;
const OP_ADC_INDIRECT_Y = 0x71;

// "And" memory with accumulator.
const OP_AND_IMMEDIATE = 0x29;
const OP_AND_ZERO_PAGE = 0x25;
const OP_AND_ZERO_PAGE_X = 0x35;
const OP_AND_ABSOLUTE = 0x2D;
const OP_AND_ABSOLUTE_X = 0x3D;
const OP_AND_ABSOLUTE_Y = 0x39;
const OP_AND_INDIRECT_X = 0x21;
const OP_AND_INDIRECT_Y = 0x31;

// Shift left one bit (memory or accumulator).
const OP_ASL_ACCUMULATOR = 0x0A;
const OP_ASL_ZERO_PAGE = 0x06;
const OP_ASL_ZERO_PAGE_X = 0x16;
const OP_ASL_ABSOLUTE = 0x0E;
const OP_ASL_ABSOLUTE_X = 0x1E;

// Branch on carry clear.
const OP_BCC_RELATIVE = 0x90;

// Branch on carry set.
const OP_BCS_RELATIVE = 0xB0;

// Branch on result zero.
const OP_BEQ_RELATIVE = 0xF0;

// Test bits in memory with accumulator.
const OP_BIT_ZERO_PAGE = 0x24;
const OP_BIT_ABSOLUTE = 0x2C;

// Branch on result minus.
const OP_BMI_RELATIVE = 0x30;

// Branch on result not zero.
const OP_BNE_RELATIVE = 0xD0;

// Branch on result plus.
const OP_BPL_RELATIVE = 0x10;

// Force break.
const OP_BRK_IMPLIED = 0x00;

// Branch on overflow clear.
const OP_BVC_RELATIVE = 0x50;

// Branch on overflow set.
const OP_BVS_RELATIVE = 0x70;

// Clear carry flag.
const OP_CLC_IMPLIED = 0x18;

// Clear decimal mode.
const OP_CLD_IMPLIED = 0xD8;

// Clear interrupt disable bit.
const OP_CLI_IMPLIED = 0x58;

// Clear overflow flag.
const OP_CLV_IMPLIED = 0xB8;

// Compare memory and accumulator.
const OP_CMP_IMMEDIATE = 0xC9;
const OP_CMP_ZERO_PAGE = 0xC5;
const OP_CMP_ZERO_PAGE_X = 0xD5;
const OP_CMP_ABSOLUTE = 0xCD;
const OP_CMP_ABSOLUTE_X = 0xDD;
const OP_CMP_ABSOLUTE_Y = 0xD9;
const OP_CMP_INDIRECT_X = 0xC1;
const OP_CMP_INDIRECT_Y = 0xD1;

// Compare memory to index X.
const OP_CPX_IMMEDIATE = 0xE0;
const OP_CPX_ZERO_PAGE = 0xE4;
const OP_CPX_ABSOLUTE = 0xEC;

// Compare memory to index Y.
const OP_CPY_IMMEDIATE = 0xC0;
const OP_CPY_ZERO_PAGE = 0xC4;
const OP_CPY_ABSOLUTE = 0xCC;

// Decrement memory by one.
const OP_DEC_ZERO_PAGE = 0xC6;
const OP_DEC_ZERO_PAGE_X = 0xD6;
const OP_DEC_ABSOLUTE = 0xCE;
const OP_DEC_ABSOLUTE_X = 0xDE;

// Decrement index X by one.
const OP_DEX_IMPLIED = 0xCA;

// Decrement index Y by one.
const OP_DEY_IMPLIED = 0x88;

// "Exclusive-Or" memory with accumulator.
const OP_EOR_IMMEDIATE = 0x49;
const OP_EOR_ZERO_PAGE = 0x45;
const OP_EOR_ZERO_PAGE_X = 0x55;
const OP_EOR_ABSOLUTE = 0x4D;
const OP_EOR_ABSOLUTE_X = 0x5D;
const OP_EOR_ABSOLUTE_Y = 0x59;
const OP_EOR_INDIRECT_X = 0x41;
const OP_EOR_INDIRECT_Y = 0x51;

// Increment memory by one.
const OP_INC_ZERO_PAGE = 0xE6;
const OP_INC_ZERO_PAGE_X = 0xF6;
const OP_INC_ABSOLUTE = 0xEE;
const OP_INC_ABSOLUTE_X = 0xFE;

// Increment index X by one.
const OP_INX_IMPLIED = 0xE8;

// Increment index Y by one.
const OP_INY_IMPLIED = 0xC8;

// Jump to new location.
const OP_JMP_ABSOLUTE = 0x4C;
const OP_JMP_INDIRECT = 0x6C;

// Jump to new location saving return address.
const OP_JSR_ABSOLUTE = 0x20;

// Load accumulator with memory.
const OP_LDA_IMMEDIATE = 0xA9;
const OP_LDA_ZERO_PAGE = 0xA5;
const OP_LDA_ZERO_PAGE_X = 0xB5;
const OP_LDA_ABSOLUTE = 0xAD;
const OP_LDA_ABSOLUTE_X = 0xBD;
const OP_LDA_ABSOLUTE_Y = 0xB9;
const OP_LDA_INDIRECT_X = 0xA1;
const OP_LDA_INDIRECT_Y = 0xB1;

// Load index X with memory.
const OP_LDX_IMMEDIATE = 0xA2;
const OP_LDX_ZERO_PAGE = 0xA6;
const OP_LDX_ZERO_PAGE_Y = 0xB6;
const OP_LDX_ABSOLUTE = 0xAE;
const OP_LDX_ABSOLUTE_Y = 0xBE;

// Load index Y with memory.
const OP_LDY_IMMEDIATE = 0xA0;
const OP_LDY_ZERO_PAGE = 0xA4;
const OP_LDY_ZERO_PAGE_X = 0xB4;
const OP_LDY_ABSOLUTE = 0xAC;
const OP_LDY_ABSOLUTE_X = 0xBC;

// Shift right one bit (memory or accumulator).
const OP_LSR_ACCUMULATOR = 0x4A;
const OP_LSR_ZERO_PAGE = 0x46;
const OP_LSR_ZERO_PAGE_X = 0x56;
const OP_LSR_ABSOLUTE = 0x4E;
const OP_LSR_ABSOLUTE_X = 0x5E;

// No operation.
const OP_NOP_IMPLIED = 0xEA;

// "OR" memory with accumulator.
const OP_ORA_IMMEDIATE = 0x09;
const OP_ORA_ZERO_PAGE = 0x05;
const OP_ORA_ZERO_PAGE_X = 0x15;
const OP_ORA_ABSOLUTE = 0x0D;
const OP_ORA_ABSOLUTE_X = 0x1D;
const OP_ORA_ABSOLUTE_Y = 0x19;
const OP_ORA_INDIRECT_X = 0x01;
const OP_ORA_INDIRECT_Y = 0x11;

// Push accumulator on stack.
const OP_PHA_IMPLIED = 0x48;

// Push processor status on stack.
const OP_PHP_IMPLIED = 0x08;

// Pull accumulator from stack.
const OP_PLA_IMPLIED = 0x68;

// Pull processor status from stack.
const OP_PLP_IMPLIED = 0x28;

// Rotate one bit left (memory or accumulator).
const OP_ROL_ACCUMULATOR = 0x2A;
const OP_ROL_ZERO_PAGE = 0x26;
const OP_ROL_ZERO_PAGE_X = 0x36;
const OP_ROL_ABSOLUTE = 0x2E;
const OP_ROL_ABSOLUTE_X = 0x3E;

// Rotate one bit right (memory or accumulator).
const OP_ROR_ACCUMULATOR = 0x6A;
const OP_ROR_ZERO_PAGE = 0x66;
const OP_ROR_ZERO_PAGE_X = 0x76;
const OP_ROR_ABSOLUTE = 0x6E;
const OP_ROR_ABSOLUTE_X = 0x7E;

// Return from interrupt.
const OP_RTI_IMPLIED = 0x40;

// Return from subroutine.
const OP_RTS_IMPLIED = 0x60;

// Subtract memory from accumulator with borrow.
const OP_SBC_IMMEDIATE = 0xE9;
const OP_SBC_ZERO_PAGE = 0xE5;
const OP_SBC_ZERO_PAGE_X = 0xF5;
const OP_SBC_ABSOLUTE = 0xED;
const OP_SBC_ABSOLUTE_X = 0xFD;
const OP_SBC_ABSOLUTE_Y = 0xF9;
const OP_SBC_INDIRECT_X = 0xE1;
const OP_SBC_INDIRECT_Y = 0xF1;

// Set carry flag.
const OP_SEC_IMPLIED = 0x38;

// Set decimal mode.
const OP_SED_IMPLIED = 0xF8;

// Set interrupt disable status.
const OP_SEI_IMPLIED = 0x78;

// Store accumulator in memory.
const OP_STA_ZERO_PAGE = 0x85;
const OP_STA_ZERO_PAGE_X = 0x95;
const OP_STA_ABSOLUTE = 0x8D;
const OP_STA_ABSOLUTE_X = 0x9D;
const OP_STA_ABSOLUTE_Y = 0x99;
const OP_STA_INDIRECT_X = 0x81;
const OP_STA_INDIRECT_Y = 0x91;

// Store index X in memory.
const OP_STX_ZERO_PAGE = 0x86;
const OP_STX_ZERO_PAGE_Y = 0x96;
const OP_STX_ABSOLUTE = 0x8E;

// Store index Y in memory.
const OP_STY_ZERO_PAGE = 0x84;
const OP_STY_ZERO_PAGE_X = 0x94;
const OP_STY_ABSOLUTE = 0x8C;

// Transfer accumulator to index X.
const OP_TAX_IMPLIED = 0xAA;

// Transfer accumulator to inxed Y.
const OP_TAY_IMPLIED = 0xA8;

// Transfer index Y to accumulator.
const OP_TYA_IMPLIED = 0x98;

// Transfer stack pointer to index X.
const OP_TSX_IMPLIED = 0xBA;

// Transfer index X to accumulator.
const OP_TXA_IMPLIED = 0x8A;

// Transfer index X to stack pointer.
const OP_TXS_IMPLIED = 0x9A;

enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    IndirectX,
    IndirectY,
    Relative,
    Accumulator,
    None
}

pub struct Mos6507<M: Memory, R: Registers> {
    ram: M,
    regs: R,
    pc: u16,
    addr_mode: AddressMode,
}

impl<M: Memory, R: Registers> Cpu<M, R> for Mos6507<M, R> {
    fn memory(&self) -> &M {
        &self.ram
    }

    fn registers(&self) -> &R {
        &self.regs
    }

    fn run(&mut self, cart : &Memory) {
        // TODO: PC initial value.
        loop {
            self.execute(cart);
            self.pc += self.pc_offset();
        }
    }
}

impl<M: Memory, R: Registers> Mos6507<M, R> {
    pub fn new(ram: M, regs: R) -> Mos6507<M, R> {
        Mos6507 {
            ram,
            regs,
            pc: 0u16,
            addr_mode: AddressMode::None
        }
    }

    fn execute(&mut self, cart: &Memory) {
    
    }

    fn pc_offset(&self) -> u16 {
        match self.addr_mode {
            AddressMode::Absolute  |
            AddressMode::AbsoluteX |
            AddressMode::AbsoluteY => 3,
            AddressMode::None      => 1,
            _                      => 2,
        }
    }

    fn get_addr_mode(&self, opcode: u8) -> AddressMode {
        match opcode {
            OP_ADC_IMMEDIATE |
            OP_AND_IMMEDIATE |
            OP_CMP_IMMEDIATE |
            OP_CPX_IMMEDIATE |
            OP_CPY_IMMEDIATE |
            OP_EOR_IMMEDIATE |
            OP_LDA_IMMEDIATE |
            OP_LDX_IMMEDIATE |
            OP_LDY_IMMEDIATE |
            OP_ORA_IMMEDIATE |
            OP_SBC_IMMEDIATE => AddressMode::Immediate,

            OP_ADC_ZERO_PAGE |
            OP_AND_ZERO_PAGE |
            OP_ASL_ZERO_PAGE |
            OP_BIT_ZERO_PAGE |
            OP_CMP_ZERO_PAGE |
            OP_CPX_ZERO_PAGE |
            OP_CPY_ZERO_PAGE |
            OP_DEC_ZERO_PAGE |
            OP_EOR_ZERO_PAGE |
            OP_INC_ZERO_PAGE |
            OP_LDA_ZERO_PAGE |
            OP_LDX_ZERO_PAGE |
            OP_LDY_ZERO_PAGE |
            OP_LSR_ZERO_PAGE |
            OP_ORA_ZERO_PAGE |
            OP_ROL_ZERO_PAGE |
            OP_ROR_ZERO_PAGE |
            OP_SBC_ZERO_PAGE |
            OP_STA_ZERO_PAGE |
            OP_STX_ZERO_PAGE |
            OP_STY_ZERO_PAGE => AddressMode::ZeroPage,

            OP_ADC_ZERO_PAGE_X |
            OP_AND_ZERO_PAGE_X |
            OP_ASL_ZERO_PAGE_X |
            OP_CMP_ZERO_PAGE_X |
            OP_DEC_ZERO_PAGE_X |
            OP_EOR_ZERO_PAGE_X |
            OP_INC_ZERO_PAGE_X |
            OP_LDA_ZERO_PAGE_X |
            OP_LDY_ZERO_PAGE_X |
            OP_LSR_ZERO_PAGE_X |
            OP_ORA_ZERO_PAGE_X |
            OP_ROL_ZERO_PAGE_X |
            OP_ROR_ZERO_PAGE_X |
            OP_SBC_ZERO_PAGE_X |
            OP_STA_ZERO_PAGE_X |
            OP_STY_ZERO_PAGE_X => AddressMode::ZeroPageX,

            OP_LDX_ZERO_PAGE_Y |
            OP_STX_ZERO_PAGE_Y => AddressMode::ZeroPageY,

            OP_ADC_ABSOLUTE |
            OP_AND_ABSOLUTE |
            OP_ASL_ABSOLUTE |
            OP_BIT_ABSOLUTE |
            OP_CMP_ABSOLUTE |
            OP_CPX_ABSOLUTE |
            OP_CPY_ABSOLUTE |
            OP_DEC_ABSOLUTE |
            OP_EOR_ABSOLUTE |
            OP_INC_ABSOLUTE |
            OP_JMP_ABSOLUTE |
            OP_JSR_ABSOLUTE |
            OP_LDA_ABSOLUTE |
            OP_LDX_ABSOLUTE |
            OP_LDY_ABSOLUTE |
            OP_LSR_ABSOLUTE |
            OP_ORA_ABSOLUTE |
            OP_ROL_ABSOLUTE |
            OP_ROR_ABSOLUTE |
            OP_SBC_ABSOLUTE |
            OP_STA_ABSOLUTE |
            OP_STX_ABSOLUTE |
            OP_STY_ABSOLUTE => AddressMode::Absolute,

            OP_ADC_ABSOLUTE_X |
            OP_AND_ABSOLUTE_X |
            OP_ASL_ABSOLUTE_X |
            OP_CMP_ABSOLUTE_X |
            OP_DEC_ABSOLUTE_X |
            OP_EOR_ABSOLUTE_X |
            OP_INC_ABSOLUTE_X |
            OP_LDA_ABSOLUTE_X |
            OP_LDY_ABSOLUTE_X |
            OP_LSR_ABSOLUTE_X |
            OP_ORA_ABSOLUTE_X |
            OP_ROL_ABSOLUTE_X |
            OP_ROR_ABSOLUTE_X |
            OP_SBC_ABSOLUTE_X |
            OP_STA_ABSOLUTE_X => AddressMode::AbsoluteX,

            OP_ADC_ABSOLUTE_Y |
            OP_AND_ABSOLUTE_Y |
            OP_CMP_ABSOLUTE_Y |
            OP_EOR_ABSOLUTE_Y |
            OP_LDA_ABSOLUTE_Y |
            OP_LDX_ABSOLUTE_Y |
            OP_ORA_ABSOLUTE_Y |
            OP_SBC_ABSOLUTE_Y |
            OP_STA_ABSOLUTE_Y => AddressMode::AbsoluteY,

            OP_ADC_INDIRECT_X |
            OP_AND_INDIRECT_X |
            OP_CMP_INDIRECT_X |
            OP_EOR_INDIRECT_X |
            OP_LDA_INDIRECT_X |
            OP_ORA_INDIRECT_X |
            OP_SBC_INDIRECT_X |
            OP_STA_INDIRECT_X => AddressMode::IndirectX,

            OP_ADC_INDIRECT_Y |
            OP_AND_INDIRECT_Y |
            OP_CMP_INDIRECT_Y |
            OP_EOR_INDIRECT_Y |
            OP_LDA_INDIRECT_Y |
            OP_ORA_INDIRECT_Y |
            OP_SBC_INDIRECT_Y |
            OP_STA_INDIRECT_Y => AddressMode::IndirectY,

            OP_BCC_RELATIVE |
            OP_BCS_RELATIVE |
            OP_BEQ_RELATIVE |
            OP_BMI_RELATIVE |
            OP_BNE_RELATIVE |
            OP_BPL_RELATIVE |
            OP_BVS_RELATIVE => AddressMode::Relative,

            OP_ASL_ACCUMULATOR |
            OP_LSR_ACCUMULATOR |
            OP_ROL_ACCUMULATOR |
            OP_ROR_ACCUMULATOR => AddressMode::Accumulator,

            _ => AddressMode::None,
        }
    }
}
