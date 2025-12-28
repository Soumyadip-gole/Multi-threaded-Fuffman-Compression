use crate::encoder::encode;
use crate::file_io::read;
use crate::compress::write_compressed;


mod sturcture;
mod file_io;
mod encoder;
mod compress;

fn main() {
    write_compressed();
}
