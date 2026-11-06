# Migrações de esquema

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

> **Regra bloqueante.** Toda alteração de banco de dados (DDL, DML, arquivo de migração, seeder com mutação, execução de migração ou de CLI de banco) exige **aprovação prévia, explícita e individual do usuário no chat**, antes de ser criada ou executada, mesmo em modo de permissão total. A baseline v8 foi autorizada pelo dono em 6/11/2026 ("um migration sem alter table"). Esta página documenta como o esquema funciona e como **propor** uma mudança futura; ela não autoriza executar SQL em nenhum banco real, nem outras mudanças de esquema.

Relacionados: [banco privado](banco-privado.md) · [sincronização](sincronizacao.md) · [visão geral](visao-geral.md).

## 1. Como o esquema funciona hoje (Implementado)

**Decisão do dono (6/11/2026, no chat):** o projeto fica com **uma migração só, sem `ALTER TABLE`**. O banco local é 100% local e precisa poder ser **regenerado** quando necessário; por isso não existe cadeia de migrações incrementais.

- **Fonte única do esquema:** `app/crates/estuda-core/src/schema.rs`. `BASELINE` é um script só com `CREATE TABLE IF NOT EXISTS` / `CREATE INDEX IF NOT EXISTS` e termina em `PRAGMA user_version=8`. `CURRENT_VERSION = 8`.
- **Abertura** (`storage.rs`, `open_connection`, usada por `AcademicStore::open` e por `set_library_path`):
  1. Lê `user_version`. **Maior que 8:** recusa com "o banco está na versão N, mais nova que a versão suportada 8" e não altera o arquivo (teste `newer_database_is_refused_and_left_byte_identical`).
  2. **Menor que 8** (ou sem versão, mas com tabelas): o arquivo **não é migrado**. `quarantine_old_database` consolida o WAL (`wal_checkpoint(TRUNCATE)`) e move o banco, com `-wal`/`-shm`/`-journal`, para `<nome>.v<versão>-<epoch>.bak.sqlite3` na mesma pasta. Nada é apagado. Um banco novo nasce da `BASELINE` e grava `settings.__regenerated_from_backup` com o caminho do backup.
  3. Versão igual ou arquivo novo/vazio: executa a `BASELINE` (idempotente; não escreve nada se o esquema já existe) e `catalog::academic::initialize_schema`.
