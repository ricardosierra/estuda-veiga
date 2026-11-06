# Estuda UVA: tutor interativo local — Implementation Plan

> **Para execução:** seguir `superpowers:executing-plans`, uma tarefa de cada vez, mantendo o ciclo TDD. Este plano foi escrito após aprovação da especificação e aguarda revisão da estudante antes de qualquer alteração de código.

**Objetivo:** permitir que qualquer Markdown local seja apresentado como aula interativa, com narração local, resposta digitada, devolutiva ligada à fonte e registro/revisão local.

**Arquitetura:** manter o parser de seções do React e o presenter existentes; enriquecer as seções com referência de página quando disponível; gerar perguntas e avaliação estritas de forma determinística em Rust; usar a síntese de fala local do WebView com controles React acessíveis; persistir tentativas em SQLite local vinculadas à sessão, revisão do Markdown, seção e trecho. Não transmitir conteúdo nem respostas.

**Tecnologias:** Tauri 2, Rust, rusqlite, React, TypeScript, `window.speechSynthesis`, Node test runner e testes Rust offline.

**Especificação:** `docs/superpowers/specs/2026-10-14-estuda-uva-tutor-interativo-local-design.md`

## Restrições globais

- Manter o processamento offline e determinístico; não adicionar cliente/API de IA nem serviço remoto.
- Preservar arquivos Markdown e PDFs de origem byte a byte; perguntas, respostas e progresso não editam o acervo nem entram no espelho acadêmico JSON/Markdown.
- Não guardar respostas de estudo em Git, Canvas ou sincronização entre dispositivos; persistir somente no SQLite local do app.
- Nunca confundir leitura, áudio, interação ou conclusão local com presença, nota, envio ou conclusão oficial da UVA.
- Não solicitar microfone, gravação ou reconhecimento de fala.
- Preservar a prática existente para materiais Canvas e a navegação atual de aulas.
- Manter o checkout compartilhado: não resetar, limpar, trocar branch/worktree ou reformatar arquivos sem relação. O commit da especificação existente contém somente seu arquivo; não alterar documentos ou materiais de captura em andamento.
- Escrever os testes de cada tarefa primeiro, confirmar que falham pelo comportamento ausente, implementar o mínimo e repetir as verificações focadas. Depois executar build e runtime local de áudio; relatar limites de plataforma explicitamente.

## Foco de revisão

- Uma paráfrase não deve receber falso resultado “errado” só porque difere do gabarito literal. Cobertura: `paraphrase_without_exact_match_is_not_marked_incorrect` (Tarefa 1).
- Resposta, pergunta e trecho podem incluir instruções hostis ou Markdown ativo. Cobertura: `tutor_activity_keeps_untrusted_markdown_inert` e renderização segura no presenter (Tarefas 1 e 4).
- A API/voz pode não existir no WebView. Cobertura: `speech_unavailable_keeps_typed_lesson_usable` (Tarefa 3).
- A fala não deve continuar após trocar matéria/seção ou desmontar o presenter. Cobertura: `speech_is_cancelled_on_section_change_and_unmount` (Tarefa 3).
- Mudança do Markdown após a tentativa não deve apontar silenciosamente para conteúdo diferente. Cobertura: `interaction_review_detects_changed_markdown_revision` (Tarefa 2).
- Estudo não pode modificar estado acadêmico oficial nem gerar registros de presença. Cobertura nos testes de comando/integração e inspeção final de comandos chamados (Tarefas 2, 4 e 5).

---

### Task 1: Definir seção, atividade e avaliação local com fonte

**Arquivos:**
- Criar: `app/src-tauri/src/tutor.rs`
- Modificar: `app/src-tauri/src/lib.rs` (declaração do módulo e comandos apenas depois do contrato)
- Modificar: `app/src/ui/studyTypes.ts`
- Testar: novo `app/scripts/tutor.test.mjs` e, se necessário, `app/scripts/study-types.test.mjs`

**Contratos:**
- `LessonSection { index, title, markdown, sourcePage?, sourceHeading, sourceExcerpt, sourceSha256 }` liga o bloco do presenter à origem exata. `sourcePage` só existe quando vem de um cabeçalho explícito `Página N` ou de um separador `\f`; texto sem metadado não recebe página estimada.
- `TutorActivity { id, sessionId, sectionIndex, kind, prompt, referenceAnswer?, sourcePage?, sourceHeading, sourceExcerpt, sourceSha256 }`; `kind` é `verifiable` ou `self_assessment`.
- `TutorAssessment { status: correct | self_assessed | pending, correct?: boolean }`; respostas diferentes do texto literal não são corrigidas automaticamente como erradas.
- `build_tutor_activity(section, session_id) -> TutorActivity` usa somente padrões explicitamente reconhecíveis (incluindo definições já suportadas por `lesson.rs`) e produz pergunta aberta/autorrevisão quando não existe gabarito confiável.
- `assess_tutor_answer(activity, response) -> TutorAssessment` confirma acerto apenas quando uma resposta de referência não vazia coincide após normalização determinística; demais respostas pedem autoavaliação e mostram referência/fonte, sem percentual enganoso.
- `splitLessonSections` inclui número de página quando encontra cabeçalho explícito `Página N` ou separador `\f`; seção de Markdown simples preserva título/trecho sem inventar uma página.

