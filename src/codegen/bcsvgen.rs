use std::io::*;
use std::fs::File;
use std::collections::HashMap;
use crc::{crc32};

//expected JSON structure of enums.json
/**
 * { 
 *   [filename: string]: 
 *     { 
 *       [enumname: string]: string[][] // [string[] english enum, string[] japanese enum]
 *     } 
 * }
**/
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
    out.write(b"use crate::bcsv::BCSVRow;\n\n").unwrap();
    let empty = HashMap::new();

    out.write(b"pub struct BCSVs {\n").unwrap();
    for dirent in std::fs::read_dir("bcsv").unwrap() {
        let dirent = dirent.unwrap();
        if !dirent.file_type().unwrap().is_file() {
            panic!("Expected only files in bcsv dir");
        }
        let osfilename = dirent.file_name();
        let filename = osfilename.to_str().unwrap();
        
        let struct_name = std::path::Path::new(filename).file_stem().unwrap().to_str().unwrap();
        out.write_fmt(format_args!("    pub {0}: Box<[{0}Row]>,\n", struct_name)).unwrap();
    }
    out.write(b"}\n\n").unwrap();

    out.write(b"impl BCSVs {\n    pub fn load() -> Result<BCSVs, String> {\n        Ok(BCSVs {\n").unwrap();
    for dirent in std::fs::read_dir("bcsv").unwrap() {
        let dirent = dirent.unwrap();
        if !dirent.file_type().unwrap().is_file() {
            panic!("Expected only files in bcsv dir");
        }
        let osfilename = dirent.file_name();
        let filename = osfilename.to_str().unwrap();
        let ospath = dirent.path().into_os_string();
        let path = ospath.to_str().unwrap();
        
        let struct_name = std::path::Path::new(filename).file_stem().unwrap().to_str().unwrap();
        out.write_fmt(format_args!("            {0}: {0}Row::make_rows(crate::bcsv::read_bcsv(r#\"{1}\"#)?),\n", struct_name, path)).unwrap();
    }
    out.write(b"        })\n    }\n}\n\n").unwrap();
    
    for dirent in std::fs::read_dir("bcsv").unwrap() {
        let dirent = dirent.unwrap();
        if !dirent.file_type().unwrap().is_file() {
            panic!("Expected only files in bcsv dir");
        }
        let osfilename = dirent.file_name();
        let filename = osfilename.to_str().unwrap();
        let val = d.get(filename).unwrap_or(&empty);
        let mut data: Vec<u8> = vec!();
        f = File::open(std::path::Path::new("bcsv").join(filename)).unwrap();
        f.read_to_end(&mut data).unwrap();
        let bcsv = crate::binary::bcsv::BCSV::new(data.into_boxed_slice()).unwrap();
        let struct_name = format!("{}Row", std::path::Path::new(filename).file_stem().unwrap().to_str().unwrap());
        out.write_fmt(format_args!("pub struct {} {{\n", struct_name)).unwrap();
        for field in bcsv.fields.into_iter() {
            let hash_str = field.name_hash.to_string();
            if val.contains_key(&hash_str) {
                out.write_fmt(format_args!("    pub enum_{0}: Enum{0},\n", hash_str)).unwrap();
            } else {
                let type_ = match field.size {
                    1 => "u8",
                    2 | 3 => "u16",
                    4 => "u32",
                    _ => "String"
                };
                out.write_fmt(format_args!("    pub {0}_{1}: {0},\n", type_, hash_str)).unwrap();
            }
        }
        out.write_fmt(format_args!("}}\n\n")).unwrap();

        out.write_fmt(format_args!("impl BCSVRow for {0} {{\n    type T = {0};\n    fn new(data: &Box<[Box<[u8]>]>) -> {0} {{\n        {0} {{\n", struct_name)).unwrap();
        let mut ind = 0;
        for field in bcsv.fields.into_iter() {
            let hash_str = field.name_hash.to_string();
            if val.contains_key(&hash_str) {
                out.write_fmt(format_args!("            enum_{0}: unsafe{{std::mem::transmute_copy::<u32,Enum{0}>(&crate::bcsv::utils::read_u32(&data[{1}]))}},\n", hash_str, ind)).unwrap();
            } else {
                let type_ = match field.size {
                    1 => "u8",
                    2 | 3 => "u16",
                    4 => "u32",
                    _ => "String"
                };
                out.write_fmt(format_args!("            {0}_{1}: crate::bcsv::utils::read_{0}(&data[{2}]),\n", type_, hash_str, ind)).unwrap();
            }
            ind += 1;
        }

        out.write(b"        }\n    }\n}\n\n").unwrap();

        for (hash_str, dat) in val.iter() {
            if defined_enums.contains(&hash_str) {continue;}
            defined_enums.push(hash_str);
            out.write_fmt(format_args!("#[repr(C)]\npub enum Enum{} {{\n", hash_str)).unwrap();
            for field in &dat[0] {
                let hash = crc32::checksum_ieee(&field.bytes().collect::<Vec<u8>>());
                let prefix = if field.chars().next().unwrap().is_ascii_digit() {"_"} else {""};
                out.write_fmt(format_args!("    {}{} = {},\n", prefix, field, hash)).unwrap();
            }
            out.write(b"}\n\n").unwrap();

            out.write_fmt(format_args!("impl Enum{} {{\n    pub fn to_str(&self) -> &str {{\n        match self {{\n", hash_str)).unwrap();
            for field in &dat[0] {
                let prefix = if field.chars().next().unwrap().is_ascii_digit() {"_"} else {""};
                out.write_fmt(format_args!("            Enum{}::{}{} => \"{2}\",\n", hash_str, prefix, field)).unwrap();
            }
            out.write(b"        }\n    }\n}\n\n").unwrap();
        }
    }

    
}

pub trait FromBCSV {
    fn from_bcsv(bcsv: crate::binary::bcsv::BCSV) -> Self;
}