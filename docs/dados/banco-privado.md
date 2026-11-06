# Banco privado (`.estuda-uva.sqlite3`)

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

Este documento descreve o banco SQLite que o aplicativo Estuda UVA mantém ao lado do acervo. O esquema abaixo foi extraído do código-fonte, não de um arquivo `.sqlite3`: **nenhum banco foi aberto nem consultado** na preparação desta página. As fontes são `app/crates/estuda-core/src/storage.rs`, `catalog/academic.rs` e `sync.rs`.

Para a visão geral das camadas, veja [visão geral dos dados](visao-geral.md). Para a sincronização, veja [sincronização](sincronizacao.md). Para alterar o esquema, veja [migrações](migracoes.md).

> Esta documentação não autoriza executar SQL, rodar migrações, apagar o banco nem restaurar cópias. Qualquer alteração em banco exige aprovação prévia, explícita e individual do usuário no chat.

## 1. Finalidade

Guardar, no computador do estudante, o estado do aplicativo que não pertence a nenhum arquivo do acervo:

- cursos do Canvas vistos e materiais importados (com origem, hash e versão da fonte);
- execuções de captura do Canvas e seus itens (retomada, cancelamento, repetição);
- sessões de aula, respostas ao tutor local, prática e histórico de estudo;
- eventos de calendário lidos do Canvas;
- resumos por matéria gerados por IA (texto, modelo, tokens);
- a cópia SQLite dos documentos sincronizados, conflitos, exclusões pendentes e fila de projeção;
- a projeção relacional do catálogo curricular (`academic_*`).

Tudo isso é **dado pessoal ou derivado de dado pessoal**. O arquivo do banco nunca é publicável: `crates/estuda-domain/src/publication.rs` classifica `.sqlite3`, `-wal`, `-shm` e `-journal` como `LocalOnly`. A política de 6/11/2026 libera o conteúdo pessoal como texto em `privado/` (camada `Personal`), não o próprio banco.

## 2. Local de armazenamento

| Item | Caminho | Observação |
|---|---|---|
| Banco ativo | `<acervo>/.estuda-uva.sqlite3` | `<acervo>` é a pasta de acervo configurada. Padrão na inicialização do app: a raiz deste repositório (`CARGO_MANIFEST_DIR/../..` em `app/src-tauri/src/lib.rs`, função `run`). |
| Arquivos WAL | `.estuda-uva.sqlite3-wal`, `-shm` | O modo `PRAGMA journal_mode=WAL` é aplicado por `AcademicStore` e por `SyncEngine`. |
| Banco legado | `<app_data_dir>/estuda-uva.sqlite3` | Passado a `AcademicStore::open_workspace` como origem de importação. Veja a seção 8. |
| Espelhos | `<acervo>/.estuda-uva/mirrors/` | Cópias em arquivo dos documentos sincronizados. Veja [sincronização](sincronizacao.md). |
| Configuração `library_path` | tabela `settings`, chave `library_path` | Guarda o caminho absoluto do acervo. Caminho absoluto da máquina do usuário. |

`.gitignore` ignora `/.estuda-uva.sqlite3`, `-wal`, `-shm`, `-journal`, `/.estuda-uva/mirrors/` e `/.estuda-uva/sync-tmp/`.

O mesmo arquivo é aberto por **duas conexões** independentes no processo: a de `AcademicStore` (`storage.rs`, `Mutex<Connection>`) e a de `SyncEngine` (`sync.rs`, `Mutex<Connection>`). Ambas executam `initialize_schema` do catálogo, por isso criam as mesmas tabelas `academic_*`.

## 3. Isolamento entre usuários

**Não existe conceito de usuário no banco nem no aplicativo.** Não há tabela de usuários, coluna de proprietário, autenticação do app nem criptografia do arquivo. O isolamento que existe é por **pasta**: cada pasta de acervo tem seu próprio `.estuda-uva.sqlite3`, e `AcademicStore::set_library_path` troca de pasta (e de banco) quando o estudante escolhe outra.

Consequências:

- Duas pessoas no mesmo computador e na mesma pasta compartilham o banco. O isolamento depende das permissões do sistema operacional sobre a pasta.
- O banco **não é criptografado**. Quem ler o arquivo lê notas, respostas e textos. **Não verificado:** permissões de arquivo aplicadas pelo app (o código não chama `set_permissions` para o banco).
- O contrato de multiusuário descrito em `ARQUITETURA_ACERVO.md` §5 ("cada estudante deve ter uma área de dados pessoais") é **Planejado**; a decisão de onde fica a área pessoal está em aberto.

