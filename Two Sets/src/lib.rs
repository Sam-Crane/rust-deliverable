/// Two Sets (CSES 1092) — three solving algorithms.
///
/// Divide {1, 2, ..., n} into two sets with equal sums.
/// Returns None if impossible (when n % 4 is 1 or 2), or
/// Some((set1, set2)) with a valid partition.

// ---------------------------------------------------------------------------
// Algorithm 1: Greedy (largest-first)
// ---------------------------------------------------------------------------
/// Greedily assigns numbers from n down to 1 into set1 whenever doing so
/// does not exceed the target sum, otherwise into set2.
///
/// Time: O(n), Space: O(n).
pub fn solve_greedy(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let total = n * (n + 1) / 2;
    if total % 2 != 0 {
        return None;
    }
    let target = total / 2;

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();
    let mut sum1: u64 = 0;

    for i in (1..=n).rev() {
        if sum1 + i <= target {
            sum1 += i;
            set1.push(i);
        } else {
            set2.push(i);
        }
    }

    Some((set1, set2))
}

// ---------------------------------------------------------------------------
// Algorithm 2: Pair-based construction
// ---------------------------------------------------------------------------
/// Exploits the structure of consecutive-number pairs.
///
/// When n % 4 == 0: form pairs (1, n), (2, n-1), ... each summing to n+1.
/// There are n/2 such pairs. Alternate them between the two sets.
///
/// When n % 4 == 3: place n into set1, then pair (1, n-1), (2, n-2), ...
/// each summing to n. There are (n-1)/2 pairs. Alternate them.
///
/// Time: O(n), Space: O(n).
pub fn solve_pairs(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let total = n * (n + 1) / 2;
    if total % 2 != 0 {
        return None;
    }

    let mut set1 = Vec::new();
    let mut set2 = Vec::new();

    if n % 4 == 0 {
        // n/2 pairs of (i, n+1-i), each summing to n+1. Alternate between sets.
        let half = n / 2;
        for i in 1..=half {
            let j = n + 1 - i;
            if i % 2 == 1 {
                set1.push(i);
                set1.push(j);
            } else {
                set2.push(i);
                set2.push(j);
            }
        }
    } else {
        // n % 4 == 3. Put n in set1. Remaining 1..n-1 has (n-1)/2 pairs
        // of (i, n-i), each summing to n. Alternate between sets.
        set1.push(n);
        let half = (n - 1) / 2;
        for i in 1..=half {
            let j = n - i;
            // First pair goes to set2 since set1 already holds n.
            if i % 2 == 1 {
                set2.push(i);
                set2.push(j);
            } else {
                set1.push(i);
                set1.push(j);
            }
        }
    }

    Some((set1, set2))
}

// ---------------------------------------------------------------------------
// Algorithm 3: Recursive subset-sum (with iterative implementation)
// ---------------------------------------------------------------------------
/// Builds set1 by subtracting the largest possible number from the remaining
/// target at each step, working from n down to 1. This mirrors a recursive
/// decomposition of the target sum into distinct values from {1..n}.
///
/// Time: O(n), Space: O(n).
pub fn solve_recursive(n: u64) -> Option<(Vec<u64>, Vec<u64>)> {
    let total = n * (n + 1) / 2;
    if total % 2 != 0 {
        return None;
    }
    let target = total / 2;

    let mut set1 = Vec::new();
    let mut in_set1 = vec![false; (n + 1) as usize];
    let mut remaining = target;

    // Decompose target into distinct numbers from {1..n}, largest first.
    for i in (1..=n).rev() {
        if i <= remaining {
            remaining -= i;
            set1.push(i);
            in_set1[i as usize] = true;
        }
        if remaining == 0 {
            break;
        }
    }

    let set2: Vec<u64> = (1..=n).filter(|&i| !in_set1[i as usize]).collect();

    Some((set1, set2))
}
