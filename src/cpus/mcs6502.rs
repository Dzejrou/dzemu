use cpus::Cpu;
use cpus::Stack;
use mems::Memory;
use inst::mcs6502::ops;
use inst::mcs6502::addr;
use inst::mcs6502::AddressMode;

// Start of the interrupt vector.
pub const INT_VECTOR_START:   usize = 0xFFFA;

// Interrupt request handler address.
pub const INT_REQ_ADDRESS:    usize = 0xFFFE;

// Non-maskable interrupt request
// handler address.
pub const INT_NOMASK_ADDRESS: usize = 0xFFFA;

// Address of the two byte address of
// the initial PC value.
pub const PC_INIT_ADDRESS:    usize = 0xFFFC;

// Starting address of the block of memory
// where the rom gets mapped.
pub const ROM_MAP_ADDRESS:    usize = 0x0000;

// Base address of the stack (higher byte).
pub const STACK_BASE_ADDRESS: usize = 0x0100;

// Starting address of the stack (lower byte).
pub const STACK_START_VALUE:  u8 = 0xFF;

// Processor status register fields.
// 5 is expansion bit.
pub const STS_CAR_MASK:    u8 = 1 << 0;
pub const STS_ZER_MASK:    u8 = 1 << 1;
pub const STS_INT_MASK:    u8 = 1 << 2;
pub const STS_DEC_MASK:    u8 = 1 << 3;
pub const STS_BRK_MASK:    u8 = 1 << 4;
pub const STS_OVF_MASK:    u8 = 1 << 6;
pub const STS_NEG_MASK:    u8 = 1 << 7;

pub struct Mcs6502<M: Memory> {
    ram: M,
    pc: usize,
    sp: u8,
    idx_x: u8,
    idx_y: u8,
    accu: u8,
    addr_mode: AddressMode,
    status: u8
}

impl<M: Memory> Cpu<M> for Mcs6502<M> {
    fn memory(&mut self) -> &mut M {
        &mut self.ram
    }

    fn boot(&mut self, cart : &Memory) {
        self.restart();

        self.ram.map(ROM_MAP_ADDRESS, cart);
    }

    fn restart(&mut self) {
        self.accu = 0;
        self.idx_x = 0;
        self.idx_y = 0;

        self.pc = self.ram.read_u16(PC_INIT_ADDRESS) as usize;
        self.sp = STACK_START_VALUE - 2;

        // Last instruction of the init sequence of a rom
        // should be CLI.
        self.set_flag(true, STS_INT_MASK);
    }

