#[derive(Debug)]
pub struct MSBTHeader {
    unk1: u16,
    unk2: u16,
    num_sections: u16,
    unk3: u16,
    file_size: usize
}

#[derive(Debug)]
pub struct MSBT {
    header: MSBTHeader,
    label_hash_table: Box<[Box<[Label]>]>,
    messages: Box<[String]>,
    attributes: Box<[Box<[u8]>]>
}

#[derive(Debug)]
pub struct Label {
    label: String,
    item_index: u32
}

fn get_u16(data: &[u8], offset: usize) -> u16 {
    (data[offset] as u16) |
    (data[offset+1] as u16) << 8
}

fn get_u32(data: &[u8], offset: usize) -> u32 {
    (data[offset] as u32) |
    (data[offset+1] as u32) << 8 |
    (data[offset+2] as u32) << 16 |
    (data[offset+3] as u32) << 24
}

fn push_unicode_escape(vec: &mut Vec<u16>, num: u16) {
    vec.push('\\' as u16);
    vec.push('u' as u16);
    vec.push('{' as u16);
    let str = format!("{0:X}", num);
    for &ch in str.as_bytes() {
        vec.push(ch as u16);
    }
    vec.push('}' as u16);
}

impl MSBT {
    pub fn new<'a>(data: Box<[u8]>) -> Result<MSBT, &'a str> {
        let mut offset = 0;
        let header: MSBTHeader;
        {
            let mut magic = [0; 8];
            for i in 0..8 { magic[i] = data[i]; }
            offset += 8;
            let order_mark = get_u16(&data, offset);
            offset += 2;
            let unk1 = get_u16(&data, offset);
            offset += 2;
            let unk2 = get_u16(&data, offset);
            offset += 2;
            let num_sections = get_u16(&data, offset);
            offset += 2;
            let unk3 = get_u16(&data, offset);
            offset += 2;
            let file_size = get_u32(&data, offset) as usize;
            offset += 4;
            //10 bytes of padding
            offset += 10;
            header = MSBTHeader {unk1, unk2, num_sections, unk3, file_size};
        }
        let mut label_hash_table = vec!();
        let mut messages = vec!();
        let mut attributes = vec!();
        for _ in 0..header.num_sections {

            //align offset with 16-byte boundaries
            {
                let dist = offset % 16;
                if dist > 0 { offset += 16 - dist }
            }
            let mut section_name_raw = [0; 4];
            for i in 0..4 { section_name_raw[i] = data[offset+i] }
            let section_name = match String::from_utf8(section_name_raw.to_vec()) {
                Ok(s) => s,
                _ => return Err("Section header was invalid UTF-8")
            };
            let section_name = section_name.as_str();
            offset += 4;
            let section_size = get_u32(&data, offset) as usize;
            offset += 4;
            //8 bytes of padding
            offset += 8;
            let sec_start = offset;
            match section_name {
                "LBL1" => {
                    let num_slots = get_u32(&data, offset);
                    offset += 4;
                    for _ in 0..num_slots {
                        let num_labels = get_u32(&data, offset);
                        offset += 4;
                        let labels_off = get_u32(&data, offset) as usize;
                        offset += 4;
                        let mut labels = vec!();
                        let mut tmp_offset = sec_start + labels_off;
                        for _ in 0..num_labels {
                            let str_len = data[tmp_offset];
                            tmp_offset += 1;
                            let str = match String::from_utf8((&data[tmp_offset..tmp_offset + str_len as usize]).to_vec()) {
                                Ok(s) => s,
                                _ => return Err("Invalid UTF-8 in label")
                            };
                            tmp_offset += str_len as usize;
                            let item_index = get_u32(&data, tmp_offset);
                            tmp_offset += 4;
                            labels.push(Label {label: str, item_index});
                        }
                        label_hash_table.push(labels.into_boxed_slice());
                    }
                },
                "TXT2" => {
                    let num_messages = get_u32(&data, offset);
                    offset += 4;
                    for _ in 0..num_messages {
                        let message_off = get_u32(&data, offset) as usize;
                        offset += 4;
                        let mut str_raw = vec!();
                        let mut tmp_offset = message_off + sec_start;
                        loop {
                            let ch = get_u16(&data, tmp_offset);
                            tmp_offset += 2;
                            if ch == 0 { break }
                            if ch == 0xE {
                                push_unicode_escape(&mut str_raw, 0xE);
                                let fnum = get_u16(&data, tmp_offset);
                                tmp_offset += 2;
                                push_unicode_escape(&mut str_raw, fnum);
                                let arg1 = get_u16(&data, tmp_offset);
                                tmp_offset += 2;
                                push_unicode_escape(&mut str_raw, arg1);
                                let num_args = get_u16(&data, tmp_offset) / 2;
                                tmp_offset += 2;
                                push_unicode_escape(&mut str_raw, num_args);
                                for _ in 0..num_args {
                                    push_unicode_escape(&mut str_raw, get_u16(&data, tmp_offset));
                                    tmp_offset += 2;
                                }
                            } else if ch == 0xF {
                                push_unicode_escape(&mut str_raw, 0xF);
                                let fnum = get_u16(&data, tmp_offset);
                                tmp_offset += 2;
                                push_unicode_escape(&mut str_raw, fnum);
                                let arg1 = get_u16(&data, tmp_offset);
                                tmp_offset += 2;
                                push_unicode_escape(&mut str_raw, arg1);
                            } else {
                                str_raw.push(ch);
                            }
                        }
                        let str = match String::from_utf16(&str_raw) {
                            Ok(s) => s,
                            _ => return Err("Invalid UTF-16 in text")
                        };
                        messages.push(str);
                    }
                },
                "ATR1" => {
                    let num_attrs = get_u32(&data, offset);
                    offset += 4;
                    let attr_bytes = get_u32(&data, offset) as usize;
                    offset += 4;
                    for _ in 0..num_attrs {
                        attributes.push(data[offset..(offset+attr_bytes)].to_vec().into_boxed_slice());
                        offset += attr_bytes;
                    }
                }
                _ => {}
            }
            offset = sec_start + section_size;
        }
        Ok(MSBT { header, label_hash_table: label_hash_table.into_boxed_slice(), messages: messages.into_boxed_slice(), attributes: attributes.into_boxed_slice() })
    }
}


