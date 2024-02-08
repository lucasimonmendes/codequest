use dialoguer::Select;

use crate::header::print_header;
use crate::markdown_render::markdown_render;

pub fn level_two() -> Result<(), Box<dyn std::error::Error>> {
    let title = "
        ██╗     ███████╗██╗   ██╗███████╗██╗          ██████╗ ██████╗     
        ██║     ██╔════╝██║   ██║██╔════╝██║         ██╔═████╗╚════██╗    
        ██║     █████╗  ██║   ██║█████╗  ██║         ██║██╔██║ █████╔╝    
        ██║     ██╔══╝  ╚██╗ ██╔╝██╔══╝  ██║         ████╔╝██║██╔═══╝     
        ███████╗███████╗ ╚████╔╝ ███████╗███████╗    ╚██████╔╝███████╗    
        ╚══════╝╚══════╝  ╚═══╝  ╚══════╝╚══════╝     ╚═════╝ ╚══════╝    
                                                                  
";

    let phrase = "Bem vindo(a) ao level 2!";

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
# Referências

- À partir daqui, haverá menos lições e mais desafios.
  
**Você terá de pesquisar e encontrar suas próprias fontes.**
  
## Tirando as rodinhas da bicicleta
- É a hora de começar a andar sem as rodinhas. 
- Você já caminhou bastante e aprendeu muito.
- Esse é o momento de fixar tudo o que aprendeu até aqui.

## Use essas referências para realizar os Desafios

- [Como escrever um post de blog: um passo a passo fácil de seguir](https://br.hubspot.com/blog/marketing/escrever-post-blog)
- [16 dicas de como fazer um bom currículo](https://br.hubspot.com/blog/service/como-fazer-um-bom-curriculo)
  
- [Documente seus Estudos - Isso vai ser Útil! Portfólio + Estudos + Oportunidades](https://www.dio.me/articles/documente-seus-estudos-isso-vai-ser-util-portfolio-estudos-oportunidades)
- [A documentação como método de estudo pessoal](https://edisciplinas.usp.br/pluginfile.php/5368375/mod_resource/content/1/Severino%202.2.2%20Documenta%C3%A7%C3%A3o%20como%20m%C3%A9todo%20de%20estudo%20pessoal.pdf)

**Após terminar esse level você estará apto a seguir na trilha Web Dev.**

"#;
static LESSONS: &str = r#"
# Lições 
Agora não há mais lições.
  
**Leia as referências e vá direto para os desafios.**

**Após terminar esse level você estará apto a seguir na trilha Web Dev.**
  
  
"#;

static CHALLENGES: &str = r#"
# Desafios 
  
**O SEU EDITOR DE TEXTO É O NEOVIM.**
- Utilize-o para editar todos os arquivos mencionados e também utilize o tmux para organizar seu ambiente de trabalho.
  
**Após terminar esse level você estará apto a seguir na trilha Web Dev.**
  
## O seu momento blogger
É hora de você mostrar ao mundo seus dotes de escrita!
  
**Desafio:**
- Escreva 5 posts de blog utilizando os passos da referência.
- Esses posts devem ser escritos em arquivos separados (ex. post1.md, post2.md)
- O blog será escrito em markdown.
- O blog deverá ter uma página inicial com link para os posts (dica: pesquise links em markdown).
- Os posts deverão ter link para a página inicial.
- Use o Git para registrar as alterações e faça o push para um repositório no seu github.
  
## A ToDoList que você vai cumprir
Essa é a lista de tarefas mais fácil do mundo!
  
**Desafio:**
- Escreva uma lista de tarefas que você já fez hoje.
- Organize as tarefas em Hobbies, Estudos, Social(sim, socialize!).
- Escreva em markdown, use o git e suba o repositório no github.
  
## A Wikipedia do seu cérebro
É isso mesmo que você leu!
  
**Desafio:**
- Documente tudo que você aprendeu até aqui.
- Para cada **grande tópico** utilize um arquivo em `md`.
- Escreva em markdown, use o git e suba o repositório no github.
- Escreva um arquivo chamado **Readme.md** explicando do **que se trata o projeto** para cada projeto que você **já fez e fará** nesse nível.
  
## Escrevendo seu primeiro Currículo 
Calma! Você **não vai enviar** pra ninguém.
  
**Desafio:**
- Utilizando as dicas da **referência,** escreva seu currículo em markdown.
- utilize o git e suba o repositório.
- Não se esqueça do readme desse projeto.
  
## Você precisa das suas fontes ;)
Sim, você mesmo!
  
**Desafio:**
- Escreva todas as os sites com o que você usou daquele site em um arquivo markdown.
- Utilize o Git e suba o repositório.
- Escreva o readme desse projeto.
  
  

**Após terminar esse level você estará apto a seguir na trilha Web Dev.**
  
  


"#;
