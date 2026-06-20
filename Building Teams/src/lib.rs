/// Building Teams (CSES 1668) — three solving algorithms.
///
/// Given n pupils and m friendship edges, assign each pupil to team 1 or 2
/// such that no edge connects two pupils on the same team. This is the
/// classic graph 2-coloring (bipartiteness) test.
///
/// Each function returns Some(teams) where `teams[i]` is the 1-or-2
/// assignment of pupil (i+1), or None if the graph is not bipartite.
use std::collections::VecDeque;

/// Build a 1-indexed adjacency list from the edge list.
fn build_adj(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    adj
}

// ---------------------------------------------------------------------------
// Algorithm 1: BFS 2-coloring
// ---------------------------------------------------------------------------
/// For each unvisited node, BFS the component assigning alternating colors.
/// If any edge connects two same-colored nodes, the graph is not bipartite.
///
/// Time: O(n + m).
/// Space: O(n + m).
pub fn solve_bfs(n: usize, edges: &[(usize, usize)]) -> Option<Vec<u8>> {
    let adj = build_adj(n, edges);
    // 0 = uncolored, 1 = team 1, 2 = team 2.
    let mut color = vec![0u8; n + 1];
    let mut queue: VecDeque<usize> = VecDeque::new();

    for start in 1..=n {
        if color[start] != 0 {
            continue;
        }
        color[start] = 1;
        queue.clear();
        queue.push_back(start);
        while let Some(u) = queue.pop_front() {
            let next = if color[u] == 1 { 2 } else { 1 };
            for &v in &adj[u] {
                if color[v] == 0 {
                    color[v] = next;
                    queue.push_back(v);
                } else if color[v] == color[u] {
                    return None;
                }
            }
        }
    }

    Some(color[1..=n].to_vec())
}

// ---------------------------------------------------------------------------
// Algorithm 2: Iterative DFS 2-coloring
// ---------------------------------------------------------------------------
/// Same 2-coloring with an explicit LIFO stack instead of a queue.
///
/// Time: O(n + m).
/// Space: O(n + m).
pub fn solve_dfs(n: usize, edges: &[(usize, usize)]) -> Option<Vec<u8>> {
    let adj = build_adj(n, edges);
    let mut color = vec![0u8; n + 1];
    let mut stack: Vec<usize> = Vec::new();

    for start in 1..=n {
        if color[start] != 0 {
            continue;
        }
        color[start] = 1;
        stack.clear();
        stack.push(start);
        while let Some(u) = stack.pop() {
            let next = if color[u] == 1 { 2 } else { 1 };
            for &v in &adj[u] {
                if color[v] == 0 {
                    color[v] = next;
                    stack.push(v);
                } else if color[v] == color[u] {
                    return None;
                }
            }
        }
    }

    Some(color[1..=n].to_vec())
}

// ---------------------------------------------------------------------------
// Algorithm 3: Union-Find with parity (weighted DSU)
// ---------------------------------------------------------------------------
/// Each node carries a parity bit relative to its root. Unioning two nodes
/// records the parity difference required to keep them on opposite teams.
/// Detect non-bipartiteness when an edge would force two nodes in the same
/// component to share parity.
///
/// Uses path compression that also updates parity as it flattens chains.
///
/// Time: O((n + m) · α(n)).
/// Space: O(n).
pub fn solve_union_find_parity(n: usize, edges: &[(usize, usize)]) -> Option<Vec<u8>> {
    // 1-indexed parent and rank arrays. `parity[v]` is the parity of v
    // relative to its parent — XOR up to the root gives parity vs root.
    let mut parent: Vec<usize> = (0..=n).collect();
    let mut parity: Vec<u8> = vec![0; n + 1];
    let mut rank: Vec<u8> = vec![0; n + 1];

    // Iterative find with path compression that recomputes parity vs root.
    fn find(parent: &mut [usize], parity: &mut [u8], mut x: usize) -> (usize, u8) {
        // First pass: walk to the root and accumulate parity.
        let mut path = Vec::new();
        while parent[x] != x {
            path.push(x);
            x = parent[x];
        }
        let root = x;
        // Second pass: compress, recomputing each node's parity vs root.
        for &v in path.iter().rev() {
            let p = parent[v];
            // p's parity vs root is parity[p] now (after compression of p).
            // v's parity vs root = parity[v] XOR p's parity vs root.
            let new_parity = parity[v] ^ parity[p];
            parent[v] = root;
            parity[v] = new_parity;
        }
        // parity[root] is 0 by convention; report parity of the original x vs root.
        let parity_x = parity[path.first().copied().unwrap_or(root)];
        let _ = parity_x; // not directly used; caller re-finds.
        // Return the root and the parity of x vs root. Since we updated parity[x]
        // (the original input is path[0] if path is non-empty), reread it.
        let parity_of_input = if let Some(&first) = path.first() {
            parity[first]
        } else {
            0 // x is the root
        };
        (root, parity_of_input)
    }

    for &(a, b) in edges {
        let (ra, pa) = find(&mut parent, &mut parity, a);
        let (rb, pb) = find(&mut parent, &mut parity, b);
        if ra == rb {
            // Same component: a and b must have OPPOSITE parities.
            if pa == pb {
                return None;
            }
        } else {
            // Union the two roots so that parity(a) XOR parity(b) = 1.
            // After union, parity of one root vs the other = pa XOR pb XOR 1.
            let connect_parity = pa ^ pb ^ 1;
            if rank[ra] < rank[rb] {
                parent[ra] = rb;
                parity[ra] = connect_parity;
            } else if rank[ra] > rank[rb] {
                parent[rb] = ra;
                parity[rb] = connect_parity;
            } else {
                parent[rb] = ra;
                parity[rb] = connect_parity;
                rank[ra] += 1;
            }
        }
    }

    // Final pass: assign team based on parity vs root of each node.
    let mut teams = vec![0u8; n];
    for v in 1..=n {
        let (_root, pv) = find(&mut parent, &mut parity, v);
        teams[v - 1] = if pv == 0 { 1 } else { 2 };
    }
    Some(teams)
}
