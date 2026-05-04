// CSES Problem 1192 – Counting Rooms
//
// Reads grid from stdin and prints the number of rooms (connected
// components of floor cells) using the BFS algorithm.

use std::io::Read;
use counting_rooms::solve_bfs;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let header = lines.next().unwrap();
    let mut iter = header.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let _m: usize = iter.next().unwrap().parse().unwrap();

    let grid: Vec<Vec<u8>> = lines.take(n).map(|line| line.as_bytes().to_vec()).collect();

    println!("{}", solve_bfs(&grid));
}
