// CSES Problem 2208 – Another Game
//
// Reads t test cases from stdin. Each test case has n followed by n heap
// sizes. For each, prints "first" if the first player wins, else "second".

use another_game::solve_scan;
use std::io::{BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    let t: usize = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let heaps: Vec<u64> = (0..n)
            .map(|_| iter.next().unwrap().parse().unwrap())
            .collect();
        let winner = if solve_scan(&heaps) {
            "first"
        } else {
            "second"
        };
        writeln!(out, "{}", winner).unwrap();
    }
}
