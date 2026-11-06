#!/usr/bin/env node
// Separa dentro do repositório o que é do aluno (privado/) do que é matéria e instituição.
// Move os arquivos, reescreve links relativos em Markdown e caminhos de origem em JSON.
// Registros de dados/ (compartilhados) não podem apontar para privado/: quando apontam,
// o arquivo sai e a fonte fica em texto.
//
// Uso:
//   node tools/acervo/src/separar-privado.mjs            simula e mostra o plano
//   node tools/acervo/src/separar-privado.mjs --aplicar  executa
import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, renameSync, statSync, writeFileSync } from 'node:fs';
import { dirname, join, posix, relative, sep } from 'node:path';
import { fileURLToPath } from 'node:url';

const RAIZ = fileURLToPath(new URL('../../..', import.meta.url));
const APLICAR = process.argv.includes('--aplicar');

// Origem -> destino, relativos à raiz do repositório.
export const MOVIMENTOS = {
  'REGISTRO_ACADEMICO.md': 'privado/REGISTRO_ACADEMICO.md',
  'PLANEJAMENTO.md': 'privado/PLANEJAMENTO.md',
  'IDEIAS.md': 'privado/IDEIAS.md',
  'docs/tmp': 'privado/docs/tmp',
  'cursos': 'privado/cursos',
  'periodos/2026.1/README.md': 'privado/periodos/2026.1/README.md',
  'periodos/2026.1/materiais.md': 'privado/periodos/2026.1/materiais.md',
  'periodos/2026.2/README.md': 'privado/periodos/2026.2/README.md',
  'periodos/2026.2/materiais.md': 'privado/periodos/2026.2/materiais.md',
  'periodos/historicos/README.md': 'privado/periodos/historicos/README.md',
  'periodos/historicos/materiais.md': 'privado/periodos/historicos/materiais.md',
  'materias/bases-de-programacao': 'privado/materias/bases-de-programacao',
  'materias/calculo-de-uma-variavel': 'privado/materias/calculo-de-uma-variavel',
  'materias/estatistica-para-computacao': 'privado/materias/estatistica-para-computacao',
  'materias/linguagens-formais-e-automatos': 'privado/materias/linguagens-formais-e-automatos',
  'materias/paradigmas-de-programacao': 'privado/materias/paradigmas-de-programacao',
  'materias/automatos': 'privado/materias/automatos',
  'materias/calculo-1': 'privado/materias/calculo-1',
  'materias/estatistica': 'privado/materias/estatistica',
  'dados/canvas': 'privado/dados/canvas',
  'dados/matriculas': 'privado/dados/matriculas',
  'dados/notas': 'privado/dados/notas',
  'dados/trajetorias': 'privado/dados/trajetorias',
  'dados/trabalhos': 'privado/dados/trabalhos',
  'dados/meta.json': 'privado/dados/meta.json',
};
// Disciplinas pessoais: registros com identityStatus "matricula_registrada".
const DISCIPLINAS_PESSOAIS = [
  'bases-de-programacao', 'calculo-de-uma-variavel', 'desenvolvimento-de-aplicacoes-para-web',
  'estatistica-para-computacao', 'interfaces-de-sistemas-computacionais', 'laboratorio-de-gestao-de-negocios',
  'leitura-e-producao-academica', 'linguagens-formais-e-automatos', 'paradigmas-de-linguagens-de-programacao',
  'projeto-de-extensao-i-ciencia-da-computacao', 'projeto-de-extensao-i-engenharia-civil', 'sistemas-computacionais-e-aplicacoes',
];
for (const n of DISCIPLINAS_PESSOAIS) MOVIMENTOS[`dados/disciplinas/${n}.json`] = `privado/dados/disciplinas/${n}.json`;

const ORIGENS = Object.keys(MOVIMENTOS).sort((a, b) => b.length - a.length);

