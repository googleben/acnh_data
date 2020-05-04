extern crate serde;
extern crate serde_json;
extern crate crc;
extern crate zstd;
extern crate codegen;

pub mod binary;
pub mod bcsv;

mod codegeneration;


fn main() {
    // #[cfg(feature = "final")]
    // {
    //     let data = load_data().unwrap();
    //     println!("done");
    // }
    
    #[cfg(feature = "codegeneration")]
    codegeneration::bcsvgen::gen();
}

// #[cfg(feature = "final")]
// struct ACNHData {
//     bcsvs: bcsv::BCSVs
// }

// #[cfg(feature = "final")]
// fn load_data() -> Result<ACNHData, String> {
//     Ok(ACNHData {
//         bcsvs: bcsv::BCSVs::load()?
//     })
// }