## 4. Esquema real (versão 8)

`CURRENT_VERSION = 8` (`schema.rs`, `BASELINE`, sem `ALTER TABLE`). O aplicativo grava a versão em `PRAGMA user_version`. Bancos de versão menor não são migrados: são guardados como `*.bak.sqlite3` e regenerados (ver [migrações](migracoes.md)). As tabelas abaixo são as da `BASELINE`.

### 4.1 Estado do aplicativo

| Tabela | Colunas | Chave e índices |
|---|---|---|
| `courses` | `id TEXT`, `title TEXT`, `url TEXT`, `kind TEXT` (`current` ou `past`), `seen_at TEXT` | PK `id` |
| `materials` | `id`, `course_id`, `course_title`, `module_title`, `title`, `path`, `lesson_path` (padrão `''`), `origin`, `pages INTEGER`, `semester` (nulo permitido), `sha256`, `imported_at`; `source_key` (padrão `''`), `source_kind` (padrão `'pdf'`), `source_etag`, `source_last_modified`, `source_validator_host`, `text_sha256` | PK `id`; índice `materials_source_version(source_key, text_sha256, imported_at DESC)` |
| `material_associations` | `material_id`, `course_id`, `course_title`, `module_title`, `item_title`, `semester` (nulo), `created_at` | PK `(material_id, course_id, module_title, item_title)`; índice `material_associations_course(course_id, semester, material_id)` |
| `study_sessions` | `id`, `course_id`, `material_id`, `correct`, `total`, `percent`, `studied_at` | PK `id` |
| `practice_questions` | `material_id`, `position`, `prompt`, `answer`, `source_page` | PK `(material_id, position)` |
| `lesson_sessions` | `id`, `resource_path`, `course_id` (nulo), `course_title`, `material_title`, `section_index`, `section_count`, `started_at`, `updated_at`, `completed_at` (nulo) | PK `id`; índices `lesson_sessions_resource_opened(resource_path, completed_at, updated_at)` e `lesson_sessions_history(updated_at DESC)` |
| `lesson_interactions` | `id`, `session_id`, `activity_id`, `section_index`, `kind`, `prompt`, `response`, `assessment`, `source_page` (nulo), `source_heading`, `source_excerpt`, `source_sha256`, `attempted_at` | PK `id`; índice `lesson_interactions_session_order(session_id, attempted_at, id)` |
| `study_summaries` | `id`, `course_id`, `course_title`, `semester`, `source_hash`, `source_manifest_json`, `short_text`, `long_text`, `model`, `input_tokens`, `output_tokens`, `created_at` | PK `id`; índice `study_summaries_subject_latest(course_id, semester, created_at DESC)` |
| `academic_events` | `id`, `semester`, `course_id` (nulo), `course_title` (nulo), `title`, `starts_at`, `all_day`, `source_kind`, `source_url`, `source_label`, `verified_at` | PK `id`; índice `academic_events_semester_date(semester, starts_at)` |
| `capture_runs` | `id`, `scope`, `limits_snapshot`, `status`, contadores `courses_total`, `courses_done`, `items_total`, `captured`, `reused`, `ignored`, `failed`, `needs_review`, `pending`, posição atual (`current_course_id`, `current_course_title`, `current_module_title`, `current_item_title`), `warning`, `started_at`, `updated_at`, `finished_at` | PK `id`; índice `capture_runs_latest(status, updated_at DESC)` |
| `capture_items` | `run_id`, `position`, `reference_key`, `content_key`, `canvas_item_id`, `course_id`, `course_title`, `module_title`, `title`, `source_kind`, `source_url`, `status`, `retryable`, `attempts`, `material_id`, `message`, `updated_at` | PK `(run_id, reference_key)`; índice `capture_items_course_order(run_id, course_id, position, reference_key)` |
| `settings` | `key`, `value` | PK `key` |
| `sync_runs` | `id`, `status` (`in_progress`, `completed`, `completed_pending_approval`, `cancelled`, `failed`), `steps_json`, `plan_digest`, `plan_json`, `metrics_json`, `started_at`, `updated_at`, `finished_at` | PK `id`; índice `sync_runs_latest(status, updated_at DESC)` |
| `http_validators` | `url_hash`, `url` (sem query), `etag`, `last_modified`, `content_hash`, `byte_len`, `fetched_at` | PK `url_hash` |
| `source_observations` | `id`, `run_id`, `source_system`, `source_url`, `read_at`, `entity`, `entity_key`, `verification` (`CHECK`), `content_hash`, `payload_json` | PK `id`; índice `source_observations_entity(entity, entity_key, read_at DESC)` |
| `ava_checks` | `subject_key`, `term`, `kind` (`ava1`/`ava2`), `existence`, `assessment_type`, `due_at`+`due_at_state`, `submission`+`submission_state`, `grade_hundredths`+`grade_state`, `feedback`+`feedback_state`, `link_confidence` (`explicit`/`probable`), `source_url`, `read_at`; todos os `*_state` com `CHECK` | PK `(subject_key, term, kind)` |
| `exam_records` | `subject_key`, `term`, `name`, `exam_date` (nulo = "a confirmar"), `date_state`, `exam_time`, `location`, `scheduling`, `status`, `source_system`, `source_url`, `read_at` | PK `(subject_key, term, name)` |

