// Integration tests for CSES 2208 – Another Game
//
// Tests the binary via stdin/stdout as well as all three algorithm
// implementations exposed by the library.

use another_game::{solve_bitor, solve_iter, solve_scan};
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_another_game");
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

fn assert_all_algorithms(heaps: &[u64], expected: bool) {
    assert_eq!(solve_scan(heaps), expected, "scan failed on {heaps:?}");
    assert_eq!(solve_iter(heaps), expected, "iter failed on {heaps:?}");
    assert_eq!(solve_bitor(heaps), expected, "bitor failed on {heaps:?}");
}

// ---------------------------------------------------------------------------
// Binary tests
// ---------------------------------------------------------------------------

#[test]
fn test_binary_example() {
    let input = "3\n3\n1 2 3\n2\n2 2\n4\n5 5 4 5\n";
    let output = run_with_input(input);
    assert_eq!(output, "first\nsecond\nfirst");
}

#[test]
fn test_binary_single_heap_one_coin() {
    let output = run_with_input("1\n1\n1\n");
    assert_eq!(output, "first");
}

#[test]
fn test_binary_single_heap_two_coins() {
    let output = run_with_input("1\n1\n2\n");
    assert_eq!(output, "second");
}

#[test]
fn test_binary_all_even() {
    let output = run_with_input("1\n5\n2 4 6 8 10\n");
    assert_eq!(output, "second");
}

#[test]
fn test_binary_all_odd() {
    let output = run_with_input("1\n4\n1 3 5 7\n");
    assert_eq!(output, "first");
}

#[test]
fn test_binary_large_values() {
    // 10^9 is even, 10^9 - 1 is odd.
    let output = run_with_input("2\n1\n1000000000\n1\n999999999\n");
    assert_eq!(output, "second\nfirst");
}

// ---------------------------------------------------------------------------
// Library tests
// ---------------------------------------------------------------------------

#[test]
fn test_all_example_cases() {
    assert_all_algorithms(&[1, 2, 3], true);
    assert_all_algorithms(&[2, 2], false);
    assert_all_algorithms(&[5, 5, 4, 5], true);
}

#[test]
fn test_all_single_heap() {
    assert_all_algorithms(&[1], true);
    assert_all_algorithms(&[2], false);
    assert_all_algorithms(&[3], true);
}

#[test]
fn test_all_even() {
    assert_all_algorithms(&[2, 4, 6, 8, 10], false);
    assert_all_algorithms(&[1_000_000_000], false);
}

#[test]
fn test_all_odd() {
    assert_all_algorithms(&[1, 3, 5, 7], true);
    assert_all_algorithms(&[999_999_999], true);
}

#[test]
fn test_all_mixed_odd_at_front() {
    let mut v: Vec<u64> = (1..=100).map(|i| i * 2).collect();
    v[0] = 1;
    assert_all_algorithms(&v, true);
}

#[test]
fn test_all_mixed_odd_at_end() {
    let mut v: Vec<u64> = (1..=100).map(|i| i * 2).collect();
    let last = v.len() - 1;
    v[last] += 1;
    assert_all_algorithms(&v, true);
}

#[test]
fn test_all_random_small_agree() {
    // Cross-validate algorithms on deterministic pseudo-random inputs.
    let mut rng: u64 = 0xdeadbeef;
    for _ in 0..50 {
        let n = {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            ((rng >> 33) as usize % 50) + 1
        };
        let heaps: Vec<u64> = (0..n)
            .map(|_| {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                (rng >> 33) % 1000 + 1
            })
            .collect();
        let s = solve_scan(&heaps);
        let i = solve_iter(&heaps);
        let b = solve_bitor(&heaps);
        assert_eq!(s, i, "scan vs iter mismatch on {heaps:?}");
        assert_eq!(s, b, "scan vs bitor mismatch on {heaps:?}");
    }
}
