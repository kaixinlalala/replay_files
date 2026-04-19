extern crate unicode_segmentation;
// unwrap error
fn main() {
    let mut _local0 = unicode_segmentation::GraphemeCursor::new(6712945236183381371 ,7017505629193139579 ,true);
    let _ = unicode_segmentation::GraphemeCursor::provide_context(&mut (_local0) ,"}}}}}}" ,3493998360278433149);
}