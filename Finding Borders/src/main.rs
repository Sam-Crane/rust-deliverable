// CSES Problem 1732 – Finding Borders
//
// Reads a single string from stdin and prints all border lengths
// (prefix == suffix, excluding the whole string) in ascending order,
// space-separated. Uses the KMP algorithm.

use std::io::Read;
use finding_borders::solve_kmp;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim().as_bytes();

    let borders = solve_kmp(s);
    let parts: Vec<String> = borders.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
