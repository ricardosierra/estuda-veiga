// Gera build/acervo.sqlite3 a partir dos registros de dados/. O arquivo é descartável:
// é recriado do zero a cada build. Não toca no .estuda-uva.sqlite3 do app.
import { rmSync, mkdirSync } from 'node:fs';
import { dirname } from 'node:path';
import { DatabaseSync } from 'node:sqlite';

const valor = (v) => {
  if (v === undefined) return null;
  if (typeof v === 'boolean') return v ? 1 : 0;
  if (v !== null && typeof v === 'object') return JSON.stringify(v);
  return v;
};

const PENDENCIAS_DE = (registro) => (Array.isArray(registro.pendencias) ? registro.pendencias : []);

// Cada tabela: nome, colunas [nome, tipo], e como extrair a linha de um registro.
const TABELAS = [
  {
    nome: 'programas',
    colunas: [['id', 'TEXT PRIMARY KEY'], ['titulo', 'TEXT'], ['codigo', 'TEXT'], ['instituicao', 'TEXT']],
    linha: (r) => [r.id, r.programTitle, r.programCode, r.institutionId],
    tipo: 'programas',
  },
  {
    nome: 'periodos',
    colunas: [['id', 'TEXT PRIMARY KEY'], ['situacao', 'TEXT'], ['observado_em', 'TEXT']],
    linha: (r) => [r.id, r.situacao, r.source?.observedAt],
    tipo: 'periodos',
  },
  {
    nome: 'cursos_canvas',
    colunas: [
      ['id', 'TEXT PRIMARY KEY'], ['titulo', 'TEXT'], ['url', 'TEXT'], ['tipo', 'TEXT'],
      ['situacao', 'TEXT'], ['periodo', 'TEXT'], ['codigo', 'TEXT'], ['secao', 'TEXT'],
      ['fonte', 'TEXT'],
    ],
    linha: (r) => [r.id, r.titulo, r.url, r.tipo, r.situacao, r.periodo, r.codigo, r.secao, r.source?.file],
    tipo: 'canvas',
  },
  {
    nome: 'disciplinas',
    colunas: [['id', 'TEXT PRIMARY KEY'], ['titulo', 'TEXT'], ['identidade', 'TEXT'], ['fonte', 'TEXT']],
    linha: (r) => [r.id, r.title, r.identityStatus, r.source?.file],
    tipo: 'disciplinas',
  },
  {
    nome: 'ofertas_curriculares',
    colunas: [
      ['id', 'TEXT PRIMARY KEY'], ['programa_id', 'TEXT'], ['codigo', 'TEXT'], ['titulo', 'TEXT'],
      ['carga_horaria', 'INTEGER'], ['situacao', 'TEXT'], ['disciplina_id', 'TEXT'],
    ],
    linha: (r) => [r.id, r.programId, r.code, r.title, r.workloadHours, r.status, r.canonicalDisciplineId],
    tipo: 'ofertas',
  },
  {
    nome: 'relacoes',
    colunas: [
      ['id', 'TEXT PRIMARY KEY'], ['tipo', 'TEXT'], ['oferta_cc', 'TEXT'], ['oferta_ec', 'TEXT'],
      ['disciplina_id', 'TEXT'], ['equivalencia_confirmada', 'INTEGER'],
    ],
    linha: (r) => [r.id, r.type, r.ccOfferingId, r.ecOfferingId, r.canonicalDisciplineId, r.equivalenceConfirmed],
    tipo: 'relacoes',
  },
  {
    nome: 'trajetorias',
    colunas: [['id', 'TEXT PRIMARY KEY'], ['programa_id', 'TEXT'], ['situacao', 'TEXT'], ['periodo', 'TEXT'], ['contexto', 'TEXT']],
    linha: (r) => [r.id, r.programId, r.status, r.semester ?? null, r.contexto ?? null],
    tipo: 'trajetorias',
  },
  {
    nome: 'matriculas',
    colunas: [
      ['id', 'TEXT PRIMARY KEY'], ['programa_id', 'TEXT'], ['periodo', 'TEXT'], ['titulo', 'TEXT'],
      ['turma', 'TEXT'], ['disciplina_id', 'TEXT'], ['canvas_id', 'TEXT'], ['docente', 'TEXT'],
      ['docente_exibido', 'TEXT'], ['inicio', 'TEXT'], ['situacao', 'TEXT'], ['carga_horaria', 'INTEGER'],
    ],
    linha: (r) => [
      r.id, r.programId, r.semester, r.title, r.section, r.disciplinaId, r.canvasCourseId, r.docente,
      r.instructorAsDisplayed ?? null, r.startDate ?? null, r.status, r.cargaHoraria,
    ],
    tipo: 'matriculas',
  },
  {
    nome: 'notas',
    colunas: [
      ['id', 'TEXT PRIMARY KEY'], ['matricula_id', 'TEXT'], ['ordem', 'INTEGER'], ['curso_titulo', 'TEXT'],
      ['situacao', 'TEXT'], ['media', 'REAL'], ['faltas', 'INTEGER'], ['frequencia_percent', 'REAL'],
      ['canvas_pontos', 'INTEGER'], ['canvas_possiveis', 'INTEGER'], ['canvas_percent', 'REAL'],
    ],
    linha: (r) => [
      r.id, r.matriculaId, r.ordem, r.courseTitle, r.status, r.lyceumAverage ?? null, r.absences ?? null,
      r.attendancePercent ?? null, r.canvasAvaTotals?.earnedPoints ?? null,
      r.canvasAvaTotals?.possiblePoints ?? null, r.canvasAvaTotals?.percent ?? null,
    ],
    tipo: 'notas',
  },
  {
    nome: 'trabalhos',
    colunas: [
      ['id', 'TEXT PRIMARY KEY'], ['titulo', 'TEXT'], ['arquivo', 'TEXT'], ['disciplina_id', 'TEXT'],
      ['canvas_id', 'TEXT'], ['periodo', 'TEXT'],
    ],
    linha: (r) => [r.id, r.titulo, r.arquivo, r.disciplinaId, r.canvasCourseId, r.periodo],
    tipo: 'trabalhos',
  },
];

