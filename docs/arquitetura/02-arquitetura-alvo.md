# 02 — Arquitetura alvo e migração incremental

## 1. Princípio e regra de dependência

**Dado de aluno fica em base separada do conteúdo acadêmico compartilhável** (camadas separadas: o conteúdo de ambos pode ser publicado, mas os arquivos de banco não). Em código: nenhum crate de domínio depende de UI, de `tauri`, de `rusqlite` ou de rede. A direção é sempre `interface → aplicação → domínio`; infraestrutura implementa traits definidos no domínio.

```text
apps/estuda-desktop (GPUI)   estuda-cli
        \                     /
         +--- estuda-app (casos de uso: sync, import, captura) ---+
                 |                    |                     |
           estuda-domain        estuda-storage         estuda-portal
        (ids, confiança,       (SQLite: privado      (Chromium, Canvas API,
         publicação, modelos)    e acervo, cache)       Lyceum)
```

**Nota de vocabulário (política de 6/11/2026):** "privado" em nomes como `PrivateDb` e `private/` significa *separado do acervo*, não *secreto*. O banco pessoal e seus exports de texto podem ser publicados; o que nunca sai são bancos SQLite em si, credenciais, perfis do navegador e binários originais (04 §1). A separação física dos dois bancos continua obrigatória.

## 2. Crates: poucos, com motivo

Não proponho os dez crates da árvore de referência. Cada crate precisa de uma razão de isolamento:

| Crate | Existe? | Motivo de existir |
|---|---|---|
| `estuda-domain` | **sim, criado** | Compila sem dependências; impede acoplamento do domínio a UI/banco; carrega a trava de publicação |
| `estuda-storage` | proposto | Único lugar que conhece `rusqlite`; abre os **dois** bancos por tipos diferentes (`PrivateDb`, `LibraryDb`), de modo que misturá-los não compila |
| `estuda-portal` | proposto | `chromiumoxide` + `reqwest` são pesados e instáveis; isolar acelera `cargo check` do resto |
| `estuda-cli` | proposto | `build`, `validate`, `check-publish`, `sync` sem interface gráfica; substitui parte de `tools/acervo` quando houver paridade |
| `estuda-desktop` (app) | futuro | GPUI; entra na migração de Tauri, que é etapa própria |

**Ficam como módulos, não crates:** `grading`, `lesson`, `tutor`, `calendar`, `pdf`/`catalog` (extração), `sync`, `cache`, `search`. São pequenos (115 a 1.485 linhas) e só um consumidor cada; separar agora só adicionaria fronteiras de versão.

## 3. Mapa dos módulos atuais para o destino

| Hoje (`app/src-tauri/src`) | Destino | Observação |
|---|---|---|
| `lib.rs` (50 comandos Tauri) | `app/` do desktop; lógica de cada comando desce para `estuda-app` | Maior ganho: 2.240 linhas viram adaptador fino |
| `storage.rs` (3.195) | `estuda-storage`, **dividido** em `private/` e `library/` | Dividir por tabela, ver 03 §4 |
| `sync.rs`, `sync_watcher.rs` | `estuda-app::sync` | Contrato em 03 §5 |
| `catalog/*` | `estuda-library` (módulo) | Leitura do catálogo; sem Tauri hoje |
| `canvas*.rs`, `browser.rs`, `lyceum.rs`, `download.rs` | `estuda-portal` | `Course` → `LmsCourse` |
| `grading.rs`, `lesson.rs`, `tutor.rs`, `calendar.rs`, `study_summaries.rs` | `estuda-app` (módulos); tipos para `estuda-domain` | `study_summaries.rs` precisa perder 2 `#[tauri::command]` |
| `pdf.rs`, `archive.rs` | `estuda-content` (módulo no `estuda-app`) | Extração em memória, sem guardar PDF |
| `anthropic.rs` | `estuda-app` atrás de trait | Opcional, fora do caminho offline |
| `app/src` (React) | **congelado**; sem novos componentes | Descartado na migração GPUI |

