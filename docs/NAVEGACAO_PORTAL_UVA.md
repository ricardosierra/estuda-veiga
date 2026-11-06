# Navegação no Portal do Aluno UVA e nos seus 5 subsistemas

**Atualizado em:** 6 de novembro de 2026
**Escopo:** como chegar a cada tela do [Portal do Aluno UVA](https://portalaluno.uva.br/Aluno/PortalAluno) e qual procedimento determinístico em Rust nativo o Estuda UVA usa (ou deve usar) para cada tarefa. Nenhum procedimento deste guia chama IA: são navegação Chromium/CDP, requisições HTTP autenticadas pela sessão do navegador, parsing de HTML/JSON e cálculos simples.

Os caminhos de tela vêm dos tutoriais oficiais extraídos para o acervo (ver [Fontes](#fontes)). Rotas e endpoints que ainda não foram observados numa sessão autenticada estão marcados como **a validar**. Não registre aqui hosts, rotas ou valores que não tenham sido vistos.

## Regras que valem para todos os procedimentos

1. **Sem IA.** Os procedimentos usam `chromiumoxide` (CDP), `reqwest`, `scraper`, `serde_json` e o conversor de PDF local. `anthropic.rs`, `tutor.rs` e `study_summaries.rs` não participam da navegação nem da coleta.
2. **Login sempre manual.** O estudante digita CPF/senha/MFA na janela Chromium visível do app (perfil isolado, `CanvasSession::launch` em `app/src-tauri/src/browser.rs`). O app não lê, não grava e não reaproveita credenciais, cookies, tokens ou URLs de SSO.
3. **Somente leitura.** O app nunca envia formulário, posta em fórum, entrega tarefa, agenda/cancela prova, abre protocolo, faz matrícula ou mexe no financeiro. Esses passos aparecem neste guia como **manual** (⛔) para o estudante saber onde fica, mas não têm automação.
4. **Não marcar como lido.** Procedimentos de leitura devem preferir APIs JSON que não alteram estado de leitura. Quando uma rota marca algo como lido por padrão, o parâmetro que desliga isso é obrigatório (ver Caixa de Entrada).
5. **Domínios permitidos.** Toda URL passa por allowlist (`lyceum::sanitize_source_url`, `canvas::sanitize_source_url`, `download::validate_material_url`). Query e fragmento são removidos antes de gravar. Um subsistema novo só entra na automação depois que seu host for observado e incluído na allowlist com teste.
6. **Saída local rastreável.** Toda coleta grava arquivo novo (JSON + Markdown) com data de captura e URL de origem sanitizada, sincroniza com o SQLite e nunca sobrescreve captura anterior nem edição local.

**Legenda de estado:** ✅ implementado no código · 🟡 procedimento definido, a implementar · ⛔ manual, o app não automatiza.

✅ quer dizer código e testes offline no checkout. Não quer dizer que a leitura já tenha rodado numa sessão autenticada; isso fica registrado em [Pendências](#pendências-deste-guia).

## Índice rápido: “quero…”

| Quero… | Subsistema | Procedimento | Estado |
|---|---|---|---|
| Ver notas oficiais (boletim) e média | Sistema do Aluno (Lyceum) | [L1](#l1-boletim-notas-e-média) | ✅ captura genérica · 🟡 rota dedicada |
| Ver frequência | Sistema do Aluno (Lyceum) | [L2](#l2-frequência) | ✅ captura genérica |
| Ver histórico, carga horária e pendências | Sistema do Aluno (Lyceum) | [L3](#l3-histórico-do-aluno) | ✅ captura genérica |
| Ver nota e feedback das AVAs (A1) | Plataforma Virtual (Canvas) | [C4](#c4-notas-e-feedback-das-avas-a1) |✅ |
| Ver prazos das atividades | Plataforma Virtual (Canvas) | [C3](#c3-prazos-das-atividades) | ✅ |
| Ler fórum da disciplina ou do Espaço Curso | Plataforma Virtual (Canvas) | [C5](#c5-fóruns) |✅ |
| Ler avisos da monitoria | Plataforma Virtual (Canvas) | [C6](#c6-avisos) |✅ |
| Ler Caixa de Entrada | Plataforma Virtual (Canvas) | [C7](#c7-caixa-de-entrada) |✅ |
| Baixar materiais (PDF, páginas de aula) | Plataforma Virtual (Canvas) | [C2](#c2-baixar-materiais) | ✅ |
| Ver nota e feedback de A2/A3 | Sistema de Provas | [P1](#p1-provas-realizadas-nota-e-feedback) | 🟡 |
| Ver data, horário e local da prova agendada | Sistema de Provas | [P2](#p2-consultar-agendamento) | 🟡 |
| Agendar, fazer ou cancelar prova | Sistema de Provas | [P3](#p3-agendar-fazer-ou-cancelar-prova) | ⛔ |
| Acompanhar solicitações/protocolos | Zeev | [Z1](#z1-acompanhar-solicitações) | 🟡 |
| Pedir revisão de prova A2/A3 | Zeev | [Z2](#z2-abrir-protocolo-incluindo-revisão-de-a2a3) | ⛔ |
| Pedir histórico, declaração ou serviço | Sistema do Aluno → Secretaria Virtual | [L4](#l4-secretaria-virtual-serviços-e-rematrícula) | ⛔ |
| Boletos, extrato, negociação | Financeiro / Plataforma de Pagamento | [F1](#5-plataforma-de-pagamento) | ⛔ |
| Calcular a nota final ponderada | Local (sem rede) | [M1](#m1-nota-final-ponderada) | ✅ |

## Mapa do Portal

O Portal do Aluno é a página inicial depois do login. Ele não guarda conteúdo próprio: cada ícone leva a um subsistema. O tutorial de agendamento de prova avisa que “a ordem e a disponibilização dos ícones em seu portal pode mudar conforme atualizações institucionais”; por isso o app localiza ícones pelo **texto visível**, nunca por posição.

| # | Ícone no Portal | O que é | Host observado |
|---|---|---|---|
| 1 | **Sistema do Aluno** | Lyceum: boletim, frequência, histórico, Secretaria Virtual, Financeiro | `sis.lyceum.com.br` (`/AOnline3/index.html#/...`) |
| 2 | **Plataforma Virtual** | Canvas: disciplinas, materiais, AVAs, notas das AVAs, fóruns, avisos, Caixa de Entrada | `uva.instructure.com` (materiais também em `ead.uva.br`) |
| 3 | **Sistema de Provas** | SGP/Avalia: provas online, agendamento de prova presencial, provas realizadas | **a validar** (não registrado no acervo) |
| 4 | **Serviços de Requerimento/Solicitações** | Zeev: protocolos, incluindo revisão de prova A2/A3 | **a validar** (não registrado no acervo) |
| 5 | **Plataforma de Pagamento** | Negociação de débitos vencidos e situação financeira | **a validar** (não registrado no acervo) |

Além dos cinco subsistemas, o Portal tem os ícones das bibliotecas virtuais **Minha Biblioteca** e **Biblioteca Virtual Pearson**. São acervos licenciados de terceiros: o app só pode abrir o link na janela do navegador; não baixa nem converte livros.

### Procedimento base: abrir o Portal e entrar num subsistema

| Passo | Tela | Rust |
|---|---|---|
| 1 | Abrir o Portal | ✅ `CanvasSession::open_lyceum` abre `lyceum::PORTAL_START_URL` numa aba própria e confere o host |
| 2 | Login manual | ✅ `CanvasSession::lyceum_status` detecta `/login`/`/sso` (`lyceum::login_required`) e espera |
| 3 | Clicar no ícone pelo texto | 🟡 localizar `a`/`button` cujo texto normalizado seja o nome do ícone; validar o `href` resultante com a allowlist antes de navegar; nunca seguir `/login`, `/sso`, `/autologin` |
| 4 | Conferir o destino | ✅ para Lyceum e Canvas (`sanitize_source_url`); 🟡 para SGP, Zeev e Pagamento, que precisam ter o host registrado e testado primeiro |

Para o Canvas, o app já abre `https://uva.instructure.com/courses` direto (`CanvasSession::launch`), sem passar pelo ícone do Portal.

---

## 1. Sistema do Aluno (Lyceum)

SPA Angular em `https://sis.lyceum.com.br/AOnline3/index.html`. As telas mudam pelo fragmento `#/...`. Ao entrar, a tela inicial é a Secretaria Virtual. Quando há mais de um vínculo (ex.: Ciência da Computação ativo e Engenharia de Computação evadido), o **seletor do cabeçalho** define qual vínculo as telas mostram; toda captura precisa registrar qual vínculo estava selecionado.

**Procedimento Rust atual (✅):** `CanvasSession::refresh_lyceum_catalog` (`browser.rs`) faz uma travessia em largura a partir do Portal, seguindo só links aprovados por `lyceum::is_read_only_navigation`: aceita rótulos acadêmicos (histórico, nota, frequência, boletim, avaliação, rendimento, serviço…) e paginação; recusa ações (solicitar, trancar, cancelar, matricular, inscrever, alterar, excluir, financeiro, boleto, pagamento, protocolo, perfil, sair). Cada página passa por `lyceum::parse_page` (campos e tabelas, rótulos sensíveis omitidos) e `lyceum::persist_capture` grava JSON e Markdown novos em `materias/catalogo/capturas/`. Limites: 50 páginas e 10.000 linhas por tabela; excedeu, a captura fica marcada como parcial.

**Melhoria definida (🟡):** em vez de depender só da travessia genérica, ter um procedimento por tela com a rota `#/...` observada, a espera pelo seletor da tabela e um parser com colunas esperadas. Cada rota abaixo marcada “a validar” deve ser registrada aqui na primeira captura autenticada.

### L1. Boletim (notas e média)

- **Tela:** Sistema do Aluno → **Avaliação** → **Boletim**. Lista as disciplinas do período; clicar no nome da disciplina expande as notas lançadas (A1, A2, A3…). Média final e situação só são atualizadas depois do fechamento do semestre; antes disso a média aparece como “---”.
- **Rota:** a validar.
- **Rust:** ✅ coberto pela travessia genérica (rótulos “avaliação”/“boletim”). 🟡 Procedimento dedicado: abrir a rota, para cada linha clicar no nome via CDP e esperar o detalhe, ler a tabela, gravar `notas/<período>/<disciplina>` com fonte e data. Expandir uma disciplina é só exibição; não altera dados.
- **Observação:** é a fonte oficial de notas. A nota de AVA no Canvas (C4) é a mesma A1 em escala diferente; o Lyceum prevalece.

### L2. Frequência

- **Tela:** Sistema do Aluno → **Frequência**. Mostra faltas e percentual por disciplina e o total.
- **Rota:** a validar.
- **Rust:** ✅ travessia genérica (rótulos “frequência”/“falta”). 🟡 Procedimento dedicado com parser de colunas disciplina / faltas / percentual.

### L3. Histórico do Aluno

- **Tela:** Sistema do Aluno → **Histórico do Aluno** → abas **Carga Horária**, **Disciplinas Pendentes**, **Atividades Complementares**, **Disciplinas Eletivas/Optativas**. Também: **Índice de Rendimentos** e **Disciplinas Matriculadas**.
- **Rota:** a validar.
- **Rust:** ✅ travessia genérica (rótulos “histórico”, “rendimento”, “disciplina”). 🟡 Procedimento dedicado: trocar de aba via clique CDP pelo texto da aba e capturar cada tabela separadamente.

### L4. Secretaria Virtual: serviços e rematrícula

- **Consultar solicitações anteriores:** Secretaria Virtual → **Serviços**. Rota observada: `#/home/servico-list`. ✅ leitura pela travessia genérica (rótulo “serviço”).
- **Ver o catálogo de serviços:** Serviços → **Nova Solicitação** → **Adicionar Serviço**. Mostra serviços como “Histórico Escolar com CR” e o valor. ⛔ manual: o botão “Nova Solicitação” é o início de uma solicitação; a automação para antes dele.
- **Solicitar serviço / Avançar:** ⛔ manual.
- **Rematrícula:** Secretaria Virtual → **Matrícula** → aceitar contrato eletrônico → confirmar disciplinas. Exige cota 01 paga. ⛔ manual.

### L5. Financeiro (dentro do Sistema do Aluno)

- **Tela:** Sistema do Aluno → **Financeiro**: boletos a vencer, declaração para Imposto de Renda, extrato.
- ⛔ manual. A travessia recusa links com “financeiro”, “boleto” e “pagamento” de propósito.

---

## 2. Plataforma Virtual (Canvas)

`https://uva.instructure.com`. Canvas LMS padrão. Para leitura, o procedimento preferido é a **API REST do Canvas** (`/api/v1/...`), chamada a partir da sessão autenticada do Chromium do app. É o padrão já usado em `CanvasSession::list_assignment_events`: `page.evaluate` com `fetch(url, { credentials: "same-origin" })`, paginação pelo cabeçalho `Link: rel="next"`, verificação de que a origem continua `uva.instructure.com`, limite de páginas, e o JSON volta para Rust e é desserializado com `serde`. A alternativa sem JavaScript é `reqwest` com o cabeçalho de cookie obtido por `CanvasSession::cookies_for_host`, como já faz `download::download_pdf`; o cookie fica só em memória.

Os endpoints abaixo são da API pública do Canvas. Só `/api/v1/courses/:id/assignments` já foi exercitado pelo app; os demais estão **a validar** na instância da UVA (uma disciplina pode ocultar abas, e a API pode responder 401/403).

**Tipos de disciplina:** o conteúdo didático aparece em três modelos de layout: **Recursos** (aba “Conteúdo Didático”, unidades com recursos), **E-book** (abas “E-book”, “Vídeos da Disciplina”, “Midiateca”) e **Roteiro de Estudos**. O **Espaço Curso** não é disciplina: é o ambiente de avisos, fóruns da coordenação e tutoriais.

### C1. Listar disciplinas

- **Tela:** **Painel Inicial** (cartões do período atual) ou **Disciplinas** → **Todas as disciplinas** (inclui períodos anteriores).
- **Rust:** ✅ `CanvasSession::list_courses` / `list_all_courses` leem `https://uva.instructure.com/courses` com `canvas::parse_courses` e paginação validada por `canvas::validate_next_course_href`.

### C2. Baixar materiais

- **Tela:** dentro da disciplina, **Módulos** / “Conteúdo Didático” / “E-book” / “Midiateca”.
- **Rust:** ✅
  1. `CanvasSession::load_course_inventory` / `load_course` abrem `/courses/:id` e montam a lista de módulos (`canvas::parse_module_items`, `group_module_items`).
  2. `CanvasSession::read_module_item` abre cada item permitido (`canvas::module_item_url`) e `canvas::discover_material_links` acha links e iframes de PDF.
  3. `download::validate_material_url` aceita só hosts oficiais (Canvas, `ead.uva.br`, S3 assinado) e `download::download_pdf` baixa com cookie em memória, limite de bytes, redirecionamentos e tempo.
  4. `pdf.rs` converte para Markdown por página; o PDF bruto é descartado.
  5. A captura em lote (`canvas_capture.rs`, comandos `start_canvas_capture`/`resume_canvas_capture`) percorre todas as disciplinas, com estado por item em SQLite, cancelamento e retomada.
- **Fora do escopo:** vídeos, e-books das bibliotecas licenciadas, conteúdo com DRM.
- 🟡 **Melhoria:** usar `GET /api/v1/courses/:id/modules?include[]=items&include[]=content_details&per_page=100` para o inventário em vez do HTML, e `GET /api/v1/courses/:id/files?per_page=100` quando a aba Arquivos estiver habilitada.

### C3. Prazos das atividades

- **Tela:** disciplina → **Tarefas**. Cada atividade mostra a data final. Os PDFs “Calendário Acadêmico” e “Critérios Avaliativos” da disciplina completam a informação.
- **Rust:** ✅ `CanvasSession::list_assignment_events` → `GET /api/v1/courses/:id/assignments?per_page=100&override_assignment_dates=true`; o resultado alimenta `calendar.rs`.

### C4. Notas e feedback das AVAs (A1)

- **Tela:** disciplina → **Notas** (menu à esquerda). Mostra a pontuação de cada atividade avaliativa; o ícone de comentário abre o feedback do tutor.
- **No app:** **Currículos** → painel **Notas, fóruns, avisos e Caixa de Entrada do Canvas** → marcar as partes → **Ler agora**. Comando Tauri `refresh_canvas_academic`; a captura sai em `materias/catalogo/capturas/canvas-<id>.json` e `.md` (pasta ignorada pelo Git) e entra no SQLite pela sincronização. Os itens C4 a C7 são lidos na mesma execução, por todas as disciplinas que `/courses` lista (até 40), incluindo o Espaço Curso.
- **Rust (✅):** `CanvasSession::capture_canvas_academic` em `browser.rs`; caminhos, conversões e Markdown em `canvas_api.rs`. Toda requisição passa por `canvas_api::validate_api_path` (só `/api/v1/`, sem host, sem `..`) e por `CanvasSession::canvas_api_json` (`fetch` da própria origem, paginação por `Link`, tempo-limite de 45 s). Falha num item vira aviso e deixa a captura parcial; o restante continua.
  1. `GET /api/v1/courses/:id/assignments?include[]=submission&per_page=100` → por tarefa: `name`, `points_possible`, `due_at`, `submission.score`, `submission.grade`, `submission.workflow_state`, `submission.submitted_at`.
  2. Feedback: `GET /api/v1/courses/:id/assignments/:assignment_id/submissions/self?include[]=submission_comments` → `submission_comments[].comment` e `created_at`. A validar: confirmar na instância que essa leitura não muda o indicador de comentário não lido (`REGISTRO_ACADEMICO.md` registra que os comentários foram deixados fechados de propósito).
  3. Busca o feedback só de atividades com entrega ou correção, até 40 por disciplina.
  4. A captura não altera `dados/notas/`. Passar valores para `canvasAvaTotals` continua sendo revisão humana, com fonte e data, sem substituir a nota do Lyceum.
- **Pedir revisão de AVA:** pela **Caixa de Entrada** do Canvas ao tutor. ⛔ manual.

### C5. Fóruns

Há quatro tipos:

| Fórum | Onde fica | Quem responde |
|---|---|---|
| Fórum de Dúvidas – Fale com o tutor | disciplina → aba **Fóruns** (canto superior esquerdo) | tutor da disciplina, até 48 h úteis |
| Fórum Avaliativo (AVA da A1) | disciplina → **Tarefas** ou **Fóruns** | fecha no fim do prazo |
| Fórum de Interação dos Alunos | página inicial do **Espaço Curso** | colegas |
| Fórum Fale com o Coordenador | página inicial do **Espaço Curso** | coordenação, até 48 h úteis |

- **Rust (✅), leitura:** mesma execução de C4 (`capture_course_forums`).
  1. `GET /api/v1/courses/:id/discussion_topics?per_page=100` → `id`, `title`, `posted_at`, `last_reply_at`, `discussion_subentry_count`, `assignment_id` (preenchido = fórum avaliativo), `locked`.
  2. `GET /api/v1/courses/:id/discussion_topics/:topic_id/view` → árvore completa de mensagens (`view[]`, `replies[]`) e `participants[]` para resolver nomes.
  3. `canvas_api::apply_discussion_view` achata a árvore na ordem da conversa, com profundidade, autor e data; HTML vira texto (`html_to_text`). Limites: 60 fóruns por disciplina e 2.000 mensagens por fórum. Fórum que exige postagem inicial pode recusar `/view`; isso aparece como aviso.
  4. Não abrir a página HTML do tópico para ler; a interface web pode marcar entradas como lidas. A validar: confirmar que `/view` não altera o estado de leitura.
- **Postar ou responder:** ⛔ manual. No fórum avaliativo, a postagem é a entrega da A1.
- **Privacidade:** mensagens de colegas são dados de terceiros. Gravar só no acervo local ignorado pelo Git, ou omitir autores que não sejam tutor/coordenação.

### C6. Avisos

- **Tela:** **Quadro de Avisos** do Espaço Curso (monitoria administrativa) e avisos de cada disciplina. A monitoria também os encaminha para a Caixa de Entrada.
- **Rust (✅):** `GET /api/v1/courses/:id/discussion_topics?only_announcements=true&per_page=50` por disciplina, incluindo o Espaço Curso → `title`, `posted_at`, autor, `message` (HTML → texto). Foi escolhido no lugar de `/api/v1/announcements` porque esse endpoint, sem `start_date`/`end_date`, devolve só as últimas semanas.

### C7. Caixa de Entrada

- **Tela:** **Caixa de Entrada** do Canvas. Canais: Monitoria Administrativa (24 h úteis), Tutor (48 h úteis), Coordenador (48 h úteis).
- **Rust (✅), leitura:** `capture_inbox`, desmarcado por padrão no painel por ter mensagens privadas.
  1. `GET /api/v1/conversations?scope=inbox&per_page=50` → assunto, participantes, `last_message_at`, `workflow_state`.
  2. `GET /api/v1/conversations/:id?auto_mark_as_read=false`. **Obrigatório** passar `auto_mark_as_read=false`: sem ele o Canvas marca a conversa como lida. `validate_api_path` recusa qualquer leitura de conversa sem esse parâmetro, e há teste para isso. Até 100 conversas por captura.
- **Enviar mensagem:** ⛔ manual.

### C8. Enviar AVA, A4 e tarefas

- **Tela:** **Tarefas** → atividade → **Iniciar Tarefa** → **Carregar Arquivo** → **Enviar Tarefa**. Depois do prazo não há novo envio; no fórum avaliativo, a postagem é a entrega.
- ⛔ manual sempre.

---

## 3. Sistema de Provas (SGP / Avalia)

Acesso pelo ícone **Sistema de Provas** no Portal. Menu lateral com **Provas Online**, **Agendar Provas**, **Provas realizadas**, **Avaliação de teste** e **Sair**. A2 é presencial no polo ou online, conforme o calendário da disciplina; A3 é online e substitui a A2 se for maior, ou serve de 2ª chamada.

**Pré-requisito para qualquer automação:** abrir uma vez pela janela do app, registrar aqui o host observado e incluí-lo na allowlist com teste. Até lá o app não navega neste subsistema.

### P1. Provas realizadas (nota e feedback)

- **Tela:** menu lateral → **Provas realizadas**. Depois do período de correção mostra nota e feedback de cada prova.
- **Rust (🟡):** abrir pelo ícone (procedimento base), clicar em “Provas realizadas” pelo texto, esperar a lista, extrair disciplina / prova / nota / data com `scraper`, gravar captura nova. Rota e seletores a validar.

### P2. Consultar agendamento

- **Tela:** **Agendar Provas** mostra as disciplinas, o dia e o horário agendados; **Ver local** mostra o polo.
- **Rust (🟡):** ler a tabela de agendamentos e o conteúdo de “Ver local”. A validar: “Ver local” precisa ser só exibição, sem envio. Os botões **Inscrever**, **Salvar** e **Cancelar Inscrição** nunca são acionados.

### P3. Agendar, fazer ou cancelar prova

- **Agendar:** **Agendar Provas** → **Inscrever** → horário → **Salvar** → **Entendi**. Para reagendar: **Cancelar Inscrição** → **Sim** e agendar de novo. Não agendar conta como falta.
- **Fazer prova:** **Provas Online** → **Acessar avaliação** → **Iniciar Prova Online** → tela cheia → responder → **Conferir minhas respostas** → entregar → **Voltar para a página inicial** → **Sair**.
- **Simulado:** **Avaliação de teste** → **Continuar prova online**.
- ⛔ manual sempre. Prova é atividade pessoal e monitorada; o app não abre essas telas nem automatiza nada nelas.

---

## 4. Serviços de Requerimento/Solicitações (Zeev)

Acesso pelo ícone **Serviços de Requerimento/Solicitações** no Portal, que redireciona ao Zeev. O primeiro acesso usa CPF como login e “Esqueci minha senha” para criar a senha por e-mail. É um login separado do Portal; o app não guarda essa senha.

### Z1. Acompanhar solicitações

- **Tela:** aba **Acompanhar Solicitações**: status de cada protocolo.
- **Rust (🟡):** depois de registrar o host na allowlist, ler a lista (número do protocolo, serviço, data, status) e gravar captura nova. Rota e seletores a validar.

### Z2. Abrir protocolo (incluindo revisão de A2/A3)

- **Revisão de prova:** no Zeev, dentro do período do calendário acadêmico. Um protocolo por questão, com nome completo da disciplina, número da questão e fundamentação. Pedido incompleto é indeferido. Se a prova ainda não foi corrigida, não abrir protocolo.
- ⛔ manual.

---

## 5. Plataforma de Pagamento

Acesso pelo ícone **Plataforma de Pagamento** no Portal: negociação de débitos vencidos e situação financeira. Boletos a vencer, extrato e declaração de IR ficam no **Financeiro** do Sistema do Aluno (L5).

⛔ manual. Dado financeiro não entra no acervo e o app não navega nesta plataforma.

---

## Cálculos locais (sem rede)

### M1. Nota final ponderada

Regra dos critérios oficiais de avaliação da graduação EAD (2024):

- **A1** = média das atividades avaliativas online (fórum avaliativo e/ou trabalho), peso 4. Sem 2ª chamada, não pode ser substituída.
- **A2** = prova regular, peso 6. **A3** = substitutiva da A2 ou 2ª chamada, peso 6; vale a maior entre A2 e A3.
- **A1 > 0:** `NFp = A1 × 0,4 + max(A2, A3) × 0,6`. Aprovado se `NFp ≥ 6,0`.
- **A1 = 0:** reprovado automaticamente. Os critérios mostram `NFp = (A1 × 0,4) + (A2 ou A3 × 0,6)/2` só como cálculo informativo.
- **Disciplinas especiais (estágio, TCC, monografia):** sem A2/A3; `NFp = A1 × 0,4 + A4 × 0,6`, aprovação com `NFp ≥ 7,0`, sem 2ª chamada nem recuperação.
- **Pendência:** o tutorial menciona disciplinas sem A1, mas não traz a fórmula para elas. Não calcular nesse caso; consultar os Critérios Avaliativos da disciplina.

**Rust (✅):** `grading::final_grade` (`app/src-tauri/src/grading.rs`), comando Tauri `calculate_final_grade`, painel **Nota final ponderada** em **Currículos**. A conta usa centésimos inteiros, então a comparação com 6,0 e 7,0 é exata (6,66 com A1 = 5 dá 5,996 e reprova; o app mostra 5,99, nunca 6,00). Também informa a nota mínima necessária na prova que falta (A2/A3 ou A4). Testes cobrem os exemplos do tutorial (10/9/7 → 9,4; 10/4/8 → 8,8; especial 10/9 → 9,4), o limite exato, A1 = 0 e entradas inválidas. O resultado aparece como **estimativa** até o Lyceum publicar a média oficial.

---

## Pendências deste guia

- Registrar o host do Sistema de Provas, do Zeev e da Plataforma de Pagamento na primeira abertura pela janela do app.
- Registrar as rotas `#/...` do Lyceum para Boletim, Frequência e Histórico do Aluno. Só `#/home/servico-list` foi observada até agora.
- Rodar a leitura C4–C7 numa sessão autenticada pelo app. Até agora só há testes offline com JSON de exemplo; nenhuma captura real foi feita.
- Validar na instância UVA os efeitos de leitura: se `submissions/self?include[]=submission_comments` e `discussion_topics/:id/view` mudam algum indicador de “não lido”. Até confirmar, rodar primeiro com uma disciplina antiga.
- O atalho Moodle do Lyceum respondeu HTTP 404 em 9/10/2026; não faz parte da navegação.

## Fontes

Tutoriais oficiais do Espaço Curso — Ciência da Computação (Canvas 53418), extraídos em `fontes-uva/2026.2/espaco-curso-ciencia-da-computacao-2023.1/`:
`Como_consultar_as_notas_no_boletim.md`, `Financeiro_e_Plataforma_de_Pagamento.md`, `Solicitacao_de_Servicos.md`, `Rematrícula_Secretaria_Virtual.md`, `Bibliotecas.md`, `Como_acessar_o_conteudo_das_disciplinas.md`, `Como_consultar_o_prazo_de_cada_atividade_nas_disciplinas.md`, `Como_enviar_uma_atividade_avaliativa_da_A1.md`, `Como_visualizar_a_correcao_das_atividades_avaliativas_da_A1.md`, `Como_realizar_as_atividades_de_A4.md`, `Como_calcular_a_Media_Final_Ponderada.md`, `Como_pedir_revisao_de_provas_A2_A3.md`, `Forum_de_Duvidas_Fale_com_o_tutor.md`, `Forum_de_Interacao_dos_Alunos.md`, `Forum_de_Coordenacao_de_Curso.md`, `Avisos.md`, `Conhecendo_os_canais_de_comunicacao.md`, `Agendamento_Prova_Avalia_v1.md`, `[SGP]Orientacoes_gerais_Prova-Online-ALUNOS.md`, `[SGP]Orientacoes_gerais_Prova_Presencial-ALUNOS.md`.

Critérios de avaliação: `fontes-uva/2026.1/apoio/CRITERIOS_GRADUACAO_AVALIACAO_2024.md`.
Rotas e telas do Lyceum observadas: [cursos/engenharia-de-computacao/README.md](../cursos/engenharia-de-computacao/README.md) (9/10/2026) e [REGISTRO_ACADEMICO.md](../REGISTRO_ACADEMICO.md).
Código: `app/src-tauri/src/browser.rs`, `lyceum.rs`, `canvas.rs`, `canvas_capture.rs`, `download.rs`, `pdf.rs`, `calendar.rs`.
