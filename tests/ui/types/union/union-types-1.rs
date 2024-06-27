//@ run-pass
// Union Types: 26ad2e4a-ff73-4eb4-b16f-d33a6e5d7e7f
// Union Types: 11a3041f-f307-4ff4-acf3-fb256baf9f49
// Union Types: 847acf71-84b6-4ace-92d8-9e127ba0911e
// Union Types: 218f449a-7973-4157-8a92-87645b9ceedc
// Union Types: d1b5850a-f09d-4785-9d56-6ec53d7cfccf
use std::mem::ManuallyDrop; 

#[allow(dead_code)]
fn main() {
    union CopyUnion {
        x: u32,
        y: f64,
    }

    let u = CopyUnion { x: 5 };
    unsafe {
        assert_eq!(u.x, 5);
    }

    union MutableRefUnion<'x> {
        x: &'x mut u32,
        y: &'x mut f64,
    }

    let mut x = 10;
    let mut y = 20.0;
    let u = MutableRefUnion { x: &mut x };
    unsafe {
        *u.x = 15;
        assert_eq!(*u.x, 15);
    }

    let u = MutableRefUnion { y: &mut y };
    unsafe {
        *u.y = 25.0;
        assert_eq!(*u.y, 25.0);
    }

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

    union TupleUnion {
        x: (u32, f64),
        y: (i32, i64),
    }

    let u = TupleUnion { x: (1, 1.5) };
    unsafe {
        assert_eq!(u.x.0, 1);
        assert_eq!(u.x.1, 1.5);
    }

    let u = TupleUnion { y: (5, 1) };
    unsafe {
        assert_eq!(u.y.0, 5);
        assert_eq!(u.y.1, 1);
    }

    union ArrayUnion {
        x: [u32; 3],
        y: [f64; 3],
    }
    
    let u = ArrayUnion { x: [1, 2, 3] };
    unsafe {
        assert_eq!(u.x[0], 1);
        assert_eq!(u.x[1], 2);
        assert_eq!(u.x[2], 3);
    }
    
    let u = ArrayUnion { y: [0.1, 0.2, 0.3] };
    unsafe {
        assert_eq!(u.y[0], 0.1);
        assert_eq!(u.y[1], 0.2);
        assert_eq!(u.y[2], 0.3);
    }
}