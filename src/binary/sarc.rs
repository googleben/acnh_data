pub const MAGIC: &str = "SARC";
pub const FS_NODE_HEADER_MAGIC: &str = "SFAT";
pub const FS_FILE_NAME_TABLE_MAGIC: &str = "SFNT";

#[derive(Debug)]
pub struct SARC {
    pub header: Header,
    pub fs_header: FSHeader,
    pub fs_nodes: Box<[FSNode]>,
}

#[derive(Debug)]
pub struct Header {
    pub file_size: u32,
    pub data_start: usize,
    pub version: u16,
    pub unk: u16
}

#[derive(Debug)]
pub struct FSHeader {
    pub num_nodes: u16,
    pub hash_key: u32
}

#[derive(Debug)]
pub struct FSNode {
    pub file_name_hash: u32,
    pub file_name: String,
    pub file_attrs: u32,
    pub file_data: Box<[u8]>
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

impl SARC {
    pub fn new<'a>(data: Box<[u8]>) -> Result<SARC, &'a str> {
        let header: Header;
        let mut offset = 0;
        {
            let mut magic = [0; 4];
            for i in 0..4 { magic[i] = data[i]; }
            offset += 4;
            match String::from_utf8_lossy(&magic) {
                std::borrow::Cow::Borrowed(MAGIC) => {},
                _ => return Err("Incorrect magic number")
            }
            let _header_len = get_u16(&data, offset);
            offset += 2;
            let _byte_order_mark = get_u16(&data, offset);
            offset += 2;
            let file_size = get_u32(&data, offset);
            offset += 4;
            let data_start = get_u32(&data, offset) as usize;
            offset += 4;
            let version = get_u16(&data, offset);
            offset += 2;
            let unk = get_u16(&data, offset);
            offset += 2;
            header = Header {file_size, data_start, version, unk}
        }

        let fs_header;
        {
            let mut magic = [0; 4];
            for i in 0..4 { magic[i] = data[offset+i]; }
            offset += 4;
            match String::from_utf8_lossy(&magic) {
                std::borrow::Cow::Borrowed(FS_NODE_HEADER_MAGIC) => {},
                _ => return Err("Expected SFAT section")
            }
            let _header_len = get_u16(&data, offset);
            offset += 2;
            let num_nodes = get_u16(&data, offset);
            offset += 2;
            let hash_key = get_u32(&data, offset);
            offset += 4;
            fs_header = FSHeader {num_nodes, hash_key}
        }

        let file_name_table_off = offset + 16 * (fs_header.num_nodes as usize);
        {
            let mut magic = [0; 4];
            for i in 0..4 { magic[i] = data[file_name_table_off+i]; }
            match String::from_utf8_lossy(&magic) {
                std::borrow::Cow::Borrowed(FS_FILE_NAME_TABLE_MAGIC) => {},
                _ => return Err("Expected SFNT section")
            }
        }

        let mut fs_nodes = vec!();
        for _ in 0..fs_header.num_nodes {
            let file_name_hash = get_u32(&data, offset);
            offset += 4;
            let file_attrs = get_u32(&data, offset);
            offset += 4;
            let start_offset_from_data_start = get_u32(&data, offset) as usize;
            offset += 4;
            let end_offset_from_data_start = get_u32(&data, offset) as usize;
            offset += 4;
            let mut file_name_raw = vec!();
            // the file name is a null-terminated string indexed in the file name table by the low 16 bits
            // of the attributes; indexing is 4-byte aligned so we multiply by 4
            let mut tmp_offset = file_name_table_off + 8 + (file_attrs & 0xFFFF) as usize * 4;
            loop {
                match data[tmp_offset] {
                    0 => break,
                    x => file_name_raw.push(x)
                }
                tmp_offset += 1;
            }
            let file_name = match String::from_utf8(file_name_raw) {
                Ok(s) => s,
                Err(_) => return Err("Filename was not valid UTF-8")
            };
            let mut file_data = vec![0; end_offset_from_data_start-start_offset_from_data_start];
            file_data.copy_from_slice(&data[(header.data_start+start_offset_from_data_start)..(header.data_start+end_offset_from_data_start)]);
            fs_nodes.push(FSNode {file_name_hash, file_name, file_attrs, file_data: file_data.into_boxed_slice()});
        }

        Ok(SARC {header, fs_header, fs_nodes: fs_nodes.into_boxed_slice()})
    }
}

