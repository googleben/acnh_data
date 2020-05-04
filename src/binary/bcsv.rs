use crate::binary::util::*;

#[derive(Debug)]
pub struct Header {
    pub num_rows: usize,
    pub row_size: u32,
    pub num_fields: usize,
    pub version: u8,
    pub jp_enum: u8,
}

#[derive(Debug)]
pub struct FieldInfo {
    pub name_hash: u32,
    pub offset: u32,
    pub size: usize
}

#[derive(Debug)]
pub struct Row {
    pub entries: Vec<Vec<u8>>
}

#[derive(Debug)]
pub struct BCSV {
    pub header: Header,
    pub fields: Vec<FieldInfo>,
    pub data: Vec<Row>
}

pub const MAGIC: &str = "VSCB";

impl BCSV {
    
    pub fn new<'a>(data: &[u8]) -> Result<BCSV, &'a str> {
        let header: Header;
        let mut offset = 0;
        {
            let num_rows = get_u32(data, offset)? as usize;
            offset += 4;
            let row_size = get_u32(&data, offset)?;
            offset += 4;
            let num_fields = get_u16(&data, offset)? as usize;
            offset += 2;
            let version = get_u8(data, offset)?;
            offset += 1;
            let jp_enum = get_u8(data, offset)?;
            offset += 1;
            let magic = get_utf8_str(data, offset, 4)?;
            //no need to increment offset because fields start at 0x1c
            
            if magic.as_str() != MAGIC {
                return Err("Incorrect magic number");
            }
            //no need to increment offset since we set it after this
            header = Header {num_rows, num_fields, row_size, version, jp_enum};
        }
        //fields always start at 0x1c
        offset = 0x1c;
        let mut fields: Vec<FieldInfo> = Vec::with_capacity(header.num_fields);
        for _ in 0..header.num_fields {
            let name_hash = get_u32(&data, offset)?;
            offset += 4;
            let field_offset = get_u32(&data, offset)?;
            offset += 4;
            fields.push(FieldInfo {name_hash, offset: field_offset, size: 0});
        }
        //field size is a computed value, not stored
        //the size of a field is either the next field's offset - its offset, 
        //or if it's the last field, the size of a row minus its offset
        for i in 0..(header.num_fields-1) {
            fields[i].size = (fields[i+1].offset - fields[i].offset) as usize;
        }
        //take care of the last header now
        if header.num_fields > 0 {
            fields[header.num_fields-1].size = (header.row_size - fields[header.num_fields-1].offset) as usize;
        }

        // now we read in the actual data
        let mut rows: Vec<Row> = Vec::with_capacity(header.num_rows);
        for _ in 0..header.num_rows {
            let mut row: Vec<Vec<u8>> = Vec::with_capacity(header.num_fields);
            // the data may not start at the beginning of the row, so make sure
            // we're starting where the data starts
            if header.num_fields > 0 {
                offset += fields[0].offset as usize;
            }
            
            for f in &fields {
                row.push(get_u8_arr(data, offset, f.size)?);
                offset += f.size;
            }
            rows.push(Row {entries: row});
        }
        
        Ok(BCSV {
            header, fields, data: rows
        })
    }
}