# Credenciais, sessões e automação de navegador

**Revisado em:** 2026-11-06 · **Base:** `HEAD` `e29f27b` + alterações não commitadas (o código de `anthropic.rs` mudou durante a revisão; os detalhes abaixo refletem a leitura de 2026-11-06).

Rótulos: **Implementado**, **Parcial**, **Planejado**, **Não verificado**. Nenhum `.env` foi lido para esta página; nenhum valor de segredo aparece aqui.

## 1. Regra geral

Senhas, códigos MFA, cookies, tokens e chaves de API **não entram** em Markdown, JSON, SQLite versionado, logs nem Git (`AGENTS.md`). O repositório não contém credenciais (`.gitignore` bloqueia `.env`, `.env.*`, exceto `.env.example`). A validação de `dados/` recusa chaves como `senha`, `cookie`, `token`, `secret` (ver [privacidade.md](privacidade.md) §3).

## 2. Login no Portal (Lyceum) e no Canvas: manual (Implementado)

- O app abre uma janela **Chromium visível** (`CanvasSession::launch` usa `.with_head()`, `app/crates/estuda-core/src/browser.rs`). O usuário digita CPF/senha e conclui o MFA **dentro dessa janela**.
- O app **não solicita, lê nem grava** senha ou código MFA: não há campo de senha na interface nem código que preencha formulário de login. Isso foi conferido por leitura; **Não verificado** em execução autenticada, pois nenhuma captura autenticada foi exercitada nesta revisão.
- Rotas `/login`, `/sso` e `/autologin` são bloqueadas como alvo de navegação automática (`lyceum::is_authentication_route`, usada por `navigation_target` e `is_read_only_navigation`).
- Regra operacional: abrir o Lyceum pela janela do app e entrar manualmente; **não reutilizar nem colar links de SSO** com parâmetros de autenticação (nota em `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`).

## 3. Perfil do navegador e cookies

| Item | Onde | Fato | Estado |
|---|---|---|---|
| Perfil Chromium do app | `<app_data_dir>/chromium-profile` (`app/src-tauri/src/lib.rs`, `connect_canvas` e `open_lyceum`) | Perfil **separado** do navegador pessoal. Fica fora do repositório, no diretório de dados do app (macOS, pelo identificador Tauri `br.edu.uva.estuda`, normalmente em `~/Library/Application Support/`). | Implementado; caminho exato **Não verificado** |
| Cookies de sessão no disco | dentro do perfil Chromium | O próprio Chromium persiste cookies de sessão no perfil. O app não controla isso. Quem tem acesso à conta de usuário do macOS tem acesso ao perfil. | Limitação conhecida |
| Cookies lidos pelo app | memória | `CanvasSession::cookies_for_host` lê cookies via CDP, filtra pelo domínio pedido, descarta cookies expirados e nomes/valores com `;`, `=`, CR, LF, e devolve uma string que fica **em memória** | Implementado |
| Cookie em download | cabeçalho `Cookie` | `download::download_pdf` só envia o cookie ao **host inicial**, e para de enviar se um redirecionamento sair desse host (`crossed_host`); `reqwest` com `redirect::Policy::none()`, seguindo redirecionamentos manualmente (máx. 4) | Implementado |
| Persistência | SQLite, Markdown, JSON | Nenhuma coluna de `app/crates/estuda-core/src/storage.rs` guarda cookie ou token; URLs de origem são gravadas sem query e fragmento | Implementado (por leitura do esquema) |
| `.gitignore` | `/app/src-tauri/chromium-profile/` | Entrada legada para perfil dentro do repositório; o perfil atual fica em `app_data_dir` | Redundante |

O botão de cancelamento interrompe o download antes de gravar o material (`app/README.md`; `download::DownloadError::Cancelled`).

## 4. Chave da API da Anthropic (Parcial, atenção)

Fatos (leitura de `app/crates/estuda-core/src/anthropic.rs` e `study_summaries.rs`):

- `AnthropicConfig::read_default()` lê o arquivo **`$HOME/Dev/Projetos/gerenciador-de-conteudo/.env`**, que pertence a **outro projeto** do mesmo usuário. O caminho está **fixo no código**.
- O analisador lê apenas `ANTHROPIC_API_KEY`, `ANTHROPIC_MODEL` (padrão `claude-haiku-5-5`) e `ANTHROPIC_WORKSPACE_ID` (opcional, enviado no cabeçalho `anthropic-workspace-id`). Ignora as demais linhas.
- A chamada acontece em `study_summaries::generate_subject_summaries`, só depois de o usuário confirmar o hash da lista de fontes. O destino é `https://api.anthropic.com/v1/messages`, com os cabeçalhos `x-api-key` e `anthropic-version`.
- Erros de configuração são textos fixos ("Defina uma ANTHROPIC_API_KEY não vazia no arquivo .env."); testes (`missing_or_empty_api_key_returns_redacted_error`, `malformed_env_quotes_return_redacted_error`) verificam que o valor não aparece na mensagem.
- `AnthropicConfig` não deriva `Debug`, o que evita imprimir a chave por engano.

Riscos e lacunas:

