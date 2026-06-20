// Benchmark comparing three Building Teams algorithms.
//
// Generates random graphs with ~2n edges and a known bipartite structure
// (split vertices into two halves, only emit cross edges).

use building_teams::{solve_bfs, solve_dfs, solve_union_find_parity};
use std::time::Instant;

/// Build a guaranteed-bipartite graph with n nodes and ~m edges.
/// Vertices 1..=n/2 form group A; n/2+1..=n form group B; edges only
/// go between groups.
fn generate_bipartite(n: usize, m: usize, seed: u64) -> Vec<(usize, usize)> {
    let half = n / 2;
    let mut rng = seed;
    let mut edges = Vec::with_capacity(m);
    while edges.len() < m {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let a = (((rng >> 33) as usize) % half) + 1;
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let b = (((rng >> 33) as usize) % (n - half)) + half + 1;
        edges.push((a, b));
    }
    edges
}

fn bench<F: Fn(usize, &[(usize, usize)]) -> Option<Vec<u8>>>(
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
        "{:<22} {:>15} {:>15} {:>18}",
        "n / m", "BFS", "DFS", "Union-Find"
    );
    println!("{}", "-".repeat(74));

    for &(n, m) in sizes {
        let edges = generate_bipartite(n, m, 42);
        let iters = if n <= 1_000 {
            500
        } else if n <= 10_000 {
            100
        } else {
            10
        };

        let t_bfs = bench(solve_bfs, n, &edges, iters);
        let t_dfs = bench(solve_dfs, n, &edges, iters);
        let t_uf = bench(solve_union_find_parity, n, &edges, iters);

        println!(
            "n={:<6} m={:<10} {:>15.3?} {:>15.3?} {:>18.3?}",
            n, m, t_bfs, t_dfs, t_uf
        );
    }
}
