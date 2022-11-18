use crate::{
    feedback::Feedback,
    graph::Graph,
    graph_exporter::{self, DOT_OUTPUT},
};
use colored::Colorize;
use std::{env, fs, io, path::Path};
use MenuOpt::*;

type RunOptResult = Result<String, String>;

pub const FILE_PATH: &str = "./graph.json";

#[derive(PartialEq)]
pub enum MenuOpt {
    No,
    Load,
    Visualize,
    Export,
    Exit,
}

pub fn show_menu() {
    println!(
        "\n{}",
        "-------------------------------------------------------------------------------".magenta()
    );
    println!("{}", "** Menu **".blue().bold());

    println!(
        "\
---
{}) Visualizar grafo
{}) Exportar grafo como PNG
{}) Encerrar",
        "v".magenta().bold(),
        "x".magenta().bold(),
        "q".magenta().bold(),
    );

    println!(
        "{}",
        "-------------------------------------------------------------------------------".magenta()
    );

    println!("{}", "Digite uma opção:".yellow());
}

pub fn read_option() -> MenuOpt {
    loop {
        let mut option = String::new();

        io::stdin().read_line(&mut option).unwrap();

        match parse_option(&option.trim()) {
            Some(opt) => {
                // Clear input
                print!("{}", Feedback::value_read(&option, "Opção digitada"));
                break opt;
            }
            None => {
                println!("{}", Feedback::invalid_option());
                continue;
            }
        };
    }
}

fn parse_option(option: &str) -> Option<MenuOpt> {
    match option {
        "n" => Some(No),
        "l" => Some(Load),
        "v" => Some(Visualize),
        "x" => Some(Export),
        "q" => Some(Exit),
        _ => None,
    }
}

pub fn run_option(option: MenuOpt, graph: &mut Graph) {
    let result = match option {
        Visualize => show_graph(graph),
        Export => export_graph(graph),
        _ => Ok(format!("")),
    };

    match result {
        Ok(success) => println!("{success}"),
        Err(err) => println!("{err}"),
    }
}

pub fn load_graph() -> Option<Graph> {
    let data = match fs::read_to_string(Path::new(FILE_PATH)) {
        Ok(data) => data,
        Err(_) => return None,
    };

    match serde_json::from_str(&data) {
        Ok(parsed_graph) => {
            println!("\n{}", Feedback::load_graph_success());
            parsed_graph
        }
        Err(_) => None,
    }
}

fn show_graph(graph: &Graph) -> RunOptResult {
    Ok(graph.to_string())
}

fn export_graph(graph: &Graph) -> RunOptResult {
    match graph_exporter::export_graph(graph) {
        Ok(_) => {
            let cwd = env::current_dir().unwrap();
            let path = Path::new(&cwd).join(DOT_OUTPUT);

            Ok(Feedback::graph_exported(path.to_str().unwrap()))
        }
        Err(_) => Err(Feedback::graph_not_exported()),
    }
}
