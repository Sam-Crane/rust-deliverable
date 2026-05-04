// CSES Problem 1092 – Two Sets
//
// Reads n from stdin, divides {1..n} into two equal-sum sets using the
// greedy algorithm, and prints the result in CSES format.

use std::io::Read;
use two_sets::solve_greedy;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let n: u64 = input.trim().parse().unwrap();

    match solve_greedy(n) {
        None => println!("NO"),
        Some((set1, set2)) => {
            println!("YES");
            println!("{}", set1.len());
            let s1: Vec<String> = set1.iter().map(|x| x.to_string()).collect();
            println!("{}", s1.join(" "));
            println!("{}", set2.len());
            let s2: Vec<String> = set2.iter().map(|x| x.to_string()).collect();
            println!("{}", s2.join(" "));
        }
    }
}
