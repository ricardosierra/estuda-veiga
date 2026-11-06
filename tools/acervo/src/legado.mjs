// Gera, a partir de dados/, os JSON no formato antigo que o app Estuda UVA lê
// (materias/catalogo/*.json). São arquivos gerados: não editar à mão.
const ARQUIVO_LEGADO = {
  'ciencia-da-computacao': 'uva-ciencia-computacao.json',
  'engenharia-de-computacao': 'uva-engenharia-computacao.json',
};

export const SAIDA_LEGADO = 'materias/catalogo';

const ordenar = (lista) =>
  [...lista].sort(
    (a, b) => (a.ordem ?? 0) - (b.ordem ?? 0) || String(a.id).localeCompare(String(b.id)),
  );

function sem(objeto, chaves) {
  const copia = { ...objeto };
  for (const chave of chaves) delete copia[chave];
  return copia;
}

// Retorna { caminhoRelativo: conteudoObjeto } para os cinco arquivos do formato antigo.
export function gerarLegado(registros) {
  const porTipo = (tipo) => registros.filter((r) => r.tipo === tipo);
  const dadosDe = (tipo) => porTipo(tipo).map((r) => r.dados);

  const programas = porTipo('programas').sort((a, b) => a.nome.localeCompare(b.nome));
  const ofertas = dadosDe('ofertas');
  const disciplinas = dadosDe('disciplinas');
  const relacoes = dadosDe('relacoes');
  const trajetorias = dadosDe('trajetorias');
  const matriculas = dadosDe('matriculas');
  const notas = dadosDe('notas');
  const canvas = new Map(dadosDe('canvas').map((c) => [c.id, c]));
  const matriculaPorId = new Map(matriculas.map((m) => [m.id, m]));
  const meta = dadosDe('meta')[0]?.trajetorias ?? {};

  const saida = {};

  for (const { nome, dados: programa } of programas) {
    const curriculo = programa.curriculo;
    const ofertasDoPrograma = ordenar(ofertas.filter((o) => o.programId === programa.id)).map((o) =>
      sem(o, ['programId', 'ordem', 'pendencias']),
    );
    const trajetoria = trajetorias.find((t) => t.programId === programa.id);
    const semestre = trajetoria?.semester;
    const ativas = ordenar(
      matriculas.filter((m) => m.programId === programa.id && m.semester === semestre && m.status === 'enrolled'),
    ).map((m) => sem(m, ['ordem', 'pendencias']));

    const arquivo = ARQUIVO_LEGADO[nome] ?? `uva-${nome}.json`;
    saida[`${SAIDA_LEGADO}/curriculos/${arquivo}`] = {
      schemaVersion: 1,
      id: curriculo.id,
      institutionId: programa.institutionId,
      programId: programa.id,
      programTitle: programa.programTitle,
      programCode: programa.programCode,
      curriculumVersion: curriculo.curriculumVersion,
      ...(curriculo.curriculumVersionConfidence !== undefined
        ? { curriculumVersionConfidence: curriculo.curriculumVersionConfidence }
        : {}),
      trajectoryStatus: curriculo.trajectoryStatus,
      offerings: ofertasDoPrograma,
      activeEnrollments: ativas,
      source: curriculo.source,
      limitations: curriculo.limitations,
    };
  }

  const trajetoriasLegado = trajetorias
    .sort((a, b) => a.id.localeCompare(b.id))
    .map((trajetoria) => {
      const semestre = trajetoria.semester;
      const matriculasDoSemestre = ordenar(
        matriculas.filter((m) => m.programId === trajetoria.programId && m.semester === semestre),
      );
      const outcomes = [];
      const priorCanvasCourses = [];
      for (const nota of ordenar(notas)) {
        const matricula = matriculaPorId.get(nota.matriculaId);
        if (!matricula || matricula.programId !== trajetoria.programId) continue;
        const cursoCanvas = canvas.get(matricula.canvasCourseId);
        if (matricula.semester === semestre) {
          const item = sem(nota, ['id', 'matriculaId', 'ordem', 'source', 'pendencias']);
          if (nota.canvasAvaTotals && cursoCanvas) item.canvasCourseCode = cursoCanvas.codigo;
          outcomes.push(item);
        } else if (nota.canvasAvaTotals) {
          priorCanvasCourses.push({
            courseTitle: nota.courseTitle,
            canvasCode: cursoCanvas?.codigo ?? null,
            earnedPoints: nota.canvasAvaTotals.earnedPoints,
            possiblePoints: nota.canvasAvaTotals.possiblePoints,
            percent: nota.canvasAvaTotals.percent,
          });
        }
      }
      return {
        ...sem(trajetoria, ['pendencias']),
        currentEnrollments: matriculasDoSemestre.map((m) => sem(m, ['ordem', 'pendencias'])),
        outcomes,
        priorCanvasCourses,
      };
    });

  saida[`${SAIDA_LEGADO}/disciplinas.json`] = {
    schemaVersion: 1,
    disciplines: ordenar(disciplinas).map((d) => sem(d, ['ordem', 'pendencias'])),
  };
  saida[`${SAIDA_LEGADO}/relacoes.json`] = {
    schemaVersion: 1,
    relations: ordenar(relacoes).map((r) => sem(r, ['ordem', 'pendencias'])),
  };
  saida[`${SAIDA_LEGADO}/trajetorias.json`] = {
    schemaVersion: 1,
    observedAt: meta.observedAt,
    trajectories: trajetoriasLegado,
    notes: meta.notes,
  };
  return saida;
}
