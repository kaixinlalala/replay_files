extern crate unicode_segmentation;
// out of bounds
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(9165160804024148064 ,6944656592455360608 ,true);
    let _ = unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0) ,"`111" ,6944656592455360608);
}