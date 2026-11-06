# Instalação

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

Este guia cobre o que é preciso ter instalado para ler o acervo, rodar o gerador e compilar o Estuda UVA. Não existe instalador publicado nem release: o aplicativo é compilado a partir do código.

## Requisitos

| Item | Versão | Fundamento |
|---|---|---|
| Rust | 1.90 ou mais novo, edition 2024. A revisão usou `rustc 1.99.0`. | `rust-version = "1.90"` em `app/Cargo.toml` e `crates/estuda-domain/Cargo.toml` |
| Node.js | 24. A revisão usou `v24.22.0`. | `tools/acervo` usa `node:sqlite`; `npm test` usa `--experimental-strip-types` |
| npm | o que acompanha o Node | `app/package.json` |
| Chromium ou Chrome | atual | só para capturar Canvas e Portal |
| `pdftotext` (poppler) | qualquer | só para `tools/acervo/src/pdf-para-md.mjs` |
| Git | qualquer | a tela de dados mostra o estado do repositório |

**Sistema operacional.** O desenvolvimento e todas as verificações foram feitas em macOS (Darwin). O código procura o Chromium em quatro caminhos fixos, todos de macOS (`detect_chromium` em `app/src-tauri/src/lib.rs`): `/opt/homebrew/bin/chromium`, `/Applications/Chromium.app/...`, `/Applications/Google Chrome.app/...` e `/Applications/Google Chrome Canary.app/...`. Em Linux e Windows a compilação e o uso **não foram verificados**; ali o caminho do navegador teria de ser escolhido manualmente na tela de dados.

O teste `launches_real_isolated_chromium_and_reads_fixture_dom` de `estuda-core` exige `/opt/homebrew/bin/chromium` e falha com `panic` se o arquivo não existir. Sem esse caminho, `cargo test` do núcleo não passa por inteiro.

## Dependências nativas da interface GPUI

**Planejado / não verificado.** O crate `estuda-gpui` não declara dependência de GPUI (`app/crates/estuda-gpui/Cargo.toml` só depende de `estuda-core`), então nenhuma dependência nativa de GPUI é exigida hoje. Quando a migração começar, esta seção precisa ser preenchida com o que o toolchain pedir. Ver [migracao-gpui.md](../arquitetura/migracao-gpui.md).

## Obter e preparar o código

```sh
cd Faculdade                      # raiz do repositório
npm install --prefix app          # dependências do frontend (React, Vite, Tauri CLI)
```

O gerador do acervo (`tools/acervo`) não tem dependências externas e não precisa de `npm install`.

## Verificar a instalação

Comandos que existem e foram executados em 2026-11-06:

```sh
node tools/acervo/build.mjs --validar        # OK: 234 registros válidos
node tools/acervo/build.mjs --verificar-texto  # 0 erros, 0 avisos
npm test --prefix app                        # 40 testes
cargo test --offline --manifest-path app/Cargo.toml -p estuda-core --lib   # 152 testes
```

`--offline` assume que as dependências já estão no cache do Cargo. Na primeira compilação, sem o cache, omita a flag.

Compilação completa e uso do aplicativo: [ambiente-e-compilacao.md](../desenvolvimento/ambiente-e-compilacao.md) e [primeiros-passos.md](primeiros-passos.md).

## Observações sobre o estado do repositório

- `app/Cargo.lock` está sem versionar e, ao ser conferido com `--locked`, não acompanha o manifesto do workspace (`cargo check --locked -p estuda-gpui` falhou com "cannot update the lock file"). Ver [lacunas-e-contradicoes.md](../referencia/lacunas-e-contradicoes.md).
- `app/target/` (cerca de 2 GB) não aparece no `.gitignore` e figura como não rastreado.
