# Primeiros passos e configuração

**Revisado em:** 2026-11-06 · **Base:** repositório em movimento; a interface mudou durante a revisão (commits `aeab5f8` e seguintes).

Este guia descreve o que dá para fazer hoje. Nenhum passo de interface foi executado nesta revisão: o aplicativo não foi aberto. As descrições vêm do código.

## O que mudou durante a revisão

- O frontend React/Vite do Tauri foi **removido** (`aeab5f8`). As telas descritas em [telas.md](../aplicativo/telas.md) existiram até o commit `aeab5f8^` e hoje só estão no histórico do Git.
- A interface passou a ser o crate GPUI `app/crates/estuda-gpui` (binário `estuda-uva`), em desenvolvimento ativo por outra sessão. Em 2026-11-06 ele **não compilava** (`E0583`, módulo `speech` ausente numa edição em andamento).
- O GPUI lê o acervo por `Services` (`app/crates/estuda-gpui/src/services.rs`), que só embrulha `CatalogService` de `estuda-core`: varre a pasta, mantém o índice em memória e lê documentos. Nessa versão ele **não abre o SQLite, não sincroniza e não fala com Canvas ou Portal**. As integrações continuam existindo em `estuda-core` e como comandos no shell `app/src-tauri`, mas sem interface que as chame. **Não verificado** se versões posteriores do GPUI as ligam.

## Usar só o acervo, sem o aplicativo

Nada exige o aplicativo para ler ou validar o acervo.

1. Leia os índices em [`periodos/README.md`](../../periodos/README.md), [`materias/README.md`](../../materias/README.md) e [`fontes-uva/README.md`](../../fontes-uva/README.md).
2. Os textos extraídos de PDFs oficiais ficam em `fontes-uva/<período>/<matéria>/`. Nem todos os arquivos têm cabeçalho com a fonte; ver [fontes-e-licencas.md](../biblioteca/fontes-e-licencas.md).
3. Os registros estruturados estão em `dados/`, um JSON por arquivo. Para conferi-los: `node tools/acervo/build.mjs --validar`.
4. Os dados pessoais do aluno estão em `privado/`, no mesmo repositório. O repositório é **público** (remoto `origin` no GitHub), então o que está em `privado/` e foi enviado ao remoto é público. Só CPF e dados muito pessoais ficam de fora; ver [privacidade.md](../seguranca/privacidade.md).

## Abrir a interface GPUI

```sh
cargo run --manifest-path app/Cargo.toml -p estuda-gpui    # Não verificado: falhou ao compilar em 2026-11-06
```

Ao iniciar, a janela de 1280 × 820 abre o acervo da pasta `../../..` a partir de `app/crates/estuda-gpui`, isto é, a raiz do repositório, resolvida em **tempo de compilação** (`CARGO_MANIFEST_DIR`). Um binário copiado para outro lugar continua apontando para o diretório onde foi compilado.

As oito telas do código atual (`enum Screen` em `app/crates/estuda-gpui/src/ui/mod.rs`) são Painel, Acadêmico, Sync, Biblioteca, Leitor, Estudo, Auxílio e Usuário. O próprio `app/crates/estuda-gpui/src/ui/data.rs` avisa que parte do conteúdo é o protótipo do canvas "Estuda UVA", copiado literalmente, e que a ligação com o acervo real acontece em `AppView`. Quais telas mostram dados reais além de Biblioteca e Leitor **não foi verificado**.

## Shell Tauri (sem interface)

`app/src-tauri` ainda registra 52 comandos (lista em [comandos.md](../referencia/comandos.md)) e, na inicialização (`run()` em `app/src-tauri/src/lib.rs`):

1. escolhe como acervo a pasta `../..` a partir de `app/src-tauri`, a raiz do repositório;
2. abre ou cria o SQLite em `<acervo>/.estuda-uva.sqlite3` e, se existir, importa uma única vez o banco antigo de `app_data_dir/estuda-uva.sqlite3`;
3. cria `AGENTS.md` e `CLAUDE.md` na raiz do acervo **somente se não existirem** (`catalog::guidance::create_missing_guidance`);
4. indexa os documentos, abre o motor de sincronização e inicia o monitor de arquivos.

Sem frontend, esse shell não tem como ser operado por um usuário. O esquema do banco mudou para a versão 8 em outra sessão (ver [banco-privado.md](../dados/banco-privado.md)), então o que o passo 2 faz com bancos antigos deve ser conferido nessa página.

## Configuração

Não há arquivo de configuração. Os ajustes abaixo existem como comandos do shell Tauri ou no `.env` externo:

| Ajuste | Como definir | Onde fica | Fundamento |
|---|---|---|---|
| Pasta do acervo | comando `set_library_path` | tabela `settings` do SQLite | `AcademicStore::set_library_path` em `storage.rs` |
| Executável do Chromium/Chrome | comando `set_chromium_path` (aceita o `.app`) | chave `chromium_executable` em `settings` | `app/src-tauri/src/lib.rs` |
| Limites da captura do Canvas | comando `set_canvas_capture_limits` | `settings` (`canvas_capture_limits`) | `storage.rs` |
| Chave e modelo da API Anthropic | Arquivo `.env` fixo em `$HOME/Dev/Projetos/gerenciador-de-conteudo/.env`, com `ANTHROPIC_API_KEY` e, opcionais, `ANTHROPIC_MODEL` e `ANTHROPIC_WORKSPACE_ID` | fora deste repositório | `AnthropicConfig::read_default` em `anthropic.rs` |

O caminho do `.env` é fixo no código e pertence a outro projeto. Só é lido quando se pedem resumos por IA. Ver [resumos-anthropic.md](../automacoes/resumos-anthropic.md) antes de usar: esse recurso envia texto das matérias a um serviço externo.

O perfil do Chromium do aplicativo fica em `app_data_dir/chromium-profile`, separado do navegador pessoal.

## Onde os dados ficam

| Dado | Local | No Git |
|---|---|---|
| Banco do aplicativo | `<acervo>/.estuda-uva.sqlite3` (WAL) | não (`*.sqlite3*` é proibido no repositório) |
| Espelhos de sincronização | `<acervo>/.estuda-uva/mirrors/` | não |
| Capturas do Portal e do Canvas | `<acervo>/materias/catalogo/capturas/` | hoje ignorado pelo `.gitignore`; alinhar com a política de `privado/` é decisão pendente |
| Perfil do Chromium | `app_data_dir/chromium-profile` | fora do repositório |

Detalhes: [visao-geral.md](../dados/visao-geral.md).

## Acervo reorganizado

Matérias, cursos, planejamento e registro acadêmico foram para `privado/`. O escopo de varredura do núcleo (`catalog/scanner.rs`) ainda lista `cursos/` e arquivos de registro na raiz e **não** lista `privado/`. Pela leitura do código, a Biblioteca do GPUI e qualquer outro consumidor do `CatalogService` não indexam `privado/` enquanto esse escopo não for atualizado. Ver [decisoes-pendentes.md](../arquitetura/decisoes-pendentes.md).
