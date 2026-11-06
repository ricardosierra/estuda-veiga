# E-book de Bases de Programação — Capítulo 4

> PDF original preservado em `BPR_Cap_04.pdf`.
> Fonte oficial: [https://ead.uva.br/disciplinas/grad/publica/cont/abc/bpr/eb/BPR_Cap_04.pdf](https://ead.uva.br/disciplinas/grad/publica/cont/abc/bpr/eb/BPR_Cap_04.pdf).
> Canvas — Bases de Programação: [item 1073056](https://uva.instructure.com/courses/55558/modules/items/1073056).
> Baixado em 2026-10-14 pela interface autenticada do Canvas; extraído com `pdftotext -layout -enc UTF-8`.
> Páginas no PDF: 15. Consulte o original para imagens, diagramas e fórmulas que a extração textual não reproduza.

## Página 1

4        Repetição de ações e agrupamento de
         dados homogêneos




Os computadores são especialistas em realizar repetições. Nisso, eles são mais rá-
pidos e mais precisos do que qualquer ser humano! Em inúmeras oportunidades
deparamo-nos com situações em que, para chegarmos ao resultado esperado, neces-
sitamos repetir uma ação, ou até mesmo uma sequência inteira de ações, várias vezes.
Exemplificando, imagine o processo de cálculo da média das notas de um aluno; um
computador pode repeti-lo inúmeras vezes. Da mesma forma, quando se deseja obter
as médias das notas para todos os alunos de uma turma inteira – uma vez para cada
aluno da turma, tornando o processo mais ágil e prático. Neste capítulo iremos estu-
dar como utilizar, em nossos algoritmos, estruturas capazes de impor a execução de
ações (ou blocos de ações) por diversas vezes, beneficiando-nos dessa especialidade
tão importante, existente em máquinas dotadas de processadores lógicos. Além disso,
aprenderemos a lidar com grandes volumes de dados de um mesmo tipo, ampliando
o poder de execução e o escopo de atuação dos nossos algoritmos, ao mesmo tempo
em que geramos códigos enxutos, eficientes e práticos.




Estruturas de repetição
Existem várias situações nas quais desejamos que um comando, ou partes do nosso
código, sejam repetidos determinado número de vezes. Nesses casos, utilizamos as
estruturas de repetição. Claro que poderíamos escrever todo o algoritmo sem utilizar
tais estruturas, mas estaríamos criando um código sem praticidade, grande e de difícil
entendimento e manutenção, entre outros problemas. O algoritmo que possui um
comando ou um bloco de comandos repetidos um determinado número de vezes é
chamado de “algoritmo de repetição”.



                                                                                         1

## Página 2

As estruturas de repetição são também conhecidas como laços ou loops e sua classifi-
cação é realizada de acordo com o conhecimento prévio do número de repetições de
um comando ou de um conjunto de comandos. Dessa forma, as estruturas de repeti-
ção se dividem em:




                              Quando já se sabe, previamente, quantas vezes o comando ou o bloco de
  Laços contados
                              comandos será executado.



                              Quando não se sabe, previamente, o número de vezes que o comando ou
                              o bloco de comandos será executado, pois esse número depende de uma
  Laços condicionais
                              condição que está sujeita a modificações realizadas por instruções contidas
                              no interior da estrutura de repetição.




Contadores e acumuladores

Ao estudar estruturas de repetição, convém compreender dois conceitos muito pre-
sentes na resolução de diversos problemas: contadores e acumuladores.

Um contador é uma variável que recebe um valor inicial (geralmente 0) antes de co-
meçar uma estrutura de repetição, sendo incrementada, no interior da estrutura, por
um valor constante (normalmente 1).

O exemplo a seguir, ilustra essa situação:

                   ...
                   cont := 0
                   <estrutura_de_repetição>
                   ...
                   cont := cont + 1
                   ...
                   <fim_da_estrutura_de_repetição>
                   ...

Uma variável acumuladora recebe 0 (zero) antes de iniciar uma estrutura de repetição
e vai sendo incrementada, no interior da estrutura, por um valor variável (normalmen-
te, a variável utilizada na estrutura de controle).




                                                                                                            2

## Página 3

...
                 soma := 0
                 <estrutura_de_repetição_com_variável_x>
                 ...
                 soma := soma + x
                 ...
                 <fim_da_estrutura_de_repetição>
                 ...



Estruturas de repetição contadas (ou laços contados)

Essa estrutura deve ser utilizada quando se deseja realizar repetições de um comando
– ou bloco de comandos – dentro do algoritmo e já se conhece, de antemão, o número
de vezes que estas deverão ocorrer. Trata-se de uma estrutura que realiza as repeti-
ções ao mesmo tempo em que conta o número de vezes que ela ocorre. Podemos
dizer que possui um contador interno.

A sintaxe dos laços contados é a seguinte:

                 PARA <variável> DE <início> ATE <final> FACA
                 <Comando ou Bloco de comandos>
                 FIMPARA

                 Ex.:    soma := 0
                         PARA i DE 1 ATE 10 FACA
                         soma := soma + i
                         Escreva (i, “ x 7 = ”, i * 7)
                         Escreva (“Soma acumulada = ”, soma)
                         FIMPARA

Em relação à semântica, a estrutura se comporta da seguinte forma:

   • o valor <início> é atribuído à <variável> e, na sequência, o valor da variável
   é comparado com o valor <final>



 se o valor da <variável> for maior do que o valor <final>, o Comando ou o Bloco de comandos não é executado
 e a próxima execução do algoritmo será o primeiro comando após a palavra reservada FIMPARA.



                                                    OU




                                                                                                               3

## Página 4

se o valor da <variável> for menor ou igual ao valor <final>, o Comando ou o Bloco de comandos é executado
 e o valor da <variável> será incrementado em uma unidade. Retorna-se à comparação entre o valor da <variá-
 vel> e o valor <final>, e o processo se repete até que o valor da <variável> exceda ao valor <final>, quando o
 laço tem sua execução finalizada e a próxima execução do algoritmo será o primeiro comando após a palavra
 reservada FIMPARA.




Opcionalmente, pode-se alterar o incremento da contagem do laço para valores dife-
rentes de 1, utilizando-se, inclusive, valores negativos para realizar contagens de trás
para frente. Nesse último caso, deve-se cuidar para que o valor <final> seja menor do
que o valor de <início>, já que a contagem é decrescente.

A sintaxe, utilizando o passo diferente da unidade, é a seguinte:

                 PARA <variável> DE <início> ATE <final> PASSO <valor> FACA
                 <Comando ou Bloco de comandos>
                 FIMPARA

A seguir, um exemplo de algoritmo que calcula a tabuada de um número definido pelo
usuário até o limite definido por ele:

                 Algoritmo “Tabuada”
                 Var i, tab, num: inteiro
                 Inicio

                      Escreva (“Digite o número para o qual deseja a tabuada: ”)
                      Leia (tab)
                      Escreva (“Digite o número até o qual deseja que os cálculos sejam
                    		         realizados: ”)
                      Leia (num)
                      PARA i DE 1 ATE num FACA

                 Escreval (i, “ x ”, tab, “ = ”, i * tab)
                 FIMPARA
                 Fimalgoritmo

Note que o comando Escreval é utilizado para exibir os resultados em tela. Ele é di-
ferente do comando Escreva, pois, assim que realiza a exibição, pula uma linha para
baixo na tela.




                                                                                                                  4

## Página 5

Estruturas de repetição condicionais (ou laços condicionais)

Nesse tipo de estrutura as repetições ocorrerão até que uma determinada condi-
ção seja satisfeita, pois não se sabe, a priori, quantas vezes o corpo do laço deverá
ser executado.

A maioria das linguagens de programação disponibiliza duas estruturas para imple-
mentar esse tipo de laço:




  ENQUANTO FACA            Possui o teste condicional no início da estrutura.



  REPITA ATE               Possui o teste condicional no final da estrutura.




Devemos cuidar para que a variável que realiza o teste condicional nesse tipo de loop,
seja no início ou no final, fique sempre atualizada no interior da estrutura, para evi-
tar que se obtenha uma situação logicamente denominada “laço infinito”, em que
as repetições jamais cessarão por falta de atingimento da condição que defina o
seu encerramento.



Estruturas de repetição condicionais com teste no início (ENQUAN-
TO FACA)

Nessa estrutura o teste lógico que define a continuação ou não das repetições é reali-
zado logo no início do laço. Sua sintaxe é a seguinte:

               ENQUANTO <condição> FACA
               <Comando ou Bloco de comandos >
               FIMENQUANTO

A semântica da estrutura é a que segue:

   • no início do laço a <condição> é testada




                                                                                          5

## Página 6

Se o resultado do teste lógico for Falso, o Comando ou o Bloco de comandos não é executado e o fluxo de
 execução do algoritmo segue para a primeira instrução, após a palavra reservada FIMENQUANTO.



                                                    OU


 Se o resultado do teste lógico for Verdadeiro, o Comando ou o Bloco de comandos é executado e, ao final da
 execução, retorna-se ao teste da <condição>. O processo será repetido, enquanto a avaliação da <condição>
 resultar Verdadeiro. A partir do momento em que esta resultar Falso, o fluxo de execução do algoritmo segue
 para a primeira instrução, após a palavra reservada FIMENQUANTO.




Conforme dito anteriormente, deve-se atentar para a necessidade de, em algum mo-
mento, a avaliação da <condição> retornar Falso como forma de se evitar a ocorrência
do “laço infinito”.

O algoritmo exemplo, a seguir, utiliza a estrutura de repetição com teste no início para
escrever os números maiores ou iguais a 1, enquanto a soma desses mesmos números
não ultrapassar 10000:

                 Algoritmo “Exemplo_enquanto_faca”
                 Var soma, num: inteiro
                 Inicio
                        soma := 0
                        num := 1
                        ENQUANTO soma < 10000 FACA
                 		           Escreva (num)
                 		           num := num + 1
                 		           soma := soma + num
                        FIMENQUANTO
                 Fimalgoritmo



Estruturas de repetição condicionais com teste no final (REPITA ATE)

Nessa estrutura o teste lógico que define a continuação ou não das repetições é reali-
zado no final do laço. Sua sintaxe é a seguinte:

                 REPITA
                 <Comando ou Bloco de comandos>
                 ATE <condição>



                                                                                                               6

## Página 7

A semântica da estrutura é a que segue:

   • Comando ou Bloco de comandos é executado uma vez e ao final do laço a
   <condição> é testada




 Se o resultado do teste lógico for Falso, repete-se a execução do Comando ou do Bloco de comandos e, ao fi-
 nal dessa execução, realiza-se novamente o teste da <condição>. O processo será repetido, enquanto a avalia-
 ção da <condição> resultar Falso. A partir do momento em que esta resultar Verdadeiro, o fluxo de execução
 do algoritmo segue para a primeira instrução, após a palavra reservada ATE.



                                                    OU


 Se o resultado do teste lógico for Verdadeiro, o Comando ou o Bloco de comandos não é mais executado e o
 fluxo de execução do algoritmo segue para a primeira instrução, após a palavra reservada ATE.




Conforme dito anteriormente, deve-se atentar para a necessidade de, em algum mo-
mento, a avaliação da <condição> retornar Falso como forma de se evitar a ocorrência
do “laço infinito”.

Cabe destacar que a estrutura ENQUANTO FACA é diferente da estrutura REPITA ATE
em alguns aspectos, a saber:

   • Em REPITA ATE, o Comando ou o Bloco de comandos é executado uma ou mais
   vezes (no mínimo uma), ao passo que, no ENQUANTO FACA, o Comando ou o Bloco
   de comandos é executado zero ou mais vezes (possivelmente nenhuma). Isso ocor-
   re devido ao teste da <condição> estar localizado no final ou no início da estrutura.

   • Na estrutura REPITA ATE, a repetição se encerra quando o teste da <condição>
   retorna Verdadeiro; já no ENQUANTO FACA, a repetição inicia e continua, enquan-
   to o teste da <condição> retorna Verdadeiro.

   • Em REPITA ATE, a variável envolvida na <condição> pode ser inicializada ou lida
   dentro do laço; no ENQUANTO FACA, isso deve ser feito fora, antes do início da
   estrutura.

O algoritmo exemplo, a seguir, solicita que o usuário digite um número e, na sequên-
cia, calcula e mostra o valor do quadrado do número digitado, repetindo esse processo
até que o usuário decida parar de digitar números respondendo “N” à pergunta “De-
seja continuar?”.


                                                                                                                7

## Página 8

Algoritmo “Exemplo_repita”
              Var
              num : inteiro
              cont: caractere
              Inicio
                     REPITA
              		           Escreva (“digite um número: “)
              		           Leia (num)
              		           Escreva (num, “ elevado ao quadrado é ”, num^2)
              		           Escreva (“Deseja continuar? (S/N)”)
              		           Leia (cont)
                     ATE cont = “N”
              Fimalgoritmo



Estruturas de dados homogêneas unidimensionais
Uma estrutura de dados homogênea é um tipo especial de variável de memória que
permite armazenar e agrupar diversos valores do mesmo tipo primitivo ao mesmo
tempo, ao contrário das variáveis de tipos primitivos, em que somente um valor pode
ser armazenado por vez. São conhecidas também como vetores, arranjos, matrizes,
variáveis indexadas, variáveis compostas etc.

As determinações mais comuns para estruturas homogêneas são matrizes, em uma
determinação mais genérica, e vetores, que são as matrizes compostas por uma linha
e várias colunas.



Vetores

Também conhecidos como matrizes unidimensionais, os vetores são utilizados, comu-
mente, para a criação de tabelas. Trata-se de uma variável única, dimensionada com
um determinado tamanho, capaz de armazenar vários valores sempre do mesmo tipo.




                                                                                      8

## Página 9

Definição

       A sintaxe para a definição de vetores é a seguinte:

                Var <nome_da_variável>: Vetor [<coluna_inicial> .. <coluna_final>]
                de <tipo_de_dado>

                Ex.:   Var V: Vetor [1 .. 10] de Inteiro

       Esse exemplo cria, na memória, um vetor capaz de armazenar 10 valores;
       todos do tipo inteiro.




Operações básicas com vetores

A exemplo do que ocorre com as variáveis simples, também é possível operar com ve-
tores; entretanto, não se opera diretamente com todo o conjunto de dados. O que se
faz, nesse caso, é trabalhar com cada um dos seus componentes de maneira indi-
vidual. Por exemplo, para somar dois vetores soma-se cada um de seus componentes
correspondentes dois a dois. Da mesma forma, as operações de atribuição, a leitura e
a escrita de vetores devem ser realizadas elemento a elemento.

O acesso individual a cada componente é feito pela especificação da sua posição den-
tro da estrutura, utilizando-se, para tanto, o seu índice. O índice é como se fosse um
“endereço” do dado dentro de um vetor.

No exemplo anterior criou-se um vetor de nome V, capaz de armazenar 10 valores
inteiros. Para acessarmos qualquer elemento desse vetor, devemos fornecer o nome
mesmo e o índice do componente, ou posição, que desejamos acessar. Nesse caso, o
índice é um número que varia entre 1 e 10. Observe, a seguir, como realizamos refe-
rências, atribuições de valores e operações utilizando variáveis do tipo vetor.

        Ex:   V[1] referencia o primeiro elemento do vetor V
              V[5] referencia o quinto elemento do vetor V
              V[1] := 8 realiza a atribuição do valor 8 à primeira posição do vetor V

              V[2] := -10 realiza a atribuição do valor -10 à segunda posição do vetor V
              V[5] := 7 realiza a atribuição do valor 7 à quinta posição do vetor V




                                                                                           9

## Página 10

V[8] := V[2] + V[5] atribui o resultado da soma dos valores armazenados
             		      nas posições 2 e 5 à oitava posição do vetor V (-3, nesse caso)




       Leitura e escrita de dados de um vetor

       A leitura e a escrita de um vetor são feitas passo a passo, elemento por ele-
       mento, usando-se a mesma sintaxe das instruções primitivas de entrada e
       saída de dados, explicitando-se o nome do vetor e o componente:

              Sintaxe:   Leia <nome_da_variável> [<índice>]

                         Escreva <nome_da_variável> [<índice>]




Combinando estruturas de repetição com vetores

Para facilitar a manipulação de dados em vetores, é comum utilizarem-se estruturas
de repetição conjugadas com as estruturas de dados a fim de executar as operações
desejadas sobre vários componentes do vetor, um de cada vez. O que se faz, na prá-
tica, é associar o contador das repetições do laço ao índice do vetor que está sendo
manipulado de forma que cada variação do contador do laço também corresponda à
variação do índice do vetor.

Os algoritmos a seguir mostram como se realizam as operações de leitura (exemplo A)
e de escrita (exemplo B) de um vetor:

              A)   Algoritmo “Exemplo_leitura_de_vetor”
                   Var
                   numeros : vetor [1..10] de inteiro
                   i : inteiro
                   Inicio
              		            Para i de 1 ate 10 faca
              			                   Leia (numeros[i])
              		 Fimpara
              Fimalgoritmo




                                                                                       10

## Página 11

B)Algoritmo “Exemplo_escrita_de_vetor”
                Var
                numeros : vetor [1..10] de inteiro
                i : inteiro
                Inicio
              		         Para i de 1 ate 10 faca
              			                Leia (numeros[i])
              		 Fimpara
              		         Para i de 1 até 10 faça
              			                Escreva (numeros[i])
              		 Fimpara
                Fimalgoritmo



