# Building Teams — Benchmark and Analysis

CSES Problem 1668. Assign each of n pupils to team 1 or team 2 so that no friendship edge connects two pupils on the same team. Output `IMPOSSIBLE` if the friendship graph is not bipartite.

## Algorithms

### 1. BFS 2-coloring (`solve_bfs`)
For each unvisited node, BFS the component assigning alternating colors level by level. If any neighbor already has the same color as the current node, the component contains an odd cycle and the graph is not bipartite.
- **Time:** Θ(n + m).
- **Space:** Θ(n + m) for the adjacency list, color array, and queue.

### 2. Iterative DFS 2-coloring (`solve_dfs`)
Same logic with an explicit LIFO stack instead of a FIFO queue. Detection of odd-cycle / non-bipartiteness is the same: same-color neighbor ⇒ `None`.
- **Time:** Θ(n + m).
- **Space:** Θ(n + m).

### 3. Union-Find with parity (`solve_union_find_parity`)
Weighted DSU where each node carries a parity bit relative to its parent (a.k.a. "potential method"). Path compression also flattens the parity. For each edge (a, b) require `parity(a) ≠ parity(b)`; if a and b are already in the same component with equal parity, report `IMPOSSIBLE`. Final team is parity vs root.
- **Time:** Θ((n + m) · α(n)), where α is the inverse Ackermann.
- **Space:** Θ(n).

## Benchmark Results

Random bipartite graphs (half vs half, edges only between halves):

| n / m            | BFS      | DFS      | Union-Find |
|-----------------:|---------:|---------:|-----------:|
| 1,000 / 2,000    | 58.2 µs  | 49.0 µs  | 95.1 µs    |
| 10,000 / 20,000  | 566.1 µs | 583.8 µs | 958.9 µs   |
| 100,000 / 200,000 | 7.26 ms | 7.55 ms  | 9.95 ms    |

## Interpretation

**BFS and DFS are tied at scale (~7 ms at n = 100k).** Both perform exactly Θ(n + m) work: visit each vertex once, follow each edge twice. The asymptotic equivalence is well known [1, §22.2 BFS; §22.3 DFS]. The practical difference comes down to memory access patterns: DFS pops the most-recently-pushed neighbor, which tends to lie near the previous node in cache (especially on the synthetic bipartite generator that produces locality), while BFS expands radially and touches a wider set of cache lines per "burst" [3, §6]. At n = 1k DFS edges out BFS by ~16 %; the gap closes as cache effects dominate either way.

**Union-Find with parity is the slowest (~1.4× BFS at n = 100k).** Asymptotically it is `(n + m) · α(n)`, which is essentially linear, but the constant factor is significantly larger: every edge requires *two* `find` calls (each with path traversal and compression), plus a union with parity bookkeeping. The compression itself allocates a small temporary `Vec` to record the path. While BFS just does one indexed array write per discovery, DSU does a small amount of pointer-chasing and XOR arithmetic per query [1, §21.4; 2, §1.5].

There is a separate reason to prefer DSU regardless: it is the most natural representation when you have an *online* stream of edges and want to reject the first edge that breaks bipartiteness without re-traversing the whole graph. BFS and DFS are offline algorithms. For the batch problem CSES asks here, BFS wins on simplicity and constant factor.

### References
1. Cormen, T. H., et al. *Introduction to Algorithms*, 4th ed. MIT Press, 2022. §22.2 (BFS), §22.3 (DFS), §21 (disjoint sets and union by rank with path compression).
2. Sedgewick, R., Wayne, K. *Algorithms*, 4th ed. Addison-Wesley, 2011. §1.5 (union-find), §4.1 (connectivity and bipartiteness).
3. Drepper, U. *What Every Programmer Should Know About Memory*. Red Hat, 2007. §3 and §6 (cache effects on graph algorithms).
4. Tarjan, R. E. *Efficiency of a Good but Not Linear Set Union Algorithm*. JACM 22(2), 1975. (Foundational analysis of α(n) bound for union-find.)
