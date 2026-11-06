# Decisões de arquitetura pendentes

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada.

Estas decisões **não foram tomadas** no repositório e exigem a pessoa responsável pelo projeto. A pasta contém também as propostas `01`–`05` ([índice](README.md)); a decisão registrada em [05-decisao-estrutura-atual.md](05-decisao-estrutura-atual.md) (estrutura de diretórios atual mantida; `crates/` na raiz e `app/crates/` coexistem; app Tauri/React congelado) vale e **não é reaberta aqui**. A política de publicação vigente, segundo [README.md](README.md), é: todo conteúdo pode ser público desde que separado em camada própria; nunca se publicam credenciais, bancos SQLite, perfis de navegador e binários originais. Cada item descreve o que o código faz hoje (com o caminho que o comprova), o efeito e as opções. Nada aqui foi alterado por esta documentação.

## 1. Escopo do aplicativo depois da separação `privado/`

**Fato verificado.** O repositório passou a guardar dados pessoais em `privado/` (`privado/cursos`, `privado/materias`, `privado/periodos`, `privado/dados`, `privado/PLANEJAMENTO.md`, `privado/REGISTRO_ACADEMICO.md`) e removeu `cursos/`, `PLANEJAMENTO.md` e `REGISTRO_ACADEMICO.md` da raiz (aparecem como removidos no `git status`; `privado/` aparece como não rastreado). O código de `estuda-core` **não foi atualizado**:

- `catalog/scanner.rs` (`is_in_scope_file`, `is_in_academic_tree`) só indexa, na raiz, `docs/`, `materias/`, `originais/`, `fontes-uva/`, `periodos/` e `cursos/`, e arquivos de primeiro nível com prefixos como `registro_academico` e `planejamento`. Não há nenhuma ocorrência de `privado/` fora de testes (`grep` em `app/crates` e `app/src-tauri/src`).
- `sync.rs::reconcile_all` usa `scan_library`, então a sincronização também não vê `privado/` nem `dados/`.
- `sync_watcher.rs` observa recursivamente apenas `materias/` e `docs/`.

**Efeito real hoje (inferido da leitura do código; não executei o app):** Markdown e JSON em `privado/` não entram no acervo do aplicativo, não são pesquisados, não são sincronizados com o SQLite e não alimentam os resumos de IA. Os testes do scanner ainda usam `cursos/engenharia-de-computacao/README.md` e `classification.rs` ainda classifica `cursos/` ("histórico de programa"), mas essa pasta não existe mais na raiz. Se o objetivo é que o app continue lendo o histórico pessoal, é preciso incluir `privado/`; se o objetivo é isolá-lo, o comportamento atual já isola, mas por omissão, não por regra explícita.

Note ainda que [05](05-decisao-estrutura-atual.md) lista `cursos/` como parte da base pública, mas `cursos/` não existe mais na raiz (foi para `privado/cursos/`).

**Decisão:** (a) o app passa a ler `privado/` com uma regra explícita de camada; (b) o app só lê a camada compartilhável e os dados pessoais ficam para um fluxo separado; (c) manter como está e documentar. Uma mudança de código aqui não faz parte deste trabalho documental.

**Relacionado.** O material importado do Canvas é gravado em `materias/<slug>-<id>/materiais/` (teste `imported_material_is_markdown_idempotent_and_never_keeps_raw_pdf`, `estuda-core/src/lib.rs`), dentro de `materias/`, que **não** está no `.gitignore` (só `materias/catalogo/capturas/` está). Texto extraído de e-books da UVA pode, portanto, acabar na camada que o repositório trata como versionável. Decidir a camada e o regime de direitos desse conteúdo (ver [segurança](../seguranca/publicacao.md)).

## 2. `estuda-domain` e `estuda-core` não se conhecem

`crates/estuda-domain` define `Confidence`, identificadores tipados e a classificação `Layer` (privado/gerado/compartilhável/revisão). `estuda-core` **não** depende dele e reimplementa ideias equivalentes de forma independente: `sync::is_sensitive_or_excluded`, `scanner::is_in_scope_file`, `classification::classify_path`, e os estados de captura. Só `estuda-sync` declara a dependência (`app/crates/estuda-sync/Cargo.toml`). O próprio `Cargo.toml` do domínio diz que um `Cargo.toml` na raiz assumirá "quando os crates forem extraídos" (`[workspace]` isolado).

**Efeito:** duas fontes de verdade para "o que é privado". As listas de `publication.rs` (por exemplo `PRIVATE_PREFIXES` com `privado/`, `cursos/`, `dados/matriculas/`) divergem do escopo do scanner.

**Já decidido (05):** `estuda-domain` permanece em `crates/` na raiz e `estuda-sync` o consome por caminho relativo; a recomendação de movê-lo para `app/crates/` está suspensa. **Pendente:** integrar `estuda-domain` ao núcleo e ao scanner, ou mantê-lo apenas como ferramenta de auditoria de publicação.

## 3. Destino de `estuda-sync`

