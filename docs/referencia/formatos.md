# Formatos e convenções

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

Convenções de nomes, identificadores, texto e dados do acervo. Cada convenção é marcada:

- **Implementado**: validada por código (cita o arquivo) ou aplicada pelo app.
- **Praticado**: observado nos arquivos, sem validação automática.
- **Proposta**: ainda não existe no código nem nos dados; é sugestão para os mantenedores.

Relacionados: [visão geral dos dados](../dados/visao-geral.md) · [proveniência e confiança](../dados/proveniencia-e-confianca.md) · [sincronização](../dados/sincronizacao.md) · [banco público](../dados/banco-publico.md).

## 1. Princípios

1. **Texto primeiro.** O que é versionado é texto: Markdown, JSON, CSV e `.txt` (`AGENTS.md`, `tools/acervo/src/texto.mjs`). PDF, Word, Pages e imagens ficam fora do Git (`.gitignore`) e são verificados por `node tools/acervo/build.mjs --verificar-texto`. **Implementado.**
2. **Formatos abertos e portáveis.** UTF-8, Markdown, JSON, Mermaid como texto-fonte. **Praticado.**
3. **Um conteúdo, um lugar.** Quem precisa de um texto aponta para ele por link relativo; não copia (`ARQUITETURA_ACERVO.md` §1). **Praticado.**
4. **Ausência é `null` com pendência**, nunca `""` nem `0` inventado. **Implementado** (`validar.mjs`).
5. **Não renomear identificadores existentes**; outros registros os referenciam (`dados/LEIA-ME.md`). **Praticado.**

## 2. Arquivos, diretórios e slugs

| Convenção | Estado | Fonte |
|---|---|---|
| Registros JSON: um por arquivo; nome do arquivo igual ao slug do `id` (ou ao id numérico do curso Canvas em `canvas/`) | **Implementado só para `canvas/`** (o nome deve ser o id; erro `nome do arquivo deve ser o id do curso Canvas`). Para os demais tipos é **Praticado** | `validar.mjs`, `dados/LEIA-ME.md` |
| Tipo do registro = primeira pasta depois de `dados/` ou `privado/dados/`; `meta.json` na raiz de `dados/` é o tipo `meta` | **Implementado**. Pasta desconhecida é erro (`pasta/tipo não reconhecido`) | `carregar.mjs`, `validar.mjs` |
| Pastas e arquivos em minúsculas, palavras separadas por hífen, sem acento (`bases-de-programacao`, `capitulo-01.md`) | **Praticado**, com exceções abaixo | árvore do repositório |
| Documentos de pasta nomeados `README.md`; documentos de documentação principal em maiúsculas (`AGENTS.md`, `CLAUDE.md`, `LEIA-ME.md`) | **Praticado** | |
| Pasta por período: `AAAA.S` (`2026.1`, `2026.2`) e `historicos` | **Praticado** | `periodos/`, `fontes-uva/` |
| Matéria: pasta canônica única; nomes alternativos são links simbólicos | **Praticado** (`ARQUITETURA_ACERVO.md` §3) | `materias/` |
| Texto extraído de PDF fica em `fontes-uva/<período>/<matéria>/`; a pasta da matéria em `materias/` aponta para ele | **Praticado** | `fontes-uva/README.md` |
| Saídas geradas (`build/`, `materias/catalogo/*.json`, `materias/catalogo/curriculos/`) nunca são editadas à mão | **Implementado** (regeneradas por `tools/acervo`; ignoradas pelo Git) | `.gitignore` |
| Capturas autenticadas ganham nomes novos e únicos: `materias/catalogo/capturas/lyceum-<uuid>.json` e `.md`, `canvas-<uuid>.json` e `.md` (`app/README.md` descreve `canvas-<id>`, mas o código usa UUID: `canvas_api.rs`); nenhuma captura anterior é substituída | **Implementado** | `lyceum.rs` (`persist_capture`), `canvas_api.rs` |

**Exceções observadas ao padrão de nomes** (2026-11-06): 7 arquivos de `fontes-uva/` têm espaço no nome (por exemplo `G_EST_Lista exercicios_U3-p3.md`) e 61 arquivos `.md` de `fontes-uva/`, `materias/` e `privado/materias/` têm letras maiúsculas (por exemplo `EST_ebook_Walter_Sande.md`, `BPR_Cap_01.md`). Espaços em nomes dificultam links relativos em Markdown; use `%20` ou renomeie **sem** quebrar referências.

