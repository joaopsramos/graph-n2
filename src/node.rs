use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Hash, Eq)]
pub struct Node {
    pub code: usize,
    pub name: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Código: {}, Nome: {}{}",
            "|".green(),
            self.code.to_string().cyan(),
            self.name.cyan(),
            "|".green()
        )
    }
}
