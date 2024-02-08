use dialoguer::Select;

use crate::header::print_header;
use crate::markdown_render::markdown_render;

pub fn level_one() -> Result<(), Box<dyn std::error::Error>> {
    let title = "
        ██╗     ███████╗██╗   ██╗███████╗██╗          ██████╗  ██╗
        ██║     ██╔════╝██║   ██║██╔════╝██║         ██╔═████╗███║
        ██║     █████╗  ██║   ██║█████╗  ██║         ██║██╔██║╚██║
        ██║     ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║         ████╔╝██║ ██║
        ███████╗███████╗ ╚████╔╝ ███████╗███████╗    ╚██████╔╝ ██║
        ╚══════╝╚══════╝  ╚═══╝  ╚══════╝╚══════╝     ╚═════╝  ╚═╝
";

    let phrase = "Bem vindo ao level 01";

    print_header(&title, &phrase);

    let menu = Select::new()
        .item("Referências")
        .item("Lições")
        .item("Desafios")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            markdown_render(REFERENCES);
        }
        1 => {
            markdown_render(LESSONS);

            // match markdown {
            //    Ok(()) => println!("Markdown renderizado com sucesso!"),
            //    Err(e) => println!("Erro ao renderizar o markdown: {}", e),
            //}
        }
        2 => {
            markdown_render(CHALLENGES);
        }
        3 => {
            println!("Saindo");
            std::process::exit(0);
        }
        _ => println!("Escolha inválida!"),
    }

    Ok(())
}

static REFERENCES: &str = r#"
# Referências Web Dev
"#;
static LESSONS: &str = r#"
# Lições Web dev
"#;
static CHALLENGES: &str = r#"
# Desafios Web dev
"#;
