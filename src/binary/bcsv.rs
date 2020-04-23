fn get_u16(data: &Box<[u8]>, offset: usize) -> u16 {
    (data[offset] as u16) |
    (data[offset + 1] as u16) << 8
}

fn get_u32(data: &Box<[u8]>, offset: usize) -> u32 {
    (data[offset] as u32) | 
    (data[offset + 1] as u32) << 8 | 
    (data[offset + 2] as u32) << 16 |
    (data[offset + 3] as u32) << 24
}

#[derive(Debug)]
pub struct Header {
    pub num_rows: usize,
    pub row_size: u32,
    pub num_fields: usize,
    pub version: u8,
    pub jp_enum: u8,
    pub magic: u32,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub name_hash: u32,
    pub offset: u32,
    pub size: usize
}

#[derive(Debug)]
pub struct Row {
    pub entries: Box<[Box<[u8]>]>
}

#[derive(Debug)]
pub struct BCSV {
    pub header: Header,
    pub fields: Box<[FieldInfo]>,
    pub data: Box<[Row]>
}

//Magic is "VSCB"
pub const MAGIC: u32 = 0x42435356;

impl BCSV {
    
    pub fn new(data: Box<[u8]>) -> Result<BCSV, String> {
        //header takes 0x1B bytes
        if data.len() < 0x1B {
            return Err("File too small".to_owned());
        }
        let header: Header;
        let mut offset = 0;
        {
            let num_rows = get_u32(&data, offset) as usize;
            offset += 4;
            let row_size = get_u32(&data, offset);
            offset += 4;
            let num_fields = get_u16(&data, offset) as usize;
            offset += 2;
            let version = data[offset];
            offset += 1;
            let jp_enum = data[offset];
            offset += 1;
            let magic = get_u32(&data, offset);
            
            if magic != MAGIC {
                return Err("Incorrect magic number".to_owned());
            }
            //no need to increment offset since we set it after this
            header = Header {num_rows, num_fields, row_size, version, jp_enum, magic};
        }
        //fields always start at 0x1c
        offset = 0x1c;
        let mut fields: Vec<FieldInfo> = Vec::with_capacity(header.num_fields);
        for _ in 0..header.num_fields {
            let name_hash = get_u32(&data, offset);
            offset += 4;
            let field_offset = get_u32(&data, offset);
            offset += 4;
            fields.push(FieldInfo {name_hash, offset: field_offset, size: 0});
        }
        //field size is a computed value, not stored
        //the size of a field is either the next field's offset - its offset, 
        //or if it's the last field, the size of a row minus its offset
        for i in 0..(header.num_fields-1) {
            fields[i].size = (fields[i+1].offset - fields[i].offset) as usize;
        }
        if header.num_fields > 0 {
            fields[header.num_fields-1].size = (header.row_size - fields[header.num_fields-1].offset) as usize;
        }

        let mut rows: Vec<Row> = Vec::with_capacity(header.num_rows);
        for _ in 0..header.num_rows {
            let mut row: Vec<Box<[u8]>> = Vec::with_capacity(header.num_fields);
            if header.num_fields > 0 {
                offset += fields[0].offset as usize;
            }
            for f in &fields {
                let data: Vec<u8> = Vec::from(&data[offset..offset+f.size]);
                offset += f.size;
                row.push(data.into_boxed_slice());
            }
            rows.push(Row {entries: row.into_boxed_slice()});
        }
        
        Ok(BCSV {
            header, fields: fields.into_boxed_slice(), data: rows.into_boxed_slice()
        })
    }
}