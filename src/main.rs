use crate::encoder::encode;
use crate::file_io::read;
use crate::compress::write_compressed;
use crate::decoder::decode;
use crate::expand::write_expanded;

mod sturcture;
mod file_io;
mod encoder;
mod compress;
mod decoder;
mod expand;

fn main() {
    write_compressed();
    write_expanded();
}
