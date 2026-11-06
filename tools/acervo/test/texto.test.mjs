// Testes da política "conteúdo em texto". Rodar: node --test tools/acervo/test/texto.test.mjs
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { verificarTexto } from '../src/texto.mjs';

function raizComArquivos(arquivos) {
  const raiz = mkdtempSync(join(tmpdir(), 'texto-'));
  for (const relativo of arquivos) {
    const destino = join(raiz, relativo);
    mkdirSync(join(destino, '..'), { recursive: true });
    writeFileSync(destino, 'x');
  }
  return raiz;
}

test('binário fora de pastas ignoradas vira aviso quando não há Git', async () => {
  const raiz = raizComArquivos(['materias/aula/prova.pdf', 'materias/aula/notas.md']);
  const { erros, avisos } = await verificarTexto(raiz);
  assert.deepEqual(erros, []);
  assert.deepEqual(avisos.map((a) => a.caminho), ['materias/aula/prova.pdf']);
});

test('PNG e DOCX são reconhecidos como binários', async () => {
  const raiz = raizComArquivos(['materias/diagrama.png', 'materias/trabalho.docx', 'materias/texto.md']);
  const { avisos } = await verificarTexto(raiz);
  assert.deepEqual(avisos.map((a) => a.caminho).sort(), ['materias/diagrama.png', 'materias/trabalho.docx']);
});

test('pastas de código, dependências e originais privados não são verificadas', async () => {
  const raiz = raizComArquivos([
    'app/src-tauri/icons/icon.png',
    'originais/trabalho.docx',
    'build/visualizacoes/x.png',
    'node_modules/pacote/logo.png',
    'tmp/pdfs/tela.png',
  ]);
  const { erros, avisos } = await verificarTexto(raiz);
  assert.deepEqual(erros, []);
  assert.deepEqual(avisos, []);
});
