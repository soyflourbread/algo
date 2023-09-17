# Sliding Window

**Sliding Window** is an optimization technique
to reduce the number of nested loops.
Let's say given an array `vec` with non-negative elements,
we want to know the number of sub-arrays
with sum equal to `k`. We can either

1. Calculate the sums of every possible sub-arrays.
   * Since there are `n^2` sub-arrays,
     this method has a minimum time complexity of `O(n^2)`.
     And yes, a prefix sum array would be necessary.
2. Notice that if sub-array `vec[ptr_0..ptr_1]`
   has sum greater than `k`, sum of `vec[ptr_0..(ptr_1 + 1)]`
   must also be greater than `k`.

If we can keep track of a minimum sub-array (aka **window**)
as we iterate through the array,
we can get the result by simply counting
how many windows sum to `k`!!
And that's the basic idea of a sliding window.

## Algorithm

Sliding window is more of a technique than a fixed algorithm.
However, we can summarize some common logic.

1. Keep the window size to a minimum.
2. Remove front elements first (the sliding).
3. Add new elements to the back.
4. Keep track of the window's internal states (e.g. its sum).

## Implementation

```rust,noplayground
{{#rustdoc_include sliding_window.rs}}
```
