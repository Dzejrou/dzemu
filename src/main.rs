extern crate dzemu;

use dzemu::util;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        panic!("rom file not provided");
    }

    let rom_data = util::read_rom(&args[1]);

    util::dump_rom(&rom_data);
}
