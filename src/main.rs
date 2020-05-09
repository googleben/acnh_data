extern crate serde;
extern crate serde_json;
extern crate crc;
extern crate zstd;
extern crate codegen;

pub mod binary;
pub mod bcsv;
pub mod msbt;

mod codegeneration;


fn main() {
    // #[cfg(feature = "final")]
    // {
    //     let data = msbt::MSBTs::new().unwrap();
    //     println!("{:#?}", data.USen.String.Item.STR_ItemName_31_Fish.entries);
    //     let data = load_data().unwrap();
    //     println!("done");
    // }

    // let file = std::fs::File::open("input\\message\\TalkSNpc_USen.sarc.zs").unwrap();

    // let data = zstd::decode_all(file).unwrap();
    // let sarc = binary::sarc::SARC::new(&data).unwrap();
    // for f in sarc.fs_nodes {
    //     println!("{}", f.file_name);
    // }
    
    
    #[cfg(feature = "codegeneration")]
    {
        // codegeneration::bcsvgen::gen().unwrap();
        codegeneration::msbtgen::gen().unwrap();
    }
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