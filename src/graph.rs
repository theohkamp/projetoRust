use std::collections::{HashMap, HashSet, VecDeque};

pub struct Graph {
    pub adjacency: HashMap<u32, Vec<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, a: u32, b: u32) {
        if a == b {
            return;
        }

        let neighbors_a = self.adjacency.entry(a).or_default();
        if !neighbors_a.contains(&b) {
            neighbors_a.push(b);
        }

        let neighbors_b = self.adjacency.entry(b).or_default();
        if !neighbors_b.contains(&a) {
            neighbors_b.push(a);
        }
    }

    pub fn bfs(&self, start: u32) -> Vec<u32> {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut order: Vec<u32> = Vec::new();
        let mut queue: VecDeque<u32> = VecDeque::new();

        queue.push_back(start);
        visited.insert(start);

        while let Some(node) = queue.pop_front() {
            order.push(node);

            if let Some(neighbors) = self.adjacency.get(&node) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back(neighbor);
                    }
                }
            }
        }

        order
    }
}