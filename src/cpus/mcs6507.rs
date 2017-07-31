use cpus::Cpu;
use mems::Memory;

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

// Mask for testing of the 7th bit.
const NEG_MASK:           u8 = 1 << 7;

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

pub struct Mcs6507<M: Memory> {
    ram: M,
    pc: usize,
    sp: u8,
    idx_x: u8,
    idx_y: u8,
    accu: u8,
    addr_mode: AddressMode,

    // These are supposed to be flags,
    // but we have the memory.
    carry: bool,
    break_cmd: bool,
    decimal: bool,
    interrupt_disable: bool,
    negative: bool,
    overflow: bool,
    zero: bool
}

impl<M: Memory> Cpu<M> for Mcs6507<M> {
    fn memory(&self) -> &M {
        &self.ram
    }

    fn run(&mut self, cart : &Memory) {
        // TODO: PC initial value.
        loop {
            self.execute(cart);
            self.pc += self.pc_offset();
        }
    }
}

impl<M: Memory> Mcs6507<M> {
    pub fn new(ram: M) -> Mcs6507<M> {
        Mcs6507 {
            ram,
            pc: 0,
            sp: 0u8,
            idx_x: 0u8,
            idx_y: 0u8,
            accu: 0u8,
            addr_mode: AddressMode::None,
            carry: false,
            break_cmd: false,
            decimal: false,
            interrupt_disable: false,
            negative: false,
            overflow: false,
            zero: false
        }
    }

