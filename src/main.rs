use dialoguer::Select;

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
            println!("Codequest é o jogo para te ensinar a programar com este ambiente de desenvolvimento, utilizando terminal e ferramentas javascript.");
            std::process::exit(0);
        }
        1 => {
            tools::tools();
        }
        2 => {
            web::web_dev();
        }
        3 => {
            println!(
                "As regras são:\n- não use chatgpt.\n- use apenas o que for fornecido a você.\n"
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
