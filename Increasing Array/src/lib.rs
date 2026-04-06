/// Increasing Array — three solving algorithms.
///
/// All functions return the minimum number of moves (increments) needed
/// to make the input array non-decreasing.

// ---------------------------------------------------------------------------
// Algorithm 1: Greedy Scan (imperative)
// ---------------------------------------------------------------------------
/// Single left-to-right pass tracking the running maximum.
/// Whenever an element falls below the running max, the difference is added
/// to the move count.
///
/// Time: O(n), Space: O(1) auxiliary.
pub fn solve_greedy(nums: &[i64]) -> i64 {
    let mut moves: i64 = 0;
    let mut prev = nums[0];
    for &x in &nums[1..] {
        if x < prev {
            moves += prev - x;
        } else {
            prev = x;
        }
    }
    moves
}

// ---------------------------------------------------------------------------
// Algorithm 2: Functional Fold
// ---------------------------------------------------------------------------
/// Same logic as the greedy scan, expressed as an iterator fold over
/// (running_max, total_moves) accumulator.
///
/// Time: O(n), Space: O(1) auxiliary.
pub fn solve_fold(nums: &[i64]) -> i64 {
    nums[1..]
        .iter()
        .fold((nums[0], 0i64), |(prev, moves), &x| {
            if x < prev {
                (prev, moves + prev - x)
            } else {
                (x, moves)
            }
        })
        .1
}

// ---------------------------------------------------------------------------
// Algorithm 3: Prefix Maximum
// ---------------------------------------------------------------------------
/// Builds a prefix-maximum array, then sums the difference between each
/// prefix max and the original element.
///
/// Time: O(n), Space: O(n) auxiliary (for the prefix-max array).
pub fn solve_prefix_max(nums: &[i64]) -> i64 {
    let n = nums.len();
    let mut prefix_max = Vec::with_capacity(n);
    prefix_max.push(nums[0]);
    for i in 1..n {
        prefix_max.push(prefix_max[i - 1].max(nums[i]));
    }

    let mut moves: i64 = 0;
    for i in 0..n {
        moves += prefix_max[i] - nums[i];
    }
    moves
}