    fn execute(&mut self, cart: &Memory) {
        // TODO: Get opcode, set addrmode, execute.
        let opcode = cart.read_u8(self.pc as usize);
        self.addr_mode = self.get_addr_mode(opcode);
        let operand = self.get_operand(cart);

        match opcode {
            OP_ADC_IMMEDIATE   |
            OP_ADC_ZERO_PAGE   |
            OP_ADC_ZERO_PAGE_X |
            OP_ADC_ABSOLUTE    |
            OP_ADC_ABSOLUTE_X  |
            OP_ADC_ABSOLUTE_Y  |
            OP_ADC_INDIRECT_X  |
            OP_ADC_INDIRECT_Y  => self.op_adc(operand),

            OP_AND_IMMEDIATE   |
            OP_AND_ZERO_PAGE   |
            OP_AND_ZERO_PAGE_X |
            OP_AND_ABSOLUTE    |
            OP_AND_ABSOLUTE_X  |
            OP_AND_ABSOLUTE_Y  |
            OP_AND_INDIRECT_X  |
            OP_AND_INDIRECT_Y  => self.op_and(operand),

            OP_ASL_ACCUMULATOR |
            OP_ASL_ZERO_PAGE   |
            OP_ASL_ZERO_PAGE_X |
            OP_ASL_ABSOLUTE    |
            OP_ASL_ABSOLUTE_X  => self.op_asl(operand, cart),

            OP_BCC_RELATIVE    => self.op_bcc(operand),

            OP_BCS_RELATIVE    => self.op_bcs(operand),

            OP_BEQ_RELATIVE    => self.op_beq(operand),

            OP_BIT_ZERO_PAGE   |
            OP_BIT_ABSOLUTE    => self.op_bit(operand),

            OP_BMI_RELATIVE    => self.op_bmi(operand),

            OP_BNE_RELATIVE    => self.op_bne(operand),

            OP_BPL_RELATIVE    => self.op_bpl(operand),

            OP_BRK_IMPLIED     => self.op_brk(operand),

            OP_BVC_RELATIVE    => self.op_bvc(operand),

            OP_BVS_RELATIVE    => self.op_bvs(operand),

            OP_CLC_IMPLIED     => self.op_clc(),

            OP_CLD_IMPLIED     => self.op_cld(),

            OP_CLI_IMPLIED     => self.op_cli(),

            OP_CLV_IMPLIED     => self.op_clv(),

            OP_CMP_IMMEDIATE   |
            OP_CMP_ZERO_PAGE   |
            OP_CMP_ZERO_PAGE_X |
            OP_CMP_ABSOLUTE    |
            OP_CMP_ABSOLUTE_X  |
            OP_CMP_ABSOLUTE_Y  |
            OP_CMP_INDIRECT_X  |
            OP_CMP_INDIRECT_Y  => self.op_cmp(operand),

            OP_CPX_IMMEDIATE   |
            OP_CPX_ZERO_PAGE   |
            OP_CPX_ABSOLUTE    => self.op_cpx(operand),

            OP_CPY_IMMEDIATE   |
            OP_CPY_ZERO_PAGE   |
            OP_CPY_ABSOLUTE    => self.op_cpy(operand),

            OP_DEC_ZERO_PAGE   |
            OP_DEC_ZERO_PAGE_X |
            OP_DEC_ABSOLUTE    |
            OP_DEC_ABSOLUTE_X  => self.op_dec(operand, cart),

            OP_DEX_IMPLIED     => self.op_dex(),

            OP_DEY_IMPLIED     => self.op_dey(),

            OP_EOR_IMMEDIATE   |
            OP_EOR_ZERO_PAGE   |
            OP_EOR_ZERO_PAGE_X |
            OP_EOR_ABSOLUTE    |
            OP_EOR_ABSOLUTE_X  |
            OP_EOR_ABSOLUTE_Y  |
            OP_EOR_INDIRECT_X  |
            OP_EOR_INDIRECT_Y  => self.op_eor(operand),

            OP_INC_ZERO_PAGE   |
            OP_INC_ZERO_PAGE_X |
            OP_INC_ABSOLUTE    |
            OP_INC_ABSOLUTE_X  => self.op_inc(operand, cart),

            OP_INX_IMPLIED     => self.op_inx(),

            OP_INY_IMPLIED     => self.op_iny(),

            OP_JMP_ABSOLUTE    |
            OP_JMP_INDIRECT    => self.op_jmp(cart),

            OP_JSR_ABSOLUTE    => self.op_jsr(cart),

            OP_LDA_IMMEDIATE   |
            OP_LDA_ZERO_PAGE   |
            OP_LDA_ZERO_PAGE_X |
            OP_LDA_ABSOLUTE    |
            OP_LDA_ABSOLUTE_X  |
            OP_LDA_ABSOLUTE_Y  |
            OP_LDA_INDIRECT_X  |
            OP_LDA_INDIRECT_Y  => self.op_lda(operand),

            OP_LDX_IMMEDIATE   |
            OP_LDX_ZERO_PAGE   |
            OP_LDX_ZERO_PAGE_Y |
            OP_LDX_ABSOLUTE    |
            OP_LDX_ABSOLUTE_Y  => self.op_ldx(operand),

            OP_LDY_IMMEDIATE   |
            OP_LDY_ZERO_PAGE   |
            OP_LDY_ZERO_PAGE_X |
            OP_LDY_ABSOLUTE    |
            OP_LDY_ABSOLUTE_X  => self.op_ldy(operand),

            OP_LSR_ACCUMULATOR |
            OP_LSR_ZERO_PAGE   |
            OP_LSR_ZERO_PAGE_X |
            OP_LSR_ABSOLUTE    |
            OP_LSR_ABSOLUTE_X  => self.op_lsr(operand),

            OP_NOP_IMPLIED     => self.op_nop(),

            OP_ORA_IMMEDIATE   |
            OP_ORA_ZERO_PAGE   |
            OP_ORA_ZERO_PAGE_X |
            OP_ORA_ABSOLUTE    |
            OP_ORA_ABSOLUTE_X  |
            OP_ORA_ABSOLUTE_Y  |
            OP_ORA_INDIRECT_X  |
            OP_ORA_INDIRECT_Y  => self.op_ora(operand),

            OP_PHA_IMPLIED     => self.op_pha(),

            OP_PHP_IMPLIED     => self.op_php(),

            OP_PLA_IMPLIED     => self.op_pla(),

            OP_PLP_IMPLIED     => self.op_plp(),

            OP_ROL_ACCUMULATOR |
            OP_ROL_ZERO_PAGE   |
            OP_ROL_ZERO_PAGE_X |
            OP_ROL_ABSOLUTE    |
            OP_ROL_ABSOLUTE_X  => self.op_rol(operand, cart),

            OP_ROR_ACCUMULATOR |
            OP_ROR_ZERO_PAGE   |
            OP_ROR_ZERO_PAGE_X |
            OP_ROR_ABSOLUTE    |
            OP_ROR_ABSOLUTE_X  => self.op_ror(operand, cart),

            OP_RTI_IMPLIED     => self.op_rti(operand),

            OP_RTS_IMPLIED     => self.op_rts(),

            OP_SBC_IMMEDIATE   |
            OP_SBC_ZERO_PAGE   |
            OP_SBC_ZERO_PAGE_X |
            OP_SBC_ABSOLUTE    |
            OP_SBC_ABSOLUTE_X  |
            OP_SBC_ABSOLUTE_Y  |
            OP_SBC_INDIRECT_X  |
            OP_SBC_INDIRECT_Y  => self.op_sbc(operand),

            OP_SEC_IMPLIED     => self.op_sec(),

            OP_SED_IMPLIED     => self.op_sed(),

            OP_SEI_IMPLIED     => self.op_sei(),

            OP_STA_ZERO_PAGE   |
            OP_STA_ZERO_PAGE_X |
            OP_STA_ABSOLUTE    |
            OP_STA_ABSOLUTE_X  |
            OP_STA_ABSOLUTE_Y  |
            OP_STA_INDIRECT_X  |
            OP_STA_INDIRECT_Y  => self.op_sta(cart),

            OP_STX_ZERO_PAGE   |
            OP_STX_ZERO_PAGE_Y |
            OP_STX_ABSOLUTE    => self.op_stx(cart),

            OP_STY_ZERO_PAGE   |
            OP_STY_ZERO_PAGE_X |
            OP_STY_ABSOLUTE    => self.op_sty(cart),

            OP_TAX_IMPLIED     => self.op_tax(),

            OP_TAY_IMPLIED     => self.op_tay(),

            OP_TYA_IMPLIED     => self.op_tya(),

            OP_TSX_IMPLIED     => self.op_tsx(),

            OP_TXA_IMPLIED     => self.op_txa(),

            OP_TXS_IMPLIED     => self.op_txs(),

            op => panic!("Unknown opcode: {}", op)
        }
    }

