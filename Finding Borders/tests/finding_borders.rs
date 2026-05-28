// Integration tests for CSES 1732 – Finding Borders
//
// Tests the binary (KMP via stdin/stdout) as well as all three algorithm
// implementations exposed by the library.

use finding_borders::{solve_hashing, solve_kmp, solve_z};
use std::process::Command;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_finding_borders");
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

fn assert_all_algorithms(s: &str, expected: &[usize]) {
    let bytes = s.as_bytes();
    assert_eq!(solve_kmp(bytes), expected, "KMP failed on {s:?}");
    assert_eq!(solve_z(bytes), expected, "Z-algorithm failed on {s:?}");
    assert_eq!(solve_hashing(bytes), expected, "Hashing failed on {s:?}");
}

/// Naive border check for cross-validation: O(n^2). Used to verify algorithms
/// agree on randomly generated strings.
fn naive_borders(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    let mut result = Vec::new();
    for k in 1..n {
        if s[0..k] == s[n - k..n] {
            result.push(k);
        }
    }
    result
}

fn random_string(n: usize, alphabet_size: u8, seed: u64) -> Vec<u8> {
    let mut rng = seed;
    (0..n)
        .map(|_| {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            b'a' + ((rng >> 33) as u8 % alphabet_size)
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Binary tests (KMP via stdin/stdout)
// ---------------------------------------------------------------------------

#[test]
fn test_binary_example() {
    let output = run_with_input("abcababcab\n");
    assert_eq!(output, "2 5");
}

#[test]
fn test_binary_single_char() {
    // Single char has no proper border.
    let output = run_with_input("a\n");
    assert_eq!(output, "");
}

#[test]
fn test_binary_no_borders() {
    // "abcdef" — no prefix == suffix.
    let output = run_with_input("abcdef\n");
    assert_eq!(output, "");
}

#[test]
fn test_binary_all_same() {
    // "aaaa" — borders of length 1, 2, 3.
    let output = run_with_input("aaaa\n");
    assert_eq!(output, "1 2 3");
}

#[test]
fn test_binary_palindrome_like() {
    // "abab" — border length 2 ("ab").
    let output = run_with_input("abab\n");
    assert_eq!(output, "2");
}

// ---------------------------------------------------------------------------
// Library tests — all three algorithms
// ---------------------------------------------------------------------------

#[test]
fn test_all_example() {
    assert_all_algorithms("abcababcab", &[2, 5]);
}

#[test]
fn test_all_single_char() {
    assert_all_algorithms("a", &[]);
}

#[test]
fn test_all_no_borders() {
    assert_all_algorithms("abcdef", &[]);
}

#[test]
fn test_all_all_same() {
    assert_all_algorithms("aaaa", &[1, 2, 3]);
}

#[test]
fn test_all_two_chars() {
    assert_all_algorithms("aa", &[1]);
    assert_all_algorithms("ab", &[]);
}

#[test]
fn test_all_palindrome_like() {
    assert_all_algorithms("abab", &[2]);
    assert_all_algorithms("ababab", &[2, 4]);
}

#[test]
fn test_all_complex() {
    // "abacabadabacaba" — known borders at 1, 3, 7.
    assert_all_algorithms("abacabadabacaba", &[1, 3, 7]);
}

#[test]
fn test_all_random_strings_agree() {
    // Cross-validate all three algorithms against the naive O(n^2) version
    // on random strings with small alphabets (to exercise lots of borders).
    for seed in 0..20 {
        for &alpha in &[2u8, 3, 5, 26] {
            let s = random_string(30, alpha, seed * 31 + alpha as u64);
            let expected = naive_borders(&s);
            assert_eq!(
                solve_kmp(&s),
                expected,
                "KMP failed seed={seed} alpha={alpha}"
            );
            assert_eq!(solve_z(&s), expected, "Z failed seed={seed} alpha={alpha}");
            assert_eq!(
                solve_hashing(&s),
                expected,
                "Hashing failed seed={seed} alpha={alpha}"
            );
        }
    }
}

#[test]
fn test_all_random_large_agree() {
    // Larger string, binary alphabet → many borders.
    let s = random_string(2000, 2, 12345);
    let expected = naive_borders(&s);
    assert_eq!(solve_kmp(&s), expected);
    assert_eq!(solve_z(&s), expected);
    assert_eq!(solve_hashing(&s), expected);
}
