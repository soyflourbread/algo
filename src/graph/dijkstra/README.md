# Dijkstra's Algorithm

**Dijkstra's Algorithm** can be seen as the generalization
of Breadth-First Search.
Similar to BFS, Dijkstra's Algorithm is used to find
the shortest path between two nodes.
Unlike BFS, it also works on *weighted graphs*.

## Algorithm

Instead of just using a queue, a **priority queue** is needed
to find the unvisited vertices with the smallest distances.

Implementations for Dijkstra's Algorithm
generally looks like this:

1. Pick the vertex `v` with minimum distance
   to `src` vertex from the priority queue.
2. For each adjacent vertex `v_next` of `v`,
   calculate its distance to `src`,
   and insert it into the priority queue.

## Implementation

```rust
{{#rustdoc_include dijkstra.rs::30}}
```
