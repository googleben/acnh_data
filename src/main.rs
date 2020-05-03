extern crate serde;
extern crate serde_json;
extern crate crc;
extern crate zstd;

pub mod binary;
pub mod bcsv;

mod codegen;


fn main() {
    #[cfg(feature = "final")]
    {
        let data = load_data().unwrap();
        println!("done");
    }
    
    #[cfg(feature = "codegen")]
    codegen::bcsvgen::gen();
}

#[cfg(feature = "final")]
struct ACNHData {
    bcsvs: bcsv::BCSVs
}

#[cfg(feature = "final")]
fn load_data() -> Result<ACNHData, String> {
    Ok(ACNHData {
        bcsvs: bcsv::BCSVs::load()?
    })
}