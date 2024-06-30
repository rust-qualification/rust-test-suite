//@ run-pass
// Union Types: 847acf71-84b6-4ace-92d8-9e127ba0911e
use std::mem::ManuallyDrop; 

#[allow(dead_code)]
fn main() {
    union ManuallyDropUnion {
        x: ManuallyDrop<String>,
        y: ManuallyDrop<Vec<u8>>,
    }
    let u = ManuallyDropUnion {
        x: ManuallyDrop::new(String::from("hello")),
    };
    unsafe {
        assert_eq!(*u.x, "hello");
    }
    let u = ManuallyDropUnion {
        y: ManuallyDrop::new(vec![1, 2, 3, 4, 5]),
    };
    unsafe {
        assert_eq!(*u.y, vec![1, 2, 3, 4, 5]);
    }
}