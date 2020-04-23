use std::io::*;
use std::fs::File;
use std::collections::HashMap;
use crc::{crc32};

#[allow(dead_code)]
type EnumsFile = HashMap<String, HashMap<String, Vec<Vec<String>>>>;

#[allow(dead_code)]
pub fn gen() {
    let mut f = File::open("enums.json").unwrap();
    let d: EnumsFile = serde_json::from_reader(f).unwrap();
    let mut out_opt = std::fs::OpenOptions::new();
    out_opt.create(true).truncate(true).write(true);
    let mut out = out_opt.open(std::path::Path::new("src").join("bcsv").join("defs.rs").with_extension("rs")).unwrap();
    out.write_fmt(format_args!("#![allow(non_camel_case_types, non_snake_case)]\n\n")).unwrap();
    let mut defined_enums = vec!();
    for (filename, val) in d.iter() {
        
        let mut data: Vec<u8> = vec!();
        f = File::open(std::path::Path::new("bcsv").join(filename)).unwrap();
        f.read_to_end(&mut data).unwrap();
        let bcsv = crate::binary::bcsv::BCSV::new(data.into_boxed_slice()).unwrap();
        out.write_fmt(format_args!("pub struct {} {{\n", std::path::Path::new(filename).file_stem().unwrap().to_str().unwrap())).unwrap();
        for field in bcsv.fields.into_iter() {
            let hash_str = field.name_hash.to_string();
            if val.contains_key(&hash_str) {
                out.write_fmt(format_args!("    pub enum_{0}: Enum{0},\n", hash_str)).unwrap();
            } else {
                let type_ = match field.size {
                    1 => "u8",
                    2 => "u16",
                    4 => "u32",
                    _ => "String"
                };
                out.write_fmt(format_args!("    pub {0}_{1}: {0},\n", type_, hash_str)).unwrap();
            }
        }
        out.write_fmt(format_args!("}}\n\n")).unwrap();

        out.write_fmt(format_args!("impl {0} {{\n    pub fn new(data: Box<[Box<[u8]>]>) -> {0} {{\n        {0} {{\n", std::path::Path::new(filename).file_stem().unwrap().to_str().unwrap())).unwrap();
        let mut ind = 0;
        for field in bcsv.fields.into_iter() {
            let hash_str = field.name_hash.to_string();
            if val.contains_key(&hash_str) {
                out.write_fmt(format_args!("            enum_{0}: unsafe{{std::mem::transmute::<u32,Enum{0}>(crate::bcsv::utils::read_u32(&data[{1}]))}},\n", hash_str, ind)).unwrap();
            } else {
                let type_ = match field.size {
                    1 => "u8",
                    2 => "u16",
                    4 => "u32",
                    _ => "String"
                };
                out.write_fmt(format_args!("            {0}_{1}: crate::bcsv::utils::read_{0}(&data[{2}]),\n", type_, hash_str, ind)).unwrap();
            }
            ind += 1;
        }
        out.write_fmt(format_args!("        }}\n    }}\n}}\n\n")).unwrap();

        for (hash_str, dat) in val.iter() {
            if defined_enums.contains(&hash_str) {continue;}
            defined_enums.push(hash_str);
            out.write_fmt(format_args!("#[repr(C)]\npub enum Enum{} {{\n", hash_str)).unwrap();
            for field in &dat[0] {
                let hash = crc32::checksum_ieee(&field.bytes().collect::<Vec<u8>>());
                let prefix = if field.chars().next().unwrap().is_ascii_digit() {"_"} else {""};
                out.write_fmt(format_args!("    {}{} = {},\n", prefix, field, hash)).unwrap();
            }
            out.write_fmt(format_args!("}}\n\n")).unwrap();
        }
    }
}

pub trait FromBCSV {
    fn from_bcsv(bcsv: crate::binary::bcsv::BCSV) -> Self;
}