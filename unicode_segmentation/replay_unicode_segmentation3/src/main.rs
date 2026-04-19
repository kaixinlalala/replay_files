extern crate unicode_segmentation;
// unwrap error
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(3544668469065756977 ,3544668469065756977 ,false);
    let _ = unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0) ,"1111" ,14857710733020442929);
}