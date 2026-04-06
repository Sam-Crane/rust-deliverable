// CSES Problem 2165 – Tower of Hanoi
//
// Reads n from stdin, solves using the recursive algorithm, and prints
// the move count followed by each move.

use std::io::Read;
use tower_of_hanoi::solve_recursive;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: u32 = input.trim().parse().unwrap();

    let moves = solve_recursive(n);

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
