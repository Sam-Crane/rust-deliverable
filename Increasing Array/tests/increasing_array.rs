// Integration tests for CSES 1094 – Increasing Array
//
// Tests the binary (greedy solver via stdin/stdout) as well as all three
// algorithm implementations exposed by the library.

use increasing_array::{solve_fold, solve_greedy, solve_prefix_max};
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Spawn the binary, pipe `input` to its stdin, and return stdout trimmed.
fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_increasing_array");
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

/// Run all three algorithms on the same input and assert they return the expected value.
fn assert_all_algorithms(nums: &[i64], expected: i64) {
    assert_eq!(solve_greedy(nums), expected, "greedy failed");
    assert_eq!(solve_fold(nums), expected, "fold failed");
    assert_eq!(solve_prefix_max(nums), expected, "prefix_max failed");
}

// ---------------------------------------------------------------------------
// Binary tests (greedy algorithm via stdin/stdout)
// ---------------------------------------------------------------------------

/// Example from the problem statement: [3, 2, 5, 1, 7] → 5 moves.
/// 2 must become 3 (+1), and 1 must become 5 (+4). Total = 5.
#[test]
fn test_binary_example() {
    let result = run_with_input("5\n3 2 5 1 7\n");
    assert_eq!(result, "5");
}

/// Single element – already non-decreasing, no moves needed.
#[test]
fn test_binary_single_element() {
    let result = run_with_input("1\n42\n");
    assert_eq!(result, "0");
}

/// Already sorted – no moves needed.
#[test]
fn test_binary_already_sorted() {
    let result = run_with_input("4\n1 2 3 4\n");
    assert_eq!(result, "0");
}

/// All elements equal – already non-decreasing.
#[test]
fn test_binary_all_equal() {
    let result = run_with_input("3\n5 5 5\n");
    assert_eq!(result, "0");
}

/// Strictly decreasing array – worst case for a given set of values.
/// [5, 4, 3, 2, 1]: each element raised to 5 → moves = 1+2+3+4 = 10.
#[test]
fn test_binary_strictly_decreasing() {
    let result = run_with_input("5\n5 4 3 2 1\n");
    assert_eq!(result, "10");
}

/// Two elements where the second is smaller.
#[test]
fn test_binary_two_elements_decreasing() {
    let result = run_with_input("2\n10 1\n");
    assert_eq!(result, "9");
}

/// Two elements already non-decreasing.
#[test]
fn test_binary_two_elements_increasing() {
    let result = run_with_input("2\n1 10\n");
    assert_eq!(result, "0");
}

/// Large values near the 10^9 constraint to test for overflow.
/// [1000000000, 1] → need 999999999 moves.
#[test]
fn test_binary_large_values() {
    let result = run_with_input("2\n1000000000 1\n");
    assert_eq!(result, "999999999");
}

/// Multiple consecutive drops to verify running-maximum tracking.
/// [10, 1, 1, 1] → each 1 raised to 10: moves = 9+9+9 = 27.
#[test]
fn test_binary_consecutive_drops() {
    let result = run_with_input("4\n10 1 1 1\n");
    assert_eq!(result, "27");
}

// ---------------------------------------------------------------------------
// Library tests — all three algorithms agree on every case
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    assert_all_algorithms(&[3, 2, 5, 1, 7], 5);
}

#[test]
fn test_all_single_element() {
    assert_all_algorithms(&[42], 0);
}

#[test]
fn test_all_already_sorted() {
    assert_all_algorithms(&[1, 2, 3, 4], 0);
}

#[test]
fn test_all_equal() {
    assert_all_algorithms(&[5, 5, 5], 0);
}

#[test]
fn test_all_strictly_decreasing() {
    assert_all_algorithms(&[5, 4, 3, 2, 1], 10);
}

#[test]
fn test_all_two_elements_decreasing() {
    assert_all_algorithms(&[10, 1], 9);
}

#[test]
fn test_all_two_elements_increasing() {
    assert_all_algorithms(&[1, 10], 0);
}

#[test]
fn test_all_large_values() {
    assert_all_algorithms(&[1_000_000_000, 1], 999_999_999);
}

#[test]
fn test_all_consecutive_drops() {
    assert_all_algorithms(&[10, 1, 1, 1], 27);
}

#[test]
fn test_all_alternating() {
    // [1, 100, 2, 100, 3] → 2 raised to 100 (+98), 3 raised to 100 (+97) = 195
    assert_all_algorithms(&[1, 100, 2, 100, 3], 195);
}

#[test]
fn test_all_large_n_decreasing() {
    // 1000 elements from 1000 down to 1 → sum of 0+1+2+...+999 = 499500
    let nums: Vec<i64> = (1..=1000).rev().collect();
    assert_all_algorithms(&nums, 499500);
}
