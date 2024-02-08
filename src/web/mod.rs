use dialoguer::Select;

use crate::header::print_header;

mod level_01;
mod level_02;
mod level_03;
mod level_04;
mod level_05;

pub fn web_dev() {
    let title = "
        ██╗    ██╗███████╗██████╗     ██████╗ ███████╗██╗   ██╗
        ██║    ██║██╔════╝██╔══██╗    ██╔══██╗██╔════╝██║   ██║
        ██║ █╗ ██║█████╗  ██████╔╝    ██║  ██║█████╗  ██║   ██║
        ██║███╗██║██╔══╝  ██╔══██╗    ██║  ██║██╔══╝  ╚██╗ ██╔╝
        ╚███╔███╔╝███████╗██████╔╝    ██████╔╝███████╗ ╚████╔╝ 
         ╚══╝╚══╝ ╚══════╝╚═════╝     ╚═════╝ ╚══════╝  ╚═══╝  
                                                       
";
    let phrase = "Escolha o seu nível:";

    print_header(&title, &phrase);

    let menu = Select::new()
        .item("Level 01")
        .item("Level 02")
        .item("Level 03")
        .item("Level 04")
        .item("Level 05")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            level_01::level_one();
        }
        1 => {
            level_02::level_two();
        }
        2 => {
            level_03::level_three();
        }
        3 => {
            level_04::level_four();
        }
        4 => {
            level_05::level_five();
        }
        5 => {
            println!("Saindo...");
            std::process::exit(0);
        }
        _ => println!("Escolha inválida"),
    }
}
