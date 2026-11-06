# Catálogo acadêmico (gerado)

Os JSON desta pasta são **gerados** por `node tools/acervo/build.mjs` a partir de `dados/`.
Não edite estes arquivos: a edição se perde no próximo build. Para mudar um dado, edite o registro em `dados/` (veja `dados/LEIA-ME.md`).

## Arquivos gerados

- `disciplinas.json`: matérias canônicas (pendências do currículo e matérias cursadas ou em curso).
- `relacoes.json`: sobreposições candidatas entre ofertas de CC e EC. Nenhuma confirma equivalência ou aproveitamento.
- `curriculos/uva-ciencia-computacao.json`: 36 pendências do currículo atual e as matrículas de 2026.2.
- `curriculos/uva-engenharia-computacao.json`: 43 pendências do vínculo anterior, marcado como evadido no Portal.
- `trajetorias.json`: situação de cada vínculo, carga horária, notas, frequência e totais do Canvas.

O app Estuda UVA lê estes arquivos no formato que sempre leu.

## Regras que continuam valendo

- Ciência da Computação e Engenharia de Computação são trajetórias diferentes. Uma disciplina canônica pode referenciar ofertas candidatas sem unir matrículas, status, notas ou créditos.
- Um campo ausente nas fontes vira `null` com pendência. Nunca vira aprovação, nota zero ou equivalência.
- Não guardar RA, CPF, credenciais, cookies, tokens, caminhos absolutos ou estado do navegador.

## Limitações da fonte

As capturas autenticadas são de 9 de outubro de 2026. As 330 horas cursadas em Ciência da Computação e as 1.446 horas de Engenharia não são distribuídas por disciplina nas telas observadas. A soma das pendências diverge do total faltante informado pelo Portal nos dois cursos. Essa divergência fica registrada, sem correção por estimativa.
