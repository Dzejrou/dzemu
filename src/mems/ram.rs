use mems::Memory;

pub struct Ram8b64kB {
    data: [u8; 64 * 1024],
    size: usize
}

impl Memory for Ram8b64kB {
    fn write_u8(&mut self, addr: usize, data: u8) {
        self.data[addr] = data;
    }

    fn read_u8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    fn write_u16(&mut self, addr: usize, data: u16) {
        let lo = (data & 0xFF) as u8;
        let hi = (data >> 8) as u8;
        self.data[addr] = lo;
        self.data[addr + 1] = hi;
    }

    fn read_u16(&self, addr: usize) -> u16 {
        let lo = self.data[addr] as u16;
        let hi = self.data[addr + 1] as u16;

        ((hi << 8) | lo)
    }

    fn size(&self) -> usize {
        self.size
    }

    fn map(&mut self, start: usize, src: &Memory) {
        for i in 0..src.size() {
            self.data[start + i] = src.read_u8(i);
        }
    }
}

impl Ram8b64kB {
    pub fn new() -> Ram8b64kB {
        Ram8b64kB {
            data: [0; 64 * 1024],
            size: 64 * 1024
        }
    }
}
