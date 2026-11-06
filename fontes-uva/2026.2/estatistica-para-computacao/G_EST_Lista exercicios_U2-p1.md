# Lista de exercícios — Unidade 2, parte 1

> Disciplina: Estatística para Computação (E10300025). Seção: E10300025-21CMP2262A-2026.2.21. Período: 2026.2.21.
> Canvas: [item 1104283](https://uva.instructure.com/courses/57827/modules/items/1104283)
> PDF anexado ao item Canvas 1104283 (original preservado em `G_EST_Lista exercicios_U2-p1.pdf`).
> Baixado em 2026-10-13 pela interface autenticada do Canvas e extraído com `pdftotext -layout -enc UTF-8`.
> Páginas do PDF: 3. Diagramas, imagens e conteúdo que exista somente como imagem permanecem no PDF original.

## Página 1

                                         ESTATÍSTICA

                                    PROFª ROBERTA MENDIONDO




Problemas propostos: Probabilidade

Problema 1


     Um executivo mora em Porto Alegre e trabalha em São Paulo, durante a semana, e no ambiente da
     empresa é exigido o uso de camisa social. Ele, então, separa 8 camisas brancas e 4 camisas azuis,
     mesmo sabendo que, provavelmente, não use todas elas.
     Se no primeiro dia em São Paulo, o executivo selecionar 2 camisas aleatoriamente e sem reposição,
     qual será a probabilidade serem duas camisas brancas


Problema 2


     Um estudante de Engenharia tem o hábito de dormir tarde e, por isso, costuma acordar com muito
     sono para ao estágio. Ele sempre usa o mesmo par de tênis com meia, que é trocada diariamente.
     Na segunda-feira, o estudante escolheu aleatoriamente um par de meias dentre os 11 disponíveis,
     sendo 6 pares de cor branca, 3 pares de cor vermelha e o restante de cor preta.


     Calcule a probabilidade de ele ter escolhido ao acaso um par de meias de cor preta.


Problema 3


     O quadro, a seguir, fornece o quantitativo de pessoas, de acordo com o sexo e o time pelo qual torce.


                     Time pelo qual torce                      Sexo
                                                   Feminino           Masculino
                  Flamengo                            80                  130
                  Vasco                               75                  120
                  Fluminense                          55                   45
                  Botafogo                            40                   35


     Uma pessoa desse grupo é selecionada aleatoriamente. Calcule a probabilidade de essa pessoa
     ser do sexo feminino e torcer pelo Vasco.




                                                                                                             1

## Página 2

Problema 4


      As probabilidades de que duas pessoas A e B executem exatamente uma tarefa de inspeção duran-
      te um dia de trabalho são: P(A) = 3/4 e P(B) = 3/5.


      Calcule a probabilidade de que ao menos uma dessas pessoas realize uma tarefa de inspeção hoje.


Problema 5


      Uma família tem dois cães: Nick e Dick. A probabilidade de dar banho no Nick no próximo fim de
      semana é de 2/3; já a probabilidade do Dick tomar banho é de 3/5. Calcule a probabilidade de, nesse
      fim de semana, Nick e Dick não tomarem banho.



RESOLUÇÕES

Problema 1


14/33.


Como são 8 camisas brancas e 4 azuis, há um total de 12 camisas.


Como é uma seleção sem reposição, a probabilidade é:


                                  8         7        14
P(duas camisas brancas) =               .        =
                                  12        11       33


Problema 2


P(pares de meias pretas) = 2/11


Total de pares de meia é 11.


nº de pares de meia brancas + nº de pares de meia vermelhas + nº de pares de meia pretas = 11


6 + 3 + nº de pares de meia pretas = 11


nº de pares de meia pretas = 11-9 = 2


Os eventos são equiprováveis, portanto a probabilidade procurada é: P(meias pretas) = nº de pares de
meia pretas / total de pares de meia = 2/11




                                                                                                            2

## Página 3

Problema 3


Vamos observar o quadro a seguir, com alguns valores grafados.



             Time pelo qual torce                     Sexo
                                         Feminino              Masculino                             Soma
         Flamengo                            80                     130                               210
         Vasco                               75                     120                               195
         Fluminense                          55                     45                                100
         Botafogo                            40                     35                                   75
                                                                                                      580




                                                     75            15
A probabilidade pedida é: P(Feminino ∩ Vasco) =            =                = 0,12931
                                                    580        116


Aproximadamente 13% de probabilidade de que um torcedor selecionado ao acaso deste grupo, seja do
sexo feminino e torcedora do Vasco.


Problema 4


Nesse caso, o espaço amostral conta com 4 eventos, ou seja: S = {AB, AB, A B, A B}


A probabilidade pedida é a de que ao menos uma das pessoas realize tarefa de inspeção, ou seja:



                                                                    1               2                 1           9
P=(AB ou AB ou A B) = P(AB) + P(A B) + P(A B) = 1 - P(A B) = 1 -            .               =1-               =
                                                                    4               5                10           10


Problema 5


                         2                                                      2                1
P(Nick tomar banho) =        , então P(Nick não tomar banho)= 1−                        =
                         3                                                      3                3


                         3                                                      3             2
P(Dick tomar banho) =        , então P(Dick não tomar banho)= 1−                        =
                         5                                                   5                5


 Assim, a probabilidade de nenhum dos dois tomarem banho é: P(Nick não tomar banho e Dick não tomar
                                                                        1               2            2
banho) = P(Nick não tomar banho) * P(Dick não tomar banho) =                    *            =
                                                                        3               5            15



                                                                                                                       3
