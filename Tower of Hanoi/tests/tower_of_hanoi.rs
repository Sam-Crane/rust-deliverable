// Integration tests for CSES 2165 – Tower of Hanoi
//
// Each test feeds n to the compiled binary via stdin, then verifies:
//   1. The declared move count k equals 2^n - 1 (the proven minimum).
//   2. The number of printed moves matches k.
//   3. Every move is legal (source stack is non-empty, no larger-on-smaller).
//   4. After all moves, every disk resides on stack 3.

use std::process::Command;

/// Spawn the binary, pipe `input` to its stdin, and return its stdout as a String.
fn run_with_input(input: &str) -> String {
    let bin = env!("CARGO_BIN_EXE_tower_of_hanoi");
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
    String::from_utf8(output.stdout).unwrap()
}

/// Simulate the Tower of Hanoi game and verify every move is legal.
///
/// Checks:
/// - Source stack is never empty when a move is attempted.
/// - A larger disk is never placed on top of a smaller disk.
/// - All disks end up on stack 3 (index 2) at the end.
fn validate_solution(n: u32, moves: &[(u8, u8)]) {
    // Three stacks represented as vectors; larger numbers = larger disks.
    let mut stacks: Vec<Vec<u32>> = vec![vec![], vec![], vec![]];

    // Initialize: stack 0 (peg 1) has disks n, n-1, ..., 1 (bottom to top).
    for d in (1..=n).rev() {
        stacks[0].push(d);
    }

    // Replay every move and assert legality.
    for (i, &(from, to)) in moves.iter().enumerate() {
        let f = (from - 1) as usize;
        let t = (to - 1) as usize;
        assert!(
            !stacks[f].is_empty(),
            "Move {}: stack {} is empty",
            i + 1,
            from
        );
        let disk = stacks[f].pop().unwrap();
        if let Some(&top) = stacks[t].last() {
            assert!(
                disk < top,
                "Move {}: disk {} placed on smaller disk {} at stack {}",
                i + 1,
                disk,
                top,
                to
            );
        }
        stacks[t].push(disk);
    }

    // After all moves: stacks 1 and 2 must be empty, stack 3 holds all disks.
    assert!(stacks[0].is_empty(), "Stack 1 should be empty");
    assert!(stacks[1].is_empty(), "Stack 2 should be empty");
    assert_eq!(stacks[2].len(), n as usize, "Stack 3 should have all disks");
}

/// Parse the program's output into the move count and a list of (from, to) pairs.
fn parse_output(output: &str) -> (usize, Vec<(u8, u8)>) {
    let lines: Vec<&str> = output.trim().lines().collect();
    let k: usize = lines[0].trim().parse().unwrap();
    let moves: Vec<(u8, u8)> = lines[1..]
        .iter()
        .map(|line| {
            let parts: Vec<u8> = line.trim().split_whitespace().map(|x| x.parse().unwrap()).collect();
            (parts[0], parts[1])
        })
        .collect();
    assert_eq!(moves.len(), k, "Declared move count doesn't match actual moves");
    (k, moves)
}

// ---------------------------------------------------------------------------
// Test cases – smallest input through the maximum constraint (n = 16).
// ---------------------------------------------------------------------------

#[test]
fn test_n1() {
    let output = run_with_input("1\n");
    let (k, moves) = parse_output(&output);
    assert_eq!(k, 1); // 2^1 - 1
    validate_solution(1, &moves);
}

#[test]
fn test_n2() {
    let output = run_with_input("2\n");
    let (k, moves) = parse_output(&output);
    assert_eq!(k, 3); // 2^2 - 1
    validate_solution(2, &moves);
    // Verify the exact expected move sequence from the problem statement.
    assert_eq!(moves, vec![(1, 2), (1, 3), (2, 3)]);
}

#[test]
fn test_n3() {
    let output = run_with_input("3\n");
    let (k, moves) = parse_output(&output);
    assert_eq!(k, 7); // 2^3 - 1
    validate_solution(3, &moves);
}

#[test]
fn test_n4() {
    let output = run_with_input("4\n");
    let (k, moves) = parse_output(&output);
    assert_eq!(k, 15); // 2^4 - 1
    validate_solution(4, &moves);
}

#[test]
fn test_n10() {
    let output = run_with_input("10\n");
    let (k, moves) = parse_output(&output);
    assert_eq!(k, 1023); // 2^10 - 1
    validate_solution(10, &moves);
}

/// Maximum constraint: n = 16 produces 65 535 moves.
#[test]
fn test_n16() {
    let output = run_with_input("16\n");
    let (k, moves) = parse_output(&output);
    assert_eq!(k, 65535); // 2^16 - 1
    validate_solution(16, &moves);
}
