// Integration tests for CSES 1192 – Counting Rooms
//
// Tests the binary (BFS via stdin/stdout) as well as all three algorithm
// implementations exposed by the library.

use counting_rooms::{solve_bfs, solve_dfs, solve_union_find};
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_counting_rooms");
    let output = Command::new(bin)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .take()
                .unwrap()
                .write_all(input.as_bytes())
                .unwrap();
            child.wait_with_output()
        })
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// Convert a multiline grid string into the internal Vec<Vec<u8>> representation.
fn parse_grid(s: &str) -> Vec<Vec<u8>> {
    s.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn assert_all_algorithms(grid: &[Vec<u8>], expected: usize) {
    assert_eq!(solve_bfs(grid), expected, "BFS failed");
    assert_eq!(solve_dfs(grid), expected, "DFS failed");
    assert_eq!(solve_union_find(grid), expected, "Union-Find failed");
}

/// Generate a random grid with a deterministic LCG.
fn generate_grid(n: usize, m: usize, seed: u64) -> Vec<Vec<u8>> {
    let mut rng = seed;
    (0..n)
        .map(|_| {
            (0..m)
                .map(|_| {
                    rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                    if (rng >> 33) % 10 < 7 { b'.' } else { b'#' }
                })
                .collect()
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Binary tests (BFS via stdin/stdout)
// ---------------------------------------------------------------------------

#[test]
fn test_binary_example() {
    let input = "5 8\n########\n#..#...#\n####.#.#\n#..#...#\n########\n";
    let output = run_with_input(input);
    assert_eq!(output, "3");
}

#[test]
fn test_binary_single_floor_cell() {
    let output = run_with_input("1 1\n.\n");
    assert_eq!(output, "1");
}

#[test]
fn test_binary_single_wall_cell() {
    let output = run_with_input("1 1\n#\n");
    assert_eq!(output, "0");
}

#[test]
fn test_binary_all_floor() {
    let output = run_with_input("3 3\n...\n...\n...\n");
    assert_eq!(output, "1");
}

#[test]
fn test_binary_all_walls() {
    let output = run_with_input("3 3\n###\n###\n###\n");
    assert_eq!(output, "0");
}

#[test]
fn test_binary_isolated_cells() {
    // Each floor cell is isolated by walls → 4 rooms.
    let output = run_with_input("3 3\n.#.\n###\n.#.\n");
    assert_eq!(output, "4");
}

// ---------------------------------------------------------------------------
// Library tests — all three algorithms
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    let grid = parse_grid("########\n#..#...#\n####.#.#\n#..#...#\n########");
    assert_all_algorithms(&grid, 3);
}

#[test]
fn test_all_single_floor() {
    let grid = parse_grid(".");
    assert_all_algorithms(&grid, 1);
}

#[test]
fn test_all_single_wall() {
    let grid = parse_grid("#");
    assert_all_algorithms(&grid, 0);
}

#[test]
fn test_all_all_floor() {
    let grid = parse_grid("...\n...\n...");
    assert_all_algorithms(&grid, 1);
}

#[test]
fn test_all_all_walls() {
    let grid = parse_grid("###\n###\n###");
    assert_all_algorithms(&grid, 0);
}

#[test]
fn test_all_isolated_cells() {
    let grid = parse_grid(".#.\n###\n.#.");
    assert_all_algorithms(&grid, 4);
}

#[test]
fn test_all_diagonal_no_connect() {
    // Diagonal floor cells are NOT adjacent (only 4-connectivity).
    let grid = parse_grid(".#\n#.");
    assert_all_algorithms(&grid, 2);
}

#[test]
fn test_all_long_corridor() {
    // 1x10 row of floor → 1 room.
    let grid = parse_grid("..........");
    assert_all_algorithms(&grid, 1);
}

#[test]
fn test_all_two_corridors() {
    let grid = parse_grid("...#...");
    assert_all_algorithms(&grid, 2);
}

#[test]
fn test_all_random_grids_agree() {
    for seed in 0..5 {
        let grid = generate_grid(50, 50, seed);
        let bfs = solve_bfs(&grid);
        let dfs = solve_dfs(&grid);
        let uf = solve_union_find(&grid);
        assert_eq!(bfs, dfs, "seed={seed}: BFS vs DFS differ");
        assert_eq!(bfs, uf, "seed={seed}: BFS vs Union-Find differ");
    }
}

#[test]
fn test_all_random_large_agree() {
    let grid = generate_grid(200, 200, 99);
    let bfs = solve_bfs(&grid);
    let dfs = solve_dfs(&grid);
    let uf = solve_union_find(&grid);
    assert_eq!(bfs, dfs, "BFS vs DFS differ on large grid");
    assert_eq!(bfs, uf, "BFS vs Union-Find differ on large grid");
}
