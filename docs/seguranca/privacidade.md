# Privacidade e separação de dados

**Revisado em:** 2026-11-06 · **Base:** `HEAD` `e29f27b` + alterações não commitadas na árvore de trabalho.

Rótulos: **Implementado**, **Parcial**, **Planejado**, **Não verificado**. Cada afirmação técnica cita o código que a fundamenta.

## 1. Princípio

O que identifica o aluno (matrícula, notas, frequência, trajetória, trabalhos entregues, planejamento) são **dados pessoais separados em `privado/`**. Pela decisão do dono de 6/11/2026, esses dados podem ir ao acervo público e ao Git, desde que estejam em `privado/`. A separação é de pasta, não de sigilo. Continuam proibidos os itens da §7 (credenciais, bancos, perfil de navegador, originais). O que é matéria e instituição é candidato a público. `AGENTS.md` fixa a regra: dados pessoais ficam em `privado/`; `dados/` nunca contém dado pessoal.

## 2. Mapa de separação (Parcial)

| Local | Conteúdo | Git | Observação |
|---|---|---|---|
| `privado/` | registro acadêmico, planejamento, cursos, `dados/` pessoais, matérias com trabalhos, períodos | **versionado** (88 arquivos rastreados) | Separação é por **pasta**, não por repositório. Publicar `privado/` é permitido pela decisão de 6/11/2026. |
| `originais/` | documentos completos dos trabalhos, binários | ignorado (`/originais/`) | Só local. |
| `.estuda-uva.sqlite3` (+ `-wal`, `-shm`, `-journal`) | banco privado do app | ignorado | Ver [../dados/banco-privado.md](../dados/banco-privado.md). |
| `.estuda-uva/mirrors/`, `.estuda-uva/sync-tmp/` | espelhos do sincronizador | ignorado | Podem conter cópia de documentos sincronizados, inclusive de áreas privadas, se o escaneador as incluir. |
| `materias/catalogo/capturas/` | capturas autenticadas do Portal e do Canvas | ignorado pelo `.gitignore` | Dado pessoal (camada `Personal`), fora de `privado/` (ver [visão geral](../dados/visao-geral.md), L9). Podem conter notas e frequência. |
| `materias/catalogo/*.json`, `build/` | saída do gerador (`Generated`) | ignorado | Não é fonte. `trajetorias.json` deriva de dado pessoal; a fonte é `privado/dados/trajetorias/`. |
| `dados/` | catálogo (disciplinas, ofertas, relações, programas, período, instituição) | versionado | Exige revisão antes de publicar; ver §3. |
| `fontes-uva/`, `periodos/` | texto extraído e notas de estudo | versionado | Direitos de `fontes-uva/` em aberto: [../biblioteca/fontes-e-licencas.md](../biblioteca/fontes-e-licencas.md). |

O contrato `crates/estuda-domain/src/publication.rs` expressa a mesma separação por caminho (`LocalOnly`, `Personal`, `Generated`, `Shareable`, `NeedsReview`), mas **não está integrado** ao app nem ao gerador (ver [../biblioteca/estrutura.md](../biblioteca/estrutura.md) §3).

## 3. Salvaguardas no gerador (Implementado)

`tools/acervo/src/validar.mjs` valida cada registro. Regras de dado sensível:

- **Chaves proibidas** (`CHAVES_PROIBIDAS`, comparação sem distinção de maiúsculas): `cpf`, `rg`, `ra`, `senha`, `password`, `pass`, `cookie`, `cookies`, `token`, `tokens`, `secret`, `credencial`, `credenciais`, `email`, `e-mail`, `telefone`, `login`, `usuario`, `usuário`.
- **Valores proibidos** (`VALORES_PROIBIDOS`): caminhos absolutos (`/Users/`, `/home/`, unidades Windows), `Bearer ` e qualquer string com formato de e-mail.
- **Strings vazias** são erro; use `null` com pendência.
- **Fonte existente:** `source.file` precisa existir.
- Resultado em 2026-11-06: `node tools/acervo/build.mjs --validar` → "OK: 234 registros válidos".
- Cobertura: o validador olha chaves e valores **dentro dos JSON de `dados/` e `privado/dados/`**. Ele **não** inspeciona Markdown, `fontes-uva/` nem `privado/materias/`. Limite conhecido: valores como nome de docente, texto livre em `contexto` ou número de RA sem chave proibida não são detectados.

