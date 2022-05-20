use std::collections::{HashMap, LinkedList};
use std::sync::{atomic::AtomicPtr, Mutex};

use crate::buffer::buffer_pool_manager::BufferPoolManager;
use crate::include::common::{FrameId, PageId};
use crate::page::{page::Page, table_page::TablePage};
use crate::buffer::replacer::Replacer;


/// The BufferPoolManagerInstance is responsible for fetching database pages from the
/// DiskManager and storing them in memory. The BufferPoolManagerInstance can also write
/// dirty pages out to disk when it is either explicitly instructed to so or when it needs
/// to evict a page to make space for a new page.
struct BufferPoolManagerInstance {
    /// Number of pages in the buffer pool.
    pool_size: usize,

    /// How many instances are in the parallel BPM (if present, otherwise just 1 BPI)
    num_instances: u32,

    /// Index of this BPI in the parallel BPM (if present, otherwise just 0)
    instance_index: u32,

    /// Each BPI maintains its own counter for page_ids to hand out, must ensure they mod back to its instance_index
    next_page_id: AtomicPtr<PageId>,

    /// Array of buffer pool pages
    pages: Vec<Box<dyn Page>>,

    /// Disk manager
    // TODO: implement
    disk_manager: bool,

    /// Log manager
    // TODO: implement
    log_manager: bool,

    /// Page table for keeping track of buffer pool pages.
    page_table: HashMap<PageId, FrameId>,

    /// Replacer to find unpinned pages for replacement.
    replacer: Box<dyn Replacer>,

    /// List of free pages
    free_list: LinkedList<FrameId>,
}

struct BufferPoolManagerInstanceMutex {
    /// This latch protects shared data structures.  It protects shared access to this buffer pool manager instance.
    // TODO: does this need to be wrapped in an atomic reference counter? 
    // As you probably noticed in the previous task, the single Buffer Pool Manager Instance needs to take latches 
    // in order to be thread safe. This can cause a lot of contention as every thread fights over a single latch when 
    //interacting with the buffer pool. One potential solution is to have multiple buffer pools in your system, each with it's own latch.
    bpmi_latch: Mutex<BufferPoolManagerInstance>
}

impl BufferPoolManagerInstance {
    // pub fn new() -> BufferPoolManagerInstance {
    //     BufferPoolManagerInstance {}
    // }   
}

impl BufferPoolManager for BufferPoolManagerInstance {
    fn get_pool_size(&self) -> usize {
        1
    }

    fn fetch_pg_imp(&self, page_id: PageId) -> Box<dyn Page> {
        Box::new(TablePage {})
    }

    fn unpin_pg_imp(&self, page_id: PageId, is_dirty: bool) -> bool {
        true
    }

    fn flush_pg_imp(&self, page_id: PageId) -> bool {
        true
    }

    fn new_pg_imp(&self, page_id: PageId) -> Option<Box<dyn Page>> {
        None
    }

    fn delete_pg_imp(&self, page_id: PageId) -> bool {
        true
    }

    fn flush_all_pgs_imp(&self) {
        
    }    
}

impl BufferPoolManagerInstanceMutex {

}