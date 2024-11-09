pub fn save_to_file(n: usize, output_sequence: &Vec<usize>, filename: &str) {
    // Save results to file
    let mut file = std::fs::File::create(filename).expect("Failed to create file");
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