- [ ] **Passo 1:** adicionar testes para associação de página/seção tanto por `## Página N` quanto por `\f`, Markdown sem página, definição explícita, seção sem gabarito, pergunta aberta e resposta exata.
- [ ] **Passo 2:** executar os testes focados e confirmar falha inicial por ausência do contrato/atividade.
- [ ] **Passo 3:** implementar DTOs e gerador determinístico em `tutor.rs`; reaproveitar a detecção já existente sem criar um segundo classificador divergente.
- [ ] **Passo 4:** separar a normalização de acerto em regra pura. Resposta diferente deve ficar em revisão/autoavaliação, nunca `incorrect` automático.
- [ ] **Passo 5:** enriquecer parser de seções com origem opcional, sem mudar o texto Markdown usado no renderizador nem a posição esperada pela sessão.
- [ ] **Passo 6:** executar `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib tutor::tests` e `node --experimental-strip-types --test app/scripts/tutor.test.mjs app/scripts/study-types.test.mjs`.

### Task 2: Persistir tentativa, revisão e retomada apenas no SQLite local

**Arquivos:**
- Modificar: `app/src-tauri/src/storage.rs`
- Modificar: `app/src-tauri/src/lib.rs`
- Modificar: `app/src/ui/studyTypes.ts`
- Testar: `storage.rs` e testes focados de comandos/types

**Contratos:**
- Tabela `lesson_interactions`: `id`, `session_id`, `activity_id`, `section_index`, `kind`, `prompt`, `response`, `assessment`, `source_page`, `source_heading`, `source_excerpt`, `source_sha256`, `attempted_at`.
- A chave de revisão é sessão + atividade; tentativas anteriores podem permanecer como histórico ou revisão repetida, sem sobrescrever respostas sem aviso.
- `save_lesson_interaction(session_id, activity, response, assessment)` valida tamanho, sessão e índice antes de gravar em transação.
- `list_lesson_interactions(session_id)` retorna respostas da própria sessão em ordem de tentativa para revisão.
- `get_lesson_session(session_id)` reabre a sessão escolhida no histórico sem iniciar outra sessão para o mesmo arquivo; `get_lesson_interaction` ou equivalente retorna o trecho registrado e compara o hash atual da seção antes de oferecer navegação de volta; hash diferente é sinalizado como material alterado.
- Migração SQLite eleva `CURRENT_SCHEMA_VERSION` sem apagar tabelas/linhas existentes. Esta tabela não é projetada para JSON acadêmico, exportação ou sync.

- [ ] **Passo 1:** escrever testes para migração da versão atual, gravação/retorno de resposta, vínculo com `lesson_sessions`, limites de entrada, preservação de tentativas repetidas e aviso de hash diferente.
- [ ] **Passo 2:** rodar somente esses testes e confirmar falha na tabela/comandos ausentes.
- [ ] **Passo 3:** implementar migração transacional compatível com `lesson_sessions` existente; não alterar as tabelas `academic_*`, `study_sessions` ou frequência/notas.
- [ ] **Passo 4:** implementar funções de storage e comandos Tauri `build_lesson_activity`, `save_lesson_interaction`, `list_lesson_interactions` e `get_lesson_session`, todos locais e sem acesso à rede.
- [ ] **Passo 5:** verificar que a revisão de atividade continua segura se o caminho local não existir ou a sessão pertencer a outro material.
- [ ] **Passo 6:** executar `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib storage::tests` e os testes de comando recém-adicionados.

### Task 3: Implementar narração local e controles acessíveis

**Arquivos:**
- Criar: `app/src/ui/lessonSpeech.ts`
- Criar: `app/scripts/lesson-speech.test.mjs`
- Modificar: `app/src/ui/LessonPresenter.tsx`
- Modificar: `app/src/ui/styles.css`

**Contratos:**
- `speechTextFromMarkdown(markdown) -> string` remove sintaxe, destinos de link, tokens de formatação, fórmulas/trechos de código não narráveis e mantém texto alternativo útil; a vista Markdown não é alterada.
- Hook `useLessonSpeech(text)` expõe `available`, `speaking`, `paused`, `voiceLabel`, `speak`, `pause`, `resume`, `stop`; escolhe voz local `pt-BR` quando houver, sem persistir preferências sensíveis ou carregar conteúdo remoto.
- `speak()` só escolhe uma voz anunciada como local (`SpeechSynthesisVoice.localService === true`); se a API/voz local não puder ser confirmada, UI mostra aviso e mantém leitura e digitação disponíveis, sem cair em voz remota.
- Mudança do texto/seção, saída do presenter ou unmount chama `speechSynthesis.cancel()` e remove handlers pendentes.