// Mapeia um caminho relativo à raiz para o novo caminho, se estiver dentro de um movimento.
export function mapear(caminho) {
  for (const origem of ORIGENS) {
    if (caminho === origem) return MOVIMENTOS[origem];
    if (caminho.startsWith(origem + '/')) return MOVIMENTOS[origem] + caminho.slice(origem.length);
  }
  return caminho;
}

const IGNORADAS = new Set(['.git', 'node_modules', 'target', 'dist', 'build', '.estuda-uva', '.superpowers', 'originais']);

function listar(dir, acc = []) {
  for (const nome of readdirSync(join(RAIZ, dir), { withFileTypes: true })) {
    if (nome.name.startsWith('.') && nome.name !== '.') continue;
    const rel = dir ? `${dir}/${nome.name}` : nome.name;
    if (nome.isDirectory()) {
      if (IGNORADAS.has(nome.name) || rel === 'app' || rel.startsWith('app/') || rel.startsWith('docs/superpowers')) continue;
      listar(rel, acc);
    } else {
      acc.push(rel);
    }
  }
  return acc;
}

// Reescreve um link relativo do arquivo `origemArq` (posição antiga) para a posição nova.
function reescreverLink(alvo, origemArqAntiga, origemArqNova) {
  if (/^(https?:|mailto:|#|\/)/.test(alvo)) return alvo;
  const [caminho, ancora = ''] = alvo.split('#');
  const decodificado = decodeURIComponent(caminho);
  const resolvidoAntigo = posix.normalize(posix.join(posix.dirname(origemArqAntiga), decodificado));
  const novoAlvo = mapear(resolvidoAntigo);
  const mudouAlvo = novoAlvo !== resolvidoAntigo;
  const mudouArquivo = posix.dirname(origemArqAntiga) !== posix.dirname(origemArqNova);
  if (!mudouAlvo && !mudouArquivo) return alvo;
  const relativo = posix.relative(posix.dirname(origemArqNova), novoAlvo) || '.';
  return (relativo + (ancora ? '#' + ancora : ''));
}

function processarMarkdown(texto, antigo, novo) {
  let trocas = 0;
  const saida = texto.replace(/\]\(([^)\s]+)((?:\s+"[^"]*")?)\)/g, (todo, alvo, titulo) => {
    const novoAlvo = reescreverLink(alvo, antigo, novo);
    if (novoAlvo !== alvo) trocas++;
    return `](${novoAlvo}${titulo})`;
  });
  return { texto: saida, trocas };
}

// Em JSON: troca caminhos de origem. Em dados/ (compartilhado) nunca sobra referência a privado/.
function processarJson(obj, compartilhado, stats) {
  if (Array.isArray(obj)) {
    return obj.map((x) => processarJson(x, compartilhado, stats));
  }
  if (obj && typeof obj === 'object') {
    const out = {};
    for (const [k, v] of Object.entries(obj)) {
      if (k === 'file' && typeof v === 'string' && mapear(v) !== v) {
        if (compartilhado && mapear(v).startsWith('privado/')) {
          out.file = null;
          stats.fonteTextual = (stats.fonteTextual || 0) + 1;
          out.fonte = 'fonte institucional/histórica, registrada sem arquivo de origem';
        } else {
          out.file = mapear(v);
          stats.caminhos = (stats.caminhos || 0) + 1;
        }
      } else if (k === 'sourceFiles' && Array.isArray(v)) {
        const mantidos = v.filter((x) => !(compartilhado && mapear(x).startsWith('privado/')));
        const trocados = mantidos.map((x) => mapear(x));
        if (mantidos.length !== v.length) stats.fonteTextual = (stats.fonteTextual || 0) + 1;
        out[k] = trocados;
      } else if (k === 'arquivo' && typeof v === 'string') {
        out[k] = mapear(v);
      } else {
        out[k] = processarJson(v, compartilhado, stats);
      }
    }
    return out;
  }
  return obj;
}

function main() {
  const arquivos = listar('');
  const relatorio = { markdownAlterados: [], jsonAlterados: [], movimentos: [], erros: [] };

  // 1. Movimentos existentes.
  for (const [origem, destino] of Object.entries(MOVIMENTOS)) {
    const abs = join(RAIZ, origem);
    if (!existsSync(abs) && !isLink(abs)) {
      relatorio.erros.push(`ausente: ${origem}`);
      continue;
    }
    if (existsSync(join(RAIZ, destino)) || isLink(join(RAIZ, destino))) {
      relatorio.erros.push(`destino já existe: ${destino}`);
      continue;
    }
    relatorio.movimentos.push(`${origem} -> ${destino}`);
  }

  // 2. Links e caminhos de origem, antes e depois.
  for (const arq of arquivos) {
    const novo = mapear(arq);
    const movido = novo !== arq;
    if (arq.endsWith('.md')) {
      const texto = readFileSync(join(RAIZ, arq), 'utf8');
      const { texto: novoTexto, trocas } = processarMarkdown(texto, arq, novo);
      if (trocas > 0 || movido) relatorio.markdownAlterados.push({ de: arq, para: novo, trocas });
      if (APLICAR && (trocas > 0 || movido) && novoTexto !== texto) {
        // A escrita é feita depois do movimento, no caminho novo.
        writeFileSync(join(RAIZ, arq) + '.__novo', novoTexto);
      }
    } else if (arq.endsWith('.json') && arq.startsWith('dados/')) {
      // Tudo continua no mesmo repositório: a proveniência é mantida e só os caminhos são reapontados.
      const compartilhado = false;
      const obj = JSON.parse(readFileSync(join(RAIZ, arq), 'utf8'));
      const stats = {};
      const novoObj = processarJson(obj, compartilhado, stats);
      if (JSON.stringify(novoObj) !== JSON.stringify(obj)) {
        relatorio.jsonAlterados.push({ arq, stats });
        if (APLICAR) writeFileSync(join(RAIZ, arq) + '.__novo', JSON.stringify(novoObj, null, 2) + '\n');
      }
    } else if (arq.endsWith('.json') && arq.startsWith('privado/')) {
      // (já movidos nesta rodada; não há reprocessamento)
    }
  }

  console.log(`movimentos: ${relatorio.movimentos.length}`);
  console.log(`erros: ${relatorio.erros.length}`);
  relatorio.erros.forEach((e) => console.log('  ERRO', e));
  console.log(`markdown com links a ajustar: ${relatorio.markdownAlterados.filter((m) => m.trocas > 0).length}`);
  console.log(`markdown que se movem: ${relatorio.markdownAlterados.filter((m) => m.de !== m.para).length}`);
  console.log(`json de dados/ alterados: ${relatorio.jsonAlterados.length}`);
  if (!APLICAR) {
    console.log('\n(simulação; use --aplicar para executar)');
    relatorio.movimentos.slice(0, 40).forEach((m) => console.log('  mv', m));
    return;
  }
  if (relatorio.erros.length) {
    console.error('abortado por erros acima');
    process.exit(1);
  }

  // 3. Aplica: primeiro escreve os textos novos nos caminhos antigos, depois move.
  for (const arq of arquivos) {
    const temp = join(RAIZ, arq) + '.__novo';
    if (existsSync(temp)) renameSync(temp, join(RAIZ, arq));
  }
  for (const [origem, destino] of Object.entries(MOVIMENTOS)) {
    const abs = join(RAIZ, origem);
    const dest = join(RAIZ, destino);
    mkdirSync(dirname(dest), { recursive: true });
    renameSync(abs, dest);
  }
  // Markdown que se moveu: o conteúdo já reescrito fica no caminho novo.
  for (const m of relatorio.markdownAlterados) {
    if (m.de !== m.para) {
      // nada a fazer: o arquivo já foi movido junto com o diretório ou com o nome
    }
  }
  console.log('aplicado.');
}

function isLink(p) {
  try {
    return lstatSync(p).isSymbolicLink();
  } catch {
    return false;
  }
}

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) main();
