# Catálogo acadêmico estruturado

Os arquivos JSON desta pasta organizam informações já registradas em `REGISTRO_ACADEMICO.md` e `ENGENHARIA_DE_COMPUTACAO.md`. O Estuda UVA poderá carregá-los no SQLite sem reinterpretar os vínculos do Portal/Canvas.

## Arquivos

- `disciplinas.json`: identificadores canônicos estáveis e referências às ofertas por currículo.
- `relacoes.json`: possíveis sobreposições entre ofertas; nenhuma relação candidata confirma equivalência ou aproveitamento.
- `curriculos/uva-ciencia-computacao.json`: 36 pendências exibidas no vínculo atual e as 7 matrículas de 2026.2.
- `curriculos/uva-engenharia-computacao.json`: 43 pendências do vínculo anterior marcado como evadido.
- `trajetorias.json`: situação separada de cada vínculo, cargas horárias, notas e frequência observadas, totais de Canvas e limites das fontes.

## Regras de manutenção

- `schemaVersion` é obrigatório. IDs de programa, currículo, oferta, disciplina canônica e trajetória são estáveis.
- Códigos, grafias exibidas, cargas, datas e estados reproduzem as fontes locais. Um campo ausente não vira aprovação, nota zero ou equivalência.
- Ciência da Computação e Engenharia de Computação são trajetórias diferentes. Uma disciplina canônica pode referenciar ofertas candidatas sem unir matrículas, status, notas ou créditos.
- Para atualizar um dado, acrescente uma proveniência com arquivo, seção e data de observação. Preserve também a informação anterior quando uma fonte nova divergir, registrando a divergência.
- Não guardar RA, CPF, credenciais, cookies, tokens, caminhos absolutos ou estado do navegador.
- Não mover materiais existentes. `offeringIds` e os documentos acadêmicos mantêm referências relativas ao repositório.

## Proveniência e limitações atuais

Os catálogos refletem as capturas autenticadas de 9 de outubro de 2026 já documentadas nos dois registros. O histórico disponível não distribui as 330 horas cursadas em Ciência da Computação nem as 1.446 horas da Engenharia por disciplina. A soma das pendências também diverge do total faltante informado em ambos os cursos. Esses números permanecem agregados e com a divergência explícita; não foram distribuídos ou corrigidos por estimativa.

As relações em `relacoes.json` são candidatas para facilitar a busca de materiais em comum. A confirmação depende de ementas ou documento institucional. Até lá, o app deve apresentar cada oferta e trajetória separadamente.