    fn execute(&mut self) {
        let opcode = self.ram.read_u8(self.pc as usize);
        self.addr_mode = addr::get_addr_mode(opcode);
        let operand = self.get_operand();

        match opcode {
            ops::ADC_IMMEDIATE   |
            ops::ADC_ZERO_PAGE   |
            ops::ADC_ZERO_PAGE_X |
            ops::ADC_ABSOLUTE    |
            ops::ADC_ABSOLUTE_X  |
            ops::ADC_ABSOLUTE_Y  |
            ops::ADC_INDIRECT_X  |
            ops::ADC_INDIRECT_Y  => self.op_adc(operand),

            ops::AND_IMMEDIATE   |
            ops::AND_ZERO_PAGE   |
            ops::AND_ZERO_PAGE_X |
            ops::AND_ABSOLUTE    |
            ops::AND_ABSOLUTE_X  |
            ops::AND_ABSOLUTE_Y  |
            ops::AND_INDIRECT_X  |
            ops::AND_INDIRECT_Y  => self.op_and(operand),

            ops::ASL_ACCUMULATOR |
            ops::ASL_ZERO_PAGE   |
            ops::ASL_ZERO_PAGE_X |
            ops::ASL_ABSOLUTE    |
            ops::ASL_ABSOLUTE_X  => self.op_asl(operand),

            ops::BCC_RELATIVE    => self.op_bcc(operand),

            ops::BCS_RELATIVE    => self.op_bcs(operand),

            ops::BEQ_RELATIVE    => self.op_beq(operand),

            ops::BIT_ZERO_PAGE   |
            ops::BIT_ABSOLUTE    => self.op_bit(operand),

            ops::BMI_RELATIVE    => self.op_bmi(operand),

            ops::BNE_RELATIVE    => self.op_bne(operand),

            ops::BPL_RELATIVE    => self.op_bpl(operand),

            ops::BRK_IMPLIED     => self.op_brk(),

            ops::BVC_RELATIVE    => self.op_bvc(operand),

            ops::BVS_RELATIVE    => self.op_bvs(operand),

            ops::CLC_IMPLIED     => self.op_clc(),

            ops::CLD_IMPLIED     => self.op_cld(),

            ops::CLI_IMPLIED     => self.op_cli(),

            ops::CLV_IMPLIED     => self.op_clv(),

            ops::CMP_IMMEDIATE   |
            ops::CMP_ZERO_PAGE   |
            ops::CMP_ZERO_PAGE_X |
            ops::CMP_ABSOLUTE    |
            ops::CMP_ABSOLUTE_X  |
            ops::CMP_ABSOLUTE_Y  |
            ops::CMP_INDIRECT_X  |
            ops::CMP_INDIRECT_Y  => self.op_cmp(operand),

            ops::CPX_IMMEDIATE   |
            ops::CPX_ZERO_PAGE   |
            ops::CPX_ABSOLUTE    => self.op_cpx(operand),

            ops::CPY_IMMEDIATE   |
            ops::CPY_ZERO_PAGE   |
            ops::CPY_ABSOLUTE    => self.op_cpy(operand),

            ops::DEC_ZERO_PAGE   |
            ops::DEC_ZERO_PAGE_X |
            ops::DEC_ABSOLUTE    |
            ops::DEC_ABSOLUTE_X  => self.op_dec(operand),

            ops::DEX_IMPLIED     => self.op_dex(),

            ops::DEY_IMPLIED     => self.op_dey(),

            ops::EOR_IMMEDIATE   |
            ops::EOR_ZERO_PAGE   |
            ops::EOR_ZERO_PAGE_X |
            ops::EOR_ABSOLUTE    |
            ops::EOR_ABSOLUTE_X  |
            ops::EOR_ABSOLUTE_Y  |
            ops::EOR_INDIRECT_X  |
            ops::EOR_INDIRECT_Y  => self.op_eor(operand),

            ops::INC_ZERO_PAGE   |
            ops::INC_ZERO_PAGE_X |
            ops::INC_ABSOLUTE    |
            ops::INC_ABSOLUTE_X  => self.op_inc(operand),

            ops::INX_IMPLIED     => self.op_inx(),

            ops::INY_IMPLIED     => self.op_iny(),

            ops::JMP_ABSOLUTE    |
            ops::JMP_INDIRECT    => self.op_jmp(),

            ops::JSR_ABSOLUTE    => self.op_jsr(),

            ops::LDA_IMMEDIATE   |
            ops::LDA_ZERO_PAGE   |
            ops::LDA_ZERO_PAGE_X |
            ops::LDA_ABSOLUTE    |
            ops::LDA_ABSOLUTE_X  |
            ops::LDA_ABSOLUTE_Y  |
            ops::LDA_INDIRECT_X  |
            ops::LDA_INDIRECT_Y  => self.op_lda(operand),

            ops::LDX_IMMEDIATE   |
            ops::LDX_ZERO_PAGE   |
            ops::LDX_ZERO_PAGE_Y |
            ops::LDX_ABSOLUTE    |
            ops::LDX_ABSOLUTE_Y  => self.op_ldx(operand),

            ops::LDY_IMMEDIATE   |
            ops::LDY_ZERO_PAGE   |
            ops::LDY_ZERO_PAGE_X |
            ops::LDY_ABSOLUTE    |
            ops::LDY_ABSOLUTE_X  => self.op_ldy(operand),

            ops::LSR_ACCUMULATOR |
            ops::LSR_ZERO_PAGE   |
            ops::LSR_ZERO_PAGE_X |
            ops::LSR_ABSOLUTE    |
            ops::LSR_ABSOLUTE_X  => self.op_lsr(operand),

            ops::NOP_IMPLIED     => self.op_nop(),

            ops::ORA_IMMEDIATE   |
            ops::ORA_ZERO_PAGE   |
            ops::ORA_ZERO_PAGE_X |
            ops::ORA_ABSOLUTE    |
            ops::ORA_ABSOLUTE_X  |
            ops::ORA_ABSOLUTE_Y  |
            ops::ORA_INDIRECT_X  |
            ops::ORA_INDIRECT_Y  => self.op_ora(operand),

            ops::PHA_IMPLIED     => self.op_pha(),

            ops::PHP_IMPLIED     => self.op_php(),

            ops::PLA_IMPLIED     => self.op_pla(),

            ops::PLP_IMPLIED     => self.op_plp(),

            ops::ROL_ACCUMULATOR |
            ops::ROL_ZERO_PAGE   |
            ops::ROL_ZERO_PAGE_X |
            ops::ROL_ABSOLUTE    |
            ops::ROL_ABSOLUTE_X  => self.op_rol(operand),

            ops::ROR_ACCUMULATOR |
            ops::ROR_ZERO_PAGE   |
            ops::ROR_ZERO_PAGE_X |
            ops::ROR_ABSOLUTE    |
            ops::ROR_ABSOLUTE_X  => self.op_ror(operand),

            ops::RTI_IMPLIED     => self.op_rti(),

            ops::RTS_IMPLIED     => self.op_rts(),

            ops::SBC_IMMEDIATE   |
            ops::SBC_ZERO_PAGE   |
            ops::SBC_ZERO_PAGE_X |
            ops::SBC_ABSOLUTE    |
            ops::SBC_ABSOLUTE_X  |
            ops::SBC_ABSOLUTE_Y  |
            ops::SBC_INDIRECT_X  |
            ops::SBC_INDIRECT_Y  => self.op_sbc(operand),

            ops::SEC_IMPLIED     => self.op_sec(),

            ops::SED_IMPLIED     => self.op_sed(),

            ops::SEI_IMPLIED     => self.op_sei(),

            ops::STA_ZERO_PAGE   |
            ops::STA_ZERO_PAGE_X |
            ops::STA_ABSOLUTE    |
            ops::STA_ABSOLUTE_X  |
            ops::STA_ABSOLUTE_Y  |
            ops::STA_INDIRECT_X  |
            ops::STA_INDIRECT_Y  => self.op_sta(),

            ops::STX_ZERO_PAGE   |
            ops::STX_ZERO_PAGE_Y |
            ops::STX_ABSOLUTE    => self.op_stx(),

            ops::STY_ZERO_PAGE   |
            ops::STY_ZERO_PAGE_X |
            ops::STY_ABSOLUTE    => self.op_sty(),

            ops::TAX_IMPLIED     => self.op_tax(),

            ops::TAY_IMPLIED     => self.op_tay(),

            ops::TYA_IMPLIED     => self.op_tya(),

            ops::TSX_IMPLIED     => self.op_tsx(),

            ops::TXA_IMPLIED     => self.op_txa(),

            ops::TXS_IMPLIED     => self.op_txs(),

            ops::custom::PRT_ABSOLUTE => self.op_prt(),

            op => panic!("Unknown opcode: {}", op)
        }

        self.pc = self.pc.wrapping_add(addr::pc_offset(&self.addr_mode));
    }

    fn run(&mut self, count: usize) {
        for _ in 0..count {
            self.execute();
        }
    }

    fn dump(&self) {
        let top = STACK_BASE_ADDRESS + STACK_START_VALUE as usize;
        println!("CPU STATE:");
        println!("|         PC: 0x{:X}", self.pc());
        println!("|         SP: 0x{:X}", self.sp());
        println!("|          X: 0x{:X}", self.idx_x);
        println!("|          Y: 0x{:X}", self.idx_y);
        println!("|       ACCU: 0x{:X}", self.accu);
        println!("|     STATUS: 0b{:b}", self.status);
        println!("|       ADDR: {:?}", self.addr_mode);
        println!("| STACK BASE: 0x{:X}", STACK_BASE_ADDRESS);
        println!("| STACK  TOP: 0x{:X}", top);

        println!("CPU STACK:");
        if top == self.sp() {
            println!("| EMPTY");
        } else {
            let mut i = top;
            while i > self.sp() {
                println!("| [{:X}]: {:X}", i, self.ram.read_u8(i));
                i = i.wrapping_sub(1);
            }
        }
    }

    fn pc(&self) -> usize {
        // Returns the addr relative to the start of
        // the rom mapping block.
        self.pc - ROM_MAP_ADDRESS
    }
}

impl<M: Memory> Stack for Mcs6502<M> {
    fn sp_dec(&mut self) {
        self.sp = self.sp.wrapping_sub(1);
    }

    fn sp_inc(&mut self) {
        self.sp = self.sp.wrapping_add(1);
    }

    fn push_u8(&mut self, data: u8) {
        let sp = self.sp();
        self.ram.write_u8(sp, data);
        self.sp_dec();
    }

