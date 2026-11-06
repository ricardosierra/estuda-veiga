// Converte um PDF em Markdown com pdftotext (camada de texto). Não remove o PDF.
// Cada página vira uma seção "## Página N". Páginas sem texto são sinalizadas.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { basename, dirname, join } from 'node:path';

function paginas(pdf) {
  const info = execFileSync('pdfinfo', [pdf], { encoding: 'utf8' });
  const m = info.match(/^Pages:\s+(\d+)/m);
  return m ? Number(m[1]) : 0;
}

export function pdfParaMarkdown({ pdf, destino, fonte }) {
  const nome = basename(pdf);
  const total = paginas(pdf);
  const texto = execFileSync('pdftotext', ['-layout', '-enc', 'UTF-8', pdf, '-'], {
    encoding: 'utf8',
    maxBuffer: 64 * 1024 * 1024,
  });
  const blocos = texto.split('\f');
  // pdftotext termina com \f: o último bloco vazio não é página.
  if (blocos.length && blocos[blocos.length - 1].trim() === '') blocos.pop();

  const titulo = basename(nome, '.pdf').replace(/[_]+/g, ' ').replace(/\s+/g, ' ').trim();
  const semTexto = [];
  let md = `# ${titulo}\n\n`;
  md += `> Texto extraído de \`${nome}\` (${total} páginas) com \`pdftotext -layout\`. `;
  md += 'Conversão textual: imagens, diagramas e layout visual não são reproduzidos. ';
  md += `O PDF original permanece local e não é versionado.${fonte ? ` Origem: ${fonte}.` : ''}\n\n`;
  blocos.forEach((bloco, indice) => {
    const numero = indice + 1;
    const limpo = bloco.replace(/\s+$/gm, '').replace(/\n{3,}/g, '\n\n').trim();
    md += `## Página ${numero}\n\n`;
    if (limpo.length === 0) {
      semTexto.push(numero);
      md += '_Página sem texto selecionável._\n\n';
    } else {
      md += `${limpo}\n\n`;
    }
  });

  mkdirSync(dirname(destino), { recursive: true });
  writeFileSync(destino, md.replace(/\n{3,}/g, '\n\n'));
  return { destino, paginas: total, semTexto, caracteres: texto.length };
}

if (process.argv[1] && process.argv[1].endsWith('pdf-para-md.mjs')) {
  const [pdf, destino, fonte] = process.argv.slice(2);
  if (!pdf || !destino) {
    console.error('Uso: node tools/acervo/src/pdf-para-md.mjs <arquivo.pdf> <saida.md> [fonte]');
    process.exit(1);
  }
  console.log(JSON.stringify(pdfParaMarkdown({ pdf, destino, fonte }), null, 2));
}
