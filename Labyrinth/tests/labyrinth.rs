// Integration tests for CSES 1193 – Labyrinth
//
// Tests the binary (BFS via stdin/stdout) and all three library algorithms.

use labyrinth::{solve_astar, solve_bfs, solve_bidirectional};
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_labyrinth");
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

fn parse_grid(s: &str) -> Vec<Vec<u8>> {
    s.lines().map(|line| line.as_bytes().to_vec()).collect()
}

fn find_cell(grid: &[Vec<u8>], target: u8) -> Option<(usize, usize)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some((r, c));
            }
        }
    }
    None
}

/// Validate that `path` walks from A to B without crossing walls,
/// stays in bounds, and has length matching `dist`.
fn validate_path(grid: &[Vec<u8>], dist: usize, path: &str) {
    assert_eq!(path.len(), dist, "path length mismatch");
    let (mut r, mut c) = find_cell(grid, b'A').unwrap();
    let (n, m) = (grid.len(), grid[0].len());
    let end = find_cell(grid, b'B').unwrap();

    for ch in path.bytes() {
        match ch {
            b'L' => {
                assert!(c > 0, "step L out of bounds");
                c -= 1;
            }
            b'R' => {
                c += 1;
                assert!(c < m, "step R out of bounds");
            }
            b'U' => {
                assert!(r > 0, "step U out of bounds");
                r -= 1;
            }
            b'D' => {
                r += 1;
                assert!(r < n, "step D out of bounds");
            }
            _ => panic!("invalid direction {}", ch as char),
        }
        let cell = grid[r][c];
        assert!(
            cell == b'.' || cell == b'A' || cell == b'B',
            "stepped onto wall at ({r}, {c})"
        );
    }
    assert_eq!((r, c), end, "path did not end at B");
}

fn assert_all_algorithms(grid: &[Vec<u8>], expected_dist: Option<usize>) {
    let bfs = solve_bfs(grid);
    let bi = solve_bidirectional(grid);
    let astar = solve_astar(grid);

    match expected_dist {
        None => {
            assert!(bfs.is_none(), "BFS should be unreachable");
            assert!(bi.is_none(), "Bidirectional should be unreachable");
            assert!(astar.is_none(), "A* should be unreachable");
        }
        Some(d) => {
            let (bfs_d, bfs_p) = bfs.expect("BFS should find path");
            let (bi_d, bi_p) = bi.expect("Bidirectional should find path");
            let (a_d, a_p) = astar.expect("A* should find path");
            assert_eq!(bfs_d, d, "BFS distance mismatch");
            assert_eq!(bi_d, d, "Bidirectional distance mismatch");
            assert_eq!(a_d, d, "A* distance mismatch");
            validate_path(grid, bfs_d, &bfs_p);
            validate_path(grid, bi_d, &bi_p);
            validate_path(grid, a_d, &a_p);
        }
    }
}

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

// ---------------------------------------------------------------------------
// Binary tests
// ---------------------------------------------------------------------------

#[test]
fn test_binary_example() {
    let input = "5 8\n########\n#.A#...#\n#.##.#B#\n#......#\n########\n";
    let output = run_with_input(input);
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "YES");
    let dist: usize = lines[1].parse().unwrap();
    assert_eq!(dist, 9);
    let grid = parse_grid("########\n#.A#...#\n#.##.#B#\n#......#\n########");
    validate_path(&grid, dist, lines[2]);
}

#[test]
fn test_binary_unreachable() {
    let input = "3 3\nA#B\n###\n...\n";
    let output = run_with_input(input);
    assert_eq!(output, "NO");
}

#[test]
fn test_binary_adjacent() {
    let input = "1 2\nAB\n";
    let output = run_with_input(input);
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "YES");
    assert_eq!(lines[1], "1");
    assert_eq!(lines[2], "R");
}

#[test]
fn test_binary_straight_line() {
    let input = "1 5\nA...B\n";
    let output = run_with_input(input);
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "YES");
    assert_eq!(lines[1], "4");
    assert_eq!(lines[2], "RRRR");
}

// ---------------------------------------------------------------------------
// Library tests — all three algorithms agree
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    let grid = parse_grid("########\n#.A#...#\n#.##.#B#\n#......#\n########");
    assert_all_algorithms(&grid, Some(9));
}

#[test]
fn test_all_unreachable() {
    let grid = parse_grid("A#B\n###\n...");
    assert_all_algorithms(&grid, None);
}

#[test]
fn test_all_adjacent() {
    let grid = parse_grid("AB");
    assert_all_algorithms(&grid, Some(1));
}

#[test]
fn test_all_straight_line() {
    let grid = parse_grid("A...B");
    assert_all_algorithms(&grid, Some(4));
}

#[test]
fn test_all_must_go_around() {
    // A wall fully blocks the middle row; the only path is the entire
    // bottom row of a 3x3 detour.
    //   A.....
    //   ######
    //   .....B
    // Path: D...DRRRRR is impossible because middle is solid; must go
    // through unblocked column on the right edge.
    //   A....
    //   ####.
    //   B....
    // Distance = 4 (right) + 2 (down) + 4 (left) = 10.
    let grid = parse_grid("A....\n####.\nB....");
    assert_all_algorithms(&grid, Some(10));
}

#[test]
fn test_all_l_shape() {
    let grid = parse_grid("A.\n..\n.B");
    assert_all_algorithms(&grid, Some(3));
}

#[test]
fn test_all_open_grid_diagonal() {
    // Distance from (0,0) to (4,4) on an open 5x5 = 8.
    let grid = parse_grid("A....\n.....\n.....\n.....\n....B");
    assert_all_algorithms(&grid, Some(8));
}

#[test]
fn test_all_random_grids_agree() {
    for seed in 0..5 {
        let grid = generate_grid(20, 20, seed);
        let bfs = solve_bfs(&grid);
        let bi = solve_bidirectional(&grid);
        let astar = solve_astar(&grid);

        match (bfs, bi, astar) {
            (None, None, None) => {}
            (Some((d1, p1)), Some((d2, p2)), Some((d3, p3))) => {
                assert_eq!(d1, d2, "seed={seed}: BFS vs bidirectional distance differ");
                assert_eq!(d1, d3, "seed={seed}: BFS vs A* distance differ");
                validate_path(&grid, d1, &p1);
                validate_path(&grid, d2, &p2);
                validate_path(&grid, d3, &p3);
            }
            other => panic!("seed={seed}: algorithms disagree on reachability: {other:?}"),
        }
    }
}

#[test]
fn test_all_random_large_agree() {
    let grid = generate_grid(100, 100, 99);
    let bfs = solve_bfs(&grid);
    let bi = solve_bidirectional(&grid);
    let astar = solve_astar(&grid);

    match (bfs, bi, astar) {
        (None, None, None) => {}
        (Some((d1, p1)), Some((d2, p2)), Some((d3, p3))) => {
            assert_eq!(d1, d2);
            assert_eq!(d1, d3);
            validate_path(&grid, d1, &p1);
            validate_path(&grid, d2, &p2);
            validate_path(&grid, d3, &p3);
        }
        other => panic!("algorithms disagree on reachability: {other:?}"),
    }
}
