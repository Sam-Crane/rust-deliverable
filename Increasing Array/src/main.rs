// CSES Problem 1094 – Increasing Array
//
// Reads n and the array from stdin, solves using the greedy algorithm,
// and prints the minimum number of moves.

use increasing_array::solve_greedy;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let nums: Vec<i64> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    println!("{}", solve_greedy(&nums));
}
