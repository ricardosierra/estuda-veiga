# E-book de Estatística — Walter Sande

> Disciplina: Estatística para Computação (E10300025). Seção: E10300025-21CMP2262A-2026.2.21. Período: 2026.2.21.
> Canvas: [item 1104270](https://uva.instructure.com/courses/57827/modules/items/1104270)
> PDF oficial (original preservado em `EST_ebook_Walter_Sande.pdf`): [https://ead.uva.br/disciplinas/grad/publica/cont/def/est/eb/EST_ebook_Walter_Sande.pdf](https://ead.uva.br/disciplinas/grad/publica/cont/def/est/eb/EST_ebook_Walter_Sande.pdf)
> Baixado em 2026-10-13 pela interface autenticada do Canvas e extraído com `pdftotext -layout -enc UTF-8`.
> Páginas do PDF: 140. Diagramas, imagens e conteúdo que exista somente como imagem permanecem no PDF original.

## Página 1

Estatística




  Rio de Janeiro
       UVA
       2016

## Página 2

Sem texto selecionável nesta página; consulte a página correspondente no PDF original.

## Página 3

  Walter Sande




Estatística




  Rio de Janeiro
       UVA
       2016

## Página 4

   Texto de acordo com as normas do Novo Acordo Ortográfico
                     da Língua Portuguesa.

                       ISBN: 978-85-69287-29-2

                        Autoria do Conteúdo
                            Walter Sande

                             Projeto Gráfico
                                  UVA

                              Diagramação
                             Isabelle Martins

                                Revisão
                             Janaína Senna
                             Isabel Oliveira
                             Lydianna Lima


   S214      Sande, Walter

                Estatística [livro eletrônico] / Walter Sande. – Rio de
             Janeiro : UVA, 2016.

                 2,4 MB.

                 ISBN 978-85-69287-29-2
                 Disponível também impresso.

                   1. Estatística. I. Universidade Veiga de Almeida.
             II. Título.

                                                       CDD – 519.5


    Ficha Catalográfica elaborada pelo Sistema de Bibliotecas da UVA.
           Biblioteca Maria Anunciação Almeida de Carvalho.

## Página 5

SUMÁRIO
Apresentação...............................................................................................................7
Sobre o autor...................................................................................................................9

Capítulo 1 - Estatística descritiva..................................11
           Diferentes tipos de dados.........................................................................13
           Resumos gráficos..........................................................................................15
           Medidas de tendência central..................................................................20
           Medidas de variação....................................................................................28
           Medidas de posição......................................................................................34
           Referências......................................................................................................40

Capítulo 2 - Distribuições de probabilidade...............41
           Introdução à probabilidade........................................................................42
           Variáveis aleatórias e distribuições de probabilidade.................51
           Valor esperado e variância......................................................................55
           Principais distribuições discretas...........................................................58
           Principais distribuições contínuas..........................................................63
           Referências......................................................................................................72

Capítulo 3 - Amostragem...............................................73
           Noções de inferência estatística e amostragem...............................74
           Intervalo de confiança...............................................................................78
           Teste de hipóteses.........................................................................................86
           Referências......................................................................................................100

Capítulo 4 - Covariância, correlação e regressão linear...101
           Diagrama de dispersão...............................................................................104
           Medida da associação entre duas variáveis.......................................107
           Regressão linear............................................................................................119
           Referências......................................................................................................135

Considerações finais......................................................136

## Página 6

Sem texto selecionável nesta página; consulte a página correspondente no PDF original.

## Página 7

                                                                            7




APRESENTAÇÃO

         Quantos filmes você assiste por mês? Qual
         a quantidade de filmes assistidos por mês
             pelas pessoas na sua faculdade?


A primeira pergunta é para uma pessoa em particular, e, portanto, só
há uma resposta. A segunda pergunta é para muitas pessoas, e, nesse
caso, a quantidade de filmes assistidos vai ser variada. Então, qual res-
posta a ser informada? Todas as idades? Vamos precisar da estatística
para encontrar traços desse conjunto e chegar a conclusões. Como ve-
remos mais adiante, podemos usar alguma medida de tendência cen-
tral, como a média, para responder.


Muitas vezes, recorremos à estatística para responder a alguns ques-
tionamentos do dia a dia. A estatística é uma ferramenta valiosa de
pesquisa que consiste em uma forma de aprender a partir de um con-
junto de dados. Seu objetivo é traduzir os dados em conhecimento e
compreensão para outras pessoas. Saber quantificar a incerteza em tor-
no desses dados também é necessário. Podemos dizer que um método
estatístico está dividido em quatro etapas:


      1. Formulação do problema estatístico de interesse.

      2. Coleta de dados.

      3. Análise dos dados, fase em que procuramos descrever os da-
      dos obtidos.

      4. Inferência dos resultados, momento em que tomamos deci-
      sões e fazemos previsões.



        Qual a quantidade média de filmes assistidos
         por mês pelas pessoas em sua faculdade?

## Página 8

8



    Para descobrirmos a verdadeira resposta, todos os alunos deveriam
    ser entrevistados. Mas isso não seria muito trabalhoso? Não existe um
    modo mais eficiente? Sim. Podemos escolher algumas pessoas ao acaso
    e calcular a média para elas, esperando que o valor encontrado seja
    uma boa indicação da média de todas as pessoas. Em termos mais for-
    mais, a ideia aqui, então, seria escolher uma amostra aleatória e calcu-
    lar a média para essa amostra, de modo a ser possível fazer inferências
    a respeito da média para a população.


    Toda pesquisa estatística possui um conjunto de elementos de interes-
    se, a partir do qual os dados são coletados e analisados. O conjunto de
    todos os elementos que possuem pelo menos uma característica em
    comum é chamado de população. No entanto, na prática, geralmente
    não dispomos da totalidade das observações de um fenômeno em estu-
    do, mas dados de apenas alguns desses elementos, pois seria inviável
    obter informações de todos, seja por falta de tempo ou de recursos.
    Chamamos de amostra esse subconjunto representativo da população,
    a partir do qual todos os elementos serão analisados para o estudo
    estatístico desejado.


    Quando temos um cálculo baseado na amostra e de alguma forma ten-
    tamos estimá-lo para a população inteira, chamamos de estatística, ou
    seja, uma característica da amostra. Esse valor que desejamos estimar
    é chamado de parâmetro, que é uma medida para descrever uma carac-
    terística da população.


    Sendo assim, podemos dizer que usamos a estatística descritiva para
    resumir os dados da amostra e, a partir de análises, usamos a esta-
    tística inferencial para tomar decisões e fazer previsões sobre uma
    população.

## Página 9

                                                                       9




SOBRE O AUTOR
Walter Wagner Carvalho Sande é doutor em Administração pela Esco-
la Brasileira de Administração Pública e de Empresas da Fundação Ge-
túlio Vargas – Ebape/FGV e mestre em Administração pelo Instituto de
Pós-Graduação e Pesquisa em Administração da Universidade Federal
do Rio de Janeiro – Coppead/UFRJ, formado em Engenharia Mecânica
pelo Instituto Militar de Engenharia – IME e em Análise de Sistemas
pela Universidade do Estado do Rio de Janeiro – Uerj. Após anos de
carreira profissional dedicada a empresas como Golden Cross, Varig,
Michelin, entre outras, atualmente é professor de graduação na FGV/
RJ e de pós-graduação em diversas instituições de ensino, além de se
dedicar à pesquisa, com foco nos métodos quantitativos aplicados às
ciências sociais.

## Página 10

      10




.........................................................................................................

## Página 11

                Diferentes tipos de dados                                                       11




                 CAPÍTULO 1
                 ESTATÍSTICA DESCRITIVA



                 Quando temos uma grande quantidade de dados e quere-
                 mos revelar algo sobre eles, sem ter que divulgá-los por
                 completo, podemos descrevê-los por meio de um conjunto
                 menor de números. Trata-se da estatística descritiva. Esses
                 resumos dos dados devem buscar evitar distorções e per-
                 da de muitas informações, constituindo-se, geralmente, em
                 gráficos e números, como médias e percentagens. A fim de
                 ilustrar o que se segue, consideremos o seguinte exemplo:


                 Exemplo 1: um pesquisador está interessado em analisar
                 algumas características dos alunos da faculdade em que
                 estuda. Usando as informações obtidas por meio de entre-
                 vistas a 30 pessoas escolhidas aleatoriamente, ele elaborou
                 a tabela 1.


                                            Tabela 1 – Tabela de dados.
                                                                          Filmes
                                                       Período
                   Id.                                                    assisti-   Nível de
                            Sexo    Idade    Altura       da       CR
                  Aluno                                                   dos por    simpatia
                                                      faculdade
                                                                           mês
                     1        F       20      1,65    Veterano      6        7         10
                     2        M       23      1,90    Veterano      6        4          1
                     3        F       18      1,60     Calouro      3        3          8
                     4        F       19      1,65    Veterano     6,6       8          2
                     5        M       20      1,92    Veterano     6,7      10          7
                     6        F       20      1,65    Veterano     6,8       7          2
                     7        M       18      1,70    Veterano     6,8       8          0




..........................................................................................................

## Página 12

      12                  Estatística descritiva




                              8         M          22    1,73     Veterano     6,9       9       9
                              9         F          19    1,60     Calouro      4,7       20      3
                             10         F          19    1,60     Veterano     7,4       9       8
                             11         F          24    1,59     Calouro      5,9       4       10
                             12         M          18    1,70     Calouro      6,5       5       2
                             13         M          20    1,70     Calouro      6,8       4       7
                             14         M          21    1,72     Calouro      6,8       6       3
                             15         F          20    1,60     Veterano     7,5       7       5
                             16         M          18    1,70     Calouro      7,5       6       7
                             17         M          19    1,74     Calouro      7,5       6       4
                             18         M          19    1,72     Calouro      7,5       6       2
                             19         M          17    1,77     Calouro      8,8       5       8
                             20         M          18    1,79     Veterano     7,5       7       9
                             21         M          19    1,76     Calouro      8,9       10      8
                             22         M          19    1,76     Veterano     7,5       11      1
                             23         M          20    1,75     Veterano     7,5       5       2
                             24         M          21    1,78     Veterano     7,7       7       3
                             25         M          22    1,76     Veterano     7,7       6       6
                             26         M          20    1,83     Calouro      9,8       9       6
                             27         M          22     1,8     Veterano     7,9       6       8
                             28         M          23     1,8     Veterano     7,9       6       1
                             29         M          24     1,8     Veterano      8        4       4
                             30         M          19    1,79     Calouro      9,9       8       9


                                            Fonte: Elaborado pelo autor com dados hipotéticos.




.........................................................................................................

## Página 13

                Diferentes tipos de dados                                                       13




                 DIFERENTES TIPOS DE DADOS




                 Uma variável é qualquer característica medida ou avalia-
                 da em cada elemento da amostra ou população. Como o
                 próprio nome diz, seus valores (observações) variam de
                 elemento para elemento.


                 Diz-se que uma variável é qualitativa (ou categórica) se
                 cada observação pertence a um subconjunto de um con-
                 junto de categorias, que correspondem aos possíveis atri-
                 butos do elemento pesquisado. Esses possíveis resultados
                 ainda podem ser classificados como ordinais, quando
                 existe uma noção de ordenação ou hierarquia entre eles,
                 ou nominais, em caso contrário. Muitas vezes utiliza-
                 mos números para representar variáveis qualitativas, por
                 exemplo, 1 para o sexo feminino e 2 para o masculino. No
                 entanto, esses valores têm função puramente de classifica-
                 ção e, portanto, não podem ser operados aritmeticamente,
                 muito menos serem usados para ordenar as categorias ou
                 lhes atribuir algum tipo de valor comparativo.


                 Dizemos que uma variável é quantitativa (ou numérica)
                 se as observações assumem valores numéricos resultantes
                 de um processo de contagem ou mensuração. As variá-
                 veis quantitativas ainda possuem uma classificação dico-
                 tômica: (a) discretas, se possuem uma quantidade finita
                 ou enumerável de valores possíveis, (b) contínuas, se pos-
                 suem uma quantidade infinita de valores possíveis, per-
                 tencentes a um intervalo de números reais.




..........................................................................................................

## Página 14

      14                  Estatística descritiva




                          A tabela 2 apresenta a classificação de algumas das variá-
                          veis presentes no exemplo 1:


                                               Tabela 2 – Tipos de dados (classificação).



                                  Variável            Observações possíveis            Classificação

                                                           Feminino (F) e
                                    Sexo                                            Qualitativa nominal
                                                           masculino (M)
                                 Período da                  Calouro e
                                                                                     Qualitativa ordinal
                                 faculdade                   veterano

                                   Idade                   …,17,18,19,…            Quantitativa discreta


                                   Altura            …,1,60,…,1,65,…,1,90,…        Quantitativa contínua




.........................................................................................................

## Página 15

                Resumos gráficos                                                                15




                 RESUMOS GRÁFICOS




                 Os dados podem ser sumarizados e descritos por meio da
                 construção de gráficos e tabelas. É sempre uma boa ideia
                 analisar inicialmente os dados por meio de gráficos, pois
                 isso ajuda na obtenção do feeling acerca dos fenômenos
                 observados, proporcionando maiores informações a res-
                 peito do comportamento de cada variável.


                 Como temos dois tipos bem distintos de variáveis, é de se
                 esperar que tenham tipos de gráficos que se adequem a
                 um conjunto de dados melhor do que outros.


                 Para as variáveis qualitativas, usamos o gráfico de pizza
                 e o gráfico de barras. Em um gráfico de pizza, cada fatia
                 corresponde à porcentagem de observações de cada atri-
                 buto observado da variável analisada. Por exemplo, pela
                 figura 1, podemos verificar que a variável “Período da fa-
                 culdade” possui apenas dois atributos possíveis, calouros
                 e veteranos, de tal forma que, do total das entrevistas
                 realizadas, 43% das observações foram respondidas por
                 calouros, e 57%, por veteranos.




..........................................................................................................

## Página 16

      16                  Estatística descritiva




                                                   Figura 1 – Gráfico de pizza.
                                                      Período da faculdade




                          Como é possível verificar pela figura 2, a diferença para
                          um gráfico de barras é que, em vez de fatias, temos barras
                          verticais, e a altura de cada barra corresponde à contagem
                          de ocorrências de cada atributo associado à variável.

                                                   Figura 2 – Gráfico de barras.
                                                      Período da faculdade




                          Para as variáveis quantitativas, em geral, usamos histo-
                          gramas, bem semelhantes aos gráficos de barras verticais.
                          Nesse tipo de gráfico, a ideia é definir faixas de valores e
                          calcular a frequência de observações pertencentes a cada
                          um desses intervalos. Essas frequências podem ser abso-
                          lutas (contagem de ocorrência) ou relativas (porcentagens



.........................................................................................................

## Página 17

                Resumos gráficos                                                                17



                 do total), como mostra a figura 3. Ou seja, o histograma é
                 criado em cima desse resumo (tabela) de frequências.


                                            Figura 3 – Histogramas.
                                   Histograma de idades (frequência absoluta)




                                   Histograma de idades (frequência relativa)




                 Forma das distribuições
                 As distribuições também podem ser classificadas pelo
                 formato de gráfico que exibem. Se uma linha vertical tra-
                 çada sobre a mediana produz duas imagens espelhadas,
                 dizemos que a distribuição é simétrica. Se a distribuição
                 apresenta todas as observações com a mesma frequência
                 (o que, em um gráfico de barras, corresponde a todas as
                 barras com mesma altura), classificamos a distribuição
                 como sendo uniforme, o que também permite classificá-la
                 como simétrica. A figura 4 apresenta exemplos de ambos
                 os gráficos.




..........................................................................................................

## Página 18

      18                  Estatística descritiva




                                                    Figura 4 – Distribuições simétricas.
                                                          Distribuição simétrica




                                                          Distribuição uniforme




                          Caso contrário, como mostra a figura 5, temos uma dis-
                          tribuição assimétrica, que ainda pode ser classificada em
                          dois tipos, de acordo com o lado que registra a maior fre-
                          quência: assimétrica à esquerda (ou negativamente assi-
                          métrica) e assimétrica à direita (ou positivamente assimé-
                          trica).
                                                   Figura 5 – Distribuições assimétricas.
                                                     Distribuição assimétrica à direita




.........................................................................................................

## Página 19

                Resumos gráficos                                                                19



                                   Distribuição assimétrica à esquerda




..........................................................................................................

## Página 20

      20                  Estatística descritiva




                          MEDIDAS DE TENDÊNCIA CENTRAL




                          Além dos resumos gráficos, também devemos considerar
                          os resumos numéricos da amostra. Uma medida de ten-
                          dência central é um valor único cuja função é representar
                          o conjunto de dados como um todo, por meio da identifi-
                          cação de um valor típico (ou central). Ou seja, representa
                          um valor em torno do qual os dados se agrupam.


                          As três medidas mais utilizadas são a moda, a média e a
                          mediana. Uma forma de entendermos o quanto cada uma
                          dessas medidas é representativa (ou não) dentro do con-
                          junto de dados que estamos analisando é por meio do au-
                          xílio dos gráficos tratados no tópico anterior.


                          Moda
                          A moda é o valor que ocorre com mais frequência em um
                          conjunto de dados, se ele existir. Se não temos números
                          repetidos, se não temos um que seja mais comum, então
                          não temos moda.


                          Trata-se de uma medida que é mais usada, em geral, para
                          identificar a categoria de uma variável qualitativa que te-
                          nha a frequência mais alta. Portanto, um gráfico de pizza é
                          de grande auxílio nesse tipo de identificação. Pelo gráfico
                          da figura 1, verifica-se que a categoria veterano é a mais
                          frequente para a variável período da faculdade. Também
                          podemos identificar a moda de uma variável quantitativa,
                          como a idade. A tabela 3 apresenta a frequência em que




.........................................................................................................

## Página 21

                Medidas de tendência central                                                    21



                 os valores ocorrem para o conjunto de dados coletados.
                 Conclui-se que a maioria das pessoas entrevistadas tem
                 19 anos.


                                        Tabela 3 – Frequência de idades.


                                           Idade           Frequência
                                               17                1
                                               18                5
                                               19                8
                                               20                7
                                               21                2
                                               22                3
                                               23                2
                                               24                2


                 Em alguns casos, podemos ter duas ou mais modas. Con-
                 siderando o exemplo, um dos interesses do pesquisador
                 foi medir o nível de simpatia dele na faculdade. Então,
                 para isso, cada pessoa entrevistada teve que lhe atribuir
                 um valor de uma escala de 0 (extremamente antipático)
                 a 10 (extremamente simpático) de quão simpático ela o
                 considera. A figura 6 mostra o histograma resultante des-
                 se estudo. Podemos verificar que os alunos da faculdade
                 são bastante divididos quanto ao nível de simpatia que
                 atribuem ao pesquisador, pois algumas pessoas o consi-
                 deram muito simpático, enquanto outras o consideram
                 muito antipático.




..........................................................................................................

## Página 22

      22                  Estatística descritiva




                                                    Figura 6 – Distribuição bimodal.
                                                           Nível de simpatia




                                          Fonte: Elaborado pelo autor com dados hipotéticos.



                          Nesse caso, podemos concluir que essa distribuição apre-
                          senta duas modas, 2 e 8, e que ambas possuem frequência
                          igual a 5. Trata-se, então, de uma distribuição bimodal.


                          Média
                          A medida mais conhecida e mais utilizada de tendência
                          central de uma variável quantitativa é a média. Existem
                          muitos tipos de média, mas estamos considerando aqui a
                          média aritmética, ou seja, aquela que corresponde à soma
                          de todas as observações da variável em questão dividido
                          pelo número delas. Podemos interpretá-la como o ponto
                          de equilíbrio da distribuição.


                          Sejam x1, x2, …, xn os n valores de uma variável X, distintos
                          ou não, a média amostral, denotada por x̅ (lê-se x barra),
                          é dada por:


                                                                                   ∑ i xi
                                                                                       n
                                                      x1 + x2 + ... + xn
                                             x̅ =                              =
                                                               n                       n




.........................................................................................................

## Página 23

                Medidas de tendência central                                                    23



                 Retornando ao exemplo, sabemos que um dos interesses
                 do pesquisador foi perguntar aos alunos de sua faculdade
                 sobre a quantidade de filmes que eles assistem por mês.
                 Os resultados para os calouros e veteranos podem ser vi-
                 sualizados pelos gráficos da figura 7.


                                      Figura 7 – Filmes assistidos por mês.
                                     Filmes assistidos por mês (calouros).




                                     Filmes assistidos por mês (veteranos)




                 A média de filmes assistidos é calculada para os calouros
                 da seguinte forma:


                3 + (2×4) + (2×5) + (4×6) + 8 + 9 + 10 + 2 × 20                   92
                                                                              =        = 7,07
                                               14                                 14




..........................................................................................................

## Página 24

      24                  Estatística descritiva




                          E para os veteranos:


                         (3×4) + 5 + (3×6) + (5×7) + (2×8) + 9 + 10 + 11       116
                                                                           =         = 6,82
                                                   17                          17


                          Portanto, a média de filmes assistidos por mês é maior
                          para os calouros. Analisando o gráfico dos calouros, ve-
                          rifica-se que o fato de a média ser 7,07 tem uma grande
                          influência da pessoa que respondeu que assiste 20 filmes
                          por mês. Se tivéssemos desconsiderado essa observação, a
                          média teria sido 6, ou seja, a observação com valor 20 des-
                          locou a média para a direita. Por outro lado, analisando o
                          gráfico dos veteranos, a média 6,82 representa bem uma
                          medida central da distribuição, pois não sofreu distorções
                          de nenhum elemento muito distante do restante da distri-
                          buição. Portanto, a média é uma boa medida para o centro
                          da distribuição dos veteranos.


                          A média é muito sensível aos valores que ficam bem dis-
                          tantes do resto da distribuição. Ou seja, a média pode ser
                          altamente influenciada pelo que chamamos de outliers,
                          observações que possuem valores muito acima ou muito
                          abaixo da grande parte dos dados. A suspeita de outliers
                          requer uma investigação mais aprofundada sobre os da-
                          dos coletados, pois podem ter sido originados por um erro
                          na entrada de dados ou podem corresponder realmente a
                          ocorrências incomuns da variável analisada.




                          Mediana
                          A mediana corresponde ao valor do meio em um conjunto
                          de dados que tenha sido ordenado do menor para o maior




.........................................................................................................

## Página 25

                Medidas de tendência central                                                      25



                 (ou do maior para o menor). É uma boa solução quando
                 temos uma observação com valor atípico, que pode dis-
                 torcer a média.


                 Se a quantidade de observações é ímpar, então a mediana
                 corresponde exatamente à observação do meio do conjun-
                 to ordenado. No caso de ser par, a mediana corresponde
                 à média entre os dois valores que estão no meio na or-
                 dem de classificação. Consideremos as n observações da
                 variável X colocadas em ordem crescente, de modo que a
                 menor observação seja denotada por x(1) e a maior por x(n) :
                 x(1) ≤ x(2) … ≤ x(n–1) ≤ x(n).


                 As observações ordenadas dessa forma recebem o nome
                 de estatísticas de ordem. E, com isso, podemos calcular a
                 mediana de X.




                                Med (X) =
                                                   {   X

                                                       X
                                                         ( ), se n ímpar
                                                               n+1
                                                                   2


                                                         ( ), se n par
                                                               n
                                                               2




                 Por exemplo, vamos considerar a variável X que corres-
                 ponde à quantidade de filmes que os alunos assistem por
                 mês. Ordenando as n = 30 observações podemos calcular
                 a mediana.


                  1    2    3    4    5        6   7       8           9   10 11 12 13 14 15 16
                  3    4    4    4    4        4   5       5           5    6 6 6 6 6 6 6


                      17 18 19 20 21 22 23 24 25 26 27 28 29 30
                       7 7 7 7 7 8 8 8 9 9 10 10 11 20




..........................................................................................................

## Página 26

      26                  Estatística descritiva




                                                           x15 + x16            6+6
                                           Med (X) =                        =           =6
                                                                2                 2


                          Vamos supor que só tivéssemos as 29 primeiras observa-
                          ções, então Med(X) = x15 = 6.


                           1     2     3     4     5   6    7       8   9   10 11 12 13 14 15 16
                           3     4     4     4     4   4    5       5   5    6 6 6 6 6 6 6


                                     17 18 19 20 21 22 23 24 25 26 27 28 29
                                      7 7 7 7 7 8 8 8 9 9 10 10 11


                          Comparando média e mediana
                          A forma como os dados estão distribuídos influencia se a
                          média é maior, igual ou menor do que a mediana.


                          Como mostra a figura 8, para distribuições perfeitamente
                          simétricas, média e mediana coincidem. Para uma distri-
                          buição assimétrica, a média é puxada na direção da cauda
                          mais longa em relação à mediana. Logo, distribuições as-
                          simétricas à direita possuem média maior do que a me-
                          diana. Para distribuições assimétricas à esquerda, ocorre
                          o inverso.


                                      Figura 8 – Média e mediana em diferentes distribuições.




                                              Fonte: Adaptado de Agresti e Franklin (2013).




.........................................................................................................

## Página 27

                Medidas de tendência central                                                    27



                 Pelo fato de a mediana não ser afetada por observações
                 com valores extremos, dizemos que ela é uma medida
                 mais robusta (resistente a esse efeito) do que a média.


                 Portanto, se temos uma distribuição assimétrica, em geral
                 escolhemos como medida de tendência central a mediana,
                 pois representa melhor o que é típico. Quando temos uma
                 distribuição simétrica ou bem perto disso, em geral a mé-
                 dia é escolhida, pois usa os valores numéricos de todas as
                 observações.




..........................................................................................................

## Página 28

      28                  Estatística descritiva




                          MEDIDAS DE VARIAÇÃO




                          Apenas as medidas de tendência central não são suficien-
                          tes para descrever as variáveis quantitativas. Precisamos
                          também de resumos numéricos que nos ajudem a compre-
                          ender o quanto os dados analisados variam entre si.


                                                   Figura 9 – Variabilidade dos dados.




                          Na figura 9, temos dois conjuntos de dados diferentes,
                          mas ambos com a mesma média (x̅ = y̅ ). Embora eles te-
                          nham a mesma média, a distribuição dos valores em cada
                          um desses conjuntos é diferente. Os dados em cinza-claro
                          estão bem mais dispersos, mais distantes da média, do
                          que os dados em cinza-escuro. Como podemos medir essa
                          dispersão em torno da média? As medidas mais frequen-
                          temente usadas são a amplitude, a variância e o desvio
                          médio padrão e servem para medir a representatividade
                          da média.


                          Para exemplificar o cálculo das medidas de variação nas
                          subseções a seguir, vamos utilizar as observações da vari-
                          ável coeficiente de rendimento – CR analisada pelo pesqui-



.........................................................................................................

## Página 29

                Medidas de variação                                                             29



                 sador do exemplo. Dividiremos essas observações em dois
                 conjuntos: XA: calouros e XB: veteranos, conforme mostra a
                 tabela 4. Para ambos os conjuntos temos que a moda (7,5),
                 a mediana (7,5) e a média (7,2) são iguais.


                                      Tabela 4 – CR dos calouros e veteranos.


                         CR dos                              CR dos
                                          Frequência                        Frequência
                        calouros                            veteranos
                             3                  1                6              2
                            4,7                 1               6,6             1
                            5,9                 1               6,7             1
                            6,5                 1               6,8             2
                            6,8                 2               6,9             1
                            7,5                 3               7,4             1
                            8,8                 1               7,5             4
                            8,9                 1               7,7             2
                            9,8                 1               7,9             2
                            9,9                 1                8              1


                 Como mostram os gráficos da figura 10, temos dois con-
                 juntos com mesmas moda, mediana e média, mas com va-
                 riabilidades dos dados distintas.


                                          Figura 10 – Distribuição do CR.
                                                 CR dos calouros




..........................................................................................................

## Página 30

      30                  Estatística descritiva




                                                   CR dos veteranos




                          Amplitude
                          A amplitude (total) da distribuição nos dá a ideia da faixa
                          máxima de variação dos dados, sendo calculada pela dife-
                          rença entre o maior e o menor valores observados.


                                                   R = xmáx – xmín


                          Apesar de ser facilmente calculada, a amplitude não é uma
                          medida muito utilizada em cálculos estatísticos, pois ig-
                          nora os valores numéricos de quase todas as observações,
                          trabalhando apenas com os dois valores extremos. Desse
                          modo, podemos ter dois conjuntos com os mesmos va-
                          lores extremos, o que resulta em uma mesma amplitude,
                          mas com os dados distribuídos de modo bem distinto, não
                          nos permitindo afirmar nada sobre a variabilidade.


                          Para os dados do exemplo, temos que RA= 9,8 – 3 = 5,8 e
                          RB= 8,2 – 4,9 = 3,3. Ou seja, há indícios de que o CR dos
                          calouros esteja mais disperso do que o dos veteranos, pois
                          a faixa de variação do CR é mais ampla para os calouros
                          do que para os veteranos.




.........................................................................................................

## Página 31

                Medidas de variação                                                             31




                 Variância e desvio-padrão
                 Acabamos de ver que o porquê de a amplitude não ser
                 uma medida muito eficiente para realizar afirmações acer-
                 ca da variabilidade dos dados. Talvez nos fosse mais útil
                 uma medida que levasse em consideração todas as n ob-
                 servações da variável analisada, em vez de apenas duas
                 delas. Como estamos interessados em medir a dispersão
                 dos dados em torno da média, uma boa solução seria co-
                 meçarmos calculando a dispersão de cada observação em
                 relação à média x̅ , calculando a diferença entre cada valor
                 observado, xi, e a média, x̅ : (xi – x̅ ).


                            Tabela 5 – Calculando a variância do CR dos calouros.


                                  CR dos
                                                   (xi – x̅ )      (xi – x̅ )2
                                 calouros
                                      3             -4,2            17,64
                                      4,7           -2,5            6,25
                                      5,9           -1,3            1,69
                                      6,5           -0,7            0,49
                                      6,8           -0,4            0,16
                                      6,8           -0,4            0,16
                                      7,5            0,3            0,09
                                      7,5            0,3            0,09
                                      7,5            0,3            0,09
                                      8,8            1,6            2,56
                                      8,9            1,7            2,89
                                      9,8            2,6            6,76
                                      9,9            2,7            7,29
                                  TOTAL               0             46,16


                 Podemos verificar pela tabela 5 que existem desvios po-
                 sitivos e negativos. De fato, observações acima da média
                 possuem (xi – x̅ ) > 0, e observações abaixo da média pos-



..........................................................................................................

## Página 32

      32                  Estatística descritiva




                          suem (xi – x̅ ) < 0. Como a média corresponde ao ponto de
                          equilíbrio da distribuição, ao somarmos esses desvios, os
                          valores negativos se cancelam com os valores positivos,
                          totalizando zero. Solução? Eliminar o sinal negativo dos
                          desvios. Para isso, ou trabalhamos com os desvios abso-
                          lutos (|xi – x̅ |) ou com o quadrado dos desvios ((xi – x̅ )2).
                          Por questões matemáticas que envolvem peculiaridades
                          de cada função, a segunda opção é escolhida. A expressão
                            n
                          ∑i=1 (xi – x̅ )2 recebe o nome de soma dos quadrados.


                          Com isso, definimos variância (σ2) como a soma dos qua-
                          drados das diferenças de cada observação em relação à
                          média (soma dos desvios ao quadrado), dividida pelo ta-
                          manho da amostra.


                                                           n
                                                         ∑i=1 (xi – x̅ )2
                                                   σ =
                                                    2
                                                                n


                          Calculada dessa forma, essa medida é chamada de vari-
                          ância populacional. Veremos nos próximos capítulos que,
                          para calcular a variância amostral, representada por S2,
                          devemos repetir o mesmo procedimento apresentado, no
                          entanto, usando o denominador n – 1 em vez de n.


                          Como podemos observar, a variância representa uma mé-
                          dia dos quadrados dos desvios dos valores observados em
                          relação à média (pois, no final das contas, é uma divisão
                          entre a soma de valores pela quantidade de valores nessa
                          soma). No entanto, como esses desvios são elevados ao
                          quadrado, resulta que as unidades de medida da média,
                          x̅ , e da variância, σ2, não são as mesmas, o que dificulta
                          bastante o uso da variância com a finalidade de analisar a
                          dispersão em torno da média.


.........................................................................................................

## Página 33

                Medidas de variação                                                             33



                 Uma medida de dispersão usada com maior frequência é
                 o desvio-padrão (σ), que corresponde à raiz quadrada da
                 variância e pode ser interpretado como a distância média
                 de uma observação a partir da média.


                 Um desvio-padrão próximo de zero significa que os dados
                 estão próximos da média. Conjuntos de dados com valo-
                 res muito afastados da média apresentam desvio-padrão
                 elevado.


                 Para os dados do exemplo, temos que:


                                      46,16
                           σA2 =              ≈ 3,55   → σA = √3,55 ≈ 1,88
                                       13


                                      6,42
                           σB2 =              ≈ 0,37   → σB = √0,37 ≈ 0,61
                                       17


                 Portanto, como era de se esperar, o CR dos calouros apre-
                 senta uma variância maior do que o CR dos veteranos.
                 Como quanto maior for o desvio, mais dispersos em rela-
                 ção à média são os valores observados, os resultados aci-
                 ma comprovam o que havíamos observado anteriormente,
                 ao analisarmos os gráficos e calcularmos a amplitude de
                 cada conjunto de dados.




..........................................................................................................

## Página 34

      34                  Estatística descritiva




                          MEDIDAS DE POSIÇÃO




                          Além das medidas já estudadas, devemos considerar algu-
                          mas outras, pois elas podem não ser suficientes para uma
                          sumarização adequada do conjunto de dados analisado.
                          São necessárias medidas que nos ajudem a compreender
                          melhor a simetria (ou assimetria) da distribuição de dados.


                          Já vimos que a mediana é uma medida mais robusta que a
                          média para valores extremos. Será que não podemos cal-
                          cular outras medidas semelhantes, mas que sejam capa-
                          zes de nos dar mais informações?


                          Quantis
                          Sabemos que a mediana é uma medida que divide o con-
                          junto amostral ordenado, do menor para o maior, em duas
                          partes, de modo que metade das observações encontra-se
                          abaixo, e a outra metade, acima dela. De modo geral, po-
                          demos definir uma medida capaz de dizer o quão longe
                          uma observação está em relação a um determinado ponto.
                          Chamamos essa medida de quantil. A figura 11 mostra
                          um quantil de ordem p, denotado por q(v), que correspon-
                          de a uma proporção p (0 < p < 1), tal que p por cento (p%)
                          das observações são menores ou iguais a esse valor. En-
                          tão, para p = 50, temos o 50º percentil, em que 50% das
                          observações estão abaixo de q(0,50). E isso corresponde
                          exatamente à mediana!




.........................................................................................................

## Página 35

                Medidas de posição                                                              35



                                     Figura 11 – Quantil de ordem p.




                 Os quantis mais usados são os quartis, que, como o pró-
                 prio nome já induz, correspondem a quartos do conjunto.
                 Ao dividir o conjunto total em quatro partes iguais, temos:


                        Q1 = q(0,25): corresponde ao primeiro quartil.


                        Q2 = q(0,50): corresponde ao segundo quartil, tam-
                        bém chamado de mediana.


                        Q3 = q(0,75): corresponde ao terceiro quartil.


                 Isso significa dizer que 25% dos dados estão abaixo do
                 primeiro quartil, 25% estão entre o primeiro e o segundo
                 quartis, 25% estão entre o segundo e o terceiro quartis e
                 25% estão acima do terceiro quartil.


                 Vamos considerar as observações do CR dos calouros.


                        1. Ordenam-se em ordem crescente os dados para
                        encontrar a mediana; com isso, temos Q2.




..........................................................................................................

## Página 36

      36                  Estatística descritiva




                           3     4,7    5,9       6,5   6,8   6,8   7,5   7,5   7,5   8,8       8,9   9,8   9,9
                                                                    Q2


                                   2. Encontra-se a mediana da metade inferior; com
                                   isso, temos Q1. Encontra-se a mediana da metade
                                   superior; com isso, temos Q3.


                           3     4,7    5,9       6,5   6,8   6,8   7,5   7,5   7,5   8,8       8,9   9,8   9,9
                                                                    Q2
                                       5,9 + 6,5                                      8,8 + 8,9
                            Q1 =                        = 6,2              Q3 =                       8,85
                                              2                                             2


                          Amplitude interquartil
                          Uma outra forma de medir a variabilidade dos dados é por
                          meio da amplitude interquartil. Trata-se de uma medida
                          de dispersão melhor do que a amplitude total, pois des-
                          considera os valores dos extremos. A sua principal van-
                          tagem é que não é afetada pela presença de outliers na
                          distribuição, pois não leva em consideração observações
                          abaixo do primeiro quartil nem as que estão acima do ter-
                          ceiro quartil.


                          A amplitude interquartil é calculada pela diferença entre o
                          terceiro e o primeiro quartis.


                                                              ΔQ = Q3 – Q1


                          Quanto mais os dados variarem em uma distribuição,
                          maior o valor de ΔQ. Para o CR dos calouros, temos que
                          ΔQA = 8,85 – 6,2 = 2,65, e dos veteranos, ΔQB = 7,7 – 6,75 =
                          0,95, ou seja, o CR dos calouros apresenta maior variação.




.........................................................................................................

## Página 37

                Medidas de posição                                                              37




                 Detectando possíveis outliers
                 Outra importante função da amplitude interquartil está re-
                 lacionada ao fato de ela funcionar com uma ferramenta na
                 identificação de potenciais outliers da distribuição. Uma
                 observação é classificada como sendo potencial outlier se
                 ela se encontra a mais do que 1,5 × ΔQ abaixo do primeiro
                 quartil (cerca inferior) ou mais do que 1,5 × ΔQ acima do
                 terceiro quartil (cerca superior).


                                     Cerca inferior = Q1 – 1,5 × ΔQ
                                     Cerca superior = Q3 + 1,5 × ΔQ


                 Para o CR dos calouros, temos que possíveis outliers se-
                 riam valores menores que 6,2 – 1,5 × 2,65 = 2,225 ou maio-
                 res que 8,85 + 1,5 × 2,65 = 12,825. E, para o CR dos vetera-
                 nos, seriam valores menores que 6,75 – 1,5 × 0,95 = 5,325
                 ou maiores que 7,7 + 1,5 × 0,95 = 12,825. Em nenhum
                 desses conjuntos aparecem observações fora dessa faixa,
                 portanto, não há suspeita da existência de outliers.


                 É importante ressaltar que, com esse critério, estamos
                 apenas identificando valores como outliers em potencial,
                 mas não dando a certeza dessa classificação. Em alguns
                 casos, como nas distribuições que apresentam o que se
                 chama de cauda longa, embora distantes da maior parte
                 dos dados, alguns valores extremos podem ser encontra-
                 dos, sem que isso configure um caso raro ou praticamente
                 impossível de ocorrer. Sendo assim, esses dados, mesmo
                 fora da faixa determinada pelas cercas superior e inferior,
                 não podem ser classificados como outliers.




..........................................................................................................

## Página 38

      38                  Estatística descritiva




                          Box plot: o resumo gráfico dos cinco números
                          Os quartis, juntamente com os valores mínimo e máximo
                          da distribuição, formam o que chamamos de resumo dos
                          cinco números. Esses números, em conjunto, funcionam
                          como a base de um gráfico denominado box plot, que for-
                          nece informações sobre posição, dispersão, assimetria,
                          caudas e outliers.


                          A posição central é dada pela mediana, e a dispersão, pelo
                          tamanho da caixa, que corresponde à amplitude inter-
                          quartil. As posições relativas dos quartis dão uma noção
                          sobre a assimetria da distribuição. As linhas verticais que
                          partem da caixa até os valores mínimo e máximo corres-
                          pondem aos comprimentos das caudas esquerda e direita,
                          respectivamente. Os pontos que ficam além dessa estrutu-
                          ra correspondem aos possíveis outliers.


                          A figura 12 apresenta box plots correspondentes às obser-
                          vações do CR dos calouros (à esquerda) e dos veteranos
                          (à direita).


                                                        Figura 12 – Box plots.
                                               CR dos calouros versus CR dos veteranos




.........................................................................................................

## Página 39

                Medidas de posição                                                              39



                 Comparando os gráficos, notamos que, no box plot do CR
                 dos calouros, os dados encontram-se mais dispersos em
                 relação à média (pois a caixa tem um tamanho maior), com
                 uma certa simetria entre a maior parte dos dados (pois a
                 mediana está bem próxima ao centro da caixa), e há pre-
                 sença de valores extremos de grande magnitude (o que
                 se vê pelo grande comprimento das linhas verticais). Já
                 no box plot do CR dos veteranos, observamos dados bem
                 concentrados ao redor da média (pois o tamanho da caixa
                 é menor), com uma assimetria à direita (a maior parte dos
                 dados se encontra abaixo da mediana), e que os valores
                 extremos não estão muito distantes da média (já que as
                 linhas não são muito longas).




..........................................................................................................

## Página 40

      40                  Estatística descritiva




                                           REFERÊNCIAS


                          AGRESTI, A.; FRANKLIN, C. A. Statistics: the art and science
                          of learning from data. 3. ed. Boston: Pearson, 2013.

                          ANDERSON, D. R.; SWEENEY, D. J.; WILLIAMS, T. A. Esta-
                          tística aplicada à administração e economia. 3. ed. São
                          Paulo: Cengage Learning, 2014.

                          BUSSAB, W. O.; MORETTIN, P. A. Estatística básica. 8. ed.
                          São Paulo: Saraiva, 2013.

                          LEVINE, D. M. et al. Estatística: teoria e aplicações: usando o
                          Microsoft Excel em português. 6. ed. Rio de Janeiro: LTC, 2013.

                          MCCLAVE, J. T.; BENSON, P. G.; TERRY, S. Estatística para
                          administração e economia. 10. ed. São Paulo: Pearson
                          Prentice Hall, 2009.

                          STEVENSON, W. J. Estatística aplicada à administração.
                          São Paulo: Harper & Row do Brasil, 1981.



                          Links interessantes

                          Portal Action – Estatística básica:
                          <http://www.portalaction.com.br/estatistica-basica>.

                          Portal Action – Estatística descritiva:
                          <http://www.portalaction.com.br/estatistica-basica/esta-
                          tisticas-descritivas>.

                          Portal Action – Análises gráficas:
                          <http://www.portalaction.com.br/estatistica-basica/grafi-
                          cos>.



.........................................................................................................

## Página 41

                Introdução à probabilidade                                                      41




                 CAPÍTULO 2
                 DISTRIBUIÇÕES DE
                 PROBABILIDADE



                 Preciso sair com um guarda-chuva hoje? Quantas pessoas
                 estarão na fila do almoço no horário de pico? Quantos pro-
                 dutos a empresa irá vender no próximo mês? Quanto tem-
                 po irá demorar até o fornecedor entregar a encomenda que
                 fizemos hoje? Quantos produtos defeituosos irão escapar
                 do controle de qualidade?


                 Todos os dias, estamos sempre tomando decisões sobre si-
                 tuações que, por mais que sejam observadas repetidamen-
                 te sob as mesmas condições, geram resultados distintos
                 e não previsíveis com certeza absoluta. Conhecemos os
                 possíveis resultados, mas, por causa da aleatoriedade, o
                 resultado de uma determinada observação é incerto. Nes-
                 te capítulo, vamos, portanto, abordar esse assunto e seus
                 desdobramentos.




..........................................................................................................

## Página 42

      42                  Distribuições de probabilidade




                          INTRODUÇÃO À PROBABILIDADE




                          A probabilidade é uma medida capaz de mensurar essa
                          incerteza, de modo a indicar o quão provável é a ocor-
                          rência desse evento de interesse. O processo de coleta de
                          dados associados a um fenômeno em que obtemos resul-
                          tados com essas características é chamado de experimen-
                          to aleatório.


                          No entanto, apesar da incerteza associada ao resultado de
                          uma observação futura de um experimento aleatório, exis-
                          te uma certa previsibilidade estatística, garantida pela lei
                          dos grandes números. Ao lançarmos uma moeda, sabe-
                          mos intuitivamente que temos 50% de chance de cair cara,
                          e 50% de cair coroa. Essa é uma noção intuitiva, que vem
                          do conhecimento implícito de que há dois resultados pos-
                          síveis, ambos com mesma chance de ocorrer. Quando re-
                          petimos o lançamento quatro vezes seguidas, é de esperar
                          que em dois deles tenhamos cara e, nos outros dois, coroa.
                          Porém, é possível que ocorram quatro caras, ou quatro
                          coroas... Contudo, como mostra a figura 13, com um nú-
                          mero de observações suficientemente grande, a frequên-
                          cia relativa de um resultado possível (cara, por exemplo)
                          se estabiliza e converge para um número específico (50%,
                          nesse caso), o qual é esperado. Essa proporção de vezes
                          que o resultado cara ocorre, no longo prazo, é a base do
                          conceito de probabilidade.




.........................................................................................................

## Página 43

                Introdução à probabilidade                                                      43



                                    Figura 13 – A lei dos grandes números.
                                       Frequência acumulada de “cara”




                 Portanto, quando lidamos com experimentos aleatórios, a
                 frequência que um determinado resultado ocorre no curto
                 prazo (ou seja, com poucas repetições do experimento) é alta-
                 mente aleatória, mas, à medida que repetimos o experimento
                 uma quantidade grande de vezes, ela se torna bem mais pre-
                 visível. Essa proporção de vezes que um determinado resul-
                 tado ocorre no longo prazo corresponde a sua probabilidade.


                 Como calcular essas probabilidades? A fim de ilustrar o que
                 se apresenta a seguir, consideremos o seguinte exemplo.


                 Exemplo 2: a disciplina de Estatística é cursada por alu-
                 nos de três cursos diferentes: Administração, Economia
                 e Matemática. Como o nível exigido de matemática varia
                 entre os cursos, a solução encontrada pelo professor foi
                 aplicar três modelos de prova (A, B e C, respectivamente)
                 que reflitam esse nível de exigência. Em uma turma de 60
                 alunos, 15 deles (25%) fizeram a prova A, que reprova o
                 aluno com uma probabilidade 0,4; 36 (60%) fizeram a pro-
                 va B, que reprova o aluno com uma probabilidade 0,25; e
                 9 (15%) fizeram a prova C, que reprova o aluno com uma
                 probabilidade 0,33.


..........................................................................................................

## Página 44

      44                  Distribuições de probabilidade




                          Conceitos fundamentais
                          Inicialmente devemos listar todos os casos possíveis. O
                          conjunto composto por todos os resultados possíveis ori-
                          ginados a partir de um experimento aleatório é denomina-
                          do espaço amostral, representado normalmente por S ou
                          Ω. A figura 14 exibe a árvore de probabilidades corres-
                          pondente ao exemplo 2, com S = {AR, AR̅ , BR, BR̅ , CR, CR̅ }, em
                          que R corresponde à categoria de reprovado, e R̅ , à de não
                          reprovado (aprovado). Ou seja, o evento AR corresponde à
                          proporção de alunos que fizeram a prova A e foram repro-
                          vados, enquanto o evento AR̅ , à proporção de alunos que
                          fizeram a prova A e não foram reprovados.


                                              Figura 14 – Árvore de probabilidades.




                          Pergunta-se, então: escolhendo aleatoriamente um aluno
                          dessa turma, qual a probabilidade de que ele tenha feito
                          a prova A?


                          Um evento E é o subconjunto desse espaço amostral for-
                          mado por todos os resultados possíveis que respondam a
                          nossa pergunta de interesse.




.........................................................................................................

## Página 45

                Introdução à probabilidade                                                      45



                           A = alunos que fizeram a prova A = {AR, AR̅ }


                 Cada resultado do espaço amostral e, consequentemente,
                 cada evento têm uma probabilidade de ocorrência associa-
                 da. Se o modelo é equiprobabilístico1, a probabilidade de
                 um evento E, indicada por P(E), é dada por:


                                      Número de resultados do evento E
                  P(E) =
                             Número total de resultados do espaço amostral


                 Dessa forma,


                                  Número de alunos que fizeram a prova A
                      P(A) =
                                             Total do número de alunos


                                                      15
                                             P(A) =        = 0,25
                                                      60


                 No nosso exemplo, AR e AR̅ são eventos mutuamente ex-
                 cludentes, ou seja, não podem ocorrer simultaneamente,
                 pois um aluno que tenha feito a prova A não pode ter sido
                 reprovado e não reprovado simultaneamente. Em outras
                 palavras, a interseção entre AR e AR̅ é o conjunto vazio (AR
                 ∩ AR̅ = Ø). Sendo assim, podemos calcular P(A) somando
                 diretamente as probabilidades dos resultados individuais
                 que compõem o evento A: P(A) = P(AR) + P(AR̅ ) = 0,1 + 0,15
                 = 0,25. Quando, ao contrário, dois eventos não são mu-
                 tuamente excludentes, devemos subtrair a probabilidade
                 associada à interseção entre eles da regra da soma.



                 1 Ou seja, todos os elementos do modelo têm a mesma probabilidade de
                 serem “sorteados”. Nesse nosso exemplo, todos.




..........................................................................................................

## Página 46

      46                  Distribuições de probabilidade




                          Podemos dizer que a probabilidade é uma função que
                          associa um número real P(E) a cada evento E do espaço
                          amostral S, satisfazendo às seguintes regras:


                                   1. Para todo evento E ∈ S, 0 ≤ P(E) ≤ 1. Em outras
                                   palavras, a probabilidade é um número entre 0 e
                                   um, inclusive.

                                   2. P(Ø) = 0 (evento impossível). Em outras palavras,
                                   a probabilidade associada a um evento impossível
                                   (com nenhum resultado possível) é zero.

                                   3. P(S) = 1 (evento certo). Ou seja, um evento que te-
                                   nha todas as ocorrências do espaço amostral como
                                   resultados possíveis tem probabilidade um.

                                   4. ∑EϵS P(E) = 1. Ou seja, a soma das probabilidades
                                   de todos os eventos do espaço amostral é um.



                          Probabilidade conjunta, marginal e
                          condicional
                          Outra forma de ilustrar o exemplo 2 é por meio de uma
                          tabela de dupla entrada (ou tabela de contingência), como
                          a apresentada pela tabela 6.


                                                Tabela 6 – Tabela de dupla entrada.

                                                             Reprovação
                             Graduação                                                TOTAL
                                                      Não                 Sim
                           Administração
                                                      0,15                0,1         0,25
                             (Prova A)
                              Economia
                                                      0,45                0,15         0,6
                              (Prova B)
                             Matemática
                                                       0,1                0,05        0,15
                              (Prova C)
                               TOTAL                   0,7                0,3          1




.........................................................................................................

## Página 47

                Introdução à probabilidade                                                      47



                 Podemos, então, fazer uma outra pergunta: escolhendo
                 aleatoriamente um aluno dessa turma, qual a probabilida-
                 de que ele tenha feito a prova B e não tenha sido reprova-
                 do em Estatística?


                 Nessa tabela, cada uma das seis probabilidades centrais
                 é chamada de probabilidade conjunta, calculada pela in-
                 terseção dos resultados das variáveis. A soma de todas
                 as probabilidades conjuntas é igual a 1. Para responder a
                 essa pergunta, cruzamos a linha da Prova B com a coluna
                 Não, ou seja, P(B e R̅ ) = 0,45.


                 Outras possíveis perguntas são: escolhendo aleatoriamen-
                 te um aluno dessa turma, qual a probabilidade de que ele
                 não tenha sido reprovado em Estatística? Qual a probabi-
                 lidade de que ele tenha feito a prova B?


                 Na tabela 6, os valores presentes na linha Total e coluna
                 Total são chamados de probabilidades marginais e levam
                 em consideração somente a probabilidade de uma variá-
                 vel. As probabilidades marginais são obtidas por meio da
                 soma das probabilidades conjuntas em ambas as direções
                 da tabela. Portanto, a probabilidade de que um aluno es-
                 colhido ao acaso não tenha sido reprovado, independen-
                 temente do modelo de prova que tenha feito, é calculada
                 pela seguinte expressão:


                   P(R̅ ) = P(AR̅ ) + P(BR̅ ) + P(CR̅ ) = 0,15 + 0,45 + 0,1 = 0,7




..........................................................................................................

## Página 48

      48                  Distribuições de probabilidade




                          E a probabilidade de esse mesmo aluno ter feito a prova B,
                          independentemente de ter sido aprovado, é:


                                       P(B) = P(BR) + P(BR̅ ) = 0,45 + 0,15 = 0,6


                          Temos agora uma nova pergunta: escolhendo aleatoria-
                          mente um aluno dessa turma, sabendo que ele fez a prova
                          B, qual a probabilidade de que ele não tenha sido reprova-
                          do em Estatística?


                          Agora que já sabemos calcular as probabilidades conjun-
                          tas e marginais, já podemos calcular a probabilidade con-
                          dicional. A probabilidade de um evento A ocorrer, dado
                          que o evento B já ocorreu, é representada por P(A│B) e cal-
                          culada dividindo-se a probabilidade conjunta dos eventos
                          A e B pela probabilidade de ocorrência do evento B.


                                                                 P(A e B)
                                                      P(A│B) =
                                                                   P(B)


                          Ou seja, sabemos o resultado de uma das variáveis (ter
                          feito a prova B) e queremos, com isso, calcular a probabili-
                          dade de a outra variável ocorrer (não reprovação).


                                                                    P(B e R̅ )       0,45
                         P(não reprovou│fez a prova B) =                         =          = 0,75
                                                                       P(B)          0,6


                          Note que, ao calcularmos a probabilidade condicional, re-
                          duzimos o espaço amostral ao espaço do evento condicio-
                          nante, que, nesse caso, é fazer a prova B.




.........................................................................................................

## Página 49

                Introdução à probabilidade                                                      49




                 Probabilidade total e teorema de Bayes
                 Vamos pensar, agora, em outra pergunta: escolhendo ale-
                 atoriamente um aluno dessa turma, qual a probabilidade
                 de que ele tenha feito a prova B sabendo-se que ele não
                 reprovou em Estatística?


                 Se, na árvore de probabilidades da figura 14, trocarmos
                 a ordem entre os três nós que correspondem ao tipo de
                 prova e os dois nós que correspondem a ter havido ou não
                 reprovação, a probabilidade conjunta não seria alterada.
                 De fato, reescrevendo a fórmula da probabilidade condi-
                 cional, obtemos:


                              P(A e B) = P(A│B) ⋅ P(B)=P(B│A) ⋅ P(A)


                 Ou seja,


                                                          P(B│A) ⋅ P(A)
                                       P(A│B) =
                                                                P(B)


                 Essa fórmula, que expressa uma probabilidade condicio-
                 nal em termos de outras probabilidades condicionais, é
                 conhecida como o teorema de Bayes.


                 Podemos calcular P(B) a partir de probabilidades condicio-
                 nais de B para os diferentes valores de A. Essa é a chama-
                 da lei da probabilidade total.


                               P ( B ) = P ( B │ A 1) P ( A 1) + P ( B │ A 2) P ( A 2)
                                         + ⋯ = ∑ P ( B │ A n) P ( A n)




..........................................................................................................

## Página 50

      50                  Distribuições de probabilidade




                          Dessa forma,


                                                                   P(B│A) P(A)
                                                P(A│B) =
                                                                ∑ P ( B │ A n) P ( A n)


                          Agora já podemos calcular a probabilidade P(fez a prova
                          B|não reprovou).


                                                                   P(R̅ │B)P(B)
                              P(B│R̅ ) =
                                               (P(R̅ │A)P(A) + P(R̅ │B)P(B) + P(R̅ │C)P(C)


                                                                   0,75 × 0,6
                                  P(B│R̅ ) =
                                                   0,6 × 0,25 + 0,75 × 0,6 + 0,67 × 0,15


                                                                 0,45
                                          P(B│R̅ ) =                               ≈ 0,65
                                                           0,15 + 0,45 + 0,1


                          Independência dos eventos
                          Dizemos que dois eventos, A e B, são independentes se o
                          fato de um evento ter ocorrido não altera a probabilidade
                          de ocorrência do outro evento. Ou seja, P(B│A) = P(B).
                          Portanto,


                                   P(A e B) = P(A)P(B) ⇔ A e B são independentes


                          No exemplo 2, para que os eventos sejam considerados
                          independentes, a relação de independência deve ser válida
                          para todas as interseções presentes na tabela 6.




.........................................................................................................

## Página 51

                 Variáveis aleatórias e distribuições de probabilidade                          51




                 VARIÁVEIS ALEATÓRIAS E DISTRIBUIÇÕES
                 DE PROBABILIDADE




                 Nem sempre estamos interessados apenas nos resultados
                 dos experimentos aleatórios, e sim em funções numéri-
                 cas associadas a esses resultados. Nesse contexto, surge o
                 conceito de variável aleatória (v. a.), uma função que as-
                 socia cada elemento do espaço amostral a um número real
                 (X: Ω→R). Ou seja, trata-se de um valor numérico originado
                 de algum experimento aleatório.


                 Uma variável aleatória X pode assumir um conjunto de
                 valores possíveis, x1, x2, …, xn, cada um com uma probabi-
                 lidade pi associada, sendo ∑pi=1, para i entre 1 e n. Dessa
                 forma, temos a distribuição de probabilidade dessa variá-
                 vel como nosso principal objetivo de estudo neste ponto.
                 Sendo assim, as distribuições de probabilidade são capa-
                 zes de descrever os possíveis resultados de uma variável
                 aleatória e suas probabilidades.


                 Existem dois tipos de variáveis aleatórias: discretas e con-
                 tínuas. Ambas serão discutidas a seguir.


                 Variáveis aleatórias discretas
                 Uma variável aleatória discreta é aquela que possui um con-
                 junto de valores possíveis finito ou infinito, porém enume-
                 rável. Vamos supor uma pesquisa fictícia sobre a quantida-
                 de total de disciplinas feitas pelos alunos de Administração
                 durante um período. Sabe-se que há um limite de cinco ma-
                 térias por período. Trata-se de um exemplo de uma v. a.



..........................................................................................................

## Página 52

      52                  Distribuições de probabilidade




                          discreta, pois sabemos todos os possíveis valores {0, 1, 2, 3,
                          4 e 5} da variável X = “quantidade de disciplinas cursadas
                          em um período por um aluno de Administração”. A tabela 7
                          apresenta um resumo dos resultados dessa pesquisa.


                                              Tabela 7 – Distribuição de frequências.



                                                           X           Frequência
                                                           1                15
                                                           2                20
                                                           3                45
                                                           4                50
                                                           5                60
                                                      Total                 190


                          A primeira coluna é a lista de possíveis valores da variável
                          X, e a segunda coluna é a quantidade de vezes que cada
                          valor ocorreu na pesquisa.


                          Função (massa) de probabilidade
                          A distribuição de probabilidade de uma v. a. discreta é
                          chamada de função (massa) de probabilidade. Nesse
                          caso, podemos especificar a probabilidade de a variável
                          X ser igual a um determinado valor x, que representamos
                          por pX (x) ou P(X = x). A tabela 8 mostra a distribuição de
                          probabilidade para essa variável.


                                            Tabela 8 – Distribuição de probabilidades.


                                             X                 Frequência         P(X = x)
                                              1                   15                0,08
                                              2                   20                0,11
                                              3                   45                0,24




.........................................................................................................

## Página 53

                 Variáveis aleatórias e distribuições de probabilidade                          53




                                     4                     50            0,26
                                     5                     60            0,32
                                   Total                  190            1,00


                 Nessa tabela, a terceira coluna é o resultado da divisão
                 da frequência de cada valor pelo total de casos contados
                 (190). Sendo assim, representa a probabilidade de que,
                 sorteando-se um aluno qualquer que tenha respondido a
                 essa pesquisa, sua resposta tenha sido cada um dos valo-
                 res da variável X.


                 Variáveis aleatórias contínuas
                 Uma variável aleatória contínua é aquela que possui um
                 conjunto de valores possíveis infinito não numerável. Va-
                 mos supor uma pesquisa fictícia sobre o total de tempo
                 que cada aluno passa estudando Estatística. Trata-se de
                 um exemplo de uma v. a. contínua, pois não somos ca-
                 pazes de definir uma faixa precisa de todos os possíveis
                 resultados para a variável X = “tempo gasto por um aluno
                 estudando Estatística”.


                 Função densidade de probabilidade
                 A distribuição de probabilidade de uma v.a. contínua é
                 chamada de função densidade de probabilidade (f. d. p.).
                 Nesse caso, só podemos especificar a probabilidade de a
                 variável X estar entre os valores a e b, que representamos
                 por P(a < X < b). Não há como calcular a probabilidade de
                 ocorrer um resultado exato para a variável X.




..........................................................................................................

## Página 54

      54                  Distribuições de probabilidade




                                         Figura 15 – Função densidade de probabilidade.




                          Como podemos verificar na figura 15, isso significa que,
                          graficamente, estamos considerando a área abaixo da cur-
                          va determinada pelo intervalo de interesse. Ou seja:


                                                              b
                                           P(a < X < b) = ∫a f(x)dx = F(b) – F(a)

                                               +∞
                          Com f(x)≥0 e ∫– ∞ f(x)dx = 1, e onde F(a) e F(b) são distribui-
                          ções acumuladas, como explicado a seguir.


                          Função de distribuição acumulada
                          A função de distribuição acumulada (f. d. a.) calcula a pro-
                          babilidade de a v. a. X ser menor ou igual a um determina-
                          do valor x. Para uma v. a. discreta, temos F(x) = P(X ≤ x) e,
                                                                          x
                          para uma v. a. contínua, temos F(x)= ∫– ∞ fx(x)dx.


                          Sendo assim, conseguimos calcular a probabilidade de
                          uma variável X estar dentro de um intervalo.


                                                    P(a < X ≤ b) = F(b) – F(a)




.........................................................................................................

## Página 55

                Valor esperado e variância                                                      55




                 VALOR ESPERADO E VARIÂNCIA




                 Com a distribuição de probabilidade definida, podemos
                 calcular algumas estatísticas de resumo, tais como a mé-
                 dia e a variância, assim como fizemos no Capítulo 1 com
                 os dados observacionais.


                 Valor esperado
                 O valor esperado (ou esperança matemática ou média) de
                 uma variável aleatória X, denotado por E(X), é uma medida
                 que dá mais (menos) ênfase aos valores mais (menos) pro-
                 váveis da variável aleatória, ou seja, valores elevados de X
                 ocasionam E(X) grande, assim como valores pequenos de
                 X ocasionam E(X) baixo.


                 A média μX de uma distribuição de probabilidade corres-
                 ponde ao valor esperado da variável aleatória X, ou seja,
                 μX = E(X).


                 Então, o que significa dizer que o número esperado de
                 pessoas em uma fila é três? Significa que, observando o
                 número de pessoas na fila durante vários dias, teremos,
                 em média, três pessoas.


                 Para uma v. a. discreta X, o valor esperado E(X) é uma mé-
                 dia ponderada de todos os possíveis valores de X com pe-
                 sos iguais às respectivas probabilidades desses valores.




..........................................................................................................

## Página 56

      56                  Distribuições de probabilidade




                                                      E(X) = ∑ x ⋅ P(X = x)
                                                              x∈S




                          Por exemplo, para os dados da tabela 8, temos que:


                                       E(X) = (1 × 0,08) + (2 × 0,11) + (3 × 0,24)
                                               + (4 × 0,26) + (5 × 0,32) ≈ 3,63


                          Note que esse valor de 3,63 nunca irá ocorrer!


                          Para uma v. a. contínua X, o raciocínio é o mesmo. No en-
                          tanto, substituímos o símbolo do somatório pela integral
                          e modificamos a função de probabilidade.


                                                               ∞
                                                      E(X) = ∫-∞ x ⋅ fX (x)dx


                          Propriedades do valor esperado
                          Ao calcular o valor esperado de uma variável aleatória, é
                          pertinente ter o conhecimento de algumas propriedades
                          associadas a essa medida.


                                   1. E(aX + b) = E(aX) + E(b) = aE(X) + b, em que a e b
                                   são constantes.

                                   2. E(X + Y) = E(X) + E(Y), em que X e Y são variáveis
                                   aleatórias.

                                   3. E(XY) = E(X)E(Y), se e somente se X e Y forem in-
                                   dependentes.



                          Variância
                          A variância corresponde ao valor esperado do quadrado
                          de quanto uma variável aleatória X se afasta de seu valor
                          esperado e é calculada pelas seguintes fórmulas:



.........................................................................................................

## Página 57

                Valor esperado e variância                                                      57



                                Var(X) = E[(X – E(X))2] = E(X2 ) – [E(X)]2


                                                                   ∞
                 Quando X é uma v. a. contínua, Var(X) = ∫-∞ (x – E(X))2 f(x)dx.


                 O valor dado por X – E(X) corresponde ao desvio de X em
                 relação à sua média.


                 Se uma variável aleatória Y é uma combinação linear da
                 variável X, de tal forma que Y = aX + b (com a e b constan-
                 tes), temos a seguinte propriedade:


                              Var(Y) = Var(aX + b) = a2 Var(X) + Var(b)
                                        = a2 Var(X) + 0 = a2 Var(X)


                 Ou seja, a soma de um valor constante a uma variável ale-
                 atória não altera a sua variância, mas a multiplicação dela
                 por um valor constante faz com que a variância fique mul-
                 tiplicada pelo quadrado desse valor.


                 Além disso, se X e Y são independentes, temos que:


                                       Var(X + Y) = Var(X) + Var(Y)




..........................................................................................................

## Página 58

      58                  Distribuições de probabilidade




                          PRINCIPAIS DISTRIBUIÇÕES DISCRETAS




                          Distribuição de Bernoulli
                          Exemplo 3: experimentos indicam que os alunos de Ad-
                          ministração que cursam Estatística têm 75% de chance de
                          passar na matéria. Qual a probabilidade de um aluno des-
                          ta turma ser aprovado em Estatística?


                          Vamos considerar uma variável aleatória X que possui
                          apenas dois resultados possíveis, mutuamente excluden-
                          tes: fracasso e sucesso, representados por 0 e 1, respec-
                          tivamente. Com probabilidade p, ocorre sucesso, e, com
                          probabilidade q = (1 – p), ocorre fracasso.


                          Quando uma v. a. X tem essas características, dizemos que
                          ela tem distribuição de Bernoulli com probabilidade p, e a
                          representamos da seguinte forma:


                                                           X ~ Be(p)


                          A função de probabilidade de Bernoulli é dada por:




                                                 P(X = x) =
                                                           {     p, se x = 1
                                                                 q, se x = 0
                                                                 0, c.c.


                          Portanto:
                          E(X) = 0 × q + 1 × p = p
                          Var(X) = [(02 × q) + (12 × p)] – p2 = p (1 – p) = pq



.........................................................................................................

## Página 59

                 Principais distribuições discretas                                             59



                 Para o exemplo 3, X = “ser aprovado ou não em Estatís-
                 tica” tem distribuição de Bernoulli, representada por X ~
                 Be(0,75). Ou seja, há uma probabilidade de 0,75 de suces-
                 so (ser aprovado).


                 Distribuição binomial
                 Exemplo 4: experimentos indicam que os alunos de Ad-
                 ministração que cursam Estatística têm 75% de chance de
                 passar na matéria. Se sortearmos três alunos dessa turma,
                 qual a probabilidade de que nenhum deles seja aprovado
                 em Estatística? E de que pelo menos dois sejam aprovados?


                 Muitas vezes não estamos interessados em saber se um
                 sucesso ocorre ou não, mas na quantidade de vezes em
                 que ele ocorre. Nesse tipo de distribuição, o número de
                 experimentos a serem realizados é conhecido (serão fei-
                 tos n ensaios, cada qual com distribuição de Bernoulli), e
                 a v. a. X corresponde ao número de sucessos observados
                 nas n tentativas do processo de Bernoulli realizadas. Cada
                 experimento tem a mesma probabilidade p de sucesso e é
                 independente de todos os outros. Trata-se da distribuição
                 binomial, representada por:


                                                      X ~ Bin(n; p)


                 A função de probabilidade da distribuição binomial é dada
                 por:


                                               n                      n!
                            P(X = x) =      ( x ) p q = x! (n – x)! p q
                                                        x   n–x            x   n–x




..........................................................................................................

## Página 60

      60                  Distribuições de probabilidade




                          O termo px qn – x indica a probabilidade de exatamente x su-
                          cessos ocorrerem nas n observações em uma determinada
                          sequência de resultados. E o termo x
                                                              n
                                                                         ( )
                                                                  indica a quanti-
                          dade de combinações existentes de x sucessos ocorrerem
                          em sequências de n observações. Por isso, a probabilidade
                          de x sucessos é dada pelo produto do número de todas
                          as sequências possíveis pela probabilidade de sucesso de
                          uma determinada sequência.


                          Para a distribuição binomial, temos


                                           E(X) = E(X1) + E(X2) + ... + E(Xn) = np


                                   Var(X) = Var(X1) + Var(X2) + ... + Var(Xn) = npq


                          Isso ocorre porque as variáveis X1, X2, ..., Xn ~ Be(p) e todas
                          elas são independentes entre si.


                          Para o exemplo 4, X = “número de alunos aprovados em
                          Estatística” tem distribuição binomial, representada por X
                          ~ Bin(3; 0,75). Sendo assim,


                                         P(X = 0) =        ( 30 ) 0,75 × 0,25 ≈ 0,016
                                                                    0       3




                          Isso corresponde à probabilidade de que nenhum aluno
                          seja aprovado em Estatística. Além disso:


                                        P (X ≥ 2) = 1 – P(X = 0) – P(X = 1) = 1 –

                                                       ( )
                                          0,016 – 3 0,751 × 0,252 ≈ 0,844
                                                     1




.........................................................................................................

## Página 61

                 Principais distribuições discretas                                                 61



                 E isso corresponde à probabilidade de que dois ou mais
                 alunos sejam aprovados em Estatística2.


                 Distribuição de Poisson
                 Exemplo 5: a fila da secretaria de uma faculdade tem, em
                 média, quatro alunos de administração por hora. Qual a
                 probabilidade de, em uma hora, não ter nenhum aluno de
                 Administração na fila? E de ter pelo menos dois alunos?


                 A princípio, podemos pensar em utilizar a distribuição bi-
                 nomial para resolver o exemplo 5. Contudo, não sabemos
                 o número (n) de testes e, consequentemente, desconhece-
                 mos também o número de fracassos ocorridos. Acontece
                 que não estamos interessados no número de sucessos ob-
                 tidos em n tentativas, mas no número de sucessos ocorri-
                 dos dentro de um espaço contínuo de tempo, representa-
                 do pela variável X. Dizemos que X segue uma distribuição
                 de Poisson, em que o parâmetro corresponde à frequência
                 média de sucesso, denotada por λ, e não à probabilidade p
                 de ocorrência de um sucesso. Representamos essa distri-
                 buição da seguinte forma:


                                                       X ~ Poi(λ)


                 A função de probabilidade da distribuição de Poisson é
                 dada por:


                                                      P(X = x) = e-λ λ
                                                                      x

                                                                     x!

                 2 Como os eventos (X ≥ 2) = “número de alunos aprovados em Estatística é
                 maior ou igual a 2” e (X < 2) = “número de alunos aprovados em Estatística
                 é menor que 2” são mutuamente excludentes, P(X ≥ 2) = 1 – P(X < 2). Além
                 disso, P(X < 2) = P(X = 0) + P(X = 1). Logo, P(X ≥ 2) = 1 – P(X = 0) – P(X = 1).




..........................................................................................................

## Página 62

      62                  Distribuições de probabilidade




                          Há uma relação interessante entre a distribuição binomial
                          e a distribuição de Poisson. Quando X ~ Bin(n; n λ ), se fi-

                          zermos n → ∞, de modo a ter um número de tentativas
                          suficientemente grande (espaço de tempo contínuo), a dis-
                          tribuição binomial se aproxima da distribuição de Poisson.


                                                               λ ) ≈ Poi(λ)
                                                    lim Bin(n; n
                                                    n →∞




                          Portanto,


                                                           E(X) = Var(X) = λ


                          Para o exemplo 5, X = “número de alunos de Administra-
                          ção na fila da secretaria no horário de pico” tem distribui-
                          ção de Poisson, representada por X ~ Poi(4). Sendo assim:


                                                 P(X = 0) = e-4 4 ≈ 0,0183
                                                                     0

                                                                0!


                          Isso corresponde à probabilidade de não haver alunos na
                          fila da secretaria no horário de pico. Além disso:


                                        P(X ≥ 2) = 1 – [P(X = 0) + P(X = 1)] = 1–
                                              [0,0183 + e-4 4 ] = 0,9084
                                                              1

                                                             1!


                          E isso corresponde à probabilidade de haver dois ou mais
                          alunos.




.........................................................................................................

## Página 63

                 Principais distribuições contínuas                                             63




                 PRINCIPAIS DISTRIBUIÇÕES CONTÍNUAS




                 Distribuição uniforme
                 Exemplo 6: as aulas de Estatística começam às 7h30 toda
                 segunda-feira. O tempo de chegada de um determinado
                 aluno é uniformemente distribuído, de modo que ele che-
                 ga entre 7h15 e 7h45. Qual a probabilidade de esse aluno
                 chegar cinco minutos depois de a aula ter começado?


                 Seja X uma v. a. contínua que assume todos os possíveis
                 valores do intervalo [a,b], se a probabilidade de se obter
                 qualquer resultado é proporcional ao tamanho do inter-
                 valo, e a probabilidade de ocorrência de resultados com
                 intervalos de tamanhos iguais é a mesma, dizemos que X
                 possui distribuição uniforme sobre o intervalo [a,b] e o
                 representamos da seguinte forma:


                                                      X ~ U(a,b)


                                         Figura 16 – Distribuição uniforme.




..........................................................................................................

## Página 64

      64                  Distribuições de probabilidade




                          Como apresentado na figura 16, a f. d. p. da distribuição
                          uniforme é constante sobre o intervalo em que a variável
                                                                                                  ∞
                          X está definida. Como ∫-∞ f(x)dx = 1, a probabilidade de
                          ocorrência de cada resultado corresponde ao inverso do
                          tamanho do intervalo do espaço amostral.




                                              f(x) =
                                                       {      1 , se a ≤ x ≤ b
                                                             b–a
                                                                    0,      c.c.



                          Sendo assim, a f. d. a. de X é:




                                                             {
                                                                          0,       se x < a
                                       F(x) = P(X ≤ x) =             x – a , se a ≤ x ≤ b
                                                                     b–a
                                                                       1,   se x > b


                          Portanto,


                                                                         a+b
                                                           E(X) =
                                                                           2


                                                                         (b – a)2
                                                       Var(X) =
                                                                           12


                          Para o exemplo 6, X ~ U(–15,15), e a probabilidade de o
                          aluno chegar no máximo cinco minutos depois da aula ter
                          começado é calculada da seguinte forma:


                                                                     5 – (–15)                2
                                         P(X ≤ 5) = F(10) =                             =
                                                                     15 – (–15)               3




.........................................................................................................

## Página 65

                 Principais distribuições contínuas                                             65




                 Distribuição exponencial
                 Exemplo 7: na fila da secretaria de uma faculdade, che-
                 gam, em média, quatro alunos de Administração por
                 hora. Qual a probabilidade de passar mais de 30 minutos
                 (0,5 hora) até chegar um aluno de Administração na fila
                 da secretaria?


                 A distribuição exponencial é geralmente associada ao
                 tempo de espera e modela tempos entre dois eventos
                 consecutivos que seguem uma distribuição de Poisson.
                 Vamos supor que os eventos corram segundo uma distri-
                 buição de Poisson, com uma taxa média de λ eventos por
                 unidade de tempo. Seja X o número de eventos ocorridos
                 no intervalo [0,t], então X ~ Poi(λt). Se T é uma v. a. contí-
                 nua que corresponde ao tempo de ocorrência do primeiro
                 evento, dizemos que T tem distribuição exponencial e o
                 representamos da seguinte forma:


                                                      T ~ Exp(λ)


                 Existe, portanto, uma relação entre a distribuição de Pois-
                 son e a distribuição exponencial. A distribuição de Pois-
                 son analisa a quantidade de observações por intervalo
                 (evento discreto), e a distribuição exponencial analisa o
                 intervalo por observação. As distribuições exponencial e
                 de Poisson correspondentes possuem o mesmo parâme-
                 tro λ.


                 A f. d. p. da distribuição exponencial é dada por:



                                            f(t) =
                                                      {  λe-λt, se t≥0

                                                            0, c.c.




..........................................................................................................

## Página 66

      66                  Distribuições de probabilidade




                          Sendo assim, a f. d. a. de T é:




                                         F(t) = P(T ≤ t) =
                                                              {              0, se t < 0

                                                                      1 – e-λt, se t ≥ 0



                          Portanto,


                                                                        1
                                                            E(T) =
                                                                        λ


                                                                         1
                                                           Var(T) =
                                                                         λ2


                          O gráfico da distribuição está ilustrado na figura 17.


                                               Figura 17 – Distribuição exponencial.




                          Uma propriedade muito importante da distribuição expo-
                          nencial é o que chamamos de falta de memória, ou seja, a
                          ocorrência ou não de um evento não afeta a probabilidade
                          de eventos futuros.




.........................................................................................................

## Página 67

                 Principais distribuições contínuas                                                   67




                                                 P(T ≥ s + t)        e-λ(s + t)
                  P(T ≥ s + t | T ≥ s) =                         =                = e-λt = P(T ≥ t)
                                                      P(T ≥ s)         e-λs


                 Para o exemplo 7, X ~ Poi(4) e T ~ Exp(4). Então:


                             P(T ≥ 0,5) = 1 – P(T ≤ 0,5) = e-4 × 0,5 = 0,135


                 Distribuição normal
                 A distribuição normal, também chamada de distribuição
                 Gaussiana, é a distribuição mais utilizada na estatística.
                 Além do fato de muitas variáveis aleatórias possuírem
                 essa distribuição, quando há um número suficientemen-
                 te grande de possíveis resultados, outras distribuições se
                 aproximam dela. Veremos isso mais à frente, quando tra-
                 tarmos do teorema central do limite.


                 Como mostra a figura 18, a distribuição normal apresenta
                 simetria ao redor da média μ, e sua curva possui o forma-
                 to de sino, cuja largura depende do desvio-padrão σ.


                                          Figura 18 – Distribuição normal.




                 Quando aumentamos (diminuímos) o valor de μ, desloca-
                 mos o gráfico para a direita (esquerda); quando aumenta-



..........................................................................................................

## Página 68

      68                  Distribuições de probabilidade




                          mos (diminuímos) o valor de σ, a curva torna-se mais (me-
                          nos) larga (espalhada) e, consequentemente, mais (menos)
                          achatada. Esses dois parâmetros caracterizam a distribui-
                          ção normal por completo. Se a variável X segue uma dis-
                          tribuição normal, representamos da seguinte forma:


                                                           X ~ N(μ, σ2)


                          A f. d. p. da distribuição normal é dada por:



                                                                 {– 2 ( σ ) }
                                                     1                 1   x–μ   2
                                         f(x) =            exp
                                                   σ√2π


                          Os possíveis valores que X pode assumir se estendem pelo
                          intervalo (-∞, +∞). No entanto, o formato da distribuição
                          torna bastante improvável que ocorram os valores pró-
                          ximos aos extremos. Para valores muito grandes, tanto
                          negativos quanto positivos, a densidade de probabilidade
                          se aproxima de zero, mas nunca será zero de fato (a curva
                          nunca toca o eixo x).


                          Para essa distribuição, temos:


                                                            E(X) = μ


                                                           Var(X) = σ2


                          Distribuição normal padrão
                          Sabemos que, para variáveis aleatórias contínuas, a pro-
                          babilidade exata de um valor específico é zero. O que po-
                          demos calcular é a probabilidade de um valor estar den-




.........................................................................................................

## Página 69

                 Principais distribuições contínuas                                             69



                 tro de uma faixa de intervalo e, para isso, precisamos da
                 f. d. a. da normal, o que envolve cálculos de integrais bem
                 complicados. Com o auxílio de ferramentas computacio-
                 nais, esse cálculo é trivial; caso contrário, a solução en-
                 contrada é tabelar todos os valores de probabilidades que
                 podem ser obtidos calculando a f. d. a. da distribuição
                 normal para um determinado intervalo. Mas não pode-
                 mos fazer uma tabela para cada combinação possível dos
                 parâmetros μ e σ. Então, padronizamos a distribuição por
                 meio do uso de uma nova variável, chamada de z-score.


                                                           (x – μ)
                                                      z=
                                                             σ


                 Com isso, passamos a ter uma distribuição normal com
                 média μ = 0 e variância σ2 = 1, ou seja, Z ~ N(0,1).


                 Vamos considerar o seguinte exemplo para ilustrarmos o
                 uso da distribuição normal.


                 Exemplo 8: o tempo necessário para entregar um traba-
                 lho de Estatística é de X semanas. Estudos mostram que
                 o tempo médio de entrega é de 1,7 semanas e que o des-
                 vio-padrão é de 0,1. Sabe-se que X é uma variável normal-
                 mente distribuída, ou seja, X ~ (1,7; 0,12).


                 Encontrando probabilidades
                 Para o contexto descrito no exemplo 8, se escolhermos um
                 aluno ao acaso, qual a probabilidade de o tempo de entrega
                 do seu trabalho ser maior do que 1,8 semanas?




..........................................................................................................

## Página 70

      70                  Distribuições de probabilidade




                          Nesse caso, temos um valor e queremos descobrir a pro-
                          babilidade associada a ele. O primeiro passo é transfor-
                          mar o valor de x = 1,8 em um z-score. Em seguida, consul-
                          tando uma tabela, determinarmos a probabilidade.



                                                    (                       )
                                                                1,8 – 1,7
                              P(X > 1,8) = P               Z>                   = P(Z > 1) ≈ 0,159
                                                                  0,1


                          Encontrando valores
                          Considerando o exemplo 8, qual o intervalo de tempo no
                          qual o professor de Estatística espera receber 90% dos
                          trabalhos?


                                             Figura 19 – Distribuição normal inversa.




                          Agora, sabemos a probabilidade e queremos descobrir o
                          valor associado a ela. Trata-se da distribuição normal in-
                          versa. Como mostra a figura 19, a distribuição é bicaudal,
                          e, com isso, os 10% restantes (correspondentes aos traba-
                          lhos que não se espera receber no intervalo a ser encon-
                          trado) são divididos igualmente entre as caudas, já que
                          alguns podem chegar antes, e outros, depois.




.........................................................................................................

## Página 71

                 Principais distribuições contínuas                                             71



                              P(Z < za) = P(X < xa) = 0,05 → za = –1,645
                               P(Z < zb) = P(X < xb) = 0,95 → zb = 1,645


                 Portanto,


                                 xa = μ + zaσ = 1,7 – 1,645 × 0,1 = 1,54
                                xb = μ + zbσ = 1,7 + 1,645 × 0,1 = 1,86


                 Sendo assim, no intervalo entre 1,54 e 1,86 semanas, o
                 professor de Estatística espera que 90% dos alunos entre-
                 guem o trabalho.




..........................................................................................................

## Página 72

      72                  Distribuições de probabilidade




                                         REFERÊNCIAS


                          ANDERSON, D. R.; SWEENEY, D. J.; WILLIAMS, T. A. Esta-
                          tística aplicada à administração e economia. 3. ed. São
                          Paulo: Cengage Learning, 2014.

                          BUSSAB, W. O.; MORETTIN, P. A. Estatística básica. 8. ed.
                          São Paulo: Saraiva, 2013.

                          LEVINE, D. M. et al. Estatística: teoria e aplicações: usando
                          o Microsoft Excel em português. 6. ed. Rio de Janeiro: LTC,
                          2013.

                          MCCLAVE, J. T.; BENSON, P. G.; TERRY, S. Estatística para
                          administração e economia. 10. ed. São Paulo: Pearson
                          Prentice Hall, 2009.

                          STEVENSON, W. J. Estatística aplicada à administração.
                          São Paulo: Harper & Row do Brasil, 1981.



                          Links interessantes

                          Portal Action – Distribuições discretas de probabilidade:
                          <http://www.portalaction.com.br/probabilidades/mode-
                          los-probabilisticos-discretos>.

                          Portal Action – Distribuições contínuas de probabilidade:
                          <http://www. portalaction.com.br/probabilidades/mode-
                          los-probabilisticos-continuos>.




.........................................................................................................

## Página 73

                 Noções de inferência estatística e amostragem                                  73




                 CAPÍTULO 3
                 AMOSTRAGEM



                 Até agora, vimos diversas análises envolvendo os dados
                 amostrais (estatística descritiva). No entanto, o nosso maior
                 interesse está em obtermos características da população. O
                 objetivo da inferência estatística consiste em estimar os pa-
                 râmetros populacionais desconhecidos com base em dados
                 provenientes de uma amostra. Portanto, é de fundamen-
                 tal importância que a escolha do processo de amostragem,
                 isto é, o processo de obtenção de amostras da população
                 estudada, reflita a representatividade da população, não
                 enviesando os resultados das inferências.




..........................................................................................................

## Página 74

      74                  Amostragem




                          NOÇÕES DE INFERÊNCIA ESTATÍSTICA E
                          AMOSTRAGEM




                          Uma boa forma para garantirmos que toda a variabilidade
                          presente na população esteja refletida na amostra con-
                          siste na obtenção desta de modo aleatório, de forma que
                          todos os elementos da população tenham a mesma pro-
                          babilidade de serem escolhidos. A figura 20 apresenta a
                          dinâmica do processo da inferência estatística aplicado a
                          uma determinada população. Existem parâmetros popula-
                          cionais desconhecidos que desejamos estudar e, para isso,
                          por meio de um processo de amostragem, extraímos uma
                          amostra desse conjunto para o qual calcularemos esta-
                          tísticas, que funcionarão como base da estimação desses
                          parâmetros, sendo média, variância e proporções os que
                          mais frequentemente desejamos estimar.


                                       Figura 20 – Esquema da inferência estatística.




                          Normalmente, representamos as estatísticas por letras ro-
                          manas (x̅ , s2), e os parâmetros, por letras gregas (μ, σ2).




.........................................................................................................

## Página 75

                 Noções de inferência estatística e amostragem                                  75



                 Vamos estudar, neste capítulo, os dois métodos de infe-
                 rência estatística:


                         • Intervalo de confiança: podemos estimar os pa-
                         râmetros populacionais por meio de uma estimativa
                         intervalar.

                         • Teste de hipótese: podemos testar hipóteses so-
                         bre os parâmetros populacionais.


                 Distribuições amostrais
                 Qual a relação entre a estatística x̅ e o parâmetro μ a ser
                 estimado? Se desejamos estimar um parâmetro θ da po-
                 pulação estudada, extraímos uma amostra de tamanho
                 n da população, para a qual calculamos uma estatística
                 que funcionará como um estimador θ̂ para o parâmetro
                 θ. Como vimos, uma estatística consiste em uma função
                 qualquer da amostra. Se extrairmos várias amostras di-
                 ferentes de uma mesma população, para cada amostra,
                 teremos estatísticas diferentes. Por causa da variabilida-
                 de dos valores que uma estatística pode assumir, dize-
                 mos que elas são variáveis aleatórias e, por isso, possuem
                 distribuições de probabilidades associadas a elas. Sendo
                 assim, o estimador θ̂ para o parâmetro θ é uma variável
                 aleatória e possui uma distribuição de probabilidade
                 amostral. Podemos falar, por exemplo, em distribuição
                 amostral da média da amostra.




..........................................................................................................

## Página 76

      76                  Amostragem




                                           Figura 21 – Distribuição amostral.




                          Como encontrar a distribuição amostral de um estimador
                          θ̂ ? Como mostra a figura 21, extraímos várias amostras de
                          tamanho n e, para cada uma delas, calculamos o valor θ̂ i
                          da estatística θ̂ de interesse. Os valores θ̂ i formam a dis-
                          tribuição amostral do estimador θ̂ , que possui uma média
                          μx̅ e desvio-padrão σx̅ . Qual é a distribuição amostral desse
                          estimador θ̂ ?


                          O teorema do limite central
                          Quando estudamos os tipos de distribuição, argumenta-
                          mos sobre o porquê de a distribuição normal ser tão im-
                          portante no mundo da estatística. O teorema do limite
                          central diz que, independentemente da distribuição po-
                          pulacional, desde que o tamanho da amostra seja suficien-
                          temente grande (n > 30), a distribuição amostral tem uma
                          distribuição aproximadamente normal. Quanto maior o
                          tamanho da amostra, melhor é essa aproximação. Caso a
                          distribuição original já seja uma distribuição normal, in-
                          dependentemente do tamanho n da amostra, a distribui-
                          ção amostral também se aproxima da distribuição normal.




.........................................................................................................

## Página 77

                 Noções de inferência estatística e amostragem                                  77



                 Portanto, não precisamos conhecer todas as amostras
                 possíveis da população de interesse para determinar a
                 forma da distribuição amostral. O teorema nos diz que,
                 para encontrarmos a forma da distribuição de média, é
                 suficiente conhecermos apenas dois parâmetros popula-
                 cionais: média μ e desvio-padrão σ.


                 Para a distribuição amostral da média, temos uma distri-
                 buição normal com média μx̅ = μ (ou seja, a média das mé-
                 dias amostrais é igual à média populacional) e desvio- pa-
                 drão σx̅ = σ , em que n é o total de indivíduos da amostra,
                           √n
                 e σ é o desvio-padrão da população. Ou seja:



                                        μx̅ = μ é a média amostral.

                                σx̅ = σ é o desvio-padrão amostral.
                                      √n
                                            x̅ ~ N(μ; σ )
                                                       2

                                                      √n

                 A partir de distribuições amostrais, podemos fazer infe-
                 rências sobre os parâmetros populacionais. Por exemplo,
                 se desejamos saber a média populacional μ, um bom esti-
                 mador é a média amostral. As distribuições amostrais são
                 a base para intervalos de confiança e testes de hipóteses.


                 É de extrema importância estarmos atentos à distinção entre
                 distribuição dos dados (uma amostra), distribuição amos-
                 tral (um grupo de amostras) e distribuição da população.




..........................................................................................................

## Página 78

      78                  Amostragem




                          INTERVALO DE CONFIANÇA




                          Qual o valor da média populacional? Um dos principais
                          objetivos da inferência estatística consiste na estimação
                          de alguma característica populacional (parâmetro) des-
                          conhecida por meio de dados amostrais. Existem dois ti-
                          pos de estimação dos parâmetros. A primeira é chamada
                          de estimativa pontual, que é quando temos uma única e
                          melhor estimativa para o parâmetro populacional; e a se-
                          gunda é a estimativa intervalar, que é quando temos um
                          intervalo composto por vários números, dentre os quais
                          acredita-se que esteja o valor do parâmetro populacional.


                          Para qualquer parâmetro, existem várias estimativas pon-
                          tuais possíveis. Então, o que faz com que uma determi-
                          nada estimativa seja considerada melhor do que a outra?
                          Para um estimador ser considerado “bom”, ele deve apre-
                          sentar as seguintes propriedades:


                                 • Não enviesado: deseja-se que o estimador não
                                 seja tendencioso, ou seja, que, em média, não erre
                                 para mais ou para menos. Portanto, o estimador
                                 deve possuir uma distribuição centrada no parâme-
                                 tro, ou seja, o valor esperado do estimador deve ser
                                 o próprio parâmetro.

                                 • Consistente: o estimador torna-se mais preciso à
                                 medida que o tamanho da amostra aumenta.

                                 • Eficiente: desde que as propriedades anteriores
                                 sejam satisfeitas, o melhor estimador é aquele que
                                 possui a menor variância.



.........................................................................................................

## Página 79

                 Intervalo de confiança                                                         79



                 Uma estimativa pontual dificilmente acerta o valor exato
                 do parâmetro em que estamos interessados. Além disso,
                 não é capaz de nos dizer se a estimativa encontrada se
                 aproxima do valor do parâmetro. A estimativa intervalar é
                 bem mais útil, pois incorpora uma margem de erro, o que
                 nos ajuda a medir a precisão da estimativa pontual.


                 A estimação intervalar estabelece os limites inferior e su-
                 perior, pontos críticos que constituem o que chamamos
                 de intervalo de confiança – IC [c1, c2]. O IC é uma forma
                 de mensurar a precisão da estimativa. Chamamos de ní-
                 vel de significância uma probabilidade α, e de nível de
                 confiança a probabilidade (1 – α). O nível de confiança cor-
                 responde à probabilidade de que o intervalo de confiança
                 contenha o valor do parâmetro de interesse. Em geral, o
                 nível de confiança assume os valores de 0,90, 0,95 e 0,99
                 (ou seja, usamos os valores de 0,10, 0,05 e 0,01 para o ní-
                 vel de significância α). A figura 22 ilustra um intervalo de
                 confiança genérico para um parâmetro θ.


                                          P(c1 < θ < c2) = 1 – α


                 Os valores críticos c1 e c2, que constituem os limites do IC,
                 são obtidos por meio da distribuição amostral do estima-
                 dor θ̂ .




..........................................................................................................

## Página 80

      80                  Amostragem




                                       Figura 22 – Intervalo de confiança genérico.




                          A seguir, vamos ver como calcular intervalos de confiança
                          para a média populacional considerando dois casos: o pri-
                          meiro é quando conhecemos, e o segundo é quando não
                          conhecemos o desvio-padrão populacional.


                          Estimativa intervalar da média populacional
                          com σ conhecido e amostras grandes
                          Exemplo 9: deseja-se estimar o CR médio dos alunos de
                          Administração de uma determinada faculdade. Para isso,
                          recolheu-se uma amostra de 50 alunos de Administração,
                          e verificou-se que o CR médio nessa amostra foi de 7 — ou
                          seja, a média amostral encontrada foi x̅ = 7. Dados antigos
                          indicam que o desvio-padrão é σ = 1,5, e acredita-se que
                          esse valor não tenha sido alterado. Qual o intervalo de
                          confiança para o CR médio populacional, com 95% de con-
                          fiança (ou seja, com α = 0,05)?


                          Para construir esse IC, devemos utilizar a distribuição
                          amostral da média. Pelo teorema do limite central, sabe-




.........................................................................................................

## Página 81

                 Intervalo de confiança                                                         81



                 mos que, pelo fato de a amostra ser suficientemente gran-
                 de (n = 50), a distribuição amostral da média é normal-
                 mente distribuída. Vamos utilizar a distribuição normal
                 padrão para os próximos cálculos.


                 O primeiro passo é determinar os pontos críticos c1 e c2
                 utilizando o z-score. Como queremos que a probabilidade
                 de o valor da média estar compreendido dentro do inter-
                 valo de confiança calculado seja de 0,95 (1 – α), restam
                 0,025 (α/2) de cada um dos lados de fora do intervalo,
                 correspondente à probabilidade de que o valor da média
                 não esteja dentro do intervalo (lembrando que, pelo o fato
                 da distribuição ser bicaudal, dividimos os 0,05 restantes
                 para as duas caudas). A figura 23 ilustra esse raciocínio.


                 Usando a tabela da distribuição normal padrão, consegui-
                 mos identificar que o z-score correspondente à área de
                 0,025 é de 1,96.


                        Figura 23 – Construção do intervalo de confiança para a média.




..........................................................................................................

## Página 82

      82                  Amostragem




                          Então, temos 95% de chance de a média amostral x̅ assu-
                          mir um valor a uma distância de 1,96 desvios-padrão da
                          média populacional μ, e essa distância é o que chamamos
                          de margem de erro (E).


                                              z = x̅ ± μ/σx̅ → μ = x̅ ± zσx̅


                                                      E = zα/2 σx̅ = zα/2 σ
                                                                          √n

                          A margem de erro nos diz com que precisão conseguimos
                          estimar a média populacional utilizando a média amos-
                          tral. Então, temos que:



                                        (                                    )
                                       P x̅ – zα/2 σ < μ < x̅ + zα/2 σ = 1 – α
                                                  √n                √n

                          Ou seja, o intervalo de confiança encontrado para a média
                          populacional μ com nível de confiança igual a (1 – α) é re-
                          presentado por:


                                        IC(μ;1 – α) = (x̅ – zα/2 σx̅ ; x ̅+ zα/2 σx̅ )


                          Vale ressaltar que, quanto maior o tamanho n da amostra,
                          menor é a margem de erro e, portanto, maior a precisão.


                          Voltando ao nosso exemplo, sabemos que x̅ = 7, σ = 1,5 e
                          n = 50. Portanto,



                                                       σ   1,5
                                              σx̅ =      =     ≈ 0,212
                                                      √n   √50

                                                E = 1,96 × σx̅ = 0,416

                               IC(μ;0,95) = (7 – 0,416;7 + 0,416) = (6,584;7,416)




.........................................................................................................

## Página 83

                 Intervalo de confiança                                                         83



                 Então, podemos dizer que, com 95% de confiança, a média
                 populacional μ está entre 6,584 e 7,416. Isso não significa
                 que a probabilidade de o parâmetro μ cair nesse intervalo
                 seja de 95%, mas que, se extrairmos diversas amostras in-
                 dependentes de mesmo tamanho dessa população, espe-
                 ra-se que, em 95% delas, o verdadeiro valor do parâmetro
                 estimado (nesse caso, a média populacional) esteja dentro
                 desse intervalo.


                 Estimativa intervalar da média populacional
                 com σ desconhecido e amostras pequenas
                 O caso retratado no exemplo 9, em que temos uma amos-
                 tra suficientemente grande e conhecemos a variância po-
                 pulacional, é bem distante da realidade. Vamos retornar
                 ao exemplo anterior e considerar algumas mudanças.


                 Exemplo 10: deseja-se estimar o CR médio dos alunos de
                 Administração de uma determinada faculdade. Para isso,
                 recolheu-se uma amostra de 50 alunos de Administração,
                 e verificou-se que o CR médio nessa amostra foi de 7, com
                 desvio-padrão de 1,8 — ou seja, a média amostral encon-
                 trada foi x̅ = 7, e o desvio-padrão amostral foi s = 1,8. Qual
                 o intervalo de confiança para o CR médio populacional
                 com 95% de confiança?


                 O que aconteceria se a amostra fosse pequena? Pelo teore-
                 ma do limite central, só podemos assumir que a distribui-
                 ção amostral é aproximadamente normal se a amostra for
                 suficientemente grande (ou seja, n > 30) ou se a população
                 tiver distribuição aproximadamente normal. Se esse fosse o
                 caso agora, poderíamos proceder da mesma maneira como
                 no caso anterior, ou seja, calculando o IC a partir do z-score.




..........................................................................................................

## Página 84

      84                  Amostragem




                          Mas e se, no caso de uma amostra pequena (n > 30), a
                          população não for normal? E se não conhecermos o valor
                          de σ? A solução é estimarmos esse parâmetro por meio
                          do desvio-padrão da amostra. Como estamos adicionando
                          uma nova fonte de incerteza, passamos a ter uma nova
                          distribuição, a distribuição t de Student.




                          A distribuição t de Student se assemelha bastante à distri-
                          buição normal padrão: possui forma de sino e é simétrica
                          em torno da média zero. Além disso, assim como existem
                          os z-scores para a distribuição normal, existem, de forma
                          similar, os t-scores para a distribuição t. A forma que essa
                          distribuição assume depende de um único parâmetro, que
                          chamamos de graus de liberdade (gl) e é igual ao tamanho
                          da amostra (n) menos um (gl = n – 1). Assim, de maneira
                          semelhante ao caso anterior, quando não conhecemos o
                          desvio-padrão populacional, calculamos o t-score com a
                          fórmula:


                                                     t = x̅ ± μ → μ = x̅ ± t s
                                                                            √n
                                                            s
                                                           √n


                                                             E = t (n – 1);(1 – α ) s
                                                                                2  √n


                          A margem de erro nos diz com que precisão conseguimos
                          estimar a média populacional utilizando a média amos-
                          tral. Então, temos que:



                             P
                                 (   x̅ – t (n – 1);(1 – α )
                                                         2
                                                                s
                                                               √n
                                                                  < μ < x̅ + t (n – 1);(1 – α )
                                                                                            2
                                                                                                 s
                                                                                                √n   )   = 1 –α




.........................................................................................................

## Página 85

                 Intervalo de confiança                                                                       85



                 Ou seja, o intervalo de confiança encontrado para a média
                 populacional μ com nível de confiança igual a (1 – α) é re-
                 presentado por:



                     IC ( μ; 1 – α) =
                                          (   x̅ – t (n – 1);(1 – α )
                                                                  2
                                                                         s
                                                                        √n
                                                                           ; x̅ + t (n – 1);(1 – α )
                                                                                                 2
                                                                                                      s
                                                                                                     √n   )
                 Voltando ao exemplo 10, sabemos que x̅ = 7, s = 1,8 e n = 50.
                 Como o valor de σ é desconhecido, devemos usar a distribui-
                 ção t de Student. Consultando uma tabela da distribuição t
                 de Student, teremos:


                                  E = t 49;0,975 s = 2,01 1,8 = 0,512
                                                 √n       √50

                         IC (μ;0,95) = (7 – 0,512; 7 + 0,512) = (6,488;7,512)


                 Então, podemos dizer que, com 95% de confiança, a média
                 populacional μ está entre 6,488 e 7,512.




..........................................................................................................

## Página 86

      86                  Amostragem




                          TESTE DE HIPÓTESES




                          Na seção anterior, estávamos interessados em, a partir de
                          dados obtidos de uma amostra, construir um intervalo de
                          confiança, dentro do qual poderíamos, com um certo nível
                          de confiança estabelecido, estimar o valor de um parâme-
                          tro populacional. Na prática, no entanto, existem diversos
                          problemas nos quais precisamos decidir entre aceitar ou re-
                          jeitar uma afirmação a respeito de um parâmetro populacio-
                          nal. A afirmativa é chamada de hipótese, e o procedimento
                          para analisar a probabilidade de que ela seja verdadeira ou
                          falsa — e, assim, aceitá-la ou refutá-la — é chamado de teste
                          de hipóteses. Há uma grande semelhança entre o teste de
                          hipóteses e a construção de intervalos de confiança.


                          Chamamos de hipótese nula a afirmação que assumimos
                          como verdadeira para a construção do teste. Ela vai ser
                          testada frente a evidências, para decidirmos pela sua re-
                          jeição ou não. A hipótese alternativa é o que aceitaremos,
                          caso a hipótese nula seja rejeitada a partir das evidências.
                          Podemos traçar um paralelo com um julgamento, no qual
                          um acusado (réu) é inocente (ou seja, a hipótese nula é que
                          ele é inocente) até que se tenham provas suficientes que
                          nos façam refutar essa inocência, rejeitando a hipótese
                          nula (ou seja, a hipótese alternativa é que ele é culpado).
                          Essa situação pode ser expressa na notação do teste de
                          hipóteses da seguinte maneira:




                                          {   H0: o réu é inocente.
                                               Ha: o réu é culpado.



.........................................................................................................

## Página 87

                 Teste de hipóteses                                                             87



                 O que se espera, em um teste de hipóteses, é rejeitar H0
                 quando ela é falsa, e não rejeitá-la quando for verdadeira.


                 Há chance de tomarmos uma decisão errada? Como deci-
                 dir se a hipótese nula deve ou não ser rejeitada?


                 São duas as possibilidades de erro em um teste de hipó-
                 teses. Primeiramente, podemos rejeitar H0 quando ela é
                 verdadeira. Esse é chamado de erro tipo I. Outra possibili-
                 dade é não rejeitarmos H0 quando ela é falsa. Nesse caso,
                 estaríamos diante de um erro tipo II.


                 A tabela 9 resume as possibilidades de decisões em um
                 teste de hipóteses.


                                 Tabela 9 – Decisões em um teste de hipóteses.



                                                                Decisão tomada

                                                        Não rejeitar H0     Rejeitar H0

                                      H0 é verdadeira   Decisão correta     Erro tipo I
                    Realidade
                                         H0 é falsa       Erro tipo II    Decisão correta


                 A fim de ilustrar o que se apresenta a seguir, considere-
                 mos o seguinte exemplo.


                 Exemplo 11: historicamente, o CR médio dos alunos de
                 Administração de uma determinada faculdade tem sido
                 considerado como sendo igual a 7. A coordenação imagina
                 que está selecionando alunos cada vez melhores e imagina
                 que o CR tenha aumentado. Então, recolheu-se uma amos-
                 tra de 50 alunos de Administração, e verificou-se que o CR




..........................................................................................................

## Página 88

      88                  Amostragem




                          médio nessa amostra foi de 7,5 (ou seja, a média amostral
                          encontrada foi x̅ = 7,5), com desvio-padrão de 1,9 (ou seja,
                          o desvio-padrão amostral encontrado foi s = 1,9). Dados
                          antigos indicam que o desvio-padrão populacional é σ =
                          1,5, e acredita-se que esse valor não tenha sido alterado.
                          Há evidências estatísticas de que o CR médio aumentou?


                          Vamos, então, entender o procedimento para a realização
                          de um teste de hipóteses para a média populacional.


                          Teste de hipóteses para a média populacional μ
                          Basicamente, o que devemos fazer é verificar se, dado que
                          sabemos o valor da média populacional μ, a ocorrência
                          de uma amostra que apresente uma média com o valor x̅
                          é um fato provável ou não. Portanto, devemos calcular a
                          probabilidade de ocorrência de um valor igual a x̅ em uma
                          distribuição amostral baseada em μ.


                          Devemos lembrar que, se a amostra é suficientemente
                          grande e conhecemos o desvio-padrão populacional, o te-
                          orema do limite central nos garante que x̅ tem distribuição
                          normal, com média μ e desvio-padrão σ x̅ = σ . Ou seja3:
                                                                      √n


                                                               ( μ; σn )
                                                                    2
                                                      x̅ ~ N


                          Porém, se a amostra é pequena ou não conhecemos o des-
                          vio-padrão populacional, usamos a distribuição t de Stu-
                          dent, com mesma média e mesmo número de graus de



                          3 Existem testes de hipóteses para outros parâmetros, cujos estimadores
                          apresentam distribuições de probabilidade diferentes da normal. Esses
                          testes não são tratados neste livro.




.........................................................................................................

## Página 89

                 Teste de hipóteses                                                             89



                 liberdade igual à quantidade de observações menos 1 (gl
                 = n – 1).
                                         x̅ ~ t(μ;n – 1)


                 Então, temos duas alternativas para avaliar a ocorrência
                 de uma média amostral com valor x̅ dentro dessa distri-
                 buição. Em ambas, devemos definir, a priori, um valor
                 aceitável para a probabilidade de ocorrência de erro do
                 tipo I. Esse valor é denominado nível de significância do
                 teste de hipóteses e, normalmente, é simbolizado por α.
                 Quanto menor o nível de significância, mais significante é
                 o resultado da amostra escolhida para a tomada de deci-
                 são de rejeitar H0. Em geral, fixamos o nível de significân-
                 cia α em 0,05, 0,01 ou 0,001.


                 A primeira opção é calcularmos, a partir de α, uma região
                 crítica de valores e analisarmos onde o valor x̅ se encon-
                 tra. Se x̅ estiver dentro dessa região crítica, rejeitamos H0;
                 caso contrário, não temos evidências para rejeitá-la. Al-
                 ternativamente, podemos calcular a probabilidade asso-
                 ciada ao valor x̅ . Essa probabilidade, denominada p-valor,
                 é comparada ao α previamente estabelecido. Se o p-valor
                 for inferior a α, significa que o valor x̅ é menos provável de
                 ocorrer do que o nível de significância definido para o tes-
                 te, o que permite rejeitar H0. Caso contrário, teremos que
                 o valor x̅ é mais provável de ocorrer que o nível de signifi-
                 cância do teste, o que significa que não podemos rejeitar
                 H0, ou, em outras palavras, não existe evidência estatística
                 de que, com a média populacional reportada, seja muito
                 improvável a ocorrência de uma média amostral no valor
                 encontrado.




..........................................................................................................

## Página 90

      90                  Amostragem




                          Vamos ilustrar as etapas desses dois procedimentos ana-
                          lisando o exemplo 11.


                          Etapa 1: formulação das hipóteses
                          Primeiramente, vamos traduzir o enunciado para a for-
                          mulação de teste de hipóteses. Iniciamos identificando
                          as hipóteses a serem formuladas. O dado correspondente
                          ao parâmetro populacional é de que μ = 7. Os dados da
                          amostra nos trouxeram uma evidência de que x̅ = 7,5. Que-
                          remos, a partir dessa evidência, verificar se μ > 7. Então,
                          podemos formular as seguintes hipóteses:




                                                 {   H 0: μ = 7
                                                     H a: μ > 7


                          A afirmação H0: μ = 7 é a hipótese nula, porque é o dado
                          que temos e que queremos confrontar com uma evidên-
                          cia. A afirmação Ha: μ > 7 é a hipótese alternativa, pois as
                          evidências obtidas nos farão assumir que essa é a média,
                          caso a hipótese nula seja refutada.


                          Pela maneira como o problema foi colocado, queremos ve-
                          rificar apenas se há evidências estatísticas de que a média
                          populacional tenha aumentado e, nessa situação, dizemos
                          que a hipótese alternativa é unilateral. Seria unilateral
                          também caso estivéssemos interessados em verificar se
                          a média diminuiu, e a hipótese alternativa seria escrita
                          como Ha: μ < 7. Porém, em algumas situações, testaremos
                          se a média populacional é diferente da reportada. Se esse
                          fosse o caso no exemplo em questão, a hipótese alternati-
                          va seria Ha: μ ≠ 7, e diríamos que ela é bilateral.




.........................................................................................................

## Página 91

                 Teste de hipóteses                                                             91




                 Etapa 2: definir o estimador e a sua distribuição
                 Tendo definido as hipóteses do teste, passamos, então, à
                 etapa seguinte, que é definir qual estatística será usada
                 para testar a hipótese nula. Em outras palavras, devemos
                 definir o estimador e a sua distribuição. Como estamos
                 tratando de teste de hipóteses para a média populacional,
                 o estimador utilizado será a média amostral, que, como
                 explicado, tem distribuição normal. Os dados do exemplo
                 nos dizem que:


                                                      μ=7
                                                      σ = 1,5
                                                      n = 50


                 Logo, podemos calcular o desvio-padrão amostral e a dis-
                 tribuição de x̅ :


                                           σ x̅ = σ = 1,5 = 0,2121
                                                  √n  √50



                                                50 )
                                           ( 7; 1,5
                                                  2
                                  x̅ ~ N             ou x̅ ~ N (7;0,045)


                 Se o desvio-padrão não fosse conhecido, usaríamos a dis-
                 tribuição t de Student:


                                                 x̅ ~ t(7;49)


                 Etapa 3: definir o nível de significância
                 O nível de significância é a probabilidade aceitável de
                 ocorrência de erro tipo I que queremos assumir no teste.
                 Vamos, nesse caso, fixar α = 0,05.




..........................................................................................................

## Página 92

      92                  Amostragem




                          Etapa 4: calcular a estatística de teste
                          A estatística de teste é o z-score que mede a distância en-
                          tre x̅ e μ especificados na hipótese nula:


                                                      x̅ – μ          x̅ – μ
                                             z0 =               =
                                                       σ x̅             σ
                                                                       √n


                          Se não soubéssemos o desvio-padrão populacional, con-
                          sideraríamos o desvio-padrão amostral s como estimador
                          para ele e usaríamos o t-score:


                                                      x̅ – μ          x̅ – μ
                                             t0 =               =       s
                                                       σ x̅
                                                                       √n


                          Substituindo os valores do exemplo 11, como conhecemos
                          o desvio-padrão populacional, calculamos o z-score:


                                                        7,5 – 7
                                               z0 =        15       = 2,357
                                                          √50


                          Se não soubéssemos o valor do desvio-padrão populacio-
                          nal, usaríamos o desvio-padrão amostral como estimador
                          para calcular o t-score:


                                                        7,5 – 7
                                               t0 =       1,9       = 1,861
                                                          √50


                          Etapa 5 (a): calcular a região de rejeição
                          Tendo definido o nível de significância do teste de hipóte-
                          ses, devemos, agora, encontrar os valores críticos que defi-
                          nem os limites da região crítica. Tal como fizemos quando
                          calculamos intervalos de confiança para a média popula-


.........................................................................................................

## Página 93

                 Teste de hipóteses                                                             93



                 cional, como o desvio-padrão populacional é conhecido,
                 usaremos o z-score. Se não conhecêssemos o desvio-pa-
                 drão populacional, ou se a amostra fosse pequena (n < 30)
                 e não tivéssemos certeza da distribuição da população,
                 usaríamos a distribuição t de Student e o t-score.


                 Quando o teste de hipóteses é unilateral para aumento da
                 média (vamos nos referenciar a esse tipo de teste como
                 unilateral com Ha: “>”), consideramos que a região de re-
                 jeição está toda na cauda direita da distribuição. Então,
                 devemos encontrar o ponto zcrit, que delimita uma área à
                 sua direita igual a α, como mostrado na figura 24. Como
                 vemos, a área da figura nos indica que P(z > zcrit) = α.

                       Figura 24 – Região de rejeição para teste unilateral com Ha: “>”.




                 Se o teste fosse unilateral, mas quiséssemos verificar a dimi-
                 nuição da média (vamos nos referenciar a esse tipo de teste
                 como unilateral com Ha: “<”), o zcrit seria o ponto que deli-
                 mita uma área à sua esquerda igual a α, como na figura 25.
                 Como vemos, a área da figura nos indica que P(z < zcrit) = α.




..........................................................................................................

## Página 94

      94                  Amostragem




                               Figura 25 – Região de rejeição para teste unilateral com Ha: “<”.




                          No entanto, se o teste fosse bilateral, dividiríamos o α por
                          dois e encontraríamos dois valores críticos. O primeiro
                          +zcrit, delimitando uma área de α/2 à sua direita, e o se-
                          gundo –zcrit, simétrico ao primeiro, delimitado uma área de
                          α/2 à sua esquerda, como mostrado na figura 26. Ou seja,
                          P(z < –zcrit ou z > zcrit = α.


                                       Figura 26 – Região de rejeição para teste bilateral.




.........................................................................................................

## Página 95

                 Teste de hipóteses                                                             95



                 Para o caso do exemplo 11, consultando uma tabela nor-
                 mal padrão, vemos que o z-score correspondente ao teste
                 unilateral com Ha: “>” e α = 0,05 é zcrit = 1,64.


                 Com isso, definimos a região de rejeição da seguinte for-
                 ma: valores z superiores a 1,64 são maiores que o zcrit cal-
                 culado para o nível de significância estabelecido e são,
                 portanto, considerados improváveis. Por outro lado, va-
                 lores z inferiores a 1,64 são menores que o zcrit e são con-
                 siderados prováveis, frente ao nível de significância esta-
                 belecido.


                 Vale notar que, se o teste fosse unilateral com Ha: “<”, te-
                 ríamos zcrit = –1,64. E, se o teste fosse bilateral, teríamos
                 zcrit = ± 1,96.


                 De forma semelhante, caso o desvio-padrão da popula-
                 ção não fosse conhecido, teríamos, respectivamente: tcrit =
                 1,68, tcrit = –1,68, e tcrit = ±2,01 (considerando gl = 49 graus
                 de liberdade).


                 Etapa 6 (a): concluir sobre a rejeição de H0 usando
                 a região de rejeição
                 Uma vez definida a região de rejeição e calculada a estatís-
                 tica de teste, temos condições de decidir se podemos, ou
                 não, rejeitar a hipótese nula. Para isso, devemos observar
                 se a estatística de teste se encontra dentro da região de
                 rejeição ou não.


                 Em um teste unilateral com Ha: “>”, teremos a seguinte
                 regra de decisão:




..........................................................................................................

## Página 96

      96                  Amostragem




                                {      z0 > zcrit ⇒ Rejeitamos Ho
                                       z0 < zcrit ⇒ Não podemos rejeitar Ho


                          Ou:




                                {      t0 > tcrit ⇒ Rejeitamos Ho
                                       t0 < tcrit ⇒ Não podemos rejeitar Ho


                          Em um teste unilateral com Ha: “<”, teremos a seguinte
                          regra de decisão:




                                {      z0 < zcrit ⇒ Rejeitamos Ho
                                       z0 > zcrit ⇒ Não podemos rejeitar Ho


                          Ou:




                                {      t0 < tcrit ⇒ Rejeitamos Ho
                                       t0 > tcrit ⇒ Não podemos rejeitar Ho


                          Em um teste bilateral, teremos a seguinte regra de decisão:




                                {      z0 < zcrit ou z0 > zcrit ⇒ Rejeitamos Ho
                                       –zcrit< z0 < + zcrit ⇒ Não podemos rejeitar Ho


                          Ou:




                                {      t0 < tcrit ou t0 > tcrit ⇒ Rejeitamos Ho
                                       –tcrit < t0 < tcrit ⇒ Não podemos rejeitar Ho




.........................................................................................................

## Página 97

                 Teste de hipóteses                                                             97



                 Podemos, portanto, aplicar a regra apropriada ao exemplo
                 11. Lembrando que se trata de um teste unilateral com Ha:
                 “<”, e que zcrit = 1,64 e z0 = 2,357. Como z0 > zcrit, rejeita-
                 mos H0. Ou seja, pelos dados da amostra, temos evidên-
                 cias estatísticas suficientes para acreditar que a média de
                 CR realmente aumentou.


                 Se tivéssemos que fazer o teste com o t-score, teríamos tcrit
                 = 1,68 e t0 = 1,861; como t0 > tcrit, também rejeitaríamos H0.


                 Alternativamente, as etapas 5 (a) e 6 (a) podem ser subs-
                 tituídas por etapas equivalentes, nas quais, no lugar de
                 encontrarmos a região de rejeição e tomarmos uma de-
                 cisão com base na análise da posição da média amostral
                 em relação a essa região, encontramos a probabilidade
                 associada à ocorrência da média amostral observada e a
                 comparamos com o nível de significância definido para o
                 teste. Essas etapas são descritas a seguir.


                 Etapa 5 (b): calcular o p-valor da estatística de
                 teste
                 Uma vez calculada a estatística de teste, que representa o
                 z-score (ou t-score) correspondente à média amostral ob-
                 servada, devemos calcular qual a probabilidade a ela asso-
                 ciada, dada a distribuição da média populacional.


                 Para isso, devemos procurar na tabela da distribuição nor-
                 mal (ou da distribuição t de Student) qual o valor da pro-
                 babilidade correspondente ao valor da estatística de teste
                 z0. Se o teste é unilateral com Ha: “>”, o p-valor é a área
                 abaixo da curva à direita de z0. Por outro lado, se o teste é
                 unilateral com Ha: “<”, o p-valor é a área abaixo da curva à




..........................................................................................................

## Página 98

      98                  Amostragem




                          esquerda de z0. A figura 27 nos mostra essas situações. Já
                          se o teste é bilateral, o p-valor é o dobro da área abaixo da
                          curva além de z0, como mostra a figura 28.


                               Figura 27 – p-valor para testes unilaterais com Ha: “>” e Ha: “<”.




                                           Figura 28 – p-valor para teste bilateral.




                          Considerando os resultados obtidos no exemplo 11, para
                          z0 = 2,357, consultando a tabela da distribuição normal pa-
                          drão, temos que p-valor = 0,009. Caso usássemos o t-score,
                          considerando t0 = 1,861, consultando a tabela da distri-
                          buição t de Student, com gl = 19, encontraríamos p-valor
                          = 0,039.


                          Etapa 6 (b): concluir sobre a rejeição de H0
                          usando o p-valor
                          Devemos, agora, comparar o p-valor encontrado na etapa
                          anterior com o nível de significância estabelecido para o
                          teste, de acordo com a seguinte regra de decisão:


.........................................................................................................

## Página 99

                 Teste de hipóteses                                                             99



                                      p-valor < α ⇒ Rejeitamos Ho
                               p-valor < α ⇒ Não podemos rejeitar Ho


                 Olhando, então, para os resultados obtidos até o momento
                 para o exemplo 11, temos p-valor = 0,009 e α = 0,05. Ou
                 seja, como p-valor < α, rejeitamos H0. Se considerássemos
                 o t-score, teríamos p-valor = 0,039 e α = 0,05. Ou seja, tam-
                 bém rejeitaríamos H0.


                 Vale notar que, como sequência de etapas 5 (a) – 6 (a) é
                 equivalente à sequência 5 (b) – 6 (b); os resultados são os
                 mesmos. Se H0 é rejeitada (ou não rejeitada) seguindo-se
                 uma das formas de análise, também será rejeitada (ou não
                 rejeitada) se seguirmos a outra.




..........................................................................................................

## Página 100

      100                 Amostragem




                                       REFERÊNCIAS


                          ANDERSON, D. R.; SWEENEY, D. J.; WILLIAMS, T. A. Esta-
                          tística aplicada à administração e economia. 3. ed. São
                          Paulo: Cengage Learning, 2014.

                          BUSSAB, W. O.; MORETTIN, P. A. Estatística básica. 8. ed.
                          São Paulo: Saraiva, 2013.

                          LEVINE, D. M. et al. Estatística: teoria e aplicações: usando
                          o Microsoft Excel em português. 6. ed. Rio de Janeiro: LTC,
                          2013.

                          MCCLAVE, J. T.; BENSON, P. G.; TERRY, S. Estatística para
                          administração e economia. 10. ed. São Paulo: Pearson
                          Prentice Hall, 2009.

                          STEVENSON, W. J. Estatística aplicada à administração.
                          São Paulo: Harper & Row do Brasil, 1981.



                          Links interessantes

                          Portal Action – Inferência:
                          <http://www.portalaction.com.br/inferencia-0>.

                          Portal Action – Teste de hipóteses:
                          <http://www.portalaction.com.br/inferencia/testes-de-hi-
                          poteses>.

                          Portal Action – Intervalos de confiança:
                          <http://www.portalaction.com.br/inferencia/intervalo-de-
                          -confianca>.




.........................................................................................................

## Página 101

                Diagrama de dispersão                                                          101




                 CAPÍTULO 4
                 COVARIÂNCIA, CORRELAÇÃO E
                 REGRESSÃO LINEAR



                 Muitas vezes, queremos medir a relação entre duas vari-
                 áveis, ou seja, o quanto o comportamento de uma delas
                 é influenciado pela outra. Também é comum querermos
                 prever o valor de uma variável a partir dos valores de ou-
                 tra. Em alguns casos, a relação é determinística, ou seja, é
                 regida por uma regra estabelecida, que pode ser expressa
                 precisamente por uma formulação matemática. Por exem-
                 plo, se sabemos a temperatura ambiente medida em graus
                 Celsius (TC), podemos calcular a temperatura medida em
                 Fahrenheit (TF) com precisão, pois a conversão é feita por
                 uma fórmula conhecida:


                                                9
                                         Tf =     T + 32
                                                5 c


                 Assim como nesse caso, há diversas outras variáveis que
                 podem ser calculadas com precisão a partir do valor de ou-
                 tras, em diversos contextos: o valor dos impostos a serem
                 pagos dependem de muitas variáveis e pode ser calculado
                 por meio das regras definidas pela Receita Federal; a comis-
                 são a ser paga a um vendedor é função do valor das suas
                 vendas e do porcentual pactuado com seu patrão.




..........................................................................................................

## Página 102

      102                 Covariância, correlação e regressão linear




                          No entanto, existem inúmeros outros casos em que não
                          existe uma regra precisa relacionando variáveis, mas, ain-
                          da assim, precisamos conhecer a maneira como uma vari-
                          ável influencia outra, se é que existe essa influência. Por
                          exemplo, será que a nota de um aluno em uma disciplina é
                          influenciada pela quantidade de horas de estudos que ele
                          dedica a ela? Em caso positivo, será que temos como prever
                          a nota de um aluno que tenha dedicado uma certa quanti-
                          dade de horas de estudo a uma disciplina específica?


                          Em situações assim, dizemos que, se houver, essa relação
                          será estatística, e não determinística. Nesses casos, usa-
                          mos as ideias de covariância, correlação e regressão line-
                          ar, que serão desenvolvidas nos tópicos a seguir. Porém,
                          antes, veremos uma maneira de representar graficamente
                          as informações de duas variáveis que queremos analisar
                          conjuntamente. A fim de ilustrar o que se apresenta a se-
                          guir, consideremos o seguinte exemplo.


                          Exemplo 12: o professor de Estatística levantou algumas
                          informações sobre os alunos de uma turma, buscando
                          compreender as notas em uma prova. Os dados estão
                          apresentados na tabela 104.


                                                                Tabela 10

                                           Tempo de
                                                              Tempo de      Dia do
                               Id.           estudo
                                                             lazer (horas   mês que   Nota
                             aluno           (horas
                                                              semanais)     nasceu
                                           semanais)
                                1               3,0               40,4        13      4,7
                                2               5,4                8,0        29      7,8



                          4 Dados fictícios.




.........................................................................................................

## Página 103

                                                                                               103




                     3          2,9          30,8          8           4,8
                     4          2,1          42,0          12          3,0
                     5          4,1          19,2          13          5,5
                     6          5,3          14,8          6           5,8
                     7          4,4          28,4          2           6,5
                     8          5,5          18,0          23          6,3
                     9          1,6          58,8          10          2,0
                     10         6,2          20,4          15          7,8
                     11         3,1          42,8          29          3,5
                     12         4,2          39,6          15          5,1
                     13         3,4          36,4          10          5,5
                     14         7,0           7,2          7           8,2
                     15         7,5           3,2          10          9,2
                     16         3,0          36,8          5           4,5
                     17         7,4           4,4          20         10,0
                     18         0,9          48,8          31          1,0
                     19         8,1           9,6          4           9,5
                     20         2,6          48,4          30          3,0




..........................................................................................................

## Página 104

      104                 Covariância, correlação e regressão linear




                          DIAGRAMA DE DISPERSÃO




                          Um diagrama de dispersão é um gráfico bidimensional,
                          em que os valores das variáveis analisadas são plotados
                          como pares ordenados (x, y) no plano cartesiano. Por meio
                          dele, podemos fazer uma primeira análise, visual, a res-
                          peito da maneira como as variáveis de interesse estão re-
                          lacionadas. A figura 29 apresenta o diagrama de dispersão
                          para as variáveis Tempo de estudo e Nota.


                                              Figura 29 – Diagrama de dispersão para
                                             Tempo de estudo (horas semanais) x Nota.




                          Podemos perceber que há fortes indícios de que os alunos
                          que dedicaram mais tempo por semana estudando a dis-
                          ciplina obtiveram notas melhores do que aqueles que es-
                          tudaram por menos tempo. Se olharmos um gráfico seme-




.........................................................................................................

## Página 105

                Diagrama de dispersão                                                          105



                 lhante, relacionando as variáveis Tempo de lazer e Nota,
                 como apresentado na figura 30, vamos ter outra ideia.

                                  Figura 30 – Diagrama de dispersão para
                                  Tempo de lazer (horas semanais) x Nota.




                 Nesse caso, parece que a relação é diferente, ou seja, as
                 notas dos alunos que dedicaram mais tempo semanal ao
                 lazer foram menores do que daqueles que tiveram menos
                 tempo, já que o valor da variável Nota decresce conforme
                 o valor da variável Tempo de lazer aumenta.


                 Considerando as variáveis Dia de nascimento e Nota, ve-
                 mos, pela figura 31, um comportamento difuso.




..........................................................................................................

## Página 106

      106                 Covariância, correlação e regressão linear




                                 Figura 31 – Diagrama de dispersão para Dia que nasceu x Nota.




                          Não está claro se há algum relacionamento entre essas
                          variáveis. Na realidade, parece não haver — o que é in-
                          tuitivamente razoável. No entanto, essa observação não é
                          conclusiva, considerando apenas essa análise visual. Pre-
                          cisamos de confirmação estatística, que será possível com
                          os conceitos tratados a seguir.




.........................................................................................................

## Página 107

                 Medida da associação entre duas variáveis                                     107




                 MEDIDA DA ASSOCIAÇÃO ENTRE DUAS
                 VARIÁVEIS




                 A associação entre duas variáveis quantitativas é medida
                 pela covariância e pelo coeficiente de correlação. Iniciare-
                 mos falando sobre covariância.


                 Covariância
                 A covariância entre as variáveis x e y — representada por
                 Cov(x, y) — dá a ideia da dispersão dos pares ordenados
                 (x, y) em relação ao ponto (E(x), E(y)), em que E(x) e E(y) são
                 os valores esperados (em outras palavras, as médias) das
                 variáveis x e y. Ela mede o grau de associação linear entre
                 as variáveis e pode ser calculada pela fórmula a seguir, em
                 que: xi e yi são os valores das variáveis correspondentes ao
                 i-ésimo par de observações, X̅ e Y̅ são as médias das variá-
                 veis x e y, e n é a quantidade de observações consideradas.


                                                       ∑ (xi – X̅ ) . (yi – Y̅ )
                                      Cov(x, y) =
                                                                n–1


                 Observando a fórmula, podemos compreender que a co-
                 variância é dada pela média dos produtos dos desvios em
                 relação ao ponto que representa a média dos valores con-
                 siderados.


                 Considerando os dados da tabela 10, primeiramente va-
                 mos calcular a covariância entre as variáveis Tempo de
                 estudo (x) e Nota (y). Para isso, usaremos uma planilha
                 auxiliar, apresentada na tabela 11, na qual calculamos as



..........................................................................................................

## Página 108

      108                 Covariância, correlação e regressão linear




                          médias das variáveis analisadas e incluímos duas colunas
                          onde são computados os desvios de cada valor em relação
                          à média da variável correspondente, e outra com o produ-
                          to entre esses desvios.


                               Tabela 11 – Cálculo da covariância entre Tempo de estudo e Nota.

                                           Tempo
                                         de estudo
                                                                       xi – X̅   yi – Y̅
                           Id. aluno       (horas        Nota (yi)                         Δxi . Δyi
                                                                       (Δxi)     (Δyi)
                                         semanais)
                                            (xi)
                               1             3,0            4,7        -1,4       -1,0       1,4
                               2             5,4            7,8         1,0       2,1        2,1
                               3             2,9            4,8        -1,5       -0,9       1,3
                               4             2,1            3,0        -2,3       -2,7       6,1
                               5             4,1            5,5        -0,3       -0,2       0,1
                               6             5,3            5,8         0,9       0,1        0,1
                               7             4,4            6,5         0,0       0,8        0,0
                               8             5,5            6,3         1,1       0,6        0,7
                               9             1,6            2,0        -2,8       -3,7      10,3
                               10            6,2            7,8         1,8       2,1        3,8
                               11            3,1            3,5        -1,3       -2,2       2,8
                               12            4,2            5,1        -0,2       -0,6       0,1
                               13            3,4            5,5        -1,0       -0,2       0,2
                               14            7,0            8,2         2,6       2,5        6,6
                               15            7,5            9,2         3,1       3,5       10,9
                               16            3,0            4,5        -1,4       -1,2       1,6
                               17            7,4            10,0        3,0       4,3       13,0
                               18            0,9            1,0        -3,5       -4,7      16,3
                               19            8,1            9,5         3,7       3,8       14,2
                               20            2,6            3,0        -1,8       -2,7       4,8
                            Soma:           87,7           113,7                 Soma:      96,5
                            Média:           4,4            5,7
                          D. Padrão:        2,09           2,51




.........................................................................................................

## Página 109

                 Medida da associação entre duas variáveis                                     109



                 Temos, então:

                                                      X̅ = 4,4

                                                      Y̅ = 5,7

                                         ∑ (x – X̅ ) . (y – Y̅ ) = 96,5
                                                 i            i


                                                             96,5
                                        Cov(x, y) =                 = 5,08
                                                             19


                 Vamos analisar esse resultado considerando a figura 32,
                 que é bem parecida com a figura 29, com as linhas trace-
                 jadas correspondentes aos valores médios das variáveis
                 Tempo de estudo e Nota.


                            Figura 32 – Tempo de estudo x Nota: valores e médias.




                 Observamos que os valores dos pares ordenados (x, y) con-
                 centram-se no primeiro e no terceiro quadrantes definidos
                 pelas linhas tracejadas, e a covariância entre as variáveis
                 consideradas tem o valor positivo. Isso indica, portanto,
                 que as variáveis Tempo de estudo e Nota têm, entre si,
                 uma associação positiva, ou seja, quando os valores de




..........................................................................................................

## Página 110

      110                 Covariância, correlação e regressão linear




                          Tempo de estudo aumentam, os valores correspondentes
                          de Nota também aumentam.


                          Fazendo cálculos semelhantes para a covariância entre as
                          duplas de variáveis Tempo de lazer (x) e Nota (y), e Dia de
                          nascimento (x) e Nota (y), separadamente, chegaremos aos
                          resultados apresentados logo abaixo.


                          Tempo de lazer (x) e Nota (y):

                                                               X̅ = 27,94

                                                                 Y̅ = 5,7

                                                 ∑ (x – X̅ ) . (y – Y̅ ) = –744,7
                                                          i             i


                                                                  –744,7
                                                 Cov(x, y) =                     = –39,20
                                                                       19


                          Dia de nascimento (x) e Nota (y):

                                                                X̅ = 14,6

                                                                 Y̅ = 5,7

                                                  ∑ (x – X̅ ) . (y – Y̅ ) = –133,2
                                                          i             i


                                                                       –133,2
                                                   Cov(x, y) =                   = –7,01
                                                                            19


                          Assim como no caso da dupla de variáveis Tempo de es-
                          tudo e Nota, vamos considerar esses valores juntamente
                          com os gráficos apresentados nas figuras a seguir.




.........................................................................................................

## Página 111

                 Medida da associação entre duas variáveis                                     111



                              Figura 33 – Tempo de lazer x Nota: valores e médias.




                 Como podemos perceber na figura 33, os pares ordenados
                 (x, y) concentram-se no segundo e no quarto quadrantes
                 definidos pelas linhas tracejadas correspondentes às mé-
                 dias das variáveis consideradas, e o valor da covariância
                 é negativo. Então, podemos dizer que as variáveis Tempo
                 de lazer e Nota têm, entre si, uma associação negativa,
                 pois, quando os valores de Tempo de lazer aumentam, os
                 valores correspondentes de Nota diminuem.


                 Considerando a figura 34, os resultados são menos con-
                 clusivos.




..........................................................................................................

## Página 112

      112                 Covariância, correlação e regressão linear




                                     Figura 34 – Dia de nascimento x Nota: valores e médias.




                          Podemos observar que os pares ordenados (x, y) espa-
                          lham-se pelos quatro quadrantes definidos pelas linhas
                          tracejadas correspondentes às médias das variáveis con-
                          sideradas. Apesar de o valor da covariância ser negativo,
                          não nos parece, pela análise da figura, haver uma associa-
                          ção forte entre as variáveis Dia de nascimento e Nota.


                          O problema é que a covariância não tem uma unidade de
                          medida razoável para analisar a força da associação entre
                          duas variáveis. Por isso, utiliza-se uma outra medida, cha-
                          mada de coeficiente de correlação, que veremos a partir
                          de agora.


                          Coeficiente de correlação
                          O valor da covariância depende das unidades de medida
                          das variáveis consideradas e pode assumir valores entre
                          -∞ e +∞. Isso impede que se façam comparações entre as
                          covariâncias correspondentes a diferentes pares de vari-




.........................................................................................................

## Página 113

                 Medida da associação entre duas variáveis                                     113



                 áveis. Usualmente, eliminamos problemas relacionados à
                 escala dividindo os valores analisados pelo desvio-padrão.
                 Assim, usamos uma medida chamada de coeficiente de
                 correlação (Corr(x, y) ou rxy), que é definida pela seguinte
                 fórmula:


                                                                    Cov(x, y)
                                        Corr(x, y) = rxy =
                                                                      σx σy


                 Em que σx é o desvio-padrão da variável x, e σy é o da variável y.


                 Considerando as variáveis Tempo de estudo (x) e Nota (y),
                 e referindo-se ao valor de covariância previamente calcu-
                 lado e à tabela 11, temos:


                                                Cov(x, y) = 5,08
                                                    σx = 2,09
                                                    σy = 2,51
                                                             5,08
                              Corr(x, y) = rxy =                         = 0,967
                                                        2,09 . 2,51


                 Fazendo cálculos semelhantes para os demais pares de va-
                 riáveis, temos:


                 Tempo de lazer (x) e Nota (y):


                                           Corr(x, y) = rxy = –0,924


                 Dia de nascimento (x) e Nota (y):


                                           Corr(x, y) = rxy = –0,301




..........................................................................................................

## Página 114

      114                 Covariância, correlação e regressão linear




                          O coeficiente de correlação é adimensional, ou seja, não
                          depende das unidades de medida das variáveis analisadas.
                          Seu valor está limitado ao intervalo –1 ≤ rxy ≤ 1. Quanto
                          mais próximo a 1, mais forte a associação crescente entre
                          as variáveis; em outras palavras, quanto mais a variável x
                          cresce, mais a variável y crescerá também. Quanto mais
                          próximo a –1, mais forte a associação decrescente entre
                          elas, ou seja, quanto mais a variável x cresce, mais a variá-
                          vel y diminui. Quanto mais próximo a zero, mais fraca é a
                          associação entre as variáveis.


                          Propriedades da covariância e do coeficiente de corre-
                          lação


                                                Cov(ax + b, cy + d) = acCov(x,y)
                                                                         ac
                                                 Corr(ax + b, cy + d)=       rxy
                                                                        |ac|

                          Observações importantes sobre a covariância

                          Correlação é uma medida de associação linear
                          Devemos ter muito cuidado, pois, quando duas variáveis
                          apresentam coeficiente de correlação igual ou próximo
                          a zero, isso não significa, necessariamente, que elas não
                          estejam associadas. Como dissemos, a covariância é uma
                          maneira de medirmos a associação linear entre as variá-
                          veis. É possível que haja algum outro tipo de associação
                          entre elas, diferente da linear. Vamos considerar os dados
                          da tabela 12, em que as variáveis x e y estão associadas
                          por uma função quadrática (ou seja, os dados foram obti-
                          dos de forma que y = x2). O diagrama de dispersão é apre-
                          sentado na figura 35.




.........................................................................................................

## Página 115

                 Medida da associação entre duas variáveis                                     115



                                      Tabela 12 – Dados – Correlação nula.

                               x                y               x             y
                             -3,0             9,00             0,2           0,04
                             -2,8             7,84             0,4           0,16
                             -2,6             6,76             0,6           0,36
                             -2,4             5,76             0,8           0,64
                             -2,2             4,84             1,0           1,00
                             -2,0             4,00             1,2           1,44
                             -1,8             3,24             1,4           1,96
                             -1,6             2,56             1,6           2,56
                             -1,4             1,96             1,8           3,24
                             -1,2             1,44             2,0           4,00
                             -1,0             1,00             2,2           4,84
                             -0,8             0,64             2,4           5,76
                             -0,6             0,36             2,6           6,76
                             -0,4             0,16             2,8           7,84
                             -0,2             0,04             3,0           9,00
                              0,0             0,00


                             Figura 35 – Diagrama de dispersão – Correlação nula.




..........................................................................................................

## Página 116

      116                 Covariância, correlação e regressão linear




                          Ao efetuarmos os cálculos, chegaremos à conclusão de
                          que o coeficiente de correlação entre x e y é igual a zero,
                          mas essas variáveis, claramente, guardam entre si uma
                          forte associação — que não é linear, como demonstrado
                          pelo coeficiente de correlação nulo.


                          Covariância é muito afetada pela presença de outliers
                          Outliers podem influenciar muito a medida da correlação
                          entre duas variáveis. Vamos considerar o diagrama de dis-
                          persão apresentado na figura 36, cujos dados estão dispo-
                          níveis na tabela 13.


                                           Figura 36 – Diagrama de dispersão – Outliers.




.........................................................................................................

## Página 117

                 Medida da associação entre duas variáveis                                     117



                                        Tabela 13 – Correlação e outliers.

                                                   Dia do mês
                             Id. aluno                                       Nota
                                                   que nasceu
                                  1                      13                  4,7
                                  2                      29                  7,8
                                  3                          8               4,8
                                  4                      12                  3,0
                                  5                      13                  5,5
                                  6                          6               5,8
                                  7                          2               6,5
                                  8                      23                  6,3
                                  9                      10                  2,0
                                 10                      15                  7,8
                                 11                      29                  3,5
                                 12                      15                  5,1
                                 13                      10                  5,5
                                 14                          7               8,2
                                 15                     100                  99,2
                                 16                          5               4,5
                                 17                      20                  10,0
                                 18                      31                  1,0
                                 19                          4               9,5
                                 20                      30                  3,0


                 Podemos notar que o par ordenado assinalado com o cír-
                 culo na figura 36 é bastante discrepante dos demais — fa-
                 zendo-se a análise citada no capítulo 1, ele seria conside-
                 rado um outlier. Na realidade, esses dados são os mesmos
                 já apresentados na tabela 10, exceto que os valores corres-
                 pondentes ao aluno #15 estão diferentes. Se isso tivesse
                 ocorrido na prática, provavelmente teria sido em decorrên-
                 cia de um erro de digitação, pois os valores corretos são
                 10 e 9,2, respectivamente, para Dia de nascimento e Nota.




..........................................................................................................

## Página 118

      118                 Covariância, correlação e regressão linear




                          O coeficiente de correlação entre x e y, considerando os
                          valores errados dessas variáveis, é igual a 0,881. Porém,
                          quando, anteriormente, havíamos calculado o coeficiente
                          de correlação considerando os valores corretos para o alu-
                          no #15, encontramos –0,301!




.........................................................................................................

## Página 119

                 Regressão linear                                                              119




                 REGRESSÃO LINEAR




                 A covariância e o coeficiente de correlação são medidas
                 importantes para avaliar a associação entre duas variáveis
                 quantitativas, mas não são suficientes para indicar, quan-
                 do existe, qual o efeito da variação de uma delas (chamada
                 de variável independente) sobre a outra (chamada de va-
                 riável dependente). Para essa finalidade, usamos a regres-
                 são linear simples.


                 Por meio da análise de regressão, buscamos identificar
                 qual a função linear (ou seja, aquela que pode ser repre-
                 sentada por uma reta) que melhor se ajusta para relacionar
                 as variáveis x (independente) e y (dependente). Em outras
                 palavras, buscamos encontrar uma função linear tal que:

                                          ŷ = b0 + b1 x


                 Em que ŷ é o valor estimado para y pela função linear, b0
                 é o coeficiente linear da reta (ou intercepto), e b1 é seu
                 coeficiente angular (ou inclinação).


                 Método dos mínimos quadrados
                 Genericamente, chamamos de resíduo (e) a diferença entre
                 uma estimativa para o valor de uma variável dependente
                 yi, feita a partir de alguma expressão que a relacione à
                 variável independente, e o valor real observado yi, corres-
                 pondente a cada valor xi. Em termos matemáticos, vamos
                 expressar o resíduo de uma observação como:

                                           ei = ŷi – yi



..........................................................................................................

## Página 120

      120                 Covariância, correlação e regressão linear




                          O método mais comumente usado para encontrarmos os
                          parâmetros da reta de regressão é chamado de método
                          dos mínimos quadrados. Ele tem esse nome, pois está ba-
                          seado na ideia de encontrar os parâmetros b0 e b1, tais que
                          a soma dos quadrados dos resíduos seja a menor possível.
                          Graficamente, essas diferenças correspondem à distância
                          vertical entre cada par ordenado (x, y) e a reta de regres-
                          são, como ilustrado na figura 37.


                                           Figura 37 – Método dos mínimos quadrados.




                          Expressando essa ideia em uma formulação matemática,
                          podemos dizer que o objetivo do método dos mínimos
                          quadrados é minimizar a soma dos quadrados dos resí-
                          duos, ou seja:

                                                                 n
                                                          min   ∑ (ŷ – y )
                                                                       i   i
                                                                               2


                                                               i=1




.........................................................................................................

## Página 121

                 Regressão linear                                                                          121



                 O desenvolvimento matemático dessa função leva aos pa-
                 râmetros da regressão. Assim, para calcularmos o parâme-
                 tro b1, usamos a seguinte fórmula:

                                       n                           n
                                 n ∑i = 1 xi yi – ∑in= 1 xi ∑i = 1 yi                nSxy – SxSy
                          b1 =                                                  =
                                       n ∑ i = 1 x i2 – ( ∑ i = 1 x i) 2
                                               n              n
                                                                                    nSxx – (Sx)2



                 Em que: Sx = ∑i = 1 xi ,                Sy = ∑ i = 1 yi ,          Sxy = ∑i = 1 xi yi e
                                           n                           n                    n



                 Sxx = ∑i = 1 xi 2 .
                           n




                 Alternativamente, podemos usar esta outra fórmula:


                                                                           σy
                                                     b1 = Corr(x,y)
                                                                           σx

                 Em que Corr(x, y) é o coeficiente de correlação entre x e y,
                 σx é o desvio-padrão de x, e σy é o desvio-padrão de y. Uma
                 vez calculado b1, podemos calcular o parâmetro b0:


                                                       b0 = Y̅ – b1 X̅


                 Em que σy é o desvio-padrão de y, σx é o desvio-padrão de
                 x, Y̅ é a média de y, e X̅ é a média de x.


                 Vamos, então, retornar ao exemplo 12 para fazermos a
                 análise das variáveis Tempo de estudo (x) e Nota (y). Para
                 o cálculo dos parâmetros da regressão, transcrevemos os
                 dados referentes às variáveis analisadas e acrescentamos
                 duas colunas à direita, conforme apresentado na tabela 14.




..........................................................................................................

## Página 122

      122                 Covariância, correlação e regressão linear




                          Tabela 14 – Cálculo dos parâmetros da regressão: Tempo de estudo x Nota.

                                             Tempo de
                           Id. aluno       estudo (horas          Nota (yi)    xi . yi     xi2       yi2
                                           semanais)(xi)
                               1                  3,0                  4,7    14,10       9,00      22,09
                               2                  5,4                  7,8    42,12      29,16      60,84
                               3                  2,9                  4,8    13,92       8,41      23,04
                               4                  2,1                  3,0     6,30       4,41       9,00
                               5                  4,1                  5,5    22,55      16,81      30,25
                               6                  5,3                  5,8    30,74      28,09      33,64
                               7                  4,4                  6,5    28,60      19,36      42,25
                               8                  5,5                  6,3    34,65      30,25      39,69
                               9                  1,6                  2,0     3,20       2,56       4,00
                              10                  6,2                  7,8    48,36      38,44      60,84
                              11                  3,1                  3,5    10,85       9,61      12,25
                              12                  4,2                  5,1    21,42      17,64      26,01
                              13                  3,4                  5,5    18,70      11,56      30,25
                              14                  7,0                  8,2    57,40      49,00      67,24
                              15                  7,5                  9,2    69,00      56,25      84,64
                              16                  3,0                  4,5    13,50       9,00      20,25
                              17                  7,4                  10,0   74,00      54,76     100,00
                              18                  0,9                  1,0     0,90       0,81       1,00
                              19                  8,1                  9,5    76,95      65,61      90,25
                              20                  2,6                  3,0     7,80       6,76       9,00
                            Soma:                87,7              113,7      595,06     467,49     766,5
                           Média:                 4,4                  5,7
                          D. Padrão:             2,09                  2,51


                          Substituindo os valores na fórmula de b1, temos:

                                    nSxy – SxSy             20 × 595,06 – 87,7 × 113,7
                            b 1=                        =                                         = 1,16
                                    nSxx – (Sx)2                 20 × 467,49 – 87,72




.........................................................................................................

## Página 123

                 Regressão linear                                                              123



                 Se usarmos a outra fórmula, apenas para nos certificar-
                 mos de seu uso, teremos:


                                                 σy             2,51
                               b1= Corr (x,y)         = 0,967          = 1,16
                                                 σx             2,09

                 Com o valor de b1, podemos calcular o valor de b0:


                                    b0= Y̅ – b1 X̅ = 5,7 – 1,16 × 4,4 = 0,59


                 E, portanto, a reta de regressão, relacionando linearmen-
                 te as variáveis Tempo de estudo (x) e Nota (y), conforme
                 mostrada na figura 38, é a dada pela seguinte expressão:


                                               ŷ = 0,59 + 1,16x


                            Figura 38 – Regressão linear: Tempo de estudo x Nota.




..........................................................................................................

## Página 124

      124                 Covariância, correlação e regressão linear




                          Interpretação dos coeficientes da regressão
                          linear
                          Matematicamente falando, o parâmetro b1 é o coeficiente
                          angular da reta de regressão. Porém, o que ele significa, na
                          prática, é quanto a variável independente x afeta a variável
                          dependente y. Seu valor é a medida de quanto a variável
                          dependente y varia quando a variável independente x so-
                          fre uma variação unitária. No caso do exemplo 12, cada
                          hora adicional de estudo leva, em média, a um aumento
                          de 1,16 pontos na nota.


                          O coeficiente b0 é o coeficiente linear, ou intercepto. Mate-
                          maticamente falando, ele corresponde ao valor da variável
                          dependente y quando a variável independente tem valor
                          nulo. Nem sempre esse parâmetro tem uma interpretação
                          prática. Por exemplo, se fizermos uma regressão entre al-
                          tura (x) e peso (y) dos alunos da turma, chegaremos a uma
                          equação de reta, e o valor do intercepto corresponderia ao
                          peso de uma pessoa com altura nula, o que é algo que não
                          tem qualquer representação prática.


                          Ajuste do modelo de regressão
                          Com as fórmulas apresentadas, conseguimos encontrar
                          os parâmetros b0 e b1 que ajustam de forma melhor um
                          modelo linear para a associação entre as variáveis x e y. A
                          avaliação do quanto uma regressão linear é representativa
                          para essa associação é feita utilizando-se o coeficiente de
                          determinação (r2). Intuitivamente, podemos concluir que
                          esse modelo será tão mais representativo da relação entre
                          as variáveis analisadas quanto maiores forem a covariân-
                          cia e o coeficiente de correlação entre elas.




.........................................................................................................

## Página 125

                 Regressão linear                                                              125



                 Matematicamente, podemos demonstrar que o coeficiente
                 de determinação de uma regressão linear é igual ao qua-
                 drado do coeficiente de correlação entre as variáveis ana-
                 lisadas. Porém, convém entendermos como ele é definido
                 conceitualmente, para compreendermos o que ele signifi-
                 ca e mede.


                 Para avaliar o modelo de regressão linear, devemos tomar
                 algum modelo como base. Para isso, devemos pensar em
                 como fazemos estimativas para o valor de uma variável
                 quando não temos qualquer informação acerca de variá-
                 veis que influenciem em seu comportamento. O melhor
                 e mais usado modelo possível, seguindo essa lógica, é o
                 modelo de estimativa de y a partir de sua média Y̅ . Intui-
                 tivamente, quando não temos informações sobre o rela-
                 cionamento de uma variável com outras, estimamos seu
                 valor por sua média.


                                                   ymi = Y̅


                 Sendo assim, os resíduos das estimativas pela média po-
                 dem ser calculados como:


                                     emi = ymi – yi ; ou emi = Y̅ – yi


                 Portanto, a soma dos quadrados dos resíduos decorrentes
                 de um modelo de estimativa pela sua média Y̅ , chamado
                 de SEQM (acrônimo para a expressão soma dos erros da
                 média ao quadrado), é calculada pela fórmula:

                                    SEQM = ∑i = 1 emi2 = ∑i = 1 (Y̅ – yi)2
                                               n                n




..........................................................................................................

## Página 126

      126                 Covariância, correlação e regressão linear




                          Da mesma forma, os resíduos das estimativas feitas a par-
                          tir da regressão linear podem ser calculados pela seguinte
                          expressão:


                                                               ei = ŷi – yi


                          A soma dos quadrados dos resíduos da regressão, chama-
                          da de SEQ (acrônimo para a expressão soma dos erros ao
                          quadrado), é calculada pela fórmula:


                                               SEQ =      ∑in= 1 ei2 = ∑in= 1 (ŷi – yi)2

                          O coeficiente de determinação (r2) da regressão linear en-
                          tre x e y é dado, então, pela fórmula:


                                                                   SEQm – SEQ
                                                          r2=
                                                                       SEQm


                          No limite superior, quando todos os pontos observados
                          estiverem exatamente sobre a reta de regressão (ou seja,
                          quando uma reta representa perfeitamente o ajuste entre
                          as variáveis analisadas), teremos SEQ = 0 e, portanto, r2 =
                          1. Quando, por outro lado, os valores de x e y não tiverem
                          qualquer relação linear entre si, teremos SEQ = SEQM e,
                          consequentemente, r2 = 1.


                          Além disso, é comum analisarmos a significância do coe-
                          ficiente b1. Para isso, recorremos a um teste de hipóteses,
                          no qual a hipótese nula é de que o valor do coeficiente
                          angular real β1 é zero (ou, em outras palavras, que não
                          existe uma relação linear associando as variáveis analisa-
                          das), contra a hipótese alternativa de que ele tenha o valor



.........................................................................................................

## Página 127

                 Regressão linear                                                                           127



                b1 encontrado. Formalmente, vamos exprimir esse teste de
                hipóteses da seguinte maneira:




                                               {      H 0: β 1 = 0
                                                      H a: β 1 = b 1


                Primeiramente, calculamos o erro padrão do coeficiente
                pela fórmula:
                                                                SEQ
                                                           (n – 1) (n – 2)
                                             S b1 =
                                                                 σx



                Em que SEQ é a soma dos quadrados dos resíduos da re-
                gressão, σx é o desvio-padrão da variável dependente x, e
                n é a quantidade de observações da amostra.


                Com esse valor, podemos calcular a estatística de teste
                para o coeficiente b1:

                                                               b1
                                                      t b1 =
                                                               S b1

                Esse valor deve ser comparado com a estatística crítica,
                considerando uma distribuição t de Student com n – 2
                graus de liberdade. É comum usarmos α = 0,05. Em resu-
                mo, lembrando que é um teste de hipóteses bilateral, tere-
                mos a seguinte regra de decisão:


                            b1
                  t b1 =            > tcrit (n – 2);(1– α ) ⇒ Rejeita H0 (ou seja, β1≠0)
                           S b1                        2



                            b1
                  t b1 =            ≤ tcrit (n – 2);(1– α ) ⇒ Não rejeita H0 (ou seja, aceita que β1 = 0)
                           S b1                        2




..........................................................................................................

## Página 128

      128                 Covariância, correlação e regressão linear




                          Podemos também calcular um intervalo de confiança para
                          o coeficiente β1 da relação entre as variáveis analisadas,
                          usando a seguinte fórmula:


                                    IC (β1; 1 – α) = (b1 – t(n – 2);(1– α ) Sb1 ; b1 + t(n – 2);(1– α ) Sb1)
                                                                           2                         2




                          Retomando os dados do exemplo 12, vamos analisar os
                          resultados obtidos para os parâmetros da regressão li-
                          near entre as variáveis Tempo de estudo (x) e Nota (y).
                          Para isso, consideremos a tabela 15, que, além dos dados
                          das variáveis, apresenta três colunas que auxiliam nos
                          cálculos necessários. A quarta coluna da tabela nos mos-
                          tra as estimativas obtidas utilizando-se os parâmetros da
                          regressão ŷi = 0,59 + 1,16 xi. As duas colunas seguintes
                          trazem o cálculo dos quadrados dos resíduos de cada es-
                          timativa em relação ao valor real observado ei2 = (ŷi – yi)2 e
                          os quadrados dos resíduos de cada observação em relação
                          à média emi2 = (Ŷ – yi)2, respectivamente.


                             Tabela 15 – Ajuste do modelo de regressão: Tempo de estudo x Nota.



                                             Tempo de                                           ei2        emi2
                           Id. aluno       estudo (horas         Nota (yi)        ŷi
                                           semanais)(xi)                                   (ŷi – yi)2   (Ŷ – yi)2

                                1                 3,0               4,7          4,07        0,97          0,40
                                2                 5,4               7,8          6,85        4,47          0,89
                                3                 2,9               4,8          3,95        0,78          0,72
                                4                 2,1               3,0          3,03        7,21          0,00
                                5                 4,1               5,5          5,35        0,03          0,02
                                6                 5,3               5,8          6,74        0,01          0,88
                                7                 4,4               6,5          5,69        0,66          0,65
                                8                 5,5               6,3          6,97        0,38          0,45
                                9                 1,6               2,0          2,45       13,58          0,20
                               10                 6,2               7,8          7,78        4,47          0,00




.........................................................................................................

## Página 129

                 Regressão linear                                                              129




                      11             3,1             3,5       4,19     4,77    0,47
                      12             4,2             5,1       5,46     0,34    0,13
                      13             3,4             5,5       4,53     0,03    0,93
                      14             7,0             8,2       8,71     6,33    0,26
                      15             7,5             9,2       9,29    12,36    0,01
                      16             3,0             4,5       4,07     1,40    0,18
                      17             7,4            10,0       9,17    18,62    0,68
                      18             0,9             1,0       1,63    21,95    0,40
                      19             8,1             9,5       9,99    14,55    0,24
                      20             2,6             3,0       3,61     7,21    0,37
                    Soma:            87,7          113,7               120,1    7,89
                   Média:            4,4             5,7               (SEQM)   SEQ

                 D. Padrão:          2,09           2,51




                 Temos, para esses dados:


                                              SEQM = 120,15

                                               SEQ = 7,89

                                                   120,15 – 7,89
                                             r2=
                                                      120,18

                                                7,89
                                              (19) (18)
                                    S b1 =                  = 0,07267
                                                2,09


                                                   1,16
                                     S b1 =                 = 15,963
                                               0,07267

                 Considerando α = 0,05, com n = 20, temos 18 graus de
                 liberdade. Assim, pesquisando em uma tabela de distri-
                 buição t de Student, temos que tcrit18; 0,975 = 2,101. Como
                 a estatística de teste calculada para b1 é superior ao valor




..........................................................................................................

## Página 130

      130                 Covariância, correlação e regressão linear




                          crítico, rejeitamos a hipótese nula de que o coeficiente an-
                          gular seja zero.


                          Além disso, podemos calcular o IC para o coeficiente an-
                          gular da relação que associa as variáveis Tempo de estudo
                          (x) e Nota (y).


                           IC(β1;0,95) = (1,16 – 2,101 × 0,07267; 1,16 + 2,101 × 0,07267)
                                                   IC(β1;0,95) = (1,001; 1,313)


                          Em outras palavras, é possível assumir que haja uma re-
                          lação linear entre Tempo de estudo (x) e Nota (y) e que o
                          coeficiente angular dessa associação é β1 = 1,16, com 95%
                          de probabilidade de estar entre 1,001 e 1,313.


                          Premissas do modelo de regressão linear
                          Para que a estimativa dos parâmetros do modelo de re-
                          gressão linear pelo método dos mínimos quadrados seja
                          válida, assumimos algumas premissas.


                                   • A relação entre as variáveis analisadas é linear.
                                   • Os resíduos são independentes e normalmente
                                   distribuídos.

                                   • A variância dos resíduos é constante — em ter-
                                   mos mais técnicos, dizemos que os resíduos são ho-
                                   mocedásticos.



                          A normalidade deve se analisada observando o histogra-
                          ma dos resíduos, e a independência e homocedasticidade,
                          um diagrama de dispersão considerando a variável inde-
                          pendente. Por exemplo, a figura 39 mostra um histograma




.........................................................................................................

## Página 131

                 Regressão linear                                                              131



                 de resíduos em que a premissa de normalidade pode ser
                 aceita, enquanto a figura 40 apresenta um diagrama de
                 dispersão com resíduos independentes.


                       Figura 39 – Exemplo de histograma de resíduos — Normalidade.




                       Figura 40 – Diagrama de dispersão — Resíduos independentes e
                                             homocedásticos.




..........................................................................................................

## Página 132

      132                 Covariância, correlação e regressão linear




                          Já a figura 41 nos mostra um caso em que as premissas
                          de independência não são observadas, enquanto a figura
                          42 apresenta um caso em que os resíduos não são homo-
                          cedásticos.

                              Figura 41 – Diagrama de dispersão — Resíduos não independentes.




                              Figura 42 – Diagrama de dispersão — Resíduos não homocedásticos
                                                      (diz-se: heterocedásticos).




.........................................................................................................

## Página 133

                 Regressão linear                                                              133



                 No caso do exemplo 12, analisamos as figuras a seguir.


                                    Figura 43 – Histograma dos resíduos.




                 Observamos que há um problema com a premissa de
                 normalidade dos resíduos, como visto na figura 39. Esse
                 problema, provavelmente, é decorrente do fato de termos
                 uma quantidade relativamente pequena de observações,
                 mas carece ser analisado mais profundamente — o que
                 não é escopo deste livro.


                 Já a independência e a homocedasticidade são analisadas
                 à luz da figura 44.




..........................................................................................................

## Página 134

      134                 Covariância, correlação e regressão linear




                                         Figura 44 – Diagrama de dispersão dos resíduos.




                          Por esse gráfico, não há nada que nos leve a crer que os
                          resíduos não sejam independentes e não apresentem vari-
                          ância constante.




.........................................................................................................

## Página 135

                                                                                               135




                            REFERÊNCIAS


                 ANDERSON, D. R.; SWEENEY, D. J.; WILLIAMS, T. A. Estatísti-
                 ca aplicada à administração e economia. 3. ed. São Paulo:
                 Cengage Learning, 2014.

                 BUSSAB, W. O.; MORETTIN, P. A. Estatística básica. 8. ed.
                 São Paulo: Saraiva, 2013.

                 LEVINE, D. M. et al. Estatística: teoria e aplicações: usando
                 o Microsoft Excel em português. 6. ed. Rio de Janeiro: LTC,
                 2013.

                 MCCLAVE, J. T.; BENSON, P. G.; TERRY, S. Estatística para
                 administração e economia. 10. ed. São Paulo: Pearson
                 Prentice Hall, 2009.

                 STEVENSON, W. J. Estatística aplicada à administração.
                 São Paulo: Harper & Row do Brasil, 1981.



                 Links interessantes

                 Portal Action – Análise de regressão:
                 <http://www.portalaction.com.br/analise-de-regressao>.




..........................................................................................................

## Página 136

136




                  CONSIDERAÇÕES FINAIS


      Procuramos, ao longo deste livro, fazer um apanhado dos
      conceitos básicos da estatística para serem aplicados a si-
      tuações típicas do dia a dia em diversos contextos. Busca-
      mos, por meio de exemplos bastante genéricos e de fácil
      compreensão para todos, ilustrar a aplicação dos concei-
      tos e mostrar como usar as ferramentas apresentadas.


      Iniciamos com o que chamamos de estatística descriti-
      va, falando primeiro sobre os tipos de dados existentes
      e apresentando as ferramentas gráficas mais usadas para
      cada um deles. Em seguida, apresentamos as principais
      medidas de tendência central, de variação e de posição,
      utilizadas para analisarmos conjuntos de dados.


      No capítulo 2, falamos sobre probabilidade e introduzi-
      mos os conceitos de variáveis aleatórias e distribuição de
      probabilidade. Além disso, apresentamos algumas das
      principais distribuições de probabilidade utilizadas, dis-
      cretas e contínuas.


      O capítulo 3 foi dedicado ao assunto de amostragem e
      inferência estatística. Nele, mostramos como se calculam
      intervalos de confiança e se realizam testes de hipóteses
      para estimativa de parâmetros populacionais a partir de
      amostras.


      Por fim, no capítulo 4, abordamos o problema básico de se
      avaliar a relação entre duas variáveis. Para isso, tratamos

## Página 137

                                                            137



dos conceitos de covariância e correlação e apresentamos
a técnica de regressão linear com o método dos mínimos
quadrados.


Certamente, um universo se encontra à nossa frente para
ser descortinado, tantos são os problemas tratados pela
estatística. Porém, o ferramental apresentado neste livro
nos coloca preparados para os próximos passos, pois re-
vela os conceitos e as técnicas fundamentais em todo o
processo. O conhecimento necessário para o estudo da es-
tatística começa por aqui. Bom proveito!

## Página 138

Sem texto selecionável nesta página; consulte a página correspondente no PDF original.

## Página 139

Sem texto selecionável nesta página; consulte a página correspondente no PDF original.

## Página 140

Sem texto selecionável nesta página; consulte a página correspondente no PDF original.
