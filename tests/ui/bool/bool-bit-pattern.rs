//@ run-pass
// Bool Types: ee7018f0-eca6-4ec1-b645-ffb98940f3ac

pub fn main() {

    let true_value: u8 = true as u8;
    let false_value: u8 = false as u8;

    assert_eq!(true_value, 0x01);
    assert_eq!(false_value, 0x00);

    fn flip_bit(bool_value: u8) -> u8 {
        bool_value ^ 0x01
    }

    let true_flipped = flip_bit(true_value);
    let false_flipped = flip_bit(false_value);

    assert_eq!(true_flipped, 0x00); 
    assert_eq!(false_flipped, 0x01); 
}