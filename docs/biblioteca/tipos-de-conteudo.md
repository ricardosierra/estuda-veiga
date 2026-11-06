# Tipos de conteúdo da biblioteca

**Revisado em:** 2026-11-06 · **Estado:** regras **Planejadas**, com o que já existe indicado em cada tipo.

Regra geral, válida para todos os tipos:

> Material restrito, não autorizado ou com direitos de distribuição desconhecidos **nunca** é apresentado como público. Na dúvida, o material fica `publication = private` (ver [catalogacao.md](catalogacao.md)).

Trabalhos entregues, provas pessoais, notas e qualquer material com identificação do aluno ficam em `privado/` (separação de pasta, não de sigilo). Pela decisão de 6/11/2026, esse conteúdo pode ir ao Git e ao acervo público quando os dados pessoais estão separados em `privado/`; RA, CPF e RG continuam fora. Todo texto versionado segue a política de conteúdo em texto de `AGENTS.md`: PDF e documento viram Markdown com página e fonte; imagem e diagrama viram Mermaid ou descrição textual.

## 1. Disciplinas

**Existe (Implementado):** `dados/disciplinas/*.json` (76 arquivos), `dados/ofertas/<programa>/*.json` (79), `dados/relacoes/*.json` (14, sobreposições candidatas entre cursos) e `dados/programas/*.json` (2). Cada registro tem `id`, `source` e `pendencias`. Disciplinas pessoais (`identityStatus = matricula_registrada`) foram movidas para `privado/dados/disciplinas/` pelo plano de `tools/acervo/src/separar-privado.mjs`.

**Regras propostas para o contrato público:**

- **Ementa:** texto próprio ou citação curta com fonte oficial; ementa copiada integralmente de documento da instituição exige conferir direitos antes de publicar.
- **Competências e tópicos:** listas em Markdown, uma ideia por item.
- **Pré-requisitos:** lista de `id` de disciplinas; só o que a matriz curricular oficial declara.
- **Relações curriculares:** equivalência **nunca** é inferida. `dados/relacoes/` guarda apenas "candidata" e o campo de equivalência confirmada permanece falso até haver fonte oficial (o app já trata assim; ver `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`).
- **Campos desconhecidos:** `null` com pendência, nunca valor inventado.
- **Nomes de docentes:** são dado de terceiros; ficam fora da camada pública (`docs/ARQUITETURA_ACERVO.md` §2).

## 2. Conteúdos (aulas, resumos, textos, fórmulas, conceitos, referências)

**Existe (Parcial):**

- `fontes-uva/<período>/<matéria>/*.md`: texto extraído de materiais oficiais, com página (`## Página N`), link oficial e data de download no cabeçalho. É material de **terceiro** (UVA e autores de e-books).
- `periodos/<período>/notas-de-estudo.md`: notas autorais de estudo, com links oficiais.
- Aulas locais geradas pelo app (`app/crates/estuda-core/src/lesson.rs`) a partir do Markdown capturado; o texto de origem continua sendo a fonte.

**Regras propostas:**

- Resumo e nota de estudo são obra derivada: citar a fonte, indicar a página e declarar autoria (própria ou gerada por IA, conferida pelo autor).
- Fórmulas em LaTeX dentro de `$...$` ou `$$...$$`, para não depender de imagem.
- Conceitos e definições: uma definição por parágrafo, com referência.
- Referências: citação completa (autor, título, ano, edição) ou URL oficial sem query.
- Texto extraído integral de e-book ou material de terceiro: **não publicar** sem licença ou permissão (ver [fontes-e-licencas.md](fontes-e-licencas.md)).
- Extração de PDF escaneado exige OCR e revisão antes de ser aceita; páginas sem texto ficam marcadas como tal (`dados/LEIA-ME.md`).

## 3. Exercícios

**Existe:** não há coleção de exercícios no repositório. As atividades AVA estão em `privado/materias/<matéria>/` e em `dados/trabalhos` (`privado/dados/trabalhos/`), classificadas como **trabalhos**, não como exercícios (§5).

**Regras propostas:**

