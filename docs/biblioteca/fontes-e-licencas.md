# Fontes, licenças e direitos autorais

**Revisado em:** 2026-11-06. Este documento não é parecer jurídico. Registra o que o repositório declara, o que não declara e as regras de trabalho adotadas.

## 1. Licenciamento efetivamente existente

**Nenhuma licença declarada.** Conferido em 2026-11-06:

- Não há `LICENSE`, `COPYING` nem `NOTICE` na raiz (`ls -a`).
- Os manifestos `app/Cargo.toml`, `crates/estuda-domain/Cargo.toml`, `app/src-tauri/Cargo.toml` e `app/package.json` não têm campo `license`; `estuda-domain` e `package.json` são marcados `publish = false`/`"private": true`.
- Nenhum README da raiz ou de `docs/` declara licença.

Consequência prática: sem licença, o padrão legal é "todos os direitos reservados" para a parte autoral; terceiros não têm permissão de reutilizar, e dependências de terceiros mantêm as próprias licenças. Escolher licenças para código, documentação e conteúdo é uma **decisão pendente do mantenedor** (ver §6).

## 2. Origem dos materiais do repositório

| Origem | Onde está | Autoria | Direitos |
|---|---|---|---|
| Código do Estuda UVA | `app/`, `crates/`, `tools/` | do mantenedor | sem licença declarada (§1) |
| Documentação | `docs/`, READMEs | do mantenedor | sem licença declarada |
| Notas de estudo | `periodos/*/notas-de-estudo.md` | do aluno, derivadas de material oficial | sem licença declarada; derivadas de obra de terceiro |
| Textos extraídos de materiais oficiais | `fontes-uva/` | **terceiros** (UVA e autores de e-books; cabeçalho cita fonte oficial) | **não verificado**; não há permissão de redistribuição registrada |
| Catálogo (disciplinas, ofertas, relações) | `dados/` | fatos observados no Portal, registrados pelo aluno | fatos acadêmicos; **não verificado** se o conjunto carrega direitos de base de dados |
| Trabalhos do aluno | `privado/materias/`, `originais/` | do aluno | Enunciado é de terceiro. Pela decisão de 6/11/2026, o texto pode ir ao Git com dados pessoais em `privado/`; `originais/` fica fora do Git |
| Dependências | `app/Cargo.lock`, `app/package-lock.json` | terceiros | licenças próprias; não auditadas neste documento |

## 3. Textos extraídos de materiais da UVA

Cada arquivo de `fontes-uva/` registra no cabeçalho: link oficial, link do item no Canvas, data de download, ferramenta de extração (`pdftotext -layout -enc UTF-8`) e o aviso de que imagens e fórmulas não são reproduzidas. O PDF original **não** é versionado (`.gitignore`; `node tools/acervo/build.mjs --verificar-texto` não achou binários rastreados).

Há **contradição interna** sobre redistribuição:

- `crates/estuda-domain/src/publication.rs` classifica `fontes-uva/` como `Shareable` (teste `default_is_deny` espera que um capítulo de `fontes-uva/` não gere violação).
- `docs/ARQUITETURA_ACERVO.md` §6 afirma que "textos extraídos de materiais da UVA e de e-books de terceiros não devem ser replicados entre usuários sem permissão" e deixa a decisão como pendente.

Regra de trabalho recomendada até a decisão: **tratar `fontes-uva/` como "uso pessoal e estudo local", não como conteúdo público**. Isso vale especialmente antes de publicar o repositório ou de qualquer sincronização entre pares. Para tornar um trecho público, obter permissão da instituição ou do autor, ou substituí-lo por link oficial e resumo próprio. A decisão de 6/11/2026, que trata de dados pessoais, não altera esta regra: os direitos de `fontes-uva/` seguem em aberto.

## 4. Como registrar a fonte

Todo registro e todo texto extraído deve permitir que outra pessoa chegue à origem:

1. **Registros de `dados/`:** `source.file` (arquivo do repositório que existe), `source.section` e `source.observedAt` (`YYYY-MM-DD`). O validador recusa fonte inexistente.
2. **Textos extraídos:** link oficial, página por página (`## Página N`), data do download e ferramenta usada.
3. **URLs:** sempre sem query, fragmento, usuário ou senha. O app já remove esses elementos (`lyceum::sanitize_source_url`, `canvas::sanitize_source_url`); manualmente, remova antes de colar. Nunca grave URL assinada ou de SSO.
4. **Dados oficiais vs. estimativas:** diferenciar o que veio de fonte oficial, o que foi estimado e o que é desconhecido (`AGENTS.md`; `crates/estuda-domain/src/confidence.rs`: `Confirmed`, `Estimated`, `Unknown`, `Conflicting`).
5. **Conteúdo gerado por IA:** marcar "gerado por IA, conferir na fonte" e nunca promover a dado oficial (`docs/features/README.md`).

## 5. Regras de uso de terceiros

Seguindo a regra global de reutilização do mantenedor: material de terceiro **pode** ser estudado, usado e adaptado, desde que se verifique a licença, se preserve atribuição e se respeitem as condições. Isso não autoriza redistribuir texto de e-book ou de material institucional sem permissão.

- Código aberto e bibliotecas: manter avisos e licenças originais ao copiar trechos.
- Citação curta com fonte é aceitável; reprodução integral de capítulo, prova ou enunciado de terceiro não é, sem licença.
- Imagens e PDFs não entram no Git; diagramas viram Mermaid ou descrição textual própria.
- Contribuições de outras pessoas exigem consentimento explícito de publicação (ver [../contribuindo/conteudo.md](../contribuindo/conteudo.md)).

## 6. Decisões pendentes

1. Licença do código (ex.: permissiva ou copyleft) e da documentação.
2. Licença do conteúdo próprio da biblioteca e se haverá uma licença por tipo (conteúdo, exercício, solução).
3. Se `fontes-uva/` será redistribuível, restrito a uso local ou substituído por links e resumos próprios.
4. Publicação dos registros de `dados/`: a decisão de 6/11/2026 permite publicá-los com os dados pessoais em `privado/`. Seguem pendentes os nomes de docentes e de terceiros nos registros (ver [lacunas-e-contradicoes.md](../referencia/lacunas-e-contradicoes.md) §1).
5. Auditoria de licenças das dependências antes de qualquer publicação binária.
