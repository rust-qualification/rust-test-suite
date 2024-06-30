// Union Types: 847acf71-84b6-4ace-92d8-9e127ba0911e

fn main() {
    struct DropType;
        
    impl Drop for DropType {
        fn drop(&mut self) {}
    }

    union InvalidDropUnion {
        x: DropType, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
    }
    
}