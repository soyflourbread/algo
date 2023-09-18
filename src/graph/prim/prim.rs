use std::collections::BinaryHeap as MaxHeap;
use std::cmp::Reverse as Rev;

pub fn prim(
    graph: Vec<Vec<(usize, u32)>>,
) -> Vec<([usize; 2], u32)> {
    let mut ret = vec![];

    let mut visited = vec![false; graph.len()];
    let mut heap = MaxHeap::from(
        [Rev((u32::MIN, [usize::MIN; 2]))] // dummy edge
    ); // min-heap
    while let Some(Rev((weight, [v, v_pr]))) = heap.pop() {
        if visited[v] { continue; }
        visited[v] = true;

        for &(v_ne, weight_ne) in &graph[v] {
            heap.push(Rev((weight_ne, [v_ne, v])));
        }

        ret.push(([v_pr, v], weight));
    }

    ret.reverse();
    ret.pop(); // remove dummy edge

    ret
}

fn main() {
    let graph = vec![
        vec![(1, 5), (2, 1)],
        vec![(0, 5), (2, 3)],
        vec![(1, 3), (0, 1)],
    ];

    let tree = prim(graph);
    println!("mst: {tree:?}");
}
