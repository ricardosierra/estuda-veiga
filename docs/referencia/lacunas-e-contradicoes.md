# Lacunas e contradições

**Revisado em:** 2026-11-06 · **Base:** repositório em movimento (ver [README](../README.md)).

Esta página reúne o que a revisão encontrou de desatualizado, contraditório ou não verificado. Nada aqui foi corrigido no código ou nos dados: a revisão foi só documental.

## 1. Resolvida: política de publicação de dados pessoais

**Decisão do dono, 2026-11-06**, confirmada nesta sessão. Substitui a regra anterior, que proibia publicar dado pessoal, tratava `privado/` como sigilo e exigia remover links para `privado/` antes de publicar.

- Todo conteúdo do acervo pode ir ao acervo público e ao Git, inclusive notas, frequência, matrículas e trabalhos entregues, desde que os dados pessoais estejam separados em `privado/` (mesmo repositório). A separação é de pasta, não de sigilo.
- Continuam proibidos no Git e em qualquer publicação: senhas, cookies, tokens, `.env`, bancos SQLite (`*.sqlite3*`, WAL, SHM, journal), perfil do Chromium e `originais/`. Binários (PDF, Word, imagens) ficam fora do Git.
- Ajustadas nesta edição: `docs/biblioteca/catalogacao.md`, `docs/biblioteca/tipos-de-conteudo.md`, `docs/biblioteca/fontes-e-licencas.md` e as linhas do README.md citadas na regra de contribuição. `docs/seguranca/credenciais-e-sessoes.md` não descrevia a regra antiga e ficou inalterada.
- Ainda com a regra antiga, segundo a revisão anterior, fora deste escopo e a alinhar: `docs/seguranca/{privacidade,ameacas,publicacao}.md`, `docs/contribuindo/conteudo.md`, `docs/dados/{visao-geral,banco-privado}.md` e `docs/getting-started/resolucao-problemas.md`.

**Pendências abertas** (ressalvas que a decisão não cobre; não decididas aqui):

- **RA, CPF e RG:** continuam fora dos registros até decisão do dono.
- **E-mail e telefone:** a decisão não os menciona; sem decisão registrada.
- **Nomes de docentes e dados de outros colaboradores ou terceiros:** continuam fora da camada pública até decisão do dono.
- **Direitos autorais e licenças de `fontes-uva/`:** em aberto (ver [fontes-e-licencas.md](../biblioteca/fontes-e-licencas.md) §3).

## 2. Contradições entre documentação e código