    fn get_operand(&self, cart: &Memory) -> u8 {
        // TODO: In relative, the offset can be negative!
        match self.addr_mode {
            AddressMode::Relative    |
            AddressMode::Immediate   => {
                cart.read_u8(self.pc + 1)
            }

            AddressMode::ZeroPage    => {
                let addr = cart.read_u8(self.pc + 1) as usize;
                self.ram.read_u8(addr)
            }

            AddressMode::ZeroPageX   => {
                let addr = cart.read_u8(self.pc + 1) as usize;
                self.ram.read_u8(addr + self.idx_x as usize)
            }

            AddressMode::ZeroPageY   => {
                let addr = cart.read_u8(self.pc + 1) as usize;
                self.ram.read_u8(addr + self.idx_y as usize)
            }

            AddressMode::Absolute    => {
                let addr = cart.read_u16(self.pc + 1) as usize;
                self.ram.read_u8(addr)
            }

            AddressMode::AbsoluteX   => {
                let addr = cart.read_u16(self.pc + 1) as usize;
                self.ram.read_u8(addr + self.idx_x as usize)
            }

            AddressMode::AbsoluteY   => {
                let addr = cart.read_u16(self.pc + 1) as usize;
                self.ram.read_u8(addr + self.idx_y as usize)
            }

            AddressMode::IndirectX   => {
                let mut ptr = (cart.read_u8(self.pc + 1) + self.idx_x) as usize;
                // TODO: Does it really wrap around zero page?
                ptr = ptr % 0xFF;

                let addr = self.ram.read_u16(ptr) as usize;
                self.ram.read_u8(addr)
            }

            AddressMode::IndirectY   => {
                let mut addr = cart.read_u16(self.pc + 1) as usize;
                addr += self.idx_y as usize;

                self.ram.read_u8(addr)
            }

            AddressMode::Accumulator => self.accu,

            AddressMode::None        => 0
        }
    }

