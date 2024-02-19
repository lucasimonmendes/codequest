use std::collections::BTreeMap;

mod markdown_render;
mod tools;
mod ui;
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

    ui::print_header(&title, &phrase);

    // Definição do menu principal
    let mut main_menu: ui::Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        ui::MenuItem {
            label: "Sobre o Jogo",
            action: ui::MenuAction::Execute(about),
        },
    );
    main_menu.insert(
        "2",
        ui::MenuItem {
            label: "Trilhas",
            action: ui::MenuAction::Submenu(trails()),
        },
    );
    main_menu.insert(
        "3",
        ui::MenuItem {
            label: "Regras",
            action: ui::MenuAction::Execute(rules),
        },
    );
    main_menu.insert(
        "4",
        ui::MenuItem {
            label: "Sair",
            action: ui::MenuAction::Exit,
        },
    );
    // Exibição do menu principal
    ui::print_menu(&main_menu);
}

fn about() {
    termimad::print_text("**CodeQuest** é o jogo para te ensinar a **programar**\n com este ambiente de desenvolvimento,\n utilizando **terminal** e ferramentas do terminal,\n como **Tmux**, **Neovim**, **Git e GitHub**. \n");
    std::process::exit(0);
}

fn rules() {
    termimad::print_text(
        "As regras são:\n- **não** use chatgpt.\n- use **apenas o que foi fornecido** a você.\n",
    );
    std::process::exit(0);
}

fn trails() -> ui::Menu {
    let mut submenu: ui::Menu = BTreeMap::new();
    submenu.insert(
        "1",
        ui::MenuItem {
            label: "Trilha Tools",
            action: ui::MenuAction::Execute(tools::tools),
        },
    );
    submenu.insert(
        "2",
        ui::MenuItem {
            label: "Trilha WebDev",
            action: ui::MenuAction::Execute(web::web_dev),
        },
    );
    submenu.insert(
        "3",
        ui::MenuItem {
            label: "Voltar",
            action: ui::MenuAction::Exit,
        },
    );
    submenu
}
