extern crate serde;
extern crate serde_json;
extern crate crc;

pub mod binary;
pub mod bcsv;

mod codegen;


fn main() -> std::io::Result<()> {
    #[cfg(feature = "final")]
    {
        let data = load_data().unwrap();
        println!("done");
    }

    #[cfg(feature = "codegen")]
    codegen::bcsvgen::gen();
    
    Ok(())
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