### Slugs (Proposta de regra)

Não há função de slug validada para arquivos do acervo. Há uma para materiais importados pelo app: `pdf::safe_slug` (`app/crates/estuda-core/src/pdf.rs`), cujo comportamento testado é `"Introdução à Web / U1"` → `introducao-a-web-u1` e entrada sem caracteres úteis → `material`. Regra proposta para novos arquivos: minúsculas ASCII, dígitos e hífens, sem acento, máximo de 80 caracteres, sem espaço.

## 3. Identificadores

### 3.1 Formas em uso (contagem de 2026-11-06)

| Tipo de registro | Forma do `id` | Exemplo sintético | Registros |
|---|---|---|---|
| instituição | `<slug>` | `uva` | 1 |
| programa | `uva:program:<slug>` | `uva:program:curso-exemplo` | 2 |
| disciplina (catálogo) | `uva:program:<programa>:discipline:<slug>` | `uva:program:curso-exemplo:discipline:calculo-exemplo` | 73 |
| disciplina (nome canônico) | `uva:discipline:<slug>` | `uva:discipline:calculo-exemplo` | 3 em `dados/`, 12 em `privado/dados/` |
| oferta curricular | `uva:program:<programa>:offering:<código>` | `uva:program:curso-exemplo:offering:E00000001` | 79 |
| relação | `uva:relation:<tipo>:<slug>` | `uva:relation:same-subject-candidate:calculo-exemplo` | 14 |
| período | `<AAAA>.<S>` (sem prefixo) | `2026.2` | 2 |
| curso do Canvas | id numérico do Canvas | `12345` | 14 |
| matrícula | `uva:program:<programa>:<tipo>:<período>:<disciplina>` | `uva:program:curso-exemplo:enrollment:2026.2:disciplina-exemplo` | 12 |
| nota | `uva:program:<programa>:<tipo>:<período>:<disciplina>` (mesma forma, `<tipo>` diferente) | — | 12 |
| trabalho | `uva:trabalho:<slug>` | `uva:trabalho:exemplo-ava1` | 7 |
| trajetória | `uva:trajectory:<programa>:<código do programa>` | `uva:trajectory:curso-exemplo:000000` | 2 |

Documentos de sincronização usam `doc-<sha256 do caminho relativo>` (`sync.rs`, `stable_id`). Chaves de material usam `pdf:<sha256>` ou `canvas_page:<sha256>` (`canvas.rs`, `content_key`) e `ref:<sha256>` para referência de item (`reference_key`).

### 3.2 O contrato `estuda-domain` (Planejado, não integrado)

`crates/estuda-domain/src/ids.rs` aceita: caracteres `a-z`, `0-9`, `:`, `-`, `_`, `.`, até 200 caracteres, e a forma `<namespace>:<tipo>:<slug>` com o tipo exigido:

- `DegreeProgramId`: `uva:program:<slug>`
- `SubjectId`: `uva:discipline:<slug>`
- `AcademicTermId`: `uva:term:2026.2`
- `LmsCourseId::canvas`: só dígitos, até 20 caracteres.

**Divergências com os dados atuais:**

- 73 dos 76 ids de `dados/disciplinas/` têm a forma `uva:program:<programa>:discipline:<slug>`: `SubjectId::parse` os recusa (`WrongShape`), porque o segundo segmento é `program`.
- O período nos dados é `2026.2`, não `uva:term:2026.2`.
- Ofertas usam códigos em maiúsculas (`E10100003`) que o conjunto de caracteres permitido recusa, e não há tipo de id para oferta, relação, trabalho ou trajetória.
- A proposta de arquitetura alvo diz que os ids existentes "são aceitos como estão"; o teste `parses_existing_acervo_ids` só cobre as formas `uva:discipline:`, `uva:program:` e `uva:term:`. **Contradição a resolver antes de integrar o crate.**

## 4. Texto: codificação, Markdown, tabelas, citações, referências, fórmulas

