// Integration tests for CSES 1094 – Increasing Array
//
// Each test pipes input to the binary and checks that the output equals
// the expected minimum number of moves to make the array non-decreasing.

use std::process::Command;

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
            child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
            child.wait_with_output()
        })
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// Example from the problem statement: [3, 2, 5, 1, 7] → 5 moves.
/// 2 must become 3 (+1), and 1 must become 5 (+4). Total = 5.
#[test]
fn test_example() {
    let result = run_with_input("5\n3 2 5 1 7\n");
    assert_eq!(result, "5");
}

/// Single element – already non-decreasing, no moves needed.
#[test]
fn test_single_element() {
    let result = run_with_input("1\n42\n");
    assert_eq!(result, "0");
}

/// Already sorted – no moves needed.
#[test]
fn test_already_sorted() {
    let result = run_with_input("4\n1 2 3 4\n");
    assert_eq!(result, "0");
}

/// All elements equal – already non-decreasing.
#[test]
fn test_all_equal() {
    let result = run_with_input("3\n5 5 5\n");
    assert_eq!(result, "0");
}

/// Strictly decreasing array – worst case for a given set of values.
/// [5, 4, 3, 2, 1]: each element raised to 5 → moves = 1+2+3+4 = 10.
#[test]
fn test_strictly_decreasing() {
    let result = run_with_input("5\n5 4 3 2 1\n");
    assert_eq!(result, "10");
}

/// Two elements where the second is smaller.
#[test]
fn test_two_elements_decreasing() {
    let result = run_with_input("2\n10 1\n");
    assert_eq!(result, "9");
}

/// Two elements already non-decreasing.
#[test]
fn test_two_elements_increasing() {
    let result = run_with_input("2\n1 10\n");
    assert_eq!(result, "0");
}

/// Large values near the 10^9 constraint to test for overflow.
/// [1000000000, 1] → need 999999999 moves.
#[test]
fn test_large_values() {
    let result = run_with_input("2\n1000000000 1\n");
    assert_eq!(result, "999999999");
}

/// Multiple consecutive drops to verify running-maximum tracking.
/// [10, 1, 1, 1] → each 1 raised to 10: moves = 9+9+9 = 27.
#[test]
fn test_consecutive_drops() {
    let result = run_with_input("4\n10 1 1 1\n");
    assert_eq!(result, "27");
}
