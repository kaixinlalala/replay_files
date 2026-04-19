extern crate unicode_segmentation;
// unwrap error
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(3544668469065756985 ,3472609582225829921 ,true);
    let _ = unicode_segmentation::GraphemeCursor::prev_boundary(&mut (_local0) ,"\u{740}ြ\u{740}\u{740}[\\E100" ,3544668469065756976);
    let _ = unicode_segmentation::GraphemeCursor::next_boundary(&mut (_local0) ,"1\u{740}\u{740}[\\[|0\u{f}11-[D" ,9251902466196254769);
}