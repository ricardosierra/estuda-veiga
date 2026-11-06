# Limites e segurança das automações

**Revisado em:** 2026-11-06. Este documento consolida os limites numéricos e as barreiras de código de cada automação. A política de privacidade, o tratamento de credenciais, o modelo de ameaças e a publicação estão em outra pasta e **não são repetidos aqui**:

- [`../seguranca/credenciais-e-sessoes.md`](../seguranca/credenciais-e-sessoes.md)
- [`../seguranca/privacidade.md`](../seguranca/privacidade.md)
- [`../seguranca/ameacas.md`](../seguranca/ameacas.md)
- [`../seguranca/publicacao.md`](../seguranca/publicacao.md)

## Limites por integração

| Integração | Limite | Valor | Código |
|---|---|---|---|
| Lyceum | Páginas por captura | 50 | `lyceum::MAX_PORTAL_PAGES` |
| Lyceum | Linhas por tabela / tabelas por página | 10.000 / 100 | `lyceum.rs` |
| Lyceum | Links examinados por página | 2.000 | `parse_page` |
| Canvas (materiais) | Páginas da lista de disciplinas | 100 (faixa 1 a 100) | `CanvasCaptureLimits` |
| Canvas (materiais) | Disciplinas | 200 (faixa 1 a 500) | `CanvasCaptureLimits` |
| Canvas (materiais) | Itens | 10.000 (faixa 1 a 20.000) | `CanvasCaptureLimits` |
| Canvas (materiais) | Tamanho do PDF | 60 MiB | `MAX_PDF_BYTES` |
| Canvas (materiais) | Redirecionamentos | 4 | `MAX_REDIRECTS` |
| Canvas (materiais) | Tempo do download | 45 s (faixa 5 a 120) | `DownloadLimits` |
| Canvas (API) | Disciplinas | 40 | `canvas_api::MAX_COURSES` |
| Canvas (API) | Páginas por listagem | 10 | `MAX_LIST_PAGES` |
| Canvas (API) | Consultas de feedback por disciplina | 40 | `MAX_COMMENT_LOOKUPS_PER_COURSE` |
| Canvas (API) | Tópicos por disciplina / respostas por tópico | 60 / 2.000 | `MAX_TOPICS_PER_COURSE`, `MAX_ENTRIES_PER_TOPIC` |
| Canvas (API) | Conversas lidas por inteiro | 100 | `MAX_CONVERSATIONS` |
| Canvas (API) | Texto por entrada | 20.000 caracteres | `MAX_TEXT_CHARS` |
| Canvas (API) | Tempo de cada consulta | 45 s | `canvas_api_json` |
| Prazos | Páginas de tarefas por disciplina | 20 | script em `list_assignment_events` |
| Chromium | Abertura / fechamento | 20 s / 5 s por etapa | `browser.rs` |
| Leitor do acervo | Tamanho por arquivo | 64 MB | `MAX_DOCUMENT_BYTES` |
| Leitor do acervo | Arquivos / profundidade / texto indexado | 10.000 / 16 / 128 MB | `catalog/scanner.rs` |
| Capturas gravadas | JSON ou Markdown | 64 MB | `persist_capture` |
| Resumos com IA | Entrada / saída / tempo | 2 MiB / 8.000 tokens / 120 s | `study_summaries.rs`, `anthropic.rs` |

Não há limitador de taxa (requisições por segundo) nem pausa entre itens: a captura é **sequencial**, uma página ou item de cada vez, e depende do tempo de resposta do servidor. Isso mantém a carga baixa, mas não é um limite explícito. Também não há política de repetição com recuo; uma falha vira `failed` e o estudante decide repetir.

## Barreiras de código

