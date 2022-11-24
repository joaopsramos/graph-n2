use crate::{
    feedback::Feedback,
    graph::Graph,
    graph_exporter::{self, DOT_OUTPUT},
    node::Node,
};
use colored::Colorize;
use itertools::Itertools;
use std::{env, fs, io, path::Path};
use MenuOpt::*;

type RunOptResult = Result<String, String>;

pub const FILE_PATH: &str = "./graph.json";

#[derive(PartialEq, Eq)]
pub enum MenuOpt {
    A,
    B,
    C,
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
{}) Busca em profundidade
{}) Busca em largura
{}) Algoritmo de Dijkstra
---
{}) Visualizar grafo
{}) Exportar grafo como PNG
{}) Encerrar",
        "a".magenta().bold(),
        "b".magenta().bold(),
        "c".magenta().bold(),
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

        match parse_option(option.trim()) {
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
        "a" => Some(A),
        "b" => Some(B),
        "c" => Some(C),
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
        A => depth_first_search(graph),
        B => breadth_first_search(graph),
        C => dijkstra(graph),
        Visualize => show_graph(graph),
        Export => export_graph(graph),
        _ => Ok(String::new()),
    };

    match result {
        Ok(success) => println!("{success}"),
        Err(err) => println!("{err}"),
    }
}

pub fn depth_first_search(graph: &Graph) -> RunOptResult {
    println!("{}", Feedback::initial_node());
    let initial_node = read_node(graph)?;

    let dfs_result = graph.depth_first_search(initial_node);
    let nodes = graph.get_by_codes(dfs_result);

    Ok(get_string_path(nodes))
}

pub fn breadth_first_search(graph: &Graph) -> RunOptResult {
    println!("{}", Feedback::initial_node());
    let initial_node = read_node(graph)?;

    let bfs_result = graph.breadth_first_search(initial_node);
    let nodes = graph.get_by_codes(bfs_result);

    Ok(get_string_path(nodes))
}

pub fn dijkstra(_graph: &Graph) -> RunOptResult {
    Ok("".to_string())
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

fn read_node(graph: &Graph) -> Result<&Node, String> {
    loop {
        let code = read_code();

        match graph.find_by_code(code) {
            Some(node) => break Ok(node),
            None => {
                println!("{}\n", Feedback::node_not_found());
                continue;
            }
        }
    }
}

fn read_code() -> usize {
    loop {
        println!("{}", Feedback::read_code());

        let mut code = String::new();

        io::stdin().read_line(&mut code).unwrap();
        println!("{}", Feedback::value_read(&code, "Código digitado"));

        match code.trim().parse::<usize>() {
            Ok(parsed_code) => {
                break parsed_code;
            }
            Err(_) => {
                println!("{}", Feedback::invalid_code());
                continue;
            }
        };
    }
}

fn get_string_path(nodes: Vec<&Node>) -> String {
    nodes
        .iter()
        .map(|x| format!("[{}] {}", x.code, x.name))
        .collect_vec()
        .join(" <-> ")
}