| Tema | Convenção | Estado |
|---|---|---|
| Codificação | UTF-8, sem BOM. A sincronização recusa arquivo que não é UTF-8 (`sync.rs`) | **Implementado** |
| Fim de linha | `\n` | **Praticado**; o sincronizador preserva "newline final" de JSON explicitamente (`<!-- estuda-uva-json final-newline=... -->`) |
| Markdown | CommonMark com tabelas GFM; o app renderiza com `react-markdown` e `remark-gfm` (`app/package.json`) | **Implementado** no app |
| Título | um `# Título` por arquivo | **Praticado** |
| Páginas de PDF | `## Página N` para cada página, com `_Página sem texto selecionável._` quando vazia; cabeçalho com origem e data. Há duas gerações: `pdf-para-md.mjs` e o importador do app (`pdf::pages_to_markdown`) usam esse formato; alguns textos antigos de `fontes-uva/2026.1/` não têm marcas de página (0 ocorrências em `unidade-u1.md`) | **Implementado** para novos; **Praticado** irregularmente nos antigos |
| Origem de texto extraído | linha de citação (`> Texto extraído de ...`) com nome do original, número de páginas, ferramenta e URL oficial **sem query** | **Implementado** (`pdf-para-md.mjs`; query removida por `sanitize_source_url`) |
| Tabelas | tabelas GFM; barras verticais escapadas (`\|`) em células | **Implementado** nas capturas (`md_cell`, `lyceum.rs`) |
| Diagramas | **Mermaid** como texto-fonte em bloco ```` ```mermaid ````, sempre com explicação textual; imagem nunca é a única representação (`AGENTS.md`) | Regra **Implementada** (política); **o app não renderiza Mermaid** (sem dependência no `package.json`: o bloco aparece como código) |
| Fórmulas | **Proposta:** LaTeX entre `$...$` (em linha) e `$$...$$` (bloco), com a leitura em palavras logo depois. O repositório não contém fórmulas nesse formato hoje, e o app **não renderiza LaTeX** (não há `remark-math`) | **Proposta** |
| Referências cruzadas | links relativos entre arquivos Markdown (`../../periodos/2026.2/README.md`); o verificador de links roda sobre os `.md` (`ARQUITETURA_ACERVO.md` §4). Em JSON, `source.file` é caminho relativo à raiz e precisa existir | **Implementado** (JSON) / **Praticado** (Markdown). **Não verificado:** localizar o programa do verificador de links de Markdown nesta revisão |
| Citações | citar fonte oficial por URL **sem query nem fragmento**, número de página e data de coleta; nunca colar URL assinada, token ou cookie | **Implementado** nas capturas e importações; **Proposta** para texto escrito à mão |
| Links externos oficiais | `https://portalaluno.uva.br/Aluno/PortalAluno`, `https://uva.instructure.com/courses/<id>` | **Implementado** (a URL do Canvas é validada contra o id: `URL_CANVAS` em `validar.mjs`) |

## 5. JSON dos registros

### 5.1 Regras validadas (`validar.mjs`)

1. O arquivo é um **objeto** JSON.
2. Campos obrigatórios por tipo (`ESPECIFICACAO`): por exemplo `canvas` exige `id, titulo, url, tipo, situacao, periodo, codigo, secao, source`; `matriculas` exige `id, programId, semester, title, section, status, disciplinaId, canvasCourseId, docente, cargaHoraria, source`.
3. Valores enumerados: `periodos.situacao` ∈ {`encerrado`, `em_andamento`, `previsto`}; `canvas.tipo` ∈ {`curricular`, `apoio`, `gratuito`}; `canvas.situacao` ∈ {`em_andamento`, `encerrada`, `apoio`, `gratuito`}; `matriculas.status` ∈ {`enrolled`, `closed`}; `notas.status` ∈ {`in_progress`, `closed`}.
4. `null` só com pendência correspondente; `""` é erro; `pendencias` é uma lista de `{ "campo", "motivo" }` com ambos textos e `motivo` não vazio.
5. `id` único por tipo; referências cruzadas existem (`programId`, `disciplinaId`, `canvasCourseId`, `periodo`, `semester`, `matriculaId`, `ccOfferingId`, `ecOfferingId`, `canonicalDisciplineId`, `offeringIds`).
6. Fontes (`source.file`, `sourceFiles`, `arquivo`) apontam para arquivos existentes.
7. **Proibições:** chaves `cpf`, `rg`, `ra`, `senha`, `password`, `pass`, `cookie(s)`, `token(s)`, `secret`, `credencial(is)`, `email`, `e-mail`, `telefone`, `login`, `usuario`/`usuário` (em qualquer profundidade) e valores que contenham `/Users/`, `/home/`, `C:\`, `Bearer ` ou e-mail.
8. Em `canvas`, `url` deve ser exatamente `https://uva.instructure.com/courses/<id>`.

### 5.2 Estilo praticado

