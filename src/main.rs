use dialoguer::Select;

// use termimad;

mod header;
mod markdown_render;
mod tools;
mod web;

fn main() {
    let title = "
         ██████╗ ██████╗ ██████╗ ███████╗ ██████╗ ██╗   ██╗███████╗███████╗████████╗
        ██╔════╝██╔═══██╗██╔══██╗██╔════╝██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
        ██║     ██║   ██║██║  ██║█████╗  ██║   ██║██║   ██║█████╗  ███████╗   ██║   
        ██║     ██║   ██║██║  ██║██╔══╝  ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║   
        ╚██████╗╚██████╔╝██████╔╝███████╗╚██████╔╝╚██████╔╝███████╗███████║   ██║   
         ╚═════╝ ╚═════╝ ╚═════╝ ╚══════╝ ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   
                                                                            ";
    let phrase = "Bem vindo(a) ao CodeQuest!";

    header::print_header(&title, &phrase);

    let menu = Select::new()
        .item("Sobre o jogo")
        .item("Trilha Tools")
        .item("Trilha Web Dev")
        .item("Regras")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            termimad::print_text("**CodeQuest** é o jogo para te ensinar a **programar**\n com este ambiente de desenvolvimento,\n utilizando **terminal** e ferramentas do terminal,\n como **Tmux**, **Neovim**, **Git e GitHub**. \n");
            std::process::exit(0);
        }
        1 => {
            tools::tools();
        }
        2 => {
            web::web_dev();
        }
        3 => {
            termimad::print_text(
                "As regras são:\n- **não** use chatgpt.\n- use **apenas o que foi fornecido** a você.\n"
            );
            std::process::exit(0);
        }
        4 => {
            println!("Saindo...");
            std::process::exit(0);
        }
        _ => println!("Escolha inválida!"),
    }
}
