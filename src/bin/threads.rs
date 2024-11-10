use std::thread;
use std::time::Instant;

fn main() {
    // Get number of available threads
    let num_threads = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);

    println!("Available threads: {}", num_threads);

    // Create a large vector to process
    let data: Vec<u64> = (0..1000_000_000).collect();
    let chunk_size = data.len() / num_threads;

    // Single thread benchmark
    let start = Instant::now();
    let single_sum: u64 = data.iter().sum();
    let single_thread_time = start.elapsed();
    println!("Single thread time: {:?}", single_thread_time);

    // Multi thread benchmark
    let start = Instant::now();
    let mut handles = Vec::new();

    // Split work among threads
    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || chunk.iter().sum::<u64>());
        handles.push(handle);
    }

    // Join threads and sum results
    let multi_sum: u64 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    let multi_thread_time = start.elapsed();
    println!("Multi thread time: {:?}", multi_thread_time);

    // Verify results match
    assert_eq!(single_sum, multi_sum);
    println!("Results match! Sum = {}", single_sum);

    // Calculate speedup
    let speedup = single_thread_time.as_secs_f64() / multi_thread_time.as_secs_f64();
    println!("Speedup: {:.2}x", speedup);
}
