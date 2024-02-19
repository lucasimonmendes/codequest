use crate::markdown_render::markdown_render;
use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

pub fn level_three() -> Result<(), Box<dyn std::error::Error>> {
    let title = "
        ██╗     ███████╗██╗   ██╗███████╗██╗          ██████╗ ██████╗ 
        ██║     ██╔════╝██║   ██║██╔════╝██║         ██╔═████╗╚════██╗
        ██║     █████╗  ██║   ██║█████╗  ██║         ██║██╔██║ █████╔╝
        ██║     ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║         ████╔╝██║ ╚═══██╗
        ███████╗███████╗ ╚████╔╝ ███████╗███████╗    ╚██████╔╝██████╔╝
        ╚══════╝╚══════╝  ╚═══╝  ╚══════╝╚══════╝     ╚═════╝ ╚═════╝ 
                                                              
";

    let phrase = "Bem vindo ao level 03";

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

    Ok(())
}

fn references() {
    static REFERENCES: &str = r#"
# Referências
  
## É uma trilha longa
 Nesta trilha você aprenderá a criar páginas interativas para **Web**, utilizando **HTML, CSS e Javascript!**
 Sim, você utilizará **apenas** as ferramentas que estudou na trilha **Tools.**
  
  
## Sobre o Level 03
- Aqui você aprenderá o básico sobre Javascript e criará seu primeiro projeto interativo.
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
### 1 JavaScript tutorial for beginners 🌐
- [Link da aula](https://www.youtube.com/watch?v=Ihy0QziLDf0&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=1&pp=iAQB)
  
### 2 Variables 📦
- [Link da aula](https://www.youtube.com/watch?v=nbX0MIV7-Ek&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=2&pp=iAQB)
  

### 3 Arithmetic operators ➕
- [Link da aula](https://www.youtube.com/watch?v=FyGIKD2fxIo&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=3&pp=iAQB)
  

### 4 Accept user input 💬
- [Link da aula](https://www.youtube.com/watch?v=JeXqaKeJSRI&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=4&pp=iAQB)
  

### 5 Type conversion 💱
- [Link da aula](https://www.youtube.com/watch?v=jLRnuVHwHKk&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=5&pp=iAQB)
  

### 6 Constants 🚫
- [Link da aula](https://www.youtube.com/watch?v=3M53uhj0D4k&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=6&pp=iAQB)
  


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
   
   


## 7 ⭐ Counter program 🔢
- [Link da aula](https://www.youtube.com/watch?v=uSJXZ3LkABE&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=7&pp=iAQB)
- Acompanhe o projeto e programe ele em português com as devidas traduções.
- Lembrando que o código fonte deve ser em inglês, mas as frases e textos que o usuário verá devem ser em português.
  


 
"#;

    markdown_render(CHALLENGES);
}
