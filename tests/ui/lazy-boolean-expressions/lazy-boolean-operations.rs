//@ run-pass
// Lazy Boolean Opearations: 344f1da0-e40d-461b-938b-288cd67857a4
// Lazy Boolean Opearations: 8d4fd9cd-5ef4-46e0-877d-a5194aab3f2c
// Lazy Boolean Opearations: d05986fe-54a4-4142-93b1-fc7a557d1884

pub fn main() {

    assert_eq!(true && true, true);
    assert_eq!(true && false, false);
    assert_eq!(false && true, false);
    assert_eq!(false && false, false);

    assert_eq!(true || true, true);
    assert_eq!(true || false, true);
    assert_eq!(false || true, true);
    assert_eq!(false || false, false);

    assert_eq!((true && true) || false, true);
    assert_eq!(true && (false || true), true);
    assert_eq!((false && true) || (true && true), true);
    assert_eq!((true || false) && (false || true), true);
}
