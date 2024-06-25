// Comparison expressions: 4eff8002-c02a-43d4-8773-ac28a98e0218

pub fn main() {
    let a = true;
    let b = true;
    let c = true;

    let invalid = a == b == c;
    //~^ ERROR comparison operators cannot be chained
    let result = (a == b) == c;
}
