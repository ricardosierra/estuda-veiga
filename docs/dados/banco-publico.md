# Banco público

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

## Resumo

**Hoje não existe um banco público dedicado.** Nenhum arquivo `.sqlite3` do projeto é publicável, e nenhum código lê ou grava um banco com esse papel. O que existe, e o que é só projeto, está separado abaixo:

| Parte | Estado | Onde |
|---|---|---|
| Catálogo de disciplinas, currículos, ofertas, relações e períodos em JSON, um registro por arquivo | **Implementado** | `dados/` |
| Banco SQLite gerado a partir dos registros | **Implementado**, mas **mistura dado pessoal** e não é público | `build/acervo.sqlite3` |
| Textos extraídos de materiais oficiais, com página e fonte | **Implementado**; direitos de distribuição **não verificados** | `fontes-uva/` |
| Biblioteca pública de conteúdos, exercícios, provas e soluções | **Planejado** (a pasta `biblioteca/` não existe; o prefixo está reservado no contrato `estuda-domain`) | — |
| Banco público somente leitura distribuído em snapshots | **Planejado** (proposta em [arquitetura alvo, §7](../arquitetura/03-dados-e-sincronizacao.md)) | — |

Documentos relacionados: [visão geral](visao-geral.md) · [banco privado](banco-privado.md) · [proveniência e confiança](proveniencia-e-confianca.md) · [formatos](../referencia/formatos.md).

> Esta documentação não autoriza criar bancos, executar SQL nem gerar arquivos `.sqlite3`. Toda mudança de banco exige aprovação prévia, explícita e individual do usuário no chat.

## 1. O que existe como "público" hoje

### 1.1 Registros em `dados/`

Fonte de verdade estruturada da parte compartilhável (`dados/LEIA-ME.md`). Estado em 2026-11-06, 174 arquivos:

| Pasta | Registros | Camada segundo `estuda-domain` |
|---|---|---|
| `dados/instituicao/` | 1 | `Shareable` |
| `dados/programas/` | 2 (Ciência da Computação e Engenharia de Computação) | `Shareable` |
| `dados/ofertas/<programa>/` | 79 | `Shareable` |
| `dados/relacoes/` | 14, todas candidatas | `Shareable` |
| `dados/periodos/` | 2 | `Shareable` |
| `dados/disciplinas/` | 76 | `NeedsReview` |

Problemas que impedem tratar esse conjunto como publicável sem revisão:

1. **Todos os 174 arquivos citam `privado/` como fonte** (`source.file` ou `sourceFiles`). Quem receber apenas a parte compartilhável não consegue abrir a fonte. Veja a lacuna L1 em [visão geral](visao-geral.md).
2. **`dados/disciplinas/` não passou por revisão registrada.** O contrato exige revisão humana explícita para `NeedsReview`; nenhum arquivo de revisão existe.
3. **Relações são candidatas.** Todas têm `equivalenceConfirmed: false`; não há equivalência curricular confirmada por fonte institucional (`dados/relacoes/*.json`).
4. **Não há licença declarada** nos registros. O campo não existe no formato atual (`validar.mjs` não o exige).

### 1.2 `build/acervo.sqlite3` (gerado, ignorado pelo Git)

Criado por `node tools/acervo/build.mjs` (não rodado nesta revisão; só `--validar`). `gerarSqlite` apaga o arquivo e `-wal`/`-shm` e o recria do zero a cada execução. É **descartável** e **não é banco público**: contém tabelas pessoais.

Tabelas (colunas de `tools/acervo/src/sqlite.mjs`; todas recebem uma coluna extra `dados_json TEXT` com o registro original):

| Tabela | Colunas | Origem | Natureza |
|---|---|---|---|
| `programas` | `id` (PK), `titulo`, `codigo`, `instituicao` | `dados/programas/` | catálogo |
| `periodos` | `id` (PK), `situacao`, `observado_em` | `dados/periodos/` | catálogo |
| `disciplinas` | `id` (PK), `titulo`, `identidade`, `fonte` | `dados/disciplinas/` e `privado/dados/disciplinas/` | misto |
| `ofertas_curriculares` | `id` (PK), `programa_id`, `codigo`, `titulo`, `carga_horaria`, `situacao`, `disciplina_id` | `dados/ofertas/` | catálogo |
| `relacoes` | `id` (PK), `tipo`, `oferta_cc`, `oferta_ec`, `disciplina_id`, `equivalencia_confirmada` | `dados/relacoes/` | catálogo |
| `cursos_canvas` | `id` (PK), `titulo`, `url`, `tipo`, `situacao`, `periodo`, `codigo`, `secao`, `fonte` | `privado/dados/canvas/` | **pessoal** |
| `trajetorias` | `id` (PK), `programa_id`, `situacao`, `periodo`, `contexto` | `privado/dados/trajetorias/` | **pessoal** |
| `matriculas` | `id` (PK), `programa_id`, `periodo`, `titulo`, `turma`, `disciplina_id`, `canvas_id`, `docente`, `docente_exibido`, `inicio`, `situacao`, `carga_horaria` | `privado/dados/matriculas/` | **pessoal** (inclui docente) |
| `notas` | `id` (PK), `matricula_id`, `ordem`, `curso_titulo`, `situacao`, `media`, `faltas`, `frequencia_percent`, `canvas_pontos`, `canvas_possiveis`, `canvas_percent` | `privado/dados/notas/` | **pessoal** |
| `avaliacoes_lyceum` | `matricula_id`, `ordem`, `avaliacao`, `nota`, `escala`, `data` | `lyceumGrades` dos registros de notas | **pessoal** |
| `trabalhos` | `id` (PK), `titulo`, `arquivo`, `disciplina_id`, `canvas_id`, `periodo` | `privado/dados/trabalhos/` | **pessoal** |
| `pendencias` | `entidade`, `id`, `campo`, `motivo`, `arquivo` | campo `pendencias` de todos os registros | misto |
| `arquivos` | `caminho` (PK), `pasta`, `extensao`, `bytes` | listagem de `materias/`, `fontes-uva/`, `periodos/`, `cursos/`, `privado/` | misto |

