# Proveniência e confiança dos dados

**Revisado em:** 2026-11-06 · **Base:** commit `3811300` mais árvore de trabalho não commitada.

`AGENTS.md` pede: "Diferencie dados confirmados por fonte oficial de estimativas, material didático e campos desconhecidos. Registre fonte e data para notas, frequência e progresso." Este documento descreve o que o projeto **de fato** faz para isso hoje, o que existe só como contrato, e onde há lacunas.

Relacionados: [visão geral](visao-geral.md) · [sincronização](sincronizacao.md) · [formatos e convenções](../referencia/formatos.md).

## 1. Dois mecanismos distintos

| Mecanismo | Estado | Onde |
|---|---|---|
| **Convenção nos registros JSON**: `source`, `pendencias`, `null` com pendência, campos de status ad hoc | **Implementado e validado** | `tools/acervo/src/validar.mjs`, `dados/LEIA-ME.md` |
| **Tipo `Confidence`** com quatro estados e regra de combinação | **Planejado**: existe com testes, mas não é usado pelos registros nem por `estuda-core` | `crates/estuda-domain/src/confidence.rs` |

Não há, hoje, uma coluna ou campo `confidence` em nenhuma tabela SQLite nem nos registros JSON. A palavra "confiança" aparece apenas em `curriculumVersionConfidence` (um campo de texto livre em `dados/programas/`) e no tipo Rust.

## 2. O contrato `Confidence` (Planejado)

Definido em `crates/estuda-domain/src/confidence.rs`:

| Estado | Significado no código |
|---|---|
| `Confirmed` | visto em fonte oficial (Portal/Lyceum) com fonte e data registradas |
| `Estimated` | calculado ou inferido localmente (exemplo dado no código: nota final ponderada pelo app) |
| `Unknown` | ausente nas fontes; representado como `None` com pendência, nunca `0` ou `""` |
| `Conflicting` | fontes divergentes; preservado para revisão humana |

Regras (`is_official`, `reconcile`):

- Só `Confirmed` é oficial (`is_official`). Estimativa nunca vira confirmação sem fonte oficial.
- `reconcile(self, other, values_equal)` combina duas observações do mesmo campo:
  - `Unknown` cede a qualquer outra observação;
  - estados iguais com valores iguais mantêm o estado;
  - `Estimated` e `Confirmed` com **valores iguais** viram `Confirmed`;
  - qualquer outra combinação vira `Conflicting`. Valores diferentes **nunca** escolhem um vencedor.
- Quatro testes cobrem essas regras (`only_confirmed_is_official`, `unknown_yields_to_any_observation`, `different_values_conflict_and_never_pick_a_winner`, `equal_estimate_confirmed_by_official_source`).

Integração: nenhuma dependência de `estuda-core` nem de `src-tauri` em `estuda-domain`. Apenas o crate `app/crates/estuda-sync` (novo, ainda não commitado, não inspecionado em detalhe nesta revisão) declara a dependência e importa `ids`. A proposta de arquitetura alvo (`docs/arquitetura/03-dados-e-sincronizacao.md` §3) prevê que todo valor acadêmico pessoal carregue `Confidence`, fonte e data. **Isso é proposta, não implementação.**

## 3. O que os registros JSON implementam (Implementado)

### 3.1 Fonte obrigatória

Quase todo tipo de registro exige `source` (todos, exceto `relacoes`, que exige `sourceFiles`, e `meta`). O formato é:

```json
{
  "source": {
    "file": "fontes-uva/2026.2/exemplo/capitulo-01.md",
    "section": "Cabeçalho da seção consultada",
    "observedAt": "2026-11-06"
  }
}
```

`validar.mjs` verifica que `source.file`, cada item de `sourceFiles` e o campo `arquivo` apontam para um **arquivo existente** no repositório. Ele **não** verifica a seção, a data nem se o conteúdo do arquivo sustenta o valor.

### 3.2 Ausência explícita

Regra central de `validar.mjs`: valor `null` só é aceito se houver, na lista `pendencias` do mesmo registro, um item com `campo` igual ao nome do campo. String vazia é erro. Cada pendência precisa de `campo` (texto) e `motivo` (texto não vazio). Exemplo sintético:

```json
{
  "id": "2026.9",
  "situacao": "previsto",
  "inicio": null,
  "source": {
    "file": "periodos/futuros.md",
    "section": "Semestres futuros",
    "observedAt": "2026-11-06"
  },
  "pendencias": [
    { "campo": "inicio", "motivo": "calendário oficial ainda não publicado" }
  ]
}
```

(O campo `inicio` é ilustrativo; `validar.mjs` aceita campos extras além dos obrigatórios. Este exemplo foi submetido à função `validar` e não produziu erros.)

O gerador reúne todas as pendências na tabela `pendencias` de `build/acervo.sqlite3` e na visualização `build/visualizacoes/pendencias.md`, para revisão.

### 3.3 Vocabulários de status em uso

Valores distintos encontrados nos registros em 2026-11-06 (conferidos por varredura dos JSON). **Não são unificados**; cada tipo usa o seu vocabulário e só alguns são validados por `validar.mjs` (marcados com "validado").

