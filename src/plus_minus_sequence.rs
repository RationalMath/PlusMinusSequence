use std::collections::BTreeSet;

pub fn generate_plus_minus_sequence(n: usize) -> Vec<usize> {
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

    output_sequence
}
