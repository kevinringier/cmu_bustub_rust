use crate::buffer::replacer::Replacer;
use crate::include::common::FrameId;

/// LRUReplacer implements the Least Recently Used replacement policy.
pub struct LRUReplacer {
    
}

impl LRUReplacer {
    pub fn new() -> LRUReplacer {
        LRUReplacer {}
    }
}

impl Replacer for LRUReplacer {
    fn victim(&self, frame_id: FrameId) -> bool {
        true
    }

    fn pin(&self, frame_id: FrameId) {

    }

    fn unpin(&self, frame_id: FrameId) {

    }

    fn size(&self) -> usize {
        0
    }
}