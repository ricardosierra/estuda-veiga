# Especificação: Estuda UVA — acervo e aulas locais

**Data:** 8 de outubro de 2026
**Repositório:** `/Users/sierra/Documents/Faculdade`
**Aplicativo:** `Estuda UVA`
**Decisão do usuário:** automação em Rust, sem Playwright.

## Objetivo

Criar um aplicativo desktop Tauri que, depois de o estudante entrar manualmente no Canvas UVA em um navegador Chromium visível, encontre disciplinas e módulos sem depender de IDs de turma fixos, localize materiais PDF vinculados ou incorporados nas unidades, converta o texto dos PDFs para Markdown e monte uma aula guiada local, sem LLM, API de IA ou custo por uso.

As conversões e aulas serão salvas no acervo selecionado pelo estudante, por disciplina. O PDF bruto existirá só em memória/arquivo temporário durante a conversão e será removido após sucesso. O repositório manterá Markdown autoral/derivado e referências oficiais; não voltará a versionar cópias de PDFs de aula.

## Evidência da navegação real

Em 8 de outubro de 2026, a lista `https://uva.instructure.com/courses` mostrou seções distintas para disciplinas publicadas atuais e inscrições anteriores. A página inicial de uma disciplina exibiu `Módulos do curso`, módulos nomeados e itens de tipos diferentes (URL externa, fórum e tarefa). O detalhe de um item de e-book abriu em `/courses/{course_id}/modules/items/{module_item_id}` e continha um `iframe` para um PDF oficial em `ead.uva.br`. A página Arquivos da mesma disciplina mostrou apenas `course_image`, sem listar esse e-book. Portanto, a coleta deve percorrer itens dos módulos e inspecionar os embeds/redirects de cada item; uma busca somente na área Arquivos deixaria material para trás.

## Resultado para o estudante

1. O usuário abre Estuda UVA e conecta o Canvas.
2. Um Chromium separado e visível abre o Canvas com um perfil local persistente. No primeiro uso, o próprio estudante realiza login, SSO/MFA ou eventual CAPTCHA nessa janela. O app não pede, lê, grava nem registra senha.
3. O app identifica a sessão autenticada e lista disciplinas que o Canvas realmente mostra, separando atuais e anteriores, com período, status de publicação e quantidade de módulos.
4. O usuário escolhe uma disciplina e uma unidade/módulo, revisa os materiais encontrados e inicia a importação. Tarefas, notas, fóruns e itens que não sejam material não são abertos nem alterados automaticamente.
5. O app segue itens de módulo, arquivos do curso e páginas HTML relevantes. Reconhece PDFs por URL, tipo de conteúdo e assinatura `%PDF-`, incluindo PDF dentro de `iframe`/`embed` e redirecionamentos.
6. O texto é extraído por página em Rust e gravado em Markdown com metadados, URL de origem, título, data, páginas e indicação de extração incompleta. Não persiste o PDF bruto após conversão bem-sucedida.
7. Uma aula local é montada a partir do texto, preservando referência de página em cada seção e exercício.
8. O usuário estuda, responde perguntas e vê acertos e progresso calculados pelo backend Rust.

## Arquitetura aprovada

### Desktop

- Tauri v2 estável; primeira entrega voltada ao macOS arm64 disponível no computador do estudante.
- Interface React + TypeScript + Vite, executada no WebView do Tauri. O frontend apresenta telas e envia intenções tipadas ao backend. Não calcula pontuações, porcentagens, duração, estatísticas de leitura, resumo ou conteúdo da aula.
- Backend Rust concentra estado, regras de domínio, rede, sistema de arquivos, automação, extração PDF, persistência, geração de aula e qualquer cálculo.
- Node/npm são usados somente para instalar e compilar o frontend durante desenvolvimento/build; não fazem parte do runtime de automação nem da aplicação distribuída.

### Navegação Canvas

