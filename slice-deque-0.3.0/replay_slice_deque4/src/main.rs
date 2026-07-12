extern crate slice_deque;
fn main() {
    let requested = usize::MAX / 2 + 1;
    // let requested = usize::MAX;
    // let requested = 13789245632553365554;
    let mut d = slice_deque::SliceDeque::<u8>::new();
    d.reserve(requested);
    println!("requested capacity = {}", requested);
    println!("actual capacity    = {}", d.capacity());

}

// 1.when requested（usize::MAX / 2 + 1····usize::MAX）
// release mode: returns capacity which is not expected. 
//  requested capacity = 9223372036854775808
//  actual capacity    = 0
// 2. when requested（13789245632553365554）
// release mode: called `Result::unwrap()` on an `Err` value: out-of-memory