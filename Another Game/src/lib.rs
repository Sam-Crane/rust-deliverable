/// Another Game (CSES 2208) — three solving algorithms.
///
/// Two players take turns. On each turn the player picks any non-empty subset
/// of heaps and removes exactly one coin from each chosen heap. The player
/// who removes the last coin wins.
///
/// Solution: the first player wins iff at least one heap has an odd count.
/// Proof sketch (mirror strategy): if every heap is even, the second player
/// can always copy the first player's move on the same set of heaps, keeping
/// the all-even invariant. Eventually the first player faces all zeros (loss).
/// If any heap is odd, the first player removes one from the odd heap(s) so
/// that every heap becomes even, handing the all-even position to the second.
///
/// Each function returns `true` if the first player wins, `false` otherwise.

// ---------------------------------------------------------------------------
// Algorithm 1: Imperative scan with early exit
// ---------------------------------------------------------------------------
/// Linear scan that returns as soon as an odd heap is found. Best case O(1)
/// when an odd heap appears near the start; worst case O(n) for all-even.
pub fn solve_scan(heaps: &[u64]) -> bool {
    for &h in heaps {
        if h & 1 == 1 {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Algorithm 2: Functional iterator
// ---------------------------------------------------------------------------
/// Idiomatic Rust using `Iterator::any`, which short-circuits internally.
/// Equivalent semantics to Algorithm 1 but in functional style.
pub fn solve_iter(heaps: &[u64]) -> bool {
    heaps.iter().any(|&h| h & 1 == 1)
}

// ---------------------------------------------------------------------------
// Algorithm 3: Bitwise OR reduce
// ---------------------------------------------------------------------------
/// Folds all heaps with the bitwise OR operator, then inspects the low bit.
/// Visits every element (no early exit), which can vectorize well on dense
/// inputs but is wasteful when an odd heap appears early.
pub fn solve_bitor(heaps: &[u64]) -> bool {
    let combined = heaps.iter().fold(0u64, |acc, &h| acc | h);
    combined & 1 == 1
}
