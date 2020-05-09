use std::collections::HashMap;
use crate::binary::msbt::MSBT;
use std::path::Path;
use std::fs::read_dir;

pub mod defs;
pub use defs::{MSBTs, Language};

fn read_msbts() -> std::io::Result<HashMap<String, HashMap<String, MSBT>>> {
    let mut ans: HashMap<String, HashMap<String, MSBT>> = HashMap::new();
    let msbt_dir = Path::new("input").join("Message");
    //let mut lang_codes = HashSet::<String>::new();
    //let mut archive_names = HashSet::<String>::new();
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
        if !ans.contains_key(lang_code) {
            ans.insert(lang_code.to_string(), HashMap::new());
        }
        let map = ans.get_mut(lang_code).unwrap();

        let file = std::fs::File::open(&path)?;

        let data = zstd::decode_all(file)?;
        let sarc = crate::binary::sarc::SARC::new(&data).unwrap();
        for f in sarc.fs_nodes {
            let res = crate::binary::msbt::MSBT::new(&f.file_data);
            if let Err(e) = res {
                println!("{}", e);
            }
            map.insert(format!("/{}/{}", archive_name, f.file_name), res.unwrap());
        }
    }
    Ok(ans)
}

#[derive(Debug)]
pub struct Entry {
    pub contents: Vec<EntryComponent>,
    pub attribute: Vec<u8>
}

impl Entry {
    pub fn strip_tags(&self) -> String {
        self.contents.iter()
            .map(|m| if let EntryComponent::Text(s) = m {s} else {""})
            .collect()
    }
}

fn parse_entry(text: &[u16]) -> Vec<EntryComponent> {
    let mut ans = vec!();
    let mut ind = 0;
    while ind < text.len() {
        if text[ind] == 0xE {
            ind += 1;
            let id = text[ind];
            ind += 1;
            let num_args_ex = text[ind+1] as usize;
            let mut args = Vec::with_capacity(num_args_ex + 1);
            args.push(text[ind]);
            ind += 2;
            for _ in 0..num_args_ex {
                args.push(text[ind]);
                ind += 1;
            }
            ans.push(EntryComponent::Tag { id, args, is_compact: false });
        } else if text[ind] == 0xF {
            ind += 1;
            let id = text[ind];
            ind += 1;
            let arg = text[ind];
            ind += 1;
            let args = vec![arg];
            ans.push(EntryComponent::Tag { id, args, is_compact: true });
        } else {
            let mut str_raw = vec!();
            while ind < text.len() && text[ind] | 1 != 0xF {
                str_raw.push(text[ind]);
                ind += 1;
            }
            ans.push(EntryComponent::Text(String::from_utf16_lossy(&str_raw)));
        }
    }
    ans
}

pub struct File {
    pub path: String,
    pub entries: HashMap<String, Entry>
}

impl File {
    pub fn new(path: &str, data: &MSBT) -> Self {
        let mut entries = HashMap::new();
        for l in data.label_hash_table.iter().flatten() {
            let attribute = data.attributes[l.item_index as usize].clone();
            let text = &data.messages_raw[l.item_index as usize];
            let contents = parse_entry(&text);
            entries.insert(l.label.clone(), Entry {attribute, contents});
        }
        File {path: path.to_owned(), entries}
    }
}

pub trait Folder {
    fn has_folders(&self) -> bool;
    fn has_files(&self) -> bool;
    fn get_folders<'a>(&self) -> Vec<&'a dyn Folder>;
    fn get_files<'a>(&self) -> Vec<&'a File>;
    fn get_files_rec<'a>(&self) -> Vec<&'a File> {
        let mut ans = vec!();
        ans.append(&mut self.get_files());
        for f in self.get_folders() {
            ans.append(&mut f.get_files_rec());
        }
        ans
    }
}

#[derive(Debug)]
pub enum EntryComponent {
    Text(String),
    Tag {id: u16, args: Vec<u16>, is_compact: bool}
}