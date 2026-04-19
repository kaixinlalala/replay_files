extern crate unicode_segmentation;
// out of bounds
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(18229554450312027253 ,8877211129670018651 ,true);
    let _ = unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0) ," \\u{" ,8969870161476615468);
}