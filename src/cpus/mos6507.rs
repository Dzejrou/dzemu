use cpus::Cpu;
use mems::Memory;
use regs::Registers;

// Add memory to accumulator with carry.
const OP_ADC_IMMEDIATE:   u8 = 0x69;
const OP_ADC_ZERO_PAGE:   u8 = 0x65;
const OP_ADC_ZERO_PAGE_X: u8 = 0x75;
const OP_ADC_ABSOLUTE:    u8 = 0x6D;
const OP_ADC_ABSOLUTE_X:  u8 = 0x7D;
const OP_ADC_ABSOLUTE_Y:  u8 = 0x79;
const OP_ADC_INDIRECT_X:  u8 = 0x61;
const OP_ADC_INDIRECT_Y:  u8 = 0x71;

// "And" memory with accumulator.
const OP_AND_IMMEDIATE:   u8 = 0x29;
const OP_AND_ZERO_PAGE:   u8 = 0x25;
const OP_AND_ZERO_PAGE_X: u8 = 0x35;
const OP_AND_ABSOLUTE:    u8 = 0x2D;
const OP_AND_ABSOLUTE_X:  u8 = 0x3D;
const OP_AND_ABSOLUTE_Y:  u8 = 0x39;
const OP_AND_INDIRECT_X:  u8 = 0x21;
const OP_AND_INDIRECT_Y:  u8 = 0x31;

// Shift left one bit (memory or accumulator).
const OP_ASL_ACCUMULATOR: u8 = 0x0A;
const OP_ASL_ZERO_PAGE:   u8 = 0x06;
const OP_ASL_ZERO_PAGE_X: u8 = 0x16;
const OP_ASL_ABSOLUTE:    u8 = 0x0E;
const OP_ASL_ABSOLUTE_X:  u8 = 0x1E;

// Branch on carry clear.
const OP_BCC_RELATIVE:    u8 = 0x90;

// Branch on carry set.
const OP_BCS_RELATIVE:    u8 = 0xB0;

// Branch on result zero.
const OP_BEQ_RELATIVE:    u8 = 0xF0;

// Test bits in memory with accumulator.
const OP_BIT_ZERO_PAGE:   u8 = 0x24;
const OP_BIT_ABSOLUTE:    u8 = 0x2C;

// Branch on result minus.
const OP_BMI_RELATIVE:    u8 = 0x30;

// Branch on result not zero.
const OP_BNE_RELATIVE:    u8 = 0xD0;

// Branch on result plus.
const OP_BPL_RELATIVE:    u8 = 0x10;

// Force break.
const OP_BRK_IMPLIED:     u8 = 0x00;

// Branch on overflow clear.
const OP_BVC_RELATIVE:    u8 = 0x50;

// Branch on overflow set.
const OP_BVS_RELATIVE:    u8 = 0x70;

// Clear carry flag.
const OP_CLC_IMPLIED:     u8 = 0x18;

// Clear decimal mode.
const OP_CLD_IMPLIED:     u8 = 0xD8;

// Clear interrupt disable bit.
const OP_CLI_IMPLIED:     u8 = 0x58;

// Clear overflow flag.
const OP_CLV_IMPLIED:     u8 = 0xB8;

// Compare memory and accumulator.
const OP_CMP_IMMEDIATE:   u8 = 0xC9;
const OP_CMP_ZERO_PAGE:   u8 = 0xC5;
const OP_CMP_ZERO_PAGE_X: u8 = 0xD5;
const OP_CMP_ABSOLUTE:    u8 = 0xCD;
const OP_CMP_ABSOLUTE_X:  u8 = 0xDD;
const OP_CMP_ABSOLUTE_Y:  u8 = 0xD9;
const OP_CMP_INDIRECT_X:  u8 = 0xC1;
const OP_CMP_INDIRECT_Y:  u8 = 0xD1;

// Compare memory to index X.
const OP_CPX_IMMEDIATE:   u8 = 0xE0;
const OP_CPX_ZERO_PAGE:   u8 = 0xE4;
const OP_CPX_ABSOLUTE:    u8 = 0xEC;

// Compare memory to index Y.
const OP_CPY_IMMEDIATE:   u8 = 0xC0;
const OP_CPY_ZERO_PAGE:   u8 = 0xC4;
const OP_CPY_ABSOLUTE:    u8 = 0xCC;

