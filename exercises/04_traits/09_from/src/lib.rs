// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        // WrappingU32: This is the name of a struct or enum variant.
        //              It signals that this type provides wrapping arithmetic.
        // { value }: This is the syntax for constructing an instance of the
        //            WrappingU32 type. It indicates that you're directly
        //            initializing the value field of the WrappingU32 with
        //            some provided value.
        WrappingU32 { value }
        // This is a shorthand way of writing WrappingU32 { value: value }
        // when the field name and the variable name you're using to initialize
        // it are the same.
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