| # | Contradição | Onde | Efeito |
|---|---|---|---|
| 1 | O escopo do escaneador e do sincronizador ainda lista `cursos/` e arquivos de registro na raiz e **não** lista `privado/` nem `dados/`. O monitor só observa `materias` e `docs`. | `app/crates/estuda-core/src/catalog/scanner.rs` (≈ linhas 470 a 520), `sync_watcher.rs` | Depois da reorganização, o app não indexa o que foi para `privado/`. Conferido por leitura do código; **não verificado** em execução. |
| 2 | A interface diz "Sem IA, nuvem ou cobrança por uso", e `app/README.md` diz "sem geração por IA". Os resumos por matéria enviam texto à API da Anthropic. | `app/src/ui/App.tsx` (barra lateral), `app/README.md`, `study_summaries.rs` | Texto de interface enganoso. |
| 3 | `docs/features/README.md` afirma que o cliente não envia `anthropic-workspace-id`. O código atual envia o cabeçalho quando `ANTHROPIC_WORKSPACE_ID` existe. O HTTP 400 registrado ali foi de versão anterior e não foi reverificado. | `anthropic.rs` | Registro obsoleto. |
| 4 | `docs/ARQUITETURA_ACERVO.md`, `app/README.md`, `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md` e um comentário de `grading.rs` citam `app/src-tauri/src/...`; o núcleo está em `app/crates/estuda-core/src/`. | os arquivos citados | Caminhos quebrados. |
| 5 | `app/README.md` manda rodar `cargo test --manifest-path src-tauri/Cargo.toml --lib`. O shell Tauri não tem nenhum `#[test]`; os testes do núcleo estão em `estuda-core`. | `app/README.md` | O comando documentado não exercita o núcleo. |
| 6 | `dados/LEIA-ME.md` lista `canvas`, `matriculas`, `notas`, `trajetorias` e `trabalhos` como pastas de `dados/`; estão em `privado/dados/`. O cabeçalho de `separar-privado.mjs` proíbe `dados/` de apontar para `privado/`, mas `dados/LEIA-ME.md` permite, e 174 registros de `dados/` citam `privado/` em `source.file`. | `dados/LEIA-ME.md`, `tools/acervo/src/separar-privado.mjs` | Regra de fonte ambígua. A outra sessão afirma ter atualizado o `LEIA-ME`; não reconferido. |
| 7 | `build/acervo.sqlite3` é classificado como `Generated`, mas contém matrículas com docente, notas e frequência. | `tools/acervo/src/sqlite.mjs`, `crates/estuda-domain` | Não é um "banco público". |
| 8 | `estuda-domain` define `Confidence`, ids tipados e camadas de publicação, mas `estuda-core` e o shell Tauri não o usam. Dos 76 ids de `dados/disciplinas/`, 73 não passam em `SubjectId::parse`. | `crates/estuda-domain` | Contrato existente, não integrado. |
| 9 | `app/README.md` diz que a captura do Canvas gera `canvas-<id>`; o código usa `canvas-<uuid>`. | `app/README.md`, `canvas_api.rs` | Nome documentado incorreto. |
| 10 | `NAVEGACAO_PORTAL_UVA.md` diz que o app "não reaproveita cookies", mas `cookies_for_host` lê cookies do Chromium via CDP para baixar PDF (só em memória). A mesma página afirma "Sem IA". | `browser.rs`, `docs/NAVEGACAO_PORTAL_UVA.md` | Garantia descrita com exagero. |
| 11 | `catalog/guidance.rs` embute uma versão antiga do `AGENTS.md` e a recria se o arquivo não existir. | `guidance.rs` | Pode reintroduzir texto desatualizado. |
| 12 | `build.mjs` sobrescreve `materias/catalogo/*.json` sem detectar conflito, e o app trata esses arquivos como bidirecionais. | `tools/acervo/build.mjs`, `sync.rs` | Risco de perder edição feita pelo app. |
| 13 | Formatos de data divergem: `startDate` em `DD/MM/AAAA`, sincronizador em segundos Unix, capturas em ISO com `Z`. | `dados/`, `sync.rs`, `lyceum.rs` | Sem convenção única; ver [formatos.md](formatos.md). |
| 14 | `docs/arquitetura/04` e `05` usam números antigos do shell Tauri (50 comandos, 2.240 linhas); hoje são 52 e 1.888. | `docs/arquitetura/` | Texto da outra sessão; não editado aqui. |
| 15 | A captura do Lyceum não tem botão de cancelar. A leitura de prazos (`list_assignment_events`) não passa por `validate_api_path`. | `browser.rs` | Limitações documentadas em [automacoes](../automacoes/limites-e-seguranca.md). |

## 3. Problemas de repositório e de build

- `app/target/` (≈ 2 GB) não está no `.gitignore` e aparece como não rastreado.
- `app/Cargo.lock` e `app/Cargo.toml` não estão versionados. `cargo check --locked -p estuda-gpui` falhou com "cannot update the lock file" (numa versão do crate que depois mudou).
- `app/src-tauri/Cargo.lock` apareceu modificado durante a sessão (adiciona `estuda-core`). Origem não determinada; não revertido.
- `cargo fmt --check` retorna diferenças: 2 em `study_summaries.rs` e 15 em `crates/estuda-domain`.
- Os testes de DOCX e Pages em `catalog/documents.rs` leem arquivos reais de `originais/` e falham em clones sem eles.
- O teste do Chromium exige `/opt/homebrew/bin/chromium`.
- Entradas do `.gitignore` sobre o Chromium e o SQLite apontam para caminhos antigos; o perfil real fica em `app_data_dir()`.
- `docs/superpowers/` é excluído do escaneador do app (`scanner.rs`), então planos e specs não aparecem na tela de acervo.

