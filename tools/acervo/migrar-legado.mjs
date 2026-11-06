#!/usr/bin/env node
// Migração única: converte o formato antigo (materias/catalogo/*.json, versionado até o
// commit anterior) em registros de texto em dados/, um arquivo por registro.
// Preenche as lacunas conhecidas a partir das fontes locais citadas em cada registro.
// Valores ausentes ficam null com pendência; nada é inferido.
//
// Uso: node tools/acervo/migrar-legado.mjs --origem <pasta com curriculos/, disciplinas.json, ...>
import { mkdirSync, readFileSync, writeFileSync, readdirSync, existsSync } from 'node:fs';
import { join, basename } from 'node:path';
import { fileURLToPath } from 'node:url';
import { URL_CANVAS } from './src/validar.mjs';

const RAIZ = fileURLToPath(new URL('../..', import.meta.url));
const args = process.argv.slice(2);
const origem = args[args.indexOf('--origem') + 1];
if (!origem || !existsSync(origem)) {
  console.error('Informe --origem <pasta do formato antigo>');
  process.exit(1);
}
const dadosDir = join(RAIZ, 'dados');
if (existsSync(dadosDir) && !args.includes('--forcar')) {
  console.error('dados/ já existe. Use --forcar para sobrescrever.');
  process.exit(1);
}

const ler = (arquivo) => JSON.parse(readFileSync(join(origem, arquivo), 'utf8'));
const escrever = (relativo, objeto) => {
  const destino = join(dadosDir, relativo);
  mkdirSync(join(destino, '..'), { recursive: true });
  writeFileSync(destino, `${JSON.stringify(objeto, null, 2)}\n`);
};
const slugDe = (id) => id.split(':').pop();
const semNulos = [];

const FONTE_2026_2_MATRICULAS = { file: 'CURSOS_2026.2.md', section: 'Matrículas informadas pelo Portal do Aluno', observedAt: '2026-10-14' };
const FONTE_2026_2_FAVORITOS = { file: 'CURSOS_2026.2.md', section: 'Favoritos que devem aparecer na lista de cursos do Canvas', observedAt: '2026-10-13' };
const FONTE_2026_1 = { file: 'CURSOS_2026.1.md', section: 'Como navegar', observedAt: '2026-10-08' };
const FONTE_HISTORICOS = { file: 'CURSOS_HISTORICOS.md', section: 'Cursos anteriores e espaços de apoio', observedAt: '2026-10-09' };
const FONTE_BOLETIM = { file: 'REGISTRO_ACADEMICO.md', section: 'Boletim oficial e frequência — 2026.2', observedAt: '2026-10-10' };
const FONTE_CANVAS_2026_1 = { file: 'REGISTRO_ACADEMICO.md', section: 'Inscrições anteriores no Canvas — 2026.1', observedAt: '2026-10-10' };
const FONTE_PORTAL_DOCENTE = { file: 'REGISTRO_ACADEMICO.md', section: 'Disciplinas matriculadas em 2026.2', observedAt: '2026-10-10' };

// ---------- 1. Instituição, programas, períodos ----------
escrever('instituicao/uva.json', {
  id: 'uva',
  title: 'Universidade Veiga de Almeida (UVA)',
  source: { file: 'REGISTRO_ACADEMICO.md', section: 'Resumo atual', observedAt: '2026-10-10' },
  pendencias: [],
});

const programasAntigos = [
  ['ciencia-da-computacao', 'uva-ciencia-computacao.json'],
  ['engenharia-de-computacao', 'uva-engenharia-computacao.json'],
];
const curriculosAntigos = {};
for (const [slug, arquivo] of programasAntigos) {
  const c = ler(`curriculos/${arquivo}`);
  curriculosAntigos[slug] = c;
  const curriculo = {
    id: c.id,
    curriculumVersion: c.curriculumVersion,
    ...(c.curriculumVersionConfidence !== undefined ? { curriculumVersionConfidence: c.curriculumVersionConfidence } : {}),
    trajectoryStatus: c.trajectoryStatus,
    source: c.source,
    limitations: c.limitations,
  };
  escrever(`programas/${slug}.json`, {
    id: c.programId,
    institutionId: c.institutionId,
    programTitle: c.programTitle,
    programCode: c.programCode,
    curriculo,
    source: c.source,
    pendencias: [],
  });
}

