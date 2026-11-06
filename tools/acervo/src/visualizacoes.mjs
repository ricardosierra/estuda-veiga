// Gera Markdown de leitura rápida a partir do SQLite de build (consultas às views).
// Os arquivos ficam em build/visualizacoes/ e não são versionados.
import { mkdirSync, rmSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { DatabaseSync } from 'node:sqlite';

const celula = (v) => {
  if (v === null || v === undefined) return '—';
  return String(v).replace(/\|/g, '\\|').replace(/\n/g, ' ');
};

function tabela(cabecalho, linhas) {
  const corpo = linhas.length
    ? linhas.map((linha) => `| ${linha.map(celula).join(' | ')} |`)
    : [`| ${cabecalho.map(() => '—').join(' | ')} |`];
  return [`| ${cabecalho.join(' | ')} |`, `| ${cabecalho.map(() => '---').join(' | ')} |`, ...corpo].join('\n');
}

const porPeriodo = (linhas) => {
  const grupos = new Map();
  for (const linha of linhas) {
    const chave = linha.periodo ?? 'sem período';
    if (!grupos.has(chave)) grupos.set(chave, []);
    grupos.get(chave).push(linha);
  }
  return grupos;
};

export function gerarVisualizacoes({ caminhoSqlite, pasta }) {
  rmSync(pasta, { recursive: true, force: true });
  mkdirSync(pasta, { recursive: true });
  const db = new DatabaseSync(caminhoSqlite);
  const consulta = (sql) => db.prepare(sql).all();
  const escrever = (nome, conteudo) => writeFileSync(join(pasta, nome), conteudo.endsWith('\n') ? conteudo : `${conteudo}\n`);

  try {
    // Matrículas por período, com Canvas, docente, carga e notas.
    const matriculas = consulta('SELECT * FROM v_matriculas ORDER BY periodo, programa, disciplina');
    let md = '# Matrículas\n\nGerado a partir de `dados/`. Não editar: altere os arquivos em `dados/`.\n\n';
    for (const [periodo, linhas] of porPeriodo(matriculas)) {
      md += `## ${periodo}\n\n`;
      md += tabela(
        ['Disciplina', 'Canvas', 'Código', 'Seção Canvas', 'Turma', 'Docente', 'Docente exibido no Portal', 'Início', 'Situação', 'Carga (h)', 'Média Lyceum', 'Faltas', 'Freq. %', 'Total Canvas %'],
        linhas.map((l) => [
          l.disciplina ?? l.titulo, l.canvas_id, l.codigo, l.canvas_secao, l.turma, l.docente,
          l.docente_exibido, l.inicio, l.situacao, l.carga_horaria, l.media, l.faltas,
          l.frequencia_percent, l.canvas_percent,
        ]),
      );
      md += '\n\n';
    }
    escrever('matriculas.md', md);

    // Cursos do Canvas.
    const cursos = consulta('SELECT * FROM v_cursos ORDER BY tipo, periodo, id');
    md = '# Cursos do Canvas conhecidos\n\n';
    md += tabela(
      ['ID', 'Título', 'Tipo', 'Situação', 'Período', 'Código', 'Seção', 'URL', 'Matrículas'],
      cursos.map((c) => [c.id, c.titulo, c.tipo, c.situacao, c.periodo, c.codigo, c.secao, c.url, c.matriculas]),
    );
    escrever('cursos-canvas.md', md + '\n');

    // Pendências: tudo que ainda falta confirmar.
    const pendencias = consulta('SELECT * FROM v_pendencias');
    md = `# Pendências (${pendencias.length})\n\nValores nulos e informações a confirmar, com o motivo registrado.\n\n`;
    md += tabela(
      ['Entidade', 'Id', 'Campo', 'Motivo', 'Arquivo'],
      pendencias.map((p) => [p.entidade, p.id, p.campo, p.motivo, p.arquivo]),
    );
    escrever('pendencias.md', md + '\n');

    // Currículo (ofertas) por programa.
    const ofertas = consulta('SELECT * FROM v_curriculo');
    md = '# Ofertas curriculares\n\n';
    const programas = [...new Set(ofertas.map((o) => o.programa_id))];
    for (const programa of programas) {
      const linhas = ofertas.filter((o) => o.programa_id === programa);
      const total = linhas.reduce((soma, o) => soma + (o.carga_horaria ?? 0), 0);
      md += `## ${programa} (${linhas.length} itens, ${total} h somadas)\n\n`;
      md += tabela(
        ['Código', 'Título', 'Carga (h)', 'Situação', 'Disciplina'],
        linhas.map((o) => [o.codigo, o.titulo, o.carga_horaria, o.situacao, o.disciplina_id]),
      );
      md += '\n\n';
    }
    escrever('curriculos.md', md);

    // Notas e frequência detalhadas (inclui avaliações do Lyceum).
    const notas = consulta('SELECT n.*, m.periodo FROM notas n LEFT JOIN matriculas m ON m.id = n.matricula_id ORDER BY m.periodo, n.ordem');
    const avaliacoes = consulta('SELECT * FROM avaliacoes_lyceum ORDER BY matricula_id, ordem');
    md = '# Notas e frequência\n\n';
    md += tabela(
      ['Período', 'Matrícula', 'Curso', 'Situação', 'Média Lyceum', 'Faltas', 'Freq. %', 'Canvas pontos', 'Canvas possíveis', 'Canvas %'],
      notas.map((n) => [n.periodo, n.matricula_id, n.curso_titulo, n.situacao, n.media, n.faltas, n.frequencia_percent, n.canvas_pontos, n.canvas_possiveis, n.canvas_percent]),
    );
    md += '\n\n## Avaliações do Lyceum\n\n';
    md += tabela(
      ['Matrícula', 'Avaliação', 'Nota', 'Escala', 'Data'],
      avaliacoes.map((a) => [a.matricula_id, a.avaliacao, a.nota, a.escala, a.data]),
    );
    escrever('notas.md', md + '\n');

    // Trajetórias: situação de cada vínculo e o contexto registrado com fontes.
    const trajetorias = consulta('SELECT id, situacao, periodo, contexto FROM trajetorias ORDER BY id');
    md = '# Trajetórias (vínculos com cursos)\n\nCada vínculo tem situação própria. Contexto e pendências vêm de `dados/trajetorias/`.\n\n';
    for (const t of trajetorias) {
      const contexto = t.contexto ? JSON.parse(t.contexto) : [];
      md += `## ${t.id}\n\nSituação: **${t.situacao}**${t.periodo ? ` · período ${t.periodo}` : ''}\n\n`;
      md += contexto.length ? contexto.map((linha) => `- ${linha}`).join('\n') : '_Sem contexto registrado._';
      md += '\n\n';
    }
    escrever('trajetorias.md', md);

    // Índice.
    const contagens = {
      matriculas: consulta('SELECT COUNT(*) AS n FROM matriculas')[0].n,
      cursos_canvas: consulta('SELECT COUNT(*) AS n FROM cursos_canvas')[0].n,
      disciplinas: consulta('SELECT COUNT(*) AS n FROM disciplinas')[0].n,
      ofertas_curriculares: consulta('SELECT COUNT(*) AS n FROM ofertas_curriculares')[0].n,
      pendencias: pendencias.length,
    };
    md = '# Visualizações do acervo\n\nGeradas por `node tools/acervo/build.mjs`. Não editar; altere `dados/`.\n\n';
    md += tabela(['Tabela', 'Registros'], Object.entries(contagens).map(([k, v]) => [k, v]));
    md += '\n\n- [Matrículas](matriculas.md)\n- [Cursos do Canvas](cursos-canvas.md)\n- [Pendências](pendencias.md)\n- [Currículos](curriculos.md)\n- [Notas e frequência](notas.md)\n- [Trajetórias](trajetorias.md)\n';
    escrever('LEIA-ME.md', md);
    return Object.keys(contagens);
  } finally {
    db.close();
  }
}
