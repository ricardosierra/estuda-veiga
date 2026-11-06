# Especificação: Estuda UVA — captura automática de aulas do Canvas

**Data:** 14 de outubro de 2026
**Repositório:** `/Users/sierra/Documents/Faculdade`
**Aplicativo:** Estuda UVA
**Estado:** especificação aprovada pela usuária em conversa; plano de implementação aguardando revisão
**Base:** complementa `2026-10-08-estuda-uva-design.md` e o fluxo por matérias de 10 de outubro.

## Entendimento aprovado

A estudante quer que o Estuda UVA percorra as matérias no Canvas, abra o conteúdo publicado das aulas e mantenha uma cópia textual local dos materiais disponíveis, sem depender de a IA ou a estudante abrir cada item manualmente. A captura deve informar o que foi concluído, o que falhou e o que pode ser retomado.

O registro do app representa leitura/estudo local. Frequência oficial, conclusão de módulos, notas, entregas e outras atividades do Canvas não serão alteradas nem simuladas.

## Causa atual

O backend consegue listar os cursos e os módulos de um curso selecionado. A interface pede a seleção manual do curso e, depois, de cada item. A importação aceita um PDF por chamada. Portanto, a navegação em lote ainda não existe. A implementação atual usa `chromiumoxide` em Rust por CDP; não usa Playwright nem depende de IA para extrair o texto.

## Objetivo

Adicionar uma captura em lote, iniciada na interface do Estuda UVA, que percorre cursos atuais e anteriores visíveis na sessão autenticada, seus módulos publicados e seus itens de conteúdo. O app captura texto de páginas de aula do Canvas e converte PDFs acessíveis em Markdown por página. O processo é local, observável, cancelável, retomável e idempotente.

O catálogo local segue disponível sem autenticação. Uma captura do Canvas só começa com uma sessão autenticada manualmente na janela Chromium do app.

## Decisões de arquitetura

### Navegação e sessão

- Reutilizar a janela Chromium visível, o perfil isolado e a automação Rust/CDP já existentes. Não introduzir runtime Playwright/Node no aplicativo instalado.
- O login, SSO, MFA e CAPTCHA permanecem sob controle da estudante na janela Chromium. O app não lê nem preenche credenciais.
- Usar somente navegação e requisições de leitura dentro da sessão existente. Não guardar tokens, cookies, URLs assinadas ou parâmetros sensíveis.
- Enumerar IDs e links dinamicamente da conta autenticada; não fixar IDs de cursos ou aulas.
- Percorrer inscrições atuais e anteriores que o Canvas apresentar. Incluir conteúdos futuros somente quando já estiverem visíveis e publicados. Não inferir cursos ou aulas que a sessão não revelou.
- Se a sessão expirar, pausar a execução, manter o progresso confirmado e pedir novo login manual antes de retomar.

### Escopo de conteúdo

- Percorrer cada curso elegível, seus módulos publicados e os itens ordenados de cada módulo.
- Abrir páginas de aula do Canvas em modo de leitura e converter o conteúdo textual e sua estrutura básica em Markdown, incluindo título, curso, módulo, endereço oficial sanitizado e data de captura.
- Localizar PDFs ligados ou incorporados em módulos e páginas de aula, baixar bytes somente para uma área temporária, validar origem, redirecionamentos, tipo e assinatura do arquivo, extrair texto por página e descartar o PDF temporário ao final.
- Deduplicar itens repetidos por identificador Canvas e URL de origem canônica; guardar as associações com todos os módulos em que o mesmo conteúdo aparece.
- Registrar itens que não são material textual ou PDF como ignorados com motivo e link seguro, sem tentar executá-los.
- Não baixar vídeos, executar scripts remotos ou produzir transcrição/OCR. Quando um item depender de vídeo, conteúdo externo não textual, permissão ausente ou PDF sem texto selecionável, registrar o limite e manter a referência segura para consulta.
- Não percorrer nem abrir provas, quizzes, entregas, submissões, fóruns, notas, presença, listas de alunos, configurações ou rotas de alteração de matrícula.

### Frequência e atividades oficiais

