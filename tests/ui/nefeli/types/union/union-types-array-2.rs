// Union Types: d1b5850a-f09d-4785-9d56-6ec53d7cfccf

fn main() {
    union InvalidArrayUnion {
        x: [String; 3], //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }
}