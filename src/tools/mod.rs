use dialoguer::Select;

use crate::header::print_header;

mod level_one;
mod level_two;

pub fn tools() {
    let title = "
        ████████╗ ██████╗  ██████╗ ██╗     ███████╗
        ╚══██╔══╝██╔═══██╗██╔═══██╗██║     ██╔════╝
           ██║   ██║   ██║██║   ██║██║     ███████╗
           ██║   ██║   ██║██║   ██║██║     ╚════██║
           ██║   ╚██████╔╝╚██████╔╝███████╗███████║
           ╚═╝    ╚═════╝  ╚═════╝ ╚══════╝╚══════╝
";
    let phrase = "Escolha o seu nível:";

    print_header(&title, &phrase);

    let menu = Select::new()
        .item("Level 01")
        .item("Level 02")
        .item("Level 03")
        .item("Level 04")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            level_one::level_one();
        }
        1 => {
            level_two::level_two();
        }
        2 => {
            println!("Level 3");
        }
        3 => {
            println!("Level 4");
        }
        4 => {
            println!("Saindo...");
            std::process::exit(0);
        }
        _ => println!("Escolha inválida"),
    }
}
