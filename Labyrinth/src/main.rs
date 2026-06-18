// CSES Problem 1193 – Labyrinth
//
// Reads the maze from stdin, finds the shortest path from A to B using BFS,
// and prints the CSES-format output:
//   YES\n<length>\n<path-string>
// or just "NO" if unreachable.

use labyrinth::solve_bfs;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let header = lines.next().unwrap();
    let mut iter = header.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _m: usize = iter.next().unwrap().parse().unwrap();

    let grid: Vec<Vec<u8>> = lines.take(n).map(|line| line.as_bytes().to_vec()).collect();

    match solve_bfs(&grid) {
        None => println!("NO"),
        Some((dist, path)) => {
            println!("YES");
            println!("{}", dist);
            println!("{}", path);
        }
    }
}
