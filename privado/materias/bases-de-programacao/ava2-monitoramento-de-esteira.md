# Monitoramento de esteira de transporte com sensor

> Bases de Programação · AVA 2. Conversão textual do documento Word; nome completo e matrícula removidos.

## 1 INTRODUÇÃO

Na automação industrial e na gestão operacional de processos produtivos, a coleta e o processamento de dados sensoriais são vitais para garantir o funcionamento contínuo, a segurança e a precisão de maquinários. Em sistemas de transporte mecânico por esteira, amplamente utilizados em linhas de triagem e montagem, sensores de deslocamento monitoram a movimentação física de peças e produtos, detectando tanto os avanços regulares quanto eventuais recuos que possam indicar acúmulo de carga, paradas forçadas ou falhas mecânicas (MANZANO; OLIVEIRA, 2019).

No âmbito da programação de computadores e da disciplina Bases de Programação, a manipulação de sequências extensas de dados exige o emprego de estruturas homogêneas — os vetores unidimensionais (arrays) —, combinadas com laços de repetição e estruturas de controle de decisão. Este trabalho apresenta o desenvolvimento de um algoritmo estruturado em pseudocódigo (Portugol/VisuAlg 3.0), projetado para receber 150 valores inteiros de um sensor mecânico, armazená-los em um vetor, quantificar os impulsos positivos (avanço da esteira) e acumular o somatório algébrico dos impulsos negativos (recuo da esteira).

## 2 OBJETIVOS

O objetivo geral deste trabalho é aplicar os conceitos teóricos e práticos de Bases de Programação para desenvolver, codificar e validar um algoritmo estruturado capaz de ler 150 valores de um sensor, registrá-los em um vetor unidimensional e apurar estatísticas de avanço e recuo da esteira de transporte.

Como objetivos específicos, destacam-se: (a) definir e dimensionar na memória um vetor de 150 posições do tipo inteiro; (b) implementar um laço de repetição determinado ('para') para captura sequencial dos dados de entrada; (c) aplicar estruturas de decisão encadeadas ('se...entao...senao') para classificar cada leitura em positiva, negativa ou nula; (d) gerenciar contadores e acumuladores aritméticos para mensurar o total de avanços e a soma dos recuos; e (e) exibir as saídas consolidadas com clareza em tela (FORBELLONE; EBERSPÄCHER, 2005).

## 3 FUNDAMENTAÇÃO TEÓRICA

Um algoritmo é definido formalmente como uma sequência finita de regras, operações e instruções lógicas destinadas a atingir um objetivo específico a partir de um conjunto inicial de dados (FORBELLONE; EBERSPÄCHER, 2005). Na programação estruturada, problemas que envolvem grandes volumes de informação do mesmo tipo de dado demandam a utilização de variáveis indexadas compostas unidimensionais, denominadas vetores ou arrays.

Diferentemente de variáveis simples que guardam apenas um valor por vez, o vetor ocupa um bloco contíguo na memória, permitindo o endereçamento dinâmico de seus compartimentos por meio de um índice numérico inteiro (MANZANO; OLIVEIRA, 2019). A iteração sobre essas estruturas é conduzida com eficiência pela estrutura 'para...de...ate...faca', cujo controle determinístico assegura que todas as posições sejam percorridas exatamente uma vez. Associada ao laço, a estrutura condicional aninhada 'se...entao...senao' permite segregar os valores de interesse: elementos maiores que zero incrementam o contador de avanços, elementos menores que zero alimentam o somatório de recuos, e leituras nulas (zero) são naturalmente ignoradas por representarem a inércia da esteira (SOUZA et al., 2011).

## 4 METODOLOGIA

A metodologia aplicada possui enfoque prático e descritivo, fundamentada na implementação em pseudocódigo utilizando o interpretador acadêmico VisuAlg 3.0. Inicialmente, procedeu-se à identificação e modelagem das variáveis do problema, alocando a estrutura 'sensor: vetor\[1..150\] de inteiro' e as variáveis escalares de apoio: a variável 'i' para controle dos laços iterativos, 'qtd_positivos' para contagem de frequência e 'soma_negativos' para o somatório acumulado.

Em seguida, estruturou-se o fluxo de execução em três fases sequenciais: primeiro, a inicialização obrigatória dos acumuladores em zero e a rotina de leitura de dados via laço determinístico; segundo, a varredura do vetor por uma nova repetição para aplicação da triagem lógica condicional; e terceiro, a apresentação dos resultados através de instruções de saída em tela. O algoritmo foi depurado no VisuAlg e submetido a testes de mesa prévios para assegurar integridade matemática e aderência às regras do enunciado.

## 5 DESENVOLVIMENTO DO ALGORITMO

O algoritmo foi intitulado 'Monitoramento_Esteira_Transporte'. Ele atende plenamente ao enunciado da atividade AVA 2, estruturando o programa em conformidade com as convenções do Portugol. Na seção 'var', declara-se o vetor de 150 posições e os contadores de apoio. No bloco 'inicio', as variáveis 'qtd_positivos' e 'soma_negativos' são zeradas, procedendo-se à captura sequencial das medições.

