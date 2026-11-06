#!/usr/bin/env node
// Lê dados/ (fonte de verdade em texto), valida e gera:
//   - materias/catalogo/*.json  (formato antigo que o app Estuda UVA lê; gerado, não editar)
//   - build/acervo.sqlite3      (SQLite com tabelas e views; gerado, não versionar)
//   - build/visualizacoes/*.md  (leitura rápida; gerado, não versionar)
//
// Uso:
//   node tools/acervo/build.mjs            valida e gera tudo
//   node tools/acervo/build.mjs --validar          só valida dados/
//   node tools/acervo/build.mjs --verificar-texto  aponta binários (PDF, PNG etc.) no repositório
import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { carregarDados, listarMateriais } from './src/carregar.mjs';
import { validar } from './src/validar.mjs';
import { gerarLegado, SAIDA_LEGADO } from './src/legado.mjs';
import { gerarSqlite } from './src/sqlite.mjs';
import { gerarVisualizacoes } from './src/visualizacoes.mjs';
import { verificarTexto } from './src/texto.mjs';

export const RAIZ_PADRAO = fileURLToPath(new URL('../..', import.meta.url));

export async function construir({ raiz = RAIZ_PADRAO, somenteValidar = false } = {}) {
  const dados = await carregarDados(raiz);
  const erros = validar({ raiz, ...dados });
  if (erros.length > 0) return { ok: false, erros };
  if (somenteValidar) return { ok: true, registros: dados.registros.length };

  const legado = gerarLegado(dados.registros);
  for (const [relativo, conteudo] of Object.entries(legado)) {
    const destino = join(raiz, relativo);
    mkdirSync(join(destino, '..'), { recursive: true });
    writeFileSync(destino, `${JSON.stringify(conteudo, null, 2)}\n`);
  }

  const materiais = await listarMateriais(raiz);
  const caminhoSqlite = join(raiz, 'build', 'acervo.sqlite3');
  gerarSqlite({ caminho: caminhoSqlite, registros: dados.registros, materiais });
  const visualizacoes = gerarVisualizacoes({
    caminhoSqlite,
    pasta: join(raiz, 'build', 'visualizacoes'),
  });

  return {
    ok: true,
    registros: dados.registros.length,
    legado: Object.keys(legado),
    sqlite: caminhoSqlite,
    visualizacoes,
    materiais: materiais.length,
    saidaLegado: SAIDA_LEGADO,
  };
}

async function principal() {
  if (process.argv.includes('--verificar-texto')) {
    const { erros, avisos } = await verificarTexto(RAIZ_PADRAO);
    for (const e of erros) console.error(`ERRO ${e.caminho}: ${e.motivo}`);
    for (const a of avisos) console.log(`AVISO ${a.caminho}: ${a.motivo}`);
    console.log(`${erros.length} erro(s), ${avisos.length} aviso(s).`);
    process.exit(erros.length > 0 ? 1 : 0);
  }
  const somenteValidar = process.argv.includes('--validar');
  const resultado = await construir({ somenteValidar });
  if (!resultado.ok) {
    console.error(`Validação falhou com ${resultado.erros.length} erro(s):`);
    for (const e of resultado.erros) {
      console.error(`- ${e.caminho}${e.campo ? ` [${e.campo}]` : ''}: ${e.motivo}`);
    }
    process.exit(1);
  }
  if (somenteValidar) {
    console.log(`OK: ${resultado.registros} registros válidos em dados/.`);
    return;
  }
  console.log(`OK: ${resultado.registros} registros validados.`);
  console.log(`- SQLite: ${resultado.sqlite}`);
  console.log(`- Visualizações: build/visualizacoes/ (${resultado.visualizacoes.length} tabelas)`);
  console.log(`- Formato antigo do app: ${resultado.saidaLegado}/ (${resultado.legado.length} arquivos)`);
  console.log(`- Materiais indexados: ${resultado.materiais}`);
}

if (process.argv[1] && fileURLToPath(import.meta.url) === process.argv[1]) {
  principal().catch((erro) => {
    console.error(erro);
    process.exit(1);
  });
}
