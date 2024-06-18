//@ run-pass
// 1.2.6.
fn main() {
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: u64,
    }
    let mut u = MyUnion { f1: 0 };
    unsafe {
        u.f1 = 5;
        assert_eq!(u.f1, 5); 
        assert_eq!(u.f2, 5 as u64);
    }
}