- Não enviar, marcar ou simular presença oficial, conclusão de aula/módulo, visualização obrigatória ou qualquer outra atividade que altere o estado acadêmico remoto.
- Abrir conteúdo para capturá-lo pode gerar registros comuns de acesso no Canvas, mas isso não equivale a presença e o app não afirmará que equivale.
- O histórico do Estuda UVA registra apenas captura e estudo local, com estados claramente separados dos dados oficiais do Portal/Lyceum.

### Execução em lote e retomada

- Disponibilizar uma ação explícita para capturar todas as matérias listadas pelo Canvas e uma ação para retomar a última captura incompleta.
- Mostrar progresso por curso, módulo e item, com totais de encontrados, importados, reutilizados, ignorados e com erro.
- Processar em série (concorrência inicial 1) para preservar estabilidade da sessão e limitar a carga no Canvas. Não paralelizar navegação entre itens na mesma janela.
- Persistir no SQLite local o identificador da captura, escopo, curso/módulo/item atual, estado, tentativas, contadores, timestamps e erro sanitizado. Segredos, cookies e URLs assinadas nunca fazem parte do estado persistido.
- Permitir cancelar. A ação interrompe o item em andamento antes de gravar um resultado parcial como concluído; resultados já confirmados permanecem disponíveis.
- Ao retomar, pular materiais já importados e íntegros, continuar itens pendentes/falhos quando solicitados e não duplicar arquivos nem sessões de estudo.
- A captura de um item com erro não interrompe os demais. Erros de sessão, permissão global ou indisponibilidade da conta pausam o lote e preservam estado.
- Respeitar limites configuráveis para cursos, itens, tamanho de arquivo, redirecionamentos e tempo por item. Ao atingir um limite, marcar a captura como parcial e indicar o ponto de retomada.

### Persistência, acervo e conflitos

- Salvar textos de páginas e PDFs convertidos na pasta de matéria/período já usada pelo acervo, com fonte, curso, módulo, identificador de origem quando disponível, data da captura, páginas e estado de extração.
- Manter os arquivos originais acadêmicos existentes em seus locais. O PDF de origem remoto é temporário; nenhum PDF bruto novo será adicionado ao Git.
- Reutilizar o índice de materiais e a sincronização existentes entre arquivos Markdown/JSON aplicáveis e SQLite. O estado operacional da captura permanece local no SQLite e não inclui dados de sessão.
- Se a URL já estiver indexada e os arquivos estiverem íntegros, reutilizar o material existente. Se o conteúdo remoto mudou ou o Markdown foi editado localmente, preservar a versão local e registrar conflito/necessidade de revisão; nunca sobrescrever silenciosamente.
- Não executar `git add`, commit, push ou publicação. O estado do Git continua apenas informativo no app.

### Interface

- Em Canvas e importação, adicionar **Capturar todas as matérias** e **Retomar captura**.
- Exibir estado de autenticação, curso/módulo/item atual, progresso agregado, resultado por disciplina e último horário de atualização.
- Oferecer cancelamento e retomada; apresentar erros por curso/item com ação de tentar novamente.
- Distinguir **capturado**, **já disponível localmente**, **ignorado com motivo**, **falhou**, **aguardando login**, **cancelado** e **parcial por limite**.
- A página da matéria deve mostrar os conteúdos textuais capturados na lista normal de aulas e materiais, com indicação de fonte e data.
- A UI não deve chamar IA para navegar, escolher links, extrair texto, classificar presença ou resumir conteúdo. A navegação e a extração são determinísticas no backend local.

## Segurança e privacidade

- Conservar a política existente de HTTPS, validação de origem em cada redirecionamento, hosts UVA autorizados e cookies limitados ao host de origem em memória.
- Retirar query e fragmento de URLs persistidas; nunca incluir cookies, credenciais, tokens, links assinados ou conteúdo pessoal nos erros/logs/arquivos.
- Tratar HTML, títulos, links e texto de aula como conteúdo não confiável. Não executar JavaScript recebido nem HTML ativo; converter apenas estrutura permitida para Markdown.
- Limpar arquivos temporários em sucesso, erro e cancelamento. Em falha inesperada, removê-los na recuperação da próxima inicialização antes de permitir retomada.
- Preservar a leitura local e a sincronização de catálogo existentes mesmo se Canvas estiver desconectado.

## Fora de escopo

