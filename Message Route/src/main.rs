// CSES Problem 1667 – Message Route
//
// Reads n, m and m edges from stdin, finds shortest path from 1 to n
// using BFS, and prints in CSES format.

use message_route::solve_bfs;
use std::io::{BufWriter, Read, Write};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut edges: Vec<(usize, usize)> = Vec::with_capacity(m);
    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        edges.push((a, b));
    }

    let stdout = std::io::stdout();
    let mut out = BufWriter::new(stdout.lock());

    match solve_bfs(n, &edges) {
        None => writeln!(out, "IMPOSSIBLE").unwrap(),
        Some(path) => {
            writeln!(out, "{}", path.len()).unwrap();
            let parts: Vec<String> = path.iter().map(|x| x.to_string()).collect();
            writeln!(out, "{}", parts.join(" ")).unwrap();
        }
    }
}
