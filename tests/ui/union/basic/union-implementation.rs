//@ run-pass

union MyUnion {
    x: u8,
}

impl MyUnion {
    fn double(&self) -> u8 { 
        unsafe { 
            self.x * 2
        } 
    }
}

fn main() {
    let u = MyUnion { x: 5 };
    assert_eq!(u.double(), 10);
}