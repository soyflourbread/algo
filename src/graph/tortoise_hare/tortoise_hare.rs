/// Given a directed functional graph with one cycle,
///   returns the entrance vertex to the cycle.
/// Parameters:
///   `graph`: function to the next vertex
pub fn cycle_detection<G: FnMut(usize) -> usize>(
    mut graph: G,
) -> usize {
    let mut v0 = graph(usize::MIN);
    let mut v1 = graph(v0);

    while v0 != v1 {
        v0 = graph(v0);
        v1 = graph(v1);
        v1 = graph(v1);
    }

    let mut v1 = usize::MIN;
    while v0 != v1 {
        v0 = graph(v0);
        v1 = graph(v1);
    }

    v0
}

fn main() {
    let graph = vec![1, 3, 4, 2, 2];

    let v = cycle_detection(|v| graph[v]);
    println!("vertex: {v}");
}
