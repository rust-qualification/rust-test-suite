//@ run-pass
// Field Access Expressions: 7fc7d0a9-9066-4c99-81a5-37bd9ca6b223

fn main() {
    let my_tuple = (3, "hi", 1.2);
    
    let first = my_tuple.0;
    let second = my_tuple.1;
    let third = my_tuple.2;
    
    assert_eq!(first, 3);
    assert_eq!(second, "hi");
    assert_eq!(third, 1.2);
}