const TABELA_AVALIACOES = {
  nome: 'avaliacoes_lyceum',
  colunas: [['matricula_id', 'TEXT'], ['ordem', 'INTEGER'], ['avaliacao', 'TEXT'], ['nota', 'REAL'], ['escala', 'REAL'], ['data', 'TEXT']],
};

const VIEWS = [
  `CREATE VIEW v_matriculas AS
     SELECT m.id, m.periodo, p.titulo AS programa, d.titulo AS disciplina, m.titulo, m.turma,
            c.id AS canvas_id, c.url AS canvas_url, c.codigo, c.secao AS canvas_secao,
            m.docente, m.docente_exibido, m.inicio, m.situacao, m.carga_horaria,
            n.media, n.faltas, n.frequencia_percent, n.canvas_percent
     FROM matriculas m
     LEFT JOIN programas p ON p.id = m.programa_id
     LEFT JOIN disciplinas d ON d.id = m.disciplina_id
     LEFT JOIN cursos_canvas c ON c.id = m.canvas_id
     LEFT JOIN notas n ON n.matricula_id = m.id`,
  `CREATE VIEW v_cursos AS
     SELECT c.*, (SELECT COUNT(*) FROM matriculas m WHERE m.canvas_id = c.id) AS matriculas
     FROM cursos_canvas c`,
  `CREATE VIEW v_pendencias AS
     SELECT entidade, id, campo, motivo, arquivo FROM pendencias ORDER BY entidade, id, campo`,
  `CREATE VIEW v_resumo_periodo AS
     SELECT periodo, COUNT(*) AS matriculas, SUM(carga_horaria) AS carga_horaria_informada
     FROM matriculas GROUP BY periodo`,
  `CREATE VIEW v_curriculo AS
     SELECT programa_id, codigo, titulo, carga_horaria, situacao, disciplina_id
     FROM ofertas_curriculares ORDER BY programa_id, codigo`,
];