escrever('periodos/2026.1.json', {
  id: '2026.1',
  situacao: 'encerrado',
  source: FONTE_2026_1,
  pendencias: [],
});
escrever('periodos/2026.2.json', {
  id: '2026.2',
  situacao: 'em_andamento',
  source: FONTE_2026_2_MATRICULAS,
  pendencias: [],
});

// ---------- 2. Cursos do Canvas ----------
const canvasRegistros = [
  {
    id: '57352', titulo: 'Cálculo de Uma Variável', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'E10100002', secao: 'E10100002-21PRD3262A-2026.2.21', source: FONTE_2026_2_FAVORITOS,
    pendencias: [],
  },
  {
    id: '55558', titulo: 'Bases de Programação', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'E10300000', secao: null, source: FONTE_2026_2_MATRICULAS,
    observacao: "Canvas mostra o curso em 'Inscrições anteriores', com período 2026.2.21.",
    pendencias: [
      { campo: 'codigo', motivo: 'código aparece só na tabela de notas do REGISTRO_ACADEMICO.md; fora da sequência E10300003–E10300055; não confirmado no Canvas' },
      { campo: 'secao', motivo: 'seção Canvas não registrada; o Portal informa a turma 21CMP2262A' },
    ],
  },
  {
    id: '57904', titulo: 'Linguagens Formais e Autômatos', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'E10300019', secao: 'E10300019-21CMP2262A-2026.2.21', source: FONTE_2026_2_FAVORITOS,
    pendencias: [
      { campo: 'materiais', motivo: 'quatro unidades oficiais listadas no Canvas; PDFs não baixados (HTTP 403 no acesso direto); captura autenticada pendente' },
    ],
  },
  {
    id: '57159', titulo: 'Paradigmas de Linguagens de Programação', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'E10300024', secao: null, source: FONTE_2026_2_MATRICULAS,
    observacao: "Canvas mostra o índice 'Análise e Projeto de Sistemas e Informação I' (ASI1) e os PDFs de ASI1.",
    pendencias: [
      { campo: 'codigo', motivo: 'código aparece só na tabela de notas do REGISTRO_ACADEMICO.md; não confirmado no Canvas' },
      { campo: 'secao', motivo: 'seção Canvas não registrada; o Portal informa a turma 21CMP2262A' },
      { campo: 'materiais', motivo: 'PDFs do módulo são de ASI1; vínculo com esta disciplina pendente de confirmação pela UVA' },
    ],
  },
  {
    id: '57827', titulo: 'Estatística para Computação', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'E10300025', secao: 'E10300025-21CMP2262A-2026.2.21', source: FONTE_2026_2_FAVORITOS, pendencias: [],
  },
  {
    id: '57397', titulo: 'Projeto de Extensão I — Ciência da Computação', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'E10300026', secao: 'E10300026-21CMP2262A-2026.2.21', source: FONTE_2026_2_FAVORITOS, pendencias: [],
  },
  {
    id: '57897', titulo: 'Projeto de Extensão I (Engenharia Civil)', tipo: 'curricular', situacao: 'em_andamento', periodo: '2026.2',
    codigo: 'EP30000011', secao: 'EP30000011-21CIV2262A-2026.2.21', source: FONTE_2026_2_FAVORITOS,
    pendencias: [
      { campo: 'vinculo', motivo: 'turma de Engenharia Civil (21CIV2262A) listada como matrícula do estudante em Ciência da Computação; confirmar na secretaria' },
    ],
  },
  {
    id: '54293', titulo: 'Desenvolvimento de Aplicações para Web', tipo: 'curricular', situacao: 'encerrada', periodo: '2026.1',
    codigo: 'E10300023', secao: null, source: FONTE_2026_1,
    pendencias: [{ campo: 'secao', motivo: 'turma não registrada nas fontes de 2026.1' }],
  },
  {
    id: '53497', titulo: 'Interfaces de Sistemas Computacionais', tipo: 'curricular', situacao: 'encerrada', periodo: '2026.1',
    codigo: 'E10300008', secao: null, source: FONTE_2026_1,
    pendencias: [{ campo: 'secao', motivo: 'turma não registrada nas fontes de 2026.1' }],
  },
  {
    id: '54292', titulo: 'Laboratório de Gestão de Negócios', tipo: 'curricular', situacao: 'encerrada', periodo: '2026.1',
    codigo: 'E10300022', secao: null, source: FONTE_2026_1,
    pendencias: [{ campo: 'secao', motivo: 'turma não registrada nas fontes de 2026.1' }],
  },
  {
    id: '53442', titulo: 'Leitura e Produção Acadêmica', tipo: 'curricular', situacao: 'encerrada', periodo: '2026.1',
    codigo: 'E80200050', secao: null, source: FONTE_2026_1,
    pendencias: [{ campo: 'secao', motivo: 'turma não registrada nas fontes de 2026.1' }],
  },
  {
    id: '53498', titulo: 'Sistemas Computacionais e Aplicações', tipo: 'curricular', situacao: 'encerrada', periodo: '2026.1',
    codigo: 'E10300021', secao: null, source: FONTE_2026_1,
    pendencias: [{ campo: 'secao', motivo: 'turma não registrada nas fontes de 2026.1' }],
  },
  {
    id: '53973', titulo: 'Liderança e Empreendedorismo', tipo: 'gratuito', situacao: 'gratuito', periodo: null,
    codigo: null, secao: null, source: FONTE_HISTORICOS,
    observacao: 'Curso gratuito exibido em inscrições anteriores; não é disciplina curricular.',
    pendencias: [
      { campo: 'periodo', motivo: 'sem período letivo identificado no Canvas' },
      { campo: 'codigo', motivo: 'curso gratuito sem código de disciplina' },
      { campo: 'secao', motivo: 'não aplicável a curso gratuito' },
    ],
  },
  {
    id: '53418', titulo: 'Espaço Curso — Ciência da Computação', tipo: 'apoio', situacao: 'apoio', periodo: null,
    codigo: null, secao: null, source: FONTE_HISTORICOS,
    observacao: 'Espaço de apoio, não é matéria. Exibido com período 2023.1.21.',
    pendencias: [
      { campo: 'periodo', motivo: 'exibido com período 2023.1.21, que não é período deste acervo' },
      { campo: 'codigo', motivo: 'espaço de apoio sem código de disciplina' },
      { campo: 'secao', motivo: 'espaço de apoio sem seção' },
    ],
  },
];
for (const c of canvasRegistros) {
  escrever(`canvas/${c.id}.json`, { ...c, url: URL_CANVAS(c.id) });
}