| Tipo | Campo | Valores observados | Validado? |
|---|---|---|---|
| períodos | `situacao` | `encerrado`, `em_andamento` (`previsto` também é permitido) | sim |
| cursos do Canvas | `tipo` | `curricular`, `apoio`, `gratuito` | sim |
| cursos do Canvas | `situacao` | `em_andamento`, `encerrada`, `apoio`, `gratuito` | sim |
| matrículas | `status` | `enrolled`, `closed` | sim |
| notas | `status` | `in_progress`, `closed` | sim |
| disciplinas | `identityStatus` | `canonical_single_offering`, `same_title_and_workload_candidate`, `matricula_registrada` | não |
| ofertas | `status` | `pending` | não |
| relações | `type` | `same_subject_candidate`, `partial_overlap_candidate` | não |
| relações | `equivalenceConfirmed` | `false` em todas as 14 | campo obrigatório, valor livre |
| programas (currículo) | `trajectoryStatus` | `active`, `evadido` | não |
| programas (currículo) | `curriculumVersionConfidence` | um texto livre (`portal_text_accessibility_could_not_distinguish_I_from_1`) | não |
| trajetórias | `status` | `active`, `evadido` | não |

Observações:

- Os valores "candidata" de `relacoes` e `identityStatus` expressam **incerteza de identidade**, não confirmação. O texto de `AGENTS.md` e de `dados/relacoes/*.json` (campo `note`) diz que equivalência e aproveitamento não foram confirmados por fonte institucional.
- `curriculumVersionConfidence` mostra o risco de campo de confiança livre: o valor descreve uma limitação do Portal em prosa de máquina, não um estado comparável.
- Há valores em português (`em_andamento`, `encerrada`, `evadido`) e em inglês (`enrolled`, `closed`, `active`) misturados. Veja [formatos](../referencia/formatos.md).

### 3.4 Mapeamento sugerido (Proposta)

Para uma futura integração com `Confidence`, uma tradução possível, **sem efeito hoje**:

| Situação nos dados | `Confidence` sugerido |
|---|---|
| valor lido do Lyceum/Portal em captura com data (`notas`, `matriculas`) | `Confirmed` |
| nota final ponderada calculada pelo app (`grading.rs`, comando `calculate_final_grade`) | `Estimated` |
| `null` com pendência | `Unknown` |
| fontes que divergem (`sync_conflicts`, conflito entre capturas) | `Conflicting` |
| relação `*_candidate` com `equivalenceConfirmed: false` | `Unknown` para a equivalência (não `Estimated`) |

## 4. Proveniência nas capturas e no banco do app

| Registro | Campos de proveniência | Fonte no código |
|---|---|---|
| Captura do Portal/Lyceum | `capturedAt` (ISO UTC, `2026-11-06T12:00:00Z`), por página `sourceUrl` (sem query nem fragmento), `capturedAt`, `pagesRead`, `inaccessibleSections`, `warnings`, `complete` | `lyceum.rs` (`PortalCapture`, `CapturedPage`) |
| Captura acadêmica do Canvas | `capturedAt`, `options`, por curso `courseId`, `sourceUrl`, `warnings`; por nota `assignmentId`, `sourceUrl`, `commentsChecked`; `complete` | `canvas_api.rs` |
| Material importado | `origin` (URL sem query), `sha256` (do Markdown gravado; usado para detectar edição local), `text_sha256` (do texto extraído), `source_key`, `source_kind`, `source_etag`, `source_last_modified`, `source_validator_host`, `pages`, `imported_at`, `semester` (nulo se não confirmado) | `storage.rs`, `materials` |
| Página de aula e Markdown | cada página do PDF vira `## Página N`, com a URL de origem sem segredos | `pdf.rs` (`pages_to_markdown`) |
| Evento de calendário | `source_kind`, `source_url`, `source_label`, `verified_at` | `storage.rs`, `academic_events` |
| Interação com o tutor | `source_page`, `source_heading`, `source_excerpt`, `source_sha256`, `attempted_at`; o app detecta se a fonte mudou depois (`source_changed`) | `storage.rs`, `lesson_interactions` |
| Resumo por IA | `source_hash`, `source_manifest_json`, `model`, `input_tokens`, `output_tokens`, `created_at` | `storage.rs`, `study_summaries` |
| Documento sincronizado | `content_hash`, `revision`, `updated_at` | `sync.rs` |

Regras explícitas já aplicadas no código:

- **Semestre desconhecido não é inferido.** Um material só recebe `semester` se há correspondência confirmada com matrícula ativa; `set_material_semester` e `save_material` mantêm o valor anterior quando o novo é nulo (teste `material_semester_is_persisted_and_unknown_does_not_erase_it`).
- **Estimativa é rotulada.** A nota final ponderada é calculada localmente e apresentada como estimativa; a nota oficial continua sendo a do boletim do Lyceum (`app/README.md`, seção sobre notas do Canvas).
- **Captura parcial é declarada.** `complete = false` e `warnings` registram falhas; o app não afirma cobertura total.
- **Atividade de estudo não é frequência.** Sessões de aula e prática não alteram presença, notas, matrícula ou entrega na UVA (`ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`).

## 5. Lacunas

1. **Sem campo de confiança por valor.** Notas, frequência e progresso em `privado/dados/notas/` têm `source.observedAt` por registro, não por valor. O campo `Confidence` não existe nos dados.
2. **`validar.mjs` não confere o conteúdo da fonte.** A existência do arquivo citado basta.
3. **Vocabulários heterogêneos e sem validação** (seção 3.3).
4. **Formatos de data heterogêneos** (`observedAt` ISO; `startDate` em `DD/MM/AAAA`; instante Unix em segundos no sincronizador). Veja [formatos](../referencia/formatos.md).
5. **Fontes `dados/` → `privado/`.** A proveniência de todos os registros compartilhados aponta para arquivos privados (lacuna L1 em [visão geral](visao-geral.md)).
6. **`Confidence` e `estuda-domain` não integrados** ao fluxo de dados (seção 2).
7. **Frequência e notas oficiais não foram obtidas por captura automática completa** segundo `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`: a leitura de frequência de 10/10/2026 foi manual e pontual. **Não verificado** aqui; nenhuma leitura autenticada foi feita nesta revisão.
