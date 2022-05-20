
use crate::include::common::FrameId;

pub trait Replacer {
    /// Remove the victim frame as defined by the replaced policy.
    /// @param[out] frame_id id of frame that was removed, nullptr if no victim was found
    /// @return true if a victim frame was found, false otherwise
    fn victim(&self, frame_id: FrameId) -> bool;

    /// Pins a frame, indicating that it should not be victimized until it is unpinned.
    /// @param frame_id the id of the frame to pin
    fn pin(&self, frame_id: FrameId);

    /// Unpins a frame, indicating that it can now be victimized.
    /// @param frame_id the id of the frame to unpin
    fn unpin(&self, frame_id: FrameId);

    /// @return the number of elements in the replacer that can be victimized
    fn size(&self) -> usize;
}
