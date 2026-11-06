# Calendário e prazos

**Revisado em:** 2026-11-06 · **Estado:** **Implementado** (2 testes em `calendar.rs`). **Não verificado:** sincronização autenticada real; o app não tem calendário oficial de 2027.1.

## Ficha

| Campo | Valor |
|---|---|
| Sistema de origem | Canvas UVA, tarefas **publicadas** das matérias atuais |
| Informação consultada | `id`, `name`, `due_at`, `html_url`, `published` de cada tarefa |
| Método | `CanvasSession::list_assignment_events` (`browser.rs`): abre `/courses/<id>/assignments` e executa `fetch` de `GET /api/v1/courses/<id>/assignments?per_page=100&override_assignment_dates=true` dentro da página autenticada |
| Autenticação | Manual (janela Chromium do app). Sessão expirada interrompe a consulta |
| Comando | `sync_calendar(semester)` e `list_calendar_events` em `app/src-tauri/src/lib.rs` |
| Persistência | Tabela `academic_events` do SQLite local, índice `academic_events_semester_date` |

## Regras

- O estudante informa o semestre (`AAAA.1` ou `AAAA.2`, validado por `valid_semester`). **O app não infere nem projeta semestres**: o `README` de `app/` registra que, para 2027.1, a tela mostra o período sem datas e o link oficial da UVA, e que nenhum prazo é estimado a partir de semestres anteriores.
- Só matérias com `kind == Current` são consultadas. Sem nenhuma, retorna erro.
- Descarta tarefa não publicada, sem nome ou sem `due_at` válido (`valid_timestamp` confere calendário, anos bissextos, hora e fuso).
- `all_day` é verdadeiro quando `due_at` não tem hora (`T`).
- A URL de origem só é mantida se pertencer a `uva.instructure.com/courses/<id>/assignments/...`, sem userinfo nem porta; query e fragmento são removidos. Caso contrário usa a URL canônica montada a partir do curso.
- `id` do evento = SHA-256 de `canvas:<curso>:<tarefa>`: estável entre execuções.
- Limite: 20 páginas de resultados por matéria (laço no script).

## Atualização e idempotência

`replace_course_calendar_events` roda numa transação: apaga os eventos `canvas_assignment` daquele semestre e curso e reinsere os atuais (`ON CONFLICT(id) DO UPDATE`, com `verified_at = CURRENT_TIMESTAMP`). Rodar de novo não duplica. Falha numa matéria vira entrada em `errors` do `CalendarSyncReport` e as demais continuam.

Não existe cancelamento específico para esta operação (é curta e serializada pelo `Mutex` `canvas_operation`).

## Proveniência gravada

`source_kind = canvas_assignment`, `source_url`, `source_label = "Canvas · <matéria>"`, `verified_at`, `semester`, `course_id`, `course_title`.

## Garantias

Somente leitura: `GET` com `credentials: same-origin`, sem abrir página de entrega e sem modificar atividades. O app **não agenda provas** e **não grava nada no Canvas nem no calendário institucional**. Provas do Sistema de Provas (SGP) não são lidas; o agendamento é manual (ver [`../NAVEGACAO_PORTAL_UVA.md`](../NAVEGACAO_PORTAL_UVA.md), P1 a P3).

## Limitações

- Esta consulta chama `fetch` com caminho literal; não passa por `validate_api_path`, que protege apenas as leituras de `canvas_api.rs`. O caminho é fixo e usa só o ID numérico do curso (validado como dígitos), mas a proteção não é a mesma.
- Prazos individuais por aluno (`override_assignment_dates=true`) dependem do que o Canvas devolve; não verificado com dados reais.
- Calendários acadêmicos oficiais (PDF "Calendário Acadêmico" da disciplina) não são lidos por esta automação.
