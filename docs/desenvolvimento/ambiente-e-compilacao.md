# Ambiente e compilação

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada.

Todos os comandos abaixo são executados **na raiz do repositório**. A coluna "Verificado" diz se esta revisão executou o comando (resultado do dia 2026-11-06) ou apenas conferiu que ele existe no código/manifesto.

## Requisitos

| Requisito | Versão exigida | Onde está declarado | Versão na máquina de revisão |
|---|---|---|---|
| Rust (edition 2024) | `rust-version = "1.90"` | `app/Cargo.toml`, `crates/estuda-domain/Cargo.toml`, `app/src-tauri/Cargo.toml` | `rustc 1.99.0`, `cargo 1.99.0` |
| Node.js | 24 (o gerador usa `node:sqlite`; o teste do app usa `--experimental-strip-types`) | `tools/acervo/README.md`, `app/package.json` | `v24.22.0` |
| npm | qualquer compatível com o `package-lock.json` | `app/package-lock.json` | não registrado |
| `pdftotext` (Poppler) | qualquer | usado por `tools/acervo/src/pdf-para-md.mjs` | `/opt/homebrew/bin/pdftotext` |
| Chromium ou Chrome | qualquer recente | `detect_chromium` em `app/src-tauri/src/lib.rs` | `/opt/homebrew/bin/chromium` |
| Git | qualquer | `repository.rs` executa `git status` | presente |
| macOS | só macOS foi exercitado | `tauri.conf.json` (`bundle.targets: ["app","dmg"]`, `macOS.signingIdentity: "-"`) | Darwin 25.6 |

Caminhos de Chromium procurados automaticamente (`detect_chromium`): `/opt/homebrew/bin/chromium`, `/Applications/Chromium.app/Contents/MacOS/Chromium`, `/Applications/Google Chrome.app/Contents/MacOS/Google Chrome`, `/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary`. Outro executável pode ser definido na interface (comando `set_chromium_path`, guardado em `settings.chromium_executable`).

**Dependências nativas do GPUI e suporte a Linux/Windows: Não verificado.** O repositório não documenta nem exercita esses ambientes. O crate `estuda-gpui` é um esqueleto (ver [migração](../arquitetura/migracao-gpui.md)).

**Toolchain Rust:** não há `rust-toolchain.toml` nem CI (`.github/` não existe). A versão mínima vem apenas de `rust-version`.

## Estrutura do Cargo

```text
app/Cargo.toml              # [workspace] members: estuda-core, estuda-gpui, estuda-sync; exclude: src-tauri
app/Cargo.lock              # novo, ainda não rastreado
app/src-tauri/Cargo.toml    # pacote "estuda-uva", fora do workspace, com Cargo.lock próprio
crates/estuda-domain/       # pacote com [workspace] próprio (isolado de propósito)
```

