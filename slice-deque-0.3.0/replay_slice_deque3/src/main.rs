extern crate slice_deque;
fn main() {
    // let requested = usize::MAX / 2 + 1;
    // let requested = usize::MAX;
    let requested = 11140386617063807544;
    let d = slice_deque::SliceDeque::<u8>::with_capacity(requested);
    println!("requested capacity = {}", requested);
    println!("actual capacity    = {}", d.capacity());

}

// 1.when requested（usize::MAX / 2 + 1····usize::MAX）
// release mode: returns capacity which is not expected. 
// 2. when requested（11140386617063807544）
// release mode: "ut-of-memory3807544 buffer with capacity "