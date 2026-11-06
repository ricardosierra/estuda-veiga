# Convenções de código

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada.

Este documento registra convenções **observadas no código existente** e as regras de `AGENTS.md`. Convenções de arquivos, slugs, JSON e Markdown do acervo estão em [referência/formatos](../referencia/formatos.md) e em [`dados/LEIA-ME.md`](../../dados/LEIA-ME.md). Onde algo é preferência sem comprovação no código, isso está dito.

## Regras do repositório (`AGENTS.md`)

Resumo; o texto normativo está em [`AGENTS.md`](../../AGENTS.md) e não é reproduzido aqui:

- Preservar originais acadêmicos; não sobrescrever nem mover.
- Notas, frequência, documentos e credenciais são dados privados; nunca em Markdown, JSON, logs ou Git.
- Edição de registro acadêmico mantém SQLite, Markdown e JSON sincronizados e preserva conflitos para revisão humana.
- Dado confirmado por fonte oficial é diferente de estimativa, material didático e campo desconhecido; registrar fonte e data.
- Conteúdo versionado é texto; binários ficam fora do Git.
- Dados pessoais do aluno ficam em `privado/`; `dados/` nunca contém dado pessoal.

## Rust

| Tema | Convenção observada | Onde se vê |
|---|---|---|
| Edição e versão | `edition = "2024"`, `rust-version = "1.90"` | `app/Cargo.toml` |
| Versões de dependência | Dependências sensíveis fixadas com `=` (por exemplo `chromiumoxide = "=0.9.1"`, `pdf-extract = "=0.12.1"`, `zip = "=0.6.6"`) | `estuda-core/Cargo.toml` |
| Erros de biblioteca | `thiserror`, enums por módulo, mensagens **em português** que não vazam segredos (`StorageError`, `ArchiveError`, `DownloadError`, `BrowserError`, `PdfError`, `AnthropicError`) | respectivos módulos |
| Erros na fronteira | `Result<_, String>` nos módulos de sincronização/scanner e em todos os comandos Tauri | `sync.rs`, `scanner.rs`, `src-tauri/src/lib.rs` |
| Nomes | Identificadores em inglês, `snake_case`/`CamelCase`; textos para o usuário e comentários de domínio em português | geral |
| Serialização para a UI | `#[serde(rename_all = "camelCase")]` nas structs enviadas à interface; `snake_case` em enums de estado (`GradeStatus`, `DisciplineKind`) e valores como `waiting_for_login` | `storage.rs`, `canvas_capture.rs`, `grading.rs` |
| Núcleo sem UI | `estuda-core` não importa Tauri nem GPUI (declarado no comentário de `lib.rs`) | `estuda-core/src/lib.rs` |
| Funções puras separadas de I/O | Parsers e regras (`canvas.rs`, `canvas_api.rs`, `grading.rs`, `calendar.rs`) não fazem I/O; a rede fica em `browser.rs`/`download.rs` | idem |
| Concorrência | `std::sync::Mutex` para estado síncrono; `tokio::sync::Mutex` e `watch` para estado assíncrono e cancelamento; trabalho pesado em `spawn_blocking` | `storage.rs`, `lib.rs` (Tauri) |
| Documentação de módulo | Comentário `//!` no início do arquivo explica a regra central (ex.: `grading.rs`, `canvas_api.rs`, `estuda-domain`) | idem |
| Formatação | `cargo fmt` — hoje **não passa** (17 trechos; ver [testes](testes.md)) | — |

### Segurança no código

- **Nada de segredos em erros, logs, URLs gravadas ou JSON**: URLs são reduzidas por `sanitize_source_url` (remove query e fragmento) antes de serem gravadas; cookies só existem em memória.
- **Caminhos**: validar caminho relativo (sem absoluto, `..`, `\`, NUL) antes de qualquer acesso (`sync::validate_relative_path`, `domain::publication::validate_relative_path`, `lyceum::write_new_file_beneath`).
- **Somente leitura nos sistemas da UVA**: o Portal só navega por links oficiais visíveis e classificados como leitura (`lyceum::is_read_only_navigation`); `/login`, `/sso`, `/autologin` são bloqueados; a leitura do Canvas usa `fetch(url, { credentials: "same-origin", headers })` dentro da sessão do navegador, sem método explícito (GET), apenas em caminhos `/api/v1/...` validados por `canvas_api::validate_api_path`, que exige `auto_mark_as_read=false` ao abrir conversas.
- **Escrita no acervo só de arquivos novos ou projeções declaradas**: capturas usam nomes novos (`lyceum-<uuid>`); materiais existentes não são sobrescritos.

## Datas e identificadores no código

- Carimbos de captura do Portal: UTC, `YYYY-MM-DDTHH:MM:SSZ` (`lyceum::captured_at_utc`).
- Carimbos do SQLite do app: `strftime('%Y-%m-%dT%H:%M:%fZ','now')` (`AcademicStore::current_timestamp`) em alguns pontos e `CURRENT_TIMESTAMP` (`YYYY-MM-DD HH:MM:SS`, UTC) como padrão de colunas — **os dois formatos coexistem**.
- O sincronizador grava `timestamp()` como segundos desde a época Unix em texto (`sync.rs`). Também coexiste com os formatos anteriores.
- Períodos letivos: `AAAA.N` com `N` igual a `1` ou `2` (`calendar::valid_semester`).
- IDs de dados: `uva:discipline:<slug>`, `uva:program:<slug>`; IDs de curso do Canvas são numéricos (`LmsCourseId`).
- Esta variedade de formatos de data é uma **inconsistência conhecida**, não uma convenção desejada.

## TypeScript / React

- Estado local com `useState`/`useEffect`; comandos via `invoke` do `@tauri-apps/api`.
- Lógica testável em módulos `.ts` separados dos componentes `.tsx` (por exemplo `studyTypes.ts`), testada com `node --test` (`app/scripts/*.test.mjs`).
- Acessibilidade observada: `aria-label` em botões de ícone, `role="status"` e `role="alert"` em avisos e erros, `nav` rotulado.
- `typescript` 7.0.2 e `vite` 8.3.4 fixados em `app/package.json`; React fixado em 19.3.0.

## Git e mensagens de commit

- Branch padrão: `master` (o repositório está nela; o README descreve o repositório como local, sem remoto).
- Estilo observado no histórico (42 commits): `tipo(escopo): resumo em português`, com tipos `feat`, `fix`, `docs`, `refactor`, `data` e escopos como `estuda-uva`, `acervo`, `fontes-uva`, `materias`, `features`. Exemplos: `feat(estuda-uva): painel acadêmico do Canvas e calculadora de nota final`, `docs: troca referências de main para master`.
- Não há `CHANGELOG.md` neste repositório hoje.
- **Esta documentação não cria commits.** As alterações ficam na árvore de trabalho.

## Documentação

- Português do Brasil; títulos em sentença; blocos de código com linguagem declarada; diagramas em Mermaid (texto), nunca imagem.
- Toda afirmação técnica aponta o caminho do código; o que é futuro é marcado **Planejado**; o que não foi conferido, **Não verificado**.
- Exemplos usam dados sintéticos. Nunca incluir matrícula, nota, CPF, cookies, tokens ou URLs com parâmetros de autenticação.
