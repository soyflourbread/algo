use std::collections::BinaryHeap as MaxHeap;
use std::cmp::Reverse as Rev;

/// Given a weighted directed graph,
///   returns the minimum distances from `src` to
///   every vertices in the graph.
/// Parameters:
///   `graph`: adjacency list for the graph
///   `src`: source vertex
pub fn dijkstra(
    graph: Vec<Vec<(usize, u32)>>,
    src: usize,
) -> Vec<Option<u32>> {
    let mut ret = vec![None; graph.len()];

    let mut heap = MaxHeap::from(
        [Rev((u32::MIN, src))]
    ); // min-heap
    while let Some(Rev((dist, v))) = heap.pop() {
        if ret[v].unwrap_or(u32::MAX) <= dist {
            continue;
        }
        ret[v] = Some(dist);

        for &(v_ne, w) in &graph[v] {
            heap.push(Rev((dist + w, v_ne)));
        }
    }

    ret
}

fn main() {
    let graph = vec![
        vec![(1, 1), (2, 3)],
        vec![(2, 4)],
        vec![(0, 2), (3, 1)],
        vec![(0, 9), (4, 2)],
        vec![],
    ];

    let dist_vec = dijkstra(graph, usize::MIN);
    println!("dist_vec: {:?}", dist_vec);
}