## 4. Lacunas de cobertura

Componentes que **não foram inspecionados** nesta revisão e, portanto, não estão documentados:

- `app/crates/estuda-sync/`: crate novo, surgiu durante a revisão (15 módulos, 112 `#[test]` segundo o fork de arquitetura), não integrado e não compilado por mim.
- A versão nova de `app/crates/estuda-gpui/` (esqueleto com `gpui-kit` 0.9.0 e uma janela). `migracao-gpui.md` descreve o que o fork leu; não foi compilada.
- Documentos em `docs/arquitetura/01` a `05`: escritos por outra sessão; referenciados, não revisados.
- Conteúdo de `privado/` e dos binários de `originais/`: não lidos de propósito.
- `docs/superpowers/` (planos e specs): não relidos.

## 5. Não verificado em execução

- Qualquer fluxo com login: captura do Canvas, leitura da API do Canvas, captura do Portal e do Lyceum, calendário.
- A API da Anthropic (o 400 de 2026-11-06 é registro anterior).
- A interface: nenhuma tela foi aberta; `npm run build` e `npm run tauri dev` não foram executados.
- Acessibilidade (teclado, contraste, leitor de tela). O que existe no código está em [telas.md](../aplicativo/telas.md).
- Linux e Windows; dependências nativas do GPUI.
- Comportamento das migrações v5 e v6 se um passo falhar no meio; como o monitor evita reconciliações repetidas; o que acontece com o Chromium se o app fechar sem `disconnect_canvas`.
- Não há medição de performance em uso real; `performance.md` registra só limites do código.

## 6. Decisões que exigem o dono

1. Confirmar ou rejeitar a política de publicação (seção 1) e decidir sobre terceiros e `fontes-uva/`.
2. Atualizar o escopo do escaneador e do sincronizador para `privado/`, ou manter o que há e mover os arquivos de volta.
3. Destino de `estuda-domain`: integrar a `estuda-core` ou removê-lo.
4. Banco público: criar ou não (hoje não existe).
5. Envio de texto de matérias à API da Anthropic: opt-in por material, trechos escolhidos ou nenhum envio. Corrigir o texto da interface conforme a decisão.
6. Onde fica o `.env` da API (hoje um caminho fixo de outro projeto em `$HOME`).
7. Licença do repositório (hoje nenhuma declarada).
8. Ignorar `app/target/` e versionar ou não os `Cargo.lock`.
9. Formato único de datas e fusos.
10. Rodar `cargo fmt` e regenerar o lock de `app/`.

## Verificações desta revisão

Executadas em 2026-11-06, macOS, `rustc 1.99.0`, `node v24.22.0`.

| Verificação | Resultado |
|---|---|
| `node tools/acervo/build.mjs --validar` | 234 registros válidos |
| `node tools/acervo/build.mjs --verificar-texto` | 0 erros, 0 avisos |
| `node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs` | 13 passaram |
| `npm test --prefix app` | 40 passaram |
| `cargo test --locked --offline --manifest-path app/Cargo.toml -p estuda-core --lib` | 152 passaram (inclui o teste com Chromium real) |
| `cargo test --locked --offline --manifest-path crates/estuda-domain/Cargo.toml` | 14 passaram na execução; a outra sessão informa 17 depois de alterar o crate. Não reexecutado. |
| `cargo check --locked --offline -p estuda-gpui` | falhou (lock desatualizado), em versão anterior do crate |
| `cargo fmt --manifest-path app/Cargo.toml --all -- --check` | diferenças de formatação |
| Links relativos e âncoras de `README.md` e `docs/` (sem `docs/superpowers/`) | verificados por script; os links que apontavam para páginas ainda não escritas foram resolvidos ao final |
| Exemplos JSON dos forks de dados e biblioteca | sintaxe validada; só um passou pela função `validar` do gerador |
| `npm run build`, `npm run tauri dev/build`, `cargo` do shell Tauri | **não executados** |

Nenhum SQLite foi aberto, nenhuma consulta SQL foi executada, nenhum código, dado ou original foi alterado, e nenhum commit foi feito.
