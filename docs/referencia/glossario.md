# Glossário

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

Termos usados na documentação, com o sentido que têm neste projeto. Quando o termo corresponde a um nome no código, o caminho está indicado.

## Projeto e arquitetura

| Termo | Sentido |
|---|---|
| **Acervo** | A pasta raiz do repositório vista pelo aplicativo: arquivos acadêmicos em texto mais `dados/` e `privado/`. No código, "library" (`library_path`). |
| **Estuda UVA** | O aplicativo desktop local. A interface em desenvolvimento é GPUI (`estuda-gpui`); o frontend Tauri/React foi removido em `aeab5f8`. |
| **estuda-core** | Crate Rust com o núcleo (domínio acadêmico, acervo, SQLite, sincronização, Canvas e Portal). Não conhece Tauri nem GPUI. `app/crates/estuda-core`. |
| **estuda-domain** | Crate Rust sem I/O com `Confidence`, identificadores tipados e `Layer` de publicação. Existe e tem testes, mas `estuda-core` não depende dele. `crates/estuda-domain`. |
| **estuda-gpui** | Crate da interface GPUI (binário `estuda-uva`, 8 telas, `gpui-kit`). Em desenvolvimento ativo; usa só `CatalogService` de `estuda-core`. **Parcial**. |
| **Shell Tauri / legado** | `app/src-tauri`: registra 52 comandos sobre `estuda-core`, mas não tem mais interface (o frontend `app/src` foi removido em `aeab5f8`). Mantido para paridade. |
| **Offline-first** | Tudo o que o app mostra vem de arquivos e SQLite locais; a rede só é usada quando o estudante pede. |
| **Text-first** | O Git versiona somente texto. Binários ficam fora; seu texto extraído entra com fonte. |

## Dados

| Termo | Sentido |
|---|---|
| **Camada privada** | Dado de uma pessoa: matrícula, notas, frequência, trajetória, trabalhos entregues, planejamento. Fica em `privado/`, no banco do app e em `materias/catalogo/capturas/`. |
| **Camada compartilhável** | Matéria e instituição sem dado de pessoa: `dados/`, `fontes-uva/` (sujeito a direitos), código e documentação. |
| **Banco do aplicativo** | `.estuda-uva.sqlite3` na raiz do acervo (WAL). Guarda estudo, capturas, índice de sincronização e a projeção do catálogo. |
| **Banco gerado** | `build/acervo.sqlite3`, produzido por `tools/acervo`; descartável e reconstruível. Não é o banco do app. |
| **Banco público** | Banco dedicado a conteúdo compartilhável. **Não existe**; é proposta. |
| **Registro** | Um arquivo JSON de `dados/` ou `privado/dados/`, com `id`, `source` e `pendencias`. |
| **Pendência** | Par `campo`/`motivo` que justifica um campo `null`. Ausência nunca é `""` nem `0`. |
| **Fonte (`source`)** | Objeto com `file`, `section` e `observedAt`: onde e quando o dado foi visto. O arquivo precisa existir. |
| **Espelho (mirror)** | Cópia de um documento em outro formato gerada pelo sincronizador em `.estuda-uva/mirrors/` (Markdown vira `.md.json`; JSON vira `.json.md`). Não é fonte. |
| **Envelope JSON** | Estrutura JSON que embrulha o conteúdo de um Markdown no espelho, com id, caminho, formato, revisão e conteúdo. |
| **Tombstone** | Registro de que um arquivo-fonte foi apagado, mantendo o conteúdo para recuperação. |
| **Outbox** | Tabela `sync_outbox` com a projeção pendente de gravação, reaplicada se o app for interrompido. |
| **Conflito** | Duas ou mais superfícies mudaram de forma diferente desde a base. Nenhuma versão vence sozinha; o caso fica para revisão humana. |
| **Projeção relacional** | Tabelas `academic_*` reconstruídas a partir dos JSON do catálogo. Não são via de edição. |
| **Confiança** | `Confirmed` (fonte oficial com data), `Estimated` (cálculo local), `Unknown`, `Conflicting` em `estuda-domain`. |
| **Formato antigo / legado** | `materias/catalogo/*.json` gerado por `tools/acervo` para o app ler. Gerado; não editar. |

## Acadêmico

| Termo | Sentido |
|---|---|
| **UVA** | Universidade Veiga de Almeida, a instituição das fontes. |
| **Portal do Aluno** | Portal institucional que leva a vários subsistemas, entre eles o Lyceum. |
| **Lyceum** | Subsistema acadêmico (histórico, boletim, frequência). Fonte oficial de nota e frequência. |
| **Canvas** | Ambiente virtual de aprendizagem (`uva.instructure.com`). Notas do Canvas são das atividades online e não são a nota oficial. |
| **AVA** | Atividade do ambiente virtual no Canvas (A1 no cálculo de nota). |
| **NFp** | Nota final ponderada calculada localmente: A1 × 0,4 + maior entre A2 e A3 × 0,6. É estimativa. Ver `grading.rs`. |
| **Oferta** | Item de currículo de um programa (`dados/ofertas/`). |
| **Trajetória** | Vínculo de uma pessoa com um programa: situação, progresso, matrículas e resultados. É pessoal. |
| **Espaço Curso / curso de apoio** | Curso do Canvas que não é disciplina; excluído da captura em lote. |

## Automação

| Termo | Sentido |
|---|---|
| **Captura** | Leitura de conteúdo no Canvas ou no Portal, iniciada pelo estudante, em modo de leitura. |
| **Captura parcial** | Captura que terminou com seção inacessível, limite atingido ou falha; é sempre marcada como tal. |
| **CDP** | Chrome DevTools Protocol, usado via `chromiumoxide` para controlar o Chromium. |
| **Chave de fonte (`source_key`)** | Identificador estável do material de origem usado para reaproveitar uma captura sem baixar de novo. |
| **Aula guiada / tutor** | Aula e perguntas geradas de forma determinística a partir do Markdown, sem IA. |

## Rótulos de estado

**Implementado**, **Parcial**, **Planejado** e **Não verificado**: definidos em [README.md](../../README.md#como-ler-os-rótulos-de-estado).
