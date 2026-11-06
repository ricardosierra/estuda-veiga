# Modelo de ameaças

**Revisado em:** 2026-11-06 · **Base:** `HEAD` `e29f27b` + alterações não commitadas. Modelo curto, baseado em leitura do código e da documentação; **não** é auditoria de segurança nem teste de intrusão.

## 1. Ativos

1. Dados pessoais acadêmicos do aluno: matrícula, notas, frequência, trajetória, trabalhos entregues, planejamento (`privado/`, SQLite local, capturas).
2. Sessões autenticadas no Portal e no Canvas (cookies no perfil Chromium do app).
3. Chave da API da Anthropic (arquivo `.env` de outro projeto).
4. Integridade do acervo e das projeções SQLite/Markdown/JSON.
5. Direitos autorais de material de terceiros em `fontes-uva/`.
6. Integridade da conta e dos registros na UVA (o app não deve alterá-los).

## 2. Fronteiras de confiança

- **Máquina do usuário** (confiável): repositório, SQLite, perfil Chromium, `.env`.
- **UVA/Canvas/Portal** (remoto, semiconfiável): conteúdo capturado é dado não confiável.
- **API da Anthropic** (remoto, de terceiro): recebe texto de materiais.
- **Remoto Git futuro** (inexistente hoje): tudo que sobe é público ou compartilhado.
- **Colaboradores** (futuros): conteúdo enviado é não confiável até revisão.

## 3. Ameaças, mitigações e lacunas

