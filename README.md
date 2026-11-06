# Faculdade e Estuda UVA

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho ainda não commitada.

Este repositório reúne duas coisas ligadas entre si:

- **Faculdade**: um acervo acadêmico pessoal em texto (Markdown e JSON) com matérias, períodos, currículos e textos extraídos de materiais oficiais da UVA.
- **Estuda UVA**: um aplicativo desktop local, escrito em Rust, que lê esse acervo, guarda estado em SQLite e consulta Canvas e Portal do Aluno em modo de leitura.

Esta página resume o projeto. O detalhe técnico está em [docs/README.md](docs/README.md).

## Como ler os rótulos de estado

Em toda a documentação, cada recurso tem um destes rótulos:

| Rótulo | Significado |
|---|---|
| **Implementado** | Existe no código e foi conferido nesta revisão. |
| **Parcial** | Existe, mas com limite relevante descrito no texto. |
| **Planejado** | Só existe como plano ou contrato proposto. Não está no código. |
| **Não verificado** | Existe no código, mas não foi executado nesta revisão (por exemplo, fluxos que exigem login). |

## Objetivos

1. Manter o material de estudo acessível sem login no Canvas, em texto que o Git consegue versionar e comparar.
2. Separar o que é matéria e instituição do que é dado pessoal do aluno.
3. Diferenciar dado confirmado em fonte oficial de estimativa, material didático e campo desconhecido, sempre com fonte e data.
4. Consultar Canvas e Portal sem alterar nada na instituição.

## O que está implementado

- **Acervo local e busca.** Varredura de `materias/`, `fontes-uva/`, `periodos/`, `docs/` e outras pastas listadas em [docs/arquitetura/crates.md](docs/arquitetura/crates.md), com leitura de Markdown, texto, JSON, HTML, PDF, DOCX e Pages. **Implementado**
- **Sincronização SQLite, Markdown e JSON.** Três representações do mesmo documento são reconciliadas; divergências viram conflito para revisão humana. **Implementado** · [docs/dados/sincronizacao.md](docs/dados/sincronizacao.md)
- **Catálogo curricular.** Currículos, ofertas, relações e trajetórias carregados do SQLite depois de reconciliar os JSON. **Implementado**
- **Captura do Canvas.** Varredura sequencial e retomável de materiais publicados, leitura de notas, fóruns, avisos e Caixa de Entrada pela API do Canvas, e conversão de PDF em Markdown por página. **Parcial**: o código e os testes existem, mas nenhuma captura autenticada foi executada nesta revisão. **Não verificado** · [docs/automacoes/canvas.md](docs/automacoes/canvas.md)
- **Captura do Portal e do Lyceum.** Navegação por links acadêmicos oficiais, até 50 páginas por captura. **Não verificado** em execução real · [docs/automacoes/portal-lyceum.md](docs/automacoes/portal-lyceum.md)
- **Aula guiada e tutor locais.** Geração determinística de aula e perguntas a partir do Markdown, sem IA. **Implementado** · [docs/automacoes/extracao-textual.md](docs/automacoes/extracao-textual.md)
- **Nota final ponderada.** Cálculo local da NFp como estimativa, nunca como nota oficial. **Implementado**
- **Resumos por matéria com a API da Anthropic.** Enviam texto das matérias a um serviço externo, depois de confirmação. **Parcial**: o último teste real registrado em [docs/features/README.md](docs/features/README.md) terminou em HTTP 400. [docs/automacoes/resumos-anthropic.md](docs/automacoes/resumos-anthropic.md)
- **Gerador do acervo.** `tools/acervo` valida `dados/` e gera `build/acervo.sqlite3` e o formato antigo `materias/catalogo/*.json`. **Implementado** · [docs/dados/visao-geral.md](docs/dados/visao-geral.md)

## O que é planejado

- **Interface GPUI.** O crate `estuda-gpui` existe, mas contém só `fn main() {}` e não depende de GPUI. A interface em uso continua sendo Tauri com React. **Planejado** · [docs/arquitetura/migracao-gpui.md](docs/arquitetura/migracao-gpui.md)
- **Biblioteca pública e banco público.** Não existem `biblioteca/`, `schemas/` nem um banco público. O que há é `dados/` (JSON compartilhável com revisão) e um banco gerado. **Planejado** · [docs/dados/banco-publico.md](docs/dados/banco-publico.md)
- **Contrato de catalogação** com identificador, licença, hash e estado de publicação. É uma proposta. [docs/biblioteca/catalogacao.md](docs/biblioteca/catalogacao.md)
- **Rede entre pares** e assinatura de manifestos. É uma recomendação em [docs/ARQUITETURA_ACERVO.md](docs/ARQUITETURA_ACERVO.md), sem código.