// ---------- 3. Disciplinas (formato antigo + matérias de 2026.1/2026.2) ----------
const disciplinasAntigas = ler('disciplinas.json').disciplines;
const nomesUsados = new Set();
disciplinasAntigas.forEach((d, indice) => {
  const slug = slugDe(d.id);
  if (nomesUsados.has(slug)) throw new Error(`slug de disciplina duplicado: ${slug}`);
  nomesUsados.add(slug);
  escrever(`disciplinas/${slug}.json`, { ...d, ordem: indice, pendencias: [] });
});

const materiasNovas = [
  ['calculo-de-uma-variavel', 'Cálculo de Uma Variável', FONTE_2026_2_MATRICULAS],
  ['bases-de-programacao', 'Bases de Programação', FONTE_2026_2_MATRICULAS],
  ['linguagens-formais-e-automatos', 'Linguagens Formais e Autômatos', FONTE_2026_2_MATRICULAS],
  ['paradigmas-de-linguagens-de-programacao', 'Paradigmas de Linguagens de Programação', FONTE_2026_2_MATRICULAS],
  ['estatistica-para-computacao', 'Estatística para Computação', FONTE_2026_2_MATRICULAS],
  ['projeto-de-extensao-i-ciencia-da-computacao', 'Projeto de Extensão I - Ciência da Computação', FONTE_2026_2_MATRICULAS],
  ['projeto-de-extensao-i-engenharia-civil', 'Projeto de Extensão I (Engenharia Civil)', FONTE_2026_2_MATRICULAS],
  ['desenvolvimento-de-aplicacoes-para-web', 'Desenvolvimento de Aplicações para Web', FONTE_2026_1],
  ['interfaces-de-sistemas-computacionais', 'Interfaces de Sistemas Computacionais', FONTE_2026_1],
  ['laboratorio-de-gestao-de-negocios', 'Laboratório de Gestão de Negócios', FONTE_2026_1],
  ['leitura-e-producao-academica', 'Leitura e Produção Acadêmica', FONTE_2026_1],
  ['sistemas-computacionais-e-aplicacoes', 'Sistemas Computacionais e Aplicações', FONTE_2026_1],
];
for (const [slug, titulo, fonte] of materiasNovas) {
  escrever(`disciplinas/${slug}.json`, {
    id: `uva:discipline:${slug}`,
    title: titulo,
    offeringIds: [],
    identityStatus: 'matricula_registrada',
    source: fonte,
    pendencias: [],
  });
}

