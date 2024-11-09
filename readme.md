# Plus Minus Sequence

The Plus Minus Sequence is a number sieve described in the video [Extraordinary new sieve variants (Plus Minus) | Number theory and Sieve Magic | Wild Egg Maths](https://youtu.be/ILmPuDn3ZZw?si=TB4fqVwi1EmBvFVT).

## Output Sequence up to 1,000,000

You can find the output sequence up to 1,000,000 in the file [`plus_minus_sequence_results.txt`](plus_minus_sequence_results.txt).

## Requirements to run the program

You need to have Rust installed to run the program. You can download Rust at [rust-lang.org](https://www.rust-lang.org/).

## Run the program

```
cargo run --release
```

This will run the program with the default upper limit of 1,000,000, check the conjectures, save the output sequence to a file, and print main results to the terminal.

```
cargo run --release 1000
```

This will run the program with the upper limit of 1,000.

## Sieve algorithm

![Local Image](assets/plus-minus-sequence.png)
![Local Image](assets/plus-minus-sequence-conjectures.png)