## Princípios

- **Offline-first.** Tudo que o aplicativo mostra vem de arquivos locais e do SQLite. Canvas e Portal só são consultados quando o estudante pede, e a ausência deles não impede estudar o que já foi capturado.
- **Text-first.** O que entra no Git é texto. PDF, Word, Pages e imagens ficam fora do Git; o texto extraído entra com página e fonte. Diagramas viram Mermaid ou descrição textual. `node tools/acervo/build.mjs --verificar-texto` lista o que ainda é binário.
- **Rust.** O núcleo (`estuda-core`) não conhece Tauri nem GPUI. O shell Tauri em `app/src-tauri` e a interface React em `app/src` são a camada legada de apresentação.
- **SQLite, Markdown e JSON.** O mesmo registro existe nas três formas e é mantido coerente. Registros estruturados em `dados/` são um arquivo JSON por registro, com `source` e `pendencias`.
- **Dado pessoal em `privado/`.** Notas, frequência, matrícula, trajetória e trabalhos entregues ficam em `privado/`, pasta do mesmo repositório. Pela decisão de 2026-11-06, todo o conteúdo do acervo pode ir ao Git e ao acervo público desde que os dados pessoais estejam nessa pasta; a separação é de pasta, não de sigilo. `dados/` nunca contém dado pessoal. Ver [docs/seguranca/privacidade.md](docs/seguranca/privacidade.md).
- **Somente leitura na instituição.** O aplicativo não envia atividades, não agenda provas e não modifica registros da UVA. Ver [docs/automacoes/limites-e-seguranca.md](docs/automacoes/limites-e-seguranca.md).

## Requisitos

| Item | Versão | Para quê |
|---|---|---|
| Rust | 1.90 ou mais novo (edition 2024); a revisão usou 1.99 | compilar `estuda-core` e o shell Tauri |
| Node.js | 24 (o gerador usa `node:sqlite`) | `tools/acervo` e o frontend |
| npm | o que acompanha o Node | dependências do frontend e CLI do Tauri |
| Chromium ou Chrome | qualquer versão atual | captura do Canvas e do Portal; o app procura em caminhos fixos do macOS |
| `pdftotext` (poppler) | qualquer | só para `tools/acervo/src/pdf-para-md.mjs` |

O desenvolvimento foi feito e verificado em macOS. Outros sistemas: **Não verificado**. Detalhes em [docs/getting-started/instalacao.md](docs/getting-started/instalacao.md).

## Instalação, compilação e testes

Os comandos abaixo existem no repositório. Os que têm resultado registrado foram executados em 2026-11-06.

```sh
# Núcleo Rust (workspace em app/)
cargo test --offline --manifest-path app/Cargo.toml -p estuda-core --lib   # 152 testes passaram

# Crate de domínio (workspace próprio)
cargo test --offline --manifest-path crates/estuda-domain/Cargo.toml        # 14 testes passaram

# Gerador do acervo
node tools/acervo/build.mjs --validar                                       # 234 registros válidos
node tools/acervo/build.mjs --verificar-texto                               # 0 erros, 0 avisos
node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs   # 13 testes passaram

# Frontend
npm install --prefix app
npm test --prefix app                                                       # 40 testes passaram
npm run build --prefix app                                                  # Não verificado nesta revisão

# Aplicativo (Tauri + React)
cd app && npm run tauri dev                                                 # Não verificado nesta revisão
```

`node tools/acervo/build.mjs` sem argumento regrava `build/` e `materias/catalogo/`. Esses caminhos são gerados e ignorados pelo Git, mas o comando altera arquivos locais; use `--validar` para só conferir.

Problemas conhecidos de compilação e formatação estão em [docs/desenvolvimento/testes.md](docs/desenvolvimento/testes.md) e [docs/referencia/lacunas-e-contradicoes.md](docs/referencia/lacunas-e-contradicoes.md).

## Organização do repositório

| Caminho | O que guarda | Versionado |
|---|---|---|
| `dados/` | registros JSON de instituição, programas, períodos, disciplinas, ofertas e relações | sim |
| `privado/` | dados e notas pessoais do aluno, planejamento, trajetórias, matérias com trabalhos | sim, neste repositório local |
| `materias/` | índice e `catalogo/` gerado | só o índice |
| `fontes-uva/` | textos extraídos de PDFs oficiais, por período e matéria | sim |
| `periodos/` | índices por semestre | sim |
| `docs/` | documentação técnica, planos e specs | sim |
| `app/` | workspace Rust (`crates/estuda-core`, `crates/estuda-gpui`), shell Tauri (`src-tauri`) e frontend (`src`) | sim |
| `crates/estuda-domain/` | tipos de domínio sem I/O (ids, confiança, camadas de publicação) | sim |
| `tools/acervo/` | gerador e validador de `dados/` | sim |
| `originais/` | documentos Word, Pages e PDFs locais | não |
| `build/` | saída do gerador | não |
| `.estuda-uva.sqlite3`, `.estuda-uva/` | banco do aplicativo e espelhos de sincronização | não |