Views: `v_matriculas`, `v_cursos`, `v_pendencias`, `v_resumo_periodo`, `v_curriculo`. **Não há chaves estrangeiras declaradas**; as ligações (`programa_id`, `disciplina_id`, `canvas_id`, `matricula_id`) são validadas em `validar.mjs` antes de gerar, não pelo SQLite.

Como `carregarDados` lê `dados/` e `privado/dados/`, o arquivo gerado contém matrículas, docentes, notas e frequência. O contrato `estuda-domain` o classifica como `Generated` (nunca publicável) e o `.gitignore` o ignora (`/build/`).

### 1.3 Textos de `fontes-uva/`

Texto extraído (Markdown por página, com fonte) de PDFs oficiais por período e matéria. Versionado no Git: 95 arquivos rastreados em 2026-11-06. O contrato `estuda-domain` os trata como `Shareable`; **a licença de redistribuição não está verificada** (lacuna L6 em [visão geral](visao-geral.md)). Até haver decisão, não os trate como públicos.

## 2. Requisitos pedidos e estado de cada um

| Requisito | Estado | Observação |
|---|---|---|
| Catálogo de disciplinas | Parcial | existe em JSON; sem revisão nem licença registradas |
| Conteúdos acadêmicos | Parcial | `fontes-uva/` e `materias/`; sem metadados de licença |
| Exercícios | Planejado | não há registro estruturado de exercício |
| Provas compartilháveis | Planejado | não há tipo de registro nem política implementada |
| Enunciados | Planejado | idem |
| Soluções autorizadas | Planejado | não há campo de autorização de publicação |
| Versionamento | Parcial | histórico do Git; `schemaVersion: 1` nos JSON legados; sem versão por material |
| Fontes | Implementado | `source.file`, `section`, `observedAt` obrigatórios e validados (`validar.mjs`) |
| Licenças | Planejado | não há campo; `source.kind`/`license` só existem na proposta |
| Distribuição | Planejado | não há processo de release nem pacote |
| Atualização offline | Planejado | não há importação de snapshot; o acervo é uma pasta local |

## 3. Projeto futuro (Planejado)

Nada desta seção existe no código nem nos dados.

A proposta de arquitetura alvo (`docs/arquitetura/03-dados-e-sincronizacao.md`, arquivo ainda não commitado) separa três domínios: privado por aluno, acervo público e plataforma. Para o acervo público ela descreve:

- fonte de verdade em **texto** (`biblioteca/` e `dados/`), com o banco `acervo-publico.sqlite3` **derivado** e aberto somente leitura, distribuído como snapshot;
- uma tabela `library_meta` com identidade, versão do snapshot e hash do manifesto de origem;
- entidades `source` (com tipo, citação, **licença** e revisor), `degree_program`, `subject`, `curriculum_item`, `subject_relation` e `learning_material`, com procedência obrigatória por linha;
- **nenhuma chave estrangeira entre o banco privado e o público**: o privado guarda apenas identificadores (`subject_id`, `library_snapshot`) e trata órfãos como aviso, porque snapshots antigos e uso offline são normais;
- promoção explícita de material do domínio privado para o público, com revisão humana, nunca automática.

A fase de integridade e assinatura descrita em `ARQUITETURA_ACERVO.md` §6 (objetos por SHA-256, manifesto JSON por período e matéria, assinatura Ed25519, depois sincronização entre pares) é igualmente **Planejada** e depende de decisões abertas: quais conteúdos podem ser compartilhados, a pilha de rede e a estrutura de repositórios.

O SQL dessa proposta consta no documento de arquitetura alvo como **não executado e aguardando aprovação**. Este documento não o reproduz para não sugerir que esteja pronto para uso.

### Pré-requisitos antes de qualquer banco público

1. Resolver a lacuna L1: registros compartilhados que dependem de fonte privada.
2. Decidir direitos de `fontes-uva/` e de qualquer material de terceiros.
3. Definir o campo de licença e o de status de verificação no formato dos registros (veja a proposta de contrato de catalogação em [formatos](../referencia/formatos.md#6-metadados-de-material-proposta)).
4. Integrar `estuda-domain::publication` a um passo de publicação que recuse o que não for `Shareable` ou revisado.
5. Aprovação explícita do usuário para criar qualquer esquema novo.
