//! Graph kit (MVP: adjacency list and PageRank).

use std::collections::HashMap;

/// Adjacency list graph with f64 weights.
#[derive(Debug, Default, Clone)]
pub struct Graph {
    edges: HashMap<usize, Vec<(usize, f64)>>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: f64) {
        self.edges.entry(from).or_default().push((to, weight));
    }

    pub fn pagerank(&self, iters: usize, damping: f64) -> HashMap<usize, f64> {
        let nodes: Vec<usize> = self.edges.keys().copied().collect();
        let n = nodes.len() as f64;
        let mut rank: HashMap<usize, f64> = nodes.iter().map(|&n| (n, 1.0 / n as f64)).collect();
        for _ in 0..iters {
            let mut next = HashMap::new();
            for (&node, &r) in &rank {
                let out = self.edges.get(&node).map(|v| v.as_slice()).unwrap_or(&[]);
                let sum_w: f64 = out.iter().map(|(_, w)| w).sum();
                for &(to, w) in out {
                    *next.entry(to).or_insert(0.0) += damping * r * w / sum_w.max(1e-12);
                }
                *next.entry(node).or_insert(0.0) += (1.0 - damping) / n;
            }
            rank = next;
        }
        rank
    }
}
