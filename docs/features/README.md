# Features com IA: pipelines e ideias

Este diretório descreve como o acervo e o Estuda UVA podem usar a API da Anthropic (Claude). Os documentos são planos e ideias. Só o que a seção "Estado atual" marca como existente está no código.

## Arquivos

- [roteamento-modelos.md](roteamento-modelos.md): qual modelo usar em cada tipo de tarefa e quando escalonar.
- [pipelines.md](pipelines.md): pipelines em etapas, com entradas, saídas, modelo por etapa e critérios de aceite.
- [ideias.md](ideias.md): backlog de ideias ainda não priorizadas.

## Estado atual (verificado em 2026-11-06)

- **Credenciais:** `ANTHROPIC_API_KEY` e `ANTHROPIC_WORKSPACE_ID` ficam no `.env` do projeto `~/Dev/Projetos/gerenciador-de-conteudo/` (permissão `0600`). Os valores não são copiados para este repositório. O `.env` é ignorado pelo Git deste repo (`.gitignore`, linha 24).
- **Workspace:** `faculdade` (`wrkspc_018v72cf9S8Mu65cT6kw6Ate`). Não é segredo, mas fica só no `.env`.
- **Modelo configurado:** `ANTHROPIC_MODEL=claude-haiku-5-5`. Sem a variável, o padrão do código também é `claude-haiku-5-5`.
- **Opus e Sonnet:** não estão configurados no app. Hoje há um único modelo para todas as chamadas.
- **Ping real (2026-11-06):**
  - Antes: com a chave anterior e sem o header de workspace, a API respondeu `HTTP 400` ("not scoped to a workspace").
  - Depois: com a chave nova e o header `anthropic-workspace-id`, `HTTP 200` com `claude-haiku-5-5`.
- **Cliente Rust:** `app/crates/estuda-core/src/anthropic.rs` (o código saiu de `app/src-tauri/src/`), ainda não versionado. Envia `x-api-key`, `anthropic-version` e, quando `ANTHROPIC_WORKSPACE_ID` existe, `anthropic-workspace-id`. Os 10 testes com servidor mock passaram em `cargo test anthropic` no crate `estuda-core`. Eles não chamam a API real.
- **Resumos com IA:** spec em `docs/superpowers/specs/2026-10-14-estuda-uva-resumos-anthropic-design.md`, plano em `docs/superpowers/plans/2026-10-14-estuda-uva-resumos-anthropic.md`, código em `app/crates/estuda-core/src/study_summaries.rs`. Também não versionado.
- **Tutor interativo:** funciona sem IA, por decisão de design (`docs/superpowers/specs/2026-10-14-estuda-uva-tutor-interativo-local-design.md`).

## Decisões em aberto

1. **Privacidade do conteúdo acadêmico.** A spec do tutor local recusou um tutor remoto porque enviaria conteúdo acadêmico a um serviço externo. A spec de resumos envia os textos das matérias à API. Os pipelines deste diretório também enviam. A política de 6/11/2026 trata da publicação no Git e no acervo público. Ela não decide se o conteúdo pode ser enviado a um serviço externo. Antes de implementar qualquer um deles, é preciso escolher uma opção:
   - (a) envio liberado com opt-in por material;
   - (b) IA restrita a trechos escolhidos pela estudante;
   - (c) IA só para tarefas que não enviam conteúdo acadêmico.
2. **Escopo de trabalhos e provas.** Estes pipelines assumem que a IA apoia o estudo e a revisão de textos que a própria estudante escreveu. A IA não gera a entrega avaliada para ser enviada como autoral.
3. **Roteamento no código.** A política de modelos precisa estar no código, e não só no `.env`. Assim, `ANTHROPIC_MODEL=claude-haiku-5-5` não rebaixa uma tarefa de trabalho ou prova. Ver [roteamento-modelos.md](roteamento-modelos.md).

## Convenções

- IDs de modelo: `claude-opus-5-5`, `claude-sonnet-5-5`, `claude-haiku-5-5`. Confirmar com uma chamada mínima antes de adotar cada um.
- Chave, cabeçalho de autenticação, `.env`, bancos SQLite, perfil do Chromium e originais (`originais/`) não vão para logs, Markdown, JSON nem Git.
- Conteúdo do acervo pode ir ao Git e ao acervo público, inclusive notas, matrículas e trabalhos (decisão do dono, 6/11/2026). Dados pessoais ficam separados em `privado/`. RA, CPF e RG ficam fora dos registros.
- Mudança no banco (tabela nova, migração, `INSERT`, `UPDATE`) exige aprovação prévia da estudante, conforme a regra global de banco.
- Saída de IA é marcada como "gerado por IA, conferir na fonte". Nunca vira dado acadêmico oficial (nota, frequência, matrícula).
- Original preservado: nenhum pipeline sobrescreve ou move arquivo do acervo (ver `AGENTS.md`).