- Marcar ou forçar frequência oficial, registrar presença em nome da estudante ou contornar exigências de comparecimento.
- Alterar conclusão de módulos, notas, matrícula, entregas, respostas de avaliações, fóruns ou dados do Portal/Lyceum.
- Capturar vídeo, executar conteúdo externo, usar OCR ou IA para completar partes que não foram extraídas.
- Guardar PDF bruto, senha, cookies, token ou URL assinada em arquivos versionados ou dados sincronizados.
- Capturar cursos não listados pela conta, conteúdos indisponíveis para a sessão ou atividades ocultas/privadas.

## Critérios de aceite

1. Após login manual, uma ação percorre todos os cursos atuais e anteriores que a conta autenticada expõe, sem selecionar cada curso/item manualmente.
2. Em cada curso, a captura percorre os módulos publicados e os itens na ordem apresentada, incluindo páginas de aula e links/embeds de PDF.
3. Conteúdo de páginas e PDFs com texto selecionável gera Markdown legível e pesquisável com fonte, curso, módulo, data e referência de página para PDFs.
4. Cursos/itens não visíveis, sem acesso ou de formatos não suportados aparecem como cobertura desconhecida, erro ou ignorado com motivo; nunca como baixados.
5. Uma repetição não duplica material íntegro. Conteúdo alterado e edições locais são preservados para revisão, sem sobrescrita automática.
6. Cancelar e retomar continua do último item confirmado, preserva os resultados prontos e não cria duplicatas.
7. Expiração de sessão pausa o lote, solicita autenticação manual e retoma sem guardar dados de autenticação.
8. Itens de avaliação, fórum, entrega, presença, notas e perfil não são abertos nem alterados.
9. Nenhuma chamada a IA, serviço externo de processamento, Playwright ou runtime Node é necessária para capturar os materiais no app instalado.
10. Progresso e erros são visíveis no app; estado local do lote não contém segredos e pode ser reconciliado com o acervo Markdown/SQLite.
11. As verificações cobrem fixtures HTML anonimizadas/sintéticas, paginação e deduplicação, limites/cancelamento/retomada, conteúdo PDF válido/inválido, sanitização de URL/erro e sincronização do material.
12. Uma validação autenticada final percorre uma amostra real de curso com consentimento manual na janela Chromium e confirma que a captura não altera atividades oficiais.

## Riscos e respostas

| Risco | Resposta |
|---|---|
| Canvas altera o DOM ou paginação | Usar seletores/rotas semânticas, testes com fixtures e estado parcial explícito quando a página não for reconhecida. |
| Curso tem muitos itens | Captura em série, progresso, limites, cancelamento e retomada persistida. |
| Sessão expira durante o lote | Pausar antes da próxima requisição, sanitizar o estado e pedir login manual. |
| Mesmo material aparece em mais de um curso/módulo | Identificar a fonte canônica e manter todas as associações sem duplicar texto. |
| Conteúdo local foi editado após captura | Comparar hash e preservar ambas as versões para revisão humana. |
| Página contém scripts, mídia ou texto visual | Não executar scripts; registrar o que não pôde ser representado em texto, sem alegar captura integral. |
| Abrir páginas aparenta contar como presença | Explicar na UI e documentação que acesso não é presença e não afeta o registro oficial. |

## Documentação operacional a atualizar na implementação

- `app/README.md`: passo a passo do login manual, captura de todas as matérias, pausa/retomada, limites de texto e estados de erro.
- `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`: cobertura real, diferenças entre planejado e verificado, data e resultado da validação autenticada.
- Inventário de materiais da matéria/período: fonte, data, estado de captura, contagem de páginas e motivos de pendência.
- Testes e fixtures de parser: somente HTML e PDFs sintéticos/anonimizados, sem credenciais ou conteúdo pessoal.

## Referências locais

- `app/src-tauri/src/browser.rs`: sessão Chromium/CDP, listagem de cursos, módulos e abertura de item.
- `app/src-tauri/src/canvas.rs`: parsing de cursos, módulos e links de PDF.
- `app/src-tauri/src/lib.rs`: comandos Tauri de listagem e importação de um item.
- `app/src/ui/App.tsx`: seleção manual de curso/itens e ação de importação individual.
- `app/src-tauri/src/storage.rs`: índice SQLite de materiais e persistência de estudo.
- `app/README.md` e `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`: instruções e limitações atualmente documentadas.