- `chromiumoxide` em Rust controla Chromium/Chrome via Chrome DevTools Protocol (CDP), em janela visível.
- O app procura Chromium/Chrome nas localizações usuais e permite selecionar o executável se a detecção falhar. Não baixa um navegador silenciosamente. A instalação de Chromium, se necessária, é uma ação explícita do usuário.
- Usar um perfil exclusivo do Estuda UVA no diretório de dados do aplicativo; não tentar abrir ou copiar o perfil pessoal do Chrome.
- A sessão persiste localmente para evitar novo login a cada abertura. O usuário pode encerrar sessão no Canvas ou remover o perfil nas configurações, com confirmação na ação de exclusão.
- Navegar inicialmente para `https://uva.instructure.com/courses`. Detectar login/SSO/expiração e esperar o usuário completar autenticação no próprio Chromium. Nunca preencher credenciais, automatizar ou contornar MFA/CAPTCHA.
- Ler HTML e DOM por CDP e analisar a estrutura em Rust. Não usar automação por coordenadas. Preferir links visíveis e atributos semânticos; interpretar rotas Canvas (`/courses/{id}`, `/modules/items/{id}`) dinamicamente. IDs observados durante descoberta não podem ser fixados no código.
- Domínios padrão autorizados para navegação de material: `uva.instructure.com`, `ead.uva.br` e hosts sob `uva.br`. Outros hosts são exibidos ao usuário e não são baixados automaticamente.
- O app não executa operações de escrita no Canvas. Materiais, mensagens, avaliações e páginas remotas são conteúdo não confiável; o app só usa os links/textos como dados.

### Download autenticado e segurança

- O download é iniciado pelo backend Rust a partir de URL observada no browser. `https` obrigatório; redirecionamentos limitados; cada salto revalida o host. Não enviar cookies de um host para outro.
- Cookies existentes podem ser lidos do contexto CDP e usados somente em memória para uma requisição ao próprio host de origem quando o PDF exigir autenticação. Valores não entram em logs, banco de dados, arquivos `.md`, eventos de frontend ou telemetria.
- Validar MIME, assinatura PDF, tamanho máximo configurável e nome de arquivo sanitizado. Evitar traversal, URLs `file:`, `javascript:`, protocolos não HTTP e downloads automáticos de conteúdo fora dos domínios autorizados.
- Limitar concorrência e tamanho; cancelamento encerra requisições e remove temporários. Itens bloqueados, expirados ou não PDF aparecem com erro por item, sem interromper outras conversões.
- Sem analytics, upload, telemetria, chamadas LLM ou serviço remoto de IA.

### Extração e acervo Markdown

- Usar `pdf-extract` em Rust para extrair páginas, mantendo os limites de página. A representação Markdown será conservadora: título/metadados, origem oficial, `## Página N` e texto da página, com separadores e ordem de leitura observada.
- Não inventar títulos, texto, tabelas ou explicações ausentes. Se a extração for vazia ou muito curta, criar status explicativo em vez de uma transcrição falsa; informar que o arquivo pode ser escaneado/protegido e que OCR ainda não foi executado.
- Guardar um `.md` por PDF dentro da pasta da disciplina, em `materias/<slug>/materiais/`, além de `AULA.md` por disciplina e um índice `README.md`. Nomes gerados passam por slug seguro e colisões são resolvidas sem sobrescrever silenciosamente.
- Tratar texto, links, nomes e metadados vindos de páginas/PDFs como conteúdo não confiável; renderizar Markdown com HTML bruto desabilitado e nunca executar scripts ou links automaticamente.
- Se um material já existir, comparar URL final e hash do conteúdo; apresentar opções para manter, atualizar ou gravar uma nova versão. Nunca descartar uma edição manual do estudante.
- Os documentos podem ser versionados no repositório quando o usuário seleciona este acervo como destino. PDFs originais não são staged pelo app.

### Aula sem IA

Todo o mecanismo é determinístico e local. Não usa rede para gerar explicações.

- Aula inclui objetivo detectado no texto (quando presente), mapa do material, seções em ordem, conceitos/definições extraídos com página, exemplos existentes, roteiro de estudo, questões de recuperação e gabarito apontando para a fonte.
- Detectar títulos pela estrutura textual e padrões tipográficos/textuais disponíveis; detectar definições por padrões linguísticos em português; usar frases existentes e paráfrases somente quando uma regra explícita puder manter a ligação com trechos da fonte.
- Se não houver sinal suficiente para pergunta/resposta confiável, não gerar item especulativo. Marcar lacunas e oferecer leitura guiada do texto original.
- Apresentar a aula como roteiro determinístico baseado no PDF, não como uma explicação gerada por professor/IA.
- O usuário pode responder às perguntas na interface; Rust confere respostas exatas/normalizadas, calcula pontuação e registra progresso. Toda soma, proporção, duração estimada, contagem, seleção de termos e pontuação ocorre em Rust.

## Dados locais

- Perfil separado do Chromium: `{app_data_dir}/chromium-profile/`.
- Banco SQLite: cursos, metadados de materiais, estado de importação e progresso de estudo. Sem notas oficiais, RA, senha ou cookies no banco.
- Temporários e PDFs durante conversão: área temporária por tarefa com remoção em sucesso, cancelamento ou reabertura após falha; não dentro do histórico Git.
- Saídas `.md`: pasta de acervo indicada pelo usuário; padrão deste uso é a pasta `materias/` do repositório da Faculdade.
- Configurações: URI base do Canvas UVA (fixa na primeira entrega), executável Chromium escolhido, diretório do acervo e preferências locais.

