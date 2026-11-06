# Prompt para o GPT — Estuda UVA (RAM, SQLite, internet, atividades, P4 e provas)

Você vai me ajudar a escrever um prompt para um agente de código (Claude Code) trabalhar no meu projeto. Leia todo o contexto abaixo antes de responder. Se algo não estiver aqui, pergunte; não invente dados acadêmicos.

## 1. O projeto
- Pasta local "Faculdade" (repositório git local, branch main, sem remoto). Guarda meu acervo acadêmico da UVA em Markdown, JSON e SQLite, com notas de estudo, materiais e um app desktop chamado Estuda UVA.
- Estuda UVA: Tauri 2 + React 19 + Vite 8 + TypeScript no frontend; backend em Rust com rusqlite 0.37 (SQLite embutido), chromiumoxide 0.9.1 (Chromium via CDP, sem Playwright), reqwest com rustls e notify para observar pastas.
- Telas principais: Acadêmico, Matérias, Histórico, Estudo pessoal, Dados e sincronização, Acervo.
- Banco: .estuda-uva.sqlite3 na raiz do acervo, PRAGMA user_version = 4 (última verificação documentada, 15/10/2026). As tabelas academic_* são projeção reconstruída a partir dos JSON. Há journal de operações pendentes e tabela de conflitos.
- Espelhos Markdown/JSON gerados ficam em .estuda-uva/mirrors/ e não são fonte.

## 2. Regras que o projeto obriga (AGENTS.md)
- Preserve arquivos acadêmicos originais: não sobrescrever nem mover durante a catalogação.
- Notas, frequência, documentos e credenciais são dados privados. Nunca gravar senha, cookie, token, CPF, nem URL com parâmetros de sessão ou SSO em Markdown, JSON, logs ou Git.
- Mantenha SQLite, Markdown e JSON sincronizados; preserve conflitos para revisão humana.
- Separe dado confirmado por fonte oficial (com fonte e data) de estimativa, material didático e campo desconhecido.
- Trava de banco (regra pessoal): qualquer alteração no SQLite (DDL, migração, INSERT, UPDATE, DELETE, backfill) exige minha aprovação explícita antes. Mostre o SQL, a justificativa e o impacto, e espere. Vale mesmo em modo automático.
- Git: nada de commit ou push sem eu pedir; nenhum trailer Co-Authored-By.
- Há muitas alterações não commitadas no working tree. Não reverta nada que eu não tenha pedido.

## 3. Estado técnico atual (documentado em 14–15/10/2026; confira no código)
- Memória: o scanner mantém um índice em memória (CatalogIndex dentro de Mutex). O catálogo curricular é carregado do SQLite depois de reconciliar os JSON. PDFs são extraídos só em memória e descartados. Não existe uma política formal "RAM primeiro" com descarga em lote para o SQLite; isso precisa ser desenhado.
- Internet: a captura do Canvas é sequencial, um curso ou item por vez, retomável e cancelável. Antes de baixar, o app procura a URL de origem no SQLite e reaproveita o arquivo local se ele existir. O Portal/Lyceum tem limite de 50 páginas e 10.000 linhas por tabela. As rotas /login, /sso e /autologin são bloqueadas. Há allowlist de hosts UVA e validação do PDF pela assinatura %PDF-. Validadores ETag e Last-Modified estão previstos no plano da varredura; confirme se já estão no código.
- Testes da última verificação: cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib (73 passaram); npm test --prefix app (8 passaram); npm run build --prefix app; cargo fmt --check.

## 4. Meu acadêmico (sem dados sensíveis)
- Duas matrículas no Portal: Engenharia de Computação (situação Evadido) e Ciência da Computação (situação Ativo). Use só a de Ciência da Computação. As trajetórias são separadas e não devem ser misturadas.
- Período atual: 2026.2. O Portal lista 7 matrículas: Cálculo de Uma Variável; Bases de Programação; Linguagens Formais e Autômatos; Paradigmas de Linguagens de Programação; Estatística para Computação; Projeto de Extensão I – Ciência da Computação; Projeto de Extensão I (turma 21CIV2262A, que o Canvas mostra como Engenharia Civil; confirmar).
- No Canvas, só 5 cursos estão favoritados. Bases e Paradigmas aparecem em "Inscrições anteriores" mesmo sendo de 2026.2. Não deduza cancelamento pela posição na lista.
- O Canvas lista "Paradigmas de Linguagens de Programação", mas os arquivos do módulo são de ASI1. Não trate como certo sem confirmação.
- "Espaço Curso — Ciência da Computação" (curso Canvas 53418) é espaço de apoio, não matéria.
- A frequência oficial é a do Portal. Não inferir presença.

