// Testes do gerador de acervo. Rodar: node --test tools/acervo/test/
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { DatabaseSync } from 'node:sqlite';
import { fileURLToPath } from 'node:url';
import { carregarDados } from '../src/carregar.mjs';
import { validar } from '../src/validar.mjs';
import { construir } from '../build.mjs';

const RAIZ_REAL = fileURLToPath(new URL('../../..', import.meta.url));

// Monta um acervo mínimo e válido numa pasta temporária.
function acervoMinimo(alteracoes = {}) {
  const raiz = mkdtempSync(join(tmpdir(), 'acervo-'));
  writeFileSync(join(raiz, 'FONTE.md'), '# fonte de teste\n');
  const escrever = (relativo, objeto) => {
    const destino = join(raiz, 'dados', relativo);
    mkdirSync(join(destino, '..'), { recursive: true });
    writeFileSync(destino, JSON.stringify(objeto, null, 2));
  };
  const fonte = { file: 'FONTE.md', section: 'teste', observedAt: '2026-11-06' };
  const base = {
    'instituicao/uva.json': { id: 'uva', title: 'UVA', source: fonte, pendencias: [] },
    'programas/cc.json': {
      id: 'uva:program:cc', institutionId: 'uva', programTitle: 'CC', programCode: '1',
      curriculo: { id: 'uva:curriculum:cc', curriculumVersion: 'V1', trajectoryStatus: 'active', source: fonte, limitations: [] },
      source: fonte, pendencias: [],
    },
    'periodos/2026.2.json': { id: '2026.2', situacao: 'em_andamento', source: fonte, pendencias: [] },
    'canvas/57352.json': {
      id: '57352', titulo: 'Cálculo', url: 'https://uva.instructure.com/courses/57352', tipo: 'curricular',
      situacao: 'em_andamento', periodo: '2026.2', codigo: 'E10100002', secao: null, source: fonte,
      pendencias: [{ campo: 'secao', motivo: 'não registrada' }],
    },
    'disciplinas/calculo.json': {
      id: 'uva:discipline:calculo', title: 'Cálculo', offeringIds: [], identityStatus: 'matricula_registrada', source: fonte, pendencias: [],
    },
    'matriculas/2026.2/calculo.json': {
      id: 'uva:program:cc:enrollment:2026.2:calculo', programId: 'uva:program:cc', semester: '2026.2',
      title: 'Cálculo', section: '21PRD3262A', status: 'enrolled', disciplinaId: 'uva:discipline:calculo',
      canvasCourseId: '57352', docente: null, cargaHoraria: null, source: fonte,
      pendencias: [
        { campo: 'docente', motivo: 'Portal exibe A' },
        { campo: 'cargaHoraria', motivo: 'não consta' },
      ],
    },
    'notas/2026.2/calculo.json': {
      id: 'uva:program:cc:nota:2026.2:calculo', matriculaId: 'uva:program:cc:enrollment:2026.2:calculo',
      ordem: 0, courseTitle: 'Cálculo', status: 'in_progress', lyceumGrades: [{ assessment: 'AVA 1', score: 9, scale: 10, date: '2026-09-07' }],
      lyceumAverage: null, absences: 0, attendancePercent: 100, source: fonte,
      pendencias: [{ campo: 'lyceumAverage', motivo: 'ainda não publicada' }],
    },
    'trajetorias/cc.json': {
      id: 'uva:trajectory:cc', programId: 'uva:program:cc', status: 'active', semester: '2026.2', source: fonte, pendencias: [],
    },
    'meta.json': { trajetorias: { observedAt: '2026-11-06', notes: [] } },
  };
  // Alteração com valor null remove o registro (simula ausência).
  for (const [relativo, objeto] of Object.entries({ ...base, ...alteracoes })) {
    if (objeto === null) rmSync(join(raiz, 'dados', relativo), { force: true });
    else escrever(relativo, objeto);
  }
  return raiz;
}

async function erros(raiz) {
  const dados = await carregarDados(raiz);
  return validar({ raiz, ...dados });
}

test('o acervo real em dados/ é válido', async () => {
  const lista = await erros(RAIZ_REAL);
  assert.deepEqual(lista, [], `erros: ${JSON.stringify(lista.slice(0, 5), null, 2)}`);
});

test('acervo mínimo de teste é válido', async () => {
  assert.deepEqual(await erros(acervoMinimo()), []);
});

