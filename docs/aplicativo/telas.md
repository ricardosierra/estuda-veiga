# Telas do Estuda UVA (aplicativo Tauri/React)

**Revisado em:** 2026-11-06
**Base:** commit `3811300` mais a árvore de trabalho não commitada daquele dia.
**Escopo:** a interface que existe hoje em `app/src/ui/`, executada pelo shell Tauri em `app/src-tauri/`. O crate `app/crates/estuda-gpui` é um esqueleto (`fn main() {}`) e **não tem tela alguma**; a interface GPUI é **planejada**, não implementada (ver [migração Tauri → GPUI](../arquitetura/migracao-gpui.md)).

Este documento descreve o código lido, não uma execução do app. O app **não foi executado** nesta revisão. Nenhum dado pessoal real aparece nos exemplos.

Convenções de estado usadas aqui: **Implementado** (o código existe e é alcançável pela interface), **Parcial** (existe com limite relevante) e **Não verificado** (depende de execução real ou de sessão autenticada).

## 1. Mapa de telas

A navegação é um único `useState<View>` em `app/src/ui/App.tsx` (sem roteador). Há oito valores de `View`, mas só seis aparecem na barra lateral.

| `View` (código) | Rótulo na barra lateral | Título no breadcrumb | Alcançável por |
|---|---|---|---|
| `inicio` | Estudar | Estudar | barra lateral (tela inicial) |
| `materias` | Matérias | Matérias | barra lateral; cartões de matéria |
| `historico` | Histórico | Histórico | barra lateral; "Ver histórico" na tela Estudar |
| `agenda` | Agenda | Agenda | barra lateral |
| `curriculos` | Acadêmico | Acadêmico | barra lateral |
| `acervo` | Dados e sync | Dados e sincronização | barra lateral; abrir documento local |
| `estudar` | (nenhum) | Aula guiada | abrir aula, material ou item do histórico |
| `canvas` | (nenhum) | Canvas e importação | botões "Abrir cursos e importação" e "Atualizar Canvas ↻" em Dados e sync (ambos levam a esta tela; `loadCourses` chama `setView("canvas")`) |

Fonte: `App.tsx` (`type View`, lista de botões em `<nav aria-label="Navegação principal">` e função `viewTitle`).

### Correspondência com os nomes usados no pedido de documentação

O pedido original cita seis telas: Acadêmico, Matérias, Histórico, Estudo pessoal, Dados e sincronização e Acervo. O código não tem exatamente essa lista:

| Nome no pedido | O que existe no código |
|---|---|
| Acadêmico | Existe: `curriculos`, rótulo "Acadêmico", título da página "Currículos e disciplinas". |
| Matérias | Existe: `materias`. |
| Histórico | Existe: `historico`. |
| Estudo pessoal | **Não existe como tela.** "Estudo pessoal" é só o texto fixo do chip de perfil na barra lateral (`<b>Estudo pessoal</b><small>sessão local</small>`, avatar "S" e "•••"), sem ação associada (é um `<div>` sem `onClick`). O estudo em si acontece em **Estudar** (`inicio`, painel de entrada) e **Aula guiada** (`estudar`). |
| Dados e sincronização | Existe: `acervo`. A barra lateral diz "Dados e sync"; o breadcrumb diz "Dados e sincronização". |
| Acervo | **Não é tela separada.** `acervo` é o identificador interno da mesma tela de Dados e sincronização; o título da página é "Acervo da pasta". Acervo e Dados e sincronização são a mesma tela. |
| (não citada) Agenda | Existe e tem item próprio na barra lateral. |
| (não citada) Canvas e importação | Existe, sem item na barra lateral. |
| (não citada) Aula guiada | Existe, sem item na barra lateral. |

## 2. Comportamento comum a todas as telas

**Atualização periódica.** `App.tsx` chama `refresh()` na montagem e a cada 3 segundos (`window.setInterval(..., 3000)`). Cada ciclo invoca `canvas_status`, `overview`, `list_materials`, `list_calendar_events` (semestre selecionado) e `lyceum_status`. Falhas individuais são ignoradas (`Promise.allSettled`).

**Sincronização na abertura.** Na montagem, `refreshLocalLibrary()` invoca `sync_local_catalog` (reconcilia arquivos Markdown/JSON com o SQLite; **escreve** em banco e em `.estuda-uva/mirrors/`) e em seguida `refresh_local_catalog`, `local_sync_conflicts`, `academic_catalog` (que chama `reconcile_all` de novo) e `repository_status`. Ou seja: abrir o app já dispara uma reconciliação. Ver [sincronização](../dados/sincronizacao.md).

