# Estado de implementação do Estuda UVA

**Atualizado em:** 10 de outubro de 2026
**Repositório:** local, branch `main`, sem remoto configurado.

Este documento separa o que está implementado e verificado do que ainda depende de acesso autenticado ou trabalho adicional. O planejamento acadêmico geral está em [PLANEJAMENTO.md](../PLANEJAMENTO.md); o passo a passo para usar o app está em [app/README.md](../app/README.md).

## Implementado no código local

- O app Tauri/React lista, lê e pesquisa documentos textuais do acervo, com extração para PDF, DOCX, Pages, HTML, JSON e formatos de texto compatíveis. O leitor do catálogo preserva os originais e aplica limites de tamanho, caminho e profundidade.
- O sincronizador concilia documentos Markdown e JSON com o SQLite do acervo, cria projeções Markdown/JSON, mantém journal para operações pendentes e apresenta conflitos sem descartar silenciosamente as versões divergentes.
- `sync_documents` é a camada de conteúdo bidirecional: edições reconciliadas no SQLite saem para os arquivos e as edições nos arquivos retornam ao SQLite. As tabelas `academic_*` são uma projeção relacional do catálogo e são reconstruídas dos JSON validados durante a reconciliação; alterações SQL manuais nessas tabelas de projeção não são uma via de edição e serão substituídas pela próxima atualização do catálogo.
- O monitor observa as pastas acadêmicas locais. O comando de sincronização manual continua disponível; a pasta `.estuda-uva/mirrors/` não volta a ser tratada como fonte.
- O catálogo curricular é carregado do SQLite depois de reconciliar as fontes JSON. Além dos documentos completos, o banco mantém uma projeção relacional com instituições, programas, currículos, disciplinas canônicas, ofertas, relações, trajetórias, matrículas e resultados; cada entidade projetada preserva seu `source_json`.
- Ciência da Computação e Engenharia de Computação permanecem em trajetórias separadas. O compartilhamento exige referências explícitas dos dois currículos ao mesmo identificador canônico; relações candidatas continuam sem equivalência confirmada e não transferem nota, matrícula ou conclusão entre trajetórias.
- O app pode abrir uma janela Chromium própria para Portal/Lyceum, solicitar login manual, navegar apenas por links acadêmicos oficiais observados e limitar cada leitura a 50 páginas e 10.000 linhas por tabela. Rotas `/login`, `/sso` e `/autologin` são bloqueadas mesmo quando o link parece acadêmico. Se uma tabela exceder o limite, a captura traz aviso e fica marcada como parcial.
- Uma captura do Portal gera arquivos novos JSON e Markdown em `materias/catalogo/capturas/` e os sincroniza no SQLite. Links gravados têm query e fragmento removidos; campos identificados por rótulos de nome/aluno/estudante, CPF, RA, matrícula, identificação ou documento são omitidos.
- No Canvas, o app lista cursos e módulos e importa um material escolhido. PDFs de material são temporários; o resultado da importação é Markdown por página e metadados de origem seguros.
- Antes de baixar, a importação procura a URL de origem sem query no SQLite e confirma que o Markdown e a aula ainda existem dentro do acervo. Se os dois arquivos locais estão presentes, reutiliza-os sem transferir o PDF novamente e atualiza o período no índice.
- O apresentador de aula e o histórico registram sessões locais de estudo para retomada. Eles não alteram presença, notas, matrículas ou entregas oficiais.

## Fluxo operacional

### Canvas: material textual

1. Abrir o app e entrar em **Canvas e importação**.
2. Conectar a janela Chromium do app e autenticar manualmente, se solicitado.
3. Atualizar a lista de cursos, escolher um curso e abrir o módulo ou item publicado.
4. Para um PDF, iniciar a importação do item. O app extrai e grava o texto por página em Markdown, remove a query/fragmento da URL registrada e descarta os bytes temporários do PDF.
5. Conferir a aula no acervo, acrescentar notas autorais e sincronizar o acervo.

A importação atual é por item escolhido. A travessia integral de todos os cursos, módulos, páginas, vídeos, avaliações e anexos, incluindo cursos futuros e históricos, ainda não foi executada como uma captura completa nesta etapa. Materiais inacessíveis devem continuar identificados por URL/erro; não se registra que foram baixados quando não foram.

### Portal do Aluno/Lyceum: histórico oficial

