/// Tower of Hanoi — three solving algorithms.
///
/// All functions return a Vec of (source, destination) pairs representing
/// the sequence of moves to transfer n disks from peg 1 to peg 3.

// ---------------------------------------------------------------------------
// Algorithm 1: Classic Recursion
// ---------------------------------------------------------------------------
/// Solve Tower of Hanoi recursively.
///
/// Moves n disks from `from` to `to` using `aux` as intermediate storage.
/// Time complexity: O(2^n), which is optimal for this problem.
pub fn hanoi_recursive(n: u32, from: u8, to: u8, aux: u8, moves: &mut Vec<(u8, u8)>) {
    if n == 0 {
        return;
    }
    hanoi_recursive(n - 1, from, aux, to, moves);
    moves.push((from, to));
    hanoi_recursive(n - 1, aux, to, from, moves);
}

/// Convenience wrapper that solves from peg 1 to peg 3.
pub fn solve_recursive(n: u32) -> Vec<(u8, u8)> {
    let mut moves = Vec::new();
    hanoi_recursive(n, 1, 3, 2, &mut moves);
    moves
}

// ---------------------------------------------------------------------------
// Algorithm 2: Iterative Bit-Manipulation
// ---------------------------------------------------------------------------
/// Solve Tower of Hanoi using binary representation of the move number.
///
/// For move m (1-based), the disk to move is determined by the position of
/// the lowest set bit: disk = trailing_zeros(m) + 1. Each disk always cycles
/// through pegs in a fixed direction that depends on whether n is even or odd.
pub fn solve_iterative_bits(n: u32) -> Vec<(u8, u8)> {
    let total_moves = (1u64 << n) - 1;
    let mut moves = Vec::with_capacity(total_moves as usize);

    // Track which peg each disk is on. Index 0 = disk 1 (smallest).
    let mut disk_peg = vec![0u8; n as usize]; // all start on peg 0

    // Disk movement direction: smallest disk cycles 0→1→2 if n is odd,
    // 0→2→1 if n is even. Larger disks alternate direction each level.
    // Direction +1 means 0→1→2→0, direction -1 (i.e. +2 mod 3) means 0→2→1→0.
    let directions: Vec<u8> = (0..n)
        .map(|d| {
            // disk d (0-indexed): if (n - d) is odd, direction is +1; else +2
            if (n - d) % 2 == 1 { 2 } else { 1 }
        })
        .collect();

    for m in 1..=total_moves {
        let disk = m.trailing_zeros() as usize; // 0-indexed disk number
        let from_peg = disk_peg[disk];
        let to_peg = (from_peg + directions[disk]) % 3;
        disk_peg[disk] = to_peg;
        moves.push((from_peg + 1, to_peg + 1)); // convert to 1-indexed
    }

    moves
}

// ---------------------------------------------------------------------------
// Algorithm 3: Iterative State-Based (Frame-Stewart style simulation)
// ---------------------------------------------------------------------------
/// Solve Tower of Hanoi iteratively by simulating three stacks.
///
/// Uses the rule: on odd steps move the smallest disk one peg forward
/// (cyclically), and on even steps make the only legal move that does
/// not involve the smallest disk.
pub fn solve_iterative_stacks(n: u32) -> Vec<(u8, u8)> {
    if n == 0 {
        return vec![];
    }

    let total_moves = (1u64 << n) - 1;
    let mut moves = Vec::with_capacity(total_moves as usize);

    // Stacks: index 0 = peg 1, index 1 = peg 2, index 2 = peg 3.
    // Each element is a disk size (larger number = larger disk).
    let mut stacks: [Vec<u32>; 3] = [vec![], vec![], vec![]];
    for d in (1..=n).rev() {
        stacks[0].push(d);
    }

    // Smallest disk direction: +1 if n is odd, +2 (i.e. -1 mod 3) if n is even.
    let small_dir: usize = if n % 2 == 1 { 2 } else { 1 };
    let mut small_peg: usize = 0; // current peg of the smallest disk

    for step in 1..=total_moves {
        if step % 2 == 1 {
            // Odd step: move smallest disk in its fixed direction.
            let from = small_peg;
            let to = (small_peg + small_dir) % 3;
            let disk = stacks[from].pop().unwrap();
            stacks[to].push(disk);
            moves.push((from as u8 + 1, to as u8 + 1));
            small_peg = to;
        } else {
            // Even step: make the only legal move not involving the smallest disk.
            // The two candidate pegs are those not holding the smallest disk.
            let a = (small_peg + 1) % 3;
            let b = (small_peg + 2) % 3;
            let top_a = stacks[a].last().copied().unwrap_or(u32::MAX);
            let top_b = stacks[b].last().copied().unwrap_or(u32::MAX);

            let (from, to) = if top_a < top_b { (a, b) } else { (b, a) };
            let disk = stacks[from].pop().unwrap();
            stacks[to].push(disk);
            moves.push((from as u8 + 1, to as u8 + 1));
        }
    }

    moves
}
