# Ideias, pendências e decisões — acervo da faculdade

Fica na raiz do repositório `Faculdade`, sem commit (arquivo não versionado). Atualizado em 6 de novembro de 2026. Tudo do acervo fica num único repositório, sem pasta separada.

Legenda: **feito** = já está no repositório; **aberto** = depende de você ou de uma fonte que ainda não foi consultada; **decidir** = escolha pendente.

## Organização (decidido)

- **Decidido:** tudo fica no mesmo repositório, como era antes. Não existe pasta privada separada.
- A pasta `Faculdade-aluno` foi desfeita: os arquivos voltaram para os caminhos originais. Os dois arquivos que ficaram nela foram trazidos para cá (este `IDEIAS.md`). O `README.md` que eu tinha criado lá só descrevia a separação e foi descartado.
- Estado atual (commit 4c7c5ee): dados pessoais estão em `privado/`, no mesmo repositório, e são versionados como texto. Documentos e imagens locais (PDFs, diagramas PNG, zip) estão em `originais/binarios/`, ignorado pelo Git. O único binário rastreado é o ícone do app, liberado.

## Reescrita do histórico (suspensa)

- Suspensa. Só será feita se você pedir de novo. Sem reescrita, nada precisa ser removido do histórico.
- Backup do histórico, verificado, em `.git/backup/faculdade-antes-da-reescrita.bundle`. Recuperação: `git clone` do bundle. Fica dentro de `.git`, fora da árvore de trabalho, e não entra em commits.
- Caminhos a remover de todo o histórico: documentos pessoais da raiz (`REGISTRO_ACADEMICO.md`, `ENGENHARIA_DE_COMPUTACAO.md`, `CURSOS_*`, `MATERIAIS_*`, `PLANEJAMENTO.md`, `FUTUROS_SEMESTRES.md`), `dados/canvas`, `dados/matriculas`, `dados/notas`, `dados/trajetorias`, `dados/trabalhos`, `dados/meta.json`, as 12 disciplinas pessoais em `dados/disciplinas/`, as pastas de trabalhos AVA em `materias/`, `materias/catalogo/*.json` (gerados), `docs/tmp/`, e todos os `*.pdf`, `*.png`, `*.icns`, `*.jpg`, `*.docx`.
- Ainda no histórico após a remoção: valores pessoais de carga cursada em `dados/programas` e `materias/catalogo/README.md`. Podem ser trocados com `--replace-text`.
- Havia um bloqueio: outras sessões do Faculdade reorganizavam o acervo ao mesmo tempo. Continua valendo para qualquer reescrita futura.
- Branch: renomeada para `master` (feito). Nada de `main` no repositório.

## Diagramas e imagens (feito, com pendências)

- **feito**: os três diagramas UML de Paradigmas viraram Mermaid em `materias/paradigmas-de-programacao/diagramas/*.md`. O AVA 2 aponta para eles.
- **aberto**: o diagrama do autômato (`diagrama-afd-termina-em-1.png`) e o `automato_termina_em_1.jff` sumiram de `materias/linguagens-formais-e-automatos/`. Há cópia de backup de outra sessão. O `.jff` é XML e pode virar texto. O PNG pode ser substituído por Mermaid a partir do `.jff`.
- **aberto**: os PNGs de diagrama ainda estão rastreados pelo Git até a reescrita.
- **decidir**: ícones do app (`app/src-tauri/icons/`). O `icon.svg` é texto e fica. Os PNG e ICNS gerados saem do repositório e são regenerados a partir do SVG com `npx tauri icon`. Preciso confirmar que o comando funciona com este SVG antes.

## PDFs e documentos (feito, com pendências)

