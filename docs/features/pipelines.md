# Pipelines de IA

Cada pipeline segue o mesmo padrão: **entrada → validação determinística → modelo → validação → revisão humana**. Etapas sem IA ficam em código, para serem testáveis e baratas.

Todo pipeline depende da decisão de privacidade em [README.md](README.md#decisões-em-aberto). Nenhum deles deve rodar antes disso.

Modelos por etapa seguem [roteamento-modelos.md](roteamento-modelos.md).

---

## P1. Revisão de trabalho próprio

**Objetivo:** apontar problemas no texto que a estudante escreveu. O pipeline não reescreve o trabalho.

**Entradas**
- Rascunho em Markdown (cópia de trabalho; o original não é alterado).
- Enunciado do AVA, quando existir em `materias/<matéria>/ava<N>-*.md`.
- Critérios de avaliação, se a estudante tiver.

**Etapas**
1. **Código (sem IA):** checar estrutura. Seções presentes, contagem de palavras, seção de referências existe.
2. **Haiku:** extrair do rascunho as afirmações, as citações (autor, ano) e a lista de referências, em JSON.
3. **Opus:** revisar aderência ao enunciado, coerência do argumento, lacunas, citações sem referência correspondente e afirmações técnicas duvidosas. Cada achado traz severidade (bloqueante, melhoria ou estilo), o trecho exato, a justificativa e uma pergunta para a estudante.
4. **Código:** descartar achado cujo trecho não existe no rascunho. Isso evita achado inventado.

**Saída:** relatório em Markdown e JSON, em pasta fora do Git.

**Não faz**
- Não escreve parágrafo de entrega.
- Não inventa referência. Citação que não bate com a lista vira "não verificado".

**Aceite**
- Achado sem trecho válido é descartado.
- Teste com um rascunho sintético de três seções, com uma citação sem referência e uma afirmação sem fonte.

---

## P2. Simulado de prova a partir do material local

**Objetivo:** gerar questões para estudar, com gabarito e referência à fonte.

**Entradas**
- Markdown das aulas e dos AVAs da matéria.
- Ementa, de `materias/catalogo/disciplinas.json`.

**Etapas**
1. **Haiku:** indexar seções e conceitos-chave de cada Markdown, em JSON com `fonte` e `seção`.
2. **Opus:** gerar N questões por conceito, com gabarito, justificativa e a seção-fonte. Tipos: múltipla escolha, verdadeiro ou falso justificado, resposta curta.
3. **Opus, chamada separada com papel de revisor:** para cada questão, verificar se o gabarito está certo, se há só uma resposta correta, se a fonte contém a informação e se há ambiguidade. Reprovar ou reescrever.
4. **Código:** validar o schema. Rejeitar questão cuja `fonte` não existe no índice.
5. **Entrega:** simulado sem gabarito e gabarito separado. Ambos marcados como "gerado por IA, conferir na aula".

**Não faz**
- Não se apresenta como prova oficial.
- Não usa prova antiga sem que o material seja da própria estudante e o uso seja permitido.

**Persistência:** a tabela de questões no SQLite exige aprovação prévia. Antes disso, saída em arquivo.

**Aceite**
- Toda questão tem `fonte` válida no índice.
- Em amostra de 10 questões, a revisão separada encontra os erros plantados de propósito.

---

## P3. Explicação de questão errada

**Objetivo:** ajudar a entender o erro depois que a prova foi corrigida.

**Entradas:** enunciado, resposta dada pela estudante, gabarito corrigido e trecho da fonte.

**Etapas**
1. **Opus:** explicar o conceito, apontar onde a resposta falhou, citar o trecho da fonte e sugerir uma questão de fixação.

**Regra:** usar só depois da correção, ou com enunciado de treino. Não revelar a resposta de prova ainda não corrigida.

**Aceite:** a explicação cita um trecho que existe na fonte. Se não existir, o pipeline diz que não há base.

---

## P4. Resumo curto e longo por matéria

**Objetivo:** resumos de estudo em duas durações, para ouvir ou ler.

**Base:** já especificado em `docs/superpowers/specs/2026-10-14-estuda-uva-resumos-anthropic-design.md`. Este pipeline acrescenta duas coisas:
- **Marcação "para prova":** quando a estudante marca, o resumo passa por revisão Opus antes de ser mostrado.
- **Modelo gravado:** ID e data ficam com o resumo, para saber qual versão foi gerada.

**Modelo:** Haiku por padrão. Sonnet com revisão Opus quando marcado para prova.

**Aceite:** resumo marcado "para prova" tem registro da revisão Opus.

---

## P5. Auditoria de referências ABNT

**Objetivo:** checar o formato das referências de um trabalho.

**Entradas:** seção de referências do Markdown do trabalho.

**Etapas**
1. **Haiku:** extrair de cada referência os campos (autor, título, ano, editora, tipo) em JSON.
2. **Código:** validar padrões. Ano presente, ordem alfabética, sobrenome em caixa-alta, pontuação. Marcar o que é incerto.
3. **Opus:** avaliar só os itens marcados como incertos pelo código.

**Limite:** o pipeline não confirma se a obra existe. Cada item sai como "existência não verificada", e a estudante confere na fonte.

**Aceite:** referência com ano faltando é marcada. A checagem não afirma que uma obra existe.

---

## P6. Ingestão de material novo

**Objetivo:** classificar um arquivo novo de `materias/fontes-uva/<período>/` e propor os metadados.

**Entradas:** arquivo novo (PDF ou Word). O original é preservado.

**Etapas**
1. **Código:** conversão para Markdown, pelo fluxo já existente.
2. **Haiku:** propor metadados em JSON: matéria, tipo (aula, AVA, lista, prova), data e título.
3. **Código:** comparar com `materias/catalogo/disciplinas.json`. Se não bater, pedir confirmação humana.

**Regra de conflito:** se o metadado proposto diverge do registro existente, o conflito vai para revisão humana. Não sobrescreve o registro (ver `AGENTS.md`).

**Persistência:** gravar no SQLite exige aprovação prévia.

**Aceite:** arquivo com matéria desconhecida não entra no catálogo sem confirmação.
