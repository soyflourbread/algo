/// Three-way partition.
/// Partitions the vector into [<, =, >] respectively.
///
/// Arguments:
///   `ret`: vector to partition
fn partition(
    ret: &mut Vec<usize>,
    ptr_0: usize, ptr_1: usize,
) -> [usize; 2] {
    assert!(!ret.is_empty()); // at least one element

    let (ptr_pivot, mut ptr_pivot_ne) = (ptr_1 - 1, ptr_0);
    for ptr in ptr_0..ptr_pivot {
        if ret[ptr] > ret[ptr_pivot] { continue; }

        ret.swap(ptr, ptr_pivot_ne);
        ptr_pivot_ne += 1;
    }
    ret.swap(ptr_pivot, ptr_pivot_ne);

    let (ptr_pivot, mut ptr_pivot_ne) = (ptr_pivot_ne, ptr_0);
    for ptr in ptr_0..ptr_pivot {
        if ret[ptr] >= ret[ptr_pivot] { continue; }

        ret.swap(ptr, ptr_pivot_ne);
        ptr_pivot_ne += 1;
    }

    [ptr_pivot_ne, ptr_pivot + 1]
}

/// Sort the given array.
/// This function has an average time complexity of O(n log k).
///
/// Implementation detail:
///   Quicksort, which is an unstable sorting algorithm.
pub fn sort_unstable(ret: &mut Vec<usize>) {
    if ret.is_empty() { return; }

    fn _impl(
        ret: &mut Vec<usize>,
        ptr_0: usize, ptr_1: usize,
    ) {
        if ptr_0 >= ptr_1 { return; }

        let [pivot_0, pivot_1] = partition(
            ret, ptr_0, ptr_1,
        );

        _impl(ret, ptr_0, pivot_0);
        _impl(ret, pivot_1, ptr_1);
    }

    _impl(ret, usize::MIN, ret.len())
}

fn main() {
    let mut vec = vec![1, 4, 2, 8, 5, 7];

    sort_unstable(&mut vec);

    println!("sorted: {:?}", vec);
}
