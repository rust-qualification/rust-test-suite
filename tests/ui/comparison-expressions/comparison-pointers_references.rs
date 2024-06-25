//@ run-pass
// Comparison expressions: 40e36ca8-a41f-4093-a3c2-8ed49161d7f6

pub fn main() {

    let a = 1;
    let b = 2;

    let a_ptr: *const i32 = &a;
    let b_ptr: *const i32 = &b;
    unsafe {
        assert_ne!(*a_ptr, *b_ptr); 
        assert_eq!(*a_ptr, *a_ptr); 
    }
    let a_ref: &i32 = &a;
    let b_ref: &i32 = &b;

    assert_ne!(a_ref, b_ref); 
    assert_eq!(a_ref, a_ref); 
}
