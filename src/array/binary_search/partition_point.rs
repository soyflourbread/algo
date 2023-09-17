/// Given a partitioned vector of [True, True, ..., False, False],
///   find the index to the partition point.
///
/// This function is identical to Vec::partition_point(),
///   but it generates elements in the vector on-demand from `predicate`,
///   making it O(\log n) space-wise.
///
/// This is especially useful when searching across a large solution space.
pub fn partition_point<P: FnMut(usize) -> bool>(
    p0: usize, p1: usize,
    mut predicate: P,
) -> usize {
    if p0 >= p1 { return p0; }

    let p_bound = p0 + (p1 - p0) / 2;
    let (p0_ne, p1_ne) = if predicate(p_bound) {
        (p_bound + 1, p1)
    } else { (p0, p_bound) };

    // tail recursion for O(1) space
    partition_point(p0_ne, p1_ne, predicate)
}

fn main() {
    let vec = vec![true, true, false, false, false];

    let ptr = partition_point(
        usize::MIN, vec.len(),
        |i| vec[i],
    );

    println!("partition point: {}", ptr);
}
