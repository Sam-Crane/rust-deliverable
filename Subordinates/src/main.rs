// CSES Problem 1674 – Subordinates
//
// Reads n and the boss list from stdin, computes subordinate counts using
// the iterative DFS algorithm (safe for maximum constraint), and prints
// the n counts separated by spaces.

use std::io::{Read, Write, BufWriter};
use subordinates::solve_iterative;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let bosses: Vec<usize> = (0..n.saturating_sub(1))
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let counts = solve_iterative(&bosses);

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());
    let parts: Vec<String> = counts.iter().map(|x| x.to_string()).collect();
    writeln!(out, "{}", parts.join(" ")).unwrap();
}
