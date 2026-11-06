Bases de Programação
Bases de Programação
Texto de acordo com as normas do Novo Acordo Ortográfico da Língua Portuguesa.




                             AUTORIA DO CONTEÚDO
                           Cláudio José Marques de Souza



                                     REVISÃO
                                   Janaina Vieira
                                   Lydianna Lima



                       PROJETO GRÁFICO E DIAGRAMAÇÃO
                                    UVA
      A P RE SE NTAÇ ÃO




O computador é uma máquina de propósitos gerais, diferentemente de uma gela-
deira, por exemplo, que, por mais que possa ser “programada” para registrar lista de
compras e outros elementos, sempre possuirá como função principal reduzir a tempe-
ratura do que nela for armazenado. Nesse sentido, o computador é multifuncional, já
que pode servir como calculadora, prancheta de projetos, controlador e supervisor de
uma linha de montagem na indústria, uma tela na qual o artista desenvolva sua arte, e
outras muitas utilidades. Para que ele possa desempenhar seu papel, é necessário que,
de alguma forma, alguém o “ensine” como proceder diante das mais diversas situações
envolvidas em seu uso. Esse ensinamento se faz por meio de uma sequência de instru-
ções que, ao serem executadas, produzem o efeito desejado. De modo geral, é a essa
sequência de instruções que chamamos de algoritmo.

Os algoritmos são tão antigos quanto a própria matemática, no entanto eles só passa-
ram a ganhar visibilidade com o surgimento das máquinas de calcular, dos computado-
res e de outras máquinas que possuem um processador capaz de executar instruções
ao representar o caminho para a solução de um determinado problema. No geral, vá-
rios caminhos levam ao mesmo local e isso também se aplica os algoritmos, uma vez
que um mesmo problema pode ser resolvido com a aplicação de algoritmos diferentes.

No entanto, por ser uma máquina eletrônica, o computador não entende algoritmos,
apenas níveis de tensão elétrica, que naturalmente não possuem um significado em
código para o ser humano. Então, para concluirmos com sucesso o processo de “ensi-
nar” um computador a realizar uma determinada tarefa, devemos partir de algo que
conhecemos, ou seja, a nossa própria linguagem, e chegar à linguagem que a máquina
compreende, que são os níveis de tensão elétrica. Assim, inicialmente algoritmos são
elaborados e transformados em programas de computador, que, por fim, são conver-
tidos na linguagem que a máquina compreende, possibilitando seu perfeito entendi-
mento. Os algoritmos são, portanto, a base da programação de qualquer equipamen-
to que possua um processador.


                                                                                        4
Diante desse contexto, esta disciplina aborda algoritmos como base da programa-
ção de máquinas e equipamentos processados, mas não se limita somente a isso. Ao
compreender os algoritmos e se tornar capaz de criá-los, sem perceber você também
treinará seu cérebro para trabalhar de forma lógica, analítica e estruturada, tanto no
âmbito pessoal como no ambiente profissional, incrementando sensivelmente suas
habilidades de resolução de problemas presentes em seu cotidiano, sejam eles sim-
ples ou mesmo os mais complexos.




                                                                                         5
     AU TO R




CLÁUDIO JOSÉ MARQUES DE SOUZA

Graduado em Engenharia Elétrica pela Universidade Federal de Juiz de Fora - UFJF
(1993) e em Tecnologia de Processamento de Dados pelo Centro de Ensino Superior
de Juiz de Fora - CES JF (1992), com mestrado em Administração de Empresas pelo
Ibmec do Rio de Janeiro (2006). Atualmente é professor na Universidade Veiga de Al-
meida - UVA, onde coordena o curso de Engenharia Elétrica e ministra disciplinas nas
áreas de Circuitos Elétricos e Programação de Computadores. Possui experiência de
mais de 25 anos em gestão de empresas com ênfase em Tecnologia e Relacionamento
com Clientes, atuando em grandes organizações nacionais e internacionais nas áreas
de Infraestrutura, Tecnologia da Informação e Comunicação, Vendas, Atendimento,
Satisfação e Lealdade do Consumidor.




                                                                                       6
1
         Uma introdução aos algoritmos e à programação
         de computadores




