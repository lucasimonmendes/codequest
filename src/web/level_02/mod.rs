use crate::markdown_render::markdown_render;
use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

pub fn level_two() -> Result<(), Box<dyn std::error::Error>> {
    let title = "
        ██╗     ███████╗██╗   ██╗███████╗██╗          ██████╗ ██████╗ 
        ██║     ██╔════╝██║   ██║██╔════╝██║         ██╔═████╗╚════██╗
        ██║     █████╗  ██║   ██║█████╗  ██║         ██║██╔██║ █████╔╝
        ██║     ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║         ████╔╝██║██╔═══╝ 
        ███████╗███████╗ ╚████╔╝ ███████╗███████╗    ╚██████╔╝███████╗
        ╚══════╝╚══════╝  ╚═══╝  ╚══════╝╚══════╝     ╚═════╝ ╚══════╝
                                                              
";

    let phrase = "Bem vindo ao level 02";

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
  
  
## Sobre o Level 02
- Aqui você aprenderá o essencial sobre CSS para estilizar suas primeiras páginas.
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
### 14 Introduction to CSS 🎨
- [Link da aula](https://www.youtube.com/watch?v=uzF1Doc3cZQ&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=14&pp=iAQB)
  
### 15 colors 🖌️
- [Link da aula](https://www.youtube.com/watch?v=LwbKb2J8iy8&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=15&pp=iAQB)
  

### 16 fonts 🔤
- [Link da aula](https://www.youtube.com/watch?v=MnyCbILmtOY&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=16&pp=iAQB)
  

### 17 borders 🖼
- [Link da aula](https://www.youtube.com/watch?v=pkNdQ7TmxIw&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=17&pp=iAQB)
  

### 18 shadows 👥
- [Link da aula](https://www.youtube.com/watch?v=Yqs_61ub1Ng&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=18&pp=iAQB)
  

### 19 margins ↔️
- [Link da aula](https://www.youtube.com/watch?v=rBWA_t-KnKk&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=19&pp=iAQB)
  

### 20 float 🎈
- [Link da aula](https://www.youtube.com/watch?v=oJe8G5XT_v4&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=20&pp=iAQB)
  

### 21 overflow 🌊
- [Link da aula](https://www.youtube.com/watch?v=d7cH8geV2dY&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=21&pp=iAQB)
  

### 22 display property 🧱
- [Link da aula](https://www.youtube.com/watch?v=9T8uxp5hQ60&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=22&pp=iAQB)
  

### 23 height and width 📏
- [Link da aula](https://www.youtube.com/watch?v=QctF-i4-GuM&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=23&pp=iAQB)
  

### 24 positions 🎯
- [Link da aula](https://www.youtube.com/watch?v=G4AWXOr4W-k&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=24&pp=iAQB)
  

### 25 background images 🏙️
- [Link da aula](https://www.youtube.com/watch?v=_oFWg_NlKdo&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=25&pp=iAQB)
  

### 26 combinators ➕
- [Link da aula](https://www.youtube.com/watch?v=swZFmJsU54s&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=26&pp=iAQB)
  

### 27 pseudo-classes ☟
- [Link da aula](https://www.youtube.com/watch?v=Nrsy_2ogRfQ&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=27&pp=iAQB)
  

### 28 pseudo-elements ✔
- [Link da aula](https://www.youtube.com/watch?v=_LxYNxeWpBo&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=28&pp=iAQB)
  

### 29 pagination 🕮
- [Link da aula](https://www.youtube.com/watch?v=6ff96BkWqmg&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=29&pp=iAQB)
  

### 30 dropdown menus 🔻
- [Link da aula](https://www.youtube.com/watch?v=VQWu4e6agPc&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=30&pp=iAQB)
  

### 31 navigation bar 🧭
- [Link da aula](https://www.youtube.com/watch?v=f3uCSh6LIY0&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=31&pp=iAQB)
  

### 32 website layout 🗺️
- [Link da aula](https://www.youtube.com/watch?v=Hsu8uqQTSV8&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=32&pp=iAQB)
  

### 33 image gallery 📷
- [Link da aula](https://www.youtube.com/watch?v=uu0lOX6Ot3s&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=33&pp=iAQB)
  

### 34 icons 🐤
- [Link da aula](https://www.youtube.com/watch?v=k3AJx11k9QY&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=34&pp=iAQB)
  

### 35 flexbox 💪
- [Link da aula](https://www.youtube.com/watch?v=GteJWhCikCk&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=35&pp=iAQB)
  

### 36 transformations 🔄
- [Link da aula](https://www.youtube.com/watch?v=qdeIy9_fbxE&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=36&pp=iAQB)
  

### 37 animations 🎬
- [Link da aula](https://www.youtube.com/watch?v=u_GIT5MJAtc&list=PLZPZq0r_RZOOxqHgOzPyCzIl4AJjXbCYt&index=37&pp=iAQB)
  
  
  
"#;

    markdown_render(LESSONS);
}

fn challenges() {
    static CHALLENGES: &str = r#"
# Desafios
  
**Observações importantes:**
- Aqui você aprenderá a **estilizar os elementos que colocou na tela.** 
- Não se preocupe em fazer eles **funcionar**, **este não é o foco agora.**
- Para **estilizá-los** você utilizará o **CSS** que aprendeu com as aulas.
- E para fazer os elementos ficarem **interativos** você utilizará o **Javascript** que aprenderá a partir do **nível 3.**
- Sempre **documente os projetos** e utilize o **git** e os **suba para o GitHub.**
   
   
## Desafio 1: Estilizando uma Página Básica
- **Aulas Envolvidas:** Introduction to CSS, Colors, Fonts.
  
**Descrição:** Você deve criar um arquivo CSS separado e estilizar uma página HTML básica, incluindo:
  - Alterar as **cores de fundo** e **texto.**
  - Experimentar **diferentes tipos de fontes** para o texto.
  - Ajustar o **espaçamento** entre linhas e parágrafos.
  
## Desafio 2: Adicionando Bordas e Sombras
- **Aulas Envolvidas:** Borders, Shadows.
  
**Descrição:** Você deve praticar a adição de bordas e sombras a elementos HTML, incluindo:
  - Adicionar **bordas** a diferentes elementos, como caixas de texto e imagens.
  - Experimentar diferentes tipos e tamanhos de **sombras** para os elementos.
  
## Desafio 3: Posicionamento e Layout
- **Aulas Envolvidas:** Positions, Website Layout.
  
**Descrição:** Você deve criar um layout de página usando técnicas de posicionamento CSS, incluindo:
  - Posicionar elementos **absolutamente** e **relativamente.**
  - Criar um **layout de grade(grid)** usando **flexbox** ou **floats.**
  
## Desafio 4: Galeria de Imagens com Flexbox
- **Aulas Envolvidas:** Flexbox, Image Gallery.
  
**Descrição:** Você deve criar uma galeria de imagens responsiva usando flexbox, incluindo:
  - Organizar **imagens** em várias **linhas e colunas.**
  - Garantir que a galeria se **ajuste dinamicamente ao tamanho da tela.**
  
## Desafio 5: Animações e Transformações
- **Aulas Envolvidas:** Transformations, Animations.
  
**Descrição:** Você deve criar elementos HTML animados usando transformações e animações CSS, incluindo:
  - **Girar ou escalar** elementos quando `hover`.
  - Criar **transições suaves** entre diferentes estados de um elemento.
  
## Desafio 6: Menus de Navegação e Ícones
- **Aulas Envolvidas:** Dropdown Menus, Navigation Bar, Icons.
  
**Descrição:** Você deve criar um menu de navegação com ícones e opções de menu suspensos, incluindo:
  - Estilizar os itens do menu e adicionar **ícones** para cada opção.
  - Implementar um **menu suspenso** que apareça quando o usuário passa o mouse sobre um item de menu.
  


"#;

    markdown_render(CHALLENGES);
}
