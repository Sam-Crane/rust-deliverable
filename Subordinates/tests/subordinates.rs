// Integration tests for CSES 1674 – Subordinates
//
// Tests the binary (iterative DFS via stdin/stdout) as well as all three
// algorithm implementations exposed by the library.

use std::process::Command;
use subordinates::{solve_iterative, solve_recursive, solve_reverse_bfs};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Spawn the binary, pipe `input` to its stdin, and return stdout trimmed.
fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_subordinates");
    let output = Command::new(bin)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
            child.wait_with_output()
        })
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// Run all three algorithms on the same input and assert they return `expected`.
fn assert_all_algorithms(bosses: &[usize], expected: &[usize]) {
    let r = solve_recursive(bosses);
    let it = solve_iterative(bosses);
    let rb = solve_reverse_bfs(bosses);
    assert_eq!(r, expected, "recursive mismatch");
    assert_eq!(it, expected, "iterative mismatch");
    assert_eq!(rb, expected, "reverse-bfs mismatch");
}

/// Generate a random tree (deterministic) for fuzzing all three algorithms.
fn generate_tree(n: usize, seed: u64) -> Vec<usize> {
    let mut rng = seed;
    (2..=n)
        .map(|i| {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            (((rng >> 33) as usize) % (i - 1)) + 1
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Binary tests (iterative DFS via stdin/stdout)
// ---------------------------------------------------------------------------

/// Example from the problem statement.
#[test]
fn test_binary_example() {
    let output = run_with_input("5\n1 1 2 3\n");
    assert_eq!(output, "4 1 1 0 0");
}

#[test]
fn test_binary_single_employee() {
    let output = run_with_input("1\n");
    assert_eq!(output, "0");
}

#[test]
fn test_binary_two_employees() {
    let output = run_with_input("2\n1\n");
    assert_eq!(output, "1 0");
}

/// Linear chain: 1 → 2 → 3 → 4 → 5. Counts: 4, 3, 2, 1, 0.
#[test]
fn test_binary_linear_chain() {
    let output = run_with_input("5\n1 2 3 4\n");
    assert_eq!(output, "4 3 2 1 0");
}

/// Star: every employee reports directly to 1.
#[test]
fn test_binary_star() {
    let output = run_with_input("5\n1 1 1 1\n");
    assert_eq!(output, "4 0 0 0 0");
}

// ---------------------------------------------------------------------------
// Library tests — all three algorithms
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    assert_all_algorithms(&[1, 1, 2, 3], &[4, 1, 1, 0, 0]);
}

#[test]
fn test_all_single_employee() {
    assert_all_algorithms(&[], &[0]);
}

#[test]
fn test_all_two_employees() {
    assert_all_algorithms(&[1], &[1, 0]);
}

#[test]
fn test_all_linear_chain() {
    // 1 → 2 → 3 → 4 → 5
    assert_all_algorithms(&[1, 2, 3, 4], &[4, 3, 2, 1, 0]);
}

#[test]
fn test_all_star() {
    // 1 has children 2, 3, 4, 5
    assert_all_algorithms(&[1, 1, 1, 1], &[4, 0, 0, 0, 0]);
}

#[test]
fn test_all_balanced_binary() {
    // 1 -> 2, 3; 2 -> 4, 5; 3 -> 6, 7
    // bosses for 2..7: [1, 1, 2, 2, 3, 3]
    assert_all_algorithms(&[1, 1, 2, 2, 3, 3], &[6, 2, 2, 0, 0, 0, 0]);
}

#[test]
fn test_all_random_small() {
    // For random trees, just verify the three algorithms agree.
    for seed in 0..10 {
        let bosses = generate_tree(50, seed);
        let r = solve_recursive(&bosses);
        let it = solve_iterative(&bosses);
        let rb = solve_reverse_bfs(&bosses);
        assert_eq!(r, it, "seed={seed}: recursive vs iterative differ");
        assert_eq!(r, rb, "seed={seed}: recursive vs reverse-bfs differ");
        // Sanity: total subordinates == n - 1 (everyone except root counted once).
        assert_eq!(r[0], 49);
    }
}

#[test]
fn test_all_random_large() {
    let bosses = generate_tree(10_000, 12345);
    let it = solve_iterative(&bosses);
    let rb = solve_reverse_bfs(&bosses);
    assert_eq!(it, rb, "iterative vs reverse-bfs differ on large random tree");
    assert_eq!(it[0], 9999);
    // Skip recursive on large input to avoid potential stack issues on chains.
}
