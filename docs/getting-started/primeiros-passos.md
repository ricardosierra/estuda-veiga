# Primeiros passos e configuração

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

Este guia descreve como o Estuda UVA é usado hoje, com a interface Tauri e React. Nenhum passo abaixo foi executado nesta revisão: o aplicativo não foi aberto. As descrições vêm do código (`app/src/ui/App.tsx`, `app/src-tauri/src/lib.rs`) e do guia anterior `app/README.md`. Todos os passos de interface são, portanto, **Não verificado** em execução. As telas estão descritas em [telas.md](../aplicativo/telas.md).

## Usar só o acervo, sem o aplicativo

Nada exige o aplicativo para ler ou validar o acervo.

1. Leia os índices em [`periodos/README.md`](../../periodos/README.md), [`materias/README.md`](../../materias/README.md) e [`fontes-uva/README.md`](../../fontes-uva/README.md).
2. Os textos extraídos de PDFs oficiais ficam em `fontes-uva/<período>/<matéria>/`, com a página e a fonte no próprio arquivo.
3. Os registros estruturados estão em `dados/`, um JSON por arquivo. Para conferi-los: `node tools/acervo/build.mjs --validar`.

## Abrir o aplicativo

```sh
cd app
npm install
npm run tauri dev      # Não verificado nesta revisão
```

Na inicialização (`run()` em `app/src-tauri/src/lib.rs`) o aplicativo:

1. escolhe como acervo a pasta `../..` a partir de `app/src-tauri`, isto é, a raiz do repositório, resolvida em tempo de compilação por `CARGO_MANIFEST_DIR`;
2. abre ou cria o SQLite em `<acervo>/.estuda-uva.sqlite3` e, se existir, importa uma única vez o banco antigo de `app_data_dir/estuda-uva.sqlite3`;
3. cria `AGENTS.md` e `CLAUDE.md` na raiz do acervo **somente se não existirem** (`catalog::guidance::create_missing_guidance`);
4. indexa os documentos, abre o motor de sincronização e inicia o monitor de arquivos.

Como o caminho do acervo padrão é fixado na compilação, um `.app` copiado para outro lugar ainda aponta para o diretório de onde foi compilado, a menos que o estudante troque a pasta (ver abaixo).

## Fluxo básico

| Objetivo | Onde | Resultado |
|---|---|---|
| Conferir o que o app encontrou | **Dados e sync** | Lista de documentos, pesquisa, estado do Git. O app só mostra o estado do Git; não faz stage, commit nem push. |
| Conciliar arquivos e SQLite | **Dados e sync** → sincronizar | Relatório com arquivos lidos, criados, atualizados, conflitos e operações pendentes. |
| Revisar conflito | **Dados e sync** | As versões divergentes (SQLite, Markdown, JSON) ficam preservadas até uma decisão humana. |
| Estudar um material já capturado | **Matérias** → matéria → material | Aula local com perguntas. Funciona sem Canvas. |
| Ver o que já estudou | **Histórico** | Sessões locais de estudo, para retomar. |
| Calcular a nota final | **Acadêmico** | Estimativa local; a nota oficial é a do Lyceum. |

Capturar materiais do Canvas e consultar o Portal exigem login manual; ver [canvas.md](../automacoes/canvas.md) e [portal-lyceum.md](../automacoes/portal-lyceum.md).

## Configuração

Não há arquivo de configuração do aplicativo. O que existe:

| Ajuste | Como definir | Onde fica | Fundamento |
|---|---|---|---|
| Pasta do acervo | Tela **Dados e sync** (comando `set_library_path`) | tabela `settings` do SQLite | `AcademicStore::set_library_path` em `storage.rs` |
| Executável do Chromium/Chrome | Escolha de arquivo (comando `set_chromium_path`), aceita o `.app` | chave `chromium_executable` em `settings` | `src-tauri/src/lib.rs` |
| Limites da captura do Canvas | Painel de captura (`set_canvas_capture_limits`) | `settings` | `storage.rs` |
| Chave e modelo da API Anthropic | Arquivo `.env` fixo em `$HOME/Dev/Projetos/gerenciador-de-conteudo/.env`, com `ANTHROPIC_API_KEY` e, opcionais, `ANTHROPIC_MODEL` e `ANTHROPIC_WORKSPACE_ID` | fora deste repositório | `AnthropicConfig::read_default` em `anthropic.rs` |

O caminho do `.env` da Anthropic é fixo no código e pertence a outro projeto. Só é lido quando o estudante pede resumos por IA. Ver [resumos-anthropic.md](../automacoes/resumos-anthropic.md) antes de usar, porque esse recurso envia texto das matérias a um serviço externo.

O perfil do Chromium do aplicativo fica em `app_data_dir/chromium-profile`, separado do navegador pessoal.

## Onde os dados ficam

| Dado | Local | No Git |
|---|---|---|
| Banco do aplicativo | `<acervo>/.estuda-uva.sqlite3` (WAL) | não |
| Espelhos de sincronização | `<acervo>/.estuda-uva/mirrors/` | não |
| Capturas do Portal e do Canvas | `<acervo>/materias/catalogo/capturas/` | não |
| Perfil do Chromium | `app_data_dir/chromium-profile` | fora do repositório |

Detalhes: [visao-geral.md](../dados/visao-geral.md).

## Atenção: acervo reorganizado

O repositório foi reorganizado: matérias, cursos, planejamento e registro acadêmico foram para `privado/`. O escopo de varredura e de sincronização do núcleo ainda lista `cursos/` e arquivos de registro na raiz, e **não** lista `privado/`. Na prática, o app não indexa o que está em `privado/` enquanto o código não for atualizado. Ver [decisoes-pendentes.md](../arquitetura/decisoes-pendentes.md).
