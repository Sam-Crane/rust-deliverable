// Integration tests for CSES 1667 – Message Route
//
// Tests the binary (BFS via stdin/stdout) and all three library algorithms.

use message_route::{solve_bfs, solve_bidirectional, solve_dijkstra};
use std::collections::HashSet;
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_message_route");
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

/// Verify the path uses real edges, starts at 1, ends at n, no duplicates.
fn validate_path(n: usize, edges: &[(usize, usize)], path: &[usize]) {
    assert!(!path.is_empty(), "empty path");
    assert_eq!(path[0], 1, "path must start at 1");
    assert_eq!(*path.last().unwrap(), n, "path must end at n");

    let mut edge_set: HashSet<(usize, usize)> = HashSet::new();
    for &(a, b) in edges {
        edge_set.insert((a, b));
        edge_set.insert((b, a));
    }

    for w in path.windows(2) {
        let (u, v) = (w[0], w[1]);
        assert!(
            edge_set.contains(&(u, v)),
            "path edge ({u}, {v}) not in graph"
        );
    }

    let unique: HashSet<usize> = path.iter().copied().collect();
    assert_eq!(unique.len(), path.len(), "path visits a node twice");
}

fn assert_all_algorithms(n: usize, edges: &[(usize, usize)], expected_len: Option<usize>) {
    let bfs = solve_bfs(n, edges);
    let bi = solve_bidirectional(n, edges);
    let dij = solve_dijkstra(n, edges);

    match expected_len {
        None => {
            assert!(bfs.is_none(), "BFS should be IMPOSSIBLE");
            assert!(bi.is_none(), "Bidirectional should be IMPOSSIBLE");
            assert!(dij.is_none(), "Dijkstra should be IMPOSSIBLE");
        }
        Some(len) => {
            let bp = bfs.expect("BFS should find path");
            let bip = bi.expect("Bidirectional should find path");
            let dp = dij.expect("Dijkstra should find path");
            assert_eq!(bp.len(), len, "BFS path length");
            assert_eq!(bip.len(), len, "Bidirectional path length");
            assert_eq!(dp.len(), len, "Dijkstra path length");
            validate_path(n, edges, &bp);
            validate_path(n, edges, &bip);
            validate_path(n, edges, &dp);
        }
    }
}

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

// ---------------------------------------------------------------------------
// Binary tests
// ---------------------------------------------------------------------------

#[test]
fn test_binary_example() {
    let input = "5 5\n1 2\n1 3\n1 4\n2 3\n5 4\n";
    let output = run_with_input(input);
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "3");
    let path: Vec<usize> = lines[1]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    assert_eq!(path[0], 1);
    assert_eq!(path[path.len() - 1], 5);
    assert_eq!(path.len(), 3);
}

#[test]
fn test_binary_impossible() {
    let input = "3 1\n1 2\n";
    let output = run_with_input(input);
    assert_eq!(output, "IMPOSSIBLE");
}

#[test]
fn test_binary_direct_edge() {
    let input = "2 1\n1 2\n";
    let output = run_with_input(input);
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "2");
    assert_eq!(lines[1], "1 2");
}

#[test]
fn test_binary_no_edges_impossible() {
    let input = "5 0\n";
    let output = run_with_input(input);
    assert_eq!(output, "IMPOSSIBLE");
}

// ---------------------------------------------------------------------------
// Library tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    let edges = vec![(1, 2), (1, 3), (1, 4), (2, 3), (5, 4)];
    assert_all_algorithms(5, &edges, Some(3));
}

#[test]
fn test_all_impossible() {
    let edges = vec![(1, 2)];
    assert_all_algorithms(3, &edges, None);
}

#[test]
fn test_all_direct_edge() {
    let edges = vec![(1, 2)];
    assert_all_algorithms(2, &edges, Some(2));
}

#[test]
fn test_all_line_graph() {
    // 1 - 2 - 3 - 4 - 5: shortest path has all 5 nodes.
    let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5)];
    assert_all_algorithms(5, &edges, Some(5));
}

#[test]
fn test_all_with_shortcut() {
    // Line plus shortcut 1-5: shortest is 2 nodes.
    let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5), (1, 5)];
    assert_all_algorithms(5, &edges, Some(2));
}

#[test]
fn test_all_disconnected() {
    let edges = vec![(1, 2), (3, 4), (4, 5)];
    assert_all_algorithms(5, &edges, None);
}

#[test]
fn test_all_random_graphs_agree() {
    for seed in 0..5 {
        let edges = generate_graph(100, 200, seed);
        let bfs = solve_bfs(100, &edges);
        let bi = solve_bidirectional(100, &edges);
        let dij = solve_dijkstra(100, &edges);

        let (b, bi2, d) = (bfs.unwrap(), bi.unwrap(), dij.unwrap());
        assert_eq!(b.len(), bi2.len(), "seed={seed}: BFS vs bi length");
        assert_eq!(b.len(), d.len(), "seed={seed}: BFS vs dijkstra length");
        validate_path(100, &edges, &b);
        validate_path(100, &edges, &bi2);
        validate_path(100, &edges, &d);
    }
}

#[test]
fn test_all_random_large_agree() {
    let edges = generate_graph(5_000, 10_000, 99);
    let bfs = solve_bfs(5_000, &edges).unwrap();
    let bi = solve_bidirectional(5_000, &edges).unwrap();
    let dij = solve_dijkstra(5_000, &edges).unwrap();

    assert_eq!(bfs.len(), bi.len());
    assert_eq!(bfs.len(), dij.len());
    validate_path(5_000, &edges, &bfs);
    validate_path(5_000, &edges, &bi);
    validate_path(5_000, &edges, &dij);
}