- **feito**: 29 PDFs do Espaço Curso foram convertidos em Markdown com página e fonte. Os 29 restantes já tinham `.md` ao lado.
- **decidir**: apagar os PDFs locais depois de conferido o `.md`. O `PLANEJAMENTO.md` manda apagar, o `AGENTS.md` manda preservar originais. Sugestão: mover para a pasta privada, não apagar.
- **aberto**: `tmp/pdfs/` tem 4 PNGs de tutorial do app. Convertem para descrição ou saem.
- **aberto**: `originais/` tem os Word/Pages originais dos trabalhos, com nomes pessoais. Fica privado. Não converti.

## Engenharia de Computação (aberto)

- Registrado: 1.446 h cursadas, status "Evadido" exibido no Portal, objetivo de verificar aproveitamento. Está em `dados/trajetorias/engenharia-de-computacao.json` (privado) e no `build/visualizacoes/trajetorias.md`.
- **aberto**: capturar o histórico e as disciplinas cursadas do vínculo. Precisa do seu login no Portal, ou da captura do app (módulo Lyceum).
- **aberto**: pedir o Histórico Escolar com CR à Secretaria Virtual (R$ 0,00). Não foi solicitado, porque é um pedido à secretaria.
- **aberto**: comparar disciplinas de CC com as de EC. As candidatas de mesmo nome estão em `dados/relacoes` (`equivalenceConfirmed = false`). Nenhuma equivalência foi confirmada.
- **aberto**: confirmar com a UVA se o vínculo "Evadido" tem efeito sobre aproveitamento.

## Lacunas das matérias (aberto)

- Códigos E10300000 (Bases) e E10300024 (Paradigmas): aparecem só na tabela de notas do REGISTRO. Conferir no Canvas. E10300000 parece placeholder.
- Docentes: o Portal mostra "A" para 5 de 7 matérias de 2026.2. Conferir no Canvas.
- Carga horária de todas as matérias de 2026.2: não está nos planos locais. Conferir no Canvas ou no Portal.
- Turma e docente de 2026.1: não registrados em nenhuma fonte local.
- Matrícula em Projeto de Extensão de Engenharia Civil (21CIV2262A): aparece sob Ciência da Computação. Confirmar na secretaria.
- Trabalhos AVA de Bases e Paradigmas: cabeçalho sem curso Canvas nem período.
- Linguagens Formais: 4 unidades oficiais não baixadas (HTTP 403). Precisa de captura autenticada.

## Gerador e app (aberto)

- **feito**: `tools/acervo` lê `dados/`, valida (234 registros), gera `build/acervo.sqlite3` com views, `build/visualizacoes/*.md`, e os JSON antigos que o app lê. 13 testes passam.
- **aberto**: fazer o app ler `build/acervo.sqlite3` em vez do banco próprio. Exige mudança em Rust e aprovação para alterar o banco, pela regra global.
- **aberto**: ajustar `dados/` para que o gerador leia também a pasta privada (`--privado`). Ainda não está implementado.
- **aberto**: `app/src-tauri/src/canvas_capture.rs` linhas 585–587 usam 57897 com o título de Cálculo (57352). É só um teste.
- **aberto**: `npm run build` do app e `tauri build` sem os ícones gerados. Validar depois da decisão de ícones.

## Regras e processo (feito)

- **feito**: conteúdo versionado é texto. Documentos viram Markdown, diagramas viram Mermaid, binário fica fora do Git. Verificador: `node tools/acervo/build.mjs --verificar-texto`.
- **feito**: `dados/LEIA-ME.md` define regras de contribuição sem conflito, com um registro por arquivo.
- **aberto**: `PLANEJAMENTO.md` manda apagar PDFs e `AGENTS.md` manda preservar originais. Conciliar.
- **aberto**: outras três sessões do Faculdade mexem no mesmo repositório. Definir quem é o dono das mudanças no acervo.

## Anotado nesta conversa (para não perder)

- Depois que o Git ficar só com texto, qualquer dado novo entra como registro pequeno em `dados/` ou como Markdown em `materias/`.
- Este arquivo é pessoal. Não é commitado. Se o repositório for compartilhado algum dia, decidir antes o que sai.
