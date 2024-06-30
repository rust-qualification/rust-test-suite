// Union Types: 26ad2e4a-ff73-4eb4-b16f-d33a6e5d7e7f

fn main() {
    union InvalidStringUnion {
        x: String, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
        y: u8, 
    }

    union InvalidGenericUnion<T> {
        x: T, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }
}