1. Na área **Acadêmico**, abrir a janela do Lyceum.
2. Digitar credenciais apenas na janela do portal e concluir o MFA manualmente. A senha não é lida pelo app nem entra no acervo.
3. Selecionar **Capturar histórico e notas**.
4. Conferir se o resultado informa captura completa ou parcial e quais seções ficaram indisponíveis.
5. Inspecionar o par JSON/Markdown em `materias/catalogo/capturas/` e o estado de sincronização. Cada execução usa nomes novos; nenhum arquivo anterior é substituído.

O código da captura está conectado, mas uma captura automatizada completa do histórico não foi executada nesta verificação. Em uma leitura manual autenticada de **Frequência**, em 10/10/2026, as sete disciplinas atuais apareceram com 0 faltas e 100% cada, total de 0 faltas e 100,0%; esse dado pontual foi anotado em `REGISTRO_ACADEMICO.md` e `materias/catalogo/trajetorias.json`. Nenhuma nota nova nem cobertura completa do Lyceum é afirmada.

**Privacidade do acesso:** um redirecionamento SSO observado durante a consulta colocou parâmetros de autenticação na URL. Não registrar, copiar para documentos nem compartilhar URLs desse tipo. Para as próximas capturas, abrir o Lyceum pela janela do app e entrar manualmente, sem reutilizar links de SSO com credenciais embutidas.

### Git e cópias locais

- Markdown, JSON de catálogo e texto acadêmico podem ser revisados no Git local.
- O SQLite, WAL/SHM, perfil do Chromium, credenciais, PDFs temporários e builds ficam ignorados pelo Git.
- A tela **Dados e sincronização** mostra o estado do Git sem fazer stage, commit ou push.
- Os arquivos originais acadêmicos permanecem no local atual; a catalogação não os move.

## Validações executadas nesta atualização

| Comando | Resultado | O que comprova |
|---|---|---|
| `cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check` | Passou | Formatação Rust. |
| `cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib` | 73 testes passaram, 0 falharam | Testes locais de leitores, sincronização nos sentidos Markdown/JSON/SQLite, projeção relacional, migração v3→v4, captura Lyceum, bloqueio de login/SSO/autologin, remoção de credenciais da rota SPA e importação de PDF. |
| `cargo build --offline --manifest-path app/src-tauri/Cargo.toml --bin estuda-uva` | Passou, com avisos de código ainda não usado | Compilação do binário Tauri da revisão atual. |
| `npm test --prefix app` | 8 testes passaram, 0 falharam | Associação explícita de disciplinas entre currículos, resposta a comandos locais e divisão de material em seções de aula. |
| `npm run build --prefix app` | Passou | TypeScript e bundle de produção do frontend. |
| `npm run tauri --prefix app -- build` | Passou; bundle `.app` (27,71 MiB) e DMG (8,51 MiB) gerados | Binário Rust otimizado, bundle macOS e instalador local; assinatura ad hoc, sem notarização. |
| Conferência pós-sincronização do SQLite | `REGISTRO_ACADEMICO.md`, `trajetorias.json` e este documento coincidem com seus registros SQLite; projeções Markdown/JSON presentes; 0 conflitos e 0 operações pendentes; frequência na projeção relacional datada de 10/10/2026 | Estado efetivo das três representações após a reconciliação do acervo. |
| Consulta SQLite em modo somente leitura | `user_version=4`; 10 tabelas do catálogo normalizado; os 5 JSON acadêmicos no SQLite têm o mesmo conteúdo semântico dos arquivos do acervo | Estado do banco local existente; a consulta não alterou o banco. |
| `cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check` e `git diff --check` | Passaram | Formatação e ausência de whitespace inválido nas alterações rastreadas. |

Não foi feita publicação, push nem instalação em `/Applications`. A versão atual foi empacotada; a tentativa de ler sua árvore de acessibilidade expirou, então o fluxo visual desta cópia não está confirmado. A janela aberta em `/Applications` continua sendo uma versão antiga e não conta como prova do build atual. A leitura manual autenticada confirmou apenas o snapshot pontual de frequência descrito acima; ela não substitui uma captura completa de histórico e notas.

## Limites que permanecem explícitos

- Uma sessão de estudo, importação de texto ou abertura de página não conta como presença. A frequência é a que a UVA exibe oficialmente.
- O acesso ao Portal depende do login humano na janela visível e pode terminar em resultado parcial.
- PDFs sem camada textual podem precisar de OCR; o texto extraído não contém diagramas, capturas de tela ou elementos visuais.
- Não há sincronização com remoto Git; o repositório permanece local para revisão antes de qualquer publicação.