Chaves de `settings` encontradas no código: `library_path`, `canvas_capture_limits` (JSON dos limites de captura), `chromium_executable` (caminho do Chromium, gravado por `app/src-tauri/src/lib.rs`) `__legacy_database_imported` (marca a importação única do banco legado) e `__regenerated_from_backup` (caminho do backup quando o banco foi regenerado por ter versão menor).

### 4.2 Projeção do catálogo curricular (`academic_*`)

Criadas por `catalog::academic::initialize_schema` (com `PRAGMA foreign_keys=ON` na conexão). Todas guardam o JSON de origem em `source_json`.

| Tabela | Colunas | Chaves estrangeiras |
|---|---|---|
| `academic_institutions` | `id` | — |
| `academic_programs` | `id`, `institution_id`, `title`, `code`, `source_json` | `institution_id` → `academic_institutions(id)` |
| `academic_curricula` | `id`, `program_id`, `version`, `source_json` | `program_id` → `academic_programs(id)` |
| `academic_disciplines` | `id`, `title`, `identity_status` (nulo), `source_json` | — |
| `academic_offerings` | `id`, `curriculum_id`, `canonical_discipline_id` (nulo), `code`, `title`, `workload_hours INTEGER`, `status`, `source_json` | `curriculum_id` → `academic_curricula(id)`; `canonical_discipline_id` → `academic_disciplines(id)` |
| `academic_discipline_offerings` | `discipline_id`, `offering_id` | ambas as colunas; PK composta |
| `academic_relations` | `id`, `relation_type`, `cc_offering_id`, `ec_offering_id`, `canonical_discipline_id` (todos nulos permitidos), `equivalence_confirmed` (`CHECK` 0 ou 1), `source_json` | três referências a `academic_offerings` e `academic_disciplines` |
| `academic_trajectories` | `id`, `program_id`, `curriculum_id` (nulo), `status`, `semester` (nulo), `source_json` | `program_id`, `curriculum_id` |
| `academic_enrollments` | `trajectory_id`, `id`, `title`, `section` (nulo), `start_date` (nulo), `status`, `semester` (nulo), `source_json` | `trajectory_id`; PK `(trajectory_id, id)` |
| `academic_outcomes` | `trajectory_id`, `position`, `course_title`, `status`, `source_json` | `trajectory_id`; PK `(trajectory_id, position)` |

Índices: `academic_offerings_by_curriculum(curriculum_id, code)` e `academic_offerings_by_discipline(canonical_discipline_id)`.

**Estas tabelas são uma projeção descartável.** `replace_projection` apaga todas e reinsere a partir dos cinco JSON do formato legado dentro de uma transação, a cada reconciliação. Edição manual nelas é sobrescrita (`docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`). Uma falha preserva o estado anterior e vira aviso em `SyncReport.errors`.

### 4.3 Tabelas da sincronização

Descritas em [sincronização](sincronizacao.md): `sync_documents`, `sync_conflicts`, `sync_tombstones`, `sync_outbox`.

### 4.4 Relacionamentos lógicos sem chave estrangeira

As tabelas de estado do aplicativo (4.1) **não declaram chaves estrangeiras**. As ligações abaixo são verificadas, quando são, pelo código Rust:

- `materials.course_id` → `courses.id`; `material_associations.material_id` → `materials.id`; `practice_questions.material_id` e `study_sessions.material_id` → `materials.id`.
- `lesson_interactions.session_id` → `lesson_sessions.id`: `save_lesson_interaction` rejeita sessão desconhecida (teste `lesson_interaction_rejects_unknown_sessions_mismatched_activities_and_oversized_answers`).
- `capture_items.run_id` → `capture_runs.id`; `capture_items.material_id` → `materials.id`.
- `materials.source_key` identifica a fonte de forma estável: `content_key` em `canvas.rs` produz `pdf:<sha256>` ou `canvas_page:<sha256>` a partir do identificador Canvas ou da URL sem query.

