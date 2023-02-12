use std::collections::{HashSet, VecDeque};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    #[inline]
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Self { vertices, edges }
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
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

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

// perform a depth first search algorithm to find a element in graph
//
pub fn depth_first_search(graph: &Graph, root: Vertex, objective: Vertex) -> Option<Vec<u32>> {
    let mut visited: HashSet<Vertex> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut deque = VecDeque::new();
    deque.push_back(root);

    // while there is an element in the deque
    // get the first element of the vertex queue
    while let Some(current_vertex) = deque.pop_front() {
        history.push(current_vertex.value());

        // verify if this vertex is the objective
        if current_vertex == objective {
            return Some(history);
        }

        // for each over the neighbors of current vertex
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            // insert in the hashset of visited if this value not exists yet
            if visited.insert(neighbor) {
                // add the neighbor on front of deque
                deque.push_front(neighbor);
            }
        }
    }
    None
}
