extern crate dzemu;

use dzemu::util;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // TODO: Assert argument count, possibly add options.

    let rom_data = util::read_rom(&args[1]);

    util::dump_rom(&rom_data);
}