`tools/acervo/src/texto.mjs` impede binários rastreados (PDF, imagem, documento Word/Pages), reduzindo vazamento por anexos.

## 4. Salvaguardas no app

### 4.1 Captura do Portal (Lyceum), `lyceum.rs`

- **URLs:** `sanitize_source_url` só aceita `https` e hosts `portalaluno.uva.br`, `sis.lyceum.com.br` ou `*.uva.br`; remove usuário, senha, query e fragmento. `navigation_target` só mantém parâmetros de paginação numéricos (`page`, `pagina`, `pag`, `p`, `offset`, `per_page`, até 8 dígitos) e recusa rotas de autenticação (`login`, `sso`, `autologin`).
- **Campos identificadores:** rótulos `cpf`, `ra`, `matricula`, `identificacao`, `documento`, `aluno`, `estudante` são omitidos (`lyceum.rs`, por volta das linhas 690–705); cabeçalhos viram "Identificador omitido" e células "[identificador omitido]". O teste `capture.page.tables[0].headers[4] == "Identificador omitido"` cobre o caso.
- **Gravação:** `write_new_file_beneath` e `persist_capture` gravam **arquivos novos** sob a raiz do acervo, recusando symlinks nos diretórios pais (teste `capture_writer_rejects_symlinked_parent_directories`). Nenhuma captura anterior é sobrescrita.
- **Limites:** 50 páginas (`MAX_PORTAL_PAGES`).

### 4.2 Canvas, `canvas.rs`, `download.rs`, `storage.rs`

- `canvas::sanitize_source_url` só aceita URLs que passam em `download::validate_material_url` (HTTPS, host oficial `uva.instructure.com`, `uva.br` ou `*.uva.br`, sem usuário, sem porta diferente de 443) e remove query e fragmento. Material local guarda a URL já sem query (ex.: `...aula.pdf?token=...` vira `...aula.pdf`; teste `markdown_preserves_page_origins_and_removes_query_secrets`).
- `storage::sanitize_capture_message` descarta mensagens de erro que contenham `access_token`, `refresh_token`, `signature=`, `x-amz-signature`, `cookie`, `password`, `authorization`, `api_key`, `?`, `#` ou `http(s)://` antes de gravar o motivo de falha de um item de captura. Rótulos de captura que contêm `http://`, `https://`, `token`, `signature`, `credential` ou `cookie` são recusados.
- O PDF bruto é descartado depois da conversão; só o texto em Markdown fica (`archive.rs`, teste `imported_material_is_markdown_idempotent_and_never_keeps_raw_pdf`).

### 4.3 Escopo de arquivos, `catalog/scanner.rs`, `sync.rs`

- Arquivos cujo nome contém `senha`, `password`, `credential`, `credencial`, `cookie`, `token` ou `secret` são excluídos do escaneador e do sincronizador (`is_credential_filename`; `is_sensitive_or_excluded` em `sync.rs`). `originais/` é excluído da sincronização.
- Limites: 10.000 arquivos, profundidade 16, 64 MiB por documento, 16 MiB por ativo, 128 MiB de texto indexado (`scanner.rs`); 64 MiB por arquivo sincronizado (`sync.rs`).
- Caminhos relativos são validados (sem absoluto nem `..`) e a leitura/gravação recusam symlinks e saídas da raiz (`validate_relative_path`, `safe_existing_path`, `read_optional_safe`, `write_new_source` em `sync.rs`; `canonicalize` em `scanner.rs`).

### 4.4 Logs

