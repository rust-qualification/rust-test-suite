// Comparison expressions: 99af6d58-c504-41dd-a325-9341bd0ae9a8

pub fn main() {
    let l: f32 = 1.4;
    let m: i32 = 2;

    let result_7: bool = l == m;
    //~^ ERROR mismatched types

    let n: i64 = 5;
    let o: i32 = 2;

    let result_8: bool = n == o;
    //~^ ERROR mismatched types


    let p: i64 = 5;
    let q: bool = true;

    let result_9: bool = p == q;
    //~^ ERROR mismatched types
}
