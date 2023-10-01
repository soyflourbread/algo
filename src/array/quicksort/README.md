# Quicksort

**Quicksort**(or more intuitively, **partition-exchange sort**)
is a randomized sorting algorithm
that's extremely efficient in practice.
Despite its `O(n^2)` worst-time complexity,
it often beats merge sort (`O(n log n)`) on randomized data.

Basically, quicksort is *really fast*.

## Algorithm

We pick a random element from the array and call it "the **pivot**".
Then,

1. Swap all elements smaller than `pivot` to the front of the array.
2. Put the `pivot` immediately after all elements smaller than itself.
   * The array's now partitioned against `pivot`.
3. Recursively sort the left and right partitions.

Ideally on randomized data, the two partitions
should have similar sizes. Therefore,
there will be `log n` recursive steps ideally.
Since each recursive step has time complexity of `O(n)`,
this gives quicksort an overall complexity of `O(n log n)`.

## Implementation

```rust
{{#rustdoc_include quicksort.rs::27}}
```
