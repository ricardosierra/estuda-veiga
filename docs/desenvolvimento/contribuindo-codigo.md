# Contribuindo com código

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada.

Este guia é para quem altera Rust, TypeScript ou o gerador Node. Contribuições de conteúdo (disciplinas, exercícios, resoluções) seguem o processo da [biblioteca](../contribuindo/conteudo.md). Leia antes [`AGENTS.md`](../../AGENTS.md), [convenções](convencoes.md) e [ambiente e compilação](ambiente-e-compilacao.md).

## Antes de começar

1. **Dados privados fora do repositório de contribuição.** Não inclua matrícula, notas, frequência, CPF, cookies, tokens, URLs com parâmetros de autenticação nem documentos acadêmicos não autorizados em código, testes, exemplos ou mensagens de commit. Use dados sintéticos.
2. **Reutilize antes de criar.** Procure o que já existe em `estuda-core` (por exemplo, `canvas::sanitize_source_url`, `pdf::safe_slug`, `sync::validate_relative_path`) antes de escrever outra versão.
3. **Escolha o pacote certo.** Regras de negócio sem UI vão para `app/crates/estuda-core`; orquestração presa ao Tauri vai para `app/src-tauri/src/lib.rs` (legado; ver [migração](../arquitetura/migracao-gpui.md)); lógica de tela testável vai para `app/src/ui/*.ts`.
4. **Banco de dados exige aprovação prévia** (ver a seção abaixo).

## Criar um módulo no núcleo

1. Crie `app/crates/estuda-core/src/<nome>.rs` com um comentário `//!` que declare a regra central do módulo.
2. Registre `pub mod <nome>;` em `app/crates/estuda-core/src/lib.rs` (a lista está em ordem alfabética).
3. Defina um enum de erro com `thiserror` (mensagens em português, sem segredo) se o módulo puder falhar.
4. Separe a regra pura (testável sem rede) da parte com I/O.
5. Escreva testes em `#[cfg(test)] mod tests` no mesmo arquivo, com `tempfile` quando houver arquivos (ver [testes](testes.md)).
6. Rode `cargo test --manifest-path app/Cargo.toml -p estuda-core --lib` e atualize a contagem de testes em [testes](testes.md) e em [crates](../arquitetura/crates.md).

Se o módulo precisar de estado compartilhado, siga o padrão existente: `Mutex` interno no tipo, não global.

## Adicionar um comando ao shell Tauri (legado)

1. Escreva `#[tauri::command] async fn nome(state: State<'_, AppState>, ...) -> Result<T, String>` em `app/src-tauri/src/lib.rs`. Para trabalho bloqueante use `tauri::async_runtime::spawn_blocking`, como `sync_local_catalog`.
2. Inclua `nome` na lista de `tauri::generate_handler![...]` ao fim do arquivo.
3. Chame com `invoke<T>("nome", {...})` no frontend; use `camelCase` nos argumentos.
4. Documente o comando em [referência/comandos](../referencia/comandos.md).

## Adicionar um adaptador acadêmico (novo sistema de origem)

