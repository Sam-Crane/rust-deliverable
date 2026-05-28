/// Finding Borders (CSES 1732) — three solving algorithms.
///
/// A border of a string s is a length k (1 ≤ k < n) such that
/// s[0..k] == s[n-k..n] (the prefix of length k equals the suffix
/// of length k, but not the whole string).
///
/// Each function returns the list of border lengths in ascending order.

// ---------------------------------------------------------------------------
// Algorithm 1: KMP failure function (prefix function)
// ---------------------------------------------------------------------------
/// Computes the standard KMP prefix function pi[], where pi[i] is the
/// length of the longest proper prefix of s[..=i] that is also a suffix.
/// The set of borders of s is then {pi[n-1], pi[pi[n-1]-1], ...} until 0.
///
/// Time: O(n), Space: O(n).
pub fn solve_kmp(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    if n <= 1 {
        return Vec::new();
    }

    let mut pi = vec![0usize; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }

    // Follow the failure-chain from pi[n-1] down to gather all border lengths.
    let mut borders = Vec::new();
    let mut k = pi[n - 1];
    while k > 0 {
        borders.push(k);
        k = pi[k - 1];
    }
    borders.reverse(); // ascending order
    borders
}

// ---------------------------------------------------------------------------
// Algorithm 2: Z-algorithm
// ---------------------------------------------------------------------------
/// Computes the Z array, where z[i] is the length of the longest substring
/// starting at i that matches a prefix of s. A border of length k exists
/// iff z[n-k] >= k AND n-k+z[n-k] == n, i.e. z[n-k] == k (the match
/// reaches the end of the string).
///
/// Time: O(n), Space: O(n).
pub fn solve_z(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    if n <= 1 {
        return Vec::new();
    }

    let mut z = vec![0usize; n];
    z[0] = n;
    let (mut l, mut r) = (0usize, 0usize);
    for i in 1..n {
        if i < r {
            z[i] = z[i - l].min(r - i);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] > r {
            l = i;
            r = i + z[i];
        }
    }

    // Border of length k corresponds to z[n-k] == k.
    let mut borders = Vec::new();
    for k in 1..n {
        if z[n - k] == k {
            borders.push(k);
        }
    }
    borders
}

// ---------------------------------------------------------------------------
// Algorithm 3: Polynomial double hashing
// ---------------------------------------------------------------------------
/// Computes prefix hashes with two moduli and bases. For each candidate
/// border length k, compares the hash of s[0..k] to the hash of s[n-k..n].
/// Using two independent hashes makes false-positive collisions astronomically
/// unlikely on the constraint range.
///
/// Time: O(n), Space: O(n).
pub fn solve_hashing(s: &[u8]) -> Vec<usize> {
    let n = s.len();
    if n <= 1 {
        return Vec::new();
    }

    const MOD1: u64 = 1_000_000_007;
    const MOD2: u64 = 998_244_353;
    const BASE1: u64 = 131;
    const BASE2: u64 = 137;

    // Prefix hashes: h[i] = hash of s[0..i]. h[0] = 0.
    let mut h1 = vec![0u64; n + 1];
    let mut h2 = vec![0u64; n + 1];
    let mut p1 = vec![1u64; n + 1];
    let mut p2 = vec![1u64; n + 1];

    for i in 0..n {
        let c = s[i] as u64;
        h1[i + 1] = (h1[i] * BASE1 + c) % MOD1;
        h2[i + 1] = (h2[i] * BASE2 + c) % MOD2;
        p1[i + 1] = (p1[i] * BASE1) % MOD1;
        p2[i + 1] = (p2[i] * BASE2) % MOD2;
    }

    // Hash of substring s[l..r] (half-open).
    let hash1 = |l: usize, r: usize| -> u64 { (h1[r] + MOD1 - h1[l] * p1[r - l] % MOD1) % MOD1 };
    let hash2 = |l: usize, r: usize| -> u64 { (h2[r] + MOD2 - h2[l] * p2[r - l] % MOD2) % MOD2 };

    let mut borders = Vec::new();
    for k in 1..n {
        let prefix_h1 = hash1(0, k);
        let prefix_h2 = hash2(0, k);
        let suffix_h1 = hash1(n - k, n);
        let suffix_h2 = hash2(n - k, n);
        if prefix_h1 == suffix_h1 && prefix_h2 == suffix_h2 {
            borders.push(k);
        }
    }
    borders
}
