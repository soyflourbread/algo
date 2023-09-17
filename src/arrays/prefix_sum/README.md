# Prefix Sum

The **Prefix Sum** array is an extra array
that stores the sums of _prefixes_ on the input array.
For example, if we have the given array,

```rust
let vec = vec![1, 2, 3, 4, 5];
```

Its prefix sum array would be

```rust
let pref_vec = [0, 1, 3, 6, 10, 15]; // notice the `0` prefix!!
```

Prefix sum arrays are especially useful when you need to
query the sum of any sub-arrays with _constant_ time complexity.
Let's say we want to know the sum of `vec[1..4]`. We can

1. Iterate through _all elements_ in the sub-array with `vec[1..4].sum::<u32>()`.
2. Use the prefix array with `pref_vec[4] - pref_vec[1]`.

And one method is obviously much faster.

## Algorithm

Iterate through the array, and keep track of
the prefix sum of the current element.
This online algorithm has a time complexity of `O(n)`.
Just make sure your integer type won't overflow!

## Implementation

```rust
{{#rustdoc_include prefix_sum.rs::12}}
```
