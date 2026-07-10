extern crate slice_deque;
fn main() {
    let mut d = slice_deque::SliceDeque::<i32>::new();
    d.push_back(1);
    d.push_back(2);
    let (left, right) = d.as_slices();
    assert_eq!(left, &[1, 2]);
    assert_eq!(right, &[]);

// 第二个同样情况的示例
    // let mut _local0 = slice_deque:: SliceDeque::<i32>::with_capacity(0);
    // let _ = _local0.as_slices();
}