Hoje os "adaptadores" existentes são: Portal/Lyceum (`lyceum.rs` + `browser.rs`), Canvas HTML (`canvas.rs`), Canvas API (`canvas_api.rs` + `browser.rs`), download de PDF (`download.rs`) e API Anthropic (`anthropic.rs`). O crate `estuda-sync` (não integrado; ver [crates](../arquitetura/crates.md#estuda-sync)) define contratos (`reader::PortalReader`, `CanvasReader`, `ExamReader`, `CalendarReader`) e a política somente-leitura (`policy::ReadOnlyPolicy`), mas **nenhum leitor concreto o implementa ainda**.

Requisitos para qualquer adaptador novo (derivados de `AGENTS.md` e do código existente):

1. **Somente leitura.** Sem POST, sem envio de formulário, sem marcar como lido/entregue, sem agendar prova. Documente a garantia e teste.
2. **Autenticação manual** na janela visível do navegador; o app não pede nem guarda senha.
3. **Allow-list de hosts e URLs** (HTTPS, sem credencial embutida, sem porta); veja `download::validate_material_url`.
4. **Limites explícitos** (páginas, linhas, bytes, tempo) e resultado **parcial** marcado quando atingidos.
5. **Proveniência**: fonte, URL sem query, data da leitura, estado (confirmado, estimado, desconhecido). `Confidence` em `estuda-domain` é o vocabulário previsto.
6. **Sem inventar dados**: o que não foi lido é "desconhecido", nunca zero.
7. **Procedimento documentado**: registre o caminho de navegação em [`docs/NAVEGACAO_PORTAL_UVA.md`](../NAVEGACAO_PORTAL_UVA.md) como "a validar" até uma observação autenticada real.
8. **Testes com HTML/JSON sintético** (nunca páginas reais).

## Adicionar materiais à biblioteca

Não é uma tarefa de código. Siga [contribuindo/conteudo.md](../contribuindo/conteudo.md). Para o gerador, cada registro de `dados/` é um arquivo JSON por entidade, com `source` apontando para um arquivo existente e `null` com `pendencias` para o que não se sabe (regras em [`dados/LEIA-ME.md`](../../dados/LEIA-ME.md)). Valide com `node tools/acervo/build.mjs --validar` antes de enviar.

## Revisar conflitos de sincronização

O sincronizador não resolve divergência sozinho. Conflitos ficam em `sync_conflicts` e aparecem na tela **Dados e sync**. Para revisar: compare os três valores (SQLite, Markdown, JSON espelho), escolha o conteúdo final e use a ação de resolver (`resolve_local_sync_conflict` → `SyncEngine::resolve_conflict(relative_path, content)`). Remoção de arquivo vira conflito de restauração (*tombstone*). Nunca apague os espelhos de `.estuda-uva/mirrors/` à mão para "limpar" um conflito. Ver [sincronização](../dados/sincronizacao.md).

## Propor migrações de banco

> **Regra bloqueante do projeto:** toda alteração de banco (DDL, DML, arquivos de migração, execução de migração) exige **aprovação prévia, explícita e individual** da pessoa responsável, no chat, antes de ser criada ou executada. Isso vale mesmo em modo de permissão total. Esta documentação **não autoriza executar SQL**.

Fluxo de proposta:

1. Escreva a proposta (em issue, PR ou mensagem), contendo: (a) o SQL exato ou o código da migração; (b) a justificativa técnica; (c) o impacto estimado (tabelas afetadas, risco de bloqueio ou perda de dados, reversibilidade).
2. Aguarde a aprovação explícita.
3. Só então implemente. O padrão existente em `storage.rs`:
   - editar `BASELINE` em `app/crates/estuda-core/src/schema.rs`: só `CREATE ... IF NOT EXISTS`, **nunca `ALTER TABLE`**; somar 1 a `CURRENT_VERSION` (hoje 8) e ao `PRAGMA user_version` no fim do script; atualizar `BASELINE_TABLES`;
   - bancos de versão menor são movidos para `*.bak.sqlite3` e regenerados pela abertura (`open_connection`): não escreva migração incremental;
   - um teste em banco temporário para a tabela nova (modelo: `fresh_database_uses_the_single_baseline_with_every_table`);
   - o código recusa bancos com versão maior que a suportada.
4. Atenção: as tabelas `sync_*` (`sync.rs`) e `academic_*` (`catalog/academic.rs`) são criadas com `CREATE TABLE IF NOT EXISTS` **sem** `user_version` próprio; alterá-las exige estratégia explícita na proposta.
5. Nunca teste migração no banco real `.estuda-uva.sqlite3`.

## Checklist antes de enviar

- [ ] Nenhum dado pessoal, segredo ou binário no diff.
- [ ] Testes novos para o comportamento novo; suítes relevantes executadas e resultado registrado com honestidade (o que passou, o que não foi executado).
- [ ] `cargo fmt` aplicado nos arquivos tocados (a base atual já falha na checagem; não reformate arquivos alheios sem combinar).
- [ ] Documentação atualizada: [crates](../arquitetura/crates.md), [comandos](../referencia/comandos.md) e, se mudou limite, [performance](performance.md).
- [ ] Nenhuma alteração de banco sem aprovação.
- [ ] Mensagem de commit no estilo `tipo(escopo): resumo` (ver [convenções](convencoes.md)).