**Eventos do backend.** `App.tsx` escuta dois eventos Tauri:

- `local-sync-updated` (emitido por `sync_watcher` em `app/src-tauri/src/lib.rs` quando o monitor de arquivos reconcilia): atualiza o relatório de sincronização e recarrega o acervo.
- `canvas-capture-updated` (emitido durante a captura em lote do Canvas): atualiza o painel de captura; ao chegar a um estado terminal (`waiting_for_login`, `cancelled`, `partial`, `completed`, `failed`) recarrega materiais e acervo.

**Mensagens.** Avisos aparecem em `role="status"` ("✓ …") e erros em `role="alert"` ("! …"), ambos com botão de fechar (`aria-label="Fechar aviso"`/`"Fechar erro"`). A importação em andamento mostra um `role="status"` com "Cancelar importação".

**Indicador de Canvas.** A barra superior mostra "Canvas conectado" ou o rótulo do estado (`Canvas · desconectado`, `aguardando login`, `sessão expirada`, `erro no navegador`) e um botão "↻" (`aria-label="Atualizar dados locais"`).

**Texto fixo na barra lateral.** "Seus dados ficam neste computador. Sem IA, nuvem ou cobrança por uso." Esse texto **contradiz** a aba Resumos de uma matéria, que envia material à API da Anthropic após confirmação (ver §5.3). Registrado em [lacunas e contradições](../referencia/lacunas-e-contradicoes.md).

**Idioma.** `app/index.html` declara `lang="pt-BR"`. Não há seletor de idioma nem internacionalização; todo o texto é português fixo no código.

## 3. Modo offline: o que funciona sem Canvas ou Portal

Sem rede e sem login, funcionam (todos leem arquivos e SQLite locais):

- **Estudar**, **Matérias** (documentos locais e materiais já importados), **Histórico**, **Aula guiada** e o tutor local.
- **Dados e sincronização**: indexação, busca, leitura, edição sincronizada, conflitos e estado do Git.
- **Acadêmico**: catálogo curricular local e calculadora de nota final (cálculo local, "sem rede").
- **Agenda**: lista os eventos já salvos no SQLite.

Dependem de sessão autenticada no Chromium do app (login manual) e **não foram verificados em execução real**: abrir Portal/Lyceum e capturar histórico e notas; ler notas, fóruns, avisos e Caixa de Entrada do Canvas; listar cursos/módulos; importar PDF; capturar todas as disciplinas; atualizar prazos da Agenda.

Depende de rede e de uma chave de API local, também **não verificado em execução real**: gerar resumos (aba Resumos).

## 4. Telas da barra lateral

### 4.1 Estudar (`inicio`) — `StudyHome.tsx`

- **Propósito:** ponto de entrada. Lista as matérias do período ativo e oferece "Continuar aula".
- **Dados apresentados:** matrículas ativas (`status === "enrolled"`) da trajetória com `status === "active"`; contagem de materiais por matéria (documentos locais + materiais importados); a aula não concluída mais recente do histórico.
- **Fonte dos dados:** `academic_catalog` (→ `SyncEngine::read_academic_catalog`, projeção `academic_*` no SQLite, reconciliada a partir de `materias/catalogo/*.json`); `local_catalog_snapshot`/`refresh_local_catalog` (→ `catalog::scanner`); `list_materials` (tabela `materials`); `list_study_history` (tabelas `lesson_sessions` e `study_sessions`). Ver [comandos](../referencia/comandos.md).
- **Funcionalidades:** abrir uma matéria; "Continuar aula" (reabre a sessão com a seção salva); "Ver histórico".
- **Estados vazios e de carga (textos reais):** "Carregando suas matérias — Consultando o catálogo acadêmico local."; "Matrículas atuais não encontradas — Os registros locais continuam disponíveis em Dados e sincronização…"; se o catálogo falhar: "Não foi possível carregar as matrículas" com a mensagem de erro. Por cartão: "Lendo a pasta local…", "N material(is) disponível(is)", "Nenhum material associado no índice parcial", "Conteúdo ainda não associado". Nota inferior: "A leitura da pasta continua em andamento…" / "O índice da pasta está parcial…".
- **Limitações:** o catálogo vem de arquivos **gerados** (`materias/catalogo/*.json`, ignorados pelo Git; gerados por `node tools/acervo/build.mjs`). Sem esses arquivos a tela não lista matérias. A associação de documentos a uma matéria exige título normalizado igual **e** semestre igual (`documentsForEnrollment` em `studyTypes.ts`); o reconhecimento da matéria pelo caminho do arquivo usa uma lista fixa de seis disciplinas em `app/crates/estuda-core/src/catalog/classification.rs` (`known_course`), então documentos de outras matérias podem não ser associados. Esse limite não foi testado contra o acervo real nesta revisão.

