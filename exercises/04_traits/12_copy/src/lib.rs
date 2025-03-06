// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

impl std::ops::Add for WrappingU32 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        WrappingU32::new(self.value.wrapping_add(other.value))
    }

    // Explanation:
    // The Add trait is used to overload the + operator for the WrappingU32 struct.

    // 1. self.value: This refers to the value field of the current WrappingU32
    // instance. Since WrappingU32 encapsulates a u32, self.value is a u32 value.

    // 2. other.value: This refers to the value field of another WrappingU32
    // instance named other. Like self.value, other.value is also a u32 value.

    // 3. self.value.wrapping_add(other.value): This is the core of the wrapping
    // addition. The wrapping_add method is a built-in method for u32 that
    // performs addition with wrapping behavior. If the sum of self.value and
    // other.value exceeds the maximum value for a u32, the result will
    // "wrap around" to zero (or a smaller value). This prevents overflow errors.

    // 4. WrappingU32::new(...): This calls the new associated function
    // (often called a constructor in other languages) of the WrappingU32 struct.
    // It takes the result of the wrapping addition (which is a u32) as its argument
    // and creates a new WrappingU32 instance with that value.
    // The new function simply initializes the value field of the new WrappingU32 struct
    // with the provided u32 value.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