No exemplo C, um vetor de 10 elementos é lido e, paralelamente, a soma desses nú-
meros é calculada e exibida no final.

              C)Algoritmo “Exemplo_escrita_de_vetor_com_soma”
                Var
                numeros : vetor [1..10] de inteiro
                i : inteiro
                soma : inteiro
                Inicio
              		         soma := 0
              		         Para i de 1 ate 10 faca
              			                Leia (numeros[i])
              			                soma := soma + numeros[i]
              		 Fimpara
              		         Para i de 1 ate 10 faca
              			                Escreva (numeros[i])
              		 Fimpara
              		         Escreva (“Soma = ”, soma)
                Fimalgoritmo



Por fim, o exemplo D realiza a leitura de um vetor de seis posições e verifica quantos
dos valores armazenados, são negativos.




                                                                                         11

## Página 12

D)Algoritmo “Vetores”
                VAR
                NUMERO: VETOR [1..6] DE REAL
                I, conta_neg: INTEIRO
                inicio
              		        conta_neg <- 0
              		        para i de 1 ate 6 faca
              			               leia (NUMERO[i])
              			               se NUMERO[i] < 0 entao
              				                     conta_neg <- conta_neg + 1
              			               fimse
              		        fimpara
              		        para i de 1 ate 6 faca
              			               escreval (NUMERO[i])
              		        fimpara
              		        escreva (“Total de números negativos: “, conta_neg)
                fimalgoritmo