## 5. Vocabulário de avaliações
- AVA 1 e AVA 2: atividades avaliativas online, entregues no Canvas, com prazo.
- No registro de 2026.2: Avaliação 1; Avaliação 2 (prova regular); Avaliação 3 (prova substituta). No calendário de 2026.1 aparecem A2 e A3 como provas, com agendamento no Sistema de Provas.
- "P4" não aparece como avaliação em nenhum documento. A única ocorrência está dentro do texto de um e-book. Não suponha o significado: trate como pergunta em aberto.
- O calendário oficial de 2027.1 ainda não havia sido publicado em outubro de 2026.

## 6. Navegação (o Portal já está aberto no navegador)
1. Portal: https://portalaluno.uva.br/Aluno/PortalAluno. Eu digito CPF, senha e MFA manualmente. O agente não digita, não lê e não armazena credenciais.
2. Na janela "Matrículas", escolha a linha com situação Ativo (Ciência da Computação, a segunda da lista). Não escolha Engenharia de Computação.
3. Depois do login, o Portal mostra estes sistemas: Sistema Acadêmico; Sistema Virtual de Ensino (provavelmente o Canvas, citado nos documentos pelo botão "Plataforma Virtual - CANVAS"); Sistema de Provas; Sistema de Solicitações; Calendário Acadêmico; Universo UVA; Carreiras UVA.
4. Leitura: Sistema Acadêmico (notas, frequência, histórico), Canvas (AVA 1, AVA 2, prazos e notas de atividades), Sistema de Provas (datas e situação de cada prova) e Calendário Acadêmico (datas oficiais). Universo UVA, Carreiras UVA e Sistema de Solicitações só se houver informação de prova ou prazo. Não abra solicitações.
5. SOMENTE LEITURA. Não agendar prova, não confirmar nada, não enviar atividade, não solicitar nada, não marcar presença e não alterar matrícula.
6. Se uma URL tiver token ou parâmetro de SSO, descarte-a e não a grave.

## 7. O que eu quero
A) Memória RAM primeiro:
- Leitura a partir do estado em memória antes de disco ou SQLite. O SQLite serve para persistir e para recarregar na inicialização.
- Escrita em lote, em transação, sem uma consulta por item dentro de laço.
- Invalidação por hash ou data de modificação quando o arquivo mudar. Limite de memória definido.
- Nenhum PDF bruto, cookie ou token guardado em memória além da requisição.

B) Persistência SQLite:
- Migração versionada por PRAGMA user_version, idempotente, com backup antes de aplicar.
- Primeiro proponha o DDL ou a migração e espere minha aprovação. Teste em cópia do banco. Não aplique no banco real sem eu aprovar.

C) Eficiência de internet:
- Reaproveitar material local antes de qualquer download.
- Requisições condicionais (ETag, Last-Modified) quando o servidor suportar.
- Um item por vez, com timeout, limites e cancelamento. Sem paralelismo agressivo.
- Não baixar vídeos ou anexos fora do escopo. Para A a F, ler só as páginas de atividades, notas, provas e calendário.
- Registrar no relatório quantas requisições foram feitas e quantas foram evitadas.

D) Atividades 1 e 2: para cada disciplina de 2026.2, o status de AVA 1 e AVA 2 (prazo, entregue ou não, nota se exibida), com fonte e data da leitura. Campo sem fonte fica como "desconhecido".

E) Matérias só com P4: para cada disciplina, liste os tipos de avaliação que aparecem no Portal e no Canvas. Marque as que têm apenas P4. Se P4 não aparecer nas fontes, escreva "não encontrado". Não suponha.

F) Datas de provas: disciplina, data, horário, local ou polo, status (agendada, não agendada, realizada), fonte e data da leitura. Sem data oficial, escreva "a confirmar".

G) Saída: Markdown, JSON e SQLite consistentes, conflitos preservados e um relatório com o que foi verificado e o que não foi.

## 8. Sua tarefa
1. Se precisar, faça até 3 perguntas, e só as que bloqueiam. Exemplos: o significado de P4; se a prova aparece no Sistema de Provas ou no Calendário. Se não precisar, vá direto ao passo 2.
2. Escreva UM prompt final, em português, pronto para eu colar no Claude Code. Ele deve:
   - implementar A a G em fases;
   - indicar os arquivos prováveis (app/src-tauri/src/..., app/src/ui/...);
   - ter critérios de aceite e testes focados;
   - incluir a trava de banco e o modo somente leitura no Portal;
   - pedir ao final um relatório do que foi e do que não foi verificado.
3. No prompt final, não coloque matrícula, CPF, notas ou senha. Use [placeholder] quando precisar.
4. Não invente disciplinas, datas, notas nem regras de avaliação.