    fn push_u16(&mut self, data: u16) {
        let sp = self.sp();
        self.ram.write_u16(sp, data);
        self.sp_dec();
        self.sp_dec();
    }

    fn pop_u8(&mut self) -> u8 {
        self.sp_inc();
        self.ram.read_u8(self.sp())
    }

    fn pop_u16(&mut self) -> u16 {
        self.sp_inc();
        self.sp_inc();
        self.ram.read_u16(self.sp())
    }

    fn sp(&self) -> usize {
        (self.sp as usize) + STACK_BASE_ADDRESS
    }
}

impl<M: Memory> Mcs6502<M> {
    pub fn new(ram: M) -> Mcs6502<M> {
        Mcs6502 {
            ram,
            pc: 0,
            sp: STACK_START_VALUE,
            idx_x: 0u8,
            idx_y: 0u8,
            accu: 0u8,
            addr_mode: AddressMode::None,
            status: 0u8
        }
    }

    pub fn set_init_pc_value(&mut self, addr: u16) {
        self.ram.write_u16(PC_INIT_ADDRESS, addr);
    }

    pub fn set_int_req_addr(&mut self, addr: u16) {
        self.ram.write_u16(INT_REQ_ADDRESS, addr);
    }

    pub fn set_int_nomask_addr(&mut self, addr: u16) {
        self.ram.write_u16(INT_NOMASK_ADDRESS, addr);
    }

    pub fn interrupt(&mut self) {
        if !self.get_flag(STS_INT_MASK) {
            let pc = self.pc as u16;
            let status = self.status;

            self.push_u16(pc);
            self.push_u8(status);
            self.set_flag(false, STS_BRK_MASK);
            self.set_flag(true, STS_INT_MASK);
            self.pc = self.ram.read_u16(INT_REQ_ADDRESS) as usize;
        }
    }

    pub fn non_maskable_interrupt(&mut self) {
        let pc = self.pc as u16;
        let status = self.status;

        self.push_u16(pc);
        self.push_u8(status);
        self.set_flag(false, STS_BRK_MASK);
        self.set_flag(true, STS_INT_MASK);
        self.pc = self.ram.read_u16(INT_NOMASK_ADDRESS) as usize;
    }

    fn get_operand(&self) -> u8 {
        match self.addr_mode {
            AddressMode::Relative    |
            AddressMode::Immediate   => {
                self.ram.read_u8(self.pc + 1)
            }

            AddressMode::ZeroPage    => {
                let addr = self.ram.read_u8(self.pc + 1) as usize;
                self.ram.read_u8(addr)
            }

            AddressMode::ZeroPageX   => {
                let mut addr = self.ram.read_u8(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_x as usize);

                self.ram.read_u8(addr)
            }

            AddressMode::ZeroPageY   => {
                let mut addr = self.ram.read_u8(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_y as usize);

                self.ram.read_u8(addr)
            }

            AddressMode::Absolute    => {
                let addr = self.ram.read_u16(self.pc + 1) as usize;
                self.ram.read_u8(addr)
            }

            AddressMode::AbsoluteX   => {
                let mut addr = self.ram.read_u16(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_x as usize);

                self.ram.read_u8(addr)
            }

            AddressMode::AbsoluteY   => {
                let mut addr = self.ram.read_u16(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_y as usize);

                self.ram.read_u8(addr)
            }

            AddressMode::IndirectX   => {
                let mut ptr = (self.ram.read_u8(self.pc + 1) + self.idx_x) as usize;
                ptr = ptr % 0xFF;

                let addr = self.ram.read_u16(ptr) as usize;
                self.ram.read_u8(addr)
            }

            AddressMode::IndirectY   => {
                let ptr = self.ram.read_u8(self.pc + 1) as usize;
                let mut addr = self.ram.read_u16(ptr) as usize;
                addr += self.idx_y as usize;

                self.ram.read_u8(addr)
            }

            AddressMode::Accumulator => self.accu,

            AddressMode::Indirect    |
            AddressMode::None        => 0
        }
    }

    fn set_flag(&mut self, cond: bool, mask: u8) {
        if cond {
            self.status |= mask;
        } else {
            self.status &= !mask;
        }
    }

    fn get_flag(&self, mask: u8) -> bool {
        self.status & mask > 0
    }

    fn set_operand(&mut self, operand: u8) {
        match self.addr_mode {
            AddressMode::ZeroPage    => {
                let addr = self.ram.read_u8(self.pc + 1) as usize;
                self.ram.write_u8(addr, operand);
            }

            AddressMode::ZeroPageX   => {
                let mut addr = self.ram.read_u8(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_x as usize);

                self.ram.write_u8(addr, operand);
            }

            AddressMode::ZeroPageY   => {
                let mut addr = self.ram.read_u8(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_y as usize);

                self.ram.write_u8(addr, operand);
            }

            AddressMode::Absolute    => {
                let addr = self.ram.read_u16(self.pc + 1) as usize;
                self.ram.write_u8(addr, operand);
            }

            AddressMode::AbsoluteX   => {
                let mut addr = self.ram.read_u16(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_x as usize);

                self.ram.write_u8(addr, operand);
            }

            AddressMode::AbsoluteY   => {
                let mut addr = self.ram.read_u16(self.pc + 1) as usize;
                addr = addr.wrapping_add(self.idx_y as usize);

                self.ram.write_u8(addr, operand);
            }

            AddressMode::IndirectX   => {
                let mut ptr = (self.ram.read_u8(self.pc + 1) + self.idx_x) as usize;
                ptr = ptr % 0xFF;

                let addr = self.ram.read_u16(ptr) as usize;
                self.ram.write_u8(addr, operand);
            }

            AddressMode::IndirectY   => {
                let ptr = self.ram.read_u8(self.pc + 1) as usize;
                let mut addr = self.ram.read_u16(ptr) as usize;
                addr += self.idx_y as usize;

                self.ram.write_u8(addr, operand);
            }

            AddressMode::Accumulator => self.accu = operand,

            _                        => ()
        }
    }

    fn branch(&mut self, cond: bool, offset: u8) {
        if cond {
            let soff = offset as i8;
            let mut spc = self.pc as isize;
            spc += soff as isize;
            spc -= addr::pc_offset(&self.addr_mode) as isize;

            self.pc = spc as usize;
        }
    }

    fn jump(&mut self, addr: usize) {
        self.pc = addr + ROM_MAP_ADDRESS;
    }

