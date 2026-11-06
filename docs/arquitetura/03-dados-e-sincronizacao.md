# 03 — Dados, bancos e sincronização

> **Trava de banco:** tudo em §7 é **proposta**. Nenhum DDL, DML, migração ou abertura de banco para escrita foi executado. Os bancos foram lidos apenas com `immutable=1`.

## 1. Três domínios

| Domínio | Contém | Onde fica | Versionado no Git |
|---|---|---|---|
| **A. Pessoal** (por aluno; publicável, separado) | perfil, matrículas, notas, frequência, progresso, anotações, prazos, operações pendentes, conflitos | `<app_data>/perfis/<perfil>/estudante.sqlite3` | sim (política 6/11/2026), exceto o banco |
| **B. Acervo** (público) | catálogo, currículos, disciplinas, ementas, conteúdos, exercícios, referências, licenças, procedência | `biblioteca/` (texto) e `acervo-publico.sqlite3` (derivado, em snapshots) | texto sim; banco como artefato de release |
| **C. Plataforma** | storage, cache, busca, sync, import/export, automação do navegador, configurações, CLI, GPUI | crates (02) | código |

Materiais do Portal/Canvas entram primeiro em A. Por decisão do dono (6/11/2026), todo conteúdo pode ir ao acervo público: a promoção a B segue o manifesto de 04 §2, sem exigir autorização adicional, mas registrando fonte e licença/origem. Dado pessoal (notas, matrículas, trabalhos) também pode ser publicado, e permanece em A (`privado/`), nunca misturado a registro de B.

## 2. Entidades e identidade

| Entidade | Domínio | Identidade | Observação |
|---|---|---|---|
| `DegreeProgram` | B | `uva:program:<slug>` (`DegreeProgramId`) | curso de graduação do catálogo |
| `Subject` | B | `uva:discipline:<slug>` (`SubjectId`) | disciplina do catálogo |
| `SubjectOffering` | B | `<subject>@<term>/<turma>` | oferta/turma; sem nome de docente em B |
| `AcademicTerm` | B | `uva:term:2026.2` (`AcademicTermId`) | período letivo |
| `LearningMaterial`, `Assessment` | B | `sha256` do texto normalizado + `subject` | procedência obrigatória |
| `AcademicEnrollment` | A | UUID local | vínculo pessoal com `DegreeProgramId`; uma pessoa pode ter várias, sempre separadas |
| `StudentSubject` | A | UUID local | liga `SubjectId` + `AcademicEnrollment` + `LmsCourseId?`; **é a única ponte** |
| `StudentAssessment` | A | UUID local | valor + `Confidence` + fonte + data |
| `StudyProgress` | A | `(StudentSubject, material sha256)` | |
| `LmsCourse` | A/C | `canvas:<id numérico>` (`LmsCourseId`) | ambiente do LMS; **nunca** vira `Subject` sozinho |

Regras de identidade:

1. Equivalência entre disciplinas só existe por **relação explícita com fonte** (hoje `dados/relacoes/`, todas candidatas). Semelhança de nome nunca liga entidades.
2. Curso de apoio do Canvas (`tipo: apoio`) não gera `Subject`. O tipo `LmsCourseId` não converte para `SubjectId`; a conversão passa por `StudentSubject`, criada por uma pessoa ou por regra com confirmação em fonte oficial.
3. CC e EC permanecem em `AcademicEnrollment` distintos; nota de um não migra para o outro sem relação confirmada.
4. IDs existentes (`uva:discipline:*`, `uva:program:*`) **não são renomeados**; o crate `estuda-domain` os aceita como estão (testado com os formatos reais).

## 3. Estados do dado

Todo valor acadêmico pessoal carrega `Confidence` (`Confirmed`, `Estimated`, `Unknown`, `Conflicting`), fonte (arquivo/seção ou Portal/Canvas) e data. `None` + pendência para ausente; nunca `0` ou `""`. Duas observações do mesmo campo se combinam por `Confidence::reconcile`: valores diferentes viram `Conflicting` e nenhuma vence sozinha. A nota final ponderada de `grading.rs` é `Estimated` e não pode alimentar campo oficial.

## 4. Separação física dos bancos

- **Privado** (`estudante.sqlite3`): em `app_data_dir`, modo `0700` no diretório e `0600` no arquivo, um diretório por perfil, backup e restauração próprios (`VACUUM INTO`), fora de qualquer `git add`.
- **Acervo** (`acervo-publico.sqlite3`): somente conteúdo apto, aberto `SQLITE_OPEN_READ_ONLY`, com tabela `library_meta` (identidade, `snapshot_version`, `built_from_manifest_sha256`). Sem BLOB: texto em Markdown/UTF-8.
- **Sem chave estrangeira entre bancos.** O privado guarda `subject_id TEXT` e `library_snapshot TEXT`; ao abrir, o storage verifica se cada `subject_id` existe no snapshot e reporta órfãos como aviso, nunca como erro fatal (offline e snapshots antigos são normais).

Mapeamento das tabelas atuais (somente classificação, nada movido):

| Tabela atual | Destino |
|---|---|
| `academic_institutions`, `academic_programs`, `academic_curricula`, `academic_disciplines`, `academic_offerings`, `academic_relations` | **B** (após remover campos de docente e disciplina pessoal) |
| `academic_trajectories`, `academic_enrollments`, `academic_outcomes`, `academic_events`, `academic_discipline_offerings` | **A** |
| `study_sessions`, `lesson_sessions`, `lesson_interactions`, `practice_questions` | **A** |
| `courses`, `materials`, tabelas de captura | **A** (LMS), promoção a B só por manifesto |
| `sync_documents`, `sync_outbox`, `sync_conflicts`, `sync_tombstones`, `settings` | **A** (um conjunto por banco que sincroniza) |

## 5. Contrato de sincronização

1. **Uma origem autoritativa por tipo:**

   | Tipo | Autoritativo | Derivados |
   |---|---|---|
   | Catálogo e currículo (B) | `dados/*.json` / `biblioteca/**` (texto) | `acervo-publico.sqlite3`, `materias/catalogo/*.json`, visualizações |
   | Conteúdo didático (B) | Markdown em `biblioteca/` | índice de busca, SQLite |
   | Registro pessoal (A) | `estudante.sqlite3` do perfil | espelhos Markdown/JSON locais (`.estuda-uva/mirrors/`), regeneráveis |
   | Edição humana de Markdown pessoal | o arquivo | linha no SQLite |

2. **Sem ciclo:** cada arquivo tem exatamente um sentido de escrita. `materias/catalogo/*.json` passa a ser **somente leitura para o app** (hoje é bidirecional no Rust e regenerado pelo Node: ver 01 §3). Isto exige teste antes de mudar.
3. **Idempotência:** reaplicar a mesma importação não altera hash nem `revision`. Chave natural = `(fonte, id externo, sha256)`.
4. **Conflito:** divergência entre `base_content`, arquivo e banco grava linha em `sync_conflicts` com as três versões e para; revisão humana decide. Já existe no Rust; mantido.
5. **Falha:** o `sync_outbox` registra hash esperado antes de escrever e confirma depois; na reabertura, `pending` é reaplicado ou revertido. Já existe; mantido.
6. **Proveniência:** toda linha de A e B guarda `source` (arquivo/seção ou URL sem query), `observed_at` e `confidence`.
7. **Teste exigido antes de implementar:** editar `materias/catalogo/disciplinas.json` pelo app, rodar `node tools/acervo/build.mjs` num **clone temporário** e comprovar perda ou não. Resultado decide se o app deve parar de escrever ali.

## 6. Texto como forma principal

Markdown (leitura), JSON (intercâmbio), SQLite (índice/consulta). Origem registrada por `{fonte, páginas, sha256 do original, data}`; o original fica fora do Git. Conteúdo ilegível é marcado `não extraível`, nunca transcrito por suposição. Sem BLOB em B. A extração já opera em memória (`archive.rs` valida PDF e descarta os bytes) e permanece assim.

## 7. SQL proposto — **NÃO EXECUTADO, aguarda sua aprovação**

### 7.1 Justificativa e impacto

**Sintaxe não validada:** por causa da trava, nem em memória executei este SQL. Valido a sintaxe (em `:memory:`) quando você autorizar isso.

Criar dois esquemas novos e vazios. **Nenhuma tabela existente é alterada ou removida**, e nenhum dado existente é lido para escrita. Impacto: 2 arquivos novos; zero lock no banco atual; zero perda de dados. Risco: baixo enquanto os esquemas novos não forem consumidos pelo app.

### 7.2 Banco do acervo público (`acervo-publico.sqlite3`)

```sql
PRAGMA user_version = 1;
PRAGMA foreign_keys = ON;

CREATE TABLE library_meta (
  key   TEXT PRIMARY KEY,
  value TEXT NOT NULL
) STRICT;                                   -- identity, snapshot_version, built_at, manifest_sha256

CREATE TABLE source (
  id           TEXT PRIMARY KEY,            -- sha256 do original
  kind         TEXT NOT NULL CHECK (kind IN ('official','student','public','textbook')),
  citation     TEXT NOT NULL,
  license      TEXT NOT NULL,               -- 'CC-BY-4.0', 'proprietary-private', ...
  reviewed_by  TEXT NOT NULL,
  reviewed_at  TEXT NOT NULL
) STRICT;

CREATE TABLE degree_program (
  id TEXT PRIMARY KEY, title TEXT NOT NULL, institution TEXT NOT NULL
) STRICT;

CREATE TABLE subject (
  id TEXT PRIMARY KEY, title TEXT NOT NULL, syllabus_md TEXT
) STRICT;

CREATE TABLE curriculum_item (
  program_id TEXT NOT NULL REFERENCES degree_program(id),
  subject_id TEXT NOT NULL REFERENCES subject(id),
  series INTEGER, workload_hours INTEGER,
  PRIMARY KEY (program_id, subject_id)
) STRICT;

CREATE TABLE subject_relation (
  subject_a TEXT NOT NULL REFERENCES subject(id),
  subject_b TEXT NOT NULL REFERENCES subject(id),
  kind TEXT NOT NULL CHECK (kind IN ('prerequisite','equivalent_confirmed','overlap_candidate')),
  source_id TEXT NOT NULL REFERENCES source(id),
  PRIMARY KEY (subject_a, subject_b, kind)
) STRICT;

CREATE TABLE learning_material (
  id TEXT PRIMARY KEY,                      -- sha256 do texto normalizado
  subject_id TEXT NOT NULL REFERENCES subject(id),
  kind TEXT NOT NULL CHECK (kind IN ('lesson','summary','exercise','exam','assignment','solution','reference')),
  title TEXT NOT NULL,
  body_md TEXT NOT NULL,
  pages TEXT,
  source_id TEXT NOT NULL REFERENCES source(id)
) STRICT;

CREATE VIRTUAL TABLE material_fts USING fts5(title, body_md, content='learning_material', content_rowid='rowid');
CREATE INDEX idx_material_subject ON learning_material(subject_id, kind);
```

### 7.3 Banco privado (`estudante.sqlite3`)

```sql
PRAGMA user_version = 1;
PRAGMA foreign_keys = ON;

CREATE TABLE profile (id TEXT PRIMARY KEY, display_name TEXT NOT NULL, created_at TEXT NOT NULL) STRICT;

CREATE TABLE enrollment (
  id TEXT PRIMARY KEY,
  program_id TEXT NOT NULL,                 -- DegreeProgramId; sem FK: está em outro banco
  library_snapshot TEXT,
  status TEXT NOT NULL, source TEXT NOT NULL, observed_at TEXT NOT NULL
) STRICT;

CREATE TABLE student_subject (
  id TEXT PRIMARY KEY,
  enrollment_id TEXT NOT NULL REFERENCES enrollment(id),
  subject_id TEXT NOT NULL,                 -- SubjectId; sem FK
  term_id TEXT NOT NULL,
  lms_provider TEXT, lms_course_id TEXT,    -- LmsCourseId opcional
  section TEXT, status TEXT NOT NULL,
  UNIQUE (enrollment_id, subject_id, term_id)
) STRICT;

CREATE TABLE student_assessment (
  id TEXT PRIMARY KEY,
  student_subject_id TEXT NOT NULL REFERENCES student_subject(id),
  label TEXT NOT NULL,
  value REAL,                               -- NULL = desconhecido
  confidence TEXT NOT NULL CHECK (confidence IN ('confirmed','estimated','unknown','conflicting')),
  source TEXT NOT NULL, observed_at TEXT NOT NULL
) STRICT;

CREATE TABLE study_progress (
  student_subject_id TEXT NOT NULL REFERENCES student_subject(id),
  material_id TEXT NOT NULL,                -- id em learning_material; sem FK
  state TEXT NOT NULL, updated_at TEXT NOT NULL,
  PRIMARY KEY (student_subject_id, material_id)
) STRICT;

CREATE TABLE sync_conflict (
  path TEXT PRIMARY KEY, base TEXT, local TEXT, remote TEXT, detected_at TEXT NOT NULL
) STRICT;
CREATE TABLE sync_outbox (
  path TEXT PRIMARY KEY, expected_hash TEXT NOT NULL, projection TEXT NOT NULL,
  revision INTEGER NOT NULL, state TEXT NOT NULL DEFAULT 'pending'
) STRICT;
```

### 7.4 Rollback e backup

- **Rollback:** apagar os dois arquivos novos. O banco atual não é tocado.
- **Backup antes de qualquer etapa que leia o banco atual para copiar dados** (etapa 4 do plano, separada): `sqlite3 estuda-uva.sqlite3 ".backup '<destino>'"` fora do repositório, `PRAGMA integrity_check` no backup, e comparação de contagem de linhas por tabela antes e depois.
- **Cópia de dados** (INSERT…SELECT do banco atual para os novos) é um segundo SQL que só escreverei **depois** de você aprovar este, com contagens esperadas por tabela.

### 7.5 Pergunta aberta que muda o SQL

`user_version=0` no banco do app e `5` no legado da raiz. Antes de qualquer cópia preciso saber qual dos dois é o seu banco real de uso.