    fn set_operand(&mut self, cart: &Memory, operand: u8) {
        match self.addr_mode {
            AddressMode::ZeroPage    => {
                let addr = cart.read_u8(self.pc + 1) as usize;
                self.ram.write_u8(addr, operand);
            }

            AddressMode::ZeroPageX   => {
                let addr = cart.read_u8(self.pc + 1) as usize;
                self.ram.write_u8(addr + self.idx_x as usize, operand);
            }

            AddressMode::ZeroPageY   => {
                let addr = cart.read_u8(self.pc + 1) as usize;
                self.ram.write_u8(addr + self.idx_y as usize, operand);
            }

            AddressMode::Absolute    => {
                let addr = cart.read_u16(self.pc + 1) as usize;
                self.ram.write_u8(addr, operand);
            }

            AddressMode::AbsoluteX   => {
                let addr = cart.read_u16(self.pc + 1) as usize;
                self.ram.write_u8(addr + self.idx_x as usize, operand);
            }

            AddressMode::AbsoluteY   => {
                let addr = cart.read_u16(self.pc + 1) as usize;
                self.ram.write_u8(addr + self.idx_y as usize, operand);
            }

            AddressMode::IndirectX   => {
                let mut ptr = (cart.read_u8(self.pc + 1) + self.idx_x) as usize;
                ptr = ptr % 0xFF;

                let addr = self.ram.read_u16(ptr) as usize;
                self.ram.write_u8(addr, operand);
            }

            AddressMode::IndirectY   => {
                let mut addr = cart.read_u16(self.pc + 1) as usize;
                addr += self.idx_y as usize;

                self.ram.write_u8(addr, operand);
            }

            AddressMode::Accumulator => self.accu = operand,

            _                        => ()
        }
    }

    fn pc_offset(&self) -> usize {
        match self.addr_mode {
            AddressMode::Absolute  |
            AddressMode::AbsoluteX |
            AddressMode::AbsoluteY => 3,
            AddressMode::None      => 1,
            _                      => 2,
        }
    }

    fn get_sp(&self) -> usize {
        (self.sp as u16 + 0x0100u16) as usize
    }