### 4.2 Matérias (`materias`) — `SubjectList.tsx`, `SubjectDetail.tsx`

**Lista (`SubjectList`)**

- **Propósito:** matrículas do semestre ativo, com busca e o painel de captura do Canvas.
- **Dados:** os mesmos de Estudar, filtrados por texto (título, turma, semestre; comparação sem distinção de maiúsculas, `pt-BR`).
- **Funcionalidades:** busca ("Buscar matéria ou turma…", campo com rótulo `sr-only` "Buscar matéria"); abrir matéria; painel **Capturar aulas do Canvas** (ver §6.3).
- **Estados vazios:** "Carregando matrículas — O conteúdo salvo na pasta não depende do Canvas."; sem resultado: "Nenhuma matéria corresponde à busca"; sem dados: "Não foi possível carregar…" (se houver erro de catálogo). Cartões: "N material(is) associado(s)" ou "Sem material associado na pasta indexada" / "…no índice parcial".

**Detalhe (`SubjectDetail`)** — abas "Aulas", "Materiais", "Resumos", "Histórico" e "Desempenho" (`<nav aria-label="Seções de <matéria>">`).

| Aba | Conteúdo | Fonte |
|---|---|---|
| Aulas | documentos locais legíveis, materiais importados do Canvas (PDF → Markdown ou página do Canvas → Markdown) e, se o Canvas estiver conectado e a matéria localizada, módulos e itens com "Localizar material →" e "Baixar texto e preparar aula" | scanner local; `list_materials`; `list_courses`, `load_course`, `open_module_item`, `import_material` |
| Materiais | lista de arquivos associados, com formato | scanner local; `list_materials` |
| Resumos | `StudySummaryPanel` (§5.3) | `get_subject_summary_snapshot`, `generate_subject_summaries` |
| Histórico | `StudyHistory` filtrado pela matéria | `list_study_history` |
| Desempenho | média do Lyceum, faltas, frequência, notas por avaliação e total das AVAs do Canvas | projeção do catálogo (`outcomes` da trajetória); dado do aluno, mantido separado do catálogo em `privado/` (ver política de publicação em `AGENTS.md`) |

- **Estados vazios (textos reais):** "Nenhuma aula associada ainda — A matéria está salva localmente. Entre no Canvas para buscar aulas novas; o conteúdo local já existente continuará disponível."; "Nenhum material local associado"; "Nenhum resultado acadêmico associado — As notas oficiais aparecem aqui quando estiverem registradas para esta matéria."
- **Desempenho:** valores ausentes aparecem como "—". Quando há total do Canvas, o texto avisa: "Esse valor não é a média oficial." A tela não distingue visualmente confirmado de estimado além desse aviso; a fonte e a data estão na tela Acadêmico.
- **Offline:** tudo, exceto "Atualizar aulas do Canvas ↻", "Buscar aulas no Canvas ↻", "Localizar material →" e "Baixar texto e preparar aula" (exigem sessão do Canvas).
- **Limitações:** se a matéria não aparece nas inscrições do Canvas, o app avisa e mantém os materiais locais. O botão de importar fica desabilitado quando a origem não é permitida ("Origem não permitida"; a validação está em `download::validate_material_url`).

### 4.3 Histórico (`historico`) — `StudyHistory.tsx`

- **Propósito:** aulas apresentadas e práticas concluídas, "separadas dos resultados oficiais da UVA".
- **Dados:** até 200 registros (`list_study_history` com `limit: 200`): tipo (AULA ou PRÁTICA), data, título do material, matéria, progresso ("Seção X de Y" ou "Concluída") ou acertos ("N de M acerto(s) · P%").
- **Fonte:** SQLite local, tabelas `lesson_sessions` (aulas) e `study_sessions` (práticas).
- **Funcionalidades:** "Atualizar histórico ↻"; "Retomar" ou "Revisar" uma aula (botão com `aria-label` do tipo "Retomar <título>"). Práticas não têm ação de reabrir.
- **Estado vazio:** "Ainda não há aulas no histórico — Quando você apresentar uma aula ou concluir uma prática, ela aparecerá aqui." Carga: "Carregando histórico…".
- **Erros:** falha de carga vai para o banner de erro ("Não foi possível carregar o histórico de estudos." ou a mensagem do backend). O carregamento usa um tempo limite (`invokeWithTimeout`, `asyncCommand.ts`).
- **Limitações:** uma sessão registrada aqui **não** altera presença, notas, matrícula ou entregas na UVA (`AGENTS.md` e `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`).

