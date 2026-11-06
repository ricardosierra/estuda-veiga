# Testes

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada. As execuções abaixo foram feitas nesta data, nesta máquina (macOS, Rust 1.99.0, Node 24.22.0, Chromium em `/opt/homebrew/bin/chromium`).

## Resumo dos resultados observados

| Suíte | Comando | Resultado em 2026-11-06 |
|---|---|---|
| Núcleo Rust (`estuda-core`) | `cargo test --locked --offline --manifest-path app/Cargo.toml -p estuda-core --lib` | **152 passaram, 0 falharam** (3,74 s de testes; compilação a frio 2 min 49 s). Inclui o teste com Chromium real |
| Domínio Rust (`estuda-domain`) | `cargo test --locked --offline --manifest-path crates/estuda-domain/Cargo.toml` | **14 passaram**, 0 falharam |
| Frontend (Node) | `npm test --prefix app` | **40 passaram**, 0 falharam |
| Gerador do acervo (Node) | `node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs` | **13 passaram**, 0 falharam |
| Validação dos dados | `node tools/acervo/build.mjs --validar` | OK, 234 registros |
| Binários versionados | `node tools/acervo/build.mjs --verificar-texto` | 0 erros, 0 avisos |
| Formatação Rust | `cargo fmt --manifest-path app/Cargo.toml --all -- --check` | **Falhou** (código de saída 1): 17 trechos a reformatar, 2 em `estuda-core/src/study_summaries.rs` e 15 em `crates/estuda-domain/src/` (`confidence.rs` 1, `ids.rs` 6, `publication.rs` 8) |
| Compilação do GPUI | `cargo check --locked --offline --manifest-path app/Cargo.toml -p estuda-gpui` | **Falhou** na primeira tentativa: "cannot update the lock file … because --locked was passed". O `Cargo.toml` do crate mudou depois (passou a declarar `gpui-kit`), e a versão nova **não** foi testada |
| Shell Tauri (`app/src-tauri`) | — | **Não executado.** O pacote não tem nenhum `#[test]` |
| `estuda-sync` | `cargo test --manifest-path app/Cargo.toml -p estuda-sync` (comando previsto) | **Não executado.** Crate surgido durante a revisão (ver [crates](../arquitetura/crates.md#estuda-sync)); contém 112 funções `#[test]` em 15 módulos, sem execução confirmada |

Não há CI configurada (`.github/` não existe), portanto nenhum resultado acima é reproduzido automaticamente.

O que **não** está coberto por nenhum teste: login real no Canvas ou no Portal/Lyceum, captura autenticada de um semestre, frequência oficial, `browser.rs` (0 testes unitários), os comandos Tauri e a interface React renderizada (os 40 testes Node cobrem só a lógica em `.ts`). `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md` registra que nenhuma varredura autenticada completa foi executada.

## Testes Rust por módulo (`estuda-core`)

Contagem por `grep -c '#\[test\]\|#\[tokio::test\]'` (soma 152, igual ao total executado).

| Módulo | Testes | Módulo | Testes |
|---|---:|---|---:|
| `storage.rs` | 19 | `lyceum.rs` | 8 |
| `lib.rs` (integração) | 11 | `grading.rs` | 8 |
| `sync.rs` | 11 | `canvas.rs` | 8 |
| `tutor.rs` | 11 | `canvas_api.rs` | 8 |
| `anthropic.rs` | 8 | `sync_watcher.rs` | 8 |
| `study_summaries.rs` | 7 | `archive.rs` | 6 |
| `canvas_capture.rs` | 6 | `download.rs` | 6 |
| `catalog/documents.rs` | 6 | `catalog/classification.rs` | 5 |
| `catalog/scanner.rs` | 5 | `catalog/guidance.rs` | 3 |
| `calendar.rs` | 2 | `repository.rs` | 2 |
| `catalog/academic.rs` | 2 | `pdf.rs` | 1 |
| `lesson.rs` | 1 | `browser.rs` | 0 |

`estuda-domain`: `confidence.rs` 4, `ids.rs` 4, `publication.rs` 6 (total 14).

## O que os testes exercitam (exemplos reais)

- **Sincronização** (`sync.rs`): edição única propaga para SQLite, Markdown e JSON; edições divergentes viram conflito; remoção de arquivo vira *tombstone* e conflito de restauração; repetição de `sync_outbox` depois de interrupção.
- **Esquema do banco** (`schema.rs`, `storage.rs`): baseline v8 sem `ALTER`, banco antigo movido para backup e regenerado, banco com `user_version` 99 recusado e deixado intacto.
- **Segurança de importação** (`lib.rs`): `material_urls_require_https_and_uva_host_boundaries` recusa HTTP, `file://`, `usuario@host`, porta 8080 e hosts parecidos (`ead.uva.br.attacker.example`); importação nunca deixa PDF no disco; arquivos manuais não são sobrescritos.
- **Escopo do acervo** (`catalog/scanner.rs`): links simbólicos para fora da raiz não aparecem; arquivos fora das pastas acadêmicas são ignorados. Os testes ainda usam `cursos/...` (ver [decisões pendentes](../arquitetura/decisoes-pendentes.md)).
- **Chromium real** (`lib.rs`, `#[tokio::test] launches_real_isolated_chromium_and_reads_fixture_dom`): abre `/opt/homebrew/bin/chromium` em modo *headless* com perfil temporário e lê um DOM de teste. **Panica se esse caminho não existir.** Para rodar em outra máquina é preciso instalar o Chromium nesse caminho ou alterar o teste (mudança de código, fora deste trabalho documental).
- **Anthropic** (`anthropic.rs`): usa servidor local simulado (`with_endpoint`); **não** chama a API real.

## Como rodar

```sh
# núcleo inteiro
cargo test --manifest-path app/Cargo.toml -p estuda-core --lib
# um módulo ou teste (filtro por nome)
cargo test --manifest-path app/Cargo.toml -p estuda-core --lib sync::
cargo test --manifest-path app/Cargo.toml -p estuda-core --lib storage::tests
# domínio
cargo test --manifest-path crates/estuda-domain/Cargo.toml
# frontend e gerador
npm test --prefix app
node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs
```

Os dois primeiros filtros por módulo (`sync::`, `storage::tests`) seguem a sintaxe padrão do `cargo test <filtro>`; os caminhos de nome vêm da estrutura `#[cfg(test)] mod tests` dos arquivos, mas **o comando específico não foi executado** nesta revisão (apenas o conjunto completo).

## Como escrever testes

Padrões já usados no repositório:

- **Dados temporários:** `tempfile::tempdir()` (dev-dependency) para acervo e banco; nunca usar o acervo real nem `.estuda-uva.sqlite3`. Ex.: `AcademicStore::open(&tmp.join("app/db.sqlite"), tmp.join("Faculdade"))`.
- **Parsers puros:** `canvas.rs`, `canvas_api.rs`, `lyceum.rs`, `calendar.rs`, `grading.rs` não fazem I/O; testam com HTML/JSON de exemplo **sintético**. Use dados fictícios (cursos `8765`, hosts `ead.uva.br`); nunca copie páginas reais, matrículas, notas ou URLs com credenciais.
- **PDF:** o auxiliar `two_page_pdf()` em `lib.rs` gera um PDF mínimo com `lopdf`; não versione PDFs.
- **Rede:** não chame serviços reais. Use servidor local (como em `anthropic.rs`) ou teste só a validação.
- **Assíncrono:** `#[tokio::test]` (feature `macros` e `rt-multi-thread` já habilitadas).
- **Banco:** teste migração criando o esquema antigo em memória/temporário e abrindo com `AcademicStore::open`. Mudança de esquema real exige a aprovação descrita em [contribuindo código](contribuindo-codigo.md).

## Pendências de teste (Planejado)

- Teste de `browser.rs` sem navegador real (extrair funções puras).
- Teste do `AppState` e dos comandos Tauri.
- CI que rode as quatro suítes e a formatação.
- Corrigir ou decidir sobre a formatação (`cargo fmt --check` falha).