Vivemos na era da tecnologia e da informação. Onde quer que estejamos, notamos a
presença de máquinas a facilitar várias atividades humanas. Aprendemos a utilizá-las
com desenvoltura enquanto usuários de suas funcionalidades, mas conseguimos ex-
trair algo além do que elas foram preparadas para nos entregar? Conseguimos adaptar
um recurso existente a uma necessidade para a qual ela não foi exatamente projetada?
A resposta a essas perguntas define de que lado do grande confronto estamos, que
se desenvolve silenciosamente nos tempos modernos. A partir do momento em que
passamos a compreender como funcionam os computadores e os equipamentos pro-
cessados, bem como desenvolvemos a capacidade de programá-los segundo nossas
necessidades, adquirimos condições de entrar para o grupo daqueles que ditarão os
rumos dessas batalhas, em vez de sermos apenas afetados por elas. Este capítulo abor-
dará as bases desse conhecimento.




Conceitos iniciais
Para que se possa automatizar uma tarefa, é necessário que seja especificada, com ab-
solutas clareza e exatidão, cada ação a ser realizada ao longo do processo, bem como
a ordem em que essas ações serão executadas, de forma a garantir sua repetibilidade.
A essa especificação da sequência, finita e ordenada, de passos a serem seguidos para
a realização de uma determinada tarefa, damos o nome de algoritmo.

Para que o computador possa realizar sua tarefa é necessário que ela seja especifica-
da, passo a passo, de forma que seja inteligível pela máquina. A isso damos o nome
de programa, que nada mais é do que um algoritmo escrito em uma linguagem com-
preensível pelo computador.


                                                                                        7
Já uma linguagem de programação de computadores deve possuir meios que permi-
tam a correta especificação e a ordem das operações básicas de computação a serem
realizadas para a solução de um problema.

O processo de desenvolvimento de programas consiste, em seu início, na criação dos
algoritmos que especificarão os passos a serem seguidos para a resolução de um pro-
blema em uma linguagem compreensível pelo ser humano. Em seguida, os algoritmos
serão reescritos em uma linguagem de programação que, enfim, será transformada
em linguagem de máquina, que é a única que o computador é capaz de entender.

Por fim, os passos especificados precisam estar na ordem correta de execução, sob
pena de não resolverem efetivamente o problema para o qual foram criados. Essa or-
dem correta dos passos para a resolução do problema chamamos de lógica.



Tipos descritivos de algoritmos
Existem diversas formas de se representar um algoritmo sem que uma delas seja, ne-
cessariamente, considerada a melhor. Algumas se detêm na parte lógica, não se preo-
cupando com detalhes técnicos relacionados, normalmente, a uma linguagem de pro-
gramação; já outras possuem riqueza de detalhes tão grande que podem, até mesmo,
dificultar o seu entendimento.

O tipo mais natural que existe é a descrição narrativa, na qual os algoritmos são re-
presentados em linguagem natural. Veja os exemplos a seguir:

   Troca de um pneu furado:
   • Afrouxar ligeiramente as porcas.
   • Suspender o carro.
   • Retirar as porcas e o pneu.
   • Colocar o pneu reserva.
   • Apertar as porcas.
   • Abaixar o carro.
   • Dar o aperto final nas porcas.

   Cálculo da média de um aluno:
   • Obter as notas da primeira e da segunda provas.
   • Calcular a média aritmética entre as duas.
   • Se a média for maior ou igual a 7, o aluno será aprovado; senão, ele será
   reprovado.




                                                                                        8
Essa maneira, apesar de ser a mais simples que existe, possibilita o mau entendimento,
a dupla interpretação ou a imprecisão das ações a serem executadas. Por esses moti-
vos, não é tão utilizada no contexto da computação.

Outra maneira de representar um algoritmo é por meio de um fluxograma, que é uma
forma gráfica na qual figuras geométricas representam ações distintas a serem exe-
cutadas. O exemplo a seguir define se um aluno foi aprovado ou não a partir da média
calculada entre suas quatro notas.


                    Início




                    Ler
                   N1, N2,
                   N3, N4




                    Calcular
             M = (N1+N2+N3+N4)/4




                                   S      Apresentar
                   M>=7                    no vídeo               Fim
                                          “Aprovado”

                        N


                  Apresentar
                   no vídeo
                 “Reprovado”




                    Fim