// ---------- 4. Ofertas curriculares (formato antigo) ----------
const programaPorId = Object.fromEntries(programasAntigos.map(([slug]) => [curriculosAntigos[slug].programId, slug]));
const usadosOferta = new Set();
for (const [slug] of programasAntigos) {
  const c = curriculosAntigos[slug];
  c.offerings.forEach((o, indice) => {
    const nome = slugDe(o.id);
    const chave = `${slug}/${nome}`;
    if (usadosOferta.has(chave)) throw new Error(`oferta duplicada: ${chave}`);
    usadosOferta.add(chave);
    escrever(`ofertas/${slug}/${nome}.json`, { ...o, programId: c.programId, ordem: indice, pendencias: [] });
  });
}

// ---------- 5. Relações ----------
const relacoesAntigas = ler('relacoes.json').relations;
relacoesAntigas.forEach((r, indice) => {
  escrever(`relacoes/${slugDe(r.id)}-${r.type.replace(/_/g, '-')}.json`, { ...r, ordem: indice, pendencias: [] });
});

// ---------- 6. Trajetórias, matrículas e notas ----------
const trajAntigas = ler('trajetorias.json');
escrever('meta.json', { trajetorias: { observedAt: trajAntigas.observedAt, notes: trajAntigas.notes } });

const CAMPOS_DERIVADOS = ['currentEnrollments', 'outcomes', 'priorCanvasCourses'];
const DOCENTE_CALCULO = 'Ronaldo Camara Cavalcante';
const DOCENTE_PARADIGMAS = 'Marco Aurelio Novaes Esteves';

// Disciplina e Canvas de cada matrícula de 2026.2, pelo título exibido no Portal.
const LIGACAO_2026_2 = {
  'Cálculo de Uma Variável': { disciplina: 'calculo-de-uma-variavel', canvas: '57352' },
  'Bases de Programação': { disciplina: 'bases-de-programacao', canvas: '55558' },
  'Linguagens Formais e Autômatos': { disciplina: 'linguagens-formais-e-automatos', canvas: '57904' },
  'Paradigmas de Linguagens de Programação': { disciplina: 'paradigmas-de-linguagens-de-programacao', canvas: '57159' },
  'Estatística para Computação': { disciplina: 'estatistica-para-computacao', canvas: '57827' },
  'Projeto de Extensão I - Ciência da Computação': { disciplina: 'projeto-de-extensao-i-ciencia-da-computacao', canvas: '57397' },
  'Projeto de Extensão I': { disciplina: 'projeto-de-extensao-i-engenharia-civil', canvas: '57897' },
};
const DOCENTE_POR_TITULO = {
  'Cálculo de Uma Variável': DOCENTE_CALCULO,
  'Paradigmas de Linguagens de Programação': DOCENTE_PARADIGMAS,
};
const PENDENCIA_CARGA = { campo: 'cargaHoraria', motivo: 'carga horária da disciplina não consta nos planos de ensino locais nem no Portal' };
const PENDENCIA_DOCENTE_A = { campo: 'docente', motivo: 'Portal exibe apenas "A" como docente; nome não encontrado nos planos de ensino locais' };

const PROGRAMA_CC = 'uva:program:ciencia-da-computacao';
const PROGRAMA_EC = 'uva:program:engenharia-de-computacao';
const trajCC = trajAntigas.trajectories.find((t) => t.programId === PROGRAMA_CC);
const trajEC = trajAntigas.trajectories.find((t) => t.programId === PROGRAMA_EC);

