// ============================================================
// UNIVERSIDADE VEIGA DE ALMEIDA - UVA
// CURSO DE GRADUAÇÃO EM ANÁLISE E DESENVOLVIMENTO DE SISTEMAS
// DISCIPLINA: BASES DE PROGRAMAÇÃO
// DATA: 08/09/2026
// ============================================================

algoritmo "Monitoramento_Esteira_Transporte"
// Disciplina : Bases de Programação
// Professor  : Universidade Veiga de Almeida
// Descrição  : Leitura de 150 valores inteiros de um sensor,
//              armazenamento em vetor, contagem de positivos
//              (avanço) e soma dos negativos (recuo).
// Data atual : 08/09/2026

var
    sensor : vetor[1..150] de inteiro   // vetor para armazenar as 150 leituras
    i : inteiro                          // contador do laço
    qtd_positivos : inteiro              // quantidade de valores positivos
    soma_negativos : inteiro             // soma dos valores negativos

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

    // Saída: exibição dos resultados
    escreval("")
    escreval("=== RESULTADOS DO MONITORAMENTO ===")
    escreval("Quantidade de valores positivos (avanço): ", qtd_positivos)
    escreval("Soma dos valores negativos (recuo): ", soma_negativos)

fimalgoritmo
