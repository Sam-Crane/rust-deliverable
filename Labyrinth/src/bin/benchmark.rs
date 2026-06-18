// Benchmark comparing three Labyrinth algorithms.
//
// Generates random grids with ~75% floor / ~25% wall and places A in the
// top-left corner area, B in the bottom-right corner area.

use labyrinth::{solve_astar, solve_bfs, solve_bidirectional};
use std::time::Instant;

fn generate_grid(n: usize, m: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = seed;
    let mut grid: Vec<Vec<u8>> = (0..n)
        .map(|_| {
            (0..m)
                .map(|_| {
                    rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                    if (rng >> 33) % 4 == 0 { b'#' } else { b'.' }
                })
                .collect()
        })
        .collect();
    grid[0][0] = b'A';
    grid[n - 1][m - 1] = b'B';
    grid
}

fn bench<F: Fn(&[Vec<u8>]) -> Option<(usize, String)>>(
    f: F,
    data: &[Vec<u8>],
    iterations: u32,
) -> std::time::Duration {
    let start = Instant::now();
    for _ in 0..iterations {
        std::hint::black_box(f(data));
    }
    start.elapsed() / iterations
}

fn main() {
    let sizes: &[(usize, usize)] = &[(100, 100), (300, 300), (500, 500), (1000, 1000)];

    println!(
        "{:<14} {:>15} {:>15} {:>15}",
        "Grid", "BFS", "Bidirectional", "A*"
    );
    println!("{}", "-".repeat(62));

    for &(n, m) in sizes {
        let grid = generate_grid(n, m, 42);
        let cells = n * m;
        let iters = if cells <= 10_000 {
            50
        } else if cells <= 100_000 {
            20
        } else {
            5
        };

        let t_bfs = bench(solve_bfs, &grid, iters);
        let t_bi = bench(solve_bidirectional, &grid, iters);
        let t_a = bench(solve_astar, &grid, iters);

        println!(
            "{}x{:<10} {:>15.3?} {:>15.3?} {:>15.3?}",
            n, m, t_bfs, t_bi, t_a
        );
    }
}
