use std::env::{self, Args};
use std::io;

pub fn get_cmdline_arg() -> Result<String, io::Error> {
    env::args()
        .skip(1)
        .next()
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))
}

pub fn get_cmdline_args() -> Args {
    env::args()
}
