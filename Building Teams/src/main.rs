// CSES Problem 1668 – Building Teams
//
// Reads n, m and m edges; tries to 2-color the friendship graph using BFS.
// Prints the team assignments (1 or 2) for pupils 1..=n on one line,
// or "IMPOSSIBLE" if the graph is not bipartite.

use building_teams::solve_bfs;
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
        Some(teams) => {
            let parts: Vec<String> = teams.iter().map(|x| x.to_string()).collect();
            writeln!(out, "{}", parts.join(" ")).unwrap();
        }
    }
}
