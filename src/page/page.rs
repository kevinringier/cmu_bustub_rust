use crate::include::common::{PAGE_SIZE, PageId};

pub trait Page {
    fn get_data(&self) -> [u8; PAGE_SIZE];

    fn get_page_id(&self) -> PageId;

    fn get_pin_count(&self) -> isize;

    fn is_dirty(&self) -> bool;
}

impl dyn Page {
    const SIZE_PAGE_HEADER: usize = 8;
    const OFFSET_PAGE_START: usize = 0;
    const OFFSET_LSN: usize = 4;

    pub fn new() -> Box<dyn Page> {
        todo!()
    }



    fn reset_memory(&self) {
        todo!()
    }

}

pub trait TablePage: Page {
    
}