### 4.4 Agenda (`agenda`)

- **Propósito:** prazos de avaliações publicados no Canvas, por semestre, com link para a página de origem.
- **Dados:** eventos `academic_events` do semestre escolhido. Cada linha (`CalendarRow`) mostra a data (`startsAt` exatamente como gravada, sem formatação), o título, a matéria (ou o rótulo da fonte) e o link "Abrir fonte ↗" para a URL de origem. O campo `verified_at` existe no banco, mas **não** é exibido.
- **Fonte:** `list_calendar_events` (SQLite). Atualizar: `sync_calendar` → `CanvasSession::list_assignment_events` (Chromium) → `replace_course_calendar_events`.
- **Funcionalidades:** seletor de semestre com **duas opções fixas no código**: `2027.1 · próximo semestre` e `2026.2 · atual`; botão "Atualizar prazos do Canvas" **só aparece para 2026.2**; link "Calendário oficial UVA" (`https://www.uva.br/vida-academica/calendario-academico/`, abre fora do app).
- **Estados vazios (textos reais):** em 2027.1: "Calendário 2027.1 ainda não confirmado — … A agenda fica sem datas até a publicação ou confirmação pelo Portal/Canvas." e "Nenhuma data oficial confirmada"; em 2026.2: "Ainda sem prazos sincronizados — Conecte-se ao Canvas e atualize para importar os prazos das disciplinas atuais." com botão "Conectar ao Canvas".
- **Erros:** falhas por matéria são somadas e exibidas no banner de erro; o aviso informa quantas matérias não puderam ser atualizadas.
- **Offline:** mostra o que já foi salvo; atualizar exige o Canvas.
- **Limitações:** os semestres da lista são constantes no código (`calendarSemester` inicia em `"2027.1"`); outros semestres não são selecionáveis. O app **não agenda nem envia nada**: só lê datas. Nenhum prazo é estimado.

### 4.5 Acadêmico (`curriculos`) — `AcademicPortal.tsx`, `CanvasAcademicPanel.tsx`, `FinalGradeCalculator.tsx`, `AcademicCurricula` (em `App.tsx`)

Reúne quatro blocos, em ordem:

1. **Portal UVA e Lyceum** (`AcademicPortal`, "REGISTRO OFICIAL").
   - Botões conforme o estado: "Abrir janela do Lyceum" (desconectado ou erro), "Verificar acesso" (aguardando login), "Capturar histórico e notas" e "Atualizar status" (acesso ativo).
   - Mostra mensagem de estado e "Página atual:" com a URL **já sanitizada** pelo backend (`lyceum::sanitize_source_url`, remove query e fragmento; hosts fora dos domínios UVA viram estado de erro).
   - Resultado da captura: páginas, campos, tabelas e links lidos, seções não lidas e avisos; mostra os caminhos do JSON e do Markdown gerados (`materias/catalogo/capturas/lyceum-<id>.*`; hoje o `.gitignore` ignora `materias/catalogo/capturas/`, decisão de configuração que pode mudar).
   - Fonte: `open_lyceum`, `lyceum_status`, `refresh_lyceum_catalog` → `lyceum.rs`. **Não verificado em execução real.**
2. **Notas, fóruns, avisos e Caixa de Entrada do Canvas** (`CanvasAcademicPanel`, "SOMENTE LEITURA").
   - Caixas "O que ler" (notas, fóruns, avisos, Caixa de Entrada) e botão "Ler agora" (ou "Conectar Canvas" se desconectado). Exige ao menos uma opção. A Caixa de Entrada traz aviso de mensagens privadas.
   - Resultado: por disciplina, atividades com nota/prazo/entrega e feedback do tutor, fóruns e avisos; caminhos de JSON e Markdown (`materias/catalogo/capturas/canvas-<uuid>.*`).
   - Fonte: `refresh_canvas_academic` → `canvas_api.rs` e `CanvasSession::capture_canvas_academic`. **Não verificado em execução real** (o estado de implementação registra "nenhuma leitura autenticada foi executada ainda").
3. **Nota final ponderada** (`FinalGradeCalculator`, "CÁLCULO LOCAL · SEM REDE").
   - Tipo Regular (A1, A2, A3) ou Especial (A1, A4); campos aceitam 0 a 10; resultado traz status, NFp, mínimo para aprovação, prova usada, nota mínima necessária e a fórmula. O texto da tela afirma que é estimativa e que vale o boletim do Lyceum.
   - Fonte: `calculate_final_grade` → `grading.rs`; não grava nada.
