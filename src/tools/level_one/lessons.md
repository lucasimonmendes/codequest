# Level 01 - Introdução às ferramentas de Desenvolvimento

- **Lições:**

  - Git: Configuração básica do Git.
    Comandos básicos como git init, git add, git commit e git status.
  - Neovim: Navegação básica no editor, edição de arquivos e saída.
  - Tmux: Navegação básica entre sessões, janelas e painéis.

- **Desafios:**
  - Criar um repositório Git local para um projeto simples.
  - Editar e salvar um arquivo usando o Neovim.
  - Criar uma sessão Tmux, abrir várias janelas e dividir os painéis.

## Referências

Em caso de dúvida ou erros, consulte um desses sites com as buscas por:

- `tmux`
- `neovim`
- `git`

### Sites

- [Dev.to](https://dev.to/)
- [Tabnews](https://www.tabnews.com.br/)
- [Youtube](https://www.youtube.com/)

## Lições

### Lição 01 - Tmux (seu organizador de telinhas)

tmux é um multiplexador de terminal. Ele permite alternar facilmente entre vários programas em um terminal, desconectá-los (eles continuam sendo executados em segundo plano) e reconectá-los a um terminal diferente.

Tmux para iniciantes - [artigo explicativo.](https://dev.to/collabcode/tmux-para-iniciantes-4kg8)
**PS:** tmux já está instalado na máquina.

### Lição 02 - Neovim (seu editor de texto)

Sobre o Neovim - [artigo explicativo.](https://www.tabnews.com.br/NathanFirmo/por-que-voce-deveria-usar-neovim-para-programar)

### Lição 03: Git (seu amigo elefante com memória)

O Git é a ferramenta utilizada para registrar as alterações do código e
das versões dos seus projetos.

Você pode utilizá-lo com os seguintes comandos:

- `git init` (inicia o git no projeto)
- `git status` (inidica o status dos arquivos do projeto)
- `git add` (adiciona os arquivos alterados para o stage)
- `git commit -m "adicionando feature email"` (faz o commit dos arquivos que estão
  onstagged com a mensagem "adicionando feature email")
- `git push` (sobe os commits para o repositório remoto(GitHub))

Aqui está a documentação do [Git.](https://git-scm.com/doc)

Você irá utilizar o **Git** sempre!
É bom memorizar alguns comandos dele.

**PS:** Leia a documentação e se ainda estiver com dúvida, veja este [vídeo.](https://www.youtube.com/watch?v=ts-H3W1uLMM&ab_channel=C%C3%B3digoFonteTV)

## **Desafios:**

### TMUX

#### Divisão Horizontal e Vertical

- Abra o **Tmux** no terminal.
- Divida a tela **horizontalmente em duas partes.**
- Em seguida, divida uma dessas partes **verticalmente em duas partes.**
- **Navegue entre as diferentes áreas** e experimente **diferentes comandos para alternar entre elas.**

#### Navegação entre Painéis

- Abra o Tmux no terminal e **divida a tela em pelo menos três painéis.**
- Use atalhos de teclado para navegar entre os painéis **sem usar o mouse.**
- Pratique **redimensionar** os painéis e _mover o foco_ entre eles.

#### Execução de Comandos em Múltiplos Painéis

- Divida a tela em pelo menos **quatro painéis.**
- Em cada painel, execute um **comando diferente**, como visualizar um arquivo de texto, rodar um comando de linha de comando ou abrir um editor de texto.
- Pratique **alternar entre os painéis** e **observar a saída** dos comandos.

#### Organização de Layouts Personalizados

- Experimente **criar layouts personalizados** dividindo a tela de maneiras diferentes, como em linhas, colunas ou células irregulares.
- Salve os layouts personalizados para uso futuro.
- Pratique **alternar entre os layouts e adaptá-los conforme necessário.**

#### Uso de Janelas e Sessões

- Experimente criar várias janelas dentro de uma sessão Tmux.
- Pratique alternar entre diferentes janelas e sessões.
- Explore **como criar novas sessões e se conectar a sessões existentes.**

### Neovim

#### Navegação Básica

- Abra um arquivo de texto no **Neovim**.
- Pratique navegar pelo arquivo usando as **teclas de seta, h, j, k e l.**
- Experimente rolar para cima e para baixo no arquivo usando as teclas **Ctrl + u e Ctrl + d**.

#### Edição de Texto:

- Abra um arquivo de texto no **Neovim**.
- Pratique a **inserção e exclusão de texto** em diferentes partes do arquivo.
- Experimente usar comandos de **edição, como copiar, colar e desfazer.**

#### Busca e Substituição:

- Abra um arquivo de texto no **Neovim**.
- Pratique buscar por palavras específicas no arquivo usando `/ seguido da palavra desejada`.
- Experimente substituir palavras específicas usando `:%s/palavra_antiga/palavra_nova/g`.

### GIT e GitHub

#### Criar um novo repositório no GitHub

- Crie uma conta no GitHub, se ainda não tiver.
- Crie um novo repositório em sua conta.
- Clone o repositório para sua máquina local usando o comando `git clone`.
- Crie um arquivo **README.md** no repositório e adicione um _breve texto_ usando [Markdown](https://blog.da2k.com.br/2015/02/08/aprenda-markdown/) para descrever o projeto.

#### Adicionar e commitar alterações

- Edite o **README.md** para adicionar uma _breve descrição de si mesma_ usando **Markdown**.
- Adicione o arquivo **README.md** ao **stage** usando `git add`.
- Commit as alterações usando `git commit -m "Adicionando descrição no README.md"`.

#### Criar e alternar entre branches

- Crie uma nova branch chamado **feature/novo-conteudo** usando `git branch` ou `git checkout -b`.
- Edite o **README.md** para adicionar mais detalhes sobre seus _hobbies ou interesses_.
- Adicione e commite as alterações na nova branch.

#### Fazer o merge de branches

- Volte para a branch principal usando `git checkout`.
- Faça o merge do branch **feature/novo-conteudo** para o branch principal usando `git merge`.
- Resolva quaisquer conflitos que possam surgir.

#### Fazer o push das alterações para o GitHub

- Faça o push das branches atualizados para o GitHub usando `git push`.
- Verifique se as alterações estão refletidas no **repositório remoto** no GitHub.
