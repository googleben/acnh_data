extern crate serde;
extern crate serde_json;
extern crate crc;

pub mod binary;
pub mod bcsv;

mod codegen;


fn main() -> std::io::Result<()> {
    #[cfg(feature = "final")]
    println!("final");
    
    #[cfg(feature = "codegen")]
    codegen::enumgen::gen();
    
    Ok(())
}