// Decrement memory by one.
const OP_DEC_ZERO_PAGE:   u8 = 0xC6;
const OP_DEC_ZERO_PAGE_X: u8 = 0xD6;
const OP_DEC_ABSOLUTE:    u8 = 0xCE;
const OP_DEC_ABSOLUTE_X:  u8 = 0xDE;

// Decrement index X by one.
const OP_DEX_IMPLIED:     u8 = 0xCA;

// Decrement index Y by one.
const OP_DEY_IMPLIED:     u8 = 0x88;

// "Exclusive-Or" memory with accumulator.
const OP_EOR_IMMEDIATE:   u8 = 0x49;
const OP_EOR_ZERO_PAGE:   u8 = 0x45;
const OP_EOR_ZERO_PAGE_X: u8 = 0x55;
const OP_EOR_ABSOLUTE:    u8 = 0x4D;
const OP_EOR_ABSOLUTE_X:  u8 = 0x5D;
const OP_EOR_ABSOLUTE_Y:  u8 = 0x59;
const OP_EOR_INDIRECT_X:  u8 = 0x41;
const OP_EOR_INDIRECT_Y:  u8 = 0x51;

// Increment memory by one.
const OP_INC_ZERO_PAGE:   u8 = 0xE6;
const OP_INC_ZERO_PAGE_X: u8 = 0xF6;
const OP_INC_ABSOLUTE:    u8 = 0xEE;
const OP_INC_ABSOLUTE_X:  u8 = 0xFE;

// Increment index X by one.
const OP_INX_IMPLIED:     u8 = 0xE8;

// Increment index Y by one.
const OP_INY_IMPLIED:     u8 = 0xC8;

// Jump to new location.
const OP_JMP_ABSOLUTE:    u8 = 0x4C;
const OP_JMP_INDIRECT:    u8 = 0x6C;

// Jump to new location saving return address.
const OP_JSR_ABSOLUTE:    u8 = 0x20;

// Load accumulator with memory.
const OP_LDA_IMMEDIATE:   u8 = 0xA9;
const OP_LDA_ZERO_PAGE:   u8 = 0xA5;
const OP_LDA_ZERO_PAGE_X: u8 = 0xB5;
const OP_LDA_ABSOLUTE:    u8 = 0xAD;
const OP_LDA_ABSOLUTE_X:  u8 = 0xBD;
const OP_LDA_ABSOLUTE_Y:  u8 = 0xB9;
const OP_LDA_INDIRECT_X:  u8 = 0xA1;
const OP_LDA_INDIRECT_Y:  u8 = 0xB1;

// Load index X with memory.
const OP_LDX_IMMEDIATE:   u8 = 0xA2;
const OP_LDX_ZERO_PAGE:   u8 = 0xA6;
const OP_LDX_ZERO_PAGE_Y: u8 = 0xB6;
const OP_LDX_ABSOLUTE:    u8 = 0xAE;
const OP_LDX_ABSOLUTE_Y:  u8 = 0xBE;

// Load index Y with memory.
const OP_LDY_IMMEDIATE:   u8 = 0xA0;
const OP_LDY_ZERO_PAGE:   u8 = 0xA4;
const OP_LDY_ZERO_PAGE_X: u8 = 0xB4;
const OP_LDY_ABSOLUTE:    u8 = 0xAC;
const OP_LDY_ABSOLUTE_X:  u8 = 0xBC;

// Shift right one bit (memory or accumulator).
const OP_LSR_ACCUMULATOR: u8 = 0x4A;
const OP_LSR_ZERO_PAGE:   u8 = 0x46;
const OP_LSR_ZERO_PAGE_X: u8 = 0x56;
const OP_LSR_ABSOLUTE:    u8 = 0x4E;
const OP_LSR_ABSOLUTE_X:  u8 = 0x5E;

// No operation.
const OP_NOP_IMPLIED:     u8 = 0xEA;

// "OR" memory with accumulator.
const OP_ORA_IMMEDIATE:   u8 = 0x09;
const OP_ORA_ZERO_PAGE:   u8 = 0x05;
const OP_ORA_ZERO_PAGE_X: u8 = 0x15;
const OP_ORA_ABSOLUTE:    u8 = 0x0D;
const OP_ORA_ABSOLUTE_X:  u8 = 0x1D;
const OP_ORA_ABSOLUTE_Y:  u8 = 0x19;
const OP_ORA_INDIRECT_X:  u8 = 0x01;
const OP_ORA_INDIRECT_Y:  u8 = 0x11;

