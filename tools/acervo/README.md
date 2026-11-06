# tools/acervo — gerador do acervo acadêmico

Lê `dados/` (texto, um registro por arquivo), valida e gera as representações derivadas.
Não depende de nenhum pacote externo: usa Node 24 (`node:sqlite`).

## Comandos

```sh
node tools/acervo/build.mjs --validar   # valida dados/
node tools/acervo/build.mjs             # valida e gera
node tools/acervo/build.mjs --verificar-texto   # binários no repositório (erro) e locais (aviso)
node tools/acervo/src/pdf-para-md.mjs <arquivo.pdf> <saida.md> "<fonte>"   # PDF -> Markdown
node --test tools/acervo/test/acervo.test.mjs tools/acervo/test/texto.test.mjs
```

## Arquivos

- `build.mjs`: ponto de entrada. Orquestra validação e geração.
- `src/carregar.mjs`: lê `dados/` e lista os arquivos de `materias/` (nome, pasta, tamanho; sem ler o conteúdo).
- `src/validar.mjs`: regras de campos, valores, nulos com pendência, referências, fontes existentes e dados sensíveis.
- `src/legado.mjs`: monta o formato antigo que o app lê (`materias/catalogo/*.json`).
- `src/sqlite.mjs`: gera `build/acervo.sqlite3` (tabelas, `pendencias`, `arquivos`, views).
- `src/visualizacoes.mjs`: gera `build/visualizacoes/*.md` consultando as views.
- `migrar-legado.mjs`: migração única do formato antigo para `dados/`, com as lacunas preenchidas a partir das fontes. Já foi executada; mantida como registro do que foi feito.
- `src/texto.mjs`: política de conteúdo em texto (binários rastreados são erro; binários locais são aviso).
- `src/pdf-para-md.mjs`: converte PDF em Markdown por página com `pdftotext`.
- `test/acervo.test.mjs` e `test/texto.test.mjs`: validação, build e política de texto.

## Relação com o app Estuda UVA

- O app continua lendo `materias/catalogo/*.json` do jeito que lia antes. Esses arquivos agora são gerados por este gerador.
- `build/acervo.sqlite3` é separado do `.estuda-uva.sqlite3` que o app usa. Não há escrita no banco do app.
- Para o app ler o `build/acervo.sqlite3` diretamente, é preciso mudar o código Rust (`src/catalog/academic.rs` e `src/sync.rs`). Isso não foi feito.
