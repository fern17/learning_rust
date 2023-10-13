use disjoint_sets::UnionFind;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

type Node = usize;
type Weight = usize;

struct Edge {
    dest: Node,
    weight: Weight,
}

type Graph = Vec<Vec<Edge>>;

fn edges_by_weight(graph: &Graph) -> Vec<(Node, Node, Weight)> {
    let mut edges = vec![];
    for (src, dest) in graph.iter().enumerate() {
        for edge in dest {
            edges.push((src, edge.dest, edge.weight));
        }
    }
    edges.sort_by_key(|&(_, _, w)| w);
    edges
}

fn minimum_spanning_tree_kruskal(graph: &Graph) -> Vec<(Node, Node)> {
    let mut result = vec![];
    let mut union_find = UnionFind::new(graph.len());

    for (src, dest, _) in edges_by_weight(graph) {
        if !union_find.equiv(src, dest) {
            union_find.union(src, dest);
            result.push((src, dest));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mst() {
        let graph = vec![
            vec![
                Edge { dest: 1, weight: 3 },
                Edge { dest: 3, weight: 6 },
                Edge { dest: 5, weight: 2 },
            ],
            vec![
                Edge { dest: 3, weight: 5 },
                Edge { dest: 5, weight: 4 },
                Edge { dest: 2, weight: 1 },
            ],
            vec![Edge { dest: 3, weight: 2 }, Edge { dest: 4, weight: 3 }],
            vec![Edge { dest: 4, weight: 7 }],
            vec![Edge { dest: 5, weight: 2 }],
            vec![],
        ];

        let expected_solution_kruskal = vec![(1, 2), (0, 5), (2, 3), (4, 5), (0, 1)];
        assert_eq!(
            minimum_spanning_tree_kruskal(&graph),
            expected_solution_kruskal
        );
    }
}

// ------------------

type GraphP<V, E> = BTreeMap<V, BTreeMap<V, E>>;
fn add_edge<V: Ord + Copy, E: Ord + Copy>(graph: &mut GraphP<V, E>, v1: V, v2: V, w: E) {
    graph.entry(v1).or_insert_with(BTreeMap::new).insert(v2, w);
    graph.entry(v2).or_insert_with(BTreeMap::new).insert(v1, w);
}

pub fn prim<V: Ord + Copy, E: Ord + Copy>(graph: &GraphP<V, E>, start: V) -> GraphP<V, E> {
    let mut mst = GraphP::new();
    let mut visited = BinaryHeap::new();

    mst.insert(start, BTreeMap::new());

    for (v, w) in &graph[&start] {
        visited.push(Reverse((*w, v, start)));
    }

    while let Some(Reverse((w, v, prev))) = visited.pop() {
        if mst.contains_key(v) {
            continue;
        }

        add_edge(&mut mst, prev, *v, w);

        for (vi, w) in &graph[v] {
            if !mst.contains_key(vi) {
                visited.push(Reverse((*w, vi, *v)));
            }
        }
    }

    mst
}

#[cfg(test)]
mod tests_prim {
    use super::*;
    #[test]
    fn test_prim() {
        let mut graph = BTreeMap::new();
        add_edge(&mut graph, 0, 1, 3);
        add_edge(&mut graph, 0, 2, 4);
        add_edge(&mut graph, 1, 2, 1);
        add_edge(&mut graph, 1, 4, 6);
        add_edge(&mut graph, 2, 4, 3);
        add_edge(&mut graph, 2, 3, 5);
        add_edge(&mut graph, 3, 4, 2);

        let mut ans = BTreeMap::new();
        add_edge(&mut ans, 0, 1, 3);
        add_edge(&mut ans, 1, 2, 1);
        add_edge(&mut ans, 2, 4, 3);
        add_edge(&mut ans, 4, 3, 2);

        assert_eq!(prim(&graph, 0), ans);
    }
}