- [ ] **Passo 1:** adicionar testes com um adaptador de speech API simulado: limpeza de Markdown, voz pt-BR prioritária, ausência de API/voz e cancelamento em mudança/desmontagem.
- [ ] **Passo 2:** confirmar que os testes falham porque a camada e os controles não existem.
- [ ] **Passo 3:** implementar limpeza de texto e um controlador de fala local testável com API injetável; ligar seu estado reativo ao presenter sem pacote externo.
- [ ] **Passo 4:** adicionar controles por teclado **Ouvir**, **Pausar**, **Continuar**, **Repetir** e **Parar**, estados live/accessibility e mensagem de voz ausente.
- [ ] **Passo 5:** impedir auto-play ao abrir aula ou retornar de outra tela; iniciar áudio apenas por ação explícita.
- [ ] **Passo 6:** executar `npm test --prefix app -- lesson-speech.test.mjs` e build TypeScript de produção.

### Task 4: Integrar atividade digitada, fonte e revisão no presenter

**Arquivos:**
- Modificar: `app/src/ui/LessonPresenter.tsx`
- Modificar: `app/src/ui/App.tsx`
- Modificar: `app/src/ui/StudyHistory.tsx`
- Modificar: `app/src/ui/studyTypes.ts`
- Modificar: `app/src/ui/styles.css`
- Testar: funções puras do fluxo de tutor; validar os componentes interativos no runtime do Tauri, pois o projeto não possui atualmente uma infraestrutura DOM de componentes React.

**Comportamento:**
- Ao abrir Markdown local ou aula Canvas já importada, preparar uma atividade local para a seção atual e carregar tentativas salvas da sessão. Reabrir uma entrada do histórico carrega a mesma sessão e suas tentativas, inclusive quando a aula já foi concluída.
- Renderizar pergunta após o trecho; permitir resposta digitada e envio, reler/ouvir, revisão de seção, autoavaliação para pergunta aberta e navegação para a fonte.
- Só mostrar referência após a tentativa. Exata coincidência normalizada pode virar acerto; respostas diferentes trazem referência e autoavaliação sem rótulo errado nem nota inventada.
- Ao mudar seção, trocar arquivo/sessão ou voltar à matéria, cancelar áudio; gravar progresso e tentativas via comandos locais com feedback de falha recuperável.
- Manter o cartão de prática Canvas existente separado e identificá-lo como prática local quando atividade vier do novo tutor.
- Informar antes da primeira tentativa que as respostas ficam no dispositivo e não são sincronizadas/exportadas.

- [ ] **Passo 1:** escrever testes de fluxo para Markdown sem Canvas, material importado, fonte sem página, resposta exata, paráfrase, autoavaliação, falha de storage e reabrir a mesma sessão pelo histórico; cobrir as transições puras do presenter sem introduzir dependências de DOM.
- [ ] **Passo 2:** confirmar falha dos casos novos antes de integrar componentes.
- [ ] **Passo 3:** ligar a preparação/retorno do DTO ao `LessonPresenter` e cobrir todos os caminhos de abertura em `App.tsx` sem remover perguntas Canvas existentes.
- [ ] **Passo 4:** ligar formulário e estado de devolutiva ao armazenamento local; revisão abre a seção e destaca o trecho sem alterar o Markdown fonte; histórico reabre as respostas da sessão correta.
- [ ] **Passo 5:** validar foco/teclado, rótulos, anúncio do estado de gravação/áudio e layout em viewport mínimo do app.
- [ ] **Passo 6:** executar testes frontend focados e `npm run build --prefix app`.

### Task 5: Validar o fluxo real no Estuda UVA e fechar a evidência

**Arquivos:**
- Nenhuma nova mudança prevista além de correções das tarefas anteriores.
- Atualizar documentação de uso somente se necessário e após a UI final existir.

- [ ] **Passo 1:** executar os testes Rust relacionados a tutor/storage, testes frontend focados, `cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check`, `git diff --check` e build local do app.
- [ ] **Passo 2:** abrir o app construído no WebView macOS, carregar um Markdown local representativo de fórmula matemática e outro de texto conceitual, e confirmar voz pt-BR, pausa/retomada, cancelamento, campo digitado, fonte/página, persistência e revisão ao reabrir a mesma sessão pelo histórico.
- [ ] **Passo 3:** repetir uma navegação offline e confirmar que nenhuma requisição de tutor é feita e nenhum comando de presença/avaliação é chamado.
- [ ] **Passo 4:** verificar status/diff e que os arquivos-fonte permaneceram intactos. Reportar evidência por nível: testes focados, build, execução autenticada/local, e limitações restantes.

## Mapa de cobertura

| Requisito da especificação | Tarefas |
|---|---|
| Qualquer Markdown local, seção e referência verificável | 1, 4 |
| Narração pt-BR local, controles, fallback e cancelamento | 3, 4, 5 |
| Respostas digitadas, pergunta aberta e devolutiva sem falso erro | 1, 2, 4 |
| Persistência/revisão local por sessão e revisão do Markdown | 2, 4 |
| Offline, privacidade, sem APIs remotas ou sincronização das respostas | 2, 3, 5 |
| Separação de prática local e registros oficiais | 2, 4, 5 |
| Acessibilidade de controles e formulário | 3, 4, 5 |