O fluxograma representa alguns aspectos físicos da implementação do algoritmo ao
associar, por exemplo, figuras geométricas diferentes para representar saídas de da-
dos realizadas por dispositivos distintos, como monitor ou armazenamento de dados.

O Diagrama de Chapin é uma alternativa ao fluxograma, que tem como vantagem
a representação hierárquica e estruturada da lógica do algoritmo. Veja o exemplo
a seguir:



                                                                                         9
                                                Início

                                         Leia N1, N2, N3, N4

                                      Média: = (N1+N2+N3+N4)/4


                                            Média > = 7
                                Sim                              Não

                                    Escreva                Escreva
                                  ”Aprovado”             ”Reprovado”


                                                 Fim


A forma que utilizaremos daqui em diante para representarmos os nossos algorit-
mos será a pseudolinguagem. Também conhecida como Português Estruturado, ou
Portugol, é bastante rica em detalhes e se parece muito com a forma como os progra-
mas são escritos, possibilitando uma tradução quase direta para a maioria das lingua-
gens de programação existentes.

A representação de um algoritmo na forma de pseudocódigo é a seguinte:

   Algoritmo <nome_do_algoritmo>
   <declaração_de_variáveis>
   <subalgoritmos>
   Inicio
   <corpo_do_algoritmo>
   Fimalgoritmo

Em que:



 Algoritmo                    Indica o início da definição de um algoritmo em forma de pseudocódigo.


                              É um nome simbólico dado ao algoritmo com a finalidade de distingui-lo
  <nome_do_algoritmo>
                              dos demais.


                              É o local onde são declaradas as variáveis globais usadas no algoritmo prin-
  <declaração_de_variáveis>
                              cipal e, eventualmente, nos subalgoritmos.


  <subalgoritmos>             Parte opcional do pseudocódigo onde são definidos os subalgoritmos.


                              São, respectivamente, as palavras que delimitam o início e o término do
  Início e Fimalgoritmo       conjunto de instruções do corpo do algoritmo, que usualmente chamamos
                              de área de processamento.




                                                                                                             10
A seguir, podemos ver a versão do algoritmo em estudo escrito em pseudolinguagem.

   Algoritmo “Aprovação”
   Var N1, N2, Media: real
   Inicio
        Leia (N1, N2, N3, N4)
        Media := (N1+N2+N3+N4)/4
        Se Media > = 7 então
            Escreva (“Aprovado”)
        senao
            Escreva (“Reprovado”)
        Fimse
   Fimalgoritmo



Algoritmos estruturados
Uma forma bastante utilizada para escrever algoritmos logicamente corretos é o mé-
todo de solução de problemas estruturado, que consiste, basicamente, no seguinte
passo a passo:



    Ler atentamente o enunciado, pois boa parte da resolução de um problema é a com-
    preensão completa de seu enunciado.




          Retirar a relação das entradas de dados do enunciado: quais são os dados que de-
          vem ser fornecidos ao programa a partir de quais são desenvolvidos os cálculos.




                Retirar do enunciado a relação das saídas das informações: descobrir quais são
                as informações que devem ser mostradas como resultado final, que é o objetivo
                do algoritmo.




                       Determinar como transformar as entradas em saídas especificadas: devemos deter-
                       minar qual é a sequência de passos capaz de transformar um conjunto de dados nas
                       informações de resultado.




                                                                                                          11
Visando descrever a sequência de instruções de forma simples e objetiva, utilizamos
algumas técnicas:

   •   Usar só um verbo por frase.
   •   Usar frases curtas e simples.
   •   Ser objetivo.
   •   Procurar usar palavras que não tenham sentido dúbio.



Tipos primitivos de dados
O trabalho realizado por computador consiste na manipulação de informações em
sua memória.



                          Essas informações podem ser de dois tipos:


                Comandam o funcionamento do computador e mostram como os dados devem ser
 Instruções
                tratados.


 Dados          Informações que serão processadas pelo computador.




Os dados podem ser subdivididos segundo seus tipos primitivos, os quais sejam:

