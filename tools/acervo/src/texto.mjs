// Verifica a política "conteúdo em texto": PDFs, documentos e imagens não ficam versionados.
// Erro: binário já rastreado pelo Git. Aviso: binário local (ignorado ou não rastreado), a converter ou remover.
import { execFileSync } from 'node:child_process';
import { readdir } from 'node:fs/promises';
import { join, relative, sep, extname, basename } from 'node:path';

const EXTENSOES_BINARIAS = new Set([
  'pdf', 'docx', 'doc', 'odt', 'rtf', 'pages', 'key', 'numbers', 'xlsx', 'xls', 'pptx', 'ppt',
  'png', 'jpg', 'jpeg', 'gif', 'webp', 'heic', 'bmp', 'tif', 'tiff',
  'mp4', 'mov', 'mp3', 'wav', 'm4a', 'zip', '7z', 'rar', 'ttf', 'otf', 'woff', 'woff2',
]);

// Pastas que não são conteúdo do acervo: código, dependências, saídas geradas, originais privados.
const PASTAS_IGNORADAS = new Set([
  '.git', 'node_modules', 'build', 'app', 'originais', 'tmp', '.estuda-uva', '.superpowers', 'target', 'dist',
]);

function listarTrackeados(raiz) {
  try {
    const saida = execFileSync('git', ['-C', raiz, 'ls-files', '-z'], { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
    return new Set(saida.split('\0').filter(Boolean));
  } catch {
    return null; // não é um repositório Git: todos são tratados como locais
  }
}

async function percorrer(dir, raiz, achados) {
  let entradas;
  try {
    entradas = await readdir(dir, { withFileTypes: true });
  } catch {
    return;
  }
  for (const entrada of entradas) {
    if (entrada.name.startsWith('.')) continue;
    const abs = join(dir, entrada.name);
    if (entrada.isDirectory()) {
      if (!PASTAS_IGNORADAS.has(entrada.name)) await percorrer(abs, raiz, achados);
    } else if (entrada.isFile()) {
      const extensao = extname(entrada.name).slice(1).toLowerCase();
      if (EXTENSOES_BINARIAS.has(extensao)) {
        achados.push(relative(raiz, abs).split(sep).join('/'));
      }
    }
  }
}

export async function verificarTexto(raiz) {
  const trackeados = listarTrackeados(raiz);
  const achados = [];
  await percorrer(raiz, raiz, achados);
  const erros = [];
  const avisos = [];
  for (const caminho of achados.sort()) {
    const nome = basename(caminho);
    if (nome === '.DS_Store') continue;
    if (trackeados?.has(caminho)) {
      erros.push({ caminho, campo: null, motivo: 'binário versionado no Git: converta para texto (Markdown/Mermaid) e remova do índice' });
    } else {
      avisos.push({ caminho, campo: null, motivo: 'binário local: converta para Markdown e remova do disco quando aprovado' });
    }
  }
  return { erros, avisos };
}