| Barreira | O que impede | Código | Teste |
|---|---|---|---|
| Allowlist do Portal | Sair de `portalaluno.uva.br`, `sis.lyceum.com.br`, `*.uva.br`; HTTP; userinfo | `lyceum::sanitize_source_url` | `lyceum.rs` |
| Rotas de autenticação | Seguir `/login`, `/sso`, `/autologin` | `lyceum::is_authentication_route` | `lyceum.rs` |
| Navegação só de leitura | Seguir links de solicitar, trancar, cancelar, matricular, inscrever, alterar, editar, excluir, financeiro, boleto, pagamento, perfil, sair, protocolo | `lyceum::is_read_only_navigation` | `lyceum.rs` |
| Query reduzida | Levar token ou parâmetro temporário para o histórico ou para a URL gravada | `lyceum::navigation_target` | `lyceum.rs` |
| Allowlist de materiais | Baixar de host fora do Canvas/UVA/S3 assinado; `http:`; `file:`; porta diferente de 443 | `download::validate_material_url` | `download.rs` (`material_urls_require_https_and_uva_host_boundaries`) |
| Redirecionamento | Perder a validação do host a cada salto; vazar cookie para outro host | `download::download_pdf` | `download.rs` |
| Tipo de arquivo | Gravar algo que não é PDF | `download::valid_pdf_response` (`%PDF-`, `Content-Type`) | `download.rs` |
| Caminhos de API | Qualquer caminho fora de `/api/v1/`; `..`, `//`, caracteres não previstos | `canvas_api::validate_api_path` | `canvas_api.rs` |
| Marcar como lido | Abrir conversa sem `auto_mark_as_read=false` | `validate_api_path`, `conversation_path` | `canvas_api.rs` |
| Escopo da captura de materiais | Tocar em `/assignments/`, `/discussion_topics/`, `/quizzes/`, `/submissions/`, `/gradebook`, `/grades` | `canvas::parse_module_items`, `module_item_url` | `canvas.rs` |
| Gravação sem sobrescrever | Substituir captura anterior ou edição local; seguir symlink para fora do acervo | `lyceum::write_new_file_beneath`, `archive::write_new_or_equal` | `lyceum.rs`, `lib.rs` |
| Identificadores omitidos | Gravar nome, CPF, RA, matrícula, documento nas capturas do Lyceum | `lyceum::is_sensitive_label` | `lyceum.rs` |
| URL limpa | Gravar query, fragmento ou userinfo | `canvas::sanitize_source_url`, `pdf::pages_to_markdown`, `archive::normalize_origin` | `lib.rs`, `pdf.rs` |
| Confirmação de envio | Enviar a uma API externa conteúdo que mudou depois da confirmação | `generate_subject_summaries` (`confirmed_source_hash`) | `study_summaries.rs` |

## Riscos da automação de navegador

1. **O Chromium é um navegador completo, visível e logado.** Tudo que a sessão alcança, o código tecnicamente poderia alcançar. A segurança vem das barreiras acima, não de uma sandbox; uma falha numa delas é uma falha de segurança, não de usabilidade.
2. **Execução de JavaScript na página.** `canvas_api_json` e `list_assignment_events` executam `fetch` via `page.evaluate`. O script é montado a partir de caminhos validados (`validate_api_path`) e IDs numéricos, mas o do calendário usa um caminho literal sem essa validação.
3. **Cookies da sessão.** `cookies_for_host` lê cookies do Chromium via CDP para o download. Ficam só em memória, mas são lidos pelo processo do app; um erro de log poderia expô-los. O código monta mensagens de erro fixas (`BrowserError`, `DownloadError`) que não ecoam cookie nem URL com query.
4. **Perfil do Chromium persistente.** O perfil dedicado (`chromium-profile`) guarda a sessão entre execuções e fica dentro do diretório de dados do aplicativo (`app_data_dir()` do Tauri, fora do repositório), não na pasta do acervo. As entradas `/app/src-tauri/chromium-profile/` e `/app/src-tauri/.estuda-uva-data/` do `.gitignore` são proteção extra para um perfil criado dentro do repositório.
5. **Rastro no sistema de origem.** O acesso pode aparecer no histórico do Canvas e nos registros do Portal. Não marca presença, conclusão, nota, entrega, fórum ou matrícula, mas existe.
6. **Terceiros.** A Caixa de Entrada e os fóruns contêm texto de outras pessoas (docentes e colegas). A captura grava esse texto na máquina do estudante. A política do dono permite versionar conteúdo em `privado/`, mas texto de terceiros (e a ausência de consentimento deles) pede revisão humana antes de publicar; ver [`../seguranca/publicacao.md`](../seguranca/publicacao.md).
7. **Mudança do site.** Seletores HTML (`.context_module`, `dl/dt/dd`, tabelas) podem quebrar sem aviso; o efeito esperado é captura parcial ou vazia, com avisos, e não escrita errada.

