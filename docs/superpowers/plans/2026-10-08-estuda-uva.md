# Plano de implementação — Estuda UVA

> Objetivo: entregar e validar um app Tauri local que navega o Canvas UVA com automação Rust, converte PDFs de curso em Markdown e monta uma aula local determinística sem IA.
>
> Design aprovado pelo usuário: opção 3 — automação em Rust sem Playwright.
> Especificação: `docs/superpowers/specs/2026-10-08-estuda-uva-design.md`.

## Contrato global

- Trabalhar na branch `main`, sem criar branch/worktree.
- Commits locais por fase; nenhum push (não há remoto configurado no checkout).
- Preservar as alterações já commitadas no acervo, inclusive as revisões de inventário feitas após `47f6af3`.
- Não versionar PDFs brutos, cookies, senhas, tokens, perfil Chromium, downloads temporários nem dados pessoais novos.
- Não usar Playwright, Node runtime, LLM, serviço de IA ou API de IA.
- React/TypeScript só apresenta dados e eventos. O backend Rust faz navegação, rede, extração, geração, persistência e todos os cálculos.
- Test-first para regras e componentes solicitados; nenhum teste usa Canvas real sem autenticação manual explícita na janela isolada.

## Fase 0 — Spike de automação e extração

**Objetivo:** provar o caminho de execução local antes de estruturar o produto.

**Arquivos:**
- Criar: `app/` (skeleton Rust temporário, depois absorvido no Tauri)
- Criar: `docs/superpowers/plans/2026-10-08-estuda-uva.md`
- Criar: `docs/superpowers/specs/2026-10-08-estuda-uva-design.md`

**Tarefas:**

1. Fixar Tauri estável 2.12.1, chromiumoxide 0.9.1, pdf-extract 0.12.1 e lockfiles.
2. Escrever testes pequenos para: abrir Chromium com profile isolado; capturar DOM de fixture; extrair um PDF textual por página; transformar páginas em Markdown com referências.
3. Implementar um spike que use `/opt/homebrew/bin/chromium` detectado em runtime e um perfil temporário, abra página local de fixture e encerre Chromium sem zombie.
4. Gerar um PDF fixture sintético com 2 páginas no teste; provar extração e cabeçalhos de página sem incluir material UVA privado no Git.
5. Confirmar no ambiente que o app não exige Node para runtime e que `Chromium` está instalado.

**Gate:** `cargo test` do núcleo passa; browser runner termina e limpa perfil temporário. Se chromiumoxide não iniciar, trocar para `thirtyfour` em Rust com WebDriver após registrar a evidência; nunca voltar a Playwright.

## Fase 1 — Esqueleto Tauri e contratos IPC

**Arquivos:**
- Criar: `app/src-tauri/` com Tauri v2/Rust.
- Criar: `app/src/` com React/TypeScript/Vite.
- Criar: `app/package.json`, `app/tsconfig.json`, `app/vite.config.ts` e lockfile.
- Criar: `app/src-tauri/capabilities/default.json`, ícones e configuração.
- Modificar: `.gitignore`, `README.md`.

**Tarefas:**

1. Criar shell acessível com navegação: Visão geral, Matérias, Importação e Aula.
2. Definir contratos serializáveis Rust→UI para estados, disciplina, módulo, item e progresso; tipagem gerada ou espelhada com validação.
3. Expor somente comandos Tauri explícitos; capability mínima sem shell genérico e sem filesystem arbitrário no frontend.
4. Fazer tela “Conectar ao Canvas” e estados claros de browser ausente / não conectado.
5. Verificar comandos/erros sem cálculos no frontend.

**Verificação:** `npm run build`, `cargo check`, `cargo test` e `cargo tauri build` para target atual; busca estática confirma ausência de Playwright.

## Fase 2 — Serviço Chromium/CDP em Rust e sessão