Mapa completo e divergências entre este desenho e o código: [docs/biblioteca/estrutura.md](docs/biblioteca/estrutura.md).

### Índices do acervo

Entradas de leitura que já existiam e continuam válidas. Os itens em `privado/` são dados pessoais do aluno.

- Compartilhável: [períodos](periodos/README.md) · [semestres futuros](periodos/futuros.md) · [matérias](materias/README.md) · [fontes UVA](fontes-uva/README.md) · [dados](dados/LEIA-ME.md) · [gerador](tools/acervo/README.md) · [app](app/README.md)
- Pessoal: [planejamento e rotina de aulas](privado/PLANEJAMENTO.md) · [registro acadêmico](privado/REGISTRO_ACADEMICO.md) · [cursos](privado/cursos/README.md) · [Engenharia de Computação, matrícula anterior](privado/cursos/engenharia-de-computacao/README.md) · [histórico de cursos e materiais](privado/periodos/historicos/README.md) · [2026.2](privado/periodos/2026.2/README.md) · [2026.1](privado/periodos/2026.1/README.md)
- Navegação no Portal do Aluno e nos subsistemas: [NAVEGACAO_PORTAL_UVA.md](docs/NAVEGACAO_PORTAL_UVA.md)
- Estado e arquitetura anteriores a esta revisão: [ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md](docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md) · [ARQUITETURA_ACERVO.md](docs/ARQUITETURA_ACERVO.md)

## Como contribuir

- **Código Rust ou frontend:** [docs/desenvolvimento/contribuindo-codigo.md](docs/desenvolvimento/contribuindo-codigo.md).
- **Conteúdo de disciplinas, exercícios, resoluções, bibliografia, metadados, traduções e correções documentais:** [docs/contribuindo/conteudo.md](docs/contribuindo/conteudo.md).
- Contribuições não podem incluir senhas, cookies, tokens, `.env`, bancos SQLite, perfil do Chromium, binários (PDF, Word, imagens) nem material acadêmico sem autorização de distribuição. Matrícula, notas e frequência podem entrar desde que os dados pessoais fiquem em `privado/` (decisão de 2026-11-06); RA, CPF e RG continuam fora. Dados de docentes e de outros colaboradores seguem pendentes ([lacunas e contradições](docs/referencia/lacunas-e-contradicoes.md) §1).
- Mudança em banco de dados (esquema, migração, `INSERT`, `UPDATE`, `DELETE`) exige proposta e aprovação explícita antes de qualquer execução. Ver [docs/dados/migracoes.md](docs/dados/migracoes.md).
- Convenções do repositório: branch `master`, versão `v0.1.0` como início, commits sem trailers de coautoria.

## Licenciamento

O repositório **não declara licença**: não há arquivo `LICENSE` nem campo `license` nos manifestos (`app/Cargo.toml`, `crates/estuda-domain/Cargo.toml`, `app/package.json`). Sem licença, ninguém além do autor tem permissão para copiar, modificar ou redistribuir o conteúdo. Os textos em `fontes-uva/` vêm de materiais da UVA e de terceiros e **não** são redistribuíveis por padrão. Ver [docs/seguranca/publicacao.md](docs/seguranca/publicacao.md) e [docs/biblioteca/fontes-e-licencas.md](docs/biblioteca/fontes-e-licencas.md).

O repositório local não tem remoto configurado.

## Endereços oficiais

- [Portal do Aluno UVA](https://portalaluno.uva.br/Aluno/PortalAluno)
- [Canvas UVA](https://uva.instructure.com/)

As credenciais são digitadas somente nessas páginas, na janela do navegador do aplicativo, e não são armazenadas aqui. Ver [docs/seguranca/credenciais-e-sessoes.md](docs/seguranca/credenciais-e-sessoes.md).

## Documentação detalhada

O índice completo, por público, está em [docs/README.md](docs/README.md). Atalhos:

- Novo usuário: [primeiros passos](docs/getting-started/primeiros-passos.md)
- Desenvolvedor: [visão geral da arquitetura](docs/arquitetura/visao-geral.md)
- Mantenedor de dados: [bancos e dados](docs/dados/visao-geral.md)
- Auditor de privacidade: [privacidade](docs/seguranca/privacidade.md)
- O que ainda não está resolvido: [lacunas e contradições](docs/referencia/lacunas-e-contradicoes.md)
