// Comparison expressions: 4eff8002-c02a-43d4-8773-ac28a98e0218

pub fn main() {
    let a = 1;
    let b = 1;
    let c = 1;

    let invalid = a == b == c;
    //~^ ERROR comparison operators cannot be chained
    //~| ERROR mismatched types
    let result = (a == b) == c;
}
