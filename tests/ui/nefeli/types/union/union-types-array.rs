//@ run-pass
// Union Types: d1b5850a-f09d-4785-9d56-6ec53d7cfccf

#[allow(dead_code)]
fn main() {
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