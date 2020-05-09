use std::io;
use std::fs::read_dir;
use std::collections::{HashMap, HashSet};
use codegen::*;

use std::path::Path;

const ALLOW: &str = "#![allow(dead_code, non_camel_case_types, non_snake_case)]";

#[derive(Debug)]
struct Folder {
    sub_dirs: HashMap<String, Folder>,
    files: Vec<String>
}
impl Folder {
    fn new() -> Self {
        Folder { sub_dirs: HashMap::new(), files: vec!() }
    }
    fn gen_code(&self, name: &str, path_pre: &str) -> (Vec<Struct>, Vec<Impl>) {
        let mut structs = vec!();
        let mut impls = vec!();
        let s_name = format!("{}_{}", path_pre.replace('/', "_"), name);
        let s_name = s_name.trim_start_matches('_');
        let mut s = Struct::new(&s_name);
        s.vis("pub");
        let mut i = Impl::new(s.ty());
        let mut fun = Function::new("new");
        fun.vis("pub")
            .arg("msbts", "&HashMap<std::string::String, crate::binary::msbt::MSBT>")
            .ret("Self");
        fun.line(format!("{} {{", s_name));

        for (n, f) in self.sub_dirs.iter() {
            let (mut a, mut b) = f.gen_code(n, &format!("{}/{}", path_pre, name));
            structs.append(&mut a);
            impls.append(&mut b);
            let s_name = format!("{}_{}_{}", path_pre.replace('/', "_"), name, n);
            let s_name = s_name.trim_start_matches('_');
            s.field(&format!("pub {}", n), s_name);
            fun.line(format!("{}: {}::new(msbts),", n, s_name));
        }

        for n in self.files.iter() {
            s.field(&format!("pub {}", n.split('.').next().unwrap()), "File");
            let path = format!("{}/{}/{}", path_pre, name, n);
            fun.line(format!("{0}: File::new(\"{1}\", msbts.get(\"{1}\").unwrap()),", n.split('.').next().unwrap(), path));
        }
        fun.line("}");

        i.push_fn(fun);
        structs.push(s);
        impls.push(i);

        (structs, impls)
    }
}

#[allow(dead_code)]
pub fn gen() -> io::Result<()> {
    let msbt_dir = Path::new("input").join("Message");
    let mut lang_codes = HashSet::<String>::new();
    let mut archive_names = HashSet::<String>::new();
    for dir_ent in read_dir(&msbt_dir)? {
        let dir_ent = dir_ent?;
        let path = dir_ent.path();
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        if !file_name.ends_with(".sarc.zs") { continue; }
        let mut sp = file_name.split('.');
        let stem = sp.next().unwrap();
        sp = stem.split('_');
        let archive_name = sp.next().unwrap();
        let lang_code = sp.next().unwrap();
        lang_codes.insert(lang_code.to_string());
        archive_names.insert(archive_name.to_string());
    }
    
    //we need to pick a language code so we can read the MSBTs.
    //all languages should have the same structure
    let lang = "USen";
    
    let mut top_st = Struct::new("Language");
    top_st.vis("pub");
    let mut top_impl = Impl::new(top_st.ty());
    let mut top_fun = Function::new("new");
    top_fun.arg("msbts", "&HashMap<std::string::String, crate::binary::msbt::MSBT>")
        .ret("Self");
    top_fun.line("Language {");

    let mut structs = vec!();
    let mut impls = vec!();
    for name in archive_names {
        let mut folder = Folder::new();

        let file = std::fs::File::open((&msbt_dir).join(format!("{}_{}.sarc.zs", name, lang))).unwrap();

        let data = zstd::decode_all(file).unwrap();
        let sarc = crate::binary::sarc::SARC::new(&data).unwrap();
        for f in sarc.fs_nodes {
            let mut sp = f.file_name.split('/').peekable();
            let mut tmp_f = &mut folder;
            let mut curr = sp.next().unwrap().to_owned();
            while let Some(_) = sp.peek() {
                if tmp_f.sub_dirs.contains_key(&curr) {
                    tmp_f = tmp_f.sub_dirs.get_mut(&curr).unwrap();
                } else {
                    tmp_f.sub_dirs.insert(curr.clone(), Folder::new());
                    tmp_f = tmp_f.sub_dirs.get_mut(&curr).unwrap();
                }
                curr = sp.next().unwrap().to_owned();
            }
            tmp_f.files.push(curr);
        }

        
        let (mut s, mut i) = folder.gen_code(&name, "");
        structs.append(&mut s);
        impls.append(&mut i);
        top_st.field(&format!("pub {}", name), format!("Box<{}>", name));
        top_fun.line(format!("{0}: Box::new({0}::new(msbts)),", &name));
    }
    top_fun.line("}");
    top_impl.push_fn(top_fun);

    let mut msbts = Struct::new("MSBTs");
    msbts.vis("pub");
    let mut msbts_impl = Impl::new(msbts.ty());
    let mut msbts_fun = Function::new("new");
    msbts_fun.ret("std::io::Result<Self>")
        .vis("pub");
    msbts_fun.line("let data = read_msbts()?;");
    msbts_fun.line("Ok(MSBTs {");
    for lc in lang_codes {
        msbts.field(&format!("pub {}", lc), "Language");
        msbts_fun.line(format!("{0}: Language::new(data.get(\"{0}\").unwrap()),", lc));
    }
    msbts_fun.line("})");
    msbts_impl.push_fn(msbts_fun);

    let mut file = Scope::new();

    file.import("std::collections", "HashMap");
    file.import("crate::msbt", "*");

    file.push_struct(msbts);
    file.push_impl(msbts_impl);
    file.push_struct(top_st);
    file.push_impl(top_impl);
    for (s, i) in structs.into_iter().zip(impls) {
        file.push_struct(s);
        file.push_impl(i);
    }
    
    let out_p = Path::new("src").join("msbt").join("defs.rs");
    
    std::fs::write(out_p, format!("{}\n{}", ALLOW, file.to_string())).unwrap();

    Ok(())
}
