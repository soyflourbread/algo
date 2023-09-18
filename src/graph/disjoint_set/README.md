# Disjoint Set/Union Find

**Disjoint Set** can be used to maintain a list of disjoint sets.
**Union Find** algorithm is used on disjoint sets
to find which subset a particular element is in.

## Algorithm

We use an array to keep track of each element's subset.
To combine two subsets together, we point
one subset's root to the other subset.

Disjoint sets have near-constant time complexity
and `O(n)` space complexity.

## Implementation

```rust
{{#rustdoc_include disjoint_set.rs::36}}
```
