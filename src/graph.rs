use crate::node::Node;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Edge {
    pub from: usize,
    pub to: usize,
    pub weight: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
    pub is_weighted: bool,
    pub size: usize,
    pub edges: Vec<Edge>,
    pub nodes: Vec<Node>,
}

impl Graph {
    pub fn find_by_code(&self, code: usize) -> Option<&Node> {
        self.nodes.iter().find(|node| node.code == code)
    }

    fn neighbors(&self, node: &Node) -> Vec<&Node> {
        self.edges
            .iter()
            .filter_map(|edge| {
                if edge.from == node.code {
                    return Some(self.find_by_code(edge.to).unwrap());
                }

                None
            })
            .collect()
    }

    fn adjacency_edges(&self, node: &Node) -> Vec<&Edge> {
        self.edges
            .iter()
            .filter(|edge| edge.from == node.code)
            .collect()
    }

    pub fn depth_first_search(&self, initial_node: &Node) -> Vec<usize> {
        let mut stack = vec![initial_node];
        let mut visited = HashSet::from([initial_node]);
        let mut history = Vec::new();

        while let Some(current_node) = stack.pop() {
            history.push(current_node.code);

            for neighbor in self.neighbors(current_node).iter().rev() {
                if visited.insert(neighbor) {
                    stack.push(neighbor);
                }
            }
        }

        history
    }

    pub fn breadth_first_search(&self, initial_node: &Node) -> Vec<usize> {
        let mut queue = VecDeque::from([initial_node]);
        let mut path = vec![initial_node];

        while let Some(current_node) = queue.pop_front() {
            for neighbor in self.neighbors(current_node) {
                if !path.contains(&neighbor) {
                    path.push(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        path.iter().map(|n| n.code).collect()
    }

    pub fn dijkstra<'a>(&'a self, initial_node: &'a Node) -> HashMap<&Node, u32> {
        let mut distances = HashMap::new();
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();

        distances.insert(initial_node, 0);
        to_visit.push((initial_node, 0));

        while let Some((current_node, distance)) = to_visit.pop() {
            if !visited.insert(current_node) {
                continue;
            }

            for edge in self.adjacency_edges(current_node) {
                let neighbor = self.find_by_code(edge.to).unwrap();
                let new_distance = distance + edge.weight;
                let is_shorter = distances
                    .get(neighbor)
                    .map_or(true, |&current| new_distance < current);

                if is_shorter {
                    distances.insert(neighbor, new_distance);
                    to_visit.push((neighbor, new_distance))
                }
            }
        }

        distances
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = "** Grafo **\n".blue().bold().to_string();
        let iter = self.nodes.iter();

        for node in iter {
            string = format!("{string}{node}\n");
        }

        if self.edges.is_empty() {
            return write!(f, "{string}");
        }

        string.push_str(&"\n** Arestas **".blue().bold());
        string = format!("{string}\n{}", format_edges(self.is_weighted, &self.edges));

        write!(f, "{string}")
    }
}

fn format_edges(weighted: bool, edges: &[Edge]) -> String {
    let mut string = "".to_string();
    let mut iter = edges.iter().peekable();

    while let Some(edge) = iter.next() {
        string = format!(
            "{string}{} <-> {}",
            edge.from.to_string().cyan(),
            edge.to.to_string().cyan()
        );

        if weighted {
            string = format!("{string}  Peso = {}", edge.weight.to_string().cyan());
        }

        if iter.peek().is_some() {
            string.push('\n');
        }
    }

    string
}
