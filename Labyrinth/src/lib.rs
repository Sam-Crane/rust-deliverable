/// Labyrinth (CSES 1193) — three solving algorithms.
///
/// Grid: '.' = floor, '#' = wall, 'A' = start, 'B' = end.
/// Each function returns Some((distance, path)) if reachable, else None.
/// `path` is a string of L/R/U/D moves from A to B.
use std::collections::{BinaryHeap, VecDeque};

const FLOOR: u8 = b'.';
const START: u8 = b'A';
const END: u8 = b'B';

/// Four moves with their (dr, dc) offsets and direction letter.
/// Order: L, R, U, D so the iteration order matches the typical CSES sample
/// output ordering when there are ties.
const MOVES: [(isize, isize, u8); 4] = [(0, -1, b'L'), (0, 1, b'R'), (-1, 0, b'U'), (1, 0, b'D')];

fn dims(grid: &[Vec<u8>]) -> (usize, usize) {
    let n = grid.len();
    let m = if n == 0 { 0 } else { grid[0].len() };
    (n, m)
}

fn passable(c: u8) -> bool {
    c == FLOOR || c == START || c == END
}

fn find_cell(grid: &[Vec<u8>], target: u8) -> Option<(usize, usize)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == target {
                return Some((r, c));
            }
        }
    }
    None
}

/// Reconstruct the path from a `from_dir` array: for each visited cell,
/// `from_dir[idx]` is the direction-letter that was taken to reach it.
/// We walk backwards from B using the inverse of each move.
fn reconstruct(from_dir: &[u8], m: usize, start: (usize, usize), end: (usize, usize)) -> String {
    let mut path = Vec::new();
    let (mut r, mut c) = end;
    while (r, c) != start {
        let dir = from_dir[r * m + c];
        path.push(dir);
        // Step backwards using the inverse of the recorded move.
        match dir {
            b'L' => c += 1,
            b'R' => c -= 1,
            b'U' => r += 1,
            b'D' => r -= 1,
            _ => unreachable!(),
        }
    }
    path.reverse();
    String::from_utf8(path).unwrap()
}

// ---------------------------------------------------------------------------
// Algorithm 1: Standard BFS
// ---------------------------------------------------------------------------
/// Single-source BFS from A, recording the move letter used to enter each
/// cell. Path is reconstructed by backtracking from B.
///
/// Time: O(n·m), Space: O(n·m).
pub fn solve_bfs(grid: &[Vec<u8>]) -> Option<(usize, String)> {
    let (n, m) = dims(grid);
    let start = find_cell(grid, START)?;
    let end = find_cell(grid, END)?;

    let mut visited = vec![false; n * m];
    let mut from_dir = vec![0u8; n * m];
    let mut dist = vec![0usize; n * m];

    let idx = |r: usize, c: usize| r * m + c;
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    visited[idx(start.0, start.1)] = true;
    queue.push_back(start);

    while let Some((r, c)) = queue.pop_front() {
        if (r, c) == end {
            return Some((dist[idx(r, c)], reconstruct(&from_dir, m, start, end)));
        }
        for &(dr, dc, letter) in &MOVES {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nr >= n || nc >= m {
                continue;
            }
            let ni = idx(nr, nc);
            if visited[ni] || !passable(grid[nr][nc]) {
                continue;
            }
            visited[ni] = true;
            from_dir[ni] = letter;
            dist[ni] = dist[idx(r, c)] + 1;
            queue.push_back((nr, nc));
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Algorithm 2: Bidirectional BFS
// ---------------------------------------------------------------------------
/// BFS simultaneously from A and B. The smaller frontier is expanded each
/// round. When a cell discovered by the forward search is reached by the
/// backward search (or vice versa), the path is stitched together.
///
/// In the worst case still O(n·m), but typically explores fewer cells.
pub fn solve_bidirectional(grid: &[Vec<u8>]) -> Option<(usize, String)> {
    let (n, m) = dims(grid);
    let start = find_cell(grid, START)?;
    let end = find_cell(grid, END)?;
    if start == end {
        return Some((0, String::new()));
    }

    let idx = |r: usize, c: usize| r * m + c;
    let total = n * m;

    // Two BFS frontiers with their own visited/parent maps.
    // For the forward search we record the direction taken to enter each cell.
    // For the backward search we record the direction taken FROM each cell
    // toward the predecessor when expanding from B; on stitch we still get
    // a valid path by walking backward through the backward `from_dir`.
    let mut fwd_visited = vec![false; total];
    let mut bwd_visited = vec![false; total];
    let mut fwd_dir = vec![0u8; total]; // direction taken to enter this cell from A side
    let mut bwd_dir = vec![0u8; total]; // direction taken to enter this cell from B side
    let mut fwd_q: VecDeque<(usize, usize)> = VecDeque::new();
    let mut bwd_q: VecDeque<(usize, usize)> = VecDeque::new();

    fwd_visited[idx(start.0, start.1)] = true;
    bwd_visited[idx(end.0, end.1)] = true;
    fwd_q.push_back(start);
    bwd_q.push_back(end);

    // Expand one BFS layer at a time, returning the meeting point if found.
    fn expand_layer(
        n: usize,
        m: usize,
        grid: &[Vec<u8>],
        q: &mut VecDeque<(usize, usize)>,
        my_visited: &mut [bool],
        my_dir: &mut [u8],
        other_visited: &[bool],
    ) -> Option<(usize, usize)> {
        let idx = |r: usize, c: usize| r * m + c;
        let len = q.len();
        for _ in 0..len {
            let (r, c) = q.pop_front().unwrap();
            for &(dr, dc, letter) in &MOVES {
                let nr = r as isize + dr;
                let nc = c as isize + dc;
                if nr < 0 || nc < 0 {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if nr >= n || nc >= m {
                    continue;
                }
                let ni = idx(nr, nc);
                if my_visited[ni] || !passable(grid[nr][nc]) {
                    continue;
                }
                my_visited[ni] = true;
                my_dir[ni] = letter;
                if other_visited[ni] {
                    return Some((nr, nc));
                }
                q.push_back((nr, nc));
            }
        }
        None
    }

    let stitch = |meet: (usize, usize),
                  from_a_side: bool,
                  fwd_dir: &[u8],
                  bwd_dir: &[u8]|
     -> (usize, String) {
        // Build forward portion: A -> meet using fwd_dir.
        let mut forward = Vec::new();
        let (mut r, mut c) = meet;
        while (r, c) != start {
            let d = fwd_dir[idx(r, c)];
            forward.push(d);
            match d {
                b'L' => c += 1,
                b'R' => c -= 1,
                b'U' => r += 1,
                b'D' => r -= 1,
                _ => unreachable!(),
            }
        }
        forward.reverse();

        // Build backward portion: meet -> B using bwd_dir.
        // bwd_dir[x] is the direction taken (from B side) to enter x.
        // To go from meet to B, we walk B-side parent chain from meet to B
        // and invert each direction.
        let mut backward = Vec::new();
        let (mut r, mut c) = meet;
        while (r, c) != end {
            let d = bwd_dir[idx(r, c)];
            // To move B->meet, d was used; the inverse for meet->B is opposite.
            let inv = match d {
                b'L' => b'R',
                b'R' => b'L',
                b'U' => b'D',
                b'D' => b'U',
                _ => unreachable!(),
            };
            backward.push(inv);
            // Walk to parent on the B side.
            match d {
                b'L' => c += 1,
                b'R' => c -= 1,
                b'U' => r += 1,
                b'D' => r -= 1,
                _ => unreachable!(),
            }
        }

        let _ = from_a_side;
        forward.extend(backward);
        let dist = forward.len();
        (dist, String::from_utf8(forward).unwrap())
    };

    while !fwd_q.is_empty() && !bwd_q.is_empty() {
        // Expand the smaller frontier first.
        if fwd_q.len() <= bwd_q.len() {
            if let Some(meet) = expand_layer(
                n,
                m,
                grid,
                &mut fwd_q,
                &mut fwd_visited,
                &mut fwd_dir,
                &bwd_visited,
            ) {
                return Some(stitch(meet, true, &fwd_dir, &bwd_dir));
            }
        } else if let Some(meet) = expand_layer(
            n,
            m,
            grid,
            &mut bwd_q,
            &mut bwd_visited,
            &mut bwd_dir,
            &fwd_visited,
        ) {
            return Some(stitch(meet, false, &fwd_dir, &bwd_dir));
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Algorithm 3: A* with Manhattan distance heuristic
// ---------------------------------------------------------------------------
/// Best-first search ordered by f = g + h, where h is Manhattan distance to B.
/// On a unit-cost grid Manhattan is admissible, so A* still yields the
/// shortest path.
pub fn solve_astar(grid: &[Vec<u8>]) -> Option<(usize, String)> {
    let (n, m) = dims(grid);
    let start = find_cell(grid, START)?;
    let end = find_cell(grid, END)?;

    let idx = |r: usize, c: usize| r * m + c;
    let manhattan = |r: usize, c: usize| {
        ((r as isize - end.0 as isize).unsigned_abs() as usize)
            + ((c as isize - end.1 as isize).unsigned_abs() as usize)
    };

    let mut g = vec![usize::MAX; n * m];
    let mut from_dir = vec![0u8; n * m];
    let mut closed = vec![false; n * m];

    // BinaryHeap is a max-heap; store negated f for min behavior via Reverse.
    use std::cmp::Reverse;
    let mut heap: BinaryHeap<Reverse<(usize, usize, usize)>> = BinaryHeap::new();
    g[idx(start.0, start.1)] = 0;
    heap.push(Reverse((manhattan(start.0, start.1), start.0, start.1)));

    while let Some(Reverse((_f, r, c))) = heap.pop() {
        if closed[idx(r, c)] {
            continue;
        }
        closed[idx(r, c)] = true;
        if (r, c) == end {
            return Some((g[idx(r, c)], reconstruct(&from_dir, m, start, end)));
        }
        for &(dr, dc, letter) in &MOVES {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nc < 0 {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if nr >= n || nc >= m {
                continue;
            }
            let ni = idx(nr, nc);
            if !passable(grid[nr][nc]) {
                continue;
            }
            let tentative = g[idx(r, c)] + 1;
            if tentative < g[ni] {
                g[ni] = tentative;
                from_dir[ni] = letter;
                heap.push(Reverse((tentative + manhattan(nr, nc), nr, nc)));
            }
        }
    }
    None
}