**Arquivos:**
- Criar: `app/src-tauri/src/browser/` (`launcher.rs`, `session.rs`, `events.rs`).
- Criar: `app/src-tauri/src/canvas/` (`auth.rs`, `dom.rs`, `courses.rs`).
- Modificar: `app/src-tauri/src/lib.rs`, `app/src-tauri/Cargo.toml`, capability e UI.

**Tarefas:**

1. Detectar executáveis Chromium/Chrome macOS arm64 e caminho selecionado; validar executável antes do launch.
2. Criar `user_data_dir` próprio em AppData; não tocar no perfil do Chrome do usuário.
3. Lançar visível em `https://uva.instructure.com/courses`, manter o `Handler` CDP vivo em task Tokio e encerrar sem processo órfão.
4. Representar `Disconnected`, `WaitingForLogin`, `Authenticated`, `Expired`, `BrowserError`; detecção por host/rota/DOM, nunca por senha.
5. Descobrir cursos da tabela/seções do Canvas, com dedupe por href e metadados visíveis. Não fixar os IDs observados em 2026.
6. Emitir eventos de progresso/estado para a UI; erros não devem conter cookies ou URLs com segredo.

**Verificação:** unit tests de classificação de rotas/HTML; launch/shutdown real com perfil descartável; janela de login mostrada ao usuário na sessão de homologação.

## Fase 3 — Descoberta robusta de módulos e arquivos

**Arquivos:**
- Criar: `app/src-tauri/src/canvas/modules.rs`, `items.rs`, `frames.rs`, `models.rs`.
- Criar: `app/src-tauri/tests/fixtures/canvas/` com HTML anonimizado/sintético.
- Modificar: `app/src/` para seleção de curso/módulo/material.

**Tarefas:**

1. Para cada curso selecionado, navegar ao caminho obtido do link da tabela; analisar `#context_modules` ou estrutura semântica atual.
2. Retornar módulos ordenados e itens com título, href, tipo, ordem e estado; suportar cards colapsados expandindo pelo DOM/CDP.
3. Filtrar tarefa/fórum/notas fora de “material”; nunca clicar em enviar, nota ou publicação.
4. Abrir apenas itens que o usuário selecionou; descobrir `iframe`, `embed`, `object`, links PDF e URLs redirecionadas.
5. Validar destino e origem; itens fora da allowlist ficam aguardando decisão na UI.
6. Não duplicar itens compartilhados e conservar ordem de leitura do curso.

**Verificação:** fixture espelha caminho real observado: lista → curso → módulos → item URL externa → iframe PDF. Testes garantem que tarefa/fórum não seja classificado como PDF e que curso IDs são dinâmicos.

## Fase 4 — Download, validação e PDF → Markdown

**Arquivos:**
- Criar: `app/src-tauri/src/download/` (`url_policy.rs`, `cookies.rs`, `client.rs`).
- Criar: `app/src-tauri/src/pdf/` (`extract.rs`, `markdown.rs`, `metadata.rs`).
- Criar: `app/src-tauri/tests/fixtures/pdf/` com PDF sintético.
- Modificar: banco/modelos de material e tela de importação.

**Tarefas:**

1. Implementar allowlist HTTPS, redirecionamentos com revalidação de host, referer apropriado, cookie somente para domínio igual e timeout.
2. Limitar tamanho, concorrência, cancelamento e nome local; identificar PDF com cabeçalho `%PDF-` e tipo de resposta.
3. Processar por páginas com `pdf-extract`; nenhuma chamada a shell para extrair texto.
4. Gerar Markdown UTF-8 por página com URL oficial, título, páginas e alertas de extração; não copiar cookies ou nome pessoal dos metadados PDF.
5. Detectar documento sem camada de texto, protegido, vazio, corrompido ou parcialmente extraído e apresentar motivo no `.md`/UI.
6. Escrever atomicamente em `materias/<slug>/materiais/`; não sobrescrever arquivo manual; comparar hash e permitir nova versão.
7. Remover temporários brutos depois da extração bem-sucedida e ao cancelar. Um erro preserva diagnóstico sem deixar PDF secreto no repositório.

