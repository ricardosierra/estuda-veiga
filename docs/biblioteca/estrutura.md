# Estrutura da biblioteca acadêmica

**Revisado em:** 2026-11-06 · **Base:** `HEAD` `e29f27b` (separação em `privado/` já commitada) mais alterações não commitadas na árvore de trabalho (extração de `estuda-core`).

Esta página descreve o que **existe hoje** no repositório e separa, em seção própria, o que é **planejado**. Rótulos usados: **Implementado**, **Parcial**, **Planejado**, **Não verificado**.

## 1. Resumo

- **Não existe uma "biblioteca pública" formal.** Não há pasta `biblioteca/` nem `schemas/` (conferido com `ls`). O que existe é um acervo pessoal em texto, dividido em camadas por pasta.
- A camada pública candidata é: `dados/` (catálogo da instituição), `fontes-uva/` (texto extraído de materiais oficiais), `periodos/` (índices e notas de estudo), `docs/` e o código.
- A camada pessoal fica em `privado/` (mesmo repositório Git, separada por pasta do catálogo). Por decisão do dono (`AGENTS.md`, "Política de publicação", 6/11/2026), notas, frequência, matrículas e trabalhos **podem** ir ao Git e ao acervo público, desde que separados do catálogo. Ficam sempre fora do Git e de qualquer publicação: senhas, cookies, tokens, `.env`, bancos `*.sqlite3*`, perfil do Chromium e `originais/`. RA, CPF e RG seguem fora dos registros até nova decisão do dono.
- Não existe banco público. O único SQLite do gerador, `build/acervo.sqlite3`, é derivado e descartável (ver [../dados/visao-geral.md](../dados/visao-geral.md)).

## 2. Pastas reais

| Pasta | O que guarda | Versionada no Git | Camada (ver §3) |
|---|---|---|---|
| `dados/` | registros JSON, um por arquivo: `instituicao/` (1), `programas/` (2), `periodos/` (2), `disciplinas/` (76), `ofertas/<programa>/` (79), `relacoes/` (14) | sim | pública candidata, exige revisão |
| `fontes-uva/` | texto extraído de PDFs e documentos oficiais, por período e matéria, com página e link oficial; 95 arquivos rastreados | sim | pública candidata, com ressalva de direitos (§4) |
| `periodos/` | `README.md`, `futuros.md`, `2026.1/` e `2026.2/` (notas de estudo, calendário), `historicos/` | sim | pública candidata, exige revisão |
| `materias/` | hoje só `README.md` e `catalogo/` (JSON **gerado**); as pastas de matéria foram movidas para `privado/materias/` | sim, exceto `catalogo/capturas/` | mista |
| `privado/` | `cursos/`, `dados/` (matrículas, notas, trajetórias, trabalhos, Canvas), `materias/`, `periodos/`, `docs/`, `IDEIAS.md`, `PLANEJAMENTO.md`, `REGISTRO_ACADEMICO.md` | sim (88 arquivos rastreados); publicação é decisão do dono | pessoal (`Personal`), separada do catálogo |
| `originais/` | documentos Word, Pages e PDFs completos dos trabalhos, `binarios/` | **não** (`/originais/` no `.gitignore`; proibido sem exceção) | somente local (`LocalOnly`) |
| `build/` | `acervo.sqlite3` e `visualizacoes/*.md`, saída do gerador | **não** | gerada |
| `tools/acervo/` | gerador e validador (Node, sem dependências) | sim | código |
| `docs/` | documentação | sim | código/documentação |
| `app/`, `crates/` | aplicativo e crates Rust | sim | código |

Contagens de arquivos em `dados/` e `privado/dados/` foram feitas em 2026-11-06 com `find`; mudam a cada edição.

## 3. Camadas e o contrato `estuda-domain`

O crate `crates/estuda-domain` (`src/publication.rs`) define um contrato de classificação por caminho, atualizado em 2026-11-06 para refletir a decisão do dono:

