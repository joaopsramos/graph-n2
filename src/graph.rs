use crate::node::Node;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = "** Grafo **\n".blue().bold().to_string();
        let mut iter = self.nodes.iter().peekable();

        while let Some(node) = iter.next() {
            string = format!("{string}{node}\n");

            if iter.peek().is_some() {
                string = format!("{string}");
            }
        }

        if self.edges.is_empty() {
            return write!(f, "{string}");
        }

        string.push_str(&"\n** Arestas **".blue().bold().to_string());
        string = format!("{string}\n{}", format_edges(self.is_weighted, &self.edges));

        write!(f, "{string}")
    }
}

fn format_edges(weighted: bool, edges: &Vec<Edge>) -> String {
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

        string = format!("{string}");
        if iter.peek().is_some() {
            string = format!("{string}\n");
        }
    }

    format!("{string}")
}