## 5. Dados pessoais armazenados

| Dado | Onde | Sensibilidade |
|---|---|---|
| Respostas do estudante ao tutor e à prática | `lesson_interactions.response`, `study_sessions` | pessoal |
| Progresso de aula e histórico de estudo | `lesson_sessions`, `study_sessions` | pessoal |
| Nomes e URLs de cursos e materiais, `semester` | `courses`, `materials`, `material_associations`, `capture_*` | pessoal (matrícula inferida) |
| Textos gerados por IA a partir de materiais | `study_summaries` | derivado; veja nota abaixo |
| Texto integral de capturas do Portal/Lyceum e do Canvas (notas, frequência, avisos, mensagens) | `sync_documents.content`, `sync_conflicts`, `sync_tombstones`, `sync_outbox` | **altamente pessoal** |
| Caminho absoluto do acervo e do Chromium | `settings` | pessoal (revela o nome de usuário do sistema) |

Nota sobre `study_summaries`: o texto vem de uma chamada à API da Anthropic (`anthropic.rs`, `study_summaries.rs`). O banco guarda a saída, o modelo e as contagens de tokens, não a chave. **Não verificado nesta rodada:** o conteúdo exato enviado à API; veja a documentação de automações.

As capturas entram no banco porque `materias/catalogo/capturas/` fica dentro da árvore sincronizada. `is_sensitive_or_excluded` só filtra por nome de segmento (`senha`, `token`, `cookie` etc.) e por `originais/`; ele **não** exclui `capturas/`. Por isso o conteúdo das capturas existe em três lugares: o arquivo original, a cópia em `sync_documents` e o espelho em `.estuda-uva/mirrors/`. Todos ignorados pelo Git.

## 6. Leitura

Toda leitura passa por `AcademicStore` (métodos `list_*`, `find_*`, `get_*`, `overview`, `latest_*`) ou por `SyncEngine` (`read_content`, `open_conflicts`, `read_academic_catalog`). Os comandos Tauri em `app/src-tauri/src/lib.rs` expõem esses métodos à interface. Nenhuma leitura grava em sistemas da UVA. A interface apresenta o catálogo curricular lido de `academic_*` após reconciliar os JSON (`catalog::academic::read_projection`).

## 7. Persistência autorizada

O banco só é escrito por estes caminhos, todos iniciados por ação do estudante ou pelo monitor de arquivos:

1. **Captura e importação** (`save_courses`, `save_material`, `save_material_association`, `create_capture_run`, `upsert_capture_item`, `update_capture_run`, `set_material_semester`, `replace_course_calendar_events`).
2. **Estudo** (`begin_lesson_session`, `save_lesson_progress`, `save_lesson_interaction`, `record_session`, `save_questions`).
3. **Resumos** (`save_study_summary`), somente depois de confirmação com `confirmed_source_hash`.
4. **Configuração** (`set_setting`, `set_library_path`).
5. **Sincronização** (`SyncEngine::reconcile_all`, `create_content`, `set_content`, `resolve_conflict`), que também reescreve arquivos do acervo.

O aplicativo **não escreve em sistemas institucionais**: o código de captura é somente leitura (veja a documentação de automações). Atividade de estudo registrada no banco não altera frequência, notas nem matrícula na UVA.

Exclusões efetivas no código: `DELETE` de `academic_events` por semestre e curso (`replace_course_calendar_events`), de `practice_questions` por material (`save_questions`), e a substituição completa de `academic_*` e de `sync_tombstones`/`sync_conflicts` por caminho. **Não há função de apagar materiais, sessões, interações ou resumos.**

## 8. Importação do banco legado

`AcademicStore::open_workspace(legacy_path, default_library)` (`storage.rs`):

1. Lê, em modo somente leitura, a chave `library_path` do banco legado (`legacy_library_path`). Se apontar para uma pasta existente, ela é o acervo; senão vale o padrão.
2. Abre ou cria `<acervo>/.estuda-uva.sqlite3`.
3. Se o banco legado existe, é outro arquivo e `settings.__legacy_database_imported` não vale `1`: executa `ATTACH DATABASE`, copia com `INSERT OR IGNORE` as tabelas `courses`, `materials`, `study_sessions`, `practice_questions`, `academic_events` e `settings`, grava `__legacy_database_imported=1`, refaz `source_key` onde faltam (`backfill_missing_material_sources`), grava `user_version = 7` e faz `COMMIT`; em erro faz `ROLLBACK`; sempre faz `DETACH`.