Não há `println!`, `eprintln!`, `tracing` nem `log` em `app/crates/estuda-core/src` nem em `app/src-tauri/src` (busca em 2026-11-06). Erros voltam como texto aos comandos Tauri e aparecem na interface; as mensagens de erro de rede e de configuração são fixas ou editadas (ex.: `DownloadError::Network("a conexão foi interrompida")`, `AnthropicError` com erro redigido, testado em `missing_or_empty_api_key_returns_redacted_error`). Não existe arquivo de log a sanitizar. **Não verificado:** logs do próprio Tauri/WebView e do Chromium (`app_data_dir/chromium-profile`) estão fora do código do projeto.

### 4.5 Permissões do sistema de arquivos

O app **não define permissões** (`set_permissions`/`PermissionsExt` não aparecem no código). O banco e os espelhos são criados com a `umask` do usuário. O controle de acesso efetivo é o do macOS (conta do usuário, FileVault). Para um banco com notas e frequência, restringir o diretório a `0700` é recomendação manual, **Planejada**, não aplicada pelo app.

## 5. Envio de conteúdo a serviço externo (Parcial, atenção)

`study_summaries::generate_subject_summaries` envia o texto dos materiais de uma matéria à API da Anthropic (`anthropic.rs`), depois que o usuário confirma o hash da lista de fontes (`confirmed_source_hash`). Detalhes de credencial em [credenciais-e-sessoes.md](credenciais-e-sessoes.md). Pontos de privacidade:

- O envio inclui conteúdo acadêmico (texto de materiais capturados e notas de estudo da matéria). A spec do tutor local recusou serviço remoto justamente por isso (`docs/features/README.md`, decisão em aberto nº 1).
- **Contradição de interface:** a barra lateral do app exibe "Sem IA, nuvem ou cobrança por uso" (`app/src/ui/App.tsx`), enquanto o painel de resumos chama a API externa. Registrado em [../referencia/lacunas-e-contradicoes.md](../referencia/lacunas-e-contradicoes.md).
- Não enviar nota, frequência, matrícula nem trabalho entregue. A decisão de 6/11/2026 trata da publicação no acervo e não autoriza envio a serviço externo. O fluxo coleta fontes de materiais por matéria (`collect_subject_sources`); **Não verificado** se um trabalho do aluno ou nota pode entrar na seleção, pois a regra depende do escopo do escaneador (hoje sem `privado/`).

## 6. Histórico do Git

O histórico contém versões antigas do registro acadêmico. `git log --stat --oneline -- REGISTRO_ACADEMICO.md` (verificado em 2026-11-06) (contagem com `git log --oneline -- REGISTRO_ACADEMICO.md | wc -l`) lista 7 commits que tocaram o arquivo, incluindo `04b8226`, `6dcfe29` e `d2d23ff`, e o commit `e29f27b` (`HEAD` na data da revisão), que o removeu da raiz ao movê-lo para `privado/` (165 linhas removidas da raiz). Remover o arquivo não apaga as versões antigas. Pela decisão de 6/11/2026, o conteúdo pessoal pode ir ao Git desde que separado em `privado/`; as versões antigas, porém, estavam na raiz, fora dessa pasta, e cabe ao dono decidir sobre elas. **Não verificado:** se o histórico contém algum item da lista proibida (credenciais, bancos SQLite, perfil do navegador, originais). Se contiver, será preciso reescrever o histórico (por exemplo com `git filter-repo`) ou publicar a partir de um repositório novo. Isso exige decisão do mantenedor e backup; nada foi executado.

## 7. Regras para quem edita ou contribui

- Nunca inclua senha, cookie, token, `.env`, CPF, RG, RA, e-mail ou caminho absoluto de máquina em Markdown, JSON, logs ou Git. Dado pessoal do aluno (matrícula, notas, frequência) pode ir ao Git desde que esteja em `privado/` (decisão de 6/11/2026).
- Exemplos em documentação devem usar placeholders e dados sintéticos.
- Material pessoal fica em `privado/`; se não houver certeza da camada, trate como privado.
- Rode `node tools/acervo/build.mjs --validar` e `node tools/acervo/build.mjs --verificar-texto` antes de concluir mudanças.
- Antes de publicar qualquer coisa: [publicacao.md](publicacao.md).