| # | Ameaça | Impacto | Mitigação existente | Lacuna |
|---|---|---|---|---|
| T1 | **Vazamento de item proibido ao publicar**: credenciais, bancos SQLite, perfil do Chromium, `originais/` ou RA/CPF/RG em arquivo versionado ou no histórico | Exposição de sessão autenticada, de banco com notas e respostas, de binários de terceiros ou de identificadores mantidos fora | `.gitignore` de originais, bancos e capturas; contrato `publication.rs` (`LocalOnly` nunca liberado); o repositório não tem remoto | Contrato não integrado; sem verificação automática pré-publicação; histórico com 7 commits que tocaram `REGISTRO_ACADEMICO.md` não verificado quanto a itens proibidos |
| T2 | **Dado de terceiro ou identificador proibido em arquivo público por engano** (texto livre, Markdown, nome de docente, e-mail, RA/CPF/RG) | Exposição de terceiros e de identificadores que seguem fora | Validador de `dados/` recusa chaves/valores sensíveis, strings vazias, caminhos absolutos; `--verificar-texto` | Validador não cobre Markdown nem `fontes-uva/`; não detecta nomes próprios; `source.file` de `dados/` aponta para `privado/` |
| T3 | **Segredos em URL** (token, assinatura, SSO) gravados em documentos ou banco | Sequestro de sessão | `sanitize_source_url` (Portal e Canvas) remove query/fragmento/credenciais; `sanitize_capture_message` filtra mensagens; testes cobrem `?token=` | Colagem manual de URLs na documentação depende de disciplina; sem varredura automática de segredos no Git |
| T4 | **Roubo de sessão** via perfil Chromium no disco | Acesso ao Portal/Canvas como o aluno | Perfil separado do navegador pessoal; cookie só enviado ao host inicial em memória; sem armazenamento de senha | Perfil guarda cookies em claro conforme o Chromium; app não aplica permissões nem encerra sessão ao fechar; sem expiração forçada |
| T5 | **Automação executa ação indevida** (enviar, marcar como lido, matricular) | Alteração de registro institucional | Navegação só por links oficiais em modo leitura (`is_read_only_navigation`), `fetch` GET, `validate_api_path`, `auto_mark_as_read=false`, limites de páginas e itens | Filtro por termos é heurístico; sem teste autenticado nem captura de tráfego nesta revisão; mudança de layout do Portal pode alterar o comportamento |
| T6 | **Conteúdo capturado hostil / injeção** (HTML, Markdown, texto com instruções) | XSS na interface, instruções que enganam o modelo | Escape de Markdown, remoção de `<script>` nos textos convertidos, CSP restritiva do Tauri (`default-src 'self'`), `react-markdown` | Sem filtro contra instruções embutidas antes de enviar texto à Anthropic; resumo gerado pode refletir texto malicioso (**Não verificado**) |
| T7 | **Leitura de arquivos fora da raiz** (symlink, `..`, caminho absoluto) | Exposição de arquivos do sistema | `validate_relative_path`, recusa de symlinks, `canonicalize` e checagem de raiz em `sync.rs`, `scanner.rs`, `lyceum.rs`; testes de regressão | Cobertura em macOS/Unix; comportamento em outros sistemas **Não verificado** |
| T8 | **Envio de conteúdo acadêmico a serviço externo** | Perda de controle sobre texto de materiais | Envio só após confirmar hash da lista de fontes; recurso não é automático | Interface afirma "Sem IA, nuvem…" (contradição); sem opt-in por material; chave em arquivo de outro projeto; decisão de política pendente (`docs/features/README.md`) |
| T9 | **Download malicioso** (host falso, redirecionamento, arquivo gigante) | Execução de conteúdo, consumo de recursos, vazamento de cookie | `validate_material_url` (HTTPS, hosts `uva.instructure.com`/`uva.br`/`*.uva.br`, sem usuário, porta 443), sem redirecionamento automático, máx. 4 saltos, 60 MiB, validação de PDF, cookie só no host inicial | PDF é interpretado por `pdf-extract`/`lopdf`; parsers de terceiros podem ter falhas (**Não auditado**) |
| T10 | **Perda ou corrupção por sincronização** (três superfícies SQLite/Markdown/JSON) | Perda de edição do usuário | Reconciliação por base comum; divergência vira conflito para revisão humana; tombstones; `sync_outbox` repete operação interrompida; arquivos de origem não são sobrescritos silenciosamente | Edição manual de tabelas `academic_*` é descartada na próxima projeção (por design); sem backup automático documentado |
| T11 | **Corrupção/adulteração do acervo** (arquivo editado fora do app) | Registro incorreto tratado como oficial | Fonte e data por registro; estados `Confirmed/Estimated/Unknown/Conflicting` em `estuda-domain` | `estuda-domain` não integrado; sem hash por arquivo publicado nem assinatura |
| T12 | **Violação de direitos autorais** ao publicar textos de terceiros | Risco jurídico, retirada | Binários fora do Git; fonte e página registradas | `fontes-uva/` classificado como `Shareable` no código, em conflito com a doc; nenhuma licença declarada |
| T13 | **Material de contribuidor sem consentimento ou com dado de terceiro** | Exposição de terceiros, direitos | Processo de revisão em [../contribuindo/conteudo.md](../contribuindo/conteudo.md) | Processo é documental; sem automação nem modelo de pull request |
| T14 | **Acesso local a `.estuda-uva.sqlite3`** por outro usuário ou processo | Leitura de notas e frequência | Arquivo ignorado pelo Git; permissões do sistema | App não restringe permissões; banco sem criptografia |
| T15 | **Dependência comprometida** (supply chain) | Execução de código | Versões fixadas com `=` para várias crates sensíveis (`chromiumoxide`, `pdf-extract`, `zip` etc. em `app/crates/estuda-core/Cargo.toml`); `Cargo.lock` e `package-lock.json` versionados | Sem `cargo audit`/`npm audit` documentado nem executado nesta revisão |
| T16 | **Execução de teste com navegador real** altera estado local | Efeitos colaterais no ambiente | Teste usa perfil temporário e modo headless (`launches_real_isolated_chromium_and_reads_fixture_dom`) | Teste exige `/opt/homebrew/bin/chromium` fixo e falha (`panic!`) se ausente |

## 4. Prioridades sugeridas

Ordem proposta, sem implementação:

1. **T1/T2/T12 antes de qualquer remoto:** integrar a verificação de camadas a um comando de pré-publicação, verificar o histórico quanto a itens proibidos e decidir sobre `fontes-uva/`.
2. **T8:** alinhar a interface ao comportamento real (aviso de envio externo) e decidir a política de IA (opt-in por material); tirar a chave de arquivo de outro projeto.
3. **T4/T14:** documentar e, se aprovado, aplicar permissões restritivas ao diretório de dados.
4. **T15:** executar auditoria de dependências e registrar o resultado.
5. **T5:** teste autenticado supervisionado em ambiente controlado, com captura de tráfego, para comprovar somente leitura.

## 5. O que não está coberto

Segurança do macOS, do Chromium e do WebView; criptografia em repouso; análise de tráfego de rede; auditoria dos parsers de PDF/DOCX/Pages (`pdf-extract`, `docx-lite`, `iwork`, `zip`); ameaças específicas da futura sincronização entre pares e do aplicativo GPUI (ainda stub).
