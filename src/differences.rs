use crate::is_odd;

// Check that the difference between all consecutive odd numbers is congruent to 0 mod 12
pub fn difference_between_all_consecutive_odd_numbers_is_congruent_to_0_mod_12(
    output_sequence: &Vec<usize>,
) {
    for i in 1..output_sequence.len() {
        let x1 = output_sequence[i - 1];
        let x2 = output_sequence[i];

        if is_odd(x1) && is_odd(x2) {
            let diff = x2 - x1;
            assert!(diff % 12 == 0);
        }
    }
}
