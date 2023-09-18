/// Given an unweighted graph's adjacency list `graph`
///   and its source vertex `src`,
/// Return the minimum distance from `src`
///   to every other nodes.
pub fn bfs(
    graph: Vec<Vec<usize>>,
    src: usize,
) -> Vec<Option<usize>> {
    let mut ret = vec![None; graph.len()];

    let mut count = usize::MIN;
    let mut breadth = vec![src];
    while !breadth.is_empty() {
        let mut breadth_ne = vec![];
        for v in breadth {
            if ret[v].unwrap_or(usize::MAX) <= count {
                continue;
            }
            ret[v] = Some(count);

            breadth_ne.extend(graph[v].clone());
        }
        breadth = breadth_ne;
        count += 1;
    }

    ret
}

fn main() {
    let graph = vec![
        vec![1, 2],
        vec![2],
        vec![0, 3],
        vec![0, 4],
        vec![],
    ];

    let dist_vec = bfs(graph, usize::MIN);
    println!("dist_vec: {:?}", dist_vec);
}
