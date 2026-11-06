# Apuração de votos em processo eleitoral empresarial

> Bases de Programação · AVA 1. Conversão textual do documento Word; nome completo e matrícula removidos.

## 1 INTRODUÇÃO

Algoritmos fazem parte do cotidiano, sejam implementados em computadores ou organizando tarefas do dia a dia, como seguir uma receita de bolo ou planejar uma compra pela internet. Em programação, compreender como estruturar algoritmos sequenciais, manipular dados de entrada e saída e modularizar soluções em procedimentos é essencial para construir programas corretos e reutilizáveis (MANZANO; OLIVEIRA, 2019). Este trabalho, desenvolvido na disciplina Bases de Programação, tem como foco a construção de um algoritmo sequencial em pseudocódigo, utilizando procedimentos e passagem de parâmetros, para apurar resultados de um processo eleitoral interno em uma empresa.

## 2 OBJETIVOS

O objetivo geral deste trabalho é aplicar os conceitos estudados em Bases de Programação para desenvolver um algoritmo sequencial, modularizado com procedimento, capaz de calcular e apresentar estatísticas de votação em um processo eleitoral de uma empresa. Como objetivos específicos, busca-se empregar estruturas básicas de entrada, processamento e saída de dados em pseudocódigo; utilizar um procedimento com parâmetros para organizar cálculos e melhorar a reutilização de código; e calcular o total de eleitores e os percentuais de votos válidos, por candidato, nulos e em branco, com base nos dados informados pelo usuário.

## 3 FUNDAMENTAÇÃO TEÓRICA

Um algoritmo pode ser entendido como uma sequência finita de passos bem definidos que descreve como resolver um problema, normalmente seguindo a estrutura de entrada, processamento e saída (FORBELLONE; EBERSPÄCHER, 2005). Em linguagens de ensino como o Portugol utilizado pelo ambiente VisuAlg, a organização básica costuma incluir declaração de variáveis, comandos de leitura, processamento com atribuições e comandos de escrita. A modularização consiste em dividir o algoritmo em partes menores, como funções e procedimentos, que executam tarefas específicas; no VisuAlg, procedimentos são blocos de código que não retornam valor, mas recebem parâmetros e podem escrever resultados em tela, favorecendo reuso de código, clareza e facilidade de manutenção.

## 4 METODOLOGIA

A metodologia adotada neste trabalho é prática e baseada na implementação em pseudocódigo usando a sintaxe do VisuAlg, um interpretador de Portugol amplamente utilizado no ensino de lógica de programação. Foram analisados os requisitos da atividade, identificando os dados de entrada e de saída, definidas variáveis inteiras para representar os votos de cada candidato, os votos nulos e os votos em branco, além de variáveis reais para representar percentuais. Em seguida, foi projetado um procedimento responsável por receber os votos como parâmetros, calcular o total de eleitores e os percentuais, e apresentar os resultados na tela, mantendo a sequência de execução sem estruturas condicionais ou de repetição. O algoritmo foi implementado e testado no VisuAlg, verificando se os cálculos produzem resultados coerentes com contas manuais para diferentes conjuntos de dados.

## 5 DESENVOLVIMENTO DO ALGORITMO

O problema considera uma eleição com três candidatos (C1, C2 e C3), além de votos nulos e em branco, e supõe que após a apuração os números de votos já são conhecidos e serão fornecidos ao programa pelo usuário. O algoritmo deve ler esses valores e, em seguida, calcular o total de eleitores, o percentual de votos válidos em relação ao total de eleitores, o percentual de votos válidos de cada candidato e os percentuais de votos nulos e em branco. O algoritmo principal foi estruturado com declaração das variáveis inteiras para os votos, exibição de mensagens solicitando os dados ao usuário, leitura dos valores e chamada de um procedimento calcularResultadosEleicao, passando como parâmetros os valores lidos.

Dentro do procedimento são declaradas variáveis reais para o total de eleitores, total de votos válidos e todos os percentuais. O total de eleitores é calculado como a soma de todas as categorias de votos, e o total de válidos como a soma dos votos dos três candidatos. Cada percentual é calculado multiplicando o número de votos correspondente por 100 e dividindo pelo total de eleitores, respeitando o conceito matemático de porcentagem. O procedimento imprime todos os resultados de forma organizada na tela, com mensagens descritivas que facilitam a interpretação para o usuário.

## 6 RESULTADOS E ANÁLISE

Para validar o algoritmo, foram realizados testes com conjuntos de valores representativos. Em um exemplo, considerou-se C1 = 10 votos, C2 = 5 votos, C3 = 5 votos, votos nulos = 2 e votos em branco = 3, totalizando 25 eleitores. Nesse cenário, o algoritmo calcula o total de eleitores corretamente como 25, e os votos válidos como 20, resultando em um percentual de votos válidos de 80% sobre o total. O percentual de cada candidato sobre o total de eleitores é calculado em 40% para C1, 20% para C2 e 20% para C3, enquanto votos nulos correspondem a 8% e votos em branco a 12%. Esses resultados foram conferidos manualmente, confirmando a correção dos cálculos e demonstrando que o algoritmo atende ao objetivo de apurar de forma clara as proporções de participação de cada tipo de voto.

## 7 CONSIDERAÇÕES FINAIS

Este trabalho permitiu aplicar de maneira integrada diversos conceitos da disciplina Bases de Programação, como declaração de variáveis, operações aritméticas, comandos de entrada e saída e a modularização por meio de procedimentos com passagem de parâmetros. A solução proposta atende às especificações do problema, utilizando exclusivamente estrutura sequencial de execução e organizando os cálculos em um procedimento responsável por concentrar a lógica de apuração e exibição dos resultados. Além de cumprir os requisitos da atividade, o exercício reforça a importância de planejar algoritmos com clareza, identificar corretamente os dados de entrada e saída e testar o código com diferentes conjuntos de valores para garantir a confiabilidade dos resultados. A abordagem pode ser adaptada para outros cenários de votação ou para problemas que envolvam cálculo de estatísticas básicas em contextos empresariais e acadêmicos.

## Referências

FORBELLONE, André Luiz Villar; EBERSPÄCHER, Henri Frederico. **Lógica de programação**: a construção de algoritmos e estruturas de dados. 3. ed. São Paulo: Pearson Prentice Hall, 2005.

MANZANO, José Augusto N. G.; OLIVEIRA, Jayr Figueiredo de. **Algoritmos**: lógica para desenvolvimento de programação de computadores. 29. ed. São Paulo: Érica, 2019.
