# E-book de Bases de Programação — Capítulo 3

> PDF original preservado em `BPR_Cap_03.pdf`.
> Fonte oficial: [https://ead.uva.br/disciplinas/grad/publica/cont/abc/bpr/eb/BPR_Cap_03.pdf](https://ead.uva.br/disciplinas/grad/publica/cont/abc/bpr/eb/BPR_Cap_03.pdf).
> Canvas — Bases de Programação: [item 1073055](https://uva.instructure.com/courses/55558/modules/items/1073055).
> Baixado em 2026-10-14 pela interface autenticada do Canvas; extraído com `pdftotext -layout -enc UTF-8`.
> Páginas no PDF: 11. Consulte o original para imagens, diagramas e fórmulas que a extração textual não reproduza.

## Página 1

3        Tomada de decisões em programas estruturados




O método de resolução de problemas nem sempre é linear e sequencial. Muitas ve-
zes, devemos proceder de formas distintas frente a condições diferentes verificadas
em certos pontos do processo. Para tornar nossos algoritmos capazes de executar
ações diferenciadas diante de necessidades específicas para cada condição existente,
utilizamos as estruturas de seleção ou estruturas condicionais. Com elas, é possível
implementar desvios no fluxo de execução dos algoritmos de modo que eles possam
se comportar de maneiras diferentes diante da avaliação das condições existentes e,
efetivamente, tomar decisões lógicas e adequadas que levarão à solução do problema.
Neste capítulo veremos como utilizar esse recurso tão importante, que nos possibili-
tará propor soluções para problemas mais complexos do que aqueles solucionados
apenas com instruções sequenciais.




Estruturas de seleção
Escrever algoritmos especificando ações que devem ser executadas sequencialmente,
uma a uma, da primeira até a última instrução, possibilita-nos resolver problemas sim-
ples. Esse recurso é suficiente quando não é necessário avaliar uma determinada con-
dição para saber qual atitude tomar, dependendo do resultado dessa análise. Ocorre
que a maioria dos problemas mais complexos dependem dessas avaliações para que
sejam executadas as ações mais adequadas em cada caso. Assim, os algoritmos que
resolvem esses casos terão passos que serão executados somente quando determina-
das condições forem observadas. Esse tipo de algoritmo é conhecido como “algoritmo
de seleção”.




                                                                                         1

## Página 2

Nesta seção estudaremos duas estruturas que nos permitem selecionar, escolher ou
decidir quais ações serão executadas em nossa lógica de programação: a estrutura
condicional simples e a estrutura condicional composta. Adicionalmente, estuda-
remos a estrutura de múltipla escolha, recurso capaz de implementar um caso espe-
cífico da estrutura condicional composta.



Estrutura condicional simples

É aquela cuja única condição, ou expressão lógica, é avaliada. Caso a avaliação retorne
o valor lógico verdadeiro, uma ação (ou um bloco de ações) será executada; caso a ava-
liação retorne o valor lógico falso, não será executada.

A sintaxe para a estrutura de decisão simples é a seguinte:

          SE <condição> ENTAO
          <Comando ou Bloco de comandos>
          FIMSE

          Ex:        SE X > 50 ENTAO
                            cont := cont + 1
                            soma := soma + X
                            Escreva (X, “ é maior que 50”)
          FIMSE

A semântica da estrutura condicional simples é:

• A condição é avaliada:



        Se o resultado dessa avaliação for Verdadeiro, o Comando ou o Bloco de
        comandos – ou tudo o que começa depois da palavra reservada ENTAO e
        termina antes da palavra reservada FIMSE – será executado. Em seguida, o
        fluxo do algoritmo executará a próxima instrução existente depois da pala-
        vra reservada FIMSE.



                                          OU




                                                                                          2

## Página 3

Se o resultado dessa avaliação for Falso, o fluxo do algoritmo executará a
        próxima instrução depois da palavra reservada FIMSE e o Comando ou o Blo-
        co de comandos – ou tudo o que começa depois da palavra reservada ENTAO
        e termina antes da palavra reservada FIMSE – não será executado.




Veremos, a seguir, um exemplo de algoritmo que lê um número qualquer digitado pelo
usuário e escreve se ele é maior do que 50 (perceba que a condição a ser analisada é “o
número digitado ser maior do que 50” e, em caso afirmativo, alguma ação é executada):

          Algoritmo “Estrutura_de_decisão_simples”
          Var A : inteiro
          Inicio
          Escreva (“Digite um número qualquer:”)
                 Leia (A)
                 SE A > 50 ENTAO
                 Escreva (A, “ é maior que 50”)
          FIMSE
          Fimalgoritmo



Estrutura condicional composta

Nessa estrutura, uma única condição, ou expressão lógica, é avaliada. Caso a avalia-
ção retorne o valor lógico Verdadeiro, uma ação (ou um bloco de ações) será executa-
da; caso a avaliação retorne o valor lógico Falso, uma OUTRA ação (ou um bloco de
ações) será executada.

A sintaxe para a estrutura de decisão composta é a seguinte:

          SE <condição> ENTAO
                 <Comando_1 ou Bloco de comandos_1>
          SENAO
                 <Comando_2 ou Bloco de comandos_2>
          FIMSE




                                                                                          3

## Página 4

Ex:       SE X > 50 ENTAO
                           cont := cont + 1
                           soma := soma + X
                           Escreva (X, “ é maior que 50”)
          		        SENAO
                           cont_b := cont_b + 1
                           soma_b := soma_b + X
                           Escreva (X, “ não é maior que 50”)
          		        FIMSE



A semântica da estrutura condicional composta é a seguinte:

· A condição é avaliada:




       Se o resultado dessa avaliação for Verdadeiro, o Comando_1 ou o Bloco de
       comandos_1 – ou tudo o que começa depois da palavra reservada ENTAO e
       termina antes da palavra reservada SENAO – será executado. Em seguida, o
       fluxo do algoritmo executará a próxima instrução existente depois da pala-
       vra reservada FIMSE.




                                         OU




       Se o resultado dessa avaliação for Falso, o Comando_2 ou o Bloco de
       comandos_2 – ou tudo o que começa depois da palavra reservada SE-
       NAO e termina antes da palavra reservada FIMSE – será executado. A
       seguir, o fluxo do algoritmo executará a próxima instrução existente
       depois da palavra reservada FIMSE.




                                                                                    4

## Página 5

Na sequência, temos o exemplo de algoritmo que lê um número qualquer digitado
pelo usuário e escreve se ele é ou não é maior do que 50 (perceba que a condição a ser
analisada é “o número digitado ser maior do que 50” e, sendo ou não, alguma ação é
executada):

                     Algoritmo “Estrutura_de_decisão_simples”
                     Var A : inteiro
                     Inicio
                     Escreva (“Digite um número qualquer:”)
                              Leia (A)
                              SE A > 50 ENTAO
                                     Escreva (A, “ é maior que 50”)
                              SENAO
                                     Escreva (A, “ não é maior que 50”)
                              FIMSE
                     Fimalgoritmo

Em relação ao uso das estruturas de programação, sobretudo considerando a lógica
a ser implementada, as diversas formulações das condições e os posicionamentos das
estruturas podem levar a algoritmos distintos e é necessário ficar atento a isso. Veja,
por exemplo, algoritmos diferentes, capazes de determinar a qual quadrante do plano
cartesiano pertence um ponto (dado pelas coordenadas X e Y) digitado pelo usuário,
ou se ele se encontra sobre um dos eixos.

     A)      Algoritmo “Estruturas_concatenadas”
             Var x, y : inteiro
             Inicio
             Escreva(Digite as coordenadas X e Y do ponto a ser pesquisado:”)
             Leia (x, y)
             SE x=0 e y=0 ENTAO
             Escreva(“Ponto na origem”)
             FIMSE
             SE x=0 e y<>0 ENTAO
             Escreva(“Ponto sobre o eixo Y”)
             FIMSE
             SE x<>0 e y=0 ENTAO
             Escreva(“Ponto sobre o eixo X”)
             FIMSE
             SE x>0 e y>0 ENTAO
             Escreva(“Ponto no quadrante I”)
             FIMSE



                                                                                          5

## Página 6

SE x<0 e y>0 ENTAO
     Escreva(“Ponto no quadrante II”)
     FIMSE
     SE x<0 e y<0 ENTAO
     Escreva(“Ponto no quadrante III”)
     FIMSE
     SE x>0 e y<0 ENTAO
     Escreva(“Ponto no quadrante VI”)
     FIMSE
     Fimalgoritmo

B)   Algoritmo “Estruturas_aninhadas”
     Var x, y : inteiro
     Inicio
              Escreva(Digite as coordenadas X e Y do ponto a ser pesquisado:”)
              Leia (x, y)
              SE x<>0 ENTAO
                      SE y=0 ENTAO
                             Escreva (“Ponto sobre o eixo X”)
                      SENAO
                             Se x>0 ENTAO
                                   SE y>0 ENTAO
                                         Escreva (“Ponto no quadrante I”)
                                   SENAO
                                         Escreva (“Ponto no quadrante IV”)
                                   FIMSE
                             SENAO
                                   SE y>0 ENTAO
                                         Escreva (“Ponto no quadrante II”)
                                   SENAO
                                         Escreva (“Ponto no quadrante III”)
                                   FIMSE
                             FIMSE
                      FIMSE
              SENAO
                      SE y=0 ENTAO
                             Escreva (“Ponto na origem”)
                      SENAO
                             Escreva (“Ponto sobre o eixo Y”)
                      FIMSE
              FIMSE
     Fimalgoritmo


                                                                                 6

