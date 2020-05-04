
pub fn read_u8(data: &[u8]) -> u8 {
    data[0]
}

pub fn read_u16(data: &[u8]) -> u16 {
    (data[0] as u16) |
    (data[1] as u16) << 8
}

pub fn read_u32(data: &[u8]) -> u32 {
    (data[0] as u32) |
    (data[1] as u32) << 8 |
    (data[2] as u32) << 16 |
    (data[3] as u32) << 24
}

#[allow(non_snake_case)]
pub fn read_String(data: &[u8]) -> String {
    match String::from_utf8(data.to_vec()) {
        Ok(s) => s.trim_end_matches("\0").to_owned(),
        Err(_) => data.iter().map(|b| format!("\\u{{{0:2X}}}", b).to_owned()).collect()
    }
}
