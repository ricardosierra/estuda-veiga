# Ideias (backlog)

Ideias ainda não priorizadas. Cada uma diz o problema que resolve e a dependência principal. Nada aqui está aprovado para implementação.

## Controle de custo e qualidade

- [ ] **Painel de custo por tarefa.** Registrar tokens de entrada e saída, modelo, tipo de tarefa e data. Resolve: não saber quanto cada pipeline custa. Depende de tabela nova no SQLite, portanto de aprovação prévia.
- [ ] **Limite de gasto mensal.** Configurar no console da Anthropic e mostrar alerta no app quando o consumo se aproximar do limite.
- [ ] **Suíte de avaliação por pipeline.** Dez casos com resposta esperada por pipeline. Serve para trocar de modelo com evidência: rodar Haiku, Sonnet e Opus na mesma suíte e comparar acerto e custo. Resolve: decidir o roteamento por dado, não por palpite.

## Privacidade

- [ ] **Opt-in por material.** Cada arquivo tem a marca "pode enviar para IA". Por padrão, nada é enviado. Resolve: a tensão com a decisão do tutor local (ver [README.md](README.md#decisões-em-aberto)).
- [ ] **Envio por trecho.** Em vez do arquivo inteiro, enviar só a seção escolhida pela estudante. Reduz o volume de conteúdo exposto e o custo.

## Eficiência

- [ ] **Cache de contexto para material repetido.** Vários pipelines usam o mesmo Markdown da matéria. Avaliar o prompt caching da API, conforme a documentação atual.
- [ ] **Processamento em lote.** Resumos de todas as matérias numa única execução, fora do uso interativo. Avaliar a Message Batches API, se o custo compensar.
- [ ] **Cache de saída por hash.** Se o material não mudou, reaproveitar o resumo ou o simulado já gerado. Já previsto na spec de resumos pela chave de desatualização.

## Estudo

- [ ] **Checklist de prontidão para prova.** Cruzar as datas do calendário do período (hoje só existe `CALENDARIO_2026.1.md`; falta o de 2026.2) com os conceitos do índice (P2) e mostrar o que falta revisar.
- [ ] **Cartões de revisão.** Gerar cartões a partir das notas de estudo com repetição espaçada. Pode ser feito sem IA, a partir do índice local.
- [ ] **Plano de estudo semanal.** Distribuir os conceitos pendentes pelas semanas até a prova. Sonnet gera, Opus revisa se for para prova.

## Trabalhos

- [ ] **Comparador de versões.** Mostrar o que mudou entre a versão 1 e a versão 2 do rascunho, com os achados de P1 que foram resolvidos.
- [ ] **Verificação de citações online.** Confirmar a existência de cada referência em base externa. Só com opt-in explícito, porque envia dados de consulta para fora. Hoje o P5 marca como "não verificado".

## Arquitetura

- [ ] **Tabela de roteamento em código, com teste.** Ver [roteamento-modelos.md](roteamento-modelos.md#como-configurar). Garante que trabalho e prova nunca rodem em modelo menor.
- [ ] **Versão do modelo na saída.** Já previsto para resumos. Estender para todos os pipelines, para saber qual modelo produziu cada resultado.