Inteiros: dados numéricos positivos ou negativos, excetuando-se números fracionários.
Ex.: 12, -4, 0, 1764, -329 etc.

Reais: dados numéricos positivos ou negativos e números fracionários.
Ex.: 2.5, -127.3, 0, 902438.23 etc.

Caractere: sequência de caracteres contendo letras, números ou tipos especiais.
Ex.: “Cláudio”, “Rua José da Silva”, “904327”, “ “ etc.

Lógico ou booleano: são aqueles que possuem somente dois valores possíveis: verda-
deiro ou falso. Ex.: V, F.




                                                                                           12
Variáveis e constantes
Uma variável é um espaço em memória destinado a armazenar um dado que o pro-
grama de computador poderá manipular ao longo de sua execução. Esse nome ad-
vém do fato de que se pode guardar um dado em cima de um anterior existente, fa-
zendo o conteúdo desse espaço de memória variar (o dado anterior é perdido, pois
foi sobrescrito).

Toda variável possui três atributos: nome, tipo e valor. O nome é o identificador da
variável; o tipo define a natureza do conteúdo que pode ser armazenado e o valor é
o conteúdo que essa variável guarda. Nome e tipo, uma vez definidos no programa,
jamais poderão ser alterados, já o valor pode variar dependendo da manipulação rea-
lizada pelo programa.

Para que se possa utilizar uma variável, é necessário que ela tenha sido criada em me-
mória pelo programa. Esse processo, no jargão da programação, chama-se declarar
a variável.

Em pseudolinguagem, declara-se uma variável da seguinte forma:

                      VAR <nome_da_variável> : <tipo_da_variável>
                                            ou
                      VAR <lista_de_variáveis> : <tipo_das_variáveis>




       Cabe destacar que:

              • A palavra VAR é utilizada só uma vez na definição de um conjunto de
              uma ou mais variáveis.
              • Em uma mesma linha poderão ser definidas uma ou mais variáveis do
              mesmo tipo, devendo-se separar seus nomes por vírgulas.
              • Variáveis de tipos diferentes são declaradas em linhas diferentes.

       Ex.:     VAR     professor, aluno: caractere
                        Idade: inteiro
                        Valor, desconto: real
                        Tem_filhos: lógico




                                                                                         13
Uma constante é um dado armazenado em memória que, ao contrário das variáveis, é
fixo ou estável (não varia). Existem várias utilidades para as constantes e, em diversos
momentos, as utilizaremos em nossos algoritmos, por exemplo, para facilitar o enten-
dimento, a programação e a manutenção do programa (alterando-se seu valor em um
único lugar, alteram-se todos os outros nos quais a constante é utilizada no programa).

Ex.: 		       CONST pi = 3.1416



Comandos primitivos
Os chamados comandos primitivos estão presentes na maioria das linguagens de
programação e se referem àqueles comandos capazes de realizar movimentações de
dados essenciais (entrada e saída).

O comando de atribuição é uma forma de armazenar valores em uma variável. Sua
sintaxe é:
                         <nome_da_variável> := <expressão>
    Ex.: Nome := “Cláudio”
          Nota := 10.0

O comando de saída de dados é o responsável por trazer a informação do computa-
dor para fora dele, na maioria dos casos para os dispositivos de saída (tela, arquivo etc.)
    Ex.: ESCREVA (“Meu nome é:”, nome, “e meu endereço é:”, endereco)

Para realizar o fluxo da informação no sentido inverso, ou seja, levar a informação
para dentro da memória do computador, deve-se utilizar o comando de entrada
de dados.
   Ex.: LEIA (nome, endereco, cidade)

Operadores e expressões

Os operadores são elementos que atuam sobre os operandos para produzir um resul-
tado. São úteis no contexto dos algoritmos para transformar e manipular dados em
memória. Por exemplo, em 8 - 5 relacionam-se dois operandos (8 e 5) por meio de um
operador (-) que representa a subtração.

Podemos classificar os operadores em aritméticos, relacionais, literais e lógicos, além
do operador de atribuição (:=, que também pode ser visto como um comando primiti-
vo), visto anteriormente, que serve para atribuir um valor a uma variável.




                                                                                              14
A sintaxe do operador de atribuição é:

                            NomedaVariável := expressão.