Aplicações práticas de vetores

Normalmente, os vetores são utilizados em tarefas importantes de processamento de
dados, como pesquisa e classificação. Pesquisar significa procurar um determinado
valor dentro de um vetor e classificar consiste em arranjar os elementos dentro de
um vetor, segundo determinada ordem, de acordo com um critério específico.

Dos diversos métodos de classificação existentes, um dos mais populares é o bubble
sort, ou o método da bolha. Ele não é o mais eficiente, mas sua simplicidade o tornou
popular. Consiste em percorrer todo o vetor comparando elementos vizinhos e, caso
estejam fora de ordem, devem ser trocados entre si. Realiza-se esse procedimento
até atingir o final do vetor. Na primeira “varredura” garante-se que o último elemento
esteja em seu devido lugar (na ordem crescente, ele é o maior); na segunda “varre-
dura” garante-se que o penúltimo elemento esteja em seu devido lugar e assim por
diante, até que todos os elementos estejam corretamente posicionados. O processo
completo realiza um número de varreduras igual ao número de elementos do vetor
menos um.

Nosso último exemplo mostra o algoritmo bubble sort ordenando 50 números intei-
ros em ordem crescente:




                                                                                         12

## Página 13

Algoritmo Bubble_Sort
             Var
             numeros : vetor [1..50] de inteiros
             aux, i, j: inteiro
             Inicio
                    Para i de 1 ate 50 faca
             		              Leia (numeros[i])
                    Fimpara
                    j := 50
                    Enquanto j > 1 faça
             		              Para i de 1 ate j-1 faca
             			                     Se numeros[i] > numeros[i+1] Entao
             				                           aux := numeros[i];
             				                           numeros[i] := numeros[j+1];
             				                           numeros[j+1] := aux;
             			Fimse
             		              Fimpara
             		              j:=j-1;
                    Fimenquanto
                    Escreva (“vetor ordenado: ”)
                    Para i de 1 ate 50 faca
             		              Escreva (numeros[i])
                    Fimpara
             Fimalgoritmo



