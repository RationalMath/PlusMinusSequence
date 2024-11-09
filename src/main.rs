use std::collections::BTreeSet;

fn main() {
    const N: usize = 100_000_000;

    let mut working_sequence: BTreeSet<usize> = (2..=N).collect();
    let mut output_sequence: Vec<usize> = Vec::new();

    while !working_sequence.is_empty() {
        let mut x = working_sequence.pop_first().expect("Sequence is empty");
        output_sequence.push(x);

        let x_plus_one = x + 1;
        let x_minus_one = x - 1;

        loop {
            x += x_plus_one;

            if x > N {
                break;
            }

            working_sequence.remove(&x);

            x += x_minus_one;

            if x > N {
                break;
            }

            working_sequence.remove(&x);
        }
    }

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
}

fn all_even_numbers_are_powers_of_two(sequence: &Vec<usize>) {
    let mut power_of_two = 2;

    let _ = sequence.iter().map(|&n| {
        if is_even(n) {
            assert!(n == power_of_two);
            power_of_two *= 2;
        }
    });
}

fn all_odd_numbers_after_3_are_2_mod_3(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 3 == 2);
            }
        }
    }
}

fn all_odd_numbers_are_3_mod_4(sequence: &Vec<usize>) {
    for n in sequence {
        if is_odd(*n) {
            assert!(*n % 4 == 3);
        }
    }
}

fn all_odd_numbers_after_11_are_not_0_or_1_mod_11(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 11 {
            if is_odd(*n) {
                assert!(*n % 11 != 0 && *n % 11 != 1);
            }
        }
    }
}

fn all_odd_numbers_after_3_are_11_mod_12(sequence: &Vec<usize>) {
    for n in sequence {
        if *n > 3 {
            if is_odd(*n) {
                assert!(*n % 12 == 11);
            }
        }
    }
}

fn is_even(number: usize) -> bool {
    number & 1 == 0
}

fn is_odd(number: usize) -> bool {
    number & 1 == 1
}
