extern crate unicode_segmentation;
// assertion failed
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(14323354221939181254 ,14339398540424824518 ,true);
    let _ = unicode_segmentation::GraphemeCursor::is_boundary(&mut (_local0) ,"\u{7f}" ,14323354221939181254);
    let _ = unicode_segmentation::GraphemeCursor::provide_context(&mut (_local0) ,"\0R" ,18410714726934759110);
}