use std::collections::BTreeSet;

fn main() {
    // Get the upper limit n from the command line or default to 1,000,000
    let n = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(1_000_000);

    // Create the working sequence of numbers 2 to n
    let mut working_sequence: BTreeSet<usize> = (2..=n).collect();

    // Create the empty output sequence
    let mut output_sequence: Vec<usize> = Vec::new();

    // Loop until the working sequence is empty
    while !working_sequence.is_empty() {
        // Pop the first element from the working sequence
        let mut x = working_sequence.pop_first().expect("Sequence is empty");

        // Add the first element to the output sequence
        output_sequence.push(x);

        // Define x+1 and x-1
        let x_plus_one = x + 1;
        let x_minus_one = x - 1;

        // Loop through the working sequence
        loop {
            // Increment x by x_plus_one
            x += x_plus_one;

            // If x is greater than n, then exit the loop
            if x > n {
                break;
            }

            // Remove x from the working sequence
            working_sequence.remove(&x);

            // Increment x by x_minus_one
            x += x_minus_one;

            // If x is greater than n, then exit the loop
            if x > n {
                break;
            }

            // Remove x from the working sequence
            working_sequence.remove(&x);
        }
    }

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

    println!();

    // Print the upper limit
    println!("Upper limit: {}", n);

    // Print the number of elements in the output sequence
    println!(
        "Number of elements in the output sequence: {}",
        output_sequence.len()
    );

    // Save results to file
    let mut file =
        std::fs::File::create("plus_minus_sequence_results.txt").expect("Failed to create file");
    use std::io::Write;

    // Write header with upper limit
    writeln!(file, "Plus Minus Sequence Results").expect("Failed to write to file");
    writeln!(file, "Upper limit: {}\n", n).expect("Failed to write to file");
    writeln!(
        file,
        "Number of elements in the output sequence: {}\n",
        output_sequence.len()
    )
    .expect("Failed to write to file");

    // Write conjectures
    writeln!(file, "Verified conjectures:").expect("Failed to write to file");
    writeln!(
        file,
        "1. All even numbers are powers of two and these all appear"
    )
    .expect("Failed to write to file");
    writeln!(
        file,
        "2. All odd numbers after three are congruent to 2 mod 3"
    )
    .expect("Failed to write to file");
    writeln!(file, "3. All odd numbers are congruent to 3 mod 4").expect("Failed to write to file");
    writeln!(
        file,
        "4. All odd numbers after eleven are not congruent to 0 or 1 mod 11"
    )
    .expect("Failed to write to file");
    writeln!(
        file,
        "5. All odd numbers after three are congruent to 11 mod 12\n"
    )
    .expect("Failed to write to file");

    writeln!(file, "Plus Minus Sequence (Output Sequence):").expect("Failed to write to file");
    writeln!(file).expect("Failed to write to file");

    // Write sequence with 10 numbers per line
    for chunk in output_sequence.chunks(10) {
        let line = chunk
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(file, "{}", line).expect("Failed to write to file");
    }
}

// Check that all even numbers are powers of two and these all appear
fn all_even_numbers_are_powers_of_two(sequence: &Vec<usize>) {
    let mut power_of_two = 2;

    for n in sequence {
        if is_even(*n) {
            assert!(*n == power_of_two);
            power_of_two *= 2;
        }
    }
}

// Check that all odd numbers after three are congruent to 2 mod 3
fn all_odd_numbers_after_3_are_2_mod_3(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 3 == 2);
            }
        }
    }
}

// Check that all odd numbers are congruent to 3 mod 4
fn all_odd_numbers_are_3_mod_4(sequence: &Vec<usize>) {
    for n in sequence {
        if is_odd(*n) {
            assert!(*n % 4 == 3);
        }
    }
}

// Check that all odd numbers after eleven are not congruent to 0 or 1 mod 11
fn all_odd_numbers_after_11_are_not_0_or_1_mod_11(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 11 {
            if is_odd(*n) {
                assert!(*n % 11 != 0 && *n % 11 != 1);
            }
        }
    }
}

// Check that all odd numbers after three are congruent to 11 mod 12
fn all_odd_numbers_after_3_are_11_mod_12(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 12 == 11);
            }
        }
    }
}

// Check that a number is even
fn is_even(number: usize) -> bool {
    number & 1 == 0
}

// Check that a number is odd
fn is_odd(number: usize) -> bool {
    number & 1 == 1
}