Consequências práticas: o comando muda conforme o pacote. Crates do workspace usam `--manifest-path app/Cargo.toml -p <crate>`; o domínio usa `--manifest-path crates/estuda-domain/Cargo.toml`; o shell Tauri usa `--manifest-path app/src-tauri/Cargo.toml`. O diretório de compilação do workspace é `app/target/`, que ainda **não** está no `.gitignore` (ver [decisões pendentes](../arquitetura/decisoes-pendentes.md#8-higiene-do-repositório-e-da-documentação-existente)).

## Compilar e executar

| Objetivo | Comando | Verificado |
|---|---|---|
| Compilar e testar o núcleo | `cargo test --manifest-path app/Cargo.toml -p estuda-core --lib` | Sim, com `--locked --offline` (152 passam; compilação a frio ≈ 2 min 49 s) |
| Testar o domínio | `cargo test --manifest-path crates/estuda-domain/Cargo.toml` | Sim, com `--locked --offline` (14 passam) |
| Verificar formatação | `cargo fmt --manifest-path app/Cargo.toml --all -- --check` | Executado: **falha** (17 trechos a reformatar; ver [testes](testes.md)) |
| Compilar o esqueleto GPUI | `cargo check --manifest-path app/Cargo.toml -p estuda-gpui` | Não verificado na versão atual (o arquivo mudou durante a revisão) |
| Compilar o shell Tauri | `cargo check --manifest-path app/src-tauri/Cargo.toml` | Não verificado nesta revisão |
| Instalar o frontend | `npm install --prefix app` | Não reexecutado (`app/node_modules` já existe) |
| Compilar o frontend | `npm run build --prefix app` (`tsc -b && vite build`, saída em `app/dist/`) | Não reexecutado: gera artefatos |
| Testar o frontend | `npm test --prefix app` | Sim (40 passam) |
| Abrir o app em desenvolvimento | `npm run tauri --prefix app -- dev` | Não verificado. Observação: `tauri.conf.json` define só `frontendDist: "../dist"` (sem `devUrl` nem `beforeDevCommand`), então rode `npm run build --prefix app` antes |
| Empacotar (`.app` e `.dmg`) | `npm run tauri --prefix app -- build` | Não verificado por esta revisão; `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md` relata sucesso em 15/10/2026, com assinatura ad hoc e sem notarização |

Os scripts reais do frontend (`app/package.json`) são `dev` (`vite`), `build`, `test` e `tauri`. O servidor Vite, quando usado, escuta em `127.0.0.1:1420` (`vite.config.ts`).

Sobre `--locked` e `--offline`: foram usados nas execuções desta revisão para não alterar `Cargo.lock` nem acessar a rede. Sem `--offline`, o Cargo pode baixar dependências.

## Gerador do acervo (Node)

| Objetivo | Comando | Verificado |
|---|---|---|
| Validar `dados/` (somente leitura) | `node tools/acervo/build.mjs --validar` | Sim: "OK: 234 registros válidos" |
| Listar binários no repositório | `node tools/acervo/build.mjs --verificar-texto` | Sim: 0 erros, 0 avisos |
| Validar **e gerar** | `node tools/acervo/build.mjs` | Não executado: **escreve** `materias/catalogo/*.json`, `build/acervo.sqlite3` e `build/visualizacoes/*.md` |
| Converter PDF em Markdown | `node tools/acervo/src/pdf-para-md.mjs <arquivo.pdf> <saida.md> "<fonte>"` | Não executado; requer `pdftotext` |
| Testes do gerador | `node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs` | Sim (13 passam) |

A execução dos testes do gerador imprime "fatal: not a git repository" em um caso de `texto.test.mjs` (pasta temporária sem Git); os 13 testes passam mesmo assim.

## Variáveis de ambiente e configuração

- `HOME`: usado por `AnthropicConfig::read_default` para localizar `~/Dev/Projetos/gerenciador-de-conteudo/.env` (chaves `ANTHROPIC_API_KEY` e `ANTHROPIC_MODEL`). O arquivo está fora do repositório e **nunca** deve ser copiado para ele. Detalhes de privacidade em [segurança](../seguranca/privacidade.md).
- `.env` e `.env.*` estão no `.gitignore` (exceto `.env.example`).
- Variáveis `TAURI_ENV_*` são lidas por `vite.config.ts` durante builds do Tauri.

## Problemas conhecidos de ambiente

- O teste `launches_real_isolated_chromium` (`app/crates/estuda-core/src/lib.rs`) **entra em pânico** se `/opt/homebrew/bin/chromium` não existir. Sem Chromium nesse caminho, `cargo test -p estuda-core --lib` falha em 1 teste mesmo com o código correto.
- Duas compilações do mesmo workspace simultâneas aguardam o lock ("Blocking waiting for file lock on build directory").
- `estuda-sync` e `estuda-gpui` surgiram/mudaram durante a revisão e não foram compilados por ela; um `cargo check` do workspace inteiro (`cargo check --manifest-path app/Cargo.toml --workspace`) pode falhar por qualquer um dos dois. **Não verificado.**
