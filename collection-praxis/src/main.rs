use collection_praxis::praxis_1::{median, mode};
use collection_praxis::praxis_2::pig_latin;

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 3, 7, 1, 1];
    assert_eq!(median(&v), 4);
    assert_eq!(mode(&v), 1);
    let s = "hello world i like apples";
    assert_eq!(pig_latin(s), "ellohay orldway ihay ikelay appleshay");
    println!("All tests passed!");
}
