pub struct FrameId { pub as_int: isize }
pub struct PageId { pub as_int: isize }
pub struct TxnId { pub as_int: isize }
pub struct Lsn { pub as_int: isize }
pub struct SlotOffset { pub as_int: isize }
pub struct Old { pub as_u_int: usize }

impl FrameId {
    pub fn new(id: isize) -> FrameId {
        FrameId { as_int: id }
    }
}

impl PageId {
    pub fn new(id: isize) -> PageId {
        PageId { as_int: id }
    }
}

impl TxnId {
    pub fn new(id: isize) -> TxnId {
        TxnId { as_int: id }
    }
}

pub const PAGE_SIZE: usize = 4096; // size of data page in byte