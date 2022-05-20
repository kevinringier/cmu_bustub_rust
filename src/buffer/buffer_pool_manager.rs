use crate::include::common::PageId;
use crate::page::{page::Page, table_page::TablePage};

pub trait BufferPoolManager {
    /// @return size of the buffer pool
    fn get_pool_size(&self) -> usize;

    /// Fetch the requested page from the buffer pool.
    /// @param page_id id of page to be fetched
    /// @return the requested page
    fn fetch_pg_imp(&self, page_id: PageId) -> Box<dyn Page>;

    ///Unpin the target page from the buffer pool.
    /// @param page_id id of page to be unpinned
    /// @param is_dirty true if the page should be marked as dirty, false otherwise
    /// @return false if the page pin count is <= 0 before this call, true otherwise
    fn unpin_pg_imp(&self, page_id: PageId, is_dirty: bool) -> bool;

    /// Flushes the target page to disk.
    /// @param page_id id of page to be flushed, cannot be INVALID_PAGE_ID
    /// @return false if the page could not be found in the page table, true otherwise
    fn flush_pg_imp(&self, page_id: PageId) -> bool;

    /// Creates a new page in the buffer pool.
    /// @param[out] page_id id of created page
    /// @return None if no new pages could be created, otherwise Some(page)
    fn new_pg_imp(&self, page_id: PageId) -> Option<Box<dyn Page>>;

    /// Deletes a page from the buffer pool.
    /// @param page_id id of page to be deleted
    /// @return false if the page exists but could not be deleted, true if the page didn't exist or deletion succeeded
    fn delete_pg_imp(&self, page_id: PageId) -> bool;

    /// Flushes all the pages in the buffer pool to disk.
    fn flush_all_pgs_imp(&self);
    
}