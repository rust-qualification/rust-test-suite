//@ run-pass
// Comparison expressions: 99af6d58-c504-41dd-a325-9341bd0ae9a8

pub fn main() {
    let x = 1;
    let y = 2;

    let result_1: bool = x == y;

    let a = 'a';
    let b = 'b';

    let result_2: bool = a >= b;

    let c = true;
    let d = false;

    let result_3: bool = c != d;


    let e = 1;
    let f = 2;

    let x_ptr: *const i32 = &x;
    let y_ptr: *const i32 = &y;

    let result_4: bool = x_ptr != y_ptr;


    let h: String = String::from("Hello");
    let i: String = String::from("world");

    let result_5: bool = h != i;

    let j = 1.4;
    let k = 2.2;

    let result_6: bool = j == k;

}