- Chaves em `camelCase` nos registros de `dados/` (`programId`, `workloadHours`, `equivalenceConfirmed`), com exceções em português (`titulo`, `situacao`, `periodo`, `docente`, `cargaHoraria`, `pendencias`, `ordem`). **Praticado**: mistura de português e inglês, sem regra.
- Campo `ordem` numérico opcional para ordenar saídas (`legado.mjs`, função `ordenar`).
- Indentação de 2 espaços e quebra de linha final. **Praticado.**

### 5.3 Exemplo sintético válido

```json
{
  "id": "uva:program:curso-exemplo:offering:E00000001",
  "code": "E00000001",
  "title": "Disciplina de Exemplo",
  "workloadHours": 66,
  "status": "pending",
  "canonicalDisciplineId": "uva:discipline:disciplina-exemplo",
  "programId": "uva:program:curso-exemplo",
  "source": {
    "file": "docs/NAVEGACAO_PORTAL_UVA.md",
    "section": "Seção de exemplo",
    "observedAt": "2026-11-06"
  },
  "ordem": 0,
  "pendencias": []
}
```

Este exemplo não é um registro real e não deve ser copiado para `dados/` (as referências `programId` e `canonicalDisciplineId` não existem).

### 5.4 Formato legado para o app

`tools/acervo/src/legado.mjs` gera, em `materias/catalogo/`, cinco arquivos com `schemaVersion: 1`: dois currículos, `disciplinas.json`, `relacoes.json` e `trajetorias.json`. São a entrada do `catalog::academic` do app. Não edite; veja [sincronização](../dados/sincronizacao.md), seção de riscos.

## 6. Metadados de material (Proposta)

Contrato **proposto** para catalogar materiais de uma futura biblioteca. **Nada disto existe hoje** no código, nos dados nem em validação. Ele reaproveita as convenções que já existem (`source`, `pendencias`, `null` com pendência).

| Campo | Tipo | Obrigatório | Observação |
|---|---|---|---|
| `id` | texto | sim | estável, nunca derivado só do título; sugestão `uva:material:<slug>` |
| `title` | texto | sim | |
| `kind` | enumeração | sim | `disciplina`, `aula`, `resumo`, `exercicio`, `prova`, `trabalho`, `solucao`, `referencia` |
| `subjectId` | texto | sim | `uva:discipline:<slug>` |
| `programIds` | lista | não | cursos relacionados |
| `institution` | texto | não | quando aplicável |
| `topics` | lista | não | assuntos |
| `tags` | lista | não | |
| `authors` | lista | sim | autoria; para soluções, autoria **autorizada** |
| `source` | objeto | sim | `{ "file", "section", "observedAt" }` como nos registros atuais |
| `sourceRef` | texto | não | referência de origem (URL sem query, citação bibliográfica) |
| `sourceDate` | data | não | data da fonte (ISO 8601) |
| `collectedAt` | data-hora | sim | data da coleta (ISO 8601 em UTC) |
| `version` | texto | sim | versão do material |
| `sha256` | texto | sim | hash do texto normalizado, para integridade |
| `language` | texto | sim | BCP 47 (`pt-BR`) |
| `format` | texto | sim | `md`, `json`, `csv`, `txt` |
| `license` | texto | sim | identificador SPDX ou `proprietary-private`; **desconhecido não é público** |
| `verificationStatus` | enumeração | sim | veja a seção 8 |
| `publicationStatus` | enumeração | sim | `private`, `needs_review`, `approved`, `published`, `withdrawn` |
| `pendencias` | lista | sim | `{ "campo", "motivo" }` |

Regras propostas:

- `license` desconhecida, ausente ou restrita implica `publicationStatus: "private"`. Material com direitos de distribuição desconhecidos **não** é apresentado como público.
- Solução só avança para `approved` com autoria autorizada registrada e revisão humana.
- Dados de matrícula, notas privadas, credenciais e materiais acadêmicos não autorizados não entram em nenhum registro de biblioteca.

Exemplo sintético de arquivo auxiliar `<slug>.meta.json` ao lado do texto:

```json
{
  "id": "uva:material:exemplo-lista-1",
  "title": "Lista de exercícios de exemplo",
  "kind": "exercicio",
  "subjectId": "uva:discipline:disciplina-exemplo",
  "programIds": [],
  "institution": null,
  "topics": ["tópico de exemplo"],
  "tags": ["exemplo"],
  "authors": ["Autor Fictício"],
  "source": {
    "file": "docs/referencia/formatos.md",
    "section": "6. Metadados de material (Proposta)",
    "observedAt": "2026-11-06"
  },
  "sourceRef": null,
  "sourceDate": null,
  "collectedAt": "2026-11-06T12:00:00Z",
  "version": "1",
  "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
  "language": "pt-BR",
  "format": "md",
  "license": "CC-BY-4.0",
  "verificationStatus": "unverified",
  "publicationStatus": "private",
  "pendencias": [
    { "campo": "institution", "motivo": "exemplo sem instituição" },
    { "campo": "sourceRef", "motivo": "exemplo sem referência de origem" },
    { "campo": "sourceDate", "motivo": "exemplo sem data de fonte" }
  ]
}
```

## 7. Datas, horas e fusos

| Contexto | Formato observado | Estado |
|---|---|---|
| `source.observedAt` e datas de registro (`date` em avaliações) | `AAAA-MM-DD` | **Praticado** (sem validação) |
| `startDate` de matrículas | `DD/MM/AAAA` | **Praticado**; inconsistente com o restante (lacuna) |
| Capturas do Portal e do Canvas (`capturedAt`) | ISO 8601 em UTC com `Z`: `2026-11-06T12:00:00Z` (`lyceum::captured_at_utc`) | **Implementado** |
| Datas de prazo do Canvas (`due_at`) | texto recebido da API; dia inteiro quando não há `T` (`calendar.rs`); `valid_timestamp` valida | **Implementado** |
| Colunas `*_at` criadas por `CURRENT_TIMESTAMP` do SQLite | `AAAA-MM-DD HH:MM:SS` em UTC, sem `Z` | **Implementado** (`storage.rs`) |
| `sync_documents.updated_at`, `updatedAt` do envelope, `sync_conflicts.created_at` | segundos Unix em texto (`1793900000`) | **Implementado** (`sync.rs`, `timestamp()`) |
| Período letivo | `AAAA.S` (`2026.2`); validado em `calendar::valid_semester` para a agenda | **Implementado** |

Fuso: nenhum campo guarda fuso explícito além do `Z` das capturas. Valores `AAAA-MM-DD` são datas de calendário sem hora. **Proposta:** usar ISO 8601 em UTC com `Z` para instantes (`collectedAt`) e `AAAA-MM-DD` para datas de calendário; converter `startDate` para ISO ao reler os registros; registrar a hora local de Brasília (UTC-03:00) apenas na apresentação.

## 8. Estados de validação

### 8.1 Do registro (Implementado)

`node tools/acervo/build.mjs --validar` retorna "OK: N registros válidos em dados/." ou a lista de erros `caminho [campo]: motivo`. O resultado de 2026-11-06 foi **OK: 234 registros válidos** (`dados/` 174 e `privado/dados/` 60). Os 13 testes `node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs` passaram na mesma data.

### 8.2 Do dado, quanto a confiança (Planejado)

`Confirmed`, `Estimated`, `Unknown`, `Conflicting` (`estuda-domain`). Veja [proveniência e confiança](../dados/proveniencia-e-confianca.md).

### 8.3 Do material, quanto a verificação (Proposta)

| Valor de `verificationStatus` | Significado proposto |
|---|---|
| `unverified` | coletado, sem conferência |
| `text_checked` | texto conferido contra a fonte (página e conteúdo) |
| `reviewed` | revisado por uma pessoa responsável |
| `disputed` | há objeção ou fontes divergentes |
| `rejected` | não deve ser usado |

### 8.4 Da sincronização (Implementado)

`SyncReport`: `scanned`, `created`, `updated`, `unchanged`, `conflicts`, `pendingOperations`, `errors`; por documento: sem conflito, em conflito, em restauração (tombstone). Veja [sincronização](../dados/sincronizacao.md).

## 9. Como conferir uma mudança

1. `node tools/acervo/build.mjs --validar`: valida `dados/` e `privado/dados/` sem gerar arquivos.
2. `node tools/acervo/build.mjs --verificar-texto`: aponta binários rastreados (erro) e locais (aviso).
3. `node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs`: testes do gerador (usam pastas temporárias; um deles lê o acervo real somente para validação).
4. Validar à mão cada bloco JSON novo de documentação, por exemplo `python3 -c "import json,sys; json.load(sys.stdin)" < arquivo.json`.

Rodar `node tools/acervo/build.mjs` **sem** `--validar` regera `build/` e `materias/catalogo/` e deve ser feito de propósito.
