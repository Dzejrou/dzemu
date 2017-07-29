use cpus::Cpu;
use mems::Memory;
use regs::Registers;

enum AddressMode {
    Immediate,
    ZeroPage,
    ZeroPageX,
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
        AddressMode::None
    }
}
