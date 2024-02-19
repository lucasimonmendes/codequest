use crate::markdown_render::markdown_render;
use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

pub fn level_five() -> Result<(), Box<dyn std::error::Error>> {
    let title = "
        ██╗     ███████╗██╗   ██╗███████╗██╗          ██████╗ ███████╗
        ██║     ██╔════╝██║   ██║██╔════╝██║         ██╔═████╗██╔════╝
        ██║     █████╗  ██║   ██║█████╗  ██║         ██║██╔██║███████╗
        ██║     ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║         ████╔╝██║╚════██║
        ███████╗███████╗ ╚████╔╝ ███████╗███████╗    ╚██████╔╝███████║
        ╚══════╝╚══════╝  ╚═══╝  ╚══════╝╚══════╝     ╚═════╝ ╚══════╝
                                                              
";

    let phrase = "Bem vindo ao level 05";

    print_header(&title, &phrase);

    let mut main_menu: Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        MenuItem {
            label: "Referências",
            action: MenuAction::Execute(references),
        },
    );
    main_menu.insert(
        "2",
        MenuItem {
            label: "Lições",
            action: MenuAction::Execute(lessons),
        },
    );
    main_menu.insert(
        "3",
        MenuItem {
            label: "Desafios",
            action: MenuAction::Execute(challenges),
        },
    );
    main_menu.insert(
        "4",
        MenuItem {
            label: "Sair",
            action: MenuAction::Exit,
        },
    );
    // Exibição do menu principal
    print_menu(&main_menu);

    /*
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
    */
    Ok(())
}

fn references() {
    static REFERENCES: &str = r#"
## Referências
  
## É uma trilha longa
 Nesta trilha você aprenderá a criar páginas interativas para **Web**, utilizando **HTML, CSS e Javascript!**
 Sim, você utilizará **apenas** as ferramentas que estudou na trilha **Tools.**
  
  
## Sobre o Level 05
- Aqui você aprenderá mais detalhes importantes sobre Javascript e criará um novo projeto interativo.
- Não se esqueça de utilizar o **Git** e o **Tmux** para aumentar a sua produtividade.
- E, é claro, **sempre documente** seus projetos práticos e seus estudos como fazia na trilha Tools. Pode utilizar o Markdown mesmo.
- Não desista e sempre pratique que você chegará lá.
  
  
"#;

    markdown_render(REFERENCES);
}

fn lessons() {
    static LESSONS: &str = r#"
# Lições 
  
Aqui estão as aulas para você estudar.
  
## Sobre as ferramentas
**Detalhe:** nos vídeos ele utiliza o editor **Visual Studio Code**, mas você utilizará o **Neovim.**  
E também utilizará o `live-server`.
  
### Instalação do live-server
- Digite no terminal o comando: `sudo npm install -g live-server`
  
### Utilizando o live-server
- Abra a pasta do projeto em que está estudando e digite o comando: `live-server .`
- Um servidor se iniciará na porta 8080 (acesse no navegador o endereço `http://localhost:8080`) e lá você conseguirá ver as modificações que fizer no seu código em tempo real.
  
## Aulas 
### 22 Functions 📞
- [Link da aula](https://www.youtube.com/watch?v=HFaxylC7bUc&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=22&pp=iAQB)
  

### 23 Variable scope 🏠
- [Link da aula](https://www.youtube.com/watch?v=KyqmbIkZGIo&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=23&pp=iAQB)
  

"#;

    markdown_render(LESSONS);
}

fn challenges() {
    static CHALLENGES: &str = r#"
# Desafio
  
**Observações importantes:**
- Aqui você aprenderá a **dinamizar os elementos que colocou na tela.**  
- Para fazer os elementos ficarem **interativos** você utilizará o **Javascript** que aprendeu nas aulas.
- Sempre **documente os projetos** e utilize o **git** e os **suba para o GitHub.**
   
   

### 24 ⭐ Temperature conversion program 🌡️
- [Link da aula](https://www.youtube.com/watch?v=6xrTdpIAsb0&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=24&pp=iAQB)
- Acompanhe o projeto e programe ele em **português** com as devidas traduções.
- Lembrando que o **código fonte deve ser em inglês**, mas as frases e textos que o usuário verá devem ser em **português.**
  
 
 


"#;

    markdown_render(CHALLENGES);
}