- **O que volta sozinho depois de regenerar:** `sync_documents` e os espelhos (a próxima sincronização lê Markdown/JSON do acervo), as tabelas `academic_*` (projeção dos JSON validados). **O que não volta sozinho:** sessões de aula, interações, resumos, histórico de estudo e execuções de captura. Eles continuam no arquivo de backup; restaurar é copiá-lo de volta (ou consultá-lo com `sqlite3` em modo somente leitura).
- **Tabelas do motor de sincronização (v8):** `sync_runs`, `http_validators`, `source_observations`, `ava_checks`, `exam_records`. Os estados de verificação têm `CHECK` (`confirmed`, `not_found`, `unknown`, `conflicting`, `not_applicable`): não é possível gravar "não entregue" por inferência. Detalhes em [banco privado](banco-privado.md#4-esquema-real-versão-8).
- **Sem arquivos de migração.** O esquema é o código Rust `schema.rs`; não há SQL em pasta nem ferramenta externa.

### O que `user_version` não cobre

- **`sync_*`** (`sync_documents`, `sync_conflicts`, `sync_tombstones`, `sync_outbox`), criadas por `SyncEngine` com `CREATE TABLE IF NOT EXISTS`, e **`academic_*`**, criadas por `catalog::academic`, também sem `ALTER`. Como o arquivo inteiro é regenerado quando a versão sobe, mudar essas tabelas é mudar o `CREATE` e subir `CURRENT_VERSION`.
- **Arquivos de espelho** têm `schemaVersion: 1` no envelope JSON; mudar o envelope é migração de arquivos, não de banco.

### Histórico (só para leitura de bancos antigos)

Versões 2 a 7 existiram com `ALTER TABLE` incremental e foram **removidas** do código. Um banco nessas versões é guardado como backup, não atualizado. Colunas hoje incorporadas à `materials`: `semester`, `source_key`, `source_kind`, `source_etag`, `source_last_modified`, `source_validator_host`, `text_sha256`.

## 2. Banco legado e importação

`open_workspace` importa uma única vez o banco legado de `<app_data_dir>/estuda-uva.sqlite3` para o banco do acervo, via `ATTACH DATABASE`. A cópia lista **colunas explicitamente** por tabela; uma coluna nova em `courses`, `materials`, `study_sessions`, `practice_questions`, `academic_events` ou `settings` não é copiada sem alterar essa lista. `lesson_sessions`, `lesson_interactions`, `study_summaries` e `capture_*` não fazem parte da importação. Detalhes em [banco privado](banco-privado.md#8-importação-do-banco-legado).

## 3. Banco gerado e arquivos de dados

- `build/acervo.sqlite3` **não tem migração**: `gerarSqlite` remove o arquivo e o recria do zero a cada `node tools/acervo/build.mjs` (`tools/acervo/src/sqlite.mjs`). Mudar suas tabelas é mudar o gerador, sem preocupação com dados antigos. Mesmo assim, rodar o gerador cria e sobrescreve arquivos locais e deve ser feito conscientemente.
- Os JSON de `dados/` e `privado/dados/` não têm campo de versão; a validação é por `ESPECIFICACAO` em `tools/acervo/src/validar.mjs`. Os JSON do formato legado (`materias/catalogo/*.json`) levam `schemaVersion: 1`.
- Mudar o formato dos registros é uma migração de **arquivos**: exige atualizar `validar.mjs`, `legado.mjs`, os testes em `tools/acervo/test/` e `dados/LEIA-ME.md`, e preservar `ids` existentes ("Não renomeie ids já existentes", `dados/LEIA-ME.md`).

## 4. Processo proposto para uma nova migração (Proposta)

Este processo ainda não está formalizado em nenhum arquivo do repositório além da regra global do usuário. Ele é uma proposta para os mantenedores.

1. **Justificar.** Registrar por que o dado não cabe no esquema atual e se uma projeção ou arquivo bastaria.
2. **Redigir a proposta, sem executar nada.** Um texto contendo: (a) o **SQL exato** (DDL/DML) ou o código Rust da migração; (b) a **justificativa técnica** e o objetivo; (c) o **impacto estimado** (tabelas afetadas, risco de lock, risco de perda de dado, reversão). Use o modelo da seção 5.
3. **Pedir aprovação no chat** e esperar uma resposta explícita. Aprovação para um SQL não vale para outro. **Não criar arquivo de migração, não editar `storage.rs`, não rodar teste que escreva em banco real antes disso.**
4. **Depois da aprovação**, editar `BASELINE` em `schema.rs` (só `CREATE ... IF NOT EXISTS`, **nunca `ALTER TABLE`**), somar 1 a `CURRENT_VERSION` e ao `PRAGMA user_version` no fim do script, e atualizar `BASELINE_TABLES`. Bancos antigos são regenerados com backup automaticamente.
5. **Testar só em banco temporário.** Os testes de `schema.rs` (sem `ALTER`, idempotência, tabelas listadas) e de `storage.rs` (`fresh_database_uses_the_single_baseline_with_every_table`, `older_database_is_backed_up_whole_and_regenerated_from_the_baseline`) cobrem o mecanismo; acrescente um para a tabela nova. Nenhum teste deve apontar para `.estuda-uva.sqlite3` real.
6. **Atualizar a lista de importação do banco legado** se a tabela nova ou alterada fizer parte dela, e `docs/dados/banco-privado.md`.
7. **Aplicar ao banco real** é abrir o app com o arquivo da pasta do acervo: a abertura **regenera** bancos de versão menor (move o antigo para `*.bak.sqlite3`). Faça com o app fechado e só com nova aprovação explícita.
8. **Reversão.** Não existe migração para baixo. A reversão é restaurar o `*.bak.sqlite3` no lugar de `.estuda-uva.sqlite3`. Documentar isso na proposta.

## 5. Modelo de proposta (sintético)

```markdown
## Proposta de migração N (NÃO EXECUTADA, aguarda aprovação)

**Objetivo:** guardar X porque Y.
**Alternativas descartadas:** projeção em arquivo; coluna em tabela existente.

**SQL exato**
(bloco com o DDL/DML completo)

**Impacto**
- Tabelas afetadas: ...
- Lock: ... (BEGIN IMMEDIATE na abertura do banco)
- Perda de dados: nenhuma / descrição
- Reversão: restaurar cópia anterior

**Testes:** nome do teste com banco temporário.
**Compatibilidade:** banco legado, espelhos, UI.
```

## 6. Propostas de migração já registradas e não aprovadas

O documento `docs/arquitetura/03-dados-e-sincronizacao.md` (§7, arquivo ainda não commitado) contém SQL de dois esquemas novos e vazios, o do acervo público e o do banco privado por perfil. Ele próprio declara: "NÃO EXECUTADO, aguarda sua aprovação" e que a sintaxe não foi validada. **Nada daquele SQL deve ser executado, copiado para arquivo de migração nem usado em testes antes de aprovação explícita.**

## 7. Lacunas

- Não há registro de histórico de migrações (nem tabela, nem documento) além do código e dos testes.
- Não há política de retenção de cópias de segurança.
- `sync_*` e `academic_*` seguem sem marcador próprio, mas regeneram junto com o arquivo quando `CURRENT_VERSION` sobe.
- Não há importação automática do backup `*.bak.sqlite3` para o banco novo (sessões de aula, resumos e capturas antigos ficam só no backup).
- Não há verificação de integridade (`PRAGMA integrity_check`) no código lido.
