use std::io;
use std::io::Read;
use std::fs::{File, read_dir};
use std::collections::HashMap;
use crc::crc32;
use codegen::*;

use std::path::Path;

const ALLOW: &str = "#![allow(dead_code, non_camel_case_types, non_snake_case, clippy::unreadable_literal)]";

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
pub fn gen() -> io::Result<()> {
    let enums_f = File::open("enums.json").unwrap();
    let enums_json: EnumsFile = serde_json::from_reader(enums_f).unwrap();

    let mut file = Scope::new();
    file.import("crate::bcsv", "BCSVRow");
    file.import("std::path", "Path");
    file.import("std::convert", "TryFrom");
    file.import("crate::bcsv::util", "*");
    file.import("crate::bcsv", "read_bcsv");
    let mut bcsvs = Struct::new("BCSVs");
    let mut bcsvs_impl = Impl::new("BCSVs");
    let mut bcsvs_load = Function::new("load");
    bcsvs_load.generic("'a").ret("Result<BCSVs, &'a str>");
    bcsvs_load.line("let dir = Path::new(\"input\").join(\"Bcsv\");");
    bcsvs_load.line("Ok(BCSVs {");
    
    let mut enums = HashMap::new();
    let mut impls = vec!();
    for (_, fenums) in enums_json.iter() {
        for (hash, vals) in fenums.iter() {
            if enums.contains_key(hash) { continue; }
            let rust_name = format!("Enum{}", hash);
            let mut enum_ = Enum::new(&rust_name);

            let mut impl_from_u32 = Impl::new(enum_.ty());
            impl_from_u32.impl_trait("TryFrom<u32>");
            impl_from_u32.associate_type("Error", "&'static str");
            let mut from_u32 = Function::new("try_from");
            from_u32.arg("value", "u32").ret("Result<Self, Self::Error>");
            from_u32.line("match value {");

            let mut impl_from_self = Impl::new("u32");
            impl_from_self.impl_trait(format!("From<{}>", &rust_name));
            let mut from_self = Function::new("from");
            from_self.arg("value", &rust_name).ret("u32");
            from_self.line("match value {");

            enum_.vis("pub");
            for field in &vals[0] {
                let hash = crc32::checksum_ieee(&field.bytes().collect::<Vec<u8>>());
                let prefix = if field.chars().next().unwrap().is_ascii_digit() {"_"} else {""};
                let var_name = format!("{}{}", prefix, field);
                enum_.new_variant(&var_name);
                from_u32.line(format!("{} => Ok(Self::{}),", hash, var_name));
                from_self.line(format!("{}::{} => {},", &rust_name, var_name, hash));
            }
            from_u32.line("_ => Err(\"Input was not a variant of this enum\")");
            from_u32.line("}");
            impl_from_u32.push_fn(from_u32);
            impls.push(impl_from_u32);

            from_self.line("}");
            impl_from_self.push_fn(from_self);
            impls.push(impl_from_self);
            
            enums.insert(hash, enum_);
        }
    }

    let mut structs = vec!();
    let bcsvs_dir = Path::new("input").join("Bcsv");
    for dirent in read_dir(bcsvs_dir)? {
        let dirent = dirent?;
        if !dirent.file_type()?.is_file() {
            continue;
        }

        let mut data: Vec<u8> = vec!();
        let mut f = File::open(dirent.path()).unwrap();
        f.read_to_end(&mut data).unwrap();
        let bcsv = crate::binary::bcsv::BCSV::new(&data).unwrap();

        let filename = dirent.file_name();
        let filename = filename.to_str().unwrap();
        let file_stem = Path::new(filename).file_stem().unwrap().to_str().unwrap();
        let struct_name = format!("{}Row", file_stem);
        let mut struct_ = Struct::new(&struct_name);
        struct_.vis("pub");
        let mut s_impl = Impl::new(struct_.ty());
        s_impl.impl_trait("BCSVRow").associate_type("T", struct_.ty());
        let mut new = Function::new("new");
        new.arg("data", "&[Vec<u8>]").ret(struct_.ty());
        new.line(format!("{} {{", struct_name));

        let mut ind = 0;
        for field in bcsv.fields.iter() {
            let hash_str = field.name_hash.to_string();
            let v_name;
            let v_type;
            if enums.contains_key(&hash_str) {
                v_name = format!("enum_{}", hash_str);
                v_type = format!("Enum{}", hash_str);
                new.line(format!("{}: {}::try_from(read_u32(&data[{}])).unwrap(),", v_name, v_type, ind));
            } else {
                v_type = match field.size {
                    1 => "u8",
                    2 | 3 => "u16",
                    4 => "u32",
                    _ => "String"
                }.to_owned();
                v_name = format!("{}_{}", v_type, hash_str);
                new.line(format!("{}: read_{}(&data[{}]),", v_name, v_type, ind));
            }
            struct_.field(&v_name, &v_type);
            ind += 1;
        }
        new.line("}");
        bcsvs.field(&file_stem, Type::new("Vec").generic(struct_.ty()).clone());
        bcsvs_load.line(format!("{}: {}::make_rows(read_bcsv(dir.join(\"{}\"))?),", file_stem, struct_name, filename));
        s_impl.push_fn(new);
        impls.push(s_impl);
        structs.push(struct_);
    }

    bcsvs_load.line("})");
    bcsvs_impl.push_fn(bcsvs_load);

    file.push_struct(bcsvs)
        .push_impl(bcsvs_impl);

    for s in structs {
        file.push_struct(s);
    }
    for (_, e) in enums {
        file.push_enum(e);
    }
    for i in impls {
        file.push_impl(i);
    }

    let out_p = Path::new("src").join("bcsv").join("defs.rs");
    
    std::fs::write(out_p, format!("{}\n{}", ALLOW, file.to_string())).unwrap();
    Ok(())
}

pub trait FromBCSV {
    fn from_bcsv(bcsv: crate::binary::bcsv::BCSV) -> Self;
}