function criarTabela(db, nome, colunas) {
  const definicao = colunas.map(([coluna, tipo]) => `"${coluna}" ${tipo}`).join(', ');
  db.exec(`CREATE TABLE "${nome}" (${definicao}, dados_json TEXT)`);
}

function inserir(db, nome, colunas, linhas) {
  if (linhas.length === 0) return;
  const nomes = [...colunas.map(([c]) => `"${c}"`), '"dados_json"'];
  const marcas = nomes.map(() => '?').join(', ');
  const comando = db.prepare(`INSERT INTO "${nome}" (${nomes.join(', ')}) VALUES (${marcas})`);
  for (const { valores, json } of linhas) comando.run(...valores.map(valor), json);
}

export function gerarSqlite({ caminho, registros, materiais }) {
  for (const sufixo of ['', '-wal', '-shm']) rmSync(caminho + sufixo, { force: true });
  mkdirSync(dirname(caminho), { recursive: true });
  const db = new DatabaseSync(caminho);
  try {
    db.exec('BEGIN');
    for (const tabela of TABELAS) {
      criarTabela(db, tabela.nome, tabela.colunas);
      const registrosDoTipo = registros.filter((r) => r.tipo === tabela.tipo);
      inserir(
        db,
        tabela.nome,
        tabela.colunas,
        registrosDoTipo.map((r) => ({ valores: tabela.linha(r.dados), json: JSON.stringify(r.dados) })),
      );
    }

    criarTabela(db, TABELA_AVALIACOES.nome, TABELA_AVALIACOES.colunas);
    const avaliacoes = [];
    for (const r of registros.filter((x) => x.tipo === 'notas')) {
      for (const [indice, avaliacao] of (r.dados.lyceumGrades ?? []).entries()) {
        avaliacoes.push({
          valores: [r.dados.matriculaId, indice + 1, avaliacao.assessment, avaliacao.score, avaliacao.scale, avaliacao.date],
          json: JSON.stringify(avaliacao),
        });
      }
    }
    inserir(db, TABELA_AVALIACOES.nome, TABELA_AVALIACOES.colunas, avaliacoes);

    // Pendências de todos os registros, numa tabela só, para revisão.
    criarTabela(db, 'pendencias', [
      ['entidade', 'TEXT'], ['id', 'TEXT'], ['campo', 'TEXT'], ['motivo', 'TEXT'], ['arquivo', 'TEXT'],
    ]);
    const pendencias = [];
    for (const r of registros) {
      for (const p of PENDENCIAS_DE(r.dados)) {
        pendencias.push({
          valores: [r.tipo, r.dados.id, p.campo, p.motivo, r.caminho],
          json: JSON.stringify(p),
        });
      }
    }
    inserir(
      db,
      'pendencias',
      [['entidade', 'TEXT'], ['id', 'TEXT'], ['campo', 'TEXT'], ['motivo', 'TEXT'], ['arquivo', 'TEXT']],
      pendencias,
    );

    criarTabela(db, 'arquivos', [['caminho', 'TEXT PRIMARY KEY'], ['pasta', 'TEXT'], ['extensao', 'TEXT'], ['bytes', 'INTEGER']]);
    inserir(
      db,
      'arquivos',
      [['caminho', 'TEXT'], ['pasta', 'TEXT'], ['extensao', 'TEXT'], ['bytes', 'INTEGER']],
      materiais.map((m) => ({ valores: [m.caminho, m.pasta, m.extensao, m.bytes], json: JSON.stringify(m) })),
    );

    for (const sql of VIEWS) db.exec(sql);
    db.exec('COMMIT');
  } catch (erro) {
    db.exec('ROLLBACK');
    throw erro;
  } finally {
    db.close();
  }
  return caminho;
}
