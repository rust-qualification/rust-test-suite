//@ run-pass
// 1.1.1.

pub fn main() {

    let true_bit_pattern: u8 = true as u8;
    let false_bit_pattern: u8 = false as u8;

    assert_eq!(true_bit_pattern, 0x01);
    assert_eq!(false_bit_pattern, 0x00);

    fn toggle_bit(bool_value: u8) -> u8 {
        bool_value ^ 0x01
    }

    let toggled_true = toggle_bit(true as u8);
    let toggled_false = toggle_bit(false as u8);

    assert_eq!(toggled_true, 0x00); 
    assert_eq!(toggled_false, 0x01); 
}
