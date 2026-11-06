# Estuda UVA (GPUI)

Interface nativa do Estuda UVA, em Rust com GPUI. Reproduz o layout do canvas
"Estuda UVA" (`Estuda UVA.dc.html`): barra de título, sidebar com as oito telas,
cabeçalho com busca, tema claro e escuro, e os tokens do design system.

## Executar

```sh
cd app
cargo run -p estuda-gpui
```

O acervo padrão é a raiz do repositório (`app/crates/estuda-gpui/../../..`).

## O que está ligado e o que não está

| Tela | Estado |
|---|---|
| Biblioteca | Lista os documentos do índice em RAM (`estuda-core`), com filtros por categoria real e busca pelo cabeçalho. |
| Leitor PDF→MD | Lê o Markdown do acervo em segundo plano. Mostra a origem do arquivo e no máximo 400 linhas. Ouvir, pausar e parar usam a voz pt-BR do eSpeak NG embutido. |
| Painel, Acadêmico, Estudo, Auxílio, Usuário | Dados do protótipo do canvas (`ui/data.rs`). Não são do acervo. |
| Sincronização | Sincronizar abre o Portal no Chromium do app (perfil em `~/Library/Application Support/br.edu.uva.estuda/chromium-profile`). "Concluí o login" confere a sessão real. Notas, Canvas e calendário ainda não são lidos. |
| Cabeçalho | Busca, tema e botão Sincronizar. A busca leva à Biblioteca. |
| Barra de RAM | RSS real do processo, lido de `ps` a cada 3 s. |

O limite de memória do layout (64 a 1024 MB) ainda não altera nenhum comportamento.

## Dados e segurança

- O app só lê arquivos do acervo. Não abre `.estuda-uva.sqlite3`: `AcademicStore`
  e `SyncEngine` executam `CREATE`/`ALTER TABLE` ao abrir, e isso exige aprovação
  prévia do dono.
- O índice do acervo fica em RAM (`CatalogService`), sem varrer o disco a cada tela.
- Perfil do Chromium e cookies não entram no repositório nem nos logs.

## Narração (eSpeak NG)

- Motor: eSpeak NG, o mesmo que o Ubuntu usa via Speech Dispatcher. Compilado a partir do fonte
  pelo crate `espeak-rs-sys`, com a feature `compile-espeak-intonations` (sem ela não há dados de voz).
- Dados de voz: `ESTUDA_ESPEAK_DATA`, a pasta `espeak-ng-data` ao lado do binário, ou o cache
  do build em `target/`. Para distribuir, copie `espeak-ng-data` junto do binário.
- **Licença:** o eSpeak NG é GPL-3.0-or-later e fica linkado estaticamente. Distribuir o binário
  obriga a licenciar o app sob GPL-3.0-or-later (ou compatível) e a entregar o código-fonte. Uso
  pessoal não tem essa obrigação.
- O narrador é criado só quando o aluno pede para ouvir.

## Decisões de base

- **GPUI:** `gpui-kit` 0.9.0 reexporta um GPUI compatível com `gpui-component`.
  O `gpui` 0.2.2 publicado no crates.io não é compatível com esses widgets.
- **Ícones:** 21 traços SVG extraídos de `_ds_bundle.js` (`DMConecta.Icon`),
  embutidos com `include_bytes!`.
- **Fontes:** o layout usa DM Sans e Inter, que não estão instaladas no macOS
  de referência; o texto cai para a fonte do sistema.

## Verificação

```sh
cargo build -p estuda-gpui
cargo test -p estuda-gpui
cargo test -p estuda-gpui -- --ignored --nocapture   # lê o acervo real, sem títulos
```

A conferência visual contra o protótipo depende da permissão de Gravação de Tela
do terminal (captura com `screencapture`).
