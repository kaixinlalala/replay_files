extern crate unicode_segmentation;
// utf-8 error: not a char boundary
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(8935127994137148074 ,13811976078547984555 ,true);
    let _ = unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0) ,"\u{600}\u{600}\u{b4d}ꦓ\u{f}" ,8935127994137148075);
}