    fn get_addr_mode(&self, opcode: u8) -> AddressMode {
        match opcode {
            OP_ADC_IMMEDIATE   |
            OP_AND_IMMEDIATE   |
            OP_CMP_IMMEDIATE   |
            OP_CPX_IMMEDIATE   |
            OP_CPY_IMMEDIATE   |
            OP_EOR_IMMEDIATE   |
            OP_LDA_IMMEDIATE   |
            OP_LDX_IMMEDIATE   |
            OP_LDY_IMMEDIATE   |
            OP_ORA_IMMEDIATE   |
            OP_SBC_IMMEDIATE   => AddressMode::Immediate,

            OP_ADC_ZERO_PAGE   |
            OP_AND_ZERO_PAGE   |
            OP_ASL_ZERO_PAGE   |
            OP_BIT_ZERO_PAGE   |
            OP_CMP_ZERO_PAGE   |
            OP_CPX_ZERO_PAGE   |
            OP_CPY_ZERO_PAGE   |
            OP_DEC_ZERO_PAGE   |
            OP_EOR_ZERO_PAGE   |
            OP_INC_ZERO_PAGE   |
            OP_LDA_ZERO_PAGE   |
            OP_LDX_ZERO_PAGE   |
            OP_LDY_ZERO_PAGE   |
            OP_LSR_ZERO_PAGE   |
            OP_ORA_ZERO_PAGE   |
            OP_ROL_ZERO_PAGE   |
            OP_ROR_ZERO_PAGE   |
            OP_SBC_ZERO_PAGE   |
            OP_STA_ZERO_PAGE   |
            OP_STX_ZERO_PAGE   |
            OP_STY_ZERO_PAGE   => AddressMode::ZeroPage,

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

            OP_ADC_ABSOLUTE    |
            OP_AND_ABSOLUTE    |
            OP_ASL_ABSOLUTE    |
            OP_BIT_ABSOLUTE    |
            OP_CMP_ABSOLUTE    |
            OP_CPX_ABSOLUTE    |
            OP_CPY_ABSOLUTE    |
            OP_DEC_ABSOLUTE    |
            OP_EOR_ABSOLUTE    |
            OP_INC_ABSOLUTE    |
            OP_JMP_ABSOLUTE    |
            OP_JSR_ABSOLUTE    |
            OP_LDA_ABSOLUTE    |
            OP_LDX_ABSOLUTE    |
            OP_LDY_ABSOLUTE    |
            OP_LSR_ABSOLUTE    |
            OP_ORA_ABSOLUTE    |
            OP_ROL_ABSOLUTE    |
            OP_ROR_ABSOLUTE    |
            OP_SBC_ABSOLUTE    |
            OP_STA_ABSOLUTE    |
            OP_STX_ABSOLUTE    |
            OP_STY_ABSOLUTE    => AddressMode::Absolute,

            OP_ADC_ABSOLUTE_X  |
            OP_AND_ABSOLUTE_X  |
            OP_ASL_ABSOLUTE_X  |
            OP_CMP_ABSOLUTE_X  |
            OP_DEC_ABSOLUTE_X  |
            OP_EOR_ABSOLUTE_X  |
            OP_INC_ABSOLUTE_X  |
            OP_LDA_ABSOLUTE_X  |
            OP_LDY_ABSOLUTE_X  |
            OP_LSR_ABSOLUTE_X  |
            OP_ORA_ABSOLUTE_X  |
            OP_ROL_ABSOLUTE_X  |
            OP_ROR_ABSOLUTE_X  |
            OP_SBC_ABSOLUTE_X  |
            OP_STA_ABSOLUTE_X  => AddressMode::AbsoluteX,

            OP_ADC_ABSOLUTE_Y  |
            OP_AND_ABSOLUTE_Y  |
            OP_CMP_ABSOLUTE_Y  |
            OP_EOR_ABSOLUTE_Y  |
            OP_LDA_ABSOLUTE_Y  |
            OP_LDX_ABSOLUTE_Y  |
            OP_ORA_ABSOLUTE_Y  |
            OP_SBC_ABSOLUTE_Y  |
            OP_STA_ABSOLUTE_Y  => AddressMode::AbsoluteY,

            OP_ADC_INDIRECT_X  |
            OP_AND_INDIRECT_X  |
            OP_CMP_INDIRECT_X  |
            OP_EOR_INDIRECT_X  |
            OP_LDA_INDIRECT_X  |
            OP_ORA_INDIRECT_X  |
            OP_SBC_INDIRECT_X  |
            OP_STA_INDIRECT_X  => AddressMode::IndirectX,

            OP_ADC_INDIRECT_Y  |
            OP_AND_INDIRECT_Y  |
            OP_CMP_INDIRECT_Y  |
            OP_EOR_INDIRECT_Y  |
            OP_LDA_INDIRECT_Y  |
            OP_ORA_INDIRECT_Y  |
            OP_SBC_INDIRECT_Y  |
            OP_STA_INDIRECT_Y  => AddressMode::IndirectY,

            OP_BCC_RELATIVE    |
            OP_BCS_RELATIVE    |
            OP_BEQ_RELATIVE    |
            OP_BMI_RELATIVE    |
            OP_BNE_RELATIVE    |
            OP_BPL_RELATIVE    |
            OP_BVS_RELATIVE    => AddressMode::Relative,

            OP_ASL_ACCUMULATOR |
            OP_LSR_ACCUMULATOR |
            OP_ROL_ACCUMULATOR |
            OP_ROR_ACCUMULATOR => AddressMode::Accumulator,

            _ => AddressMode::None,
        }
    }