4. **Catálogo curricular** (`AcademicCurricula`): dois currículos lado a lado (campo "Buscar disciplina ou código"), pendências, matrículas atuais, notas e frequência por disciplina com fonte e data de observação, cursos Canvas anteriores e limitações registradas. Disciplinas candidatas a conteúdo comum aparecem sem equivalência confirmada.
   - Fonte: `academic_catalog`. Se falhar: nota "Catálogos curriculares não encontrados nesta pasta." (ou a mensagem do backend). Sem catálogo, os blocos 1 a 3 continuam visíveis.
- **Publicação:** esta tela exibe dados do aluno (notas, frequência, trajetória). Pela política de publicação do dono (6/11/2026, `AGENTS.md`), notas, frequência, matrículas e trabalhos podem ir ao Git e ao acervo público, desde que separados do catálogo em `privado/`. Continuam proibidos senhas, cookies, tokens, `.env`, `*.sqlite3*`, perfil do Chromium e `originais/`; RA, CPF e RG seguem fora dos registros (a captura do Lyceum já omite campos de nome, CPF, RA e matrícula). O que o app grava e onde não mudou: `materias/catalogo/capturas/` e o SQLite local; a regra atual do `.gitignore` para `capturas/` é configuração do repositório, não requisito do app.
- **Limitações:** a captura do Lyceum respeita 50 páginas e 10.000 linhas por tabela e pode ficar parcial; o botão não executa ações acadêmicas (não envia atividade, não altera registro).

### 4.6 Dados e sincronização (`acervo`, a tela "Acervo da pasta")

- **Propósito:** estado da pasta do acervo, sincronização entre arquivos e SQLite, conflitos, Git e busca/leitura de documentos. Descrita na própria tela como "leitura somente local, sem login no Canvas".
- **Blocos, em ordem:**
  1. **Fontes acadêmicas:** estado do Canvas, "Abrir cursos e importação", "Atualizar Canvas ↻" ou "Conectar Canvas", "Escolher Chromium" (diálogo de arquivo; grava `chromium_executable` em `settings`).
  2. **Pasta raiz do acervo:** caminho, "Sincronizar agora" (`sync_local_catalog` e recarga) e "Escolher pasta" (`set_library_path`).
  3. **Sincronização local:** contagens (arquivos, novos, atualizados, pendências, conflitos), data da última reconciliação e primeiro erro, se houver.
  4. **Repositório Git:** branch, quantidade de alterações rastreadas, arquivos novos e ignorados, e até 8/8/4 caminhos de exemplo. Somente leitura (`repository_status`); o app não faz stage, commit nem push.
  5. **Conflitos para revisar:** cada conflito é um `<details>` com botões para escolher a versão (SQLite, Markdown ou JSON; para arquivo excluído, "Restaurar do …"). Resolver chama `resolve_local_sync_conflict` e **escreve** nas três representações.
  6. **Busca** (`aria-label="Buscar nos arquivos locais"`; até 100 resultados) e **lista paginada** de documentos (100 por página; `aria-label="Páginas do acervo"`).
  7. **Leitor de documento** (Markdown renderizado com `remark-gfm`; links relativos entre documentos locais abrem no próprio leitor; imagens locais via `read_local_asset`, até 16 MiB). Formatos editáveis: Markdown, texto, ALG, CSV e JSON, com "Editar conteúdo" → `edit_synced_document` (**escreve** no arquivo, SQLite e espelho).
  8. **Aulas guiadas:** lista de materiais importados pelo app.
- **Formatos listados na tela:** "Markdown · TXT · ALG · CSV · JSON · PDF · DOCX · Pages".
- **Estados vazios (textos reais):** "Lendo a pasta do acervo…"; "Nenhum documento compatível encontrado — Escolha a pasta em que estão seus materiais acadêmicos."; busca sem resultado: "Nenhum resultado encontrado — Tente outro termo…".
- **Avisos de limite (textos reais):** "O limite de 10.000 documentos compatíveis ou a profundidade de pastas foi atingido; parte da pasta ainda não foi indexada."; pastas e arquivos ilegíveis; "mostrando os primeiros 100".
- **Fonte:** `refresh_local_catalog`/`search_local_library`/`read_local_document` (→ `catalog::scanner`, `catalog::documents`) e `SyncEngine`.
- **Limitações importantes:**
  - O escopo do scanner e da sincronização é uma lista fixa de pastas no código (`docs`, `materias`, `originais`, `fontes-uva`, `periodos`, `cursos` e alguns arquivos na raiz). Na árvore de trabalho atual a pasta `privado/` **não está nessa lista**, e `cursos/` foi movida para `privado/cursos/`; portanto o app não indexa o conteúdo de `privado/`. Ver [lacunas e contradições](../referencia/lacunas-e-contradicoes.md).
  - O monitor de arquivos (`sync_watcher`) acompanha `materias` e `docs` de forma recursiva e a raiz de forma não recursiva.
  - Arquivos sob `originais/` e nomes contendo senha, token, cookie, credencial ou segredo ficam fora da sincronização.

