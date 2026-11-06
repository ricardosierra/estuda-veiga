# Desempenho e limites

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` + árvore não commitada.

## Estado da medição

**Não existe suíte de benchmark, perfil de desempenho nem orçamento de desempenho no repositório.** Nenhum teste mede tempo, memória ou vazão; os 152 testes de `estuda-core` (contagem de 2026-11-06, antes do esquema v8) rodaram em 3,74 s no total, mas isso mede a execução dos testes, não o aplicativo com o acervo real. Este documento registra apenas os **limites que existem no código** e custos que se deduzem da leitura dele. Qualquer número de latência ou de tamanho do acervo em uso real é **Não verificado**.

## Limites implementados

### Acervo local (`app/crates/estuda-core/src/catalog/`)

| Limite | Valor | Constante | Efeito ao exceder |
|---|---|---|---|
| Arquivos por varredura | 10 000 | `MAX_FILES` (`scanner.rs`) | Varredura marcada `truncated`/`partial` |
| Profundidade de diretórios | 16 | `MAX_DEPTH` | Subárvore não é percorrida |
| Tamanho de documento | 64 MiB | `MAX_DOCUMENT_BYTES` (`scanner.rs`, `documents.rs`) | Documento não é lido/pesquisável |
| Tamanho de asset (imagem local) | 16 MiB | `MAX_ASSET_BYTES` | `read_asset` recusa |
| Texto total indexado em memória | 128 MiB | `MAX_INDEX_TEXT_BYTES` | Documento fica `index_limit`, não pesquisável, snapshot `partial` |
| Resultados por pesquisa | 100 | `MAX_SEARCH_RESULTS` | Resultado truncado |

### Sincronização local (`sync.rs`, `sync_watcher.rs`)

| Limite | Valor | Observação |
|---|---|---|
| Tamanho de arquivo sincronizado | 64 MiB (`MAX_SYNC_FILE_BYTES`) | Erro por arquivo no `SyncReport`, sem abortar a rodada |
| Extensões sincronizadas | `md`, `markdown`, `txt`, `alg`, `csv`, `json` | `reconcile_all` aceita o mesmo conjunto (o monitor e o motor usam listas equivalentes) |
| Silêncio para agrupar eventos | 500 ms (`QUIET_PERIOD`) | Rajadas de eventos viram uma única reconciliação |
| Espera máxima por evento | 200 ms | Define a latência de encerramento do monitor (`Drop`) |
| Pastas monitoradas | raiz (sem recursão), `materias/`, `docs/` | Mudanças em `fontes-uva/` e `periodos/` não disparam o monitor; entram na próxima reconciliação manual ou disparada por outro evento |

Custo deduzido do código: cada reconciliação chama `scan_library` e, para cada arquivo em escopo, `reconcile_path`, que lê o arquivo e seu espelho. O custo cresce com o número de arquivos, não só com os alterados. Cada `refreshLocalLibrary` na UI (na abertura, a cada `local-sync-updated` e ao fim de capturas) executa `sync_local_catalog` e `refresh_local_catalog`, que **relê e reindexa todos os documentos pesquisáveis**. Sem medição, não se sabe a partir de qual tamanho de acervo isso incomoda.

### Download e importação de PDF (`download.rs`, `canvas_capture.rs`)

| Limite | Padrão | Máximo aceito |
|---|---|---|
| Tamanho do PDF | 60 MiB | 60 MiB |
| Redirecionamentos (seguidos manualmente) | 4 | 4 |
| Tempo por requisição | 45 s | 5–120 s |
| `maxCoursePages` | 100 | 1–100 |
| Cursos por captura (`maxCourses`) | 200 | 1–500 |
| Itens por captura (`maxItems`) | 10 000 | 1–20 000 |
| Tempo por item (`itemTimeoutSeconds`) | 45 s | 5–120 s |

Os limites da captura são configuráveis (`set_canvas_capture_limits`) e validados por `CanvasCaptureLimits::validate`. A captura processa **um curso/item por vez** (sequencial), e há no máximo **uma captura ativa** (`active_captures`).

### Leitura do Canvas pela API (`canvas_api.rs`)

| Limite | Valor |
|---|---|
| Cursos lidos | 40 (`MAX_COURSES`) |
| Páginas de listagem por requisição de lista | 10 (`MAX_LIST_PAGES`) |
| Consultas de comentários por curso | 40 (`MAX_COMMENT_LOOKUPS_PER_COURSE`) |
| Tópicos de fórum por curso | 60 (`MAX_TOPICS_PER_COURSE`) |
| Entradas por tópico | 2 000 (`MAX_ENTRIES_PER_TOPIC`) |
| Conversas da Caixa de Entrada | 100 (`MAX_CONVERSATIONS`) |
| Texto de campos longos | cortado em 20 000 caracteres (`MAX_TEXT_CHARS`, constante privada) |

### Portal/Lyceum (`lyceum.rs`, `browser.rs`)

| Limite | Valor |
|---|---|
| Páginas por captura | 50 (`MAX_PORTAL_PAGES`) |
| Linhas por tabela | 10 000 (`MAX_TABLE_ROWS`); acima disso a captura é marcada parcial |
| Links visíveis lidos por página | 2 000 (`slice(0, 2000)` no script injetado) |
| Tamanho do arquivo de captura | 64 MiB (JSON e Markdown) |

### Navegador (`browser.rs`)

Inicialização do Chromium 20 s; abertura da primeira página 20 s; fechamento, espera de saída e `kill`, 5 s cada.

### IA (`anthropic.rs`, `study_summaries.rs`)

Texto de fontes enviado a um resumo: até 2 MiB (`MAX_SUMMARY_SOURCE_BYTES`); tempo da requisição à API: 120 s.

## Concorrência relevante para desempenho

- `AcademicStore`, `SyncEngine` e `CatalogService` usam `std::sync::Mutex`; chamadas longas seguram o lock. Por isso os comandos pesados rodam em `spawn_blocking` (ver [eventos e estado](../arquitetura/eventos-e-estado.md)).
- O monitor roda em uma thread própria e chama `reconcile_all` de forma síncrona; uma reconciliação longa atrasa as seguintes.
- Duas conexões SQLite (WAL) para o mesmo arquivo: leituras não bloqueiam escritas, mas escritas simultâneas se serializam.

## Como medir (Planejado)

Nada abaixo existe hoje; são propostas para uma etapa futura:

1. Um comando ou teste ignorado por padrão (`#[ignore]`) que cria um acervo sintético (por exemplo, 1 000, 5 000 e 10 000 arquivos Markdown em pasta temporária) e mede `scan_library`, `CatalogService::refresh`, `search` e `reconcile_all`.
2. Registro de memória residente durante `refresh` com o limite de 128 MiB.
3. Orçamento por operação (por exemplo, pesquisa local abaixo de um valor a definir) escrito aqui **somente depois** de medido.
4. Medição de abertura da UI (Tauri e, futuramente, GPUI) com o acervo real, feita pela pessoa responsável, sem copiar dados pessoais para o repositório.
