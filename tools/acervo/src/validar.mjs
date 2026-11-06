// Valida os registros de dados/ antes de qualquer geração.
// Regras: campos obrigatórios, valores permitidos, nulos sempre com pendência,
// referências entre registros, fontes existentes, nada sensível.
import { existsSync } from 'node:fs';
import { join } from 'node:path';

export const URL_CANVAS = (id) => `https://uva.instructure.com/courses/${id}`;

const ESPECIFICACAO = {
  instituicao: { obrigatorios: ['id', 'title', 'source'] },
  programas: {
    obrigatorios: ['id', 'institutionId', 'programTitle', 'programCode', 'curriculo', 'source'],
  },
  periodos: {
    obrigatorios: ['id', 'situacao', 'source'],
    valores: { situacao: ['encerrado', 'em_andamento', 'previsto'] },
  },
  canvas: {
    obrigatorios: ['id', 'titulo', 'url', 'tipo', 'situacao', 'periodo', 'codigo', 'secao', 'source'],
    valores: {
      tipo: ['curricular', 'apoio', 'gratuito'],
      situacao: ['em_andamento', 'encerrada', 'apoio', 'gratuito'],
    },
  },
  disciplinas: { obrigatorios: ['id', 'title', 'offeringIds', 'identityStatus', 'source'] },
  ofertas: {
    obrigatorios: ['id', 'programId', 'code', 'title', 'workloadHours', 'status', 'canonicalDisciplineId', 'source'],
  },
  relacoes: {
    // canonicalDisciplineId é opcional: relações de sobreposição parcial não o têm no formato antigo.
    obrigatorios: ['id', 'type', 'ccOfferingId', 'ecOfferingId', 'equivalenceConfirmed', 'sourceFiles'],
  },
  trajetorias: { obrigatorios: ['id', 'programId', 'status', 'source'] },
  matriculas: {
    obrigatorios: ['id', 'programId', 'semester', 'title', 'section', 'status', 'disciplinaId', 'canvasCourseId', 'docente', 'cargaHoraria', 'source'],
    valores: { status: ['enrolled', 'closed'] },
  },
  notas: {
    obrigatorios: ['id', 'matriculaId', 'ordem', 'courseTitle', 'status', 'source'],
    valores: { status: ['in_progress', 'closed'] },
  },
  trabalhos: {
    obrigatorios: ['id', 'titulo', 'arquivo', 'disciplinaId', 'canvasCourseId', 'periodo', 'source'],
  },
  meta: { obrigatorios: [] },
};

const CHAVES_PROIBIDAS = /^(cpf|rg|ra|senha|password|pass|cookie|cookies|token|tokens|secret|credencial|credenciais|email|e-mail|telefone|login|usuario|usuário)$/i;
const VALORES_PROIBIDOS = /\/Users\/|\/home\/|[A-Za-z]:\\|Bearer |[\w.+-]+@[\w-]+\.[a-z]{2,}/i;

function percorrerValores(valor, visitar, caminho = '') {
  if (Array.isArray(valor)) {
    valor.forEach((item, indice) => percorrerValores(item, visitar, `${caminho}[${indice}]`));
  } else if (valor && typeof valor === 'object') {
    for (const [chave, sub] of Object.entries(valor)) {
      visitar(chave, sub, caminho ? `${caminho}.${chave}` : chave);
      percorrerValores(sub, visitar, caminho ? `${caminho}.${chave}` : chave);
    }
  }
}

