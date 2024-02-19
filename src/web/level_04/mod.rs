use crate::markdown_render::markdown_render;
use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

pub fn level_four() -> Result<(), Box<dyn std::error::Error>> {
    let title = "
        ██╗     ███████╗██╗   ██╗███████╗██╗          ██████╗ ██╗  ██╗
        ██║     ██╔════╝██║   ██║██╔════╝██║         ██╔═████╗██║  ██║
        ██║     █████╗  ██║   ██║█████╗  ██║         ██║██╔██║███████║
        ██║     ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║         ████╔╝██║╚════██║
        ███████╗███████╗ ╚████╔╝ ███████╗███████╗    ╚██████╔╝     ██║
        ╚══════╝╚══════╝  ╚═══╝  ╚══════╝╚══════╝     ╚═════╝      ╚═╝
                                                              
";

    let phrase = "Bem vindo ao level 04";

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
  
  
## Sobre o Level 04
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
### 8 Math object 🧮
- [Link da aula](https://www.youtube.com/watch?v=uy-1WNqecnI&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=8&pp=iAQB)
  

### 9 Random number generator ⁉
- [Link da aula](https://www.youtube.com/watch?v=K2upGO5Bb48&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=9&pp=iAQB)
  

### 10 If statements 🤔
- [Link da aula](https://www.youtube.com/watch?v=PgUXiprlg1k&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=10&pp=iAQB)
  

### 11 Checked property ✅
- [Link da aula](https://www.youtube.com/watch?v=SgxzJdqhyfw&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=11&pp=iAQB)
  

### 12 Ternary operator ❓
- [Link da aula](https://www.youtube.com/watch?v=atS_A9HHAVo&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=12&pp=iAQB)
  

### 13 Switches 💡
- [Link da aula](https://www.youtube.com/watch?v=z2fcWdoph4U&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=13&pp=iAQB)
  

### 14 String methods 🧵
- [Link da aula](https://www.youtube.com/watch?v=wssvLtVSFeI&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=14&pp=iAQB)
  

### 15 String slicing ✂️
- [Link da aula](https://www.youtube.com/watch?v=sPPGd4Lfh3s&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=15&pp=iAQB)
  

### 16 Method chaining ⛓
- [Link da aula](https://www.youtube.com/watch?v=J4YhlDsNqeE&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=16&pp=iAQB)
  

### 17 Logical operators ❗
- [Link da aula](https://www.youtube.com/watch?v=ovWYhDVQiR8&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=17&pp=iAQB)
  

### 18 Strict equality 🟰
- [Link da aula](https://www.youtube.com/watch?v=O7aUm0AuUy4&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=18&pp=iAQB)
  

### 19 While loops 🔁
- [Link da aula](https://www.youtube.com/watch?v=TDUz9QcGPoE&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=19&pp=iAQB)
  

### 20 For loops 🔂
- [Link da aula](https://www.youtube.com/watch?v=ZOQYIWLngSU&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=20&pp=iAQB)
  

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
   
   

## 21 ⭐ Number guessing game ↕
- [Link da aula](https://www.youtube.com/watch?v=maB0r59KOUk&list=PLZPZq0r_RZOO1zkgO4bIdfuLpizCeHYKv&index=21&pp=iAQB)
- Acompanhe o projeto e programe ele em **português** com as devidas traduções.
- Lembrando que o **código fonte deve ser em inglês**, mas as frases e textos que o usuário verá devem ser em **português.**
  

  


"#;

    markdown_render(CHALLENGES);
}
