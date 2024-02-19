use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

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

    let mut main_menu: Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        MenuItem {
            label: "Level 01",
            action: MenuAction::Execute(print_level_one),
        },
    );
    main_menu.insert(
        "2",
        MenuItem {
            label: "Level 02",
            action: MenuAction::Execute(print_level_two),
        },
    );
    main_menu.insert(
        "3",
        MenuItem {
            label: "Level 03",
            action: MenuAction::Execute(print_level_three),
        },
    );
    main_menu.insert(
        "4",
        MenuItem {
            label: "Level 04",
            action: MenuAction::Execute(print_level_four),
        },
    );
    main_menu.insert(
        "5",
        MenuItem {
            label: "Sair",
            action: MenuAction::Exit,
        },
    );
    // Exibição do menu principal
    print_menu(&main_menu);
}

fn print_level_one() {
    level_one::level_one();
}
fn print_level_two() {
    level_two::level_two();
}
fn print_level_three() {
    println!("Level 03");
}
fn print_level_four() {
    println!("Level 04");
}