**Verificação:** testes do validador de URL, redirects, PDF válido/inválido, saída de duas páginas, cancelamento, colisões e limpeza de temporários; inspeção de `git status` não mostra PDFs.

## Fase 5 — Aula guiada determinística sem IA

**Arquivos:**
- Criar: `app/src-tauri/src/lesson/` (`outline.rs`, `terms.rs`, `questions.rs`, `lesson.rs`).
- Criar: fixtures de texto PT-BR autorizadas/sintéticas e golden Markdown.
- Modificar: UI Aula e modelo de progresso.

**Tarefas:**

1. Extrair títulos, objetivos, definições, exemplos e frases de revisão por padrões determinísticos, sempre ligados ao número de página.
2. Ordenar pela sequência do PDF e produzir `AULA.md` com visão geral, mapa, leitura guiada, vocabulário, prática e gabarito referenciado.
3. Gerar apenas perguntas cuja resposta possa ser apontada no texto. Ausência de evidência vira aviso, nunca conteúdo inventado.
4. Implementar apresentação passo a passo e perguntas interativas; toda validação, pontuação e cálculo fica no Rust.
5. Incluir uma descrição clara no app: material produzido localmente por regras, sem IA generativa; não prometer explicação livre.

**Verificação:** snapshots determinísticos; mesmo PDF/texto gera saída idêntica; pergunta e gabarito apontam para a página-fonte; teste prova que UI só apresenta pontuação fornecida pelo backend.

## Fase 6 — Persistência e experiência completa

**Arquivos:**
- Criar: `app/src-tauri/src/storage/` (`db.rs`, migrations, repositories).
- Modificar: páginas Matérias, Importação, Aula e Visão geral.
- Modificar: `README.md`, `materias/README.md`, `.gitignore`.

**Tarefas:**

1. Criar SQLite local para catálogo importado, estados e resultados de prática; nenhuma credencial oficial/notas acadêmicas.
2. Criar arquivo por PDF + `AULA.md` + índice por matéria e atualizar `materias/README.md` sem perder edições manuais.
3. Mostrar importações concluídas, parciais, canceladas e falhas por arquivo.
4. Oferecer seleção de pasta do acervo; padrão deste uso aponta para a raiz Faculdade configurada, nunca para perfil/browser/temporary.
5. Integrar cancelamento, repetição, resolução de colisões e abrir o Markdown no app.

**Verificação:** persistência reabre corretamente; migrações idempotentes; fixture de matéria produz índice consistente e todos os links relativos resolvem.

## Fase 7 — Homologação real e pacote macOS

**Arquivos:**
- Modificar: README/app README e notas de distribuição.
- Criar: script de build/QA se reduzir erros de repetição.
- Ignorar: bundle/artefatos gerados, logs, perfis e dados privados.

**Tarefas:**

1. Executar todas as verificações do plano e `cargo tauri build`.
2. Abrir `.app` gerado; confirmar janela, estados e navegação de frontend.
3. Na janela Chromium isolada, permitir que o usuário entre no Canvas manualmente; sem digitar senha no app/terminal.
4. Selecionar uma disciplina real e um item PDF real; importar para a pasta Faculdade e conferir título, todas as páginas, link de origem e presença de texto.
5. Gerar e estudar `AULA.md`, responder questão e conferir score calculado no Rust.
6. Reabrir app e confirmar persistência, cancelamento e que nenhum PDF bruto/cookie foi versionado.
7. Fazer auditoria requisito→evidência, revisar `git diff --check`, segredos, `git status --ignored` e commits; commitar as mudanças finais localmente.

**Gate de saída:** build local abre, fluxo Canvas seleciona curso/módulo, PDF real vira Markdown com páginas e aula determinística, progresso funciona, fontes ficam sem PDF no Git, nenhuma IA/Playwright/runtime Node aparece, commits completos e working tree limpo.