Após a entrada, um laço de repetição com índice 'i' de 1 até 150 analisa cada célula 'sensor\[i\]': se o valor for estritamente positivo (\> 0), soma-se 1 ao contador 'qtd_positivos'; caso seja estritamente negativo (\< 0), acumula-se o valor na variável 'soma_negativos'. Ao final, o comando 'escreval' formata a saída com espaçamento legível. O código-fonte integral é transcrito a seguir:

```text
algoritmo "Monitoramento_Esteira_Transporte"
// Disciplina : Bases de Programação - Universidade Veiga de Almeida
// Aluno(a) : [nome removido] (Matrícula: [matrícula removida]) - Data: 08/09/2026
// Descrição : Leitura de 150 leituras de sensor em vetor, contagem de avanços (> 0) e soma de recuos (< 0).
var
sensor : vetor[1..150] de inteiro // vetor para armazenar as 150 leituras
i : inteiro // contador do laço
qtd_positivos : inteiro // quantidade de valores positivos (avanço)
soma_negativos : inteiro // soma acumulada dos valores negativos (recuo)
inicio
// Inicialização das variáveis de controle
qtd_positivos <- 0
soma_negativos <- 0
// Leitura dos 150 valores do sensor
para i de 1 ate 150 faca
escreva("Digite a leitura do sensor (", i, "/150): ")
leia(sensor[i])
fimpara
// Processamento: contagem de positivos e soma de negativos
para i de 1 ate 150 faca
se sensor[i] > 0 entao
qtd_positivos <- qtd_positivos + 1
senao
se sensor[i] < 0 entao
soma_negativos <- soma_negativos + sensor[i]
fimse
fimse
fimpara
// Saída: exibição dos resultados consolidados
escreval("")
escreval("=== RESULTADOS DO MONITORAMENTO ===")
escreval("Quantidade de valores positivos (avanço): ", qtd_positivos)
escreval("Soma dos valores negativos (recuo): ", soma_negativos)
fimalgoritmo
```

Fonte: Elaborado pelo autor (2026).

## 6 RESULTADOS E CASOS DE TESTE

Para validar a acurácia dos cálculos e a ausência de inconsistências operacionais no código, realizou-se uma simulação controlada com uma massa de testes mista de 10 amostras: \[5, -3, 8, -2, 0, 12, -7, 0, 4, -1\].

Durante a execução, o algoritmo identificou quatro valores estritamente positivos (5, 8, 12 e 4), computando corretamente 'qtd_positivos = 4'. Paralelamente, identificou quatro valores estritamente negativos (-3, -2, -7 e -1), acumulando com exatidão 'soma_negativos = -13'. As duas leituras com valor zero foram desconsideradas tanto na contagem de avanços quanto no somatório de recuos, confirmando que a estrutura condicional aninhada atua de forma determinística e precisa. Os testes manuais conferiram integralmente com as saídas geradas pelo VisuAlg 3.0.

## 7 CONSIDERAÇÕES FINAIS

A elaboração deste trabalho permitiu exercitar de forma integrada os principais alicerces da programação estruturada na disciplina Bases de Programação. A utilização de vetores unidimensionais comprovou ser indispensável para manipular volumes consideráveis de amostras de forma eficiente, garantindo acesso direto por índice e otimização do fluxo de instruções.

A combinação entre o laço 'para' e as estruturas de decisão encadeadas 'se...entao...senao' ofereceu uma solução concisa e de alta legibilidade para o problema de monitoramento de esteiras mecânicas. Além de satisfazer integralmente as exigências técnicas da atividade AVA 2 da Universidade Veiga de Almeida, a prática reforça a importância de planejar algoritmos modularizados, testar casos de borda (como o valor zero) e seguir os padrões normativos da ABNT.

## Referências

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. **ABNT NBR 6023: Informação e documentação — Referências — Elaboração**. Rio de Janeiro: ABNT, 2018.

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. **ABNT NBR 10520: Informação e documentação — Citações em documentos — Apresentação**. Rio de Janeiro: ABNT, 2023.

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. **ABNT NBR 14724: Informação e documentação — Trabalhos acadêmicos — Apresentação**. Rio de Janeiro: ABNT, 2011.

FORBELLONE, André Luiz Villar; EBERSPÄCHER, Henri Frederico. **Lógica de programação: a construção de algoritmos e estruturas de dados**. 3. ed. São Paulo: Pearson Prentice Hall, 2005.

MANZANO, José Augusto N. G.; OLIVEIRA, Jayr Figueiredo de. **Algoritmos: lógica para desenvolvimento de programação de computadores**. 29. ed. São Paulo: Érica, 2019.

SOUZA, Marco Antônio Furlan de; GOMES, Marcelo Marques; SOARES, Marçalo; CONCILIO, Ricardo. **Algoritmos e lógica de programação: um texto introdutório para a engenharia**. 2. ed. São Paulo: Cengage Learning, 2011.
