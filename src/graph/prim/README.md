# Prim's Algorithm for Minimum Spanning Tree

**Prim's Algorithm** is a greedy algorithm used to construct a
minimum spanning tree from an undirected, weighted graph.
Unlike _Kruskal's algorithm_, Prim's algorithm is
easier to understand and implement.

## Algorithm

Prim's Algorithm is based on the following observation:

> Given a partial MST, the smallest edge that
  connects the partial tree with other vertices
  must be in the final MST.

So the algorithm is split into three parts:

1. Log vertices in the partial MST.
2. Keep track of the smallest edge neighbouring the partial MST.
3. Explore more vertices with the addition of each edge.

The time complexity is a pleasant `O((V + E) log V)`.
Given how simple this algorithm is, this time complexity
is more than enough.

## Implementation

```rust
{{#rustdoc_include prim.rs::28}}
```
