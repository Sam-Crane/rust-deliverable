// CSES Problem 1094 – Increasing Array
//
// Given an array of n integers, make it non-decreasing by only increasing
// elements. Each +1 to any element costs one move. Find the minimum total
// number of moves.
//
// Strategy: scan left to right. Whenever an element is smaller than its
// predecessor, it must be raised to match. The cost is the difference.
// This greedy approach is optimal because raising to exactly the previous
// value is the cheapest way to satisfy the non-decreasing constraint.

use std::io::Read;

fn main() {
    // Read all input at once.
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // Parse n (array size) and the n array elements.
    let n: usize = iter.next().unwrap().parse().unwrap();
    let nums: Vec<i64> = iter.take(n).map(|x| x.parse().unwrap()).collect();

    // Track the current required minimum (the running maximum so far).
    // Whenever an element is below this minimum, add the difference.
    let mut moves: i64 = 0;
    let mut prev: i64 = nums[0];
    for i in 1..n {
        if nums[i] < prev {
            moves += prev - nums[i];
        } else {
            prev = nums[i];
        }
    }

    println!("{}", moves);
}
