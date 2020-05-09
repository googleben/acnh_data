fn ensure_has<'a>(data: &[u8], offset: usize, required: usize) -> Result<(), &'a str> {
    if offset + required <= data.len() { Ok(()) } else { Err("Unexpected end of input data") }
}

pub fn get_u8<'a>(data: &[u8], offset: usize) -> Result<u8, &'a str> {
    ensure_has(data, offset, 1)?;
    Ok(data[offset])
}

pub fn get_u16<'a>(data: &[u8], offset: usize) -> Result<u16, &'a str> {
    ensure_has(data, offset, 2)?;
    Ok((data[offset] as u16) |
    (data[offset+1] as u16) << 8)
}

pub fn get_u32<'a>(data: &[u8], offset: usize) -> Result<u32, &'a str> {
    ensure_has(data, offset, 4)?;
    Ok((data[offset] as u32) |
    (data[offset+1] as u32) << 8 |
    (data[offset+2] as u32) << 16 |
    (data[offset+3] as u32) << 24)
}

pub fn get_u8_arr<'a>(data: &[u8], offset: usize, len: usize) -> Result<Vec<u8>, &'a str> {
    ensure_has(data, offset, len)?;
    Ok(data[offset..offset+len].to_vec())
}

pub fn get_utf8_str<'a>(data: &[u8], offset: usize, len: usize) -> Result<String, &'a str> {
    ensure_has(data, offset, len)?;
    match String::from_utf8((&data[offset..offset + len]).to_vec()) {
        Ok(s) => Ok(s),
        Err(e) => {
            let s = format!("Invalid UTF-8 {} {:?}", e, &data[offset..offset + len]);
            Err(Box::leak(s.into_boxed_str()))
        }
    }
}

pub fn get_utf8_cstr<'a>(data: &[u8], offset: &mut usize) -> Result<String, &'a str> {
    let mut str_raw = vec!();
    loop {
        match get_u8(data, *offset)? {
            0 => break,
            x => str_raw.push(x)
        }
        *offset += 1;
    }
    match String::from_utf8(str_raw) {
        Ok(s) => Ok(s),
        Err(_) => Err("Invalid UTF-8")
    }
}