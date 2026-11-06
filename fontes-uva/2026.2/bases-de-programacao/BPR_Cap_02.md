# E-book de Bases de Programação — Capítulo 2

> PDF original preservado em `BPR_Cap_02.pdf`.
> Fonte oficial: [https://ead.uva.br/disciplinas/grad/publica/cont/abc/bpr/eb/BPR_Cap_02.pdf](https://ead.uva.br/disciplinas/grad/publica/cont/abc/bpr/eb/BPR_Cap_02.pdf).
> Canvas — Bases de Programação: [item 1073054](https://uva.instructure.com/courses/55558/modules/items/1073054).
> Baixado em 2026-10-14 pela interface autenticada do Canvas; extraído com `pdftotext -layout -enc UTF-8`.
> Páginas no PDF: 11. Consulte o original para imagens, diagramas e fórmulas que a extração textual não reproduza.

## Página 1

2
         O uso de execuções em sequência e subdivisões de
         programas




De acordo com o pensamento estruturado, resolver um problema consiste em deter-
minar uma sequência de passos, ou ações, que serão executados e em que ordem
isso se dará. No entanto, existem problemas maiores e de maior complexidade, os
quais, talvez, seja mais interessante subdividir em problemas menores e determinar a
solução de cada um desses subproblemas de forma que, justapostas, essas soluções
menores componham a solução do problema maior e mais complexo. Veremos, nesta
unidade, como implementar essa sequencialidade de ações e como propor soluções
para problemas parciais que comporão a solução de um problema maior no âmbito
dos algoritmos.




Estrutura sequencial
Até o momento, aprendemos sobre variáveis e comandos primitivos capazes de reali-
zar a entrada ou a saída de dados do computador e, no máximo, operar e atribuir valo-
res às variáveis de memória. No entanto, somente esses comandos não são suficientes
para construirmos algoritmos capazes de resolver qualquer situação que se apresente.
É necessário realizar diversos tipos de ações no intuito de capturar, transformar e re-
tornar dados, implementando uma sequência lógica de instruções que determinarão a
lógica da solução do problema.

Pode ser preciso executar essas instruções em ordem de ocorrência no algoritmo;
pode ser necessário executar ações diferentes tendo como referência a avaliação de
uma determinada situação; e pode ser que uma instrução, ou um grupo de instruções,
seja repetida diversas vezes visando chegar ao objetivo do algoritmo. Em resumo, é



                                                                                          1

## Página 2

necessário determinar e controlar a ordem da execução das instruções dos algoritmos,
o que chamamos de fluxo de execução do algoritmo.

Em programação estruturada realizamos a determinação dessa ordem utilizando três
estruturas de controle de fluxo de execução: a estrutura sequencial, a estrutura
condicional e a estrutura de repetição. Neste capítulo, abordaremos a primeira es-
trutura; as demais veremos nos capítulos 3 e 4, respectivamente.

Na estrutura sequencial os comandos e as instruções devem ser executados, obriga-
toriamente, uns após os outros, na exata ordem em que aparecem no algoritmo, ou
seja, um comando só é executado quando a execução do comando anterior a ele
já tiver sido completamente encerrada.

A estrutura sequencial é delimitada pelas palavras reservadas início e fimalgoritmo,
e contém apenas comandos de entrada de dados, saída de dados e atribuições de va-
lores (vistos no Capítulo 1).



       Apesar de simples, essa estrutura se mostra capaz de resolver alguns proble-
       mas em que não seja necessário repetir comandos ou blocos de comandos,
       bem como não seja fundamental a realização de uma escolha baseada em
       uma determinada condição ao longo da execução do algoritmo.




Na sequência, apresentaremos um exemplo simples de confecção de algoritmos se-
quenciais, que pode ser aplicado para desenvolver qualquer algoritmo cuja solução
possua características puramente sequenciais.



       Escreva um algoritmo em pseudocódigo, capaz de calcular e exibir a força
       da gravidade em Newtons, que atua sobre um objeto de massa conhecida a
       ser informada pelo usuário (em kg), em qualquer planeta cuja aceleração da
       gravidade também deve ser informada pelo usuário (em m/s2). Dica: F=m.g




Para resolver esse problema começaremos por ler e entender completamente o
enunciado da situação apresentada. Não há como resolver um problema sem antes
termos certeza de que sabemos realmente o que é e do que trata esse problema. E,
acredite, muitos dos erros cometidos ao se criar um algoritmo são originados quando
essa diretriz fundamental é negligenciada.



                                                                                       2

## Página 3

Perfeitamente entendida a situação, é hora de proceder à identificação dos dados de
entrada do algoritmo, ou seja, quais são as informações contidas no enunciado que
se deve fornecer ao algoritmo para que ele possa trabalhar e chegar ao resultado que
se espera. Nesse caso, ao examinarmos o enunciado do problema fica claro que as in-
formações que serão imputadas no algoritmo são a massa do objeto e a aceleração da
gravidade no planeta em questão.

A próxima etapa consiste em determinar, também pelo enunciado, quais são os dados
de saída ou informações que deverão ser exibidas como o resultado do algoritmo.
Normalmente, são as respostas que resolvem o problema e são o objetivo do al-
goritmo. Aqui, no exemplo apresentado, o que se pede é a determinação da força da
gravidade à qual estará submetido o objeto.

Por fim, devemos determinar a sequência de ações que devem ser realizadas visan-
do transformar o conjunto dos dados de entrada nas saídas que representam o resul-
tado. Essa etapa define a parte mais dinâmica do algoritmo. No nosso caso, consiste
em transformar as entradas (massa e aceleração da gravidade) em uma saída (força da
gravidade), por meio do uso da segunda lei de Newton, da Física, que diz que a força
é sempre diretamente proporcional ao produto da aceleração de um corpo pela sua
massa, ou seja: F=m.a. Nesse caso, a fórmula foi dada pelo enunciado, mas nem sem-
pre isso ocorre, sendo necessário que o programador tenha de recorrer a conhecimen-
tos prévios ou realizar pesquisas na área envolvida no problema.

Agora que já passamos pelas quatro etapas preparatórias, podemos escrever nosso
algoritmo utilizando o que já aprendemos até aqui, em termos de estrutura do algo-
ritmo em pseudolinguagem, declaração de variáveis e comandos de entrada de dados,
saída de dados e atribuição de valores a variáveis.

Inicialmente, descreveremos a estrutura do algoritmo já o batizando com um nome.

                           Algoritmo “Força da Gravidade”
                           <declaração_de_variáveis>
                           <subalgoritmos>
                           Início
                                  <corpo_do_algoritmo>
                           Fimalgoritmo

Na sequência, realizaremos a declaração de todas as variáveis pela especificação de
seus nomes e respectivos tipos, que serão utilizadas pelo algoritmo para guardar e
manipular dados em memória.




                                                                                       3

## Página 4

Nesse caso, criaremos três variáveis:

     1. Para guardar a informação sobre a massa do objeto.
     2. Para armazenar a informação da aceleração da gravidade no planeta.
     3. Para receber o resultado do cálculo da força da gravidade à qual o objeto es-
     tará submetido nesse determinado planeta.

Como se trata de um problema simples, não haverá necessidade de subdividi-lo em
problemas menores, razão pela qual eliminaremos, nesse exemplo, o uso dos subalgo-
ritmos, que serão assunto tratado mais adiante.

                            Algoritmo “Força da Gravidade”
                            Var
                            Forca, Massa, Aceleracao: real
                            Início
                                   <corpo_do_algoritmo>
                            Fimalgoritmo

Note que definimos o tipo “real” para as variáveis, por entender que ele representa
o conjunto de dados mais adequado aos dados que serão armazenados na memó-
ria, sendo assim que devemos proceder ao definir o tipo de todas as variáveis de
um algoritmo.

Para finalizar, devemos construir a parte do corpo do algoritmo, em que vamos
utilizar os comandos primitivos, que já conhecemos, para capturar e trabalhar as in-
formações a serem fornecidas pelo usuário do programa. Nesse contexto, devem ser
realizadas as seguintes etapas:

     1. Interagir com o usuário, indicando, por meio de mensagens, o que se deseja
     que ele faça no momento da execução do algoritmo (chamamos essa parte de
     interface homem-máquina).
     2. Trabalhar os dados fornecidos pelo usuário para chegar às respostas espera-
     das, por meio de manipulações dos dados, envolvendo comandos e operações.
     3. Por fim, apresentar a resposta ao usuário, também por meio de mensagens
     interativas (outra parte da interface).

Desse modo, podemos propor a seguinte sequência:

   Algoritmo “Força da Gravidade”
   Var
   Forca, Massa, Aceleracao: real



                                                                                        4

## Página 5

Inicio
         ESCREVA(“Digite o valor da massa do objeto (em kg):”)
         LEIA(Massa)
         ESCREVA(“Digite agora o valor da aceleração da gravidade no planeta considerado
   (em m/s2):”)
         LEIA(Aceleracao)
         Forca := Massa * Aceleracao
         ESCREVA(“A força à qual o objeto está submetido é igual a”, Forca, “N”)
   Fimalgoritmo




       Existem alguns programas, inclusive de uso livre e de distribuição gratuita, que
       permitem criar, editar, interpretar e executar os algoritmos em portugol (por-
       tuguês estruturado), como se fosse um “programa” normal de computador.

       Experimente digitar e executar esse algoritmo fazendo uso de um desses
       programas. Observe, principalmente, os efeitos de cada comando e execu-
       ção sequencial, linha a linha, do algoritmo. Após isso, desafie-se a resolver
       novos problemas simples com a criação de algoritmos sequenciais.




Subprogramas – Funções e procedimentos
A complexidade de um algoritmo depende do problema que ele se propõe resolver.
Na maioria das vezes, problemas complexos exigem algoritmos mais extensos para
sua solução. Sempre é possível dividir um problema complexo em problemas menores
e resolver esses problemas menores por meio de algoritmos (subalgoritmos), de for-
ma que, ao se unir esses subalgoritmos, tem-se a solução do problema original.

Essa técnica é conhecida como método dos refinamentos sucessivos, sendo impor-
tante para:

   • Dividir o algoritmo complexo, facilitando o seu entendimento.
   • Estruturar o algoritmo, facilitando a descoberta de erros e sua documentação.
   • Modularizar sistemas, facilitando a criação, a manutenção e o reaproveitamen-
   to do código gerado.

O uso de subalgoritmos também pode servir para evitar a repetição de comandos ou
de blocos de comandos que se repetem ao longo do algoritmo, diminuindo-se, consi-
deravelmente, o tamanho do algoritmo desenvolvido.


                                                                                           5

## Página 6

Na prática, o algoritmo é dividido em um algoritmo principal e tantos subalgoritmos
quantos forem necessários. No momento de execução, o algoritmo principal “cha-
ma” o subalgoritmo à execução e este, após executado, devolve o fluxo de execução
para o algoritmo principal, procedendo-se dessa maneira até que a execução total
seja completada.

Em termos de estrutura, o algoritmo que possui subalgoritmos fica da seguinte forma:

                        Algoritmo <nome do algoritmo>
                        Var <definição das variáveis globais>
                               <definições dos subalgoritmos>
                        Início
                               <corpo do algoritmo principal>
                        Fimalgoritmo

A definição do subalgoritmo é composta:

   • Por um cabeçalho, no qual se define o nome e o tipo do subalgoritmo, assim
   como suas variáveis locais e parâmetros.
   • Pelo corpo do subalgoritmo, que é onde se encontram os comandos que esse
   subalgoritmo deve executar.

Convém destacar que o nome do subalgoritmo será utilizado pelo algoritmo chama-
dor para invocar o subalgoritmo à execução. Outros pontos importantes são que as
variáveis locais somente existem e podem ser utilizadas no momento de execução do
subalgoritmo. Além disso, os parâmetros do algoritmo são as vias bidirecionais que
promovem a comunicação entre o subalgoritmo e o algoritmo chamador.

Existem dois tipos de subalgoritmos: procedimentos e funções.



Funções

Uma função é um subalgoritmo que tem suas origens nas funções da matemática, cuja
ideia é calcular um valor a partir de outros fornecidos à função. São exemplos clássicos
as funções raiz quadrada, seno, cosseno, logaritmo etc. A chamada a uma função apa-
rece sempre no meio de uma expressão situada em um comando de atribuição dentro
do algoritmo principal. Essa expressão deve manipular dados sempre do mesmo tipo
do que o do valor retornado pela função.




                                                                                           6

## Página 7

A sintaxe para a realização da definição de uma função, em pseudocódigo, é a seguinte:

                         FUNCAO <nome> (<parâmetros>) <tipo_de_dado>
                         VAR <variáveis locais>
                         INICIO
                               <comando composto>
                         FIMFUNCAO

Em que:


          <nome>              É o nome pelo qual a função será chamada por outros algoritmos.

      <parâmetros>            São os parâmetros de comunicação entre a função e o algoritmo chamador.

      <tipo de dado>          É o tipo de dado retornado pela função ao algoritmo chamador.

    <variáveis locais>        É a definição das variáveis válidas internamente à função.

   <comando composto>         É o conjunto de instruções a ser executado pela função.



A seguir, veremos o exemplo de um algoritmo que utiliza função para calcular o valor
da raiz quadrada de um número.

                          Algoritmo “Exemplo_de_função”
                          Var A, B : real

                          Funcao RaizQuad(X : real): real
                          Var Y : real
                          Inicio
                          Y := X^(1/2)
                          Retorne Y
                          Fimfuncao

                          Inicio
                          Escreva (“Digite um número: “)
                          Leia (A)
                          B := RaizQuad(A)
                          Escreva (“A raiz quadrada de”, A, “ é”, B)
                          Fimalgoritmo




                                                                                                        7

## Página 8

Ao digitar e executar esse algoritmo em um programa com características já
       descritas, observe, principalmente, a chamada da função executada pelo algo-
       ritmo principal e a passagem de valores entre o algoritmo principal e a função.
       Depois disso, desafie-se a resolver novos problemas, desta vez mais comple-
       xos, com a criação de algoritmos sequenciais, utilizando funções.




Procedimentos

Um procedimento é um subalgoritmo que retorna (ou não) valores ao algoritmo prin-
cipal, por meio de parâmetros ou de variáveis globais (válidas ao longo de toda a exe-
cução do algoritmo). Sua chamada somente ocorre com comandos isolados, ao contrá-
rio das funções, que são invocadas no meio de expressões.

A sintaxe para a realização da definição de um procedimento, em pseudocódigo, é a
seguinte:

                         PROCEDIMENTO <nome> (<parâmetros>)
                         Var <variáveis locais>
                         Inicio
                                <comando composto>
                         Fimprocedimento

Em que:


          <nome>            É o nome pelo qual o procedimento será chamado por outros algoritmos.

                            São os parâmetros de comunicação entre o procedimento e o algoritmo cha-
      <parâmetros>
                            mador.
    <variáveis locais>      É a definição das variáveis válidas internamente ao procedimento.

   <comando composto>       É o conjunto de instruções a ser executado pelo procedimento.



A seguir, para efeitos de melhor comparação, veremos um exemplo de um algoritmo
que utiliza um procedimento para calcular o valor da raiz quadrada de um número.

                         Algoritmo “Exemplo_de_procedimento”
                         Var A, B : real




                                                                                                       8

## Página 9

Procedimento RaizQuad(X : real)
                      Var Y : real
                      Inicio
                      Y := X^(1/2)
                      Escreva (Y)
                      Fimprocedimento

                      Inicio
                      Escreva (“Digite um número: “)
                      Leia (A)
                      Escreva (“A raiz quadrada de”, A, “ é”)
                      RaizQuad(A)
                      Fimalgoritmo

Ao digitar e executar esse algoritmo, em um programa com características já descritas,
observe, sobretudo, a chamada do procedimento executado pelo algoritmo principal
por meio de um comando isolado e a passagem de valores entre algoritmo principal
e o procedimento. Após isso, desafie-se a resolver novos problemas, dessa vez mais
complexos, com a criação de algoritmos sequenciais, utilizando procedimentos.




                                                                                         9

## Página 10

RE SUMO




Neste capítulo aprendemos a desenvolver algoritmos sequenciais, que são aqueles
que não admitem desvios ou repetições de trechos em seu fluxo de execução, para
solucionarmos problemas cujos processos de obtenção de respostas apresentam es-
sas características. Aprendemos, também, a como dividir um problema grande e com-
plexo em problemas menores, bem como a utilizarmos os subalgoritmos para chegar
às soluções desses problemas menores, compondo, com elas, a solução do problema
original. Dessa forma, já dispomos de um ferramental simples, porém efetivo, para
propormos soluções por meio de algoritmos para boa parte das situações que viven-
ciamos no cotidiano pessoal e profissional.




                                                                                    10

## Página 11

RE FE RÊ NCIA S




ASCENCIO, A. F. G.; CAMPOS, E. A. V. Fundamentos da programação de computado-
res. 2. ed. São Paulo: Pearson, 2012.

FORBELLONE, A. L. V.; EBERSPACHER, H. F. Lógica de programação. 3. ed. São Paulo:
Pearson, 2005.

GUIMARÃES, A. M.; LAGES, N. A. C. Algoritmos e estruturas de dados. Rio de Janei-
ro: LTC, 1994.




                                                                                    11
