#!/bin/sh
# Ativa o hook versionado neste clone sem mexer em core.hooksPath (o que
# desligaria hooks globais). Cria .git/hooks/pre-commit apontando para
# .githooks/pre-commit. Não sobrescreve um hook que já exista e seja diferente.
set -e
raiz=$(git rev-parse --show-toplevel)
# Usa o .git/hooks real (não core.hooksPath): hooks globais costumam delegar para ele.
gitdir=$(cd "$raiz" && cd "$(git rev-parse --git-dir)" && pwd)
destino="$gitdir/hooks/pre-commit"
alvo="$raiz/.githooks/pre-commit"
chmod +x "$alvo" "$raiz/.githooks/strip-copyright.mjs"
if [ -L "$destino" ] && [ "$(readlink "$destino")" = "$alvo" ]; then
  echo "já instalado: $destino"
  exit 0
fi
if [ -e "$destino" ] || [ -L "$destino" ]; then
  echo "já existe um pre-commit diferente em $destino; nada foi alterado." >&2
  echo "Chame .githooks/pre-commit de dentro dele para manter os dois." >&2
  exit 1
fi
ln -s "$alvo" "$destino"
echo "instalado: $destino -> $alvo"