Outros operadores podem ser vistos a seguir:

Os operadores aritméticos dizem respeito às operações aritméticas básicas da mate-
mática. Aqui, vale também a prioridade entre os operadores, que define a ordem em
que eles devem ser avaliados. Ex.: (3+7)*9^2



                                 Operadores aritméticos

    Operador              Tipo                   Operação           Prioridade

         +               Binário                   Adição                4

         -               Binário                 Subtração               4

         *               Binário               Multiplicação             3

         /               Binário                  Divisão                3

       MOD               Binário              Resto da divisão           3

       DIV               Binário               Divisão inteira           3

        **               Binário               Exponenciação             2

         +               Unário             Manutenção do sinal          1

         -               Unário               Inversão do sinal          1




Os operadores relacionais são aqueles que retornam valores “verdadeiro” ou “falso”,
resultados possíveis para uma comparação entre objetos de mesma natureza (variá-
veis do mesmo tipo de dado). Ex.: 9 > 3 (verdadeiro), 7 < 4 (falso).




                                                                                      15
                                Operadores relacionais

             Operador                                  Comparação

                >                                        maior que

                <                                        menor que

                >=                                     maior ou igual

                <=                                     menor ou igual

                =                                           Igual

                <>                                       Diferente




Já os operadores lógicos combinam expressões relacionais e também resultam ape-
nas em “verdadeiro” ou “falso”.



                                  Operadores lógicos

    Operador             Tipo                   Operação                Prioridade

       OU               Binário                 Disjunção                   3

        E               Binário                Conjunção                    2

       NÃO              Unário                  Negação                     1



Fornecendo-se dois valores ou expressões lógicas (expressão1 e expressão2), pode-se
descrever as operações lógicas seguintes:

1) expressão1 E expressão2 é verdadeiro somente se ambas, expressão1 e expres-
são2, forem verdadeiras.

2) expressão1 OU expressão2 é verdadeiro se ao menos um valor entre expressão1 e
expressão2 for verdadeiro.

3) NÃO expressão1 avalia verdadeiro se expressão1 for falsa; de modo contrário, a
expressão NÃO resultará em falso se expressão1 for verdadeira.




                                                                                      16
Desse modo, os operadores literais atuam sobre caracteres e podem variar muito de
linguagem para linguagem, sendo o mais conhecido deles o operador de concatena-
ção (+), que adiciona uma cadeia de caracteres ao final de outra.




       Assim como em matemática, as expressões representam a relação de variá-
       veis e constantes por meio de operadores, gerando uma fórmula que, quan-
       do avaliada, resulta em um valor.

       Ex.:   Expressão aritmética: (5+4)*2 = 18
              Expressão lógica: (5+4)*2 < 10*(7-2) à resulta em Verdadeiro
              Expressão literal: “José” + “ “ + “Carlos” = “José Carlos”




                                                                                    17
      RE SUMO




Neste capítulo, inicial e preparatório para a nossa disciplina, conhecemos os principais
conceitos envolvidos em programação de computadores, como algoritmos, lingua-
gens de programação, variáveis, operadores, comandos básicos etc., bem como os ti-
pos descritivos de algoritmos. Estudamos, ainda, o método estruturado de construção
de algoritmos e os tipos primitivos de dados. Todo esse conteúdo é de suma importân-
cia para nos apresentar os aspectos básicos do pseudocódigo, que certamente estarão
presentes ao longo de todo o nosso estudo e nos possibilitarão construir algoritmos
(computacionais ou não) para a resolução estruturada de problemas, seja no âmbito
pessoal ou profissional.




                                                                                           18
     RE FE RÊ NCIA S




ASCENCIO, A. F. G.; CAMPOS, E. A. V. Fundamentos da programação de computadores:
algoritmos, Pascal, C, C++ e Java. 3. ed. São Paulo: Pearson, 2012.

FORBELLONE, A. L. V; EBERSPÄCHER, H. F. Lógica de programação: a construção de
algoritmos e estruturas de dados. 3. ed. São Paulo: Pearson, 2005.

GUIMARÃES, A. de M.; LAGES, N. A. de C. Algoritmos e estruturas de dados. Rio de
Janeiro: LTC, 1994.




                                                                                   19
