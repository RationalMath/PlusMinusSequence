use differences::*;
use fs::save_to_file;
use mod_conjectures::*;
use plus_minus_sequence::generate_plus_minus_sequence;

fn main() {
    // Get the upper limit n from the command line or default to 1,000,000
    let n = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(1_000_000);

    // Generate the plus-minus sequence
    let output_sequence = generate_plus_minus_sequence(n);

    // Check conjectures
    all_even_numbers_are_powers_of_two(&output_sequence);
    println!("Conjecture 1: All even numbers are powers of two and these all appear");

    all_odd_numbers_after_3_are_2_mod_3(&output_sequence);
    println!("Conjecture 2: All odd numbers after three are congruent to 2 mod 3");

    all_odd_numbers_are_3_mod_4(&output_sequence);
    println!("Conjecture 3: All odd numbers are congruent to 3 mod 4");

    all_odd_numbers_after_11_are_not_0_or_1_mod_11(&output_sequence);
    println!("Conjecture 4: All odd numbers after eleven are not congruent to 0 or 1 mod 11");

    all_odd_numbers_after_3_are_11_mod_12(&output_sequence);
    println!("Conjecture 5: All odd numbers after three are congruent to 11 mod 12");

    all_odd_numbers_after_3_are_5_mod_6(&output_sequence);
    println!("Conjecture 6: All odd numbers after three are congruent to 5 mod 6");

    all_odd_numbers_are_3_or_7_mod_8(&output_sequence);
    println!("Conjecture 7: All odd numbers are congruent to 3 or 7 mod 8");

    all_odd_numbers_after_3_are_11_or_23_mod_24(&output_sequence);
    println!("Conjecture 8: All odd numbers after three are congruent to 11 or 23 mod 24");

    difference_between_all_consecutive_odd_numbers_is_congruent_to_0_mod_12(&output_sequence);
    println!(
        "Conjecture 9: The difference between all consecutive odd numbers is congruent to 0 mod 12"
    );

    println!();

    // Print the upper limit
    println!("Upper limit: {}", n);

    // Print the number of elements in the output sequence
    println!(
        "Number of elements in the output sequence: {}",
        output_sequence.len()
    );

    save_to_file(n, &output_sequence, "plus_minus_sequence_results.txt");
}

mod differences;
mod fs;
mod helpers;
mod mod_conjectures;
mod plus_minus_sequence;