// Push accumulator on stack.
const OP_PHA_IMPLIED:     u8 = 0x48;

// Push processor status on stack.
const OP_PHP_IMPLIED:     u8 = 0x08;

// Pull accumulator from stack.
const OP_PLA_IMPLIED:     u8 = 0x68;

// Pull processor status from stack.
const OP_PLP_IMPLIED:     u8 = 0x28;

// Rotate one bit left (memory or accumulator).
const OP_ROL_ACCUMULATOR: u8 = 0x2A;
const OP_ROL_ZERO_PAGE:   u8 = 0x26;
const OP_ROL_ZERO_PAGE_X: u8 = 0x36;
const OP_ROL_ABSOLUTE:    u8 = 0x2E;
const OP_ROL_ABSOLUTE_X:  u8 = 0x3E;

// Rotate one bit right (memory or accumulator).
const OP_ROR_ACCUMULATOR: u8 = 0x6A;
const OP_ROR_ZERO_PAGE:   u8 = 0x66;
const OP_ROR_ZERO_PAGE_X: u8 = 0x76;
const OP_ROR_ABSOLUTE:    u8 = 0x6E;
const OP_ROR_ABSOLUTE_X:  u8 = 0x7E;

// Return from interrupt.
const OP_RTI_IMPLIED:     u8 = 0x40;

// Return from subroutine.
const OP_RTS_IMPLIED:     u8 = 0x60;

// Subtract memory from accumulator with borrow.
const OP_SBC_IMMEDIATE:   u8 = 0xE9;
const OP_SBC_ZERO_PAGE:   u8 = 0xE5;
const OP_SBC_ZERO_PAGE_X: u8 = 0xF5;
const OP_SBC_ABSOLUTE:    u8 = 0xED;
const OP_SBC_ABSOLUTE_X:  u8 = 0xFD;
const OP_SBC_ABSOLUTE_Y:  u8 = 0xF9;
const OP_SBC_INDIRECT_X:  u8 = 0xE1;
const OP_SBC_INDIRECT_Y:  u8 = 0xF1;

// Set carry flag.
const OP_SEC_IMPLIED:     u8 = 0x38;

// Set decimal mode.
const OP_SED_IMPLIED:     u8 = 0xF8;

// Set interrupt disable status.
const OP_SEI_IMPLIED:     u8 = 0x78;

// Store accumulator in memory.
const OP_STA_ZERO_PAGE:   u8 = 0x85;
const OP_STA_ZERO_PAGE_X: u8 = 0x95;
const OP_STA_ABSOLUTE:    u8 = 0x8D;
const OP_STA_ABSOLUTE_X:  u8 = 0x9D;
const OP_STA_ABSOLUTE_Y:  u8 = 0x99;
const OP_STA_INDIRECT_X:  u8 = 0x81;
const OP_STA_INDIRECT_Y:  u8 = 0x91;

// Store index X in memory.
const OP_STX_ZERO_PAGE:   u8 = 0x86;
const OP_STX_ZERO_PAGE_Y: u8 = 0x96;
const OP_STX_ABSOLUTE:    u8 = 0x8E;

// Store index Y in memory.
const OP_STY_ZERO_PAGE:   u8 = 0x84;
const OP_STY_ZERO_PAGE_X: u8 = 0x94;
const OP_STY_ABSOLUTE:    u8 = 0x8C;

// Transfer accumulator to index X.
const OP_TAX_IMPLIED:     u8 = 0xAA;

// Transfer accumulator to inxed Y.
const OP_TAY_IMPLIED:     u8 = 0xA8;

// Transfer index Y to accumulator.
const OP_TYA_IMPLIED:     u8 = 0x98;

// Transfer stack pointer to index X.
const OP_TSX_IMPLIED:     u8 = 0xBA;

// Transfer index X to accumulator.
const OP_TXA_IMPLIED:     u8 = 0x8A;

// Transfer index X to stack pointer.
const OP_TXS_IMPLIED:     u8 = 0x9A;

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
        // TODO: Get opcode, set addrmode, execute.
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
