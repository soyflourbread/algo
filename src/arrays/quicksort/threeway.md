# Threeway Quicksort

For an unsorted array with few distinct values, we can
improve the efficiency of quicksort by grouping equal elements
during the partition step. This reduces quicksort's complexity to
`O(n log k)`, where `k` is the number of distinct elements in the array.

## Implementation

```rust
{{#rustdoc_include quicksort_threeway.rs::55}}
```
