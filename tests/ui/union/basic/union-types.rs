// 1.2.4.
use std::mem::ManuallyDrop; 


fn main() {
    // 1.2.4.1.
    union CopyUnion {
        x: u32,
        y: f64,
    }

    let u = CopyUnion { x: 5 };
    unsafe {
        assert_eq!(u.x, 5);
    }

    // 1.2.4.2.
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

    // 1.2.4.3.
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

    // 1.2.4.4.
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

    // 1.2.4.5.
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




    union InvalidStringUnion {
        x: String, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
        y: u8, 
    }

    struct DropType;
        
    impl Drop for DropType {
        fn drop(&mut self) {}
    }

    union InvalidDropUnion {
        x: DropType, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }

    union InvalidArrayUnion {
        x: [String; 3], //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }

    union InvalidTupleUnion {
        x: (String, f64), //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }

    union InvalidGenericUnion<T> {
        x: T, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }
    
}