### 4.7 Canvas e importação (`canvas`, sem item na barra lateral)

- **Propósito:** navegar pelos cursos do Canvas e módulos e importar um PDF individual.
- **Funcionalidades:** "Atualizar lista" (`list_courses`), selecionar disciplina (`load_course`), "Localizar PDF" (`open_module_item`), "Importar PDF" (`import_material`; cancelável) e fechar a lista de origens.
- **Estados vazios (textos reais):** "Carregue suas disciplinas — A lista vem diretamente da sua conta Canvas…"; "Os módulos aparecem aqui"; "Nenhum módulo encontrado".
- **Avisos:** "Este item não apresentou link de PDF reconhecido. O app não abriu tarefas, fóruns ou avaliações."
- **Efeito de uma importação:** converte o PDF em Markdown por página, grava o Markdown e a aula no acervo, registra o material no SQLite e abre a aula guiada. O PDF bruto não é mantido.
- **Offline:** inutilizável sem sessão do Canvas. **Não verificado em execução real.**

## 5. Aula guiada e Resumos

### 5.1 Aula guiada (`estudar`) — `LessonPresenter.tsx`

- **Propósito:** apresentar um documento (local ou importado) por seções, com progresso salvo, narração e atividade local.
- **Entrada:** abrir um documento local (`read_local_document`, `begin_lesson_session` com o caminho relativo), um material importado (`get_lesson`, `resource_path = "canvas:<id>"`) ou retomar pelo histórico.
- **Seções:** `splitLessonSections` (`studyTypes.ts`) divide por marcadores de página (`## Página N`), por form feed ou por títulos Markdown, descarta seções sem conteúdo de ensino e preserva a página de origem.
- **Progresso:** cada troca de seção chama `save_lesson_progress` (SQLite, `lesson_sessions`); na última seção o botão "Próxima seção →" dá lugar a "Concluir aula ✓" (`completed_at` é gravado). Barra de progresso com percentual.
- **Atividade interativa local:** para cada seção, `build_lesson_activity` monta uma pergunta determinística com trecho da fonte; a resposta digitada (até 16.384 caracteres) é avaliada por `assess_lesson_answer` e salva por `save_lesson_interaction` (`lesson_interactions`). O texto da tela afirma que as respostas "ficam neste dispositivo; não são sincronizadas nem exportadas". Se o Markdown mudar depois da tentativa, a tela avisa e mostra o trecho salvo. Não há IA nesse fluxo.
- **Prática do material importado:** apenas para materiais importados (não para documentos locais): perguntas de `start_practice`, conferência com `submit_practice`, resultado em percentual e resposta de referência com página. O resultado vai para `study_sessions` e para o Histórico. Texto da tela: "não altera suas notas oficiais".
- **Narração local:** ver §5.2.
- **Imagens e links:** imagens locais PNG/JPEG/WebP/GIF via `read_local_asset`; falha mostra "Imagem local indisponível: …". Links para documentos locais abrem como nova aula; `http(s)` e `mailto:` abrem fora do app; os demais viram texto simples.
- **Estados e erros (textos reais):** sem aula aberta: "Nenhuma aula aberta — Entre em uma matéria e escolha uma aula para apresentar."; "Preparando atividade local desta seção…"; falha de salvar progresso: "Não foi possível salvar seu progresso. Tente novamente." (`role="alert"`); sem perguntas na prática: "Este texto não gerou perguntas automáticas confiáveis…".
- **Limitações:** a leitura de documentos DOCX/Pages/PDF passa pelo conversor do backend e pode trazer avisos ("warnings" exibidos no leitor); diagramas e imagens de PDF não são reproduzidos.

### 5.2 Síntese de voz (`lessonSpeech.ts`, `summarySpeech.ts`)

Usa a Web Speech API do WebView (`window.speechSynthesis`), sem serviço externo do app.

