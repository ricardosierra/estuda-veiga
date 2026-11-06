// Testes do hook de copyright. Rodar: node --test .githooks/strip-copyright.test.mjs
// Usa só texto sintético e repositórios temporários; não toca no repositório real.
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { execFileSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { emEscopo, removerAviso } from './strip-copyright.mjs';

const SCRIPT = join(dirname(fileURLToPath(import.meta.url)), 'strip-copyright.mjs');

test('remove o aviso de copyright e a reserva de reprodução, mantém o resto', () => {
  const entrada = [
    '## Página 3',
    '',
    'Copyright © Editora Exemplo 2021',
    'Nenhuma parte desta publicação pode ser reproduzida por qualquer',
    'meio sem a prévia autorização desta instituição.',
    '',
    'Texto de acordo com as normas do Novo Acordo Ortográfico.',
  ].join('\n');
  const { texto, removidas } = removerAviso(entrada);
  assert.equal(removidas, 3);
  assert.equal(texto, ['## Página 3', '', 'Texto de acordo com as normas do Novo Acordo Ortográfico.'].join('\n'));
});

test('remove o aviso com recuo e sem a palavra Copyright', () => {
  const { texto, removidas } = removerAviso('   © 2020 Autor Exemplo\n       Todos os direitos reservados.\n\nConteúdo.\n');
  assert.equal(removidas, 2);
  assert.equal(texto, 'Conteúdo.\n');
});

test('linha avulsa de direitos reservados sai; a sigla em inglês também', () => {
  assert.equal(removerAviso('A\nAll rights reserved.\nB').texto, 'A\nB');
  assert.equal(removerAviso('A\n“Todos os direitos reservados”\nB').removidas, 1);
});

test('qualquer linha com a palavra copyright sai, o resto fica', () => {
  const entrada = [
    'SOBRE COPYRIGHT e CREATIVE COMMONS',
    '',
    'Copyright é a palavra que, em inglês, significa “direitos autorais” e que',
    'meios sem a sua autorização expressa (“Todos os direitos reservados”).',
    'O DIREITO DO AUTOR É INQUESTIONÁVEL ©',
    'Texto com o copyright no meio da frase.',
    'Corpo normal.',
  ].join('\n');
  const { texto, removidas } = removerAviso(entrada);
  assert.equal(removidas, 4); // título, "Copyright é…", a continuação com "direitos reservados" e a frase
  assert.equal(texto, [
    'O DIREITO DO AUTOR É INQUESTIONÁVEL ©',
    'Corpo normal.',
  ].join('\n'));
});

test('alternativa de prova "(C)" não é copyright', () => {
  const entrada = '(C) a primeira asserção é verdadeira, e a segunda é falsa.\n(c) outra alternativa\n';
  assert.equal(removerAviso(entrada).removidas, 0);
  assert.equal(removerAviso('Copyright (c) 2020 Autor\nCorpo').texto, 'Corpo');
});

test('reserva de reprodução separada do aviso por linha em branco também sai', () => {
  const entrada = '## Página 3\n\nCopyright © UVA 2021\n\nNenhuma parte desta publicação pode ser reproduzida por qualquer meio sem a prévia\nautorização desta instituição.\n\nCorpo do texto.\n';
  const { texto, removidas } = removerAviso(entrada);
  assert.equal(removidas, 3);
  assert.equal(texto, '## Página 3\n\nCorpo do texto.\n');
});

test('é idempotente e preserva CRLF e a quebra de linha final', () => {
  const entrada = 'a\r\nCopyright © X 2019\r\nb\r\n';
  const um = removerAviso(entrada);
  assert.equal(um.texto, 'a\r\nb\r\n');
  assert.equal(removerAviso(um.texto).removidas, 0);
});

test('só trata textos extraídos em .md/.txt de fontes-uva, materias e privado/materias', () => {
  assert.ok(emEscopo('fontes-uva/2026.2/bases/capitulo-01.md'));
  assert.ok(emEscopo('privado/materias/calculo/notas.txt'));
  assert.ok(emEscopo('materias/README.md'));
  assert.ok(!emEscopo('docs/seguranca/publicacao.md'));
  assert.ok(!emEscopo('tools/acervo/src/validar.mjs'));
  assert.ok(!emEscopo('dados/disciplinas/x.json'));
  assert.ok(!emEscopo('fontes-uva/2026.2/x.pdf'));
});

function repoTemporario() {
  const raiz = mkdtempSync(join(tmpdir(), 'hook-copyright-'));
  const g = (...args) => execFileSync('git', args, { cwd: raiz, encoding: 'utf8' });
  g('init', '-q', '-b', 'master');
  g('config', 'user.name', 'Teste');
  g('config', 'user.email', 'teste@example.invalid');
  return { raiz, g };
}

test('modo --staged limpa o índice e o arquivo quando coincidem', () => {
  const { raiz, g } = repoTemporario();
  try {
    mkdirSync(join(raiz, 'fontes-uva/p'), { recursive: true });
    mkdirSync(join(raiz, 'docs'), { recursive: true });
    const aviso = 'Titulo\nCopyright © UVA 2021\nNenhuma parte desta publicação pode ser reproduzida\nCorpo\n';
    writeFileSync(join(raiz, 'fontes-uva/p/a.md'), aviso);
    writeFileSync(join(raiz, 'docs/fora.md'), aviso);
    g('add', '.');
    execFileSync('node', [SCRIPT, '--staged'], { cwd: raiz, encoding: 'utf8' });
    assert.equal(g('show', ':fontes-uva/p/a.md'), 'Titulo\nCorpo\n');
    assert.equal(readFileSync(join(raiz, 'fontes-uva/p/a.md'), 'utf8'), 'Titulo\nCorpo\n');
    assert.equal(g('show', ':docs/fora.md'), aviso, 'fora do escopo não é alterado');
  } finally {
    rmSync(raiz, { recursive: true, force: true });
  }
});

test('modo --staged com alterações não staged limpa só o índice e preserva o disco', () => {
  const { raiz, g } = repoTemporario();
  try {
    mkdirSync(join(raiz, 'materias'), { recursive: true });
    writeFileSync(join(raiz, 'materias/b.md'), 'x\nCopyright © Y 2018\ny\n');
    g('add', '.');
    writeFileSync(join(raiz, 'materias/b.md'), 'x\nCopyright © Y 2018\ny\nedição local\n');
    execFileSync('node', [SCRIPT, '--staged'], { cwd: raiz, encoding: 'utf8' });
    assert.equal(g('show', ':materias/b.md'), 'x\ny\n');
    assert.equal(readFileSync(join(raiz, 'materias/b.md'), 'utf8'), 'x\nCopyright © Y 2018\ny\nedição local\n');
  } finally {
    rmSync(raiz, { recursive: true, force: true });
  }
});
