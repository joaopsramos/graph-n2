use colored::Colorize;

pub struct Feedback;

impl Feedback {
    pub fn graph_file_not_found() -> String {
        format!(
            "{}",
            "Arquivo necessário para importar grafo não encontrado, encerrando...".red()
        )
    }

    pub fn clear_line() {
        print!("\u{1b}[1F");
    }

    pub fn value_read(value: &str, text: &str) -> String {
        Self::clear_line();
        format!("{text}: {}", value.magenta())
    }

    pub fn invalid_option() -> String {
        format!(
            "{}",
            "Por favor, digite uma opção válida conforme o menu.".red()
        )
    }

    pub fn load_graph_success() -> String {
        format!("{}", "Grafo carregado com sucesso!".green())
    }

    pub fn graph_exported(path: &str) -> String {
        let text = format!("{}", "O grafo foi exportado com sucesso!".green());
        format!("{text} Arquivo: {path}")
    }

    pub fn graph_not_exported() -> String {
        let executable = format!("{}", "graphviz".green());
        let link = format!("{}", "https://graphviz.org/".cyan());

        format!("{}\nVerifique se você possui o programa {executable} ({link}) instalado e se é possivel criar arquivos na pasta atual", "Erro ao exportar grafo".red())
    }
}