## 4. Árvore: proposta e implementada

**Implementada nesta etapa** (apenas adições):

```text
crates/estuda-domain/     ← novo: Cargo.toml, Cargo.lock, src/{lib,ids,confidence,publication}.rs
docs/arquitetura/         ← novo: 5 documentos
.gitignore                ← +1 linha: /crates/*/target/
```

**Proposta, ainda não criada:** `crates/{estuda-storage,estuda-portal,estuda-cli}`, `biblioteca/` (catálogo textual publicável), `schemas/` (JSON Schema dos registros) e `Cargo.toml` na raiz como workspace.

**Relação com o workspace GPUI já existente.** `app/Cargo.toml` (não rastreado, de outro processo) já define o workspace com `estuda-core` e `estuda-gpui`. Portanto não proponho `Cargo.toml` na raiz: o destino natural é `app/crates/`. `estuda-core` é a etapa 2 do plano (extração mecânica, ainda um crate só) e `estuda-domain` é uma folha sem dependências que ele pode importar. Deixei `crates/estuda-domain` isolado, com `[workspace]` vazio, para não editar o `Cargo.toml` de outro trabalho em andamento. **Recomendação:** quando esse workspace for commitado, mover `estuda-domain` para `app/crates/` e acrescentá-lo a `members`. Duas pastas `crates/` (raiz e `app/`) não devem coexistir.

**Não migrado de propósito:** `materias/`, `fontes-uva/`, `periodos/`, `cursos/`, `originais/`. Continuam em seus caminhos até haver plano aprovado.

## 5. Plano de migração (cada etapa é reversível e testável isolada)

1. **Feito.** Crate `estuda-domain` e este conjunto de documentos.
2. **Em andamento por outro processo (`estuda-core`, não verificado por mim).** Depois, `estuda-app` e `estuda-storage` recebem o código de `storage.rs` por *extração mecânica* (mover funções, manter assinaturas), com os 152 testes como rede de segurança. Critério: mesmos 152 testes verdes antes e depois.
3. **Sem banco.** Trocar `Course` por `LmsCourse` e usar `SubjectId`/`LmsCourseId` nas assinaturas novas; só renomeia tipos e campos serializados com `#[serde(alias)]` para não quebrar JSON existente.
4. **Precisa de aprovação.** Divisão física dos bancos (03 §4): SQL, rollback e backup estão em 03 §7.
5. **Superada pela política de 6/11/2026 (confirmada pelo dono).** Dados pessoais podem ficar no Git, inclusive em `dados/`. Seguem proibidos: senhas, cookies, tokens, `.env`, `*.sqlite3*`, perfil do Chromium e `originais/`; RA, CPF e RG ficam fora até decisão do dono.
6. **Etapa própria.** GPUI. Só depois de 2 e 4.

## 6. Métricas para provar ganho (não prometidas, a medir)

| Métrica | Linha de base | Como medir |
|---|---|---|
| Testes preservados | 152 verdes | `cargo test --offline --lib` antes/depois de cada etapa |
| Tempo de `cargo check` do domínio | a medir | `cargo check -p estuda-domain` |
| Tempo de abertura do banco e 1ª leitura do catálogo | a medir | `Instant` em `AcademicStore::open_workspace` e `catalog::load` |
| Releituras de arquivo inalterado | a medir | contador de hits/misses do cache por (mtime, tamanho, hash) |
| Memória do cache | orçamento fixo (proposto: 64 MiB) | contador de bytes; teste de despejo |
| Latência de busca | a medir | `EXPLAIN QUERY PLAN` + benchmark com 1.000 documentos |

Cache: um LRU limitado por bytes, chaveado por `(caminho, mtime, tamanho)` e validado por SHA-256 só quando mtime/tamanho mudam. Sem camadas adicionais até a medição mostrar necessidade.
