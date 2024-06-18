//@ run-pass
// 2.2.1.

pub fn main() {

    // 2.2.1.1.
    assert_eq!(true && true, true);
    assert_eq!(true && false, false);
    assert_eq!(false && true, false);
    assert_eq!(false && false, false);

    assert!(!(false && panic!()));
    
    // 2.2.1.2.
    assert_eq!(true || true, true);
    assert_eq!(true || false, true);
    assert_eq!(false || true, true);
    assert_eq!(false || false, false);

    assert!(true || panic!());
}
