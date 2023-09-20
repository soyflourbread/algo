# Floyd's Tortoise and Hare for Cycle Detection

**Floyd's Tortoise and Hare** can be used to find
the first cycle in a functional graph.
An example of such functional graph is
a **linked list with cycles**.

## Algorithm

Let's look at the simplified version of a cyclic graph:

```
0 -> 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -|
                    ^                      |
                    |______________________|
```

This graph has `a=4` edges before the cycle,
and `b=5` edges within the cycle.

Floyd's Algorithm involves two pointers:
the tortoise and the hare (pointers).

### Part I

In the first part of the algorithm,
these two pointers propagate at different speeds:

* Tortoise pointer goes to the next vertex in one iteration.
* Hare pointer goes to the **next two** vertex in one iteration.

It ends when the two pointers met.
Since one pointer is faster than the other,
the only place they could've met is **in the cycle**.
Let's mark this meeting place as `k` steps from
the cycle's entrance.

In this part,

* Tortoise pointer moved through `a + k` edges.
* Hare pointer moved through `a + i * b + k` edges,
  where `i` is any positive integer.

Since hare moves twice as fast, we know that

```
(a + k) * 2 == a + i * b + k
```

which is equivalent to
`a + k == i * b`. Surprisingly, this means that
the number of edges the tortoise went through
is _a multiple of the size of the cycle_.
This will be the base for the second part of the algorithm.

### Part II

Now we reset **the hare** to the starting vertex.
Unlike the first part, in this part, the two pointers
move at the same speed, one edge at a time.
The algorithm still ends when the two pointers met.

**The vertex the two pointers met is the entrance to the cycle.**
Why? Let's consider where the tortoise is when
the hare had just reached the entrance of the cycle:

* The hare moved through `a` edges.
* The tortoise moved through `a` extra edges,
  making it `2a + k` edges in total.

Equivalently, the tortoise moved `a + k` edges
away from the entrance of the cycle.
Remember `a + k` is a multiple of the size of the cycle?
Yep, the **tortoise is back at the entrance of the cycle**.

This algorithm surprisingly has linear time complexity
and constant space complexity.

## Implementation

```rust
{{#rustdoc_include tortoise_hare.rs::24}}
```