| `Layer` | Significado | Prefixos reais no código |
|---|---|---|
| `LocalOnly` | nunca sai da máquina, por nenhuma política: bancos, perfis de navegador, `.env`, binários originais | prefixos `.estuda-uva`, `originais/`, `app/src-tauri/chromium-profile/`, `chromium-profile/`; sufixos `.sqlite3`, `.sqlite3-wal`, `.sqlite3-shm`, `.sqlite3-journal`, `.sqlite`, `.db`; arquivos `.env` e `.env.*` |
| `Personal` | dado de pessoa, separado do catálogo; publicável **somente** se a política permitir (`allow_personal`) | `privado/`, `cursos/`, `dados/matriculas/`, `dados/notas/`, `dados/trajetorias/`, `dados/trabalhos/`, `materias/catalogo/capturas/`; arquivos `REGISTRO_ACADEMICO.md`, `PLANEJAMENTO.md`, `IDEIAS.md`, `dados/meta.json` |
| `Generated` | saída reconstruível, não é fonte; nunca publicável | `build/`, `materias/catalogo/`, `app/dist/`, `app/node_modules/`, `app/src-tauri/target/`, `app/target/` |
| `Shareable` | código, documentação e catálogo sem dado de pessoa | `docs/arquitetura/`, `crates/`, `tools/`, `schemas/`, `biblioteca/`, `fontes-uva/`, `dados/instituicao/`, `dados/programas/`, `dados/ofertas/`, `dados/relacoes/`, `dados/periodos/` |
| `NeedsReview` | qualquer outro caminho, por exemplo `dados/disciplinas/`; só publicável se uma pessoa o aprovou | tudo que não casa acima |

`classify()` dá precedência a `LocalOnly`, depois `Personal`, `Generated` e `Shareable`. `PublicationPolicy` tem `allow_personal`: o padrão da biblioteca é `false` (conservador) e a constante `PublicationPolicy::OWNER` é `true`, a política adotada para este acervo em 6/11/2026. `check_publishable(paths, reviewed, policy)` nega por padrão: `LocalOnly` e `Generated` nunca são liberados; `Personal` só com `allow_personal`; a aprovação humana só desbloqueia `NeedsReview`. Há 9 testes em `publication.rs` (17 no crate `estuda-domain`, somando `ids.rs` e `confidence.rs`).

**Limitação (Parcial):** o contrato existe como biblioteca, mas **nenhum fluxo o chama**. O app (`estuda-core`) não depende de `estuda-domain` (conferido em `app/crates/estuda-core/Cargo.toml`), e o gerador Node não o usa. Hoje a separação depende de convenção de pastas, de `.gitignore` e das regras do validador. A decisão do dono cobre os dados do **próprio dono**; nomes de docentes e dados de outras pessoas, e os direitos de `fontes-uva/`, não têm decisão registrada e são tratados de forma conservadora (ver item 3 das divergências abaixo).

### Divergências encontradas entre o contrato e o repositório