## Página 7

Perceba que as estruturas concatenadas tornam o algoritmo mais legível e as estru-
turas aninhadas ou encadeadas fazem com que o algoritmo seja mais rápido em sua
execução. Essa rapidez deve-se ao fato de realizar menos testes e comparações, resul-
tando em um menor número de passos executados.


Estruturas de múltipla escolha

Quando um conjunto de valores para uma determinada variável precisa ser testado e
ações distintas associadas a esses valores devem ser executadas, estamos diante de
uma estrutura aninhada do tipo se-senao-entao. Nesse caso, podemos utilizar a estru-
tura de seleção de múltipla escolha, que tornará nosso código mais prático e simples.

A sintaxe da construção de escolha é:

              ESCOLHA <expressão de seleção>
                  CASO <exp_1>
                        <Bloco de comandos_1>
                  CASO <exp_2>
                        <Bloco de comandos_2>

              .......

                   CASO <exp_n>
                         <Bloco de comandos_N>
                   OUTROCASO
                   <Bloco de comandos_O>
              FIMESCOLHA

A semântica da estrutura escolha caso é a seguinte:

• O valor da expressão de seleção é verificado




       <exp_1> é testada e, se for Verdadeira, o Bloco de comandos_1 é exe-
       cutado. Em seguida, o fluxo do algoritmo executará a próxima instru-
       ção existente depois da palavra reservada FIMESCOLHA.




                                                                                        7