- **Aula (`lessonSpeech.ts`):** escolhe apenas vozes com `localService === true`, preferindo `pt-BR`, depois qualquer `pt`. Se não houver voz local, os botões "Ouvir"/"Repetir" ficam desabilitados e a tela diz: "Narração local indisponível neste WebView. O conteúdo continua disponível para leitura e resposta digitada." Controles: Ouvir (ao terminar, avança para a próxima seção), Pausar, Continuar, Repetir, Parar; a linha de estado é `aria-live="polite"`.
- **Resumos (`summarySpeech.ts`):** usa `SpeechSynthesisUtterance` com `lang = "pt-BR"`; Pausar, Retomar, Parar. Se a síntese não existir: "A síntese de voz não está disponível neste dispositivo." Esse arquivo **não** filtra por voz local (diferente de `lessonSpeech.ts`); a voz escolhida pelo sistema pode ser remota, dependendo do WebView. Comportamento real **não verificado**.

### 5.3 Aba Resumos (`StudySummaryPanel.tsx`)

- **Propósito:** gerar um resumo curto e um longo (rótulos "cerca de 2 minutos" e "cerca de 8 minutos") do material didático local de uma matéria e semestre, para ouvir.
- **Envio a terceiro:** depois de uma confirmação explícita ("Enviar material à Anthropic?", `role="alertdialog"`, lista os caminhos dos arquivos), os **textos dos materiais são enviados à API da Anthropic**. A tela avisa que "a chamada pode consumir créditos" e que "notas e registros oficiais não fazem parte das fontes".
- **Fluxo:** `get_subject_summary_snapshot` lista as fontes e calcula um hash; "Gerar resumos" abre a confirmação; `generate_subject_summaries` recebe o hash confirmado, recoleta as fontes e **recusa** se mudaram ("Os materiais desta matéria mudaram desde a confirmação…"). O resultado é salvo em `study_summaries` (SQLite local) com modelo, tokens e manifesto de fontes. Um resumo salvo mostra "Atualizado" ou "Fontes alteradas".
- **Credencial:** a chave é lida pelo backend de um arquivo `.env` fora do repositório (função `AnthropicConfig::read_default`); a interface não pede nem exibe chave. O valor nunca deve aparecer em documentação.
- **Estados vazios (textos reais):** "Sem semestre associado — Esta matrícula ativa ainda não informa o semestre necessário…"; "Não há materiais legíveis associados a esta matéria e semestre. Nenhuma chamada à API será feita."
- **Offline:** ler um resumo salvo e ouvi-lo funcionam; gerar exige rede e chave. O estado de verificação da API real está em [features/README.md](../features/README.md); o cliente mostrou erro HTTP 400 na checagem de 2026-11-06 (chave sem workspace). **Não verificado em execução real.**
- **Observação de consistência:** o texto da barra lateral ("Sem IA, nuvem ou cobrança por uso") não vale para esta aba.

## 6. Painéis reutilizados

### 6.1 `StudyHistory`
Descrito em §4.3; também aparece na aba Histórico de cada matéria.

### 6.2 `CanvasAcademicPanel`, `AcademicPortal`, `FinalGradeCalculator`
Descritos em §4.5.

### 6.3 `CanvasCapturePanel` (dentro de Matérias)

- **Propósito:** varrer, uma disciplina e um item por vez, os cursos e materiais publicados do Canvas e salvá-los como Markdown local.
- **Controles:** "Abrir Canvas para entrar ↗", "Capturar todas as disciplinas", "Retomar captura" / "Retomar após entrar no Canvas", "Cancelar com segurança", "Tentar novamente" por item com falha reutilizável, "Carregar mais resultados" (páginas de 200 itens).
- **Limites editáveis** (bloco "Limites da varredura", validados no frontend e no backend): páginas de cursos (1–100), cursos (1–500), itens (1–20.000), tamanho máximo do PDF (1–60 MiB), redirecionamentos (0–4) e tempo por item (5–120 s). Salvos por `set_canvas_capture_limits`.
- **Progresso:** `<progress aria-label="Progresso da captura">`, contagem de cursos e itens, avisos em `role="status"`, erros em `role="alert"`.
- **Texto de privacidade exibido:** "A captura apenas lê materiais publicados. O acesso pode aparecer no histórico comum do Canvas; ele não registra presença, conclusão de aula, notas ou entregas."
- **Estados do item:** o rótulo vem de `captureItemStatusLabel` (`canvasCapture.ts`); itens `needs_review` mostram "Revisar arquivo local" (arquivo existente e alterado não é sobrescrito).
- **Fonte e escrita:** `start_canvas_capture`, `resume_canvas_capture`, `cancel_canvas_capture`, `canvas_capture_status`, `canvas_capture_items`, `retry_canvas_capture_item`, `get/set_canvas_capture_limits` (SQLite: `capture_runs`, `capture_items`, `materials`, `material_associations`; arquivos Markdown no acervo).
- **Não verificado em execução real:** nenhuma varredura autenticada foi executada pelo app nesta revisão; os testes automatizados não comprovam que os cursos foram baixados.

