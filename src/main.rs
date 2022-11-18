mod feedback;
mod graph;
mod graph_exporter;
mod menu;
mod node;

use crate::{feedback::Feedback, menu::MenuOpt};

fn main() {
    let mut graph = match menu::load_graph() {
        Some(graph) => graph,
        None => {
            print!("{}", Feedback::graph_file_not_found());
            return;
        }
    };

    loop {
        menu::show_menu();

        let option = menu::read_option();

        if option == MenuOpt::Exit {
            println!("\nEncerrando...");
            break;
        }

        print!("\n");

        menu::run_option(option, &mut graph);
    }
}
