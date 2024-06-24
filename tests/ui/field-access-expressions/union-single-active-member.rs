//@ run-pass
// Field Access Expressions: 7bc74e90-12b3-4ca7-8275-d31bc204b655

fn main() {
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: u32,
    }
    let mut u = MyUnion { f1: 0 };
    unsafe {
        assert_eq!(u.f1, 0); 
        u.f2 = 10;
        assert_eq!(u.f2, 10);
    }
}