1. O contrato classifica `cursos/` como `Personal`, mas a pasta foi movida para `privado/cursos/` e `cursos/` não existe mais na raiz. O prefixo é redundante, não prejudicial.
2. `materias/catalogo/` é `Generated`, mas `materias/catalogo/trajetorias.json` é gerado a partir de registros pessoais (`privado/dados/trajetorias/`) e pode conter dado de pessoa (é `Generated`, nunca publicável como fonte). O arquivo está no `.gitignore`; o dado pessoal que ele reflete é publicável, se o dono quiser, a partir de `privado/dados/trajetorias/`, não deste derivado.
3. `fontes-uva/` é `Shareable`, mas [../ARQUITETURA_ACERVO.md](../ARQUITETURA_ACERVO.md) §6 diz que textos extraídos de materiais da UVA e de e-books de terceiros não devem ser replicados sem permissão. As duas afirmações não podem valer ao mesmo tempo; ver [fontes-e-licencas.md](fontes-e-licencas.md).
4. `dados/` não deveria apontar para `privado/` segundo o cabeçalho de `tools/acervo/src/separar-privado.mjs` ("Registros de dados/ não podem apontar para privado/"), mas [../../dados/LEIA-ME.md](../../dados/LEIA-ME.md) permite ("Um registro de `dados/` pode citar um arquivo de `privado/` na fonte. Não o contrário."), e registros reais de `dados/disciplinas/`, `dados/ofertas/` e `dados/periodos/` citam `privado/REGISTRO_ACADEMICO.md` e `privado/periodos/…` em `source.file`. Isso viola a separação catálogo × `privado/` (regra que continua obrigatória): o catálogo não deveria depender de arquivo pessoal. Como a política do dono permite publicar `privado/`, o efeito é um acoplamento, não um vazamento; a decisão sobre a regra a seguir (LEIA-ME ou script) é do mantenedor.
5. `materias/README.md` e `periodos/README.md` (candidatos a públicos) contêm links relativos para `privado/…`. Se `privado/` não for publicado (a publicação é decisão do dono), esses links quebram.

## 4. O que o repositório já impõe

- **Texto, não binário.** `node tools/acervo/build.mjs --verificar-texto` falha se um binário (PDF, DOCX, imagem etc.) estiver rastreado pelo Git e avisa sobre binários locais (`tools/acervo/src/texto.mjs`). Resultado em 2026-11-06: 0 erros e 0 avisos.
- **Fonte obrigatória.** O validador exige `source.file` existente, `section` e `observedAt` em cada registro de `dados/`.
- **Ausência é `null` com pendência.** Nunca `""` nem `0`.
- **Sem dado sensível em `dados/`.** Chaves e valores proibidos pelo validador; detalhes em [../seguranca/privacidade.md](../seguranca/privacidade.md).
- **Um conteúdo, um lugar.** Matéria tem pasta canônica; textos oficiais têm uma cópia em `fontes-uva/` (`docs/ARQUITETURA_ACERVO.md` §3). O gerador **ainda não** detecta duplicatas por hash.

## 5. Planejado (não existe hoje)

| Item | Estado |
|---|---|
| Pastas `biblioteca/` e `schemas/` (prefixos `Shareable` no contrato) | **Planejado**; não criadas |
| Contrato de metadados por material ([catalogacao.md](catalogacao.md)) | **Planejado**; proposta |
| Banco SQLite público com atualização offline | **Planejado**; não existe. Não pode ser versionado no Git: bancos `*.sqlite3*` são proibidos (`AGENTS.md`); a distribuição teria de ser por outro meio ou por exportação em texto |
| Manifestos por período/matéria com hash SHA-256 e verificação pelo gerador | **Planejado** (fase 0 em `docs/ARQUITETURA_ACERVO.md` §6) |
| Assinatura Ed25519 de manifestos e sincronização entre pares | **Planejado**; decisões pendentes no mesmo documento |
| Comando `--verificar-duplicados` | **Planejado** |
| Integração de `estuda-domain::publication` ao gerador ou ao app | **Planejado**; não integrado |

## 6. Como o app enxerga a biblioteca

O escaneador do app (`app/crates/estuda-core/src/catalog/scanner.rs`) indexa `docs/`, `materias/`, `originais/`, `fontes-uva/`, `periodos/` e `cursos/` mais alguns arquivos de registro na raiz. **Não indexa `privado/` nem `dados/`.** Como `cursos/` e as pastas de matéria foram movidas para `privado/`, o app deixa de listar esse conteúdo até o escaneador ser atualizado. Isto é uma contradição entre código e layout (ver [../referencia/lacunas-e-contradicoes.md](../referencia/lacunas-e-contradicoes.md)); **Não verificado em execução**: apenas a leitura do código foi feita.