## Página 8

OU




        Se <exp_1> for Falsa, <exp_2> é testada e, se for Verdadeira, o Bloco
        de comandos_2 é executado. A seguir, o fluxo do algoritmo executará a
        próxima instrução existente depois da palavra reservada FIMESCOLHA.




                                         OU




        Se <exp_2> for Falsa, <exp_3> é testada e assim sucessivamente.




                                         OU




        Se todas as condições forem avaliadas como Falsas, o Bloco de coman-
        dos_O será executado.




O próximo exemplo utiliza a estrutura de múltipla escolha que acabamos de estudar
para implementar um algoritmo cuja função é solicitar ao usuário que digite o nome e
a idade de um atleta para, a seguir, determinar sua categoria. A categoria varia confor-
me a tabela a seguir:

                               IDADE CATEGORIA
                               De 05 a 10 Infantil
                               De 11 a 15 Juvenil
                               De 16 a 20 Junior
                               De 21 a 25 Profissional




                                                                                           8

## Página 9

Nesse caso, em vez de fazer uso de uma sequência de estruturas condicionais com-
postas encadeadas ou aninhadas, utilizaremos apenas uma estrutura de múltipla es-
colha. Ela terá, por objetivo, testar o conteúdo armazenado na variável idade, que
definirá a categoria.

                  Algoritmo “CLASSIFICAÇÃO DE ATLETAS
                  var
                  nome, categoria : caractere
                  idade : inteiro
                  Inicio
                          Escreva(“Nome do Atleta = “)
                          Leia (nome)
                          Escreva(“Idade do Atleta = “)
                          Leia (idade)
                          Escolha idade
                                 caso 5,6,7,8,9,10
                                       categoria <- “Infantil”
                                 caso 11,12,13,14,15
                                       categoria <- “Juvenil”
                                 caso 16,17,18,19,20
                                       categoria <- “Junior”
                                 caso 21,22,23,24,25
                                       categoria <- “Profissional”
                                 outrocaso
                                       categoria <- “INVALIDO”
                          Fimescolha
                          Escreva (“Categoria = “,categoria)
                  Fimalgoritmo




                                                                                     9

## Página 10

RE SUMO




Neste capítulo estudamos as estruturas de desvio condicional e aprendemos como
elas podem nos auxiliar a implementar lógicas mais complexas que envolvem tomadas
de decisões. Conhecemos as estruturas de desvios condicionais simples e composta,
bem como a estrutura de seleção múltipla. Compreendemos que, a depender da ló-
gica de resolução adotada para um determinado problema, pode ser necessário ou
interessante realizar a concatenação das estruturas (uma estrutura após a outra) ou
a utilização de estruturas “dentro” de estruturas, o que chamamos de aninhamento
(conforme visto no exemplo da determinação do quadrante do plano cartesiano em
que está situado um determinado ponto). A partir de todo esse aprendizado, dispo-
mos de ferramental adequado para propor lógicas de soluções mais complexas que
envolvem, além da característica sequencial natural, também as tomadas de decisão.
Tais possibilidades tornam nosso poder de resolução de problemas muito maior frente
aos desafios geralmente presentes em ambientes corporativos e em nosso cotidiano.




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
