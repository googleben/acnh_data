pub struct Attrs {
    num_attrs: u32,
    attr_len: u32,
    attrs: Vec<Box<[u8]>>
}


pub struct Text {
    offsets: Vec<u32>,
    messages: Vec<String>
}

pub struct Labels {
    slots: Vec<TableSlot>,
    labels: Vec<Label>
}

#[repr(C)]
pub struct TableSlot {
    num_labels: u32,
    label_offset: u32
}

#[repr(C)]
pub struct Label {
    str_len: u8,
    label: String,
    item_index: u32
}


pub struct MSBTHeader {
    magic: u64,
    order_mark: u16,
    uk1: u16,
    uk2: u16,
    num_sections: u16,
    uk3: u16,
    file_size: usize
}

pub struct MSBT {
    header: MSBTHeader,
}

pub enum Block {
    LBL1 {size: usize, labels: Labels}
}

pub enum BlockType {
    LBL1, ATR1, TXT2
}

pub struct BlockHeader {
    block_type: BlockType,
    size: usize //size without header
}