Ver [crates](crates.md#estuda-sync). O crate (15 módulos, 112 `#[test]`) surgiu durante esta revisão, não foi compilado nem testado por ela e não é usado por nenhum outro crate. Decidir: quem o integra, se coexistirá com `estuda-core::sync` (nome parecido, responsabilidades diferentes: o primeiro é leitura acadêmica; o segundo, sincronização local de arquivos) e se haverá renomeação para evitar confusão. A persistência dele é uma proposta com aprovação; qualquer tabela nova no SQLite exige **aprovação prévia explícita** (regra de banco do projeto).

## 4. Interface GPUI

Estado: esqueleto (ver [migração](migracao-gpui.md)). Decisões abertas:

- Usar `gpui-kit` (declarado) ou `gpui` diretamente.
- O binário de `estuda-gpui` se chama `estuda-uva` (registrado em 05 como escolha), o mesmo nome do binário do shell Tauri (`app/src-tauri/Cargo.toml`, `[[bin]] name = "estuda-uva"`). Os workspaces são distintos, mas os artefatos têm o mesmo nome; pode confundir quem empacota.
- Onde viverá a orquestração hoje presa a `tauri::State`: núcleo, novo crate de serviços ou cada interface.
- Plataformas-alvo (hoje só macOS é exercitado; `detect_chromium` lista caminhos de macOS e Homebrew).
- Quando aposentar `src-tauri` e a UI React.

## 5. Banco público inexistente

Não existe banco "público". O que existe: o banco pessoal `.estuda-uva.sqlite3` e o banco gerado `build/acervo.sqlite3` (descartável, produzido por `tools/acervo`), que, segundo `dados/LEIA-ME.md`, lê `dados/` **e** `privado/dados/`, ou seja, mistura as duas camadas e está classificado como "gerado" (nunca publicável). `docs/ARQUITETURA_ACERVO.md` §5 e §6 tratam o compartilhamento e a rede entre pares como pendentes. `estuda-domain` já reserva `biblioteca/` e `schemas/` como compartilháveis, mas essas pastas não existem.

**Decisão (modelo alvo em [03](03-dados-e-sincronizacao.md), SQL proposto e não executado):** onde viverá a biblioteca pública, em que formato (arquivos + manifesto, ou SQLite distribuível), quem revisa e quais conteúdos podem entrar. Ver [banco público](../dados/banco-publico.md) e [biblioteca](../biblioteca/estrutura.md).

## 6. Envio de conteúdo à API Anthropic

**Fato verificado.** `study_summaries::generate_subject_summaries` coleta texto dos materiais da matéria (até 2 MiB) e `anthropic::AnthropicClient::generate_pair` o envia a `https://api.anthropic.com/v1/messages`. A chave é lida de `$HOME/Dev/Projetos/gerenciador-de-conteudo/.env` (caminho fixo em `AnthropicConfig::read_default`), fora deste repositório. A UI exige confirmação do hash das fontes antes da geração. Ao mesmo tempo, a barra lateral da UI afirma "Sem IA, nuvem ou cobrança por uso" (`app/src/ui/App.tsx`) e a especificação do tutor local recusa tutor remoto por privacidade (`docs/features/README.md`, decisão 1).

**Decisão pendente (já registrada em `docs/features/README.md`):** (a) envio liberado com adesão por material, (b) IA só sobre trechos escolhidos, (c) IA só para tarefas sem conteúdo acadêmico. Também: corrigir o texto da barra lateral, definir onde a chave fica, e o cabeçalho `anthropic-workspace-id` (o mesmo documento relata resposta HTTP 400 para a chave atual; **não reverifiquei**).

## 7. Raiz do acervo fixada na compilação

`run()` usa `env!("CARGO_MANIFEST_DIR")` + `../..` como acervo padrão. Um binário empacotado e movido continua apontando para o caminho de compilação até que o usuário defina outro (`set_library_path`, guardado em `settings`). Decidir se o padrão deve ser configurável de forma explícita no primeiro uso.

## 8. Higiene do repositório e da documentação existente

- A extração para `estuda-core` e a reorganização `privado/` **não estão commitadas**; o histórico Git ainda contém os caminhos antigos. Decidir quando e como commitar.
- `app/target/` (alvo de compilação do novo workspace) aparece como não rastreado; o `.gitignore` ignora `/app/src-tauri/target/` e `/crates/*/target/`, mas não `/app/target/`. Não foi alterado por esta documentação.
- Documentos existentes citam caminhos antigos (`docs/ARQUITETURA_ACERVO.md` cita `app/src-tauri/src/catalog/scanner.rs`; comentário de `grading.rs` cita `materias/fontes-uva/2026.1/...`; `app/README.md` e `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md` usam `--manifest-path app/src-tauri/Cargo.toml`). Ver a lista completa na página de lacunas da documentação.
- `catalog/guidance.rs` embute uma versão antiga do texto de `AGENTS.md` (seis itens, sem as regras de conteúdo em texto e de `privado/`) e a recria se o arquivo faltar.
- A máquina de CI e a política de testes com Chromium real (`launches_real_isolated_chromium` exige `/opt/homebrew/bin/chromium`) não estão definidas. Ver [testes](../desenvolvimento/testes.md).
