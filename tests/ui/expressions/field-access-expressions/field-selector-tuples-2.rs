// Field Access Expressions: 7fc7d0a9-9066-4c99-81a5-37bd9ca6b223

fn main() {
    let my_tuple = (3, "hi", 1.2);
    let _ = my_tuple.first;
    //~^ ERROR no field `first` on type `({integer}, &str, {float})`
    let _ = my_tuple.3;
    //~^ ERROR no field `3` on type `({integer}, &str, {float})`
}
