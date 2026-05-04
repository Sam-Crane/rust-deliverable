// Integration tests for CSES 1092 – Two Sets
//
// Tests the binary (greedy solver via stdin/stdout) as well as all three
// algorithm implementations exposed by the library.

use std::process::Command;
use two_sets::{solve_greedy, solve_pairs, solve_recursive};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Spawn the binary, pipe `input` to its stdin, and return stdout trimmed.
fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_two_sets");
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

/// Validate that a partition is correct: both sets together contain exactly
/// {1..n} and have equal sums.
fn validate_partition(n: u64, set1: &[u64], set2: &[u64]) {
    // All elements present exactly once.
    let mut seen = vec![false; (n + 1) as usize];
    for &x in set1.iter().chain(set2.iter()) {
        assert!(x >= 1 && x <= n, "Element {x} out of range");
        assert!(!seen[x as usize], "Duplicate element {x}");
        seen[x as usize] = true;
    }
    for i in 1..=n {
        assert!(seen[i as usize], "Missing element {i}");
    }

    // Equal sums.
    let sum1: u64 = set1.iter().sum();
    let sum2: u64 = set2.iter().sum();
    assert_eq!(sum1, sum2, "Sums differ: {sum1} vs {sum2}");
}

/// Run all three algorithms on n and validate each result.
fn assert_all_algorithms(n: u64, should_exist: bool) {
    let results = [
        ("greedy", solve_greedy(n)),
        ("pairs", solve_pairs(n)),
        ("recursive", solve_recursive(n)),
    ];

    for (name, result) in &results {
        if should_exist {
            let (set1, set2) = result.as_ref().unwrap_or_else(|| {
                panic!("{name} returned None for n={n}")
            });
            validate_partition(n, set1, set2);
        } else {
            assert!(result.is_none(), "{name} should return None for n={n}");
        }
    }
}

// ---------------------------------------------------------------------------
// Binary tests (greedy via stdin/stdout)
// ---------------------------------------------------------------------------

#[test]
fn test_binary_n7() {
    let output = run_with_input("7\n");
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "YES");
    // Verify the partition is valid by parsing.
    let k1: usize = lines[1].parse().unwrap();
    let s1: Vec<u64> = lines[2].split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert_eq!(s1.len(), k1);
    let k2: usize = lines[3].parse().unwrap();
    let s2: Vec<u64> = lines[4].split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert_eq!(s2.len(), k2);
    validate_partition(7, &s1, &s2);
}

#[test]
fn test_binary_n6_impossible() {
    let output = run_with_input("6\n");
    assert_eq!(output, "NO");
}

#[test]
fn test_binary_n1_impossible() {
    let output = run_with_input("1\n");
    assert_eq!(output, "NO");
}

#[test]
fn test_binary_n2_impossible() {
    let output = run_with_input("2\n");
    assert_eq!(output, "NO");
}

#[test]
fn test_binary_n3() {
    let output = run_with_input("3\n");
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "YES");
}

#[test]
fn test_binary_n4() {
    let output = run_with_input("4\n");
    let lines: Vec<&str> = output.lines().collect();
    assert_eq!(lines[0], "YES");
    let k1: usize = lines[1].parse().unwrap();
    let s1: Vec<u64> = lines[2].split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert_eq!(s1.len(), k1);
    let k2: usize = lines[3].parse().unwrap();
    let s2: Vec<u64> = lines[4].split_whitespace().map(|x| x.parse().unwrap()).collect();
    assert_eq!(s2.len(), k2);
    validate_partition(4, &s1, &s2);
}

// ---------------------------------------------------------------------------
// Library tests — all three algorithms
// ---------------------------------------------------------------------------

#[test]
fn test_all_impossible_cases() {
    // n % 4 == 1 or 2 → impossible
    for n in [1, 2, 5, 6, 9, 10, 13, 14] {
        assert_all_algorithms(n, false);
    }
}

#[test]
fn test_all_n3() {
    assert_all_algorithms(3, true);
}

#[test]
fn test_all_n4() {
    assert_all_algorithms(4, true);
}

#[test]
fn test_all_n7() {
    assert_all_algorithms(7, true);
}

#[test]
fn test_all_n8() {
    assert_all_algorithms(8, true);
}

#[test]
fn test_all_small_possible() {
    // Test all possible cases up to n=100
    for n in 1..=100u64 {
        let total = n * (n + 1) / 2;
        let possible = total % 2 == 0;
        assert_all_algorithms(n, possible);
    }
}

#[test]
fn test_all_n1000() {
    assert_all_algorithms(1000, true);
}

#[test]
fn test_all_n9999() {
    // n % 4 == 3
    assert_all_algorithms(9999, true);
}

#[test]
fn test_all_n10000() {
    // n % 4 == 0
    assert_all_algorithms(10000, true);
}
