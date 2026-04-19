extern crate unicode_segmentation;
// utf-8 error: not a char boundary
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(9042521604759584127 ,6727108446334188925 ,false);
    let _ = unicode_segmentation::GraphemeCursor::is_boundary(&mut (_local0) ,"}Ԟ" ,9042521604759584125);
}