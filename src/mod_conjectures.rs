use crate::helpers::{is_even, is_odd};

// Check that all even numbers are powers of two and these all appear
pub fn all_even_numbers_are_powers_of_two(sequence: &Vec<usize>) {
    let mut power_of_two = 2;

    for n in sequence {
        if is_even(*n) {
            assert!(*n == power_of_two);
            power_of_two *= 2;
        }
    }
}

// Check that all odd numbers after three are congruent to 2 mod 3
pub fn all_odd_numbers_after_3_are_2_mod_3(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 3 == 2);
            }
        }
    }
}

// Check that all odd numbers are congruent to 3 mod 4
pub fn all_odd_numbers_are_3_mod_4(sequence: &Vec<usize>) {
    for n in sequence {
        if is_odd(*n) {
            assert!(*n % 4 == 3);
        }
    }
}

// Check that all odd numbers after eleven are not congruent to 0 or 1 mod 11
pub fn all_odd_numbers_after_11_are_not_0_or_1_mod_11(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 11 {
            if is_odd(*n) {
                assert!(*n % 11 != 0 && *n % 11 != 1);
            }
        }
    }
}

// Check that all odd numbers after three are congruent to 11 mod 12
pub fn all_odd_numbers_after_3_are_11_mod_12(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 12 == 11);
            }
        }
    }
}

// Check that all odd numbers after 3 are congruent to 5 mod 6
pub fn all_odd_numbers_after_3_are_5_mod_6(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 6 == 5);
            }
        }
    }
}

// Check that all odd numbers are congruent to 3 or 7 mod 8
pub fn all_odd_numbers_are_3_or_7_mod_8(sequence: &Vec<usize>) {
    for n in sequence {
        if is_odd(*n) {
            assert!(*n % 8 == 3 || *n % 8 == 7);
        }
    }
}

// Check that all odd numbers after three are congruent to 11 or 23 mod 24
pub fn all_odd_numbers_after_3_are_11_or_23_mod_24(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 24 == 11 || *n % 24 == 23);
            }
        }
    }
}