## Limites de ação acadêmica

O app **não** executa nenhuma das ações abaixo. Elas só aparecem no guia como passos **manuais** do estudante:

| Ação | Onde acontece | Automação |
|---|---|---|
| Entregar AVA, A4 ou tarefa | Canvas, Tarefas | Nenhuma (C8 é manual) |
| Postar em fórum ou responder mensagem | Canvas | Nenhuma: só leitura |
| Agendar, reagendar ou cancelar prova; fazer prova | Sistema de Provas (SGP) | Nenhuma: o app nem navega nesse host |
| Abrir protocolo ou pedir revisão de prova | Zeev | Nenhuma |
| Matrícula, rematrícula, trancamento | Lyceum | Nenhuma: os rótulos são bloqueados |
| Financeiro e pagamento | Lyceum e plataforma de pagamento | Nenhuma: bloqueado |
| Marcar presença ou frequência | Portal | Nenhuma: sessões de estudo do app **não** contam como presença |

## Dados enviados para fora do computador

Uma única automação transmite conteúdo: [resumos-anthropic.md](resumos-anthropic.md). As demais enviam apenas requisições de leitura aos sistemas da UVA, sem conteúdo do estudante além da sessão autenticada.

## Lacunas e riscos conhecidos

1. **Caminho das capturas.** Hoje o **código** grava todas as capturas (Lyceum e Canvas) em `materias/catalogo/capturas/`, caminho fixo (`lyceum::persist_capture`, `canvas_api::persist_capture`), ignorado pelo Git no `.gitignore`. A política de publicação do dono (`AGENTS.md`, 6/11/2026) permite versionar notas, frequência, matrículas e trabalhos, separados do catálogo, em `privado/`. Alinhar caminho e ignore à política é **decisão pendente**. `crates/estuda-domain/src/publication.rs` ainda classifica `materias/catalogo/capturas/` como privado (nunca publicável), o que também precisa ser revisto. Continuam vedados, sem exceção: senhas, cookies, URLs assinadas, tokens, `.env`, `*.sqlite3*`, perfil do Chromium e `originais/`; RA, CPF e RG seguem fora até decisão do dono.
2. **Escopo do leitor.** O escaneamento do acervo (`catalog/scanner.rs`) e a sincronização conhecem as pastas `docs`, `materias`, `originais`, `fontes-uva`, `periodos` e `cursos`; `cursos/` foi removida da raiz na árvore de trabalho e `privado/` não consta. Efeito sobre o que o app indexa: **não verificado**; ver [`../arquitetura/`](../arquitetura/README.md) e [`../dados/sincronizacao.md`](../dados/sincronizacao.md).
3. **Heurísticas por palavra.** `is_read_only_navigation` e `is_sensitive_label` são listas de palavras. Podem deixar passar (ou bloquear) casos não previstos.
4. **Calendário sem `validate_api_path`** (ver acima).
5. **Nenhuma execução autenticada comprovada.** Os limites e as barreiras estão em código e em testes com HTML e servidores simulados. Cobertura real de cursos, notas e histórico **não foi verificada**.
6. **Revisão de material publicável e atualização da biblioteca** não têm código. Ver [visao-geral.md](visao-geral.md).
7. **Resumos com IA** contradizem o texto "Sem IA, nuvem" da interface. Ver [resumos-anthropic.md](resumos-anthropic.md).