const matriculasCC2026_2 = trajCC.currentEnrollments.map((m, indice) => {
  const ligacao = LIGACAO_2026_2[m.title];
  if (!ligacao) throw new Error(`matrícula sem ligação: ${m.title}`);
  const docente = DOCENTE_POR_TITULO[m.title] ?? null;
  const pendencias = [PENDENCIA_CARGA];
  if (docente === null) pendencias.push(PENDENCIA_DOCENTE_A);
  if (ligacao.canvas === '57897') {
    pendencias.push({ campo: 'vinculo', motivo: 'turma de Engenharia Civil listada como matrícula do estudante em Ciência da Computação; confirmar na secretaria' });
  }
  return {
    id: m.id,
    programId: PROGRAMA_CC,
    semester: '2026.2',
    title: m.title,
    section: m.section,
    startDate: m.startDate,
    instructorAsDisplayed: m.instructorAsDisplayed,
    status: 'enrolled',
    ordem: indice,
    disciplinaId: `uva:discipline:${ligacao.disciplina}`,
    canvasCourseId: ligacao.canvas,
    docente,
    cargaHoraria: null,
    source: m.source,
    pendencias,
  };
});

// Cursos de 2026.1 (inscrições anteriores no Canvas), a partir de priorCanvasCourses.
const MAPA_2026_1 = {
  'Desenvolvimento de Aplicações para Web': { disciplina: 'desenvolvimento-de-aplicacoes-para-web', canvas: '54293' },
  'Interfaces de Sistemas Computacionais': { disciplina: 'interfaces-de-sistemas-computacionais', canvas: '53497' },
  'Laboratório de Gestão de Negócios': { disciplina: 'laboratorio-de-gestao-de-negocios', canvas: '54292' },
  'Leitura e Produção Acadêmica': { disciplina: 'leitura-e-producao-academica', canvas: '53442' },
  'Sistemas Computacionais e Aplicações': { disciplina: 'sistemas-computacionais-e-aplicacoes', canvas: '53498' },
};
const matriculas2026_1 = trajCC.priorCanvasCourses.map((p, indice) => {
  const ligacao = MAPA_2026_1[p.courseTitle];
  if (!ligacao) throw new Error(`curso anterior sem ligação: ${p.courseTitle}`);
  return {
    id: `uva:program:ciencia-da-computacao:enrollment:2026.1:${ligacao.disciplina}`,
    programId: PROGRAMA_CC,
    semester: '2026.1',
    title: p.courseTitle,
    section: null,
    status: 'closed',
    ordem: indice,
    disciplinaId: `uva:discipline:${ligacao.disciplina}`,
    canvasCourseId: ligacao.canvas,
    docente: null,
    cargaHoraria: null,
    source: FONTE_2026_1,
    pendencias: [
      { campo: 'section', motivo: 'turma não registrada nas fontes de 2026.1' },
      { campo: 'docente', motivo: 'docente não registrado nas fontes de 2026.1' },
      { campo: 'cargaHoraria', motivo: 'carga horária não consta nas fontes de 2026.1' },
    ],
  };
});

// Trajetória de Ciência da Computação: matrículas/notas viram arquivos próprios.
const trajetoriaCC = { ...trajCC };
for (const campo of CAMPOS_DERIVADOS) delete trajetoriaCC[campo];
escrever('trajetorias/ciencia-da-computacao.json', { ...trajetoriaCC, pendencias: [] });

for (const m of [...matriculasCC2026_2, ...matriculas2026_1]) {
  const periodo = m.semester;
  escrever(`matriculas/${periodo}/${slugDe(m.disciplinaId)}.json`, m);
}

// Notas de 2026.2 (Lyceum + Canvas) a partir de outcomes.
trajCC.outcomes.forEach((n, indice) => {
  const matricula = matriculasCC2026_2.find((m) => m.title === n.courseTitle);
  if (!matricula) throw new Error(`nota sem matrícula: ${n.courseTitle}`);
  const pendencias = [];
  if (n.lyceumAverage === null) pendencias.push({ campo: 'lyceumAverage', motivo: 'média ainda não publicada no Lyceum em 10/10/2026' });
  const registro = {
    id: `uva:program:ciencia-da-computacao:nota:2026.2:${slugDe(matricula.disciplinaId)}`,
    matriculaId: matricula.id,
    ordem: indice,
    courseTitle: n.courseTitle,
    status: n.status,
    lyceumGrades: n.lyceumGrades,
    lyceumAverage: n.lyceumAverage,
    absences: n.absences,
    attendancePercent: n.attendancePercent,
    ...(n.canvasAvaTotals ? { canvasAvaTotals: n.canvasAvaTotals } : {}),
    source: FONTE_BOLETIM,
    pendencias,
  };
  escrever(`notas/2026.2/${slugDe(matricula.disciplinaId)}.json`, registro);
});