export function validar({ raiz, registros, erros: errosLeitura = [] }) {
  const erros = [...errosLeitura];
  const erro = (registro, campo, motivo) =>
    erros.push({ caminho: registro?.caminho ?? '', campo, motivo });

  // Índices de ids por tipo.
  const ids = {};
  for (const tipo of Object.keys(ESPECIFICACAO)) ids[tipo] = new Set();
  const duplicados = [];
  for (const r of registros) {
    if (!ESPECIFICACAO[r.tipo] || !r.dados || typeof r.dados.id !== 'string') continue;
    if (ids[r.tipo].has(r.dados.id)) duplicados.push(r);
    ids[r.tipo].add(r.dados.id);
  }
  for (const r of duplicados) erro(r, 'id', `id duplicado: ${r.dados.id}`);

  const existe = (tipo, id, registro, campo) => {
    if (id === null || id === undefined) return;
    if (!ids[tipo]?.has(id)) erro(registro, campo, `referência a ${tipo} inexistente: ${id}`);
  };

  for (const r of registros) {
    const spec = ESPECIFICACAO[r.tipo];
    if (!spec) {
      erro(r, null, `pasta/tipo não reconhecido: '${r.tipo}'`);
      continue;
    }
    const d = r.dados;
    if (!d || typeof d !== 'object' || Array.isArray(d)) {
      erro(r, null, 'o arquivo deve conter um objeto JSON');
      continue;
    }

    for (const campo of spec.obrigatorios) {
      if (!(campo in d)) erro(r, campo, 'campo obrigatório ausente');
    }
    for (const [campo, permitidos] of Object.entries(spec.valores ?? {})) {
      if (campo in d && d[campo] !== null && !permitidos.includes(d[campo])) {
        erro(r, campo, `valor '${d[campo]}' fora de: ${permitidos.join(', ')}`);
      }
    }

    // Regra central: ausência de dado é null + pendência explícita. Nunca "" nem zero inventado.
    const pendencias = Array.isArray(d.pendencias) ? d.pendencias : [];
    if ('pendencias' in d && !Array.isArray(d.pendencias)) erro(r, 'pendencias', 'deve ser uma lista');
    for (const p of pendencias) {
      if (!p || typeof p.campo !== 'string' || typeof p.motivo !== 'string' || !p.motivo.trim()) {
        erro(r, 'pendencias', 'cada pendência precisa de campo e motivo');
      }
    }
    for (const [campo, valor] of Object.entries(d)) {
      if (campo === 'pendencias') continue;
      if (valor === null && !pendencias.some((p) => p.campo === campo)) {
        erro(r, campo, 'valor nulo sem pendência correspondente (registre em pendencias)');
      }
      if (valor === '') erro(r, campo, 'string vazia: use null e registre pendência');
    }

    percorrerValores(d, (chave, valor, caminho) => {
      if (CHAVES_PROIBIDAS.test(chave)) erro(r, caminho, 'chave sensível não permitida no repositório');
      if (typeof valor === 'string' && VALORES_PROIBIDOS.test(valor)) {
        erro(r, caminho, 'valor sensível ou caminho absoluto não permitido');
      }
    });

    // Fontes: arquivos citados precisam existir no repositório.
    const arquivos = [];
    if (d.source && typeof d.source.file === 'string') arquivos.push(d.source.file);
    if (Array.isArray(d.sourceFiles)) arquivos.push(...d.sourceFiles);
    if (typeof d.arquivo === 'string') arquivos.push(d.arquivo);
    for (const arquivo of arquivos) {
      if (!existsSync(join(raiz, arquivo))) erro(r, 'source', `arquivo citado não existe: ${arquivo}`);
    }

    // Referências.
    switch (r.tipo) {
      case 'canvas':
        if (d.url !== URL_CANVAS(d.id)) erro(r, 'url', `url deve ser ${URL_CANVAS(d.id)}`);
        if (r.nome !== d.id) erro(r, 'id', 'nome do arquivo deve ser o id do curso Canvas');
        existe('periodos', d.periodo, r, 'periodo');
        break;
      case 'programas':
        break;
      case 'ofertas':
        existe('programas', d.programId, r, 'programId');
        existe('disciplinas', d.canonicalDisciplineId, r, 'canonicalDisciplineId');
        break;
      case 'disciplinas':
        for (const oferta of d.offeringIds ?? []) existe('ofertas', oferta, r, 'offeringIds');
        break;
      case 'relacoes':
        existe('ofertas', d.ccOfferingId, r, 'ccOfferingId');
        existe('ofertas', d.ecOfferingId, r, 'ecOfferingId');
        existe('disciplinas', d.canonicalDisciplineId, r, 'canonicalDisciplineId');
        break;
      case 'trajetorias':
        existe('programas', d.programId, r, 'programId');
        break;
      case 'matriculas':
        existe('programas', d.programId, r, 'programId');
        existe('periodos', d.semester, r, 'semester');
        existe('disciplinas', d.disciplinaId, r, 'disciplinaId');
        existe('canvas', d.canvasCourseId, r, 'canvasCourseId');
        break;
      case 'notas':
        existe('matriculas', d.matriculaId, r, 'matriculaId');
        break;
      case 'trabalhos':
        existe('disciplinas', d.disciplinaId, r, 'disciplinaId');
        existe('canvas', d.canvasCourseId, r, 'canvasCourseId');
        existe('periodos', d.periodo, r, 'periodo');
        break;
      default:
        break;
    }
  }

  return erros;
}