## 7. Acessibilidade

**Presente no código (verificado por leitura):**

- `lang="pt-BR"` em `app/index.html`.
- Landmarks e rótulos: `<nav aria-label="Navegação principal">`, `<nav aria-label="Seções de <matéria>">`, `<nav aria-label="Páginas do acervo">`; seções com `aria-labelledby` (captura, Canvas acadêmico, nota final, Lyceum, resumos); campos de busca com `aria-label` ou rótulo `sr-only` (classe `.sr-only` existe em `styles.css`).
- Anúncios: `role="status"` e `role="alert"` nos avisos, erros, progresso de importação, estado do Lyceum e resultados; `aria-live="polite"` na linha de narração; `role="alertdialog"` com `aria-modal="true"` na confirmação de envio dos resumos; `<progress>` nativo com `aria-label`.
- Ícones decorativos de pelo menos um cartão têm `aria-hidden="true"`; botões de fechar têm `aria-label`.
- O corpo da aula (`.markdown-body`) tem `tabIndex={-1}` e estilo `:focus-visible`, usado pelo botão "Ir ao trecho da fonte acima ↑".
- Narração em voz local (§5.2) e alternativa textual quando indisponível.

**Pontos observados que merecem atenção (fatos do código, não avaliação WCAG):**

- Os botões da barra lateral, das abas da matéria e do sumário da aula **não** usam `aria-current` nem `role="tab"`/`aria-selected`; o item ativo só é marcado pela classe CSS `active`.
- `styles.css` define a regra `:focus-visible` uma única vez (para `.markdown-body`) e tem 6 ocorrências de `outline:0`/`outline:none`; o foco visível dos demais controles depende do estilo padrão do WebView ou de regras `:focus` pontuais (5 ocorrências). Não há `prefers-reduced-motion`.
- Há muitos tamanhos de fonte pequenos: 1 regra de 6 px, 6 de 7 px, 50 de 8 px, 56 de 9 px e 51 de 10 px.
- O mesmo glifo "◷" é usado para Histórico e Agenda; vários ícones são glifos de texto sem alternativa.
- A confirmação de envio dos resumos declara `aria-modal`, mas o código lido não mostra captura de foco nem fechamento por Escape.

**Não verificado:** navegação completa por teclado, ordem de tabulação, contraste de cores, leitura com VoiceOver ou outro leitor de tela, zoom/ajuste de texto e uso em janela de 960×640 (tamanho mínimo definido em `tauri.conf.json`). Nenhuma auditoria automatizada de acessibilidade foi executada.

## 8. Testes do frontend

`npm test --prefix app` executa `node --experimental-strip-types --test scripts/*.test.mjs`. Resultado desta revisão: **40 testes passaram, 0 falharam**. Cobrem a lógica pura em TypeScript (catálogo acadêmico, comandos assíncronos, captura do Canvas, leitura acadêmica, fala da aula, resumos, divisão em seções e fluxo do tutor). Não exercitam a interface renderizada nem o backend.

`npm run build` (`tsc -b && vite build`) **não foi executado** nesta revisão; portanto a compilação do frontend não está comprovada aqui.

Nenhum teste automatizado cobre as telas em um WebView real. O fluxo visual do build atual também não foi confirmado (o estado de implementação registra que a tentativa de ler a árvore de acessibilidade do app empacotado expirou).

## 9. Itens que não existem na interface

Para evitar leitura equivocada, **não há** na interface atual: tela de login do app (a autenticação é feita pelo usuário na janela do Chromium), edição de notas, envio de atividades, agendamento de provas, alteração de registros institucionais, configuração de chave de API, seletor de idioma, modo escuro, nem tela GPUI.

## Referências de código

- `app/src/ui/App.tsx` — navegação, estado, chamadas `invoke`, tela Dados e sincronização, Agenda, Canvas e importação, `AcademicCurricula`.
- `app/src/ui/StudyHome.tsx`, `SubjectList.tsx`, `SubjectDetail.tsx`, `StudyHistory.tsx`, `LessonPresenter.tsx`, `StudySummaryPanel.tsx`.
- `app/src/ui/CanvasCapturePanel.tsx`, `CanvasAcademicPanel.tsx`, `FinalGradeCalculator.tsx`, `AcademicPortal.tsx`.
- `app/src/ui/studyTypes.ts`, `canvasCapture.ts`, `canvasAcademic.ts`, `lessonSpeech.ts`, `summarySpeech.ts`, `asyncCommand.ts`.
- `app/src-tauri/src/lib.rs` — comandos e eventos; `app/src-tauri/tauri.conf.json` — janela (1280×820, mínimo 960×640) e CSP.