    fn branch(&mut self, cond: bool, offset: u8) {
        if cond {
            let soff = offset as i8;
            let mut spc = self.pc as isize;
            spc += soff as isize;

            self.pc = spc as usize;
        }
    }

    fn jump(&mut self, offset: u16) {
        self.pc = offset as usize;
    }

    fn op_adc(&mut self, operand: u8) {
        if self.decimal {
            panic!("Decimal addition not implemented yet!");
        } else {
            let mut result = self.accu.wrapping_add(operand);
            result = result.wrapping_add(self.carry as u8);

            let mut check = (self.accu as u16) + (self.carry as u16);
            check +=  operand as u16;

            self.carry = check > 255;

            let signed = result as i16;
            self.overflow = signed > 127 || signed < -128;
        }

        let result = self.accu;
        self.zero = result == 0;
        self.negative = (result as i8) < 0;
    }

    fn op_and(&mut self, operand: u8) {
        self.accu &= operand;

        self.negative = (self.accu & NEG_MASK) > 0;
        self.zero = self.accu == 0;
    }

    fn op_asl(&mut self, mut operand: u8, cart: &Memory) {
        self.carry = (operand >> 7) == 1;

        operand <<= 1;

        self.negative = (operand & NEG_MASK) > 0;
        self.zero = operand == 0;
    
        self.set_operand(cart, operand);
    }

    fn op_bcc(&mut self, operand: u8) {
        let cond = !self.carry;
        self.branch(cond, operand);
    }

    fn op_bcs(&mut self, operand: u8) {
        let cond = self.carry;
        self.branch(cond, operand);
    }

    fn op_beq(&mut self, operand: u8) {
        let cond = self.zero;
        self.branch(cond, operand);
    }

    fn op_bit(&mut self, operand: u8) {
        let res = self.accu & operand;

        self.negative = (operand & NEG_MASK) > 0;
        self.overflow = (operand & (1 << 6)) > 0;
        self.zero = res == 0;
    }

    fn op_bmi(&mut self, operand: u8) {
        let cond = self.negative;
        self.branch(cond, operand);
    }

    fn op_bne(&mut self, operand: u8) {
        let cond = !self.zero;
        self.branch(cond, operand);
    }

    fn op_bpl(&mut self, operand: u8) {
        let cond = !self.negative;
        self.branch(cond, operand);
    }

    fn op_brk(&mut self, operand: u8) {

    }

    fn op_bvc(&mut self, operand: u8) {
        let cond = !self.overflow;
        self.branch(cond, operand);
    }

    fn op_bvs(&mut self, operand: u8) {
        let cond = self.overflow;
        self.branch(cond, operand);
    }

    fn op_clc(&mut self) {
        self.carry = true;
    }

    fn op_cld(&mut self) {
        self.decimal = true;
    }

    fn op_cli(&mut self) {
        self.interrupt_disable = false;
    }

    fn op_clv(&mut self) {
        self.overflow = false;
    }

    fn op_cmp(&mut self, operand: u8) {
        let res = (self.accu as i8) - (operand as i8);

        self.carry = res >= 0;
        self.negative = res < 0;
        self.zero = res == 0;
    }

    fn op_cpx(&mut self, operand: u8) {
        self.carry = self.idx_x >= operand;
        self.negative = self.idx_x < operand;;
        self.zero = self.idx_x == operand;
    }

    fn op_cpy(&mut self, operand: u8) {
        self.carry = self.idx_y >= operand;
        self.negative = self.idx_y < operand;;
        self.zero = self.idx_y == operand;
    }

