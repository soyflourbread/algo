# A* Search Algorithm

**A-star Search** can be seen as a guided (smarter)
version of Dijkstra Algorithm.
In addition to the adjacency list of the graph,
we are also given a _heuristic function_
that returns the estimated distances from
each vertex to the source vertex.

## Algorithm

An additional _heuristics_ is given in combination with
actual distances to source vertex to guide the search.
Common heuristics are:

* Manhattan distance
* Euclidean distance

A major downside of A-star search is
its space complexity of `O(V)`.
Also, given incorrect heuristics,
it might yield faulty results.

## Implementation

```rust
{{#rustdoc_include a_star.rs::41}}
```
