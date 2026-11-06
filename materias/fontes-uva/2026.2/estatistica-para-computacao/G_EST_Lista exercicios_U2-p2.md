# Lista de exercícios — Unidade 2, parte 2

> Disciplina: Estatística para Computação (E10300025). Seção: E10300025-21CMP2262A-2026.2.21. Período: 2026.2.21.
> Canvas: [item 1104285](https://uva.instructure.com/courses/57827/modules/items/1104285)
> PDF anexado ao item Canvas 1104285 (original preservado em `G_EST_Lista exercicios_U2-p2.pdf`).
> Baixado em 2026-10-13 pela interface autenticada do Canvas e extraído com `pdftotext -layout -enc UTF-8`.
> Páginas do PDF: 4. Diagramas, imagens e conteúdo que exista somente como imagem permanecem no PDF original.

## Página 1

                                           ESTATÍSTICA

                                PROFª ROBERTA MENDIONDO




Problemas propostos: Probabilidade Condicional

Problema 1

     Foi realizado um levantamento com os trabalhadores de uma empresa de comércio varejista, no
     qual se perguntou o meio de transporte utilizado para se chegar ao local de trabalho. Constatou-
     se que 10% vão a pé, 40%, de carro e 50% de ônibus. No entanto, nem todos chegam no horário.
     A pesquisa também verificou que atrasos ocorrem dentre 5% dos que vão a pé, 10% dos que vão
     de carro e 15% dos que usam ônibus.
     Uma trabalhador é selecionado aleatoriamente. Calcule a probabilidade de essa pessoa chegar
     atrasada:

Problema 2

     O bufê de saladas de um restaurante apresenta alface, tomate, agrião, cebola, pepino, beterraba
     e cenoura. Calcule os tipos de saladas diferentes que podem ser preparados com cinco desses
     ingredientes.

Problema 3

     Uma professora de Ciência de Dados mantém na sala de aula livros que empresta a seus alunos,
     organizados em três caixas: na 1ª coloca livros de Estatística, na 2ª coloca livros de Computação e na
     3ª, livros sobre negócios. Observou-se que a probabilidade de um de seus alunos escolher a caixa 1
     para retirar um livro aleatoriamente é de 0,3; a caixa 2 é 0,2 e a caixa 3 é 0,5. Dentro de cada caixa, há
     livros classificados para dois públicos: iniciantes e intermediários. Em cada caixa, temos:
        • Caixa 1: 40% dos livros são para alunos iniciantes.

        • Caixa 2: 30% dos livros são para alunos iniciantes.

        • Caixa 3: 20% dos livros são para alunos iniciantes.

     Um livro para iniciantes foi selecionado. Calcule a probabilidade de que ele seja da caixa 3.

Problema 4

     Theo, Lúcia, Márcio, Camila e Lucas são estagiários de uma empresa que oferece um serviço de
     van para transportá-los à faculdade, ao término do expediente diário. Para deixar tudo
     organizado, eles formam uma fila enquanto esperam para entrar no transporte.
     Calcule de quantos modos essa fila pode ser formada:



                                                                                                                  1

## Página 2

Problema 5

       Uma companhia multinacional tem três fábricas que produzem o mesmo tipo de produto. A fábrica
       I é responsável por 30% do total produzido, a fábrica II produz 45% do total, e o restante vem da
       fábrica III. Cada uma das fábricas, no entanto, produz uma proporção de produtos que não
       atendem aos padrões estabelecidos pelas normas internacionais. Tais produtos são considerados
       defeituosos e correspondem a 1%, 2% e 1,5%, respectivamente, dos totais produzidos por
       fábrica. No centro de distribuição, é feito o controle de qualidade da produção de todas as
       fábricas. Um item produzido foi selecionado aleatoriamente.
       Se o item selecionado é defeituoso, avalie a probabilidade que tenha sido fabricado pela fábrica II.




RESOLUÇÕES

Problema 1

Resposta: 0,12.

Vamos designar o evento A da seguinte forma:

A: pessoa chega atrasada.

A probabilidade de A é calculada como sendo:

P(A) = P(A ∩ a pé) + P(A ∩ carro) + P(A ∩ ônibus).

P(A) = (0,1 x 0,05) + (0,4 x 0,1) + (0,5 x 0,15) = 0,12 = 12%




Problema 2

Resposta correta: 21.

Esse é um problema de combinação, no qual m=7 e r=2. Assim, temos:

C =C =         7!
 m,r    7,5
              2!.5! = 21




                                                                                                              2

## Página 3

Problema 3

Dados do problema:
P(1)=0,3
P(2)=0,2
P(3)=0,5

P(in 1)=0,4
P(in 2)=0,3
P(in 3)=0,2

P(3/in) = ????




Resposta: Aproximadamente 36% é a probabilidade de um aluno escolher um livro para iniciantes da caixa 3.




Problema 4

Resposta: 120.

É um enunciado de permutação, no qual m=5.

Então, a resposta correta é Pm = m! , ou seja, P5 = 5! = 5 . 4 . 3 . 2 . 1 = 120 .




Problema 5

## Página 4

3
