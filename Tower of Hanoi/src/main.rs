// CSES Problem 2165 – Tower of Hanoi
//
// Three stacks (1 = left, 2 = middle, 3 = right) and n disks of distinct sizes.
// All disks start on stack 1, sorted largest-on-bottom. The goal is to move
// every disk to stack 3, moving one disk at a time, never placing a larger
// disk on a smaller one, using the minimum number of moves (2^n - 1).
//
// The classic recursive strategy is used:
//   1. Move the top n-1 disks from `from` to `aux` (using `to` as helper).
//   2. Move the largest remaining disk from `from` to `to`.
//   3. Move the n-1 disks from `aux` to `to` (using `from` as helper).

use std::io::Read;

/// Recursively solves the Tower of Hanoi problem and appends each move
/// as a (source, destination) pair to the `moves` vector.
///
/// # Arguments
/// * `n`     - Number of disks to move.
/// * `from`  - Stack to move disks from (1, 2, or 3).
/// * `to`    - Stack to move disks to (1, 2, or 3).
/// * `aux`   - Auxiliary stack used as intermediate storage.
/// * `moves` - Accumulator for the sequence of moves.
fn hanoi(n: u32, from: u8, to: u8, aux: u8, moves: &mut Vec<(u8, u8)>) {
    if n == 0 {
        return;
    }
    // Step 1: Move the top n-1 disks out of the way onto the auxiliary stack.
    hanoi(n - 1, from, aux, to, moves);
    // Step 2: Move the nth (largest remaining) disk directly to the target.
    moves.push((from, to));
    // Step 3: Move the n-1 disks from the auxiliary stack to the target.
    hanoi(n - 1, aux, to, from, moves);
}

fn main() {
    // Read the number of disks from stdin.
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    // Solve: move all n disks from stack 1 to stack 3 using stack 2.
    let mut moves = Vec::new();
    hanoi(n, 1, 3, 2, &mut moves);

    // Output the total move count followed by each move (source dest).
    let mut out = String::new();
    out.push_str(&moves.len().to_string());
    out.push('\n');
    for (a, b) in &moves {
        out.push_str(&a.to_string());
        out.push(' ');
        out.push_str(&b.to_string());
        out.push('\n');
    }
    print!("{}", out);
}