    fn op_dec(&mut self, mut operand: u8, cart: &Memory) {
        operand += 1;
        self.set_operand(cart, operand);

        self.negative = (operand & NEG_MASK) > 0;
        self.zero = operand == 0;
    }

    fn op_dex(&mut self) {
        // TODO: Apparently, idx_x and idx_y are handled as signed.
        self.idx_x -= 1;
    
        self.negative = (self.idx_x & NEG_MASK) > 0;
        self.zero = self.idx_x == 0;
    }

    fn op_dey(&mut self) {
        self.idx_y -= 1;
    
        self.negative = (self.idx_y & NEG_MASK) > 0;
        self.zero = self.idx_y == 0;
    }

    fn op_eor(&mut self, operand: u8) {
        self.accu ^= operand;
    
        self.negative = (self.accu & NEG_MASK) > 0;
        self.zero = self.accu == 0;
    }

    fn op_inc(&mut self, mut operand: u8, cart: &Memory) {
        operand += 1;
        self.set_operand(cart, operand);

        self.negative = (operand & NEG_MASK) > 0;
        self.zero = operand == 0;
    }

    fn op_inx(&mut self) {
        self.idx_x += 1;

        self.negative = (self.idx_x & NEG_MASK) > 0;
        self.zero = self.idx_x == 0;
    }

    fn op_iny(&mut self) {
        self.idx_y += 1;

        self.negative = (self.idx_y & NEG_MASK) > 0;
        self.zero = self.idx_y == 0;
    }

    fn op_jmp(&mut self, cart: &Memory) {
        match self.addr_mode {
            AddressMode::Absolute => {
                let addr = cart.read_u16(self.pc + 1);
                self.jump(addr);
            }

            AddressMode::IndirectX => {
                // TODO: Page 141.
                // TODO: This is supposed to be just Indirect?
                ()
            }

            _                     => ()
        }
    }

    fn op_jsr(&mut self, cart: &Memory) {
        // TODO: PC is incremented after this!
        let addr = cart.read_u16(self.pc + 1) as usize;

        // Store pc.
        // TODO: SP starts at 0x01FF, but atari2600
        //       has only 128 bytes of memory?
        let sp_addr = self.get_sp();
        self.ram.write_u16(sp_addr, self.pc as u16);
        self.sp -= 2;

        self.pc = addr;
    }

    fn op_lda(&mut self, operand: u8) {
        self.accu = operand;
    
        self.negative = (self.accu & NEG_MASK) > 0;
        self.zero = self.accu == 0;
    }

    fn op_ldx(&mut self, operand: u8) {
        self.idx_x = operand;

        self.negative = (self.idx_x & NEG_MASK) > 0;
        self.zero = self.idx_x == 0;
    }

    fn op_ldy(&mut self, operand: u8) {
        self.idx_y = operand;

        self.negative = (self.idx_y & NEG_MASK) > 0;
        self.zero = self.idx_y == 0;
    
    }

    fn op_lsr(&mut self, operand: u8) {
    
    }

    fn op_nop(&mut self) {
        // Nothing done.
    }

    fn op_ora(&mut self, operand: u8) {
        self.accu |= operand;
    
        self.negative = (self.accu & NEG_MASK) > 0;
        self.zero = self.accu == 0;
    }

    fn op_pha(&mut self) {
        let addr = self.get_sp();
        self.ram.write_u8(addr, self.accu);
        self.sp -= 1;
    }

    fn op_php(&mut self) {
        // Note: We don't have the register so we
        //       manually store the boolean flags.
        // Problem: If any roms use this stored byte,
        //          we might be in trouble as the ordering
        //          is different.
        let mut reg = 0u8;

        if self.carry {
            reg |= 1;
        }

        if self.negative {
            reg |= 1 << 2;
        }

        if self.zero {
            reg |= 1 << 3;
        }

        if self.decimal {
            reg |= 1 << 4;
        }

        if self.break_cmd {
            reg |= 1 << 5;
        }

        if self.interrupt_disable {
            reg |= 1 << 6;
        }

        if self.overflow {
            reg |= 1 << 7;
        }
    
        let addr = self.get_sp();
        self.ram.write_u8(addr, reg);
        self.sp -= 1;
    }