// Notas de 2026.1 (somente totais do Canvas; o Lyceum não tem decisão localizada).
trajCC.priorCanvasCourses.forEach((p, indice) => {
  const ligacao = MAPA_2026_1[p.courseTitle];
  const matricula = matriculas2026_1.find((m) => m.disciplinaId === `uva:discipline:${ligacao.disciplina}`);
  escrever(`notas/2026.1/${ligacao.disciplina}.json`, {
    id: `uva:program:ciencia-da-computacao:nota:2026.1:${ligacao.disciplina}`,
    matriculaId: matricula.id,
    ordem: indice,
    courseTitle: p.courseTitle,
    status: 'closed',
    canvasAvaTotals: { earnedPoints: p.earnedPoints, possiblePoints: p.possiblePoints, percent: p.percent },
    source: FONTE_CANVAS_2026_1,
    pendencias: [{ campo: 'lyceum', motivo: 'boletim do Lyceum sem decisão final localizada para 2026.1' }],
  });
});

// Trajetória de Engenharia de Computação: vínculo evadido, com o contexto do estudante.
const trajetoriaEC = { ...trajEC };
for (const campo of CAMPOS_DERIVADOS) delete trajetoriaEC[campo];
escrever('trajetorias/engenharia-de-computacao.json', {
  ...trajetoriaEC,
  contexto: [
    'O estudante já cursou 1.446 h de Engenharia de Computação (Histórico Detalhado do Portal).',
    'O Portal mostra o vínculo como "Evadido", sem data nem motivo. O rótulo não é conclusão de aproveitamento.',
    'Objetivo do estudante: verificar o aproveitamento das disciplinas já cursadas em Ciência da Computação. Disciplinas de mesmo nome estão em relacoes/, com equivalenceConfirmed = false.',
    'Histórico Escolar com CR aparece no catálogo da Secretaria Virtual (R$ 0,00). Não foi solicitado.',
  ],
  pendencias: [
    { campo: 'disciplinasCursadas', motivo: 'as disciplinas que compõem as 1.446 h cursadas não aparecem nas telas observadas' },
    { campo: 'notas', motivo: 'boletim, frequência e índice de rendimento de Engenharia não apresentaram registros no Portal' },
    { campo: 'aproveitamento', motivo: 'nenhuma equivalência confirmada; depende de ementas ou documento oficial da UVA' },
    { campo: 'capturaPortal', motivo: 'capturar o histórico e as disciplinas cursadas deste vínculo, após login do estudante no Portal ou pelo app' },
  ],
});

