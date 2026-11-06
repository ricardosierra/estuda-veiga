# 01 — Diagnóstico

Cada afirmação marca como foi obtida: **[medido]** por comando nesta sessão, **[lido]** no código ou documento, **[inferido]** por raciocínio, sem teste.

## 1. Estrutura real

- **Um pacote Rust rastreado**, `app/src-tauri` (`estuda-uva`, edição 2024), sem workspace e sem `Cargo.toml` na raiz **[medido]**. Durante a auditoria apareceu, não rastreado, um workspace GPUI em `app/Cargo.toml` (criado 00:46 por outro processo) com `estuda-core` (cópia de 16.881 linhas dos módulos de `src-tauri`, ainda monolítica) e `estuda-gpui`; `src-tauri` está em `exclude` **[medido]**. As contagens abaixo são de `src-tauri`. Existe um segundo `Cargo.toml` em `.superpowers/sdd/.../parser-probe`, que é experimento.
- **18.693 linhas de Rust** em 19 módulos de topo + 5 em `catalog/`; **3.401 linhas de TS/React** **[medido]**.
- Maiores arquivos: `storage.rs` 3.195, `lib.rs` 2.240, `sync.rs` 1.485, `browser.rs` 1.107, `canvas_api.rs` 1.103 **[medido]**.
- **Acoplamento ao Tauri está concentrado**: só `lib.rs` (67 referências, 50 `#[tauri::command]`) e `study_summaries.rs` (4 referências, 2 comandos) **[medido]**. Os demais arquivos Rust não importam `tauri`. Isso torna a extração para crates barata: o domínio já está, em grande parte, desacoplado da interface.
- Grafo `use crate::` (nível de módulo): `storage` é o hub (depende de `calendar`, `canvas`, `canvas_capture`, `catalog`, `lesson`, `tutor`); `sync` só depende de `catalog`; `canvas_api` e `lyceum` dependem de `sync`. **Não encontrei ciclos** nesse nível **[lido]**.

## 2. Bancos e formatos

| Artefato | Onde | Versão | Papel |
|---|---|---|---|
| Banco legado do repositório | `.estuda-uva.sqlite3` (6,5 MB, ignorado pelo Git) | `user_version=5` **[medido, imutável]** | Caminho antigo; o código atual não o abre no `run()` |
| Banco real do app | `~/Library/Application Support/br.edu.uva.estuda/estuda-uva.sqlite3` | `user_version=0` **[medido, imutável]** | Aberto por `AcademicStore::open_workspace` (`lib.rs:1776`). `0` sugere banco ainda vazio ou nunca migrado; **não afirmo mais que isso** |
| Banco gerado pelo Node | `build/acervo.sqlite3` (descartável) | `0`, 12 tabelas | Recriado a cada `node tools/acervo/build.mjs` |

O código define `CURRENT_SCHEMA_VERSION = 7` (`storage.rs:18`). O banco legado está em 5 e a documentação (`docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`) diz 4: **o documento está defasado** **[medido/lido]**. **Atualização 6/11/2026:** `CURRENT_SCHEMA_VERSION` passou a 8 (`schema.rs`, baseline única sem `ALTER`). A próxima abertura pelo app não migra o legado: move o arquivo v5 para `*.bak.sqlite3` e regenera, o que continua sendo mudança de arquivo e exige sua aprovação.

O esquema do banco do app mistura em um só arquivo, com 23 `CREATE TABLE` em `storage.rs`:

- **catálogo compartilhável:** `academic_institutions`, `academic_programs`, `academic_curricula`, `academic_disciplines`, `academic_offerings`, `academic_relations`;
- **pessoal:** `academic_trajectories`, `academic_enrollments`, `academic_outcomes`, `study_sessions`, `lesson_sessions`, `lesson_interactions`, `practice_questions`;
- **infraestrutura de sync:** `sync_documents`, `sync_outbox`, `sync_conflicts`, `sync_tombstones`;
- **Canvas/LMS:** `courses`, `materials`, tabelas de captura.

Chaves estrangeiras internas (por exemplo `academic_enrollments.trajectory_id → academic_trajectories`) ligam dado pessoal ao catálogo no mesmo arquivo. Esse é o acoplamento que impede publicar o catálogo sem levar dado de pessoa.

## 3. Fonte da verdade — o que é e o que não é verdade hoje

O estado documentado dizia: JSON é a origem e SQLite/Markdown são derivados. **Isso já não descreve o repositório inteiro.** Há dois pipelines:

