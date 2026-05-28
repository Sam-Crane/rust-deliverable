// Benchmark comparing three Tower of Hanoi algorithms.
//
// Measures wall-clock time for each algorithm at several disk counts
// and prints a comparison table.

use std::time::Instant;
use tower_of_hanoi::{solve_iterative_bits, solve_iterative_stacks, solve_recursive};

fn bench<F: Fn() -> Vec<(u8, u8)>>(f: F, iterations: u32) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f());
    }
    start.elapsed() / iterations
}

fn main() {
    let disk_counts: &[u32] = &[10, 15, 20, 25];

    println!(
        "{:<8} {:>18} {:>18} {:>18}",
        "Disks", "Recursive", "Bit-Manipulation", "Stack-Based"
    );
    println!("{}", "-".repeat(64));

    for &n in disk_counts {
        // Fewer iterations for larger n to keep runtime reasonable.
        let iters = if n <= 15 {
            100
        } else if n <= 20 {
            10
        } else {
            3
        };

        let t_rec = bench(|| solve_recursive(n), iters);
        let t_bits = bench(|| solve_iterative_bits(n), iters);
        let t_stacks = bench(|| solve_iterative_stacks(n), iters);

        println!(
            "n={:<5} {:>15.3?} {:>18.3?} {:>18.3?}",
            n, t_rec, t_bits, t_stacks
        );
    }
}