// ---------- 7. Trabalhos (AVA) ----------
const trabalhos = [
  {
    slug: 'calculo-ava1-custo-marginal',
    titulo: 'AVA 1 — Custo marginal, pontos críticos e segunda derivada',
    arquivo: 'materias/calculo-de-uma-variavel/ava1-custo-marginal-pontos-criticos.md',
    disciplinaId: 'uva:discipline:calculo-de-uma-variavel', canvasCourseId: '57352', periodo: '2026.2',
    fonte: { file: 'materias/calculo-de-uma-variavel/ava1-custo-marginal-pontos-criticos.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [{ campo: 'periodo', motivo: 'período derivado do curso Canvas 57352 (2026.2); o cabeçalho do arquivo não informa período' }],
  },
  {
    slug: 'estatistica-ava1-amostra-populacao',
    titulo: 'AVA 1 — Amostra versus população: métodos de pesquisa e análise estatística',
    arquivo: 'materias/estatistica-para-computacao/ava1-amostra-populacao-analise-estatistica.md',
    disciplinaId: 'uva:discipline:estatistica-para-computacao', canvasCourseId: '57827', periodo: '2026.2',
    fonte: { file: 'materias/estatistica-para-computacao/ava1-amostra-populacao-analise-estatistica.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [{ campo: 'periodo', motivo: 'período derivado do curso Canvas 57827 (2026.2); o cabeçalho do arquivo não informa período' }],
  },
  {
    slug: 'bases-ava1-apuracao-votos',
    titulo: 'AVA 1 — Apuração de votos em processo eleitoral empresarial',
    arquivo: 'materias/bases-de-programacao/ava1-apuracao-de-votos.md',
    disciplinaId: 'uva:discipline:bases-de-programacao', canvasCourseId: null, periodo: null,
    fonte: { file: 'materias/bases-de-programacao/ava1-apuracao-de-votos.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [
      { campo: 'canvasCourseId', motivo: 'cabeçalho não informa o curso Canvas' },
      { campo: 'periodo', motivo: 'cabeçalho não informa o período' },
    ],
  },
  {
    slug: 'bases-ava2-monitoramento-esteira',
    titulo: 'AVA 2 — Monitoramento de esteira de transporte com sensor',
    arquivo: 'materias/bases-de-programacao/ava2-monitoramento-de-esteira.md',
    disciplinaId: 'uva:discipline:bases-de-programacao', canvasCourseId: null, periodo: null,
    fonte: { file: 'materias/bases-de-programacao/ava2-monitoramento-de-esteira.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [
      { campo: 'canvasCourseId', motivo: 'cabeçalho não informa o curso Canvas' },
      { campo: 'periodo', motivo: 'cabeçalho não informa o período' },
    ],
  },
  {
    slug: 'paradigmas-ava1-caso-uso-saque',
    titulo: 'AVA 1 — Descrição de caso de uso CSU01: realizar saque bancário em caixa eletrônico',
    arquivo: 'materias/paradigmas-de-programacao/ava1-caso-de-uso-saque.md',
    disciplinaId: 'uva:discipline:paradigmas-de-linguagens-de-programacao', canvasCourseId: null, periodo: null,
    fonte: { file: 'materias/paradigmas-de-programacao/ava1-caso-de-uso-saque.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [
      { campo: 'disciplinaId', motivo: 'cabeçalho diz "Paradigmas de Programação"; ligado a Paradigmas de Linguagens por nome aproximado' },
      { campo: 'canvasCourseId', motivo: 'cabeçalho não informa o curso Canvas' },
      { campo: 'periodo', motivo: 'cabeçalho não informa o período' },
    ],
  },
  {
    slug: 'paradigmas-ava2-modelagem-uml-biblioteca',
    titulo: 'AVA 2 — Modelagem UML para o sistema de empréstimos e reservas de biblioteca universitária',
    arquivo: 'materias/paradigmas-de-programacao/ava2-modelagem-uml-biblioteca.md',
    disciplinaId: 'uva:discipline:paradigmas-de-linguagens-de-programacao', canvasCourseId: null, periodo: null,
    fonte: { file: 'materias/paradigmas-de-programacao/ava2-modelagem-uml-biblioteca.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [
      { campo: 'disciplinaId', motivo: 'cabeçalho diz "Paradigmas de Programação"; ligado a Paradigmas de Linguagens por nome aproximado' },
      { campo: 'canvasCourseId', motivo: 'cabeçalho não informa o curso Canvas' },
      { campo: 'periodo', motivo: 'cabeçalho não informa o período' },
    ],
  },
  {
    slug: 'lfa-ava1-afd-termina-em-1',
    titulo: 'AVA 1 — Autômato Finito Determinístico para cadeias binárias terminadas em 1',
    arquivo: 'materias/linguagens-formais-e-automatos/ava1-automato-finito-deterministico.md',
    disciplinaId: 'uva:discipline:linguagens-formais-e-automatos', canvasCourseId: '57904', periodo: '2026.2',
    fonte: { file: 'materias/linguagens-formais-e-automatos/ava1-automato-finito-deterministico.md', section: 'Cabeçalho do trabalho', observedAt: '2026-11-06' },
    pendencias: [{ campo: 'periodo', motivo: 'período derivado do curso Canvas 57904 (2026.2); o cabeçalho do arquivo não informa período' }],
  },
];
for (const t of trabalhos) {
  const { slug, fonte, ...resto } = t;
  escrever(`trabalhos/${slug}.json`, { id: `uva:trabalho:${slug}`, ...resto, source: fonte });
}

console.log('dados/ gerado a partir de', origem);
if (semNulos.length) console.log(semNulos);
