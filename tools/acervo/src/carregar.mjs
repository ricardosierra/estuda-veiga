// Lê todos os registros JSON de dados/ e a lista de arquivos de materias/.
// Não escreve nada.
import { readdir, readFile, stat } from 'node:fs/promises';
import { basename, join, relative, sep } from 'node:path';

async function percorrer(dir) {
  let entradas;
  try {
    entradas = await readdir(dir, { withFileTypes: true });
  } catch (erro) {
    if (erro.code === 'ENOENT') return [];
    throw erro;
  }
  const achados = [];
  for (const entrada of entradas) {
    if (entrada.name.startsWith('.')) continue;
    const caminho = join(dir, entrada.name);
    if (entrada.isDirectory()) achados.push(...(await percorrer(caminho)));
    else if (entrada.isFile()) achados.push(caminho);
  }
  return achados.sort();
}

const paraCaminhoRepo = (raiz, abs) => relative(raiz, abs).split(sep).join('/');

// Cada registro: { tipo, caminho, nome, dados }.
// tipo é a primeira pasta de dados/ (ex.: "canvas", "matriculas"); "meta" para dados/meta.json.
// Lê dados/ (matéria e instituição) e privado/dados/ (dados do aluno), no mesmo repositório.
// Os dois conjuntos têm o mesmo formato; o tipo vem da primeira pasta depois de dados/.
export const RAIZES_DADOS = ['dados', join('privado', 'dados')];

export async function carregarDados(raiz) {
  const registros = [];
  const erros = [];
  for (const pasta of RAIZES_DADOS) {
    const dadosDir = join(raiz, pasta);
    const arquivos = (await percorrer(dadosDir)).filter((arquivo) => arquivo.endsWith('.json'));
    for (const abs of arquivos) {
      const caminho = paraCaminhoRepo(raiz, abs);
      const partes = relative(dadosDir, abs).split(sep);
      const tipo = partes.length === 1 ? basename(partes[0], '.json') : partes[0];
      let dados;
      try {
        dados = JSON.parse(await readFile(abs, 'utf8'));
      } catch (erro) {
        erros.push({ caminho, campo: null, motivo: `JSON inválido: ${erro.message}` });
        continue;
      }
      registros.push({ tipo, caminho, nome: basename(abs, '.json'), dados, privado: pasta !== 'dados' });
    }
  }
  return { raiz, registros, erros };
}

// Lista os arquivos de materias/ (nome, pasta, extensão, tamanho). Não lê o conteúdo.
const IGNORADOS = new Set(['originais', 'capturas', 'catalogo', 'build', 'node_modules']);

// Pastas de conteúdo indexadas: matérias, fontes UVA, períodos, cursos e a parte privada.
export const PASTAS_MATERIAIS = ['materias', 'fontes-uva', 'periodos', 'cursos', 'privado'];

export async function listarMateriais(raiz) {
  const achados = [];
  for (const pasta of PASTAS_MATERIAIS) {
    await listarPasta(raiz, join(raiz, pasta), achados);
  }
  return achados.sort((a, b) => a.caminho.localeCompare(b.caminho));
}

async function listarPasta(raiz, base, achados) {
  async function visitar(dir) {
    let entradas;
    try {
      entradas = await readdir(dir, { withFileTypes: true });
    } catch (erro) {
      if (erro.code === 'ENOENT') return;
      throw erro;
    }
    for (const entrada of entradas) {
      if (entrada.name.startsWith('.') || IGNORADOS.has(entrada.name)) continue;
      const abs = join(dir, entrada.name);
      if (entrada.isDirectory()) {
        await visitar(abs);
      } else if (entrada.isFile()) {
        const info = await stat(abs);
        const caminho = paraCaminhoRepo(raiz, abs);
        const partes = relative(base, abs).split(sep);
        const extensao = entrada.name.includes('.') ? entrada.name.split('.').pop().toLowerCase() : '';
        achados.push({
          caminho,
          pasta: partes.length > 1 ? partes[0] : null,
          extensao,
          bytes: info.size,
        });
      }
    }
  }
  await visitar(base);
}
