# Roteamento de modelos Claude

## Regra da estudante

- **Trabalhos e provas:** sempre `claude-opus-5-5`.
- **Alternativa aceita:** `claude-sonnet-5-5` gera e `claude-opus-5-5` revisa. Sonnet sozinho não serve para trabalho ou prova.
- **Demais tarefas:** escolher pela necessidade. Haiku para tarefas mecânicas, Sonnet para uso geral.

## Tabela

| Tipo de tarefa | Modelo | Observação |
|---|---|---|
| Revisão de trabalho próprio: aderência ao enunciado, argumento, ABNT, coerência | `claude-opus-5-5` | Sempre. |
| Prova: geração de questões, gabarito, justificativa | `claude-opus-5-5` | Sempre. |
| Prova: explicação de questão errada depois da correção | `claude-opus-5-5` | Sempre. |
| Trabalho ou prova com volume grande (varredura, esboço, muitas questões) | `claude-sonnet-5-5` + revisão `claude-opus-5-5` | Opus revisa antes de a estudante ver o resultado. |
| Conversa de estudo sobre conceito (fora de trabalho e prova) | `claude-sonnet-5-5` | |
| Resumo curto e longo por matéria | `claude-haiku-5-5` | Padrão já especificado. Se a estudante marcar o resumo para prova, subir para Sonnet com revisão Opus. |
| Extração e classificação: metadados, datas, tags, referências em JSON | `claude-haiku-5-5` | Saída validada por schema e por código. |
| Escalonamento: saída do nível anterior falha na validação | Próximo nível | Haiku → Sonnet → Opus. |

## Regras

1. **Trabalho e prova nunca rodam só em Haiku ou só em Sonnet.** O caminho é Opus direto ou Sonnet com revisão Opus.
2. **Haiku não decide nada acadêmico.** Ele só extrai, classifica e formata.
3. **A revisão Opus é uma chamada separada,** com prompt próprio de crítica. Recebe o texto de entrada e a saída do Sonnet. Devolve achados classificados, sem reescrever o texto inteiro.
4. **Escalonar por validação, nunca rebaixar.** Se a saída falha no schema ou no código de checagem, sobe um nível. Nunca desce para economizar.
5. **Gravar o modelo junto da saída:** ID do modelo, data e tipo de tarefa. Isso segue o que a spec de resumos já pede.

## Como configurar

- **Hoje:** `ANTHROPIC_MODEL` no `.env` (fora do repositório), lido a cada uso. Padrão Haiku. Um único modelo para o app inteiro.
- **Proposto:** uma tabela de roteamento em código, com um tipo de tarefa por função (por exemplo, `TRABALHO`, `PROVA`, `RESUMO`, `EXTRACAO`). `TRABALHO` e `PROVA` resolvem sempre para `claude-opus-5-5`, sem override por variável de ambiente.
- **Teste obrigatório:** um teste que falha se `TRABALHO` ou `PROVA` resolverem para outro modelo.
- **Custo:** consultar a tabela de preços atual da Anthropic antes de fixar orçamento. Definir limite de gasto no console da conta. Opus é o nível mais caro, e o uso em trabalhos e provas deve ser acompanhado.
- **Avaliação antes de trocar modelo:** rodar a mesma suíte de casos em Haiku, Sonnet e Opus, e registrar acerto e custo. A decisão de roteamento deve vir desses números, não de palpite. Ver [ideias.md](ideias.md).
