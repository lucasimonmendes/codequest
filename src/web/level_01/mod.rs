use crate::markdown_render::markdown_render;
use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

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

Parabéns, você acaba de chegar na trilha de **Web Dev.**
  

## É uma trilha longa
 Nesta trilha você aprenderá a criar páginas interativas para **Web**, utilizando **HTML, CSS e Javascript!**
 Sim, você utilizará **apenas** as ferramentas que estudou na trilha **Tools.**
  
  
## Sobre o Level 01
- Aqui você aprenderá o essencial sobre HTML para construir suas primeiras páginas.
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
**Detalhe:** nos vídeos ele utiliza o editor Visual Studio Code, mas você utilizará o Neovim. E também utilizará o `live-server`.

### Instalação do live-server
- Digite no terminal o comando: `sudo npm install -g live-server`

### Utilizando o live-server
- Abra a pasta do projeto em que está estudando e digite o comando: `live-server .`
- Um servidor se iniciará na porta 8080 (acesse no navegador o endereço `http://localhost:8080`) e lá você conseguirá ver as modificações que fizer no seu código em tempo real.


## Aulas 
### 1 Introduction to HTML 🌎
- [Link da aula](https://www.youtube.com/watch?v=2oCN2q1x3c4&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=1&pp=iAQB)
  

### 2 hyperlinks 👈
- [Link da aula](https://www.youtube.com/watch?v=ZOI7Tq5Zq2s&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=2&pp=iAQB)
  

### 3 images 🖼️
- [Link da aula](https://www.youtube.com/watch?v=sm5hTFzSs5Y&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=3&pp=iAQB)
  


### 4 audio 🔊
- [Link da aula](https://www.youtube.com/watch?v=uof_zYxtnp0&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=4&pp=iAQB)
  


### 5 video 🎥
- [Link da aula](https://www.youtube.com/watch?v=BAx2GaMW2qA&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=5&pp=iAQB)
  


### 6 favicons 🗿
- [Link da aula](https://www.youtube.com/watch?v=8iXHciqlAdA&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=6&pp=iAQB)
  


### 7 text formatting 💬
- [Link da aula](https://www.youtube.com/watch?v=urT4pdM3sr4&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=7&pp=iAQB)
  


### 8 span & div 🏁
- [Link da aula](https://www.youtube.com/watch?v=WbnCll6vvw4&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=8&pp=iAQB)
  


### 9 lists 📄
- [Link da aula](https://www.youtube.com/watch?v=oUm7rvMUNq8&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=9&pp=iAQB)
  


### 10 tables 📊
- [Link da aula](https://www.youtube.com/watch?v=aNC6LY34yVM&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=10&pp=iAQB)
  


### 11 buttons 🔘
- [Link da aula](https://www.youtube.com/watch?v=tDqTXipQmBU&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=11&pp=iAQB)
  


### 12 forms 📝
- [Link da aula](https://www.youtube.com/watch?v=zIN54lhJtQU&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=12&pp=iAQB)
  


### 13 headers & footers 🤯
- [Link da aula](https://www.youtube.com/watch?v=JNFdCgmMkPk&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=13&pp=iAQB)
  


"#;

    markdown_render(LESSONS);
}

fn challenges() {
    static CHALLENGES: &str = r#"
# Desafios
  
**Observações importantes:**
- Aqui você aprenderá a **colocar os elementos na tela da maneira correta.** 
- Não se preocupe em fazer eles **funcionar** ou **estilizá-los**, **este não é o foco agora.**
- Para **estilizá-los** você utilizará o **CSS**, que aprenderá no **nível 2.**
- E para fazer os elementos ficarem **interativos** você utilizará o **Javascript** que aprenderá a partir do **nível 3.**
- Sempre **documente os projetos** e utilize o **git** e os **suba para o GitHub.**
  
  
## Desafio 1: Construção de uma Página de Blog Simples
- **Aulas Envolvidas:** Introdução ao HTML, Hiperlinks, Imagens, Text Formatting, Headers & Footers.
  
**Descrição:** Você deve criar uma **página de blog simples** que inclua:
  - Um **cabeçalho** `<h1>` com o *título do blog.*
  - Um **menu de navegação** usando listas `<ul>` e `<li>`, com links `<a>` para diferentes seções do blog.
  - Uma **seção principal** com postagens de blog, cada uma contendo uma imagem `<img>`, título `<h2>`, e trecho do texto `<p>`.
  - Um **rodapé** `<footer>` com informações sobre o autor e links de contato.
  
## Desafio 2: Portfólio de Multimídia
- Aulas Envolvidas: Imagens, Audio, Vídeo, Favicons.

**Descrição:** Você deve criar uma **página de portfólio** que destaque suas habilidades multimídia. A página deve incluir:
  - Uma **galeria de imagens** de trabalhos anteriores usando tags `<img>`.
  - Um **reprodutor de áudio** `<audio>` incorporado com uma amostra de seu trabalho de áudio.
  - Um **reprodutor de vídeo** `<video>` incorporado com um clipe de seu trabalho de vídeo.
  - Um **favicon personalizado** usando a tag `<link>` dentro do elemento `<head>`.
  
## Desafio 3: Página de Perfil Social
- Aulas Envolvidas: Text Formatting, Span & Div, Lists, Buttons, Forms.

**Descrição:** Você deve criar uma **página de perfil social** fictícia. A página deve incluir:
  - Um **cabeçalho** `<h1>` com o nome do perfil.
  - Uma **seção de biografia** `<div>` com formatação de texto usando tags como `<strong>` e `<em>`.
  - **Listas de amigos** ou seguidores usando tags `<ul>` e `<li>`.
  - Um **formulário** `<form>` para atualizar informações do perfil, como status, foto de perfil etc., com diferentes tipos de campos como `<input>` e `<textarea>`.
  
  
"#;

    markdown_render(CHALLENGES);
}
