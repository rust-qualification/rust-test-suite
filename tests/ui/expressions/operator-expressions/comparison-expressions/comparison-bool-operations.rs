//@ run-pass
// Comparison expressions: e420b920-c632-48bb-ab6c-e3f069f3b893
// Comparison expressions: 776d385c-c14e-4a68-a8ad-bb4170d0a41d
// Comparison expressions: 7eb21a55-d01d-4115-9c46-1d733586abeb
// Comparison expressions: d2b1ae2b-5b9e-4133-852c-a0433a2d3ca5
// Comparison expressions: 4c3ff96c-5612-4e32-baf8-3b97aa49a52f


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

    assert_eq!(true != true, !(true == true));
    assert_eq!(true != false, !(true == false));
    assert_eq!(false != true, !(false == true));
    assert_eq!(false != false, !(false == false));

    // Greater equal than
    assert_eq!(true >= true, true);
    assert_eq!(true >= false, true);
    assert_eq!(false >= true, false);
    assert_eq!(false >= false, true);

    assert_eq!(true >= true, (true == true) | (true > true));
    assert_eq!(true >= false, (true == false) | (true > false));
    assert_eq!(false >= true, (false == true) | (false > true));
    assert_eq!(false >= false, (false == false) | (false > false));

    // Less than
    assert_eq!(true < true, false);
    assert_eq!(true < false, false);
    assert_eq!(false < true, true);
    assert_eq!(false < false, false);

    assert_eq!(true < true, !(true >= true));
    assert_eq!(true < false, !(true >= false));
    assert_eq!(false < true, !(false >= true));
    assert_eq!(false < false, !(false >= false));

    // Less equal than
    assert_eq!(true <= true, true);
    assert_eq!(true <= false, false);
    assert_eq!(false <= true, true);
    assert_eq!(false <= false, true);

    assert_eq!(true <= true, (true == true) | (true < true));
    assert_eq!(true <= false, (true == false) | (true < false));
    assert_eq!(false <= true, (false == true) | (false < true));
    assert_eq!(false <= false, (false == false) | (false < false));
}
