//@ run-pass
// Bitwise expressions: 1b093a78-f66d-4803-988c-d30851762abb

pub fn main() {
    
    // Bitwise NOT
    assert_eq!(!true, false);
    assert_eq!(!false, true);
    
    // Bitwise OR
    assert_eq!(true | true, true);
    assert_eq!(true | false, true);
    assert_eq!(false | true, true);
    assert_eq!(false | false, false);

    // Bitwise AND
    assert_eq!(true & true, true);
    assert_eq!(true & false, false);
    assert_eq!(false & true, false);
    assert_eq!(false & false, false);

    // Bitwise XOR
    assert_eq!(true ^ true, false);
    assert_eq!(true ^ false, true);
    assert_eq!(false ^ true, true);
    assert_eq!(false ^ false, false);
}
