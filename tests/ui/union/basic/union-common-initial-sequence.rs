//@ run-pass
use std::ffi::CString;
use std::mem::ManuallyDrop; 

fn main() {
    #[repr(C)]
    struct Struct1 {
        x: i32,
        y: f64,
    }

    #[repr(C)]
    struct Struct2 {
        x: i32,
        name: *const i8,
    }

    union MyUnion2 {
        s1: ManuallyDrop<Struct1>, 
        s2: ManuallyDrop<Struct2>, 
    }

    let mut u2 = MyUnion2 {
        s1: ManuallyDrop::new(Struct1 { x: 5, y: 2.20 }), 
    };

    assert_eq!(unsafe { u2.s1.x }, 5);
    assert_eq!(unsafe { u2.s1.y }, 2.20);

    unsafe {
        let name = CString::new("Hello").expect("CString::new failed");
        u2.s2 = ManuallyDrop::new(Struct2 { x: 43, name: name.as_ptr() }); 
        assert_eq!(u2.s2.x, 43);
        assert_eq!(u2.s1.x, 43);
        assert_eq!(u2.s2.name, name.as_ptr());
    }
    drop(u2);
}