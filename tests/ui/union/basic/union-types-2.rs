// Union Types: 26ad2e4a-ff73-4eb4-b16f-d33a6e5d7e7f
// Union Types: 11a3041f-f307-4ff4-acf3-fb256baf9f49
// Union Types: 847acf71-84b6-4ace-92d8-9e127ba0911e
// Union Types: 218f449a-7973-4157-8a92-87645b9ceedc
// Union Types: d1b5850a-f09d-4785-9d56-6ec53d7cfccf

fn main() {
    union InvalidStringUnion {
        x: String, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
        y: u8, 
    }

    struct DropType;
        
    impl Drop for DropType {
        fn drop(&mut self) {}
    }

    union InvalidDropUnion {
        x: DropType, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }

    union InvalidArrayUnion {
        x: [String; 3], //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }

    union InvalidTupleUnion {
        x: (String, f64), //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }

    union InvalidGenericUnion<T> {
        x: T, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }
    
}