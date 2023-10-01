fn partition(ret: &mut [usize]) -> usize {
    let ptr_pivot = ret.len() - 1; // pick last element as pivot
    let mut ptr_pivot_ne = usize::MIN;
    for ptr in usize::MIN..ptr_pivot {
        if ret[ptr] >= ret[ptr_pivot] { continue; }

        ret.swap(ptr, ptr_pivot_ne);
        ptr_pivot_ne += 1;
    }
    ret.swap(ptr_pivot, ptr_pivot_ne);

    ptr_pivot_ne
}

/// Sort the given array.
/// This function has an average time complexity of O(n log n).
///
/// Implementation detail:
///   Quicksort, which is an unstable sorting algorithm.
pub fn sort_unstable(ret: &mut [usize]) {
    if ret.is_empty() { return; }

    let ptr_pivot = partition(ret);

    sort_unstable(&mut ret[..ptr_pivot]);
    sort_unstable(&mut ret[ptr_pivot + 1..]);
}

fn main() {
    let mut vec = vec![1, 4, 2, 8, 5, 7];

    sort_unstable(&mut vec);

    println!("sorted: {:?}", vec);
}
