# Dados do acervo acadêmico

Esta pasta é a fonte de verdade estruturada do acervo. Cada registro é um arquivo JSON pequeno.
Tudo aqui é texto. Nada de PDF, imagem ou planilha.

## Como contribuir sem conflito

- **Um registro por arquivo.** Duas pessoas que editam arquivos diferentes não geram conflito de merge.
- **Não edite arquivos gerados.** `materias/catalogo/*.json` e `build/` são saída do gerador e não entram no Git.
- **Não reescreva registros de outras pessoas** para "arrumar" a formatação. Mude só o campo que você verificou.
- **Ao criar um registro**, use o nome do arquivo igual ao slug do `id` (ou o id do curso Canvas, no caso de `canvas/`).
- **Antes de enviar**, rode a validação. Ela recusa o que quebraria o gerador.

## Regras dos dados

1. **Ausência é `null` com pendência.** Se um campo não existe nas fontes, use `null` e registre o motivo em `pendencias`:
   `"pendencias": [{ "campo": "docente", "motivo": "Portal exibe apenas \"A\"" }]`.
   Nunca use `""`, nunca use `0` para "não sei", nunca invente valor.
2. **Toda informação tem fonte.** O campo `source` cita o arquivo do repositório, a seção e a data em que o dado foi observado.
   O arquivo citado precisa existir.
3. **Confirmado, estimado e pendente são coisas diferentes.** Se a fonte só mostra um valor provisório (ex.: um código que aparece numa tabela mas não foi confirmado no Canvas), registre o valor e uma pendência dizendo isso.
4. **Nunca registre dado pessoal ou sensível**: RA, CPF, RG, senha, cookie, token, e-mail, login, nem caminhos absolutos da sua máquina.
5. **Matrícula é do estudante, notas são do estudante.** Não copie o Portal inteiro; registre só o que a fonte mostra.

## Conteúdo em texto

- Documento (PDF, Word, Pages) vira Markdown, com a página indicada e a fonte. O PDF original continua local e não entra no Git. Para converter um PDF: `node tools/acervo/src/pdf-para-md.mjs <arquivo.pdf> <saida.md> "<fonte>"`.
- Diagrama e imagem viram Mermaid ou descrição textual, com a imagem de origem citada. A imagem pode ficar local, mas não é versionada.
- Extração de texto de PDF sem camada de texto (escaneado) precisa de OCR e revisão antes de ser aceita. Página sem texto aparece marcada como tal.
- `node tools/acervo/build.mjs --verificar-texto` lista binários rastreados (erro) e binários locais (aviso).

## Estrutura

| Pasta | O que guarda | Um arquivo por |
|---|---|---|
| `instituicao/` | a instituição | instituição |
| `programas/` | cursos (CC, EC) e o currículo de cada um | programa |
| `periodos/` | semestres (`2026.1`, `2026.2`) | período |
| `canvas/` | cursos do Canvas: id, URL, código, seção, tipo | id do curso Canvas |
| `disciplinas/` | matérias canônicas (as do currículo e as que você cursa) | disciplina |
| `ofertas/<programa>/` | itens do currículo (pendências), com carga horária | código da oferta |
| `relacoes/` | sobreposições candidatas entre CC e EC (não confirmadas) | relação |
| `trajetorias/` | vínculo com o curso: situação, série, progresso, contexto | programa |
| `matriculas/<período>/` | matrícula em cada disciplina: turma, docente, carga, curso Canvas | disciplina |
| `notas/<período>/` | notas do Lyceum, frequência e totais do Canvas | disciplina |
| `trabalhos/` | trabalhos AVA em `materias/`, com curso Canvas e período | trabalho |
| `meta.json` | observações gerais de data de coleta das trajetórias | arquivo único |

Os `id`s seguem o padrão das fontes (`uva:discipline:...`, `uva:program:...`). Não renomeie ids já existentes:
outros registros os referenciam.

## Campos que mais importam

- `canvas/<id>.json`: `titulo`, `url` (deve ser `https://uva.instructure.com/courses/<id>`), `tipo` (`curricular`, `apoio`, `gratuito`), `situacao`, `periodo`, `codigo`, `secao`.
- `matriculas/...`: `disciplinaId`, `canvasCourseId`, `section` (turma), `docente`, `cargaHoraria`, `status` (`enrolled` ou `closed`).
- `notas/...`: `lyceumGrades`, `lyceumAverage`, `absences`, `attendancePercent`, `canvasAvaTotals`.
- `trajetorias/...`: `contexto` (texto livre com fatos da situação) e `pendencias` (o que falta confirmar).

## Como validar e gerar

```sh
node tools/acervo/build.mjs --validar   # só valida dados/
node tools/acervo/build.mjs             # valida e gera build/ e materias/catalogo/
node --test tools/acervo/test/acervo.test.mjs
```

O que é gerado, e onde:

- `build/acervo.sqlite3`: banco com tabelas e views (`v_matriculas`, `v_cursos`, `v_pendencias`, `v_curriculo`, `v_resumo_periodo`). Descartável: é recriado a cada build.
- `build/visualizacoes/*.md`: matrículas, cursos do Canvas, pendências, currículos, notas.
- `materias/catalogo/*.json`: o formato antigo que o app Estuda UVA lê. Gerado; não editar.

## Pendências em aberto

Para ver tudo que falta confirmar, abra `build/visualizacoes/pendencias.md` depois do build,
ou rode `SELECT * FROM v_pendencias;` no `build/acervo.sqlite3`.
