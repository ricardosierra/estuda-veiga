# Resolução de problemas

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

Problemas conhecidos, com a causa verificada no código ou em uma execução desta revisão. Mensagens citadas entre aspas vêm do código ou da saída do comando.

## Compilação e testes

| Sintoma | Causa | O que fazer |
|---|---|---|
| `cargo test` do `estuda-core` falha em `launches_real_isolated_chromium_and_reads_fixture_dom` com "Chromium esperado em /opt/homebrew/bin/chromium está ausente" | O teste exige esse caminho (`app/crates/estuda-core/src/lib.rs`). | Instalar o Chromium nesse caminho (Homebrew) ou rodar os demais testes. Não há como pular o teste por flag sem editar o código. |
| `cargo check --locked -p estuda-gpui` falha com "cannot update the lock file ... because --locked was passed" | `app/Cargo.lock` (não versionado) não corresponde ao manifesto do workspace. | Decisão do mantenedor: regenerar o lock sem `--locked`. Não é mudança de documentação. |
| `cargo fmt --check` retorna diferenças | Há formatação fora do padrão em `study_summaries.rs` e em `crates/estuda-domain/src/*.rs`. | Rodar `cargo fmt`. Mudança de código; não feita nesta revisão. |
| `cargo ... --offline` falha por falta de crate | Cache do Cargo vazio. | Rodar sem `--offline` uma vez. |
| `node` reclama de `node:sqlite` | Node anterior ao 24. | Atualizar o Node. |
| `npm test` reclama de `--experimental-strip-types` | Node antigo. | Usar Node 24. |
| `pdf-para-md.mjs` falha | `pdftotext` ausente. | `brew install poppler`. |

## Aplicativo

| Sintoma | Causa provável | O que fazer |
|---|---|---|
| "Chromium não encontrado. Escolha o executável para continuar." | Nenhum dos quatro caminhos fixos de macOS existe e `chromium_executable` não está definido. | Escolher o executável na interface. |
| App não lista matérias, cursos ou registros pessoais | O escopo de varredura não inclui `privado/` e ainda lista `cursos/` na raiz, que não existe mais. | Ver [decisoes-pendentes.md](../arquitetura/decisoes-pendentes.md). Não é erro de uso. |
| Pasta do acervo errada em um `.app` movido | O padrão é fixado na compilação. | Trocar a pasta em **Dados e sync**. |
| "o banco está na versão N, mais nova que a versão suportada 7" | O SQLite foi criado por uma versão mais nova do app. | Usar um app compatível. Não apagar o banco nem editá-lo à mão. |
| Sincronização relata conflitos | Duas superfícies (SQLite, Markdown, JSON) mudaram de forma diferente, ou um arquivo-fonte foi apagado. | Revisar na tela de dados; o app preserva todas as versões. Ver [sincronizacao.md](../dados/sincronizacao.md). |
| Arquivo acima de 64 MB aparece como erro na sincronização | Limite `MAX_SYNC_FILE_BYTES`. | Dividir ou manter fora do acervo sincronizado. |
| Captura do Canvas ou Portal para no meio | Sessão expirada, limite de itens ou de páginas (Portal: 50). | Entrar de novo e retomar. Resultado parcial é marcado como tal. |
| Resumo por IA retorna erro | Chave ausente, `.env` no caminho fixo inexistente ou resposta HTTP de erro da API. | Ver [resumos-anthropic.md](../automacoes/resumos-anthropic.md). |

## Gerador do acervo

| Sintoma | Causa | O que fazer |
|---|---|---|
| `Validação falhou com N erro(s)` | Regra de `tools/acervo/src/validar.mjs`: `null` sem pendência, string vazia, referência inexistente, fonte inexistente, chave sensível, caminho absoluto de usuário, URL do Canvas incoerente com o id. | Corrigir o registro apontado. |
| `--verificar-texto` lista binário | PDF, imagem ou documento rastreado pelo Git (erro) ou presente localmente (aviso). | Converter em texto e manter o binário fora do Git. |
| Um aviso do Git "not a git repository" na saída dos testes | Um teste do gerador roda em pasta temporária sem Git. | Esperado; os 13 testes passam. |

## Antes de pedir ajuda

Não cole no chat, em issues ou em commits: cookies, tokens, senhas, URLs com parâmetros de autenticação (SSO), conteúdo do `.env`, CPF, RA ou RG. Notas, frequência e matrículas saíram desta lista pela política de 6/11/2026, que as admite no acervo desde que estejam em `privado/`. Ver [privacidade.md](../seguranca/privacidade.md).
