//@ run-pass
// 1.1.4.

pub fn main() {
    
    // 1.1.4.1.
    assert_eq!(!true, false);
    assert_eq!(!false, true);
    
    // 1.1.4.2.
    assert_eq!(true | true, true);
    assert_eq!(true | false, true);
    assert_eq!(false | true, true);
    assert_eq!(false | false, false);

    // 1.1.4.3.
    assert_eq!(true & true, true);
    assert_eq!(true & false, false);
    assert_eq!(false & true, false);
    assert_eq!(false & false, false);

    // 1.1.4.4.
    assert_eq!(true ^ true, false);
    assert_eq!(true ^ false, true);
    assert_eq!(false ^ true, true);
    assert_eq!(false ^ false, false);

    // 1.1.4.5.
    assert_eq!(true == true, true);
    assert_eq!(true == false, false);
    assert_eq!(false == true, false);
    assert_eq!(false == false, true);

    // 1.1.4.6.
    assert_eq!(true > true, false);
    assert_eq!(true > false, true);
    assert_eq!(false > true, false);
    assert_eq!(false > false, false);

    // 1.1.4.7.
    assert_eq!(true != true, false);
    assert_eq!(true != false, true);
    assert_eq!(false != true, true);
    assert_eq!(false != false, false);

    assert_eq!(true != true, !(true == true));
    assert_eq!(true != false, !(true == false));
    assert_eq!(false != true, !(false == true));
    assert_eq!(false != false, !(false == false));

    // 1.1.4.8.
    assert_eq!(true >= true, true);
    assert_eq!(true >= false, true);
    assert_eq!(false >= true, false);
    assert_eq!(false >= false, true);

    assert_eq!(true >= true, true == true || true > true);
    assert_eq!(true >= false, true == false || true > false);
    assert_eq!(false >= true, false == true || false > true);
    assert_eq!(false >= false, false == false || false > false);

    // 1.1.4.9.
    assert_eq!(true < true, false);
    assert_eq!(true < false, false);
    assert_eq!(false < true, true);
    assert_eq!(false < false, false);

    assert_eq!(true < true, !(true >= true));
    assert_eq!(true < false, !(true >= false));
    assert_eq!(false < true, !(false >= true));
    assert_eq!(false < false, !(false >= false));

    // 1.1.4.10.
    assert_eq!(true <= true, true);
    assert_eq!(true <= false, false);
    assert_eq!(false <= true, true);
    assert_eq!(false <= false, true);

    assert_eq!(true <= true, true == true || true < true);
    assert_eq!(true <= false, true == false || true < false);
    assert_eq!(false <= true, false == true || false < true);
    assert_eq!(false <= false, false == false || false < false);
}
