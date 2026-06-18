// Benchmark comparing three Message Route algorithms.
//
// Generates random connected-ish graphs with ~2n edges and measures
// wall-clock time for each algorithm.

use message_route::{solve_bfs, solve_bidirectional, solve_dijkstra};
use std::time::Instant;

/// Build a random graph with n nodes and m edges using an LCG.
/// Ensures node n is reachable from 1 by first wiring a hamiltonian path
/// 1->2->...->n, then adding (m - (n-1)) random extra edges.
fn generate_graph(n: usize, m: usize, seed: u64) -> Vec<(usize, usize)> {
    let mut edges = Vec::with_capacity(m);
    for i in 1..n {
        edges.push((i, i + 1));
    }
    let mut rng = seed;
    while edges.len() < m {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = (((rng >> 33) as usize) % n) + 1;
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let b = (((rng >> 33) as usize) % n) + 1;
        if a != b {
            edges.push((a, b));
        }
    }
    edges
}

fn bench<F: Fn(usize, &[(usize, usize)]) -> Option<Vec<usize>>>(
    f: F,
    n: usize,
    edges: &[(usize, usize)],
    iterations: u32,
) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(n, edges));
    }
    start.elapsed() / iterations
}

fn main() {
    let sizes: &[(usize, usize)] = &[(1_000, 2_000), (10_000, 20_000), (100_000, 200_000)];

    println!(
        "{:<22} {:>15} {:>15} {:>15}",
        "n / m", "BFS", "Bidirectional", "Dijkstra"
    );
    println!("{}", "-".repeat(70));

    for &(n, m) in sizes {
        let edges = generate_graph(n, m, 42);
        let iters = if n <= 1_000 {
            500
        } else if n <= 10_000 {
            100
        } else {
            10
        };

        let t_bfs = bench(solve_bfs, n, &edges, iters);
        let t_bi = bench(solve_bidirectional, n, &edges, iters);
        let t_dij = bench(solve_dijkstra, n, &edges, iters);

        println!(
            "n={:<6} m={:<10} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, m, t_bfs, t_bi, t_dij
        );
    }
}
