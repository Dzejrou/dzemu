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
}