- **Enunciado:** texto próprio ou com permissão; enunciado copiado de livro ou prova de terceiro exige licença.
- **Alternativas:** lista numerada; resposta correta marcada em campo próprio, não no texto do enunciado.
- **Dificuldade:** `basic`, `intermediate` ou `advanced`, critério definido pelo revisor.
- **Assunto:** campo `subject` e `tags` da ficha.
- **Resolução:** é uma **solução** e segue §6 (separada do enunciado, com permissão própria).
- Exercícios gerados automaticamente pelo app (`lesson.rs`, `tutor.rs`) são práticas locais de estudo e não são material curado.

## 4. Provas

**Existe:** documentos originais ficam em `originais/` (ignorado pelo Git). Provas e avaliações não são capturadas pelo app (o importador ignora avaliações, fóruns, notas e frequência; ver `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`).

**Regras propostas:**

- **Origem e período:** instituição, disciplina, período e tipo de avaliação (`A1`, `A2`, `A3`, prova especial etc., conforme a política da disciplina).
- **Questões:** uma por seção, numeradas como na prova original.
- **Condição de compartilhamento:** prova só é pública se a instituição ou o docente autorizou a divulgação por escrito. Prova pessoal (com nome, nota ou correção do aluno) fica em `privado/`; a decisão de 6/11/2026 permite o Git com os dados pessoais separados em `privado/`, mas a autorização da instituição ou do docente acima continua valendo.
- **Prova em andamento ou futura:** nunca publicar.
- Não fazer a biblioteca servir como meio de fraude acadêmica: enunciados atuais de avaliações em aberto não entram.

## 5. Trabalhos

**Existe (Implementado, privado):** 12 arquivos em `privado/materias/` e registros em `privado/dados/trabalhos/` (um por trabalho, com `id`, `titulo`, `arquivo`, `disciplinaId`, `canvasCourseId`, `periodo`, `pendencias`). Os textos foram convertidos de Word/Pages para Markdown "removendo nome completo e matrícula" (`materias/README.md`); os originais ficam em `originais/`.

**Regras propostas:**

- **Enunciado:** separado da entrega. O enunciado oficial é de terceiro (instituição); a entrega é obra do aluno.
- **Requisitos e critérios:** transcritos com fonte; sem inferir critério não publicado.
- **Referências e versões:** cada versão da entrega é uma ficha com `version` próprio.
- **Entrega do aluno:** fica em `privado/`. Pela decisão de 6/11/2026, pode ir ao acervo público e ao Git com os dados pessoais separados em `privado/`; RA, CPF e RG continuam fora. Identificação de docente ou de terceiro na entrega segue pendente (ver [lacunas-e-contradicoes.md](../referencia/lacunas-e-contradicoes.md) §1). Publicar resolução de trabalho em andamento pode violar regras de integridade acadêmica.
- A IA do app não gera a entrega avaliada para ser enviada como autoral (decisão registrada em `docs/features/README.md`).

## 6. Soluções

**Existe:** não há soluções publicáveis catalogadas. Há resoluções do próprio aluno dentro de trabalhos privados (§5).

**Regras propostas:**

- **Autoria autorizada:** o autor declara a autoria e consente a publicação (`authorship.consent`).
- **Abordagem e explicação:** a solução traz o raciocínio, não só o resultado.
- **Revisão:** pelo menos uma pessoa diferente do autor confere correção e ausência de dado pessoal (processo em [../contribuindo/conteudo.md](../contribuindo/conteudo.md)).
- **Fontes:** citar o material em que a solução se baseia.
- **Permissões de publicação:** solução de exercício de livro ou de prova de terceiro depende da licença da origem.
- Solução de avaliação em aberto nunca é publicada.

## 7. Resumo do estado

| Tipo | Existe hoje | Camada atual | Publicável hoje |
|---|---|---|---|
| Disciplina | sim (`dados/`) | candidata, exige revisão | só após revisão (contrato: `NeedsReview` para `dados/disciplinas/`) |
| Conteúdo | parcial (`fontes-uva/`, `periodos/`) | candidata | **não verificado**; direitos de `fontes-uva/` em aberto |
| Exercício | não | — | — |
| Prova | não (só originais locais) | privada | não |
| Trabalho | sim, privado | privada | sim, pela decisão de 6/11/2026, com dados pessoais em `privado/`; identificação de docente pendente |
| Solução | não catalogada | privada | não |
