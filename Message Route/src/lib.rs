/// Message Route (CSES 1667) — three solving algorithms.
///
/// Finds the shortest path in an unweighted undirected graph from node 1
/// to node n. Nodes are 1-indexed.
/// Each function returns Some(path) where `path` lists the 1-indexed
/// node sequence from 1 to n inclusive, or None if unreachable.
use std::cmp::Reverse;
use std::collections::{BinaryHeap, VecDeque};

/// Build a 1-indexed adjacency list from the edge list.
fn build_adj(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    for &(a, b) in edges {
        adj[a].push(b);
        adj[b].push(a);
    }
    adj
}

/// Reconstruct a path from node 1 to node `end` given a `parent` array
/// where `parent[v]` is the predecessor of `v` (0 if v is the source).
fn reconstruct(parent: &[usize], end: usize) -> Vec<usize> {
    let mut path = Vec::new();
    let mut cur = end;
    while cur != 0 {
        path.push(cur);
        cur = parent[cur];
    }
    path.reverse();
    path
}

// ---------------------------------------------------------------------------
// Algorithm 1: Standard BFS
// ---------------------------------------------------------------------------
/// Single-source BFS from node 1, tracking parent pointers and reconstructing
/// the path backward from n once it's reached.
///
/// Time: O(n + m), Space: O(n + m).
pub fn solve_bfs(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if n == 0 {
        return None;
    }
    let adj = build_adj(n, edges);
    let mut visited = vec![false; n + 1];
    let mut parent = vec![0usize; n + 1];

    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(1);
    visited[1] = true;

    while let Some(u) = queue.pop_front() {
        if u == n {
            return Some(reconstruct(&parent, n));
        }
        for &v in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                parent[v] = u;
                queue.push_back(v);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Algorithm 2: Bidirectional BFS
// ---------------------------------------------------------------------------
/// BFS from both 1 and n alternately, expanding the smaller frontier each
/// round. When a vertex newly visited by one side is also visited by the
/// other, stitch the path through the meeting vertex.
///
/// Time: O(n + m) worst case, often less; Space: O(n + m).
pub fn solve_bidirectional(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(vec![1]);
    }
    let adj = build_adj(n, edges);

    let mut fwd_parent = vec![0usize; n + 1];
    let mut bwd_parent = vec![0usize; n + 1];
    let mut fwd_visited = vec![false; n + 1];
    let mut bwd_visited = vec![false; n + 1];
    let mut fwd_q: VecDeque<usize> = VecDeque::new();
    let mut bwd_q: VecDeque<usize> = VecDeque::new();

    fwd_visited[1] = true;
    bwd_visited[n] = true;
    fwd_q.push_back(1);
    bwd_q.push_back(n);

    fn expand_layer(
        adj: &[Vec<usize>],
        q: &mut VecDeque<usize>,
        my_visited: &mut [bool],
        my_parent: &mut [usize],
        other_visited: &[bool],
    ) -> Option<usize> {
        let len = q.len();
        for _ in 0..len {
            let u = q.pop_front().unwrap();
            for &v in &adj[u] {
                if !my_visited[v] {
                    my_visited[v] = true;
                    my_parent[v] = u;
                    if other_visited[v] {
                        return Some(v);
                    }
                    q.push_back(v);
                }
            }
        }
        None
    }

    while !fwd_q.is_empty() && !bwd_q.is_empty() {
        let meet = if fwd_q.len() <= bwd_q.len() {
            expand_layer(
                &adj,
                &mut fwd_q,
                &mut fwd_visited,
                &mut fwd_parent,
                &bwd_visited,
            )
        } else {
            expand_layer(
                &adj,
                &mut bwd_q,
                &mut bwd_visited,
                &mut bwd_parent,
                &fwd_visited,
            )
        };

        if let Some(m) = meet {
            // Build A-side path: 1 -> m.
            let mut front = reconstruct(&fwd_parent, m);
            // Build B-side path: m -> n by walking bwd_parent chain.
            let mut back = Vec::new();
            let mut cur = bwd_parent[m];
            while cur != 0 {
                back.push(cur);
                cur = bwd_parent[cur];
            }
            front.extend(back);
            return Some(front);
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Algorithm 3: Dijkstra (binary heap)
// ---------------------------------------------------------------------------
/// All edges treated as weight 1. Dijkstra still produces the shortest path
/// but with O((n + m) log n) overhead from the binary heap.
pub fn solve_dijkstra(n: usize, edges: &[(usize, usize)]) -> Option<Vec<usize>> {
    if n == 0 {
        return None;
    }
    let adj = build_adj(n, edges);
    let mut dist = vec![usize::MAX; n + 1];
    let mut parent = vec![0usize; n + 1];
    let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();

    dist[1] = 0;
    heap.push(Reverse((0, 1)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        if u == n {
            return Some(reconstruct(&parent, n));
        }
        for &v in &adj[u] {
            let nd = d + 1;
            if nd < dist[v] {
                dist[v] = nd;
                parent[v] = u;
                heap.push(Reverse((nd, v)));
            }
        }
    }
    None
}
