use mems::Memory;

pub struct Rom8b {
    data: Vec<u8>
}

impl Memory for Rom8b {
    fn write_u8(&mut self, _: u8) {
        panic!("Trying to overwrite read-only memory.");
    }

    fn read_u8(&self, addr: usize) -> u8 {
        self.data[addr]
    
    }

    fn write_u16(&mut self, _: u16) {
        panic!("Trying to overwrite read-only memory.");
    }

    fn read_u16(&self, addr: usize) -> u16 {
        let lo = self.data[addr] as u16;
        let hi = self.data[addr + 1] as u16;
    
        ((hi << 8) | lo)
    }
}

impl Rom8b {
    pub fn from_vec(vec: Vec<u8>) -> Rom8b {
        Rom8b {
            data: vec
        }
    }
}
