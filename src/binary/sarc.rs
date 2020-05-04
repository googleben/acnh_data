use crate::binary::util::*;

pub const MAGIC: &str = "SARC";
pub const FS_NODE_HEADER_MAGIC: &str = "SFAT";
pub const FS_FILE_NAME_TABLE_MAGIC: &str = "SFNT";

#[derive(Debug)]
pub struct SARC {
    pub header: Header,
    pub fs_header: FSHeader,
    pub fs_nodes: Vec<FSNode>
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
    pub file_data: Vec<u8>
}

impl SARC {
    pub fn new<'a>(data: &[u8]) -> Result<SARC, &'a str> {
        let header: Header;
        let mut offset = 0;

        {
            let magic = get_utf8_str(data, offset, 4)?;
            offset += 4;
            if magic.as_str() != MAGIC { return Err("Incorrect magic number") }
            let _header_len = get_u16(&data, offset)?;
            offset += 2;
            let _byte_order_mark = get_u16(&data, offset)?;
            offset += 2;
            let file_size = get_u32(&data, offset)?;
            offset += 4;
            let data_start = get_u32(&data, offset)? as usize;
            offset += 4;
            let version = get_u16(&data, offset)?;
            offset += 2;
            let unk = get_u16(&data, offset)?;
            offset += 2;
            header = Header {file_size, data_start, version, unk}
        }

        let fs_header;
        {
            let magic = get_utf8_str(data, offset, 4)?;
            offset += 4;
            if magic.as_str() != FS_NODE_HEADER_MAGIC { 
                return Err("Incorrect SFAT magic number; expected SFAT section") 
            }
            let _header_len = get_u16(&data, offset)?;
            offset += 2;
            let num_nodes = get_u16(&data, offset)?;
            offset += 2;
            let hash_key = get_u32(&data, offset)?;
            offset += 4;
            fs_header = FSHeader { num_nodes, hash_key }
        }

        //the file name table starts right after the fs nodes, which are 16 bytes each
        //we'll need it while reading the nodes in so we can get their names as we read them in
        //rather than adding the names afterwards
        let file_name_table_off = offset + 16 * (fs_header.num_nodes as usize);
        //ensure that actually is the file name table
        {
            let magic = get_utf8_str(data, file_name_table_off, 4)?;
            match magic.as_str() {
                FS_FILE_NAME_TABLE_MAGIC => {},
                _ => return Err("Incorrect SFNT header; expected SFNT section")
            }
        }

        //now we read in the fs nodes
        let mut fs_nodes = vec!();
        for _ in 0..fs_header.num_nodes {
            let file_name_hash = get_u32(&data, offset)?;
            offset += 4;
            let file_attrs = get_u32(&data, offset)?;
            offset += 4;
            //the node contains the file's start relative to header.data_start
            //and same for the file's end, so we just convert that to an absolute offset
            let start_offset = get_u32(&data, offset)? as usize + header.data_start;
            offset += 4;
            let end_offset = get_u32(&data, offset)? as usize + header.data_start;
            offset += 4;
            
            // the flag 0x01000000 means that the node has a file name in the file name table
            // right now we require all files to have names
            if file_attrs & 0x01_00_00_00 == 0 { return Err("File nodes must have names") }

            // the file name is a null-terminated string indexed in the file name table by 
            // the low 16 bits of the attributes; indexing is 4-byte aligned so we multiply by 4
            let mut str_offset = file_name_table_off + 8 + (file_attrs & 0xFFFF) as usize * 4;
            let file_name = get_utf8_cstr(data, &mut str_offset)?;
            
            //and read in the file data as well
            let file_data = get_u8_arr(&data, start_offset, end_offset-start_offset)?;
            fs_nodes.push(FSNode {file_name_hash, file_name, file_attrs, file_data});
        }

        Ok(SARC {header, fs_header, fs_nodes})
    }
}

