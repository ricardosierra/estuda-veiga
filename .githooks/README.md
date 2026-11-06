# Hook de copyright

**Regra (decisão do dono, 2026-11-06):** a cada commit, o Git remove dos textos extraídos a **mensagem de copyright e de direitos reservados** e **qualquer linha que contenha a palavra "copyright"**, mesmo no meio de uma frase ou em texto que só fala do assunto. Todo o resto do conteúdo fica.

## O que é removido

Só em arquivos `.md`, `.markdown` e `.txt` sob `fontes-uva/`, `materias/` e `privado/materias/`:

| Remove | Exemplo |
|---|---|
| **Qualquer linha** que contenha a palavra `copyright` (sem diferenciar maiúsculas) | `Texto com o copyright no meio da frase.` |
| Linha de aviso que começa com `Copyright ©`, `Copyright (c)` ou `©` | `Copyright © Editora Exemplo 2021` |
| As linhas seguintes da mesma reserva de reprodução (até 6, mesmo parágrafo ou separadas por uma linha em branco) | `Nenhuma parte desta publicação pode ser reproduzida… autorização desta instituição.` |
| Linha avulsa de reserva | `Todos os direitos reservados.` · `All rights reserved.` |

## O que é mantido

- Alternativas de prova como `(C) a primeira asserção…`: o `(c)` só conta depois da palavra `Copyright`.
- Código, dados, documentação e qualquer arquivo fora do escopo acima.

## Como funciona

`.githooks/pre-commit` chama `strip-copyright.mjs --staged`, que lê o conteúdo **em staging**, remove o aviso, grava o resultado no índice e reescreve o arquivo no disco **se ele ainda for igual ao que estava em staging**. Se o arquivo tiver outras alterações não staged, só o índice é limpo e o disco é mantido (o aviso é impresso). O commit segue normalmente.

Vale só para commits novos. O que já foi enviado continua no histórico do repositório.

## Ativar neste clone

```sh
sh .githooks/instalar.sh
```

Cria `.git/hooks/pre-commit` como link para `.githooks/pre-commit`. **Não** usa `core.hooksPath`, porque isso desligaria hooks globais (por exemplo, os que limpam trailers de coautoria de IA). Se já existir um `pre-commit` diferente em `.git/hooks`, o instalador não o sobrescreve: chame `.githooks/pre-commit` de dentro dele. Exige Node.js (o mesmo do gerador do acervo); sem Node o hook falha com mensagem, e `--no-verify` o ignora conscientemente.

## Comandos

```sh
node .githooks/strip-copyright.mjs --check    # lista o que seria removido; não altera nada
node .githooks/strip-copyright.mjs --all      # limpa os arquivos rastreados no disco; não faz commit
node --test .githooks/strip-copyright.test.mjs
```
