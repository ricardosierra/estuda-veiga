#!/usr/bin/env node
// Remove a mensagem de copyright / direitos reservados dos textos extraídos que
// vão para o Git. Mantém todo o resto do conteúdo.
//
// Uso:
//   node .githooks/strip-copyright.mjs --staged   (hook pre-commit: limpa o que está em staging)
//   node .githooks/strip-copyright.mjs --check    (lista o que seria removido; não altera nada)
//   node .githooks/strip-copyright.mjs --all      (limpa os arquivos rastreados no disco; não faz commit)
//
// Regra (decisão do dono, 2026-11-06): remover o aviso de copyright e de direitos
// reservados e qualquer linha que contenha a palavra "copyright".
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';

// Só textos extraídos de materiais; código, dados e documentação ficam de fora
// porque citam a expressão sem serem o aviso.
export const ESCOPO_PREFIXOS = ['fontes-uva/', 'materias/', 'privado/materias/'];
export const ESCOPO_EXTENSOES = ['.md', '.markdown', '.txt'];

// Linha de aviso: "Copyright © UVA 2021", "© 2021 Editora", "Copyright (c) 2020".
// "(c)" só vale depois da palavra Copyright: "(C) a primeira asserção" é alternativa de prova.
const LINHA_COPYRIGHT = /^\s*(?:copyright\s*(?:©|\(c\))|©)/i;
const LINHA_COPYRIGHT_SEM_SIMBOLO = /^\s*copyright\s+(?:\d{4}|de\s+\d{4})\b/i;
// Decisão do dono (2026-11-06): qualquer linha com a palavra "copyright", em qualquer
// posição, sai inteira (inclusive texto que só fala do assunto).
const LINHA_MENCIONA_COPYRIGHT = /copyright/i;
// Linha que é só a fórmula de reserva.
const LINHA_RESERVA = /^[\s"'“”()\[\].,;:*_-]*(?:todos os direitos(?: autorais)? reservados|direitos(?: autorais)? reservados|all rights reserved)[\s"'“”()\[\].,;:*_-]*$/i;
// Linhas que continuam o aviso logo abaixo de "Copyright ©" (mesmo parágrafo).
const CONTINUACAO_AVISO = /nenhuma parte|reproduzid|reprodu[cç][aã]o|autoriza[cç][aã]o desta|direitos (?:autorais )?reservados|rights reserved/i;
const MAX_CONTINUACAO = 6;

export function emEscopo(caminho) {
  const c = caminho.replace(/\\/g, '/');
  return (
    ESCOPO_PREFIXOS.some((p) => c.startsWith(p)) &&
    ESCOPO_EXTENSOES.some((e) => c.toLowerCase().endsWith(e))
  );
}

/** Retorna { texto, removidas } com as linhas de aviso retiradas. Idempotente. */
export function removerAviso(texto) {
  const eol = texto.includes('\r\n') ? '\r\n' : '\n';
  const linhas = texto.split(/\r?\n/);
  const saida = [];
  let removidas = 0;
  let i = 0;
  while (i < linhas.length) {
    const linha = linhas[i];
    if (LINHA_MENCIONA_COPYRIGHT.test(linha) || LINHA_COPYRIGHT.test(linha) || LINHA_COPYRIGHT_SEM_SIMBOLO.test(linha)) {
      removidas += 1;
      i += 1;
      // Reserva de reprodução separada do aviso por até 2 linhas em branco.
      let j = i;
      while (j < linhas.length && j - i < 2 && linhas[j].trim() === '') j += 1;
      if (j > i && j < linhas.length && CONTINUACAO_AVISO.test(linhas[j])) i = j;
      let seguintes = 0;
      while (
        i < linhas.length &&
        seguintes < MAX_CONTINUACAO &&
        linhas[i].trim() !== '' &&
        CONTINUACAO_AVISO.test(linhas[i])
      ) {
        removidas += 1;
        i += 1;
        seguintes += 1;
      }
      pularBrancos(linhas, i, saida, (n) => { i = n; });
      continue;
    }
    if (LINHA_RESERVA.test(linha)) {
      removidas += 1;
      i += 1;
      pularBrancos(linhas, i, saida, (n) => { i = n; });
      continue;
    }
    saida.push(linha);
    i += 1;
  }
  return { texto: saida.join(eol), removidas };
}

// Depois de um bloco removido, não deixa duas linhas em branco seguidas.
function pularBrancos(linhas, i, saida, definir) {
  const anteriorEmBranco = saida.length === 0 || saida[saida.length - 1].trim() === '';
  let n = i;
  if (anteriorEmBranco) while (n < linhas.length && linhas[n].trim() === '') n += 1;
  definir(n);
}

function git(args, opcoes = {}) {
  return execFileSync('git', args, { encoding: 'utf8', maxBuffer: 256 * 1024 * 1024, ...opcoes });
}

function relatar(caminho, n, extra = '') {
  process.stderr.write(`copyright removido: ${caminho} (${n} linha${n === 1 ? '' : 's'})${extra}\n`);
}

function modoStaged() {
  const lista = git(['diff', '--cached', '--name-only', '--diff-filter=ACM', '-z'])
    .split('\0')
    .filter(Boolean)
    .filter(emEscopo);
  const raiz = git(['rev-parse', '--show-toplevel']).trim();
  let total = 0;
  for (const caminho of lista) {
    const indexado = git(['show', `:${caminho}`]);
    if (indexado.includes('\0')) continue;
    const { texto, removidas } = removerAviso(indexado);
    if (removidas === 0) continue;
    const modo = git(['ls-files', '-s', '--', caminho]).split(/\s+/)[0] || '100644';
    const hash = git(['hash-object', '-w', '--stdin'], { input: texto }).trim();
    git(['update-index', '--cacheinfo', `${modo},${hash},${caminho}`]);
    total += removidas;
    const disco = join(raiz, caminho);
    let atual = null;
    try { atual = readFileSync(disco, 'utf8'); } catch { /* arquivo ausente no disco */ }
    if (atual === indexado) {
      writeFileSync(disco, texto);
      relatar(caminho, removidas);
    } else {
      relatar(caminho, removidas, ' só no índice: o arquivo no disco tem outras alterações e foi mantido');
    }
  }
  return total;
}

function modoArquivos(escrever) {
  const raiz = git(['rev-parse', '--show-toplevel']).trim();
  const lista = git(['ls-files', '-z']).split('\0').filter(Boolean).filter(emEscopo);
  let total = 0;
  for (const caminho of lista) {
    let atual;
    try { atual = readFileSync(join(raiz, caminho), 'utf8'); } catch { continue; }
    if (atual.includes('\0')) continue;
    const { texto, removidas } = removerAviso(atual);
    if (removidas === 0) continue;
    total += removidas;
    if (escrever) writeFileSync(join(raiz, caminho), texto);
    process.stderr.write(`${escrever ? 'copyright removido' : 'copyright encontrado'}: ${caminho} (${removidas})\n`);
  }
  return total;
}

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  const modo = process.argv[2];
  try {
    if (modo === '--staged') modoStaged();
    else if (modo === '--check') process.stderr.write(`${modoArquivos(false)} linha(s) a remover.\n`);
    else if (modo === '--all') process.stderr.write(`${modoArquivos(true)} linha(s) removidas no disco.\n`);
    else {
      process.stderr.write('uso: strip-copyright.mjs --staged | --check | --all\n');
      process.exit(2);
    }
  } catch (erro) {
    process.stderr.write(`strip-copyright falhou: ${erro.message}\n`);
    process.exit(1);
  }
}
