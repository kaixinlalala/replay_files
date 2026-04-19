extern crate unicode_segmentation;
// utf-8 error: not a char boundary
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(16 ,17870283321406128129 ,false);
    let _ = unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0) ,"\0\0\u{1}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}\u{7f}ʙ\0W" ,0);
}