    fn op_adc(&mut self, operand: u8) {
        if self.get_flag(STS_DEC_MASK) {
            panic!("Decimal addition not implemented yet!");
        } else {
            let mut result = self.accu.wrapping_add(operand);
            let carry = self.get_flag(STS_CAR_MASK);
            result = result.wrapping_add(carry as u8);

            let mut check = (self.accu as u16) + (carry as u16);
            check +=  operand as u16;

            self.set_flag(check > 255, STS_CAR_MASK);

            let signed = result as i16;
            self.set_flag(signed > 127 || signed < -128, STS_OVF_MASK);

            self.accu = result;
        }

        let result = self.accu;
        self.set_flag(result == 0, STS_ZER_MASK);
        self.set_flag((result as i8) < 0, STS_NEG_MASK);
    }

    fn op_and(&mut self, operand: u8) {
        self.accu &= operand;

        let accu = self.accu;
        self.set_flag((accu & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(accu == 0, STS_ZER_MASK);
    }

    fn op_asl(&mut self, mut operand: u8) {
        self.set_flag((operand >> 7) == 1, STS_CAR_MASK);

        operand <<= 1;

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(operand == 0, STS_ZER_MASK);

        self.set_operand(operand);
    }

    fn op_bcc(&mut self, operand: u8) {
        let cond = !self.get_flag(STS_CAR_MASK);
        self.branch(cond, operand);
    }

    fn op_bcs(&mut self, operand: u8) {
        let cond = self.get_flag(STS_CAR_MASK);
        self.branch(cond, operand);
    }

    fn op_beq(&mut self, operand: u8) {
        let cond = self.get_flag(STS_ZER_MASK);
        self.branch(cond, operand);
    }

    fn op_bit(&mut self, operand: u8) {
        let res = self.accu & operand;

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag((operand & STS_OVF_MASK) > 0, STS_OVF_MASK);
        self.set_flag(res == 0, STS_ZER_MASK);
    }

    fn op_bmi(&mut self, operand: u8) {
        let cond = self.get_flag(STS_NEG_MASK);
        self.branch(cond, operand);
    }

    fn op_bne(&mut self, operand: u8) {
        let cond = !self.get_flag(STS_ZER_MASK);
        self.branch(cond, operand);
    }

    fn op_bpl(&mut self, operand: u8) {
        let cond = !self.get_flag(STS_NEG_MASK);
        self.branch(cond, operand);
    }

    fn op_brk(&mut self) {
        let pc = self.pc + 2;
        let status = self.status;
        self.push_u16(pc as u16);
        self.push_u8(status);

        self.set_flag(true, STS_BRK_MASK);
        self.pc = self.ram.read_u16(INT_REQ_ADDRESS) as usize;
        self.pc = self.pc.wrapping_sub(addr::pc_offset(&self.addr_mode));
    }

    fn op_bvc(&mut self, operand: u8) {
        let cond = !self.get_flag(STS_OVF_MASK);
        self.branch(cond, operand);
    }

    fn op_bvs(&mut self, operand: u8) {
        let cond = self.get_flag(STS_OVF_MASK);
        self.branch(cond, operand);
    }

    fn op_clc(&mut self) {
        self.set_flag(false, STS_CAR_MASK);
    }

    fn op_cld(&mut self) {
        self.set_flag(false, STS_DEC_MASK);
    }

    fn op_cli(&mut self) {
        self.set_flag(false, STS_INT_MASK);
    }

    fn op_clv(&mut self) {
        self.set_flag(false, STS_OVF_MASK);
    }

    fn op_cmp(&mut self, operand: u8) {
        let res = (self.accu as i8) - (operand as i8);

        self.set_flag(res >= 0, STS_CAR_MASK);
        self.set_flag(res < 0, STS_NEG_MASK);
        self.set_flag(res == 0, STS_ZER_MASK);
    }

    fn op_cpx(&mut self, operand: u8) {
        let idx_x = self.idx_x;
        self.set_flag(idx_x >= operand, STS_CAR_MASK);
        self.set_flag(idx_x < operand, STS_NEG_MASK);
        self.set_flag(idx_x == operand, STS_ZER_MASK);
    }

    fn op_cpy(&mut self, operand: u8) {
        let idx_y = self.idx_y;
        self.set_flag(idx_y >= operand, STS_CAR_MASK);
        self.set_flag(idx_y < operand, STS_NEG_MASK);
        self.set_flag(idx_y == operand, STS_ZER_MASK);
    }

    fn op_dec(&mut self, mut operand: u8) {
        operand -= 1;
        self.set_operand(operand);

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(operand == 0, STS_ZER_MASK);
    }

    fn op_dex(&mut self) {
        self.idx_x = (self.idx_x as i8 - 1) as u8;

        let idx_x = self.idx_x;
        self.set_flag((idx_x & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_x == 0, STS_ZER_MASK);
    }

    fn op_dey(&mut self) {
        self.idx_y = (self.idx_y as i8 - 1) as u8;

        let idx_y = self.idx_y;
        self.set_flag((idx_y & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_y == 0, STS_ZER_MASK);
    }

    fn op_eor(&mut self, operand: u8) {
        self.accu ^= operand;

        let accu = self.accu;
        self.set_flag((accu & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(accu == 0, STS_ZER_MASK);
    }

    fn op_inc(&mut self, mut operand: u8) {
        operand += 1;
        self.set_operand(operand);

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(operand == 0, STS_ZER_MASK);
    }

    fn op_inx(&mut self) {
        self.idx_x += 1;

        let idx_x = self.idx_x;
        self.set_flag((idx_x & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_x == 0, STS_ZER_MASK);
    }

    fn op_iny(&mut self) {
        self.idx_y += 1;

        let idx_y = self.idx_y;
        self.set_flag((idx_y & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_y == 0, STS_ZER_MASK);
    }

    fn op_jmp(&mut self) {
        let offs = addr::pc_offset(&self.addr_mode);
        let addr = self.ram.read_u16(self.pc + 1) as usize;

        match self.addr_mode {
            AddressMode::Absolute => {
                self.jump(addr - offs);
            }

            AddressMode::Indirect => {
                let actual_addr = self.ram.read_u16(addr) as usize;
                self.jump(actual_addr - offs);
            }

            _                     => ()
        }
    }

    fn op_jsr(&mut self) {
        let addr = self.ram.read_u16(self.pc + 1) as usize;

        // Note: Stored PC points at the last byte of jsr instruction,
        //       so return must let the cpu to increment the restored pc.
        let pc = self.pc + 2;
        self.push_u16(pc as u16);

        self.pc = addr.wrapping_sub(addr::pc_offset(&self.addr_mode));
    }

    fn op_lda(&mut self, operand: u8) {
        self.accu = operand;

        let accu = self.accu;
        self.set_flag((accu & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(accu == 0, STS_ZER_MASK);
    }

    fn op_ldx(&mut self, operand: u8) {
        self.idx_x = operand;

        let idx_x = self.idx_x;
        self.set_flag((idx_x & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_x == 0, STS_ZER_MASK);
    }

    fn op_ldy(&mut self, operand: u8) {
        self.idx_y = operand;

        let idx_y = self.idx_y;
        self.set_flag((idx_y & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_y == 0, STS_ZER_MASK);
    }

    fn op_lsr(&mut self, mut operand: u8) {
        self.set_flag((operand & 1) == 1, STS_CAR_MASK);

        operand >>= 1;

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(operand == 0, STS_ZER_MASK);

        self.set_operand(operand);
    }

    fn op_nop(&mut self) {
        // Nothing done.
    }

    fn op_ora(&mut self, operand: u8) {
        self.accu |= operand;

        let accu = self.accu;
        self.set_flag((accu & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(accu == 0, STS_ZER_MASK);
    }

    fn op_pha(&mut self) {
        let accu = self.accu;
        self.push_u8(accu);
    }

    fn op_php(&mut self) {
        let status = self.status;
        self.push_u8(status);
    }

    fn op_pla(&mut self) {
        self.accu = self.pop_u8();
    }

    fn op_plp(&mut self) {
        self.status = self.pop_u8();
    }

    fn op_rol(&mut self, mut operand: u8) {
        let input_carry = self.get_flag(STS_CAR_MASK) as u8;
        self.set_flag((operand >> 7) == 1, STS_CAR_MASK);

        operand <<= 1;
        operand |= input_carry;

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(operand == 0, STS_ZER_MASK);

        self.set_operand(operand);
    }

    fn op_ror(&mut self, mut operand: u8) {
        let input_carry = self.get_flag(STS_CAR_MASK) as u8;
        self.set_flag((operand & 1) == 1, STS_CAR_MASK);

        operand >>= 1;
        operand |= input_carry << 7;

        self.set_flag((operand & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(operand == 0, STS_ZER_MASK);

        self.set_operand(operand);
    }

    fn op_rti(&mut self) {
        self.op_plp();
        self.op_rts();
    }

    fn op_rts(&mut self) {
        self.pc = self.pop_u16() as usize;
        self.pc = self.pc.wrapping_sub(addr::pc_offset(&self.addr_mode));
        self.pc = self.pc.wrapping_add(1); // JSR sets PC to its last byte.
    }

    fn op_sbc(&mut self, operand: u8) {
        if self.get_flag(STS_DEC_MASK) {
            panic!("Decimal sbc mode not implemented!");
        } else {
            let mut res = self.accu as i16;
            res -= operand as i16;
            if !self.get_flag(STS_CAR_MASK) {
                res -= 1i16;
            }

            self.set_flag(res >= 0, STS_CAR_MASK);
            self.set_flag(res < -127i16 || res > 127i16, STS_OVF_MASK);
            self.set_flag(res < 0, STS_NEG_MASK);

            self.accu = (res & 0xFF) as u8;
        }
    }

    fn op_sec(&mut self) {
        self.set_flag(true, STS_CAR_MASK);
    }

    fn op_sed(&mut self) {
        self.set_flag(true, STS_DEC_MASK);
    }

    fn op_sei(&mut self) {
        self.set_flag(true, STS_INT_MASK);
    }

    fn op_sta(&mut self) {
        let res = self.accu;
        self.set_operand(res);
    }

    fn op_stx(&mut self) {
        let res = self.idx_x;
        self.set_operand(res);
    }

    fn op_sty(&mut self) {
        let res = self.idx_y;
        self.set_operand(res);
    }

    fn op_tax(&mut self) {
        self.idx_x = self.accu;

        let idx_x = self.idx_x;
        self.set_flag((idx_x & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_x == 0, STS_ZER_MASK);
    }

    fn op_tay(&mut self) {
        self.idx_y = self.accu;

        let idx_y = self.idx_y;
        self.set_flag((idx_y & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_y == 0, STS_ZER_MASK);
    }

    fn op_tya(&mut self) {
        self.accu = self.idx_y;

        let accu = self.accu;
        self.set_flag((accu & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(accu == 0, STS_ZER_MASK);
    }

    fn op_tsx(&mut self) {
        self.idx_x = self.sp;

        let idx_x = self.idx_x;
        self.set_flag((idx_x & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(idx_x == 0, STS_ZER_MASK);
    }

    fn op_txa(&mut self) {
        self.accu = self.idx_x;

        let accu = self.accu;
        self.set_flag((accu & STS_NEG_MASK) > 0, STS_NEG_MASK);
        self.set_flag(accu == 0, STS_ZER_MASK);
    }

    fn op_txs(&mut self) {
        self.sp = self.idx_x;
    }

    fn op_prt(&mut self) {
        let pc = self.pc;
        let mut addr = self.ram.read_u16(pc + 1) as usize;
        let mut byte = 0x01u8;
        let mut buf: Vec<u8> = Vec::new();

        while byte != 0x00 {
            byte = self.ram.read_u8(addr);
            addr += 1;
            buf.push(byte);
        }

        let str = String::from_utf8_lossy(&buf);
        println!("{}", str);
    }
}

#[cfg(test)]
mod tests {
    use mems::Memory;
    use mems::rom::Rom8b;
    use mems::ram::Ram8b;
    use cpus::Cpu;
    use cpus::Stack;
    use cpus::mcs6502::Mcs6502;
    use cpus::mcs6502;
    use inst::mcs6502::ops;
    use inst::mcs6502::AddressMode;

    #[test]
    fn op_adc() {
        // TODO: Test decimal addition when it's implemented.
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::ADC_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.set_flag(true, mcs6502::STS_NEG_MASK);
        cpu.accu = 0x05;
        cpu.memory().write_u8(0x0A, 0x02);

        let target = 0x05 + 0x02;
        cpu.execute();
        assert_eq!(cpu.accu, target);
        assert!(!cpu.get_flag(mcs6502::STS_NEG_MASK));
    }

    #[test]
    fn op_and() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::AND_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0xD5;
        cpu.memory().write_u8(0x0A, 0xAC);

        let target = 0xD5 & 0xAC;
        cpu.execute();
        assert_eq!(cpu.accu, target);
    }

    #[test]
    fn op_asl() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::ASL_ZERO_PAGE);
        instructions.push(0x0A);
        instructions.push(ops::ASL_ACCUMULATOR);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        let mut orig = 0xFF;
        cpu.memory().write_u8(0x0A, orig);
        let mut target = orig << 1;
        cpu.execute();
        assert_eq!(cpu.memory().read_u8(0x0A), target);
        assert_eq!((orig & (1 << 7)) > 0, cpu.get_flag(mcs6502::STS_CAR_MASK));

        orig = 0x07;
        cpu.accu = orig;
        target = cpu.accu << 1;
        cpu.execute();
        assert_eq!(cpu.accu, target);
        assert_eq!((orig & (1 << 7)) > 0, cpu.get_flag(mcs6502::STS_CAR_MASK));
    }

    fn aux_branch(opcode: u8, flag: u8, cond: bool) {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(opcode);
        instructions.push(0x0A);
        instructions.push(opcode);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        // Fail to branch.
        let mut target = 0x02;
        cpu.set_flag(!cond, flag);
        cpu.execute();
        assert_eq!(cpu.pc(), target);

        // Succeed at branching.
        target += 0x0A;
        cpu.set_flag(cond, flag);
        cpu.execute();
        assert_eq!(cpu.pc(), target);
    }

    #[test]
    fn op_bcc() {
        aux_branch(ops::BCC_RELATIVE, mcs6502::STS_CAR_MASK, false);
    }

    #[test]
    fn op_bcs() {
        aux_branch(ops::BCS_RELATIVE, mcs6502::STS_CAR_MASK, true);
    }

    #[test]
    fn op_beq() {
        aux_branch(ops::BEQ_RELATIVE, mcs6502::STS_ZER_MASK, true);
    }

    #[test]
    fn op_bit() {
        let addr: u8 = 0x7F;
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::BIT_ZERO_PAGE);
        instructions.push(addr);
        instructions.push(ops::BIT_ZERO_PAGE);
        instructions.push(addr);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.memory().write_u8(addr as usize, 0b11001010);
        cpu.accu = 0b11101100;

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_NEG_MASK));
        assert!(cpu.get_flag(mcs6502::STS_OVF_MASK));
        assert!(!cpu.get_flag(mcs6502::STS_ZER_MASK));

        cpu.memory().write_u8(addr as usize, 0b00000011);
        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_ZER_MASK));
    }

    #[test]
    fn op_bmi() {
        aux_branch(ops::BMI_RELATIVE, mcs6502::STS_NEG_MASK, true);
    }

    #[test]
    fn op_bne() {
        aux_branch(ops::BNE_RELATIVE, mcs6502::STS_ZER_MASK, false);
    }

    #[test]
    fn op_bpl() {
        aux_branch(ops::BPL_RELATIVE, mcs6502::STS_NEG_MASK, false);
    }

    #[test]
    fn op_brk() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::BRK_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.memory().write_u16(mcs6502::INT_REQ_ADDRESS, 0xBEEF);
        cpu.set_flag(true, mcs6502::STS_CAR_MASK);
        cpu.set_flag(true, mcs6502::STS_OVF_MASK);
        cpu.set_flag(true, mcs6502::STS_INT_MASK);
        let pc = cpu.pc();
        let status = cpu.status;

        cpu.execute();
        assert_eq!(cpu.pc(), 0xBEEF);
        let stack_status = cpu.pop_u8();
        assert_eq!(stack_status, status);
        let stack_pc = cpu.pop_u16() as usize;
        assert_eq!(stack_pc, pc + 2)
    }

    #[test]
    fn op_bvc() {
        aux_branch(ops::BVC_RELATIVE, mcs6502::STS_OVF_MASK, false);
    }

    #[test]
    fn op_bvs() {
        aux_branch(ops::BVS_RELATIVE, mcs6502::STS_OVF_MASK, true);
    }

    fn aux_clear(opcode: u8, flag: u8) {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(opcode);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.set_flag(true, flag);
        cpu.execute();
        assert!(!cpu.get_flag(flag));
    }

    #[test]
    fn op_clc() {
        aux_clear(ops::CLC_IMPLIED, mcs6502::STS_CAR_MASK);
    }

    #[test]
    fn op_cld() {
        aux_clear(ops::CLD_IMPLIED, mcs6502::STS_DEC_MASK);
    }

    #[test]
    fn op_cli() {
        aux_clear(ops::CLI_IMPLIED, mcs6502::STS_INT_MASK);
    }

    #[test]
    fn op_clv() {
        aux_clear(ops::CLV_IMPLIED, mcs6502::STS_OVF_MASK);
    }

    #[test]
    fn op_cmp() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::CMP_IMMEDIATE);
        instructions.push(0x05);
        instructions.push(ops::CMP_IMMEDIATE);
        instructions.push(0x06);
        instructions.push(ops::CMP_IMMEDIATE);
        instructions.push(0x07);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0x06;

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_ZER_MASK));
        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_NEG_MASK));
    }

    #[test]
    fn op_cpx() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::CPX_IMMEDIATE);
        instructions.push(0x05);
        instructions.push(ops::CPX_IMMEDIATE);
        instructions.push(0x06);
        instructions.push(ops::CPX_IMMEDIATE);
        instructions.push(0x07);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_x = 0x06;

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_ZER_MASK));
        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_NEG_MASK));
    }

    #[test]
    fn op_cpy() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::CPY_IMMEDIATE);
        instructions.push(0x05);
        instructions.push(ops::CPY_IMMEDIATE);
        instructions.push(0x06);
        instructions.push(ops::CPY_IMMEDIATE);
        instructions.push(0x07);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_y = 0x06;

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_ZER_MASK));
        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));

        cpu.execute();
        assert!(cpu.get_flag(mcs6502::STS_NEG_MASK));
    }

    #[test]
    fn op_dec() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::DEC_ABSOLUTE);
        instructions.push(0x0A);
        instructions.push(0x00);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.memory().write_u8(0x000A, 5);
        cpu.execute();
        assert_eq!(cpu.memory().read_u8(0x000A), 4);
    }

    #[test]
    fn op_dex() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::DEX_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_x = 0x0B;
        cpu.execute();
        assert_eq!(cpu.idx_x, 0x0A);
    }

    #[test]
    fn op_dey() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::DEY_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_y = 0x0B;
        cpu.execute();
        assert_eq!(cpu.idx_y, 0x0A);
    }

    #[test]
    fn op_eor() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::EOR_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0x31;
        cpu.memory().write_u8(0x0A, 0xF3);

        let target = 0x31 ^ 0xF3;
        cpu.execute();
        assert_eq!(cpu.accu, target);
    }

    #[test]
    fn op_inc() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::INC_ABSOLUTE);
        instructions.push(0x0A);
        instructions.push(0x00);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.memory().write_u8(0x000A, 5);
        cpu.execute();
        assert_eq!(cpu.memory().read_u8(0x000A), 6);
    }

    #[test]
    fn op_inx() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::INX_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_x = 0x0A;
        cpu.execute();
        assert_eq!(cpu.idx_x, 0x0B);
    }

    #[test]
    fn op_iny() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::INY_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_y = 0x0A;
        cpu.execute();
        assert_eq!(cpu.idx_y, 0x0B);
    }

    #[test]
    fn op_jmp() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::JMP_ABSOLUTE);
        instructions.push(0xA0);
        instructions.push(0x01);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        let mut target = 0x01A0;

        cpu.execute();
        assert_eq!(cpu.addr_mode, AddressMode::Absolute);
        assert_eq!(cpu.pc(), target);

        cpu.memory().write_u8(target, ops::JMP_INDIRECT);
        cpu.memory().write_u16(target + 1, 0xABBA);
        cpu.memory().write_u16(0xABBA, 0xBAAB);
        target = 0xBAAB;

        cpu.execute();
        assert_eq!(cpu.addr_mode, AddressMode::Indirect);
        assert_eq!(cpu.pc(), target);
    }

    #[test]
    fn op_jsr() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(0x00);
        instructions.push(ops::JSR_ABSOLUTE);
        instructions.push(0xA0);
        instructions.push(0x01);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.pc += 1;

        cpu.execute();
        let stored_pc = cpu.pop_u16() as usize;
        assert_eq!(stored_pc, 0x03);
        assert_eq!(cpu.pc(), 0x01A0);
    }

    #[test]
    fn op_lda() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::LDA_IMMEDIATE);
        instructions.push(0xAB);
        instructions.push(ops::LDA_ABSOLUTE);
        instructions.push(0x34);
        instructions.push(0x12);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        let mut target = 0xAB;

        cpu.execute();
        assert_eq!(cpu.accu, target);

        target = 0xFC;
        cpu.memory().write_u8(0x1234, target);
        cpu.execute();
        assert_eq!(cpu.accu, target);
    }

    #[test]
    fn op_ldx() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::LDX_IMMEDIATE);
        instructions.push(0xAB);
        instructions.push(ops::LDX_ABSOLUTE);
        instructions.push(0x34);
        instructions.push(0x12);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        let mut target = 0xAB;

        cpu.execute();
        assert_eq!(cpu.idx_x, target);

        target = 0xFC;
        cpu.memory().write_u8(0x1234, target);
        cpu.execute();
        assert_eq!(cpu.idx_x, target);
    }

    #[test]
    fn op_ldy() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::LDY_IMMEDIATE);
        instructions.push(0xAB);
        instructions.push(ops::LDY_ABSOLUTE);
        instructions.push(0x34);
        instructions.push(0x12);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        let mut target = 0xAB;

        cpu.execute();
        assert_eq!(cpu.idx_y, target);

        target = 0xFC;
        cpu.memory().write_u8(0x1234, target);
        cpu.execute();
        assert_eq!(cpu.idx_y, target);
    }

    #[test]
    fn op_lsr() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::LSR_ZERO_PAGE);
        instructions.push(0x0A);
        instructions.push(ops::LSR_ACCUMULATOR);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        let mut orig = 0xF3;
        cpu.memory().write_u8(0x0A, orig);
        let mut target = orig >> 1;
        cpu.execute();
        assert_eq!(cpu.memory().read_u8(0x0A), target);
        assert_eq!((orig & 1) == 1, cpu.get_flag(mcs6502::STS_CAR_MASK));

        orig = 0x0A;
        cpu.accu = orig;
        target = cpu.accu >> 1;
        cpu.execute();
        assert_eq!(cpu.accu, target);
        assert_eq!((orig & 1) == 1, cpu.get_flag(mcs6502::STS_CAR_MASK));
    }

    #[test]
    fn op_nop() {
        assert!(true);
    }

    #[test]
    fn op_ora() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::ORA_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0x31;
        cpu.memory().write_u8(0x0A, 0xF3);

        let target = 0x31 | 0xF3;
        cpu.execute();
        assert_eq!(cpu.accu, target);
    }

    #[test]
    fn op_pha() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::PHA_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        cpu.accu = 0xAF;
        cpu.execute();

        let top = cpu.pop_u8();
        assert_eq!(top, cpu.accu);
    }

    #[test]
    fn op_php() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::PHP_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        cpu.set_flag(true, mcs6502::STS_CAR_MASK);
        cpu.set_flag(true, mcs6502::STS_NEG_MASK);
        cpu.set_flag(false, mcs6502::STS_ZER_MASK);
        cpu.set_flag(true, mcs6502::STS_DEC_MASK);
        cpu.set_flag(false, mcs6502::STS_INT_MASK);
        cpu.set_flag(true, mcs6502::STS_OVF_MASK);
        cpu.execute();

        let top = cpu.pop_u8();

        assert!((top & mcs6502::STS_CAR_MASK) > 0);
        assert!((top & mcs6502::STS_NEG_MASK) > 0);
        assert!((top & mcs6502::STS_ZER_MASK) == 0);
        assert!((top & mcs6502::STS_DEC_MASK) > 0);
        assert!((top & mcs6502::STS_INT_MASK) == 0);
        assert!((top & mcs6502::STS_OVF_MASK) > 0);
    }

    #[test]
    fn op_pla() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::PLA_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.push_u8(0xFA);

        cpu.execute();

        assert_eq!(cpu.accu, 0xFA);
    }

    #[test]
    fn op_plp() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::PLP_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        let mut state = 0u8;
        state |= mcs6502::STS_CAR_MASK;
        state |= mcs6502::STS_INT_MASK;
        state |= mcs6502::STS_NEG_MASK;
        cpu.push_u8(state);

        cpu.execute();

        assert!(cpu.get_flag(mcs6502::STS_CAR_MASK));
        assert!(cpu.get_flag(mcs6502::STS_NEG_MASK));
        assert!(!cpu.get_flag(mcs6502::STS_ZER_MASK));
        assert!(!cpu.get_flag(mcs6502::STS_DEC_MASK));
        assert!(cpu.get_flag(mcs6502::STS_INT_MASK));
        assert!(!cpu.get_flag(mcs6502::STS_OVF_MASK));
    }

    #[test]
    fn op_rol() {
        // TODO: Test variant with carry clear.
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::ROL_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.set_flag(true, mcs6502::STS_CAR_MASK);

        let orig = 0b1011;
        cpu.memory().write_u8(0x0A, orig);
        let mut target = orig << 1;
        if cpu.get_flag(mcs6502::STS_CAR_MASK) {
            target |= 1;
        }

        cpu.execute();
        assert_eq!(cpu.memory().read_u8(0x0A), target);
        assert_eq!((orig & (1 << 7)) > 0, cpu.get_flag(mcs6502::STS_CAR_MASK));
    }

    #[test]
    fn op_ror() {
        // TODO: Test variant with carry clear.
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::ROR_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.set_flag(true, mcs6502::STS_CAR_MASK);

        let orig = 0b1011;
        cpu.memory().write_u8(0x0A, orig);
        let mut target = orig >> 1;
        if cpu.get_flag(mcs6502::STS_CAR_MASK) {
            target |= 1 << 7;
        }

        cpu.execute();
        assert_eq!(cpu.memory().read_u8(0x0A), target);
        assert_eq!((orig & 1) == 1, cpu.get_flag(mcs6502::STS_CAR_MASK));
    }

    #[test]
    fn op_rti() {
        // Composite instruction.
        op_plp();
        op_rts();
    }

    #[test]
    fn op_rts() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::RTS_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        let mut target = 0xFFAA;
        cpu.push_u16(target);
        target += 1; // JSR stores address of its last byte.

        cpu.execute();
        assert_eq!(cpu.pc(), target as usize);
    }

    #[test]
    fn op_sbc() {
        // TODO: Test decimal subtraction when it's implemented.
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::SBC_ZERO_PAGE);
        instructions.push(0x0A);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.set_flag(true, mcs6502::STS_CAR_MASK);
        cpu.accu = 0x05;
        cpu.memory().write_u8(0x0A, 0x02);

        let target = 0x05 - 0x02;
        cpu.execute();
        assert_eq!(cpu.accu, target);
    }

    fn aux_set(opcode: u8, flag: u8) {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(opcode);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.set_flag(false, flag);
        cpu.execute();
        assert!(cpu.get_flag(flag));
    }

    #[test]
    fn op_sec() {
        aux_set(ops::SEC_IMPLIED, mcs6502::STS_CAR_MASK);
    }

    #[test]
    fn op_sed() {
        aux_set(ops::SED_IMPLIED, mcs6502::STS_DEC_MASK);
    }

    #[test]
    fn op_sei() {
        aux_set(ops::SEI_IMPLIED, mcs6502::STS_INT_MASK);
    }

    #[test]
    fn op_sta() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::STA_ZERO_PAGE);
        instructions.push(0x35);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0xAC;

        cpu.execute();
        let res = cpu.memory().read_u8(0x35);
        assert_eq!(cpu.accu, res);
    }

    #[test]
    fn op_stx() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::STX_ZERO_PAGE_Y);
        instructions.push(0x35);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_x = 0xAC;
        cpu.idx_y = 0x05;

        cpu.execute();
        let addr: usize = 0x35 + 0x05;
        let res = cpu.memory().read_u8(addr);
        assert_eq!(cpu.idx_x, res);
    }

    #[test]
    fn op_sty() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::STY_ZERO_PAGE_X);
        instructions.push(0x35);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_y = 0xAC;
        cpu.idx_x = 0xFB; // -5

        cpu.execute();
        let addr: usize = 0x30;
        let res = cpu.memory().read_u8(addr);
        assert_eq!(cpu.idx_y, res);
    }

    #[test]
    fn op_tax() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::TAX_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0xC5;

        cpu.execute();
        assert_eq!(cpu.idx_x, cpu.accu);
    }

    #[test]
    fn op_tay() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::TAY_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.accu = 0xC5;

        cpu.execute();
        assert_eq!(cpu.idx_y, cpu.accu);
    }

    #[test]
    fn op_tya() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::TYA_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_y = 0xC5;

        cpu.execute();
        assert_eq!(cpu.accu, cpu.idx_y);
    }

    #[test]
    fn op_tsx() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::TSX_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        cpu.execute();
        assert_eq!(cpu.idx_x, cpu.sp);
    }

    #[test]
    fn op_txa() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::TXA_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_x = 0xC5;

        cpu.execute();
        assert_eq!(cpu.accu, cpu.idx_x);
    }

    #[test]
    fn op_txs() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::TXS_IMPLIED);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.idx_x = 0xC5;

        cpu.execute();
        assert_eq!(cpu.sp, cpu.idx_x);
    }

    #[test]
    fn op_prt() {
        let addr: usize = 0x04FF;
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::custom::PRT_ABSOLUTE);
        instructions.push((addr & 0xFF) as u8);
        instructions.push((addr >> 8) as u8);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);
        cpu.memory().write_u8(addr, 0x48);
        cpu.memory().write_u8(addr + 1, 0x45);
        cpu.memory().write_u8(addr + 2, 0x4C);
        cpu.memory().write_u8(addr + 3, 0x4C);
        cpu.memory().write_u8(addr + 4, 0x4F);
        cpu.memory().write_u8(addr + 5, 0x00);

        cpu.execute();
        // TODO: Make PRT use a display device (by default some
        //       stdout wrapper), so we can test this properly.
        //assert!(false);
    }

    #[test]
    fn prg_function_call() {
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::LDX_IMMEDIATE);
        instructions.push(0xAB);
        instructions.push(ops::JSR_ABSOLUTE);
        instructions.push(0x34);
        instructions.push(0x12);
        instructions.push(ops::LDX_IMMEDIATE);
        instructions.push(0xBC);

        let cart = Rom8b::from_vec(instructions);
        let mut cpu = Mcs6502::new(Ram8b::new(64 * 1024));

        cpu.boot(&cart);

        // Create the function.
        let mut instructions: Vec<u8> = Vec::new();
        instructions.push(ops::LDY_IMMEDIATE);
        instructions.push(0x3D);
        instructions.push(ops::RTS_IMPLIED);
        let func = Rom8b::from_vec(instructions);
        cpu.memory().map(0x1234, &func);

        // Note: After it executes everything, mem[pc] is equal
        //       to 0, which will execute BRK, which loads the
        //       value at mem[INT_REQ_ADDRESS], resetting the
        //       sequence.
        for _ in 0..5 {
            cpu.execute();
        }

        assert_eq!(cpu.idx_x, 0xBC);
        assert_eq!(cpu.idx_y, 0x3D);
    }
}
