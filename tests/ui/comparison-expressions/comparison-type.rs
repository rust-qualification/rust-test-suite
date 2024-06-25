//@ run-pass
// Comparison expressions: 915004e3-13c5-445a-9159-f29f39b65362
use std::any::type_name;
use std::any::type_name_of_val;

pub fn main() {
    let x = 1;
    let y = 2;

    let result_1: bool = (x == y);
    let result_2: bool = (x != y);
    let result_3: bool = (x >= y);
    let result_4: bool = (x <= y);
    let result_5: bool = (x > y);
    let result_6: bool = (x < y);

    assert_eq!(type_name::<bool>(), type_name_of_val(&result_1));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_2));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_3));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_4));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_5));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_6));


    let a = 'a';
    let b = 'b';

    let result_7: bool = (a == b);
    let result_8: bool = (a != b);
    let result_9: bool = (a >= b);
    let result_10: bool = (a <= b);
    let result_11: bool = (a > b);
    let result_12: bool = (a < b);

    assert_eq!(type_name::<bool>(), type_name_of_val(&result_7));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_8));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_9));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_10));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_11));
    assert_eq!(type_name::<bool>(), type_name_of_val(&result_12));


}
