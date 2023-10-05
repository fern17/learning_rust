use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct NodeNotInGraph; // error type if node does not exist

pub struct DirectedGraph {
    adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

pub struct UndirectedGraph {
    adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

pub trait AdjacencyMatrixGraph {
    fn new() -> Self;
    fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_matrix().get(node) {
            None => {
                self.adjacency_matrix()
                    .insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    // directed edge from first argument to the second argument
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_matrix()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    fn neighbors(&mut self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        match self.adjacency_matrix().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
}

impl AdjacencyMatrixGraph for DirectedGraph {
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjacency_matrix: HashMap::new(),
        }
    }

    fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_matrix
    }
}

impl AdjacencyMatrixGraph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_matrix: HashMap::new(),
        }
    }

    fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_matrix
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_matrix
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });

        self.adjacency_matrix
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}

#[cfg(test)]
mod test_unidirected_graph {
    use super::*;
    #[test]
    fn test_neighbors() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));

        assert_eq!(
            graph.neighbors("a").unwrap(),
            &vec![(String::from("b"), 5), (String::from("c"), 7),]
        );
    }
}

#[cfg(test)]
mod test_directed_graph {
    use super::*;
    #[test]
    fn test_neighbors() {
        let mut graph = DirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        graph.add_edge(("b", "a", 5));

        assert_eq!(graph.neighbors("a").unwrap(), &vec![(String::from("b"), 5)]);
        assert_eq!(
            graph.neighbors("b").unwrap(),
            &vec![(String::from("c"), 10), (String::from("a"), 5)]
        );
    }
}

// -------------------------------------------------------------------------------------------

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u8);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u8, u8);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u8> for Vertex {
    fn from(item: u8) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u8, u8)> for Edge {
    fn from(item: (u8, u8)) -> Self {
        Edge(item.0, item.1)
    }
}

pub fn depth_first_search(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u8>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u8> = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(root);

    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());
        if current_vertex == target {
            return Some(history);
        }
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            if visited.insert(neighbor) {
                queue.push_front(neighbor)
            }
        }
    }

    None
}

pub fn breadth_first_search(graph: &Graph, root: Vertex, target: Vertex) -> Option<Vec<u8>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u8> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);

    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());
        if current_vertex == target {
            return Some(history);
        }
        for neighbor in current_vertex.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor)
            }
        }
    }

    None
}

#[cfg(test)]
mod test_graph {
    use super::*;
    #[test]
    fn test_dfs() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
        let root = 1;
        let objective = 7;
        let correct = vec![1, 2, 4, 5, 3, 6, 7];
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );
        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct)
        );
    }
    #[test]
    fn test_bfs_not_found() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
        let root = 1;
        let objective = 10;
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );
        assert_eq!(
            breadth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }
    #[test]
    fn test_bfs_found() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];
        let root = 1;
        let objective = 7;
        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );
        let correct = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(
            breadth_first_search(&graph, root.into(), objective.into()),
            Some(correct)
        );
    }
}

// ---------------------------------------
#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraState {
    cost: usize,
    position: usize,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DijkstraEdge {
    node: usize,
    cost: usize,
}

fn shortest_path_dijkstra(
    graph: &Vec<Vec<DijkstraEdge>>,
    start: usize,
    goal: usize,
) -> Option<usize> {
    let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
    let mut visited = BinaryHeap::new();
    dist[start] = 0;
    visited.push(DijkstraState {
        cost: 0,
        position: start,
    });
    while let Some(DijkstraState { cost, position }) = visited.pop() {
        if position == goal {
            return Some(cost);
        }
        if cost > dist[position] {
            continue;
        }

        for edge in &graph[position] {
            let next = DijkstraState {
                cost: cost + edge.cost,
                position: edge.node,
            };
            if next.cost < dist[next.position] {
                visited.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let graph = vec![
            // Node 0
            vec![
                DijkstraEdge { node: 1, cost: 6 },
                DijkstraEdge { node: 2, cost: 4 },
                DijkstraEdge { node: 3, cost: 1 },
            ],
            // Node 1
            vec![
                DijkstraEdge { node: 0, cost: 6 },
                DijkstraEdge { node: 2, cost: 3 },
            ],
            // Node 2
            vec![
                DijkstraEdge { node: 0, cost: 4 },
                DijkstraEdge { node: 1, cost: 3 },
                DijkstraEdge { node: 3, cost: 1 },
            ],
            // Node 3
            vec![
                DijkstraEdge { node: 0, cost: 1 },
                DijkstraEdge { node: 2, cost: 1 },
            ],
        ];

        assert_eq!(shortest_path_dijkstra(&graph, 0, 1), Some(5));
        assert_eq!(shortest_path_dijkstra(&graph, 0, 2), Some(2));
    }
}
