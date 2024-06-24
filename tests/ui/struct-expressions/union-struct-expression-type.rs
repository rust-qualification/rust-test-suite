//@ run-pass
// 2.3.1.
use std::any::type_name;
use std::mem;

fn type_of<T>(_val: &T) -> &'static str {
    let var_type = type_name::<T>();
    match var_type.rfind("::") {
        Some(i) => &var_type[i + 2..],
        None => var_type,
    }
}

#[repr(C)]
union MyUnion {
    x: i32,
    y: u32,
}

fn main() {
    let u = MyUnion { x: 2 };
    assert_eq!(mem::size_of::<MyUnion>(), mem::size_of::<u32>());
    assert_eq!(type_of(&u), "MyUnion");
}