1. **Node** (`tools/acervo`): `dados/*.json` (234 registros, validados em 0,58 s **[medido]**) → gera `materias/catalogo/*.json` (ignorado, "gerado; não editar") e `build/acervo.sqlite3`. Mão única.
2. **Rust** (`sync.rs`): trata documentos JSON/Markdown como bidirecionais com `sync_documents` (`base_content`, `revision`, `content_hash`, `sync_outbox`, `sync_conflicts`) e lê `materias/catalogo/*.json` como entrada **[lido]**.

**Risco de ciclo/perda [inferido, não testado]:** se o Rust escreve de volta em `materias/catalogo/*.json` e o Node o regenera a partir de `dados/`, a edição feita pelo app é sobrescrita na próxima geração. As duas ferramentas declaram autoridades diferentes sobre o mesmo arquivo. Antes de qualquer mudança, é preciso um teste que prove ou descarte isso (ver 03 §5).

## 4. Dados pessoais no Git

**Nota (política de 6/11/2026):** versionar esses dados é permitido. O ponto relevante passa a ser a *mistura* com o catálogo, não a presença no Git.

No HEAD, **39 arquivos rastreados** são pessoais: `dados/matriculas/` (12), `dados/notas/` (12), `dados/trajetorias/` (2), `dados/trabalhos/` (7), `dados/meta.json`, `REGISTRO_ACADEMICO.md`, `PLANEJAMENTO.md`, `cursos/**` (3) **[medido via `git ls-tree`]**. Além disso, 12 das 88 disciplinas em `dados/disciplinas/` são pessoais (`identityStatus: matricula_registrada`) e 14 registros em `dados/canvas/` descrevem as matrículas. O histórico tem 42 commits, então esses arquivos continuam no histórico mesmo depois de movidos.

## 5. Mudança concorrente observada

Durante a auditoria, outro processo (não esta sessão) criou `tools/acervo/src/separar-privado.mjs` (não rastreado) e aparentemente o executou com `--aplicar`: surgiu `privado/` na raiz e o working tree passou a ter **87 arquivos removidos e 186 modificados** não commitados **[medido]**. O plano dessa ferramenta move dados pessoais para `privado/` **dentro do mesmo repositório**.

Com a política de 6/11/2026 (dado pessoal pode ser público, desde que separado), `privado/` dentro do repositório **é aceitável** e esta mudança está alinhada. O risco que resta é técnico: o plano move Markdown e JSON que o app e o gerador referenciam; quem a executou precisa validar links e testes. Não a revertí nem a continuei.

Ver também o workspace GPUI em §1: é a migração de interface rodando em paralelo e não foi tocada. O `estuda-core` copia `storage.rs` (3.195 linhas) inteiro, então o acoplamento privado/público de §2 passa a existir em dois lugares até a divisão de banco.

## 6. Problemas de modelagem

1. **`Course` significa três coisas.** Em `canvas.rs`, `Course` é curso do Canvas; a tabela `courses` e `materials.course_id` seguem isso; em `academic_*`, "curso" é programa de graduação e `academic_enrollments.id` é identificador de turma/Canvas. Sem tipo próprio, nada impede passar um curso de apoio do Canvas onde se espera disciplina.
2. **`slug` como identidade.** Os IDs `uva:discipline:<slug>` derivam do nome. Existe divergência real: `materias/paradigmas-de-programacao/` versus o registro canônico `paradigmas-de-linguagens-de-programacao` (já listada em `ARQUITETURA_ACERVO.md`).
3. **Catálogo e vínculo pessoal no mesmo registro** em `dados/disciplinas/` e em `cursos/engenharia-de-computacao/README.md`.
4. **Materiais por `course_id` do Canvas.** `materials` guarda `course_id`/`course_title` do LMS e só depois associa período; o conteúdo didático depende do LMS em vez de depender de `Subject`.
5. **Sem tipo para estado do dado.** "Confirmado, estimado, desconhecido, conflitante" é convenção de texto em `dados/LEIA-ME.md` e nos JSON, não um tipo. O `grading.rs` calcula nota estimada sem um marcador tipado que a impeça de ser tratada como oficial.

## 7. Linha de base de desempenho (para comparar depois)

| Métrica | Valor | Como |
|---|---|---|
| Testes Rust existentes | 152 passaram, 0 falharam, 2,96 s | `cargo test --offline --lib` em `app/src-tauri` |
| Tempo total do comando com build incremental | 33,8 s | mesmo comando |
| Validação do acervo | 0,58 s, 234 registros | `node tools/acervo/build.mjs --validar` |
| Banco legado | 6,5 MB | `ls -l` |
| Texto extraído (`fontes-uva/`) | 40 MB | `du -sh` |

Nenhuma melhoria de desempenho foi medida: esta etapa não altera caminho de execução do app.
