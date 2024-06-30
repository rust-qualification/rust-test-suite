//@ run-pass
// Union Types: 11a3041f-f307-4ff4-acf3-fb256baf9f49

#[allow(dead_code)]
fn main() {
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
}