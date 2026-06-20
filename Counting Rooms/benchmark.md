# Counting Rooms — Benchmark and Analysis

CSES Problem 1192. Count connected components of floor cells (`.`) in an n×m grid where `#` are walls and adjacency is 4-directional.

## Algorithms

### 1. BFS (`solve_bfs`)
For each unvisited floor cell, BFS marks all reachable floor cells visited; each successful start increments the counter. Uses a `Vec` as a FIFO queue with a head index.
- **Time:** Θ(n·m).
- **Space:** Θ(n·m) for the visited matrix and queue.

### 2. Iterative DFS (`solve_dfs`)
Same flood-fill pattern but with a LIFO stack — push neighbors, pop and explore.
- **Time:** Θ(n·m).
- **Space:** Θ(n·m).

### 3. Union-Find (`solve_union_find`)
Each floor cell is a DSU node. For every floor cell, union it with its right and down floor neighbors. Final answer = number of distinct roots among floor cells. Uses path compression + union by rank.
- **Time:** Θ(n·m · α(n·m)) where α is the inverse Ackermann function (≈ constant).
- **Space:** Θ(n·m) for the parent and rank arrays.

## Benchmark Results

Random grid with ~70 % floor / ~30 % wall:

| Grid       | BFS     | Iterative DFS | Union-Find |
|-----------:|--------:|--------------:|-----------:|
| 100×100    | 107.0 µs | 71.2 µs       | 107.9 µs   |
| 300×300    | 1.32 ms | 953.1 µs      | 918.4 µs   |
| 500×500    | 3.54 ms | 2.95 ms       | 2.76 ms    |
| 1000×1000  | 15.2 ms | 11.9 ms       | 11.2 ms    |

## Interpretation

**Union-Find ties with DFS at the top end and beats BFS by ~26 % at 1000×1000.** DSU avoids both the queue/stack overhead and the "discover unvisited cell" check inside an inner loop — it just walks the grid in row-major order and performs two cheap unions per floor cell. Union by rank with path compression makes the amortized cost ~O(α(n·m)) ≈ O(1) per operation [1, §21.4 Tarjan's analysis].

**Iterative DFS beats BFS** even though both are Θ(n·m). DFS pops the most recently discovered cell, so the next cell explored is usually adjacent (spatially close) — this hits warm cache lines in the visited matrix. BFS expands radially, which causes more cache misses on the visited matrix as the frontier grows [3, §6]. Modern CPUs also predict the simpler `while let Some((cr, cc)) = stack.pop()` better than the BFS queue's `head < queue.len()` loop.

**BFS is slowest but still O(n·m).** Its cost is the queue-management overhead: every push extends the `Vec`, every pop advances the head index, and the head-vs-len check is a separate branch per inner iteration. None of this is asymptotically worse, but for a single flood-fill where shortest paths are irrelevant the queue is pure overhead — DFS and DSU both win [2, §4.1].

### References
1. Cormen, T. H., et al. *Introduction to Algorithms*, 4th ed. MIT Press, 2022. §21 (disjoint sets, union by rank, path compression) and §22.2/§22.3 (BFS/DFS).
2. Sedgewick, R., Wayne, K. *Algorithms*, 4th ed. Addison-Wesley, 2011. §1.5 (union-find) and §4.1 (connected components).
3. Drepper, U. *What Every Programmer Should Know About Memory*. Red Hat, 2007. §6 (irregular memory access).