## Estados e erros visíveis

- Chromium ausente, versão incompatível ou perfil bloqueado.
- Precisa entrar/SSO/MFA; sessão expirada.
- Canvas indisponível ou HTML mudou; link de curso não publicado.
- Lista sem disciplinas; curso sem módulos; material sem link.
- Item que é tarefa/fórum, não material.
- Host fora da lista; redirect externo; resposta que não é PDF.
- PDF protegido, corrompido, grande demais, sem camada de texto ou parcialmente extraído.
- Caminho sem permissão, colisão com arquivo manual, disco cheio ou cancelamento.
- Aula sem seções/perguntas confiáveis.

Cada estado contém uma mensagem acionável, o passo alcançado e, quando possível, botão de tentar novamente; os segredos não aparecem na mensagem.

## Fora de escopo da primeira entrega

- Playwright ou runtime Node no app instalado.
- API de IA, conta de provedor, modelo remoto ou custos de tokens.
- Alterar notas, enviar entregas, publicar conteúdo, postar em fóruns ou mudar inscrição no Canvas.
- OCR automático/IA de PDF digitalizado; o app detectará essa limitação explicitamente.
- Importação indiscriminada de todos os cursos sem seleção do usuário.
- Aplicativo móvel ou sincronização em nuvem.

## Critérios de aceite

1. Aplicativo `.app` Tauri abre no macOS arm64; frontend não precisa de Node instalado no runtime.
2. Chromium separado abre com perfil persistente e deixa o login manual no controle do estudante; senhas/cookies nunca aparecem nos arquivos rastreados ou logs.
3. Com a sessão autenticada, o app lista disciplinas e navega a um curso novo sem course ID codificado.
4. O app percorre módulos e classifica tipos de item; encontra PDF incorporado em iframe e PDF de arquivo Canvas, não só a área `Arquivos`.
5. Um PDF de texto é convertido em `.md` UTF-8 com cada página, origem, metadados e texto conferível. Temporário bruto é removido em sucesso.
6. Um PDF escaneado/bloqueado recebe estado explícito; nenhuma aula é inventada a partir de conteúdo que não foi extraído.
7. A aula é criada sem API/rede de IA, contém rastreio de página e exercícios verificáveis derivados das fontes; a pontuação de estudo é calculada no Rust.
8. O app permite cancelar e recupera importações falhas sem duplicar, sobrescrever ou deixar PDFs temporários.
9. Testes cobrem parser, conversão, geração determinística e seleção/segurança de URLs; build Tauri passa; fluxo real de Canvas é percorrido em Chromium com autenticação manual do usuário.
10. `git diff --check` passa; não há PDFs de materiais de aula versionados; app, especificação, plano, documentação, testes e arquivos Markdown da demonstração são commitados localmente.

## Riscos e respostas

| Risco | Resposta |
|---|---|
| Alterações de layout Canvas | Seletores semânticos/rotas, análise de estado, erros por etapa, fixture HTML e fluxo real de regressão |
| PDF em iframe externo | Enumerar `iframe`/`embed` em módulo-item, validar host e formato; não depender da lista de arquivos |
| Sessão/mfa | Chromium visível e autenticação manual; app não lê credenciais |
| Saída PDF para texto de ordem imperfeita | Extrair por página e não simular layout; manter fonte e link para conferência |
| PDF escaneado | Detectar baixa densidade; não gerar explicação; deixar status “precisa de OCR” |
| Perfil Chromium aberto em paralelo | Perfil exclusivo, lock/erro explícito, encerramento limpo e opção reconectar |
| Conteúdo educacional raso sem IA | Usar citações, conceitos, questões de recuperação e lacunas transparentes; não alegar paráfrase livre |

## Referências técnicas consultadas

- [Tauri v2](https://v2.tauri.app/): usa WebView do sistema e backend Rust; CLI/Cargo consultados em 2.12.1 estável.
- [chromiumoxide 0.9.1](https://docs.rs/chromiumoxide/0.9.1/chromiumoxide/): automação Rust por CDP; suporta Chromium visível, perfil `user_data_dir`, páginas, frames, DOM/JS e cookies.
- [pdf-extract 0.12.1](https://docs.rs/pdf-extract/0.12.1/pdf_extract/): extração de texto/PDF por página em Rust.
- [Canvas LMS API documentation](https://canvas.instructure.com/doc/api/): referência de conceitos de cursos, módulos e arquivos; primeira entrega navega a interface pela decisão do usuário, sem exigir token/API pessoal.