1. **Acoplamento a arquivo de outro projeto.** O app depende da existência e do conteúdo de um `.env` fora do acervo. Se esse arquivo mudar de lugar, o recurso falha; se for lido por ferramentas com acesso ao diretório pessoal, o acesso à chave é o do usuário.
2. **Sem armazenamento seguro.** Não há uso do Keychain do macOS. A chave fica em arquivo de texto (permissão `0600` registrada em `docs/features/README.md`, não verificada aqui).
3. **Chave sem escopo de workspace** produzia `HTTP 400` em ping de 2026-11-06 (`docs/features/README.md`); o suporte a `ANTHROPIC_WORKSPACE_ID` trata isso, mas **Não verificado** contra a API real nesta revisão (testes usam servidor simulado).
4. **Conteúdo sai da máquina.** O texto dos materiais é enviado a um serviço externo (ver [privacidade.md](privacidade.md) §5).
5. **Planejado (não existe):** guardar a chave no Keychain, ler de variável de ambiente ou de configuração do app, e permitir desativar o recurso.

## 5. Riscos da automação de navegador

| Risco | Detalhe | Mitigação existente | Lacuna |
|---|---|---|---|
| Ação não intencional no Portal/Canvas | Navegação automática poderia clicar em botão que altera registro | Só segue links visíveis que `is_read_only_navigation` aceita (exclui, por exemplo, "matricular") e rotas oficiais; Canvas lido por `fetch` GET `credentials: "same-origin"` (`browser.rs` ~linhas 396 e 437) e por caminhos validados por `canvas_api::validate_api_path` | Lista de termos proibidos é heurística; **Não verificado** contra todas as telas do Portal |
| Sessão do usuário reutilizada | A automação roda com a sessão logada | Perfil separado; captura só começa por ação do usuário; limites de páginas (Portal 50; Canvas: 40 cursos, 10 páginas de lista, 40 consultas de comentário por curso, 60 tópicos por curso, 2.000 entradas por tópico, 100 conversas) | Sessão expirada gera captura parcial; o usuário precisa entrar de novo |
| Marcar mensagem como lida | Abrir conversa no Canvas poderia alterar estado | `validate_api_path` exige `auto_mark_as_read=false` em leitura de conversa | Cabeçalho "Caixa de Entrada" vem desmarcado por padrão na interface |
| Rastro de acesso | O acesso aparece no histórico comum do Canvas | Documentado em `app/README.md` e `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md` | Não evitável por design |
| Conteúdo capturado hostil (injeção) | HTML/JSON do Canvas pode conter instruções ou marcação maliciosa | Texto escapado ao virar Markdown (`escape_markdown_text`, remoção de `<script>` coberta pelo teste `markdown_preserves_page_origins_and_removes_query_secrets`); conteúdo capturado é tratado como dado; interface usa `react-markdown` | Se o texto capturado for enviado a um modelo (resumos), ele pode conter instruções que influenciem a saída; nenhum filtro específico existe. **Não verificado** |
| Chromium instalado | O app localiza Chromium/Chrome em caminhos fixos (`/opt/homebrew/bin/chromium`, `/Applications/...`) ou pelo caminho escolhido pelo usuário (`set_chromium_path`) | — | Executável escolhido pelo usuário só é validado como arquivo existente (`set_chromium_path`); o app não verifica que seja de fato Chromium/Chrome |

## 6. Limites de ações acadêmicas

O app **não envia atividades, não agenda provas, não responde fóruns, não envia mensagens e não modifica registros institucionais**. Fundamentos:

- O código do Canvas só faz leituras (`fetch` sem método, que é GET; `canvas_api::validate_api_path` aceita só caminhos `/api/v1/…` de leitura, sem `//` nem `..`, com conjunto restrito de caracteres).
- O download usa só `client.get(...)` (`download.rs`), com HTTPS, host oficial, sem usuário, porta somente 443, limite de 60 MiB e no máximo 4 redirecionamentos.
- O Portal é lido por navegação a links acadêmicos oficiais em modo leitura; rotas de autenticação e links de ação são recusados.
- Abrir material ou registrar sessão de estudo no app grava só no SQLite local e não altera presença, notas, matrícula ou entrega (`docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`).
- A nota final ponderada e a nota mínima são **estimativas locais** (`grading.rs`), não oficiais.
- A agenda lê atribuições publicadas e prazos; não cria eventos na UVA (`docs/NAVEGACAO_PORTAL_UVA.md`, `app/README.md`).

**Limitação:** estas garantias vêm da leitura do código e de testes unitários. Não houve auditoria do tráfego de rede real nem teste autenticado nesta revisão (**Não verificado**).

## 7. O que fazer em caso de exposição

1. Revogar a credencial exposta (trocar senha da UVA; revogar e recriar a chave da Anthropic no console).
2. Encerrar a sessão no Chromium do app e apagar o diretório `chromium-profile`, se houver dúvida sobre cookies.
3. Se o segredo foi commitado: tratá-lo como comprometido mesmo após remoção; reescrever histórico só com decisão do mantenedor (ver [privacidade.md](privacidade.md) §6).
4. Registrar o incidente sem copiar o valor do segredo.
