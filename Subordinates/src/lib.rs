/// Subordinates (CSES 1674) — three solving algorithms.
///
/// Each function takes `bosses`: a slice of length n-1 where bosses[i] is the
/// 1-indexed boss of employee (i+2). Employee 1 is the root.
/// Returns a Vec of length n where result[i] is the subordinate count for
/// employee (i+1).

// ---------------------------------------------------------------------------
// Helper: build children adjacency list from boss list.
// ---------------------------------------------------------------------------
fn build_children(bosses: &[usize], n: usize) -> Vec<Vec<usize>> {
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n + 1]; // 1-indexed
    for (i, &boss) in bosses.iter().enumerate() {
        let employee = i + 2; // employees 2..=n
        children[boss].push(employee);
    }
    children
}

// ---------------------------------------------------------------------------
// Algorithm 1: Recursive DFS
// ---------------------------------------------------------------------------
/// Recursively counts descendants. Simple and clear, but may overflow the
/// stack on degenerate (linear chain) inputs at maximum constraints.
///
/// Time: O(n), Space: O(n) recursion depth in the worst case.
pub fn solve_recursive(bosses: &[usize]) -> Vec<usize> {
    let n = bosses.len() + 1;
    let children = build_children(bosses, n);
    let mut counts = vec![0usize; n + 1];

    fn dfs(node: usize, children: &[Vec<usize>], counts: &mut [usize]) -> usize {
        let mut total = 0;
        for &child in &children[node] {
            let sub = dfs(child, children, counts);
            total += sub + 1;
        }
        counts[node] = total;
        total
    }

    dfs(1, &children, &mut counts);
    counts[1..=n].to_vec()
}

// ---------------------------------------------------------------------------
// Algorithm 2: Iterative DFS (explicit stack, post-order)
// ---------------------------------------------------------------------------
/// Iterative post-order DFS using an explicit stack. Each node is pushed
/// twice: first as "enter" (push children), then as "exit" (sum subtree).
/// Safe for the maximum constraint without stack overflow.
///
/// Time: O(n), Space: O(n).
pub fn solve_iterative(bosses: &[usize]) -> Vec<usize> {
    let n = bosses.len() + 1;
    let children = build_children(bosses, n);
    let mut counts = vec![0usize; n + 1];

    // Stack entries: (node, visited_flag)
    let mut stack: Vec<(usize, bool)> = Vec::with_capacity(n);
    stack.push((1, false));

    while let Some((node, visited)) = stack.pop() {
        if visited {
            // Exit: sum children's counts + 1 each.
            let mut total = 0;
            for &child in &children[node] {
                total += counts[child] + 1;
            }
            counts[node] = total;
        } else {
            // Enter: schedule exit, then push children.
            stack.push((node, true));
            for &child in &children[node] {
                stack.push((child, false));
            }
        }
    }

    counts[1..=n].to_vec()
}

// ---------------------------------------------------------------------------
// Algorithm 3: Reverse-BFS (level processing)
// ---------------------------------------------------------------------------
/// Computes a BFS order from the root, then iterates in reverse, adding
/// each node's (count + 1) to its parent's count. No recursion, no
/// post-order stack — just a queue and a parent-pointer array.
///
/// Time: O(n), Space: O(n).
pub fn solve_reverse_bfs(bosses: &[usize]) -> Vec<usize> {
    let n = bosses.len() + 1;
    let children = build_children(bosses, n);
    let mut counts = vec![0usize; n + 1];

    // BFS to establish a top-down order.
    let mut order: Vec<usize> = Vec::with_capacity(n);
    let mut head = 0;
    order.push(1);
    while head < order.len() {
        let node = order[head];
        head += 1;
        for &child in &children[node] {
            order.push(child);
        }
    }

    // Build parent-pointer array (1-indexed).
    let mut parent = vec![0usize; n + 1];
    for (i, &boss) in bosses.iter().enumerate() {
        parent[i + 2] = boss;
    }

    // Process in reverse BFS order: each node's count + 1 contributes to parent.
    for &node in order.iter().rev() {
        if node != 1 {
            let p = parent[node];
            counts[p] += counts[node] + 1;
        }
    }

    counts[1..=n].to_vec()
}
