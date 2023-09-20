use std::collections::BinaryHeap as MaxHeap;
use std::cmp::Reverse as Rev;

// Given a weighted directed graph,
//   returns the minimum distances from `src` to
//   every vertices in the graph.
// Parameters:
//   `graph`: adjacency list for the graph
//   `src`: source vertex
//   `heuristic_fn`: the heuristics
pub fn a_star<H: FnMut(usize) -> u32>(
    graph: Vec<Vec<(usize, u32)>>,
    src: usize,
    mut heuristic_fn: H,
) -> Vec<Option<u32>> {
    let mut dist_vec = vec![None; graph.len()];
    let mut f_vec = vec![None; graph.len()];

    let mut heap = MaxHeap::new(); // min-heap
    heap.push(Rev((heuristic_fn(src), u32::MIN, src)));
    while let Some(Rev((f_dist, dist, v))) = heap.pop() {
        if f_vec[v].unwrap_or(u32::MAX) <= f_dist {
            continue;
        }
        f_vec[v] = Some(f_dist);

        if dist_vec[v].unwrap_or(u32::MAX) <= dist {
            continue;
        }
        dist_vec[v] = Some(dist);

        for &(v_ne, weight) in &graph[v] {
            let dist_ne = dist + weight;
            let f_dist_ne = dist_ne + heuristic_fn(v_ne);

            heap.push(Rev((f_dist_ne, dist_ne, v_ne)));
        }
    }

    dist_vec
}

fn main() {
    let graph = vec![
        vec![(1, 1), (2, 3)],
        vec![(2, 4)],
        vec![(0, 2), (3, 1)],
        vec![(0, 9), (4, 2)],
        vec![],
    ];

    let dist_vec = a_star(
        graph, usize::MIN,
        |v| u32::MIN
    );
    println!("dist_vec: {:?}", dist_vec);
}
