//@ run-pass
// Comparison expressions: e420b920-c632-48bb-ab6c-e3f069f3b893


pub fn main() {
    // Equality
    assert_eq!(true == true, true);
    assert_eq!(true == false, false);
    assert_eq!(false == true, false);
    assert_eq!(false == false, true);

    // Greater than
    assert_eq!(true > true, false);
    assert_eq!(true > false, true);
    assert_eq!(false > true, false);
    assert_eq!(false > false, false);

    // Inequality
    assert_eq!(true != true, false);
    assert_eq!(true != false, true);
    assert_eq!(false != true, true);
    assert_eq!(false != false, false);

    // Greater equal than
    assert_eq!(true >= true, true);
    assert_eq!(true >= false, true);
    assert_eq!(false >= true, false);
    assert_eq!(false >= false, true);

    // Less than
    assert_eq!(true < true, false);
    assert_eq!(true < false, false);
    assert_eq!(false < true, true);
    assert_eq!(false < false, false);

    // Less equal than
    assert_eq!(true <= true, true);
    assert_eq!(true <= false, false);
    assert_eq!(false <= true, true);
    assert_eq!(false <= false, true);
}
