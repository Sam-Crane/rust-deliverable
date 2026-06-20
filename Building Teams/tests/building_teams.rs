// Integration tests for CSES 1668 – Building Teams
//
// Tests the binary (BFS via stdin/stdout) and all three library algorithms.

use building_teams::{solve_bfs, solve_dfs, solve_union_find_parity};
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_building_teams");
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

/// Validate that for every edge, the two endpoints have different teams.
fn validate_assignment(n: usize, edges: &[(usize, usize)], teams: &[u8]) {
    assert_eq!(teams.len(), n, "team vector length");
    for &t in teams {
        assert!(t == 1 || t == 2, "team must be 1 or 2, got {t}");
    }
    for &(a, b) in edges {
        assert_ne!(
            teams[a - 1],
            teams[b - 1],
            "edge ({a}, {b}) endpoints on same team"
        );
    }
}

fn assert_all_algorithms(n: usize, edges: &[(usize, usize)], expect_possible: bool) {
    let bfs = solve_bfs(n, edges);
    let dfs = solve_dfs(n, edges);
    let uf = solve_union_find_parity(n, edges);

    if expect_possible {
        let bp = bfs.expect("BFS should succeed");
        let dp = dfs.expect("DFS should succeed");
        let up = uf.expect("Union-Find should succeed");
        validate_assignment(n, edges, &bp);
        validate_assignment(n, edges, &dp);
        validate_assignment(n, edges, &up);
    } else {
        assert!(bfs.is_none(), "BFS should be IMPOSSIBLE");
        assert!(dfs.is_none(), "DFS should be IMPOSSIBLE");
        assert!(uf.is_none(), "Union-Find should be IMPOSSIBLE");
    }
}

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

// ---------------------------------------------------------------------------
// Binary tests
// ---------------------------------------------------------------------------

#[test]
fn test_binary_example() {
    let input = "5 3\n1 2\n1 3\n4 5\n";
    let output = run_with_input(input);
    // Multiple valid colorings exist; verify the structural constraint.
    let teams: Vec<u8> = output
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(teams.len(), 5);
    let edges = vec![(1, 2), (1, 3), (4, 5)];
    for &(a, b) in &edges {
        assert_ne!(teams[a - 1], teams[b - 1]);
    }
}

#[test]
fn test_binary_odd_cycle_impossible() {
    // Triangle 1-2-3-1 has an odd cycle → not bipartite.
    let input = "3 3\n1 2\n2 3\n3 1\n";
    let output = run_with_input(input);
    assert_eq!(output, "IMPOSSIBLE");
}

#[test]
fn test_binary_single_vertex() {
    let input = "1 0\n";
    let output = run_with_input(input);
    assert_eq!(output, "1");
}

#[test]
fn test_binary_two_disjoint_edges() {
    let input = "4 2\n1 2\n3 4\n";
    let output = run_with_input(input);
    let teams: Vec<u8> = output
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(teams.len(), 4);
    assert_ne!(teams[0], teams[1]);
    assert_ne!(teams[2], teams[3]);
}

// ---------------------------------------------------------------------------
// Library tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    let edges = vec![(1, 2), (1, 3), (4, 5)];
    assert_all_algorithms(5, &edges, true);
}

#[test]
fn test_all_triangle_impossible() {
    let edges = vec![(1, 2), (2, 3), (3, 1)];
    assert_all_algorithms(3, &edges, false);
}

#[test]
fn test_all_square_possible() {
    // 4-cycle: 1-2-3-4-1 — bipartite (even cycle).
    let edges = vec![(1, 2), (2, 3), (3, 4), (4, 1)];
    assert_all_algorithms(4, &edges, true);
}

#[test]
fn test_all_pentagon_impossible() {
    // 5-cycle is an odd cycle.
    let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 1)];
    assert_all_algorithms(5, &edges, false);
}

#[test]
fn test_all_no_edges() {
    assert_all_algorithms(5, &[], true);
}

#[test]
fn test_all_disconnected_components() {
    // Component 1: triangle (odd) → IMPOSSIBLE.
    let edges = vec![(1, 2), (2, 3), (3, 1), (4, 5), (5, 6)];
    assert_all_algorithms(6, &edges, false);
}

#[test]
fn test_all_disconnected_components_all_ok() {
    let edges = vec![(1, 2), (3, 4), (5, 6)];
    assert_all_algorithms(6, &edges, true);
}

#[test]
fn test_all_self_loop_impossible() {
    // Self-loop forces a node to differ from itself.
    let edges = vec![(1, 1)];
    assert_all_algorithms(3, &edges, false);
}

#[test]
fn test_all_complete_bipartite_k33() {
    // K_{3,3} — every left-3 connects to every right-3.
    let mut edges = Vec::new();
    for a in 1..=3 {
        for b in 4..=6 {
            edges.push((a, b));
        }
    }
    assert_all_algorithms(6, &edges, true);
}

#[test]
fn test_all_random_bipartite_agree() {
    for seed in 0..5 {
        let edges = generate_bipartite(100, 200, seed);
        let bfs = solve_bfs(100, &edges).expect("bipartite");
        let dfs = solve_dfs(100, &edges).expect("bipartite");
        let uf = solve_union_find_parity(100, &edges).expect("bipartite");
        validate_assignment(100, &edges, &bfs);
        validate_assignment(100, &edges, &dfs);
        validate_assignment(100, &edges, &uf);
    }
}

#[test]
fn test_all_random_large_bipartite() {
    let edges = generate_bipartite(5_000, 10_000, 99);
    let bfs = solve_bfs(5_000, &edges).expect("bipartite");
    let dfs = solve_dfs(5_000, &edges).expect("bipartite");
    let uf = solve_union_find_parity(5_000, &edges).expect("bipartite");
    validate_assignment(5_000, &edges, &bfs);
    validate_assignment(5_000, &edges, &dfs);
    validate_assignment(5_000, &edges, &uf);
}
