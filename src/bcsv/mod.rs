#[cfg(feature = "final")]
pub mod defs;
pub mod utils;
#[cfg(feature = "final")]
pub use defs::*;

pub trait BCSVRow {
    type T;
    fn new(data: &Vec<Vec<u8>>) -> Self::T;
    fn make_rows(bcsv: crate::binary::bcsv::BCSV) -> Vec<Self::T> {
        let mut ans = vec!();
        for row in bcsv.data.into_iter() {
            ans.push(Self::new(&row.entries));
        }
        ans
    }
}

use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn read_bcsv<'a, P: AsRef<Path>>(filename: P) -> Result<crate::binary::bcsv::BCSV, &'a str> {
    let mut data = vec!();
    let mut f = match File::open(filename) {
        std::io::Result::Ok(f) => f,
        _ => return std::result::Result::Err("FS error")
    };
    match f.read_to_end(&mut data) {
        std::io::Result::Err(_) => return Err("FS error"),
        _ => {}
    };
    crate::binary::bcsv::BCSV::new(&data)
}