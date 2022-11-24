use crate::node::Node;
use colored::Colorize;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, fmt::Display};

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

    pub fn get_by_codes(&self, codes: Vec<usize>) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter(|node| codes.contains(&node.code))
            .collect()
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

    pub fn depth_first_search(&self, initial_node: &Node) -> Vec<usize> {
        let mut stack = vec![initial_node];
        let mut history = vec![initial_node];
        let mut visited = vec![initial_node];

        while let Some(current_node) = stack.pop() {
            history.push(current_node);

            for neighbor in self.neighbors(current_node).iter().rev() {
                if !visited.contains(neighbor) {
                    visited.push(neighbor);
                    stack.push(neighbor);
                }
            }
        }

        history.iter().unique().map(|n| n.code).collect()
    }

    pub fn breadth_first_search(&self, initial_node: &Node) -> Vec<usize> {
        let mut queue = VecDeque::from([initial_node]);
        let mut visited = vec![initial_node];

        while let Some(current_node) = queue.pop_front() {
            let neighbors = self.neighbors(current_node);

            for neighbor in neighbors.iter() {
                if !visited.contains(neighbor) {
                    visited.push(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        visited.iter().map(|n| n.code).collect()
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