A importação **não** copia `lesson_sessions`, `lesson_interactions`, `study_summaries` nem `capture_*` do banco legado. O teste `legacy_import_preserves_practice_history_and_current_schema_version` cobre o histórico de prática.

## 9. Backup, restauração e exclusão

| Operação | O que existe | Estado |
|---|---|---|
| Backup | `Backup` da API do SQLite é usada **uma única vez**, em `set_library_path`, para copiar o banco ativo para o novo `.estuda-uva.sqlite3` quando o destino não existe ou está vazio (`storage.rs`). Não há comando de backup para o estudante. | Parcial |
| Backup manual recomendado | copiar `.estuda-uva.sqlite3` **junto** de `-wal` e `-shm` com o app fechado, ou usar o `.backup` do `sqlite3` | Proposta (não é recurso do app) |
| Restauração | não existe comando. Restaurar significa colocar um arquivo `.estuda-uva.sqlite3` na pasta do acervo com o app fechado. Um banco com `user_version` maior que 8 é recusado e deixado intacto (testes `unsupported_destination_schema_is_left_untouched` e `newer_database_is_refused_and_left_byte_identical`); um menor é movido para `*.bak.sqlite3` e regenerado. | Não implementado como recurso |
| Exclusão controlada | não existe comando. Os dados do aplicativo são reconstruíveis só em parte: `academic_*`, `sync_documents` e espelhos se refazem dos arquivos; progresso de estudo, respostas, resumos e execuções de captura **não** se refazem. | Não implementado como recurso |
| Exportação | não existe | Não implementado |

Apagar ou substituir o arquivo do banco é uma alteração de banco: exige aprovação prévia explícita.

## 10. Proteção contra vazamento

| Camada | Mecanismo |
|---|---|
| Git | `.gitignore` ignora os arquivos do banco, WAL/SHM, espelhos, capturas, `build/` e `originais/` |
| Contrato de publicação | `classify` marca bancos como `LocalOnly` e capturas como `Personal` ([visão geral](visao-geral.md), seção 3). **Planejado:** ainda não integrado a nenhum fluxo |
| Registros de dados | `validar.mjs` recusa chaves e valores sensíveis em `dados/` e `privado/dados/` |
| Sincronização | caminhos sensíveis por nome e `originais/` não entram; links simbólicos e `..` são recusados |
| Capturas | links gravados perdem query e fragmento (`sanitize_source_url`); o valor de campos cujo rótulo é sensível (nome, CPF, RA, matrícula, identificação, documento, aluno, estudante, discente) vira `[identificador omitido]` e o rótulo de links assim vira `Identificador omitido` (`lyceum.rs`, `is_sensitive_label`). O filtro é por rótulo: dado pessoal em campo com outro rótulo não é detectado |
| Materiais | URL de origem é guardada sem query (`sanitize_source_url`, teste `capture_item_source_url_drops_query_and_fragment_on_persist`); validadores `etag`/`last-modified` são higienizados |
| Credenciais | senha, cookies e URLs assinadas ficam só em memória; a chave da API não é gravada no banco |

Riscos residuais conhecidos: o arquivo do banco não é criptografado; `settings` contém caminhos absolutos; capturas existem em três cópias locais (ver lacuna L9 em [visão geral](visao-geral.md)).

## 11. Testes associados

Em `app/crates/estuda-core/src/storage.rs`, módulo `tests` (19 testes de banco, entre eles): `fresh_database_uses_the_single_baseline_with_every_table`, `reopening_keeps_rows_and_does_not_regenerate`, `older_database_is_backed_up_whole_and_regenerated_from_the_baseline`, `unversioned_database_with_tables_is_treated_as_old_and_empty_one_is_not`, `newer_database_is_refused_and_left_byte_identical`, `engine_tables_refuse_invented_verification_states` (mais os 3 de `schema.rs`), `legacy_import_preserves_practice_history_and_current_schema_version`, `selecting_a_workspace_with_an_existing_database_preserves_its_local_state`, `unsupported_destination_schema_is_left_untouched`. Em `catalog/academic.rs`: `reconcile_materializes_shared_curriculum_entities_in_normalized_sqlite_tables`. Todos usam bancos temporários (`tempfile`).

**Resultado de execução:** este documento não afirma que a suíte passou; o resultado das execuções fica na documentação de testes do desenvolvimento.