Observa-se que, para ordenar o vetor em ordem decrescente, é só inverter o sinal
de comparação no teste da condição lógica Se numeros[i] > numeros[j], ficando:
Se numeros[i] < numeros[j].




                                                                                   13

## Página 14

RE SUMO




O conteúdo deste capítulo nos mostrou como explorar a possibilidade de implemen-
tar repetições para construir um código mais enxuto, limpo e eficiente. As estruturas
de repetição se mostram úteis e adequadas quando podemos contar o número de
repetições que desejamos que ocorram ou quando estamos sujeitos à constatação
de determinada condição para decidirmos se continuamos, ou não, a repetir uma par-
te do nosso algoritmo. Ao combinarmos estruturas de repetição com estruturas de
dados homogêneas, ou vetores, é possível dotar nossos algoritmos de grande poder
de manipulação e processamento de dados, tornando possível a resolução dos mais
complexos problemas.

Por fim, ao dominar as três estruturas básicas de programação – sequencial, condicio-
nal e de repetição – tornamo-nos capazes de escrever soluções estruturadas para re-
solver qualquer tipo de problema em que exista um ordenamento lógico para sua so-
lução, independentemente de sua complexidade. Este é um conhecimento que, sem
dúvida alguma, agrega um alto valor a qualquer profissional no mundo corporativo
moderno, seja qual for sua área de atuação, além de tornar sua capacidade de racio-
cínio muito mais estruturada e apta a realizar análises mais complexas, bem como a
propor soluções mais assertivas e eficientes.




                                                                                        14

## Página 15

RE FE RÊ NCIA S




ASCENCIO, A. F. G.; CAMPOS, E. A. V. Fundamentos da programação de computadores.
2. ed. São Paulo: Pearson, 2012.

FORBELLONE, A. L. V.; EBERSPACHER, H. F. Lógica de programação. 3. ed. São Paulo:
Pearson, 2005.

GUIMARÃES, A. M.; LAGES, N. A. C. Algoritmos e estruturas de dados. Rio de Janeiro:
LTC, 1994.




                                                                                      15