test('null sem pendência correspondente é erro', async () => {
  const raiz = acervoMinimo({
    'canvas/57352.json': {
      id: '57352', titulo: 'Cálculo', url: 'https://uva.instructure.com/courses/57352', tipo: 'curricular',
      situacao: 'em_andamento', periodo: '2026.2', codigo: 'E10100002', secao: null, source: { file: 'FONTE.md', section: 's', observedAt: 'x' },
      pendencias: [],
    },
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => e.campo === 'secao' && /sem pendência/.test(e.motivo)), JSON.stringify(lista));
});

test('chave sensível é bloqueada', async () => {
  const raiz = acervoMinimo({
    'trajetorias/cc.json': {
      id: 'uva:trajectory:cc', programId: 'uva:program:cc', status: 'active', source: { file: 'FONTE.md', section: 's', observedAt: 'x' },
      pendencias: [], cpf: '000',
    },
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => /sensível/.test(e.motivo)), JSON.stringify(lista));
});

test('valor com caminho absoluto de usuário é bloqueado', async () => {
  const raiz = acervoMinimo({
    'trajetorias/cc.json': {
      id: 'uva:trajectory:cc', programId: 'uva:program:cc', status: 'active',
      source: { file: '/Users/alguem/arquivo.md', section: 's', observedAt: 'x' }, pendencias: [],
    },
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => /caminho absoluto|sensível/.test(e.motivo)), JSON.stringify(lista));
});

test('url do Canvas inconsistente com o id é erro', async () => {
  const raiz = acervoMinimo({
    'canvas/57352.json': {
      id: '57352', titulo: 'Cálculo', url: 'https://uva.instructure.com/courses/99999', tipo: 'curricular',
      situacao: 'em_andamento', periodo: '2026.2', codigo: 'E10100002', secao: 'x', source: { file: 'FONTE.md', section: 's', observedAt: 'x' },
      pendencias: [],
    },
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => e.campo === 'url'), JSON.stringify(lista));
});

test('referência a período inexistente é erro', async () => {
  const raiz = acervoMinimo({
    'periodos/2026.2.json': null,
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => /referência a periodos inexistente/.test(e.motivo)), JSON.stringify(lista));
});

test('arquivo de fonte inexistente é erro', async () => {
  const raiz = acervoMinimo({
    'instituicao/uva.json': { id: 'uva', title: 'UVA', source: { file: 'NAO_EXISTE.md', section: 's', observedAt: 'x' }, pendencias: [] },
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => /arquivo citado não existe/.test(e.motivo)), JSON.stringify(lista));
});

test('string vazia é erro: use null', async () => {
  const raiz = acervoMinimo({
    'canvas/57352.json': {
      id: '57352', titulo: 'Cálculo', url: 'https://uva.instructure.com/courses/57352', tipo: 'curricular',
      situacao: 'em_andamento', periodo: '2026.2', codigo: '', secao: 'x', source: { file: 'FONTE.md', section: 's', observedAt: 'x' },
      pendencias: [],
    },
  });
  const lista = await erros(raiz);
  assert.ok(lista.some((e) => /string vazia/.test(e.motivo)), JSON.stringify(lista));
});

test('build gera SQLite com views, formato antigo e visualizações', async () => {
  const raiz = acervoMinimo();
  const resultado = await construir({ raiz });
  assert.equal(resultado.ok, true, JSON.stringify(resultado.erros));

  const db = new DatabaseSync(join(raiz, 'build', 'acervo.sqlite3'));
  try {
    const [{ n: matriculas }] = db.prepare('SELECT COUNT(*) AS n FROM v_matriculas').all();
    assert.equal(matriculas, 1);
    const [linha] = db.prepare('SELECT disciplina, canvas_url, codigo FROM v_matriculas').all();
    assert.equal(linha.canvas_url, 'https://uva.instructure.com/courses/57352');
    assert.equal(linha.codigo, 'E10100002');
    const [{ n: pendencias }] = db.prepare('SELECT COUNT(*) AS n FROM v_pendencias').all();
    assert.equal(pendencias, 4);
    const [{ n: avaliacoes }] = db.prepare('SELECT COUNT(*) AS n FROM avaliacoes_lyceum').all();
    assert.equal(avaliacoes, 1);
  } finally {
    db.close();
  }

  const trajetorias = JSON.parse(readFileSync(join(raiz, 'materias', 'catalogo', 'trajetorias.json'), 'utf8'));
  assert.equal(trajetorias.trajectories[0].currentEnrollments.length, 1);
  assert.equal(trajetorias.trajectories[0].outcomes.length, 1);
  assert.ok(existsSync(join(raiz, 'build', 'visualizacoes', 'matriculas.md')));
  assert.ok(existsSync(join(raiz, 'build', 'visualizacoes', 'pendencias.md')));
});
