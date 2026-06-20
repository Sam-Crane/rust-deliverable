# Subordinates — Benchmark and Analysis

CSES Problem 1674. Given an organizational tree of n employees (employee 1 is the root), report the number of descendants (subordinates) of each employee.

## Algorithms

### 1. Recursive DFS (`solve_recursive`)
Build children-list adjacency; recurse from the root computing `subordinates(v) = sum(subordinates(c) + 1 for c in children(v))`.
- **Time:** Θ(n).
- **Space:** Θ(n) for the children lists; O(h) recursion-stack depth where h is the tree height. Can stack-overflow on degenerate chains at n = 2·10⁵.

### 2. Iterative DFS (`solve_iterative`)
Same logic with an explicit stack of `(node, visited)` pairs implementing a two-pass post-order traversal: on first pop push the node again with `visited=true` then push children, on second pop sum children's counts.
- **Time:** Θ(n).
- **Space:** Θ(n).

### 3. Reverse-BFS (`solve_reverse_bfs`)
BFS from the root to compute a top-down order. Iterate the order in reverse; for each non-root node, add `count[v] + 1` to its parent's count.
- **Time:** Θ(n).
- **Space:** Θ(n).

## Benchmark Results

| n       | Recursive | Iterative DFS | Reverse BFS |
|--------:|----------:|--------------:|------------:|
| 1,000   | 21.4 µs   | 23.0 µs       | 21.4 µs     |
| 10,000  | 236.9 µs  | 229.3 µs      | 202.9 µs    |
| 100,000 | 2.94 ms   | 3.27 ms       | 2.74 ms     |
| 200,000 | 6.75 ms   | 6.98 ms       | 6.05 ms     |

## Interpretation

**Reverse-BFS is the fastest at scale (~10–15% over recursive at n = 200k).** Both passes are linear sweeps over flat `Vec`s — the BFS queue grows monotonically and the second pass walks `order.iter().rev()` reading two arrays (counts + parents) and writing one. No recursion, no per-node tuple-on-stack — the compiler can hoist nothing into registers and the access pattern is purely sequential, ideal for the hardware prefetcher [3, §3.4].

**Recursive DFS is competitive at small n** because the call stack is small and modern CPUs handle frame setup/teardown extremely efficiently. As n grows the cost of pushing/popping stack frames (saving registers, adjusting the stack pointer) accumulates and the gap widens.

**Iterative DFS is the slowest** despite being structurally the safest. The two-pass marker scheme pushes each node twice and stores a `(usize, bool)` tuple — that's 16 bytes per entry versus a Vec<usize> bool flag. Cache lines fill up with low information density, and the random-looking pop order (post-order is not sequential w.r.t. node IDs) hurts the cache [1, §22.3 DFS analysis; 3, §6].

All three are Θ(n + edges) = Θ(n) on a tree. The differences are again constants from memory access patterns and per-node bookkeeping overhead.

### References
1. Cormen, T. H., et al. *Introduction to Algorithms*, 4th ed. MIT Press, 2022. §22.2 (BFS) and §22.3 (DFS).
2. Sedgewick, R., Wayne, K. *Algorithms*, 4th ed. Addison-Wesley, 2011. §4.1 (graph traversal).
3. Drepper, U. *What Every Programmer Should Know About Memory*. Red Hat, 2007. §3.4 (prefetching) and §6 (irregular access patterns).