    fn op_pla(&mut self) {
        self.sp += 1;
        let addr = self.get_sp();
        self.accu = self.ram.read_u8(addr);
    }

    fn op_plp(&mut self) {
        self.sp += 1;
        let addr = self.get_sp();
        let reg = self.ram.read_u8(addr);

        self.carry =             (reg & 1) > 0;
        self.negative =          (reg & (1 << 2)) > 0;
        self.zero =              (reg & (1 << 3)) > 0;
        self.decimal =           (reg & (1 << 4)) > 0;
        self.break_cmd =         (reg & (1 << 5)) > 0;
        self.interrupt_disable = (reg & (1 << 6)) > 0;
        self.overflow =          (reg & (1 << 7)) > 0;
    }

    fn op_rol(&mut self, mut operand: u8, cart: &Memory) {
        let input_carry = self.carry as u8;
        self.carry = (operand >> 7) == 1;

        operand <<= 1;
        if self.carry {
            operand |= input_carry;
        }

        self.negative = (operand >> 7) == 1;
        self.zero = operand == 0;
    
        self.set_operand(cart, operand);
    }

    fn op_ror(&mut self, mut operand: u8, cart: &Memory) {
        let input_carry = self.carry as u8;
        self.carry = (operand & 1) == 1;

        operand >>= 1;
        operand |= input_carry << 7;

        self.negative = (operand >> 7) == 1;
        self.zero = operand == 0;
    
        self.set_operand(cart, operand);
    }

    fn op_rti(&mut self, operand: u8) {
    
    }

    fn op_rts(&mut self) {
        self.sp += 2;
        self.pc = self.ram.read_u16(self.get_sp()) as usize;
    }

    fn op_sbc(&mut self, operand: u8) {
        if self.decimal {
            panic!("Decimal sbc mode not implemented!");
        } else {
            let mut res = self.accu as i16;
            res -= operand as i16;
            if !self.carry {
                res -= 1i16;
            }

            self.carry = res >= 0;
            self.overflow = (res < -127i16) || (res > 127i16);
            self.negative = res < 0;

            self.accu = (res & 0xFF) as u8;
        }
    }

    fn op_sec(&mut self) {
        self.carry = true;
    }

    fn op_sed(&mut self) {
        self.decimal = true;
    }

    fn op_sei(&mut self) {
        self.interrupt_disable = true;
    }

    fn op_sta(&mut self, cart: &Memory) {
        let res = self.accu;
        self.set_operand(cart, res);
    }

    fn op_stx(&mut self, cart: &Memory) {
        let res = self.idx_x;
        self.set_operand(cart, res);
    }

    fn op_sty(&mut self, cart: &Memory) {
        let res = self.idx_y;
        self.set_operand(cart, res);
    }

    fn op_tax(&mut self) {
        self.idx_x = self.accu;

        self.negative = (self.idx_x & NEG_MASK) > 0;
        self.zero = self.idx_x == 0;
    }

    fn op_tay(&mut self) {
        self.idx_y = self.accu;

        self.negative = (self.idx_y & NEG_MASK) > 0;
        self.zero = self.idx_y == 0;
    }

    fn op_tya(&mut self) {
        self.accu = self.idx_y;

        self.negative = (self.accu & NEG_MASK) > 0;
        self.zero = self.accu == 0;
    }

    fn op_tsx(&mut self) {
        self.idx_x = self.sp;
    
        self.negative = (self.idx_x & NEG_MASK) > 0;
        self.zero = self.idx_x == 0;
    }

    fn op_txa(&mut self) {
        self.accu = self.idx_x;

        self.negative = (self.accu & NEG_MASK) > 0;
        self.zero = self.accu == 0;
    }

    fn op_txs(&mut self) {
        self.sp = self.idx_x;
    }
}