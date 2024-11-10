// Check that a number is even
pub fn is_even(number: usize) -> bool {
    number & 1 == 0
}

// Check that a number is odd
pub fn is_odd(number: usize) -> bool {
    number & 1 == 1
}
