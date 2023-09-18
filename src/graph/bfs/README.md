# Breadth-First Search

**Breadth-First Search** is useful for finding
the minimum distance between two nodes in a
directed, unweighted graph.
An **unweighted graph**'s edges have no (aka equal) weights.

## Algorithm

We use an array to keep track of
each vertex's minimum distances to the source vertex.
The current distance to the source vertex
is equal to **the number of iterations**.

For each iteration, iterate through vertices in the queue:

1. If the vertex's minimum distance is
   smaller than the current distance, skip it.
2. Otherwise, set the vertex's minimum distance
   to the current distance.
3. Add all adjacent vertices to the back of the queue.

The above BFS implementation, if adjacency lists are used,
has a time complexity of `O(V + E)`.

## Implementation

Most implementations of BFS use a deque.
However, using two arrays is simpler and is equally fast,
so that is what the implementation below uses.

```rust
{{#rustdoc_include bfs.rs::28}}
```
