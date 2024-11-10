use plus_minus_sequence::{fs::save_to_file, plus_minus_sequence::generate_plus_minus_sequence};
use std::thread;

fn main() {
    let now = std::time::Instant::now();

    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    println!("Number of threads: {}", num_threads);

    // Get the upper limit N and second argument from command line
    let n = std::env::args()
        .nth(1)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(1_000_000);

    let base = std::env::args()
        .nth(2)
        .and_then(|arg| arg.parse::<usize>().ok())
        .unwrap_or(100);

    println!("Upper limit: {}", n);

    let pms = generate_plus_minus_sequence(base);

    println!("Number of elements in the input sequence: {}", pms.len());

    let mut output_sequence = vec![true; n];
    output_sequence[0] = false;

    let chunks: Vec<Vec<usize>> = pms
        .into_iter()
        .collect::<Vec<_>>()
        .chunks(num_threads)
        .map(|c| c.to_vec())
        .collect();

    for chunk in chunks {
        let chunk = chunk.clone();
        let mut handles = Vec::new();

        for x in chunk {
            let handle = thread::spawn(move || {
                let mut ws = vec![true; n];
                ws[0] = false;

                let mut x = x;

                let x_plus_one = x + 1;
                let x_minus_one = x - 1;

                loop {
                    x += x_plus_one;

                    if x >= n {
                        break;
                    }

                    ws[x - 1] = false;

                    x += x_minus_one;

                    if x >= n {
                        break;
                    }

                    ws[x - 1] = false;
                }

                ws
            });

            handles.push(handle);
        }

        for handle in handles {
            let ws = handle.join().unwrap();

            for i in 0..n {
                output_sequence[i] = output_sequence[i] && ws[i];
            }
        }
    }

    let result: Vec<usize> = output_sequence
        .iter()
        .enumerate()
        .filter_map(|(i, val)| if *val == true { Some(i + 1) } else { None })
        .collect();

    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

    println!("Number of threads: {}", num_threads);
    println!("Upper limit: {}", n);
    println!(
        "Number of elements in the output sequence: {}",
        result.len()
    );
    // println!("Output sequence: {:?}", result);
    save_to_file(n, &result, "mt");
}
