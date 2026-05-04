/// Counting Rooms (CSES 1192) — three solving algorithms.
///
/// The grid is a slice of byte vectors where `b'.'` is floor and `b'#'` is wall.
/// Each function returns the number of connected components of floor cells
/// (rooms), where adjacency is 4-directional (up/down/left/right).

const FLOOR: u8 = b'.';

/// Returns (rows, cols) of the grid. Assumes all rows have equal length.
fn dims(grid: &[Vec<u8>]) -> (usize, usize) {
    let n = grid.len();
    let m = if n == 0 { 0 } else { grid[0].len() };
    (n, m)
}

// ---------------------------------------------------------------------------
// Algorithm 1: BFS (queue-based flood fill)
// ---------------------------------------------------------------------------
/// For each unvisited floor cell, BFS marks all reachable floor cells as
/// visited; each successful start increments the room counter.
///
/// Time: O(n·m), Space: O(n·m).
pub fn solve_bfs(grid: &[Vec<u8>]) -> usize {
    let (n, m) = dims(grid);
    if n == 0 || m == 0 {
        return 0;
    }
    let mut visited = vec![vec![false; m]; n];
    let mut rooms = 0;
    let mut queue: Vec<(usize, usize)> = Vec::new();

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] != FLOOR || visited[r][c] {
                continue;
            }
            rooms += 1;
            queue.clear();
            queue.push((r, c));
            visited[r][c] = true;
            let mut head = 0;
            while head < queue.len() {
                let (cr, cc) = queue[head];
                head += 1;
                // Four neighbors. Use saturating arithmetic for bounds.
                let neighbors = [
                    (cr.wrapping_sub(1), cc),
                    (cr + 1, cc),
                    (cr, cc.wrapping_sub(1)),
                    (cr, cc + 1),
                ];
                for &(nr, nc) in &neighbors {
                    if nr < n && nc < m && !visited[nr][nc] && grid[nr][nc] == FLOOR {
                        visited[nr][nc] = true;
                        queue.push((nr, nc));
                    }
                }
            }
        }
    }
    rooms
}

// ---------------------------------------------------------------------------
// Algorithm 2: Iterative DFS (stack-based flood fill)
// ---------------------------------------------------------------------------
/// Same flood-fill structure as BFS but with an explicit LIFO stack,
/// producing depth-first traversal order.
///
/// Time: O(n·m), Space: O(n·m).
pub fn solve_dfs(grid: &[Vec<u8>]) -> usize {
    let (n, m) = dims(grid);
    if n == 0 || m == 0 {
        return 0;
    }
    let mut visited = vec![vec![false; m]; n];
    let mut rooms = 0;
    let mut stack: Vec<(usize, usize)> = Vec::new();

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] != FLOOR || visited[r][c] {
                continue;
            }
            rooms += 1;
            stack.clear();
            stack.push((r, c));
            visited[r][c] = true;
            while let Some((cr, cc)) = stack.pop() {
                let neighbors = [
                    (cr.wrapping_sub(1), cc),
                    (cr + 1, cc),
                    (cr, cc.wrapping_sub(1)),
                    (cr, cc + 1),
                ];
                for &(nr, nc) in &neighbors {
                    if nr < n && nc < m && !visited[nr][nc] && grid[nr][nc] == FLOOR {
                        visited[nr][nc] = true;
                        stack.push((nr, nc));
                    }
                }
            }
        }
    }
    rooms
}

// ---------------------------------------------------------------------------
// Algorithm 3: Union-Find (Disjoint Set Union)
// ---------------------------------------------------------------------------
/// Treats each floor cell as a node. Unions each floor cell with its
/// right and down neighbors when those are also floor. The room count
/// is the number of distinct roots among floor cells.
///
/// Uses union by rank with path compression — near-O(1) per op.
///
/// Time: O(n·m·α(n·m)), Space: O(n·m).
pub fn solve_union_find(grid: &[Vec<u8>]) -> usize {
    let (n, m) = dims(grid);
    if n == 0 || m == 0 {
        return 0;
    }
    let total = n * m;
    let mut parent: Vec<usize> = (0..total).collect();
    let mut rank: Vec<u8> = vec![0; total];

    fn find(parent: &mut [usize], mut x: usize) -> usize {
        while parent[x] != x {
            parent[x] = parent[parent[x]]; // path compression
            x = parent[x];
        }
        x
    }

    fn union(parent: &mut [usize], rank: &mut [u8], a: usize, b: usize) {
        let ra = find(parent, a);
        let rb = find(parent, b);
        if ra == rb {
            return;
        }
        if rank[ra] < rank[rb] {
            parent[ra] = rb;
        } else if rank[ra] > rank[rb] {
            parent[rb] = ra;
        } else {
            parent[rb] = ra;
            rank[ra] += 1;
        }
    }

    let idx = |r: usize, c: usize| r * m + c;

    for r in 0..n {
        for c in 0..m {
            if grid[r][c] != FLOOR {
                continue;
            }
            // Union with right neighbor.
            if c + 1 < m && grid[r][c + 1] == FLOOR {
                union(&mut parent, &mut rank, idx(r, c), idx(r, c + 1));
            }
            // Union with down neighbor.
            if r + 1 < n && grid[r + 1][c] == FLOOR {
                union(&mut parent, &mut rank, idx(r, c), idx(r + 1, c));
            }
        }
    }

    // Count distinct roots among floor cells.
    let mut roots = std::collections::HashSet::new();
    for r in 0..n {
        for c in 0..m {
            if grid[r][c] == FLOOR {
                let root = find(&mut parent, idx(r, c));
                roots.insert(root);
            }
        }
    }
    roots.len()
}
