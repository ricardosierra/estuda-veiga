# E-book de Estatística — Adriana Tostes

> Disciplina: Estatística para Computação (E10300025). Seção: E10300025-21CMP2262A-2026.2.21. Período: 2026.2.21.
> Canvas: [item 1104271](https://uva.instructure.com/courses/57827/modules/items/1104271)
> PDF oficial (original preservado em `EST_ebook_Adriana_Tostes.pdf`): [https://ead.uva.br/disciplinas/grad/publica/cont/def/est/eb/EST_ebook_Adriana_Tostes.pdf](https://ead.uva.br/disciplinas/grad/publica/cont/def/est/eb/EST_ebook_Adriana_Tostes.pdf)
> Baixado em 2026-10-13 pela interface autenticada do Canvas e extraído com `pdftotext -layout -enc UTF-8`.
> Páginas do PDF: 292. Diagramas, imagens e conteúdo que exista somente como imagem permanecem no PDF original.

## Página 1

ESTATÍSTICA

## Página 2

ESTATÍSTICA

## Página 3


          Texto de acordo com as normas do Novo Acordo Ortográfico
                            da Língua Portuguesa.




AUTORIA DO CONTEÚDO                                  PROJETO GRÁFICO
Adriana Maria Balena Tostes                          UVA


REVISÃO                                              DIAGRAMAÇÃO
Janaina Vieira                                       UVA
Lydianna Lima




T716       Tostes, Adriana Maria Balena.
              Estatística [recurso eletrônico] / Adriana Maria Balena Tostes. – Rio de
           Janeiro: UVA, 2021.

               1 recurso digital (3621 KB)

               Formato: PDF
               ISBN 978-65-5700-089-2
			
               1. Estatística. 2. Probabilidades. I. Universidade Veiga de Almeida. II.
            Título.




                                                                                    CDD – 519.5



                   Bibliotecária Adriana R. C. de Sá CRB 7 – 4049.
          Ficha Catalográfica elaborada pelo Sistema de Bibliotecas da UVA.

## Página 4

                                               SUMÁRIO



Apresentação                                        6
Autor                                               7

UNIDADE 1

Introdução ao estudo estatístico                    8
• A estatística no Brasil e no mundo

• Elaboração e análise de gráficos e tabelas

• Medidas de posição e de dispersão




UNIDADE 2

Probabilidade                                       93
• Introdução à probabilidade

• Conceitos de probabilidade

• Probabilidade condicional

## Página 5

                                                          SUMÁRIO



UNIDADE 3

Distribuições de Probabilidade                                 166
• Variáveis aleatórias e distribuições de probabilidade

• Distribuições de probabilidades discretas

• Distribuição de probabilidade normal




UNIDADE 4

Intervalos de confiança e Relação entre variáveis              228
• Intervalos de confiança

• Correlação

• Regressão linear

## Página 6

                                         APRESENTAÇÃO


O mundo em que vivemos está repleto de uma quantidade extraordinária de informa-
ções. Sozinhas, essas informações podem gerar grandes confusões ou simplesmente
podem não ter significado algum.


Para coletar, organizar, analisar, apresentar, interpretar e tirar conclusões sobre elas preci-
samos de um ramo poderoso da Matemática Aplicada, que denominamos de Estatística.


A Estatística é ferramenta essencial para fazer o tratamento de informações, que deno-
minamos de tratamento dos dados. É considerada disciplina-chave da Teoria de Infor-
mação, tendo como objetivo principal subsidiar o processo decisório. Ela é indispensável
para qualquer profissional que necessita analisar informações em suas tomadas de de-
cisão diárias, seja no campo profissional ou na vida pessoal.



                      Não podemos escapar dos dados, assim como não podemos evitar o
                      uso de palavras. Tal como palavras os dados não se interpretam a si
                      mesmos, mas devem ser lidos com entendimento. Da mesma maneira
                      que um escritor pode dispor as palavras em argumentos convincentes
                      ou frases sem sentido, assim também os dados podem ser convincen-
                      tes, enganosos ou simplesmente inócuos. A instrução numérica, a capa-
                      cidade de acompanhar e compreender argumentos baseados em dados,
                      é importante para qualquer um de nós. O estudo da estatística é parte
                      essencial de uma formação sólida. (MOORE, 2000)



Seja qual for a área ou o objeto de estudo do pesquisador, este poderá utilizar conceitos
de Estatística. É difícil encontrar uma situação real em que não se utilize a Estatística para
solucionar problemas, gerenciar qualidade, comparar metodologias, testar hipóteses, evi-
denciar tendências, diminuir riscos e, principalmente, auxiliar na tomada de decisões.




                                              6

## Página 7

                                                                  AUTOR




ADRIANA MARIA BALENA TOSTES

Graduada em Engenharia Civil pela Universidade Federal de Juiz de Fora – UFJF, licen-
ciada em Matemática pela Universidade Federal de Santa Catarina – UFSC e mestra em
Educação Matemática pela Universidade Severino Sombra – USS. Iniciou sua trajetória
docente no Ensino Superior em 1991, ingressando como professora da Universidade Vei-
ga de Almeida – UVA em 2010. Ministra aulas de Cálculo Elementar, Cálculo I, Cálculo
II, Matemática I, Matemática II, Matemática Financeira e Estatística, para os cursos de
Engenharias, Administração e Ciências Contábeis.




                                           7

## Página 8

                   UNIDADE 1

Introdução ao estudo estatístico

## Página 9

                                                    INTRODUÇÃO


Nesta unidade apresentaremos os principais conceitos da Estatística, bem como os plane-
jamentos para coleta de dados, organização por meio de tabelas, apresentação a partir de
gráficos e cálculo das medidas de resumo, tanto as medidas de posição quanto as medidas
de dispersão, os quais compõem a estatística descritiva. O que veremos nos tópicos, a saber:


     • Tópico 1: um breve histórico enfocando a relevância da Estatística no mundo em
     que vivemos e algumas situações que exemplificam o mau uso dessa área do co-
     nhecimento humano. Seu estudo se desenvolverá ancorado em conceitos básicos
     como dados, população e amostra, censo e amostragem, parâmetros e estatísticas,
     tipos de variáveis, tipos de amostragens, índices e indicadores.
     • Tópico 2: a organização e a visualização dos conjuntos de dados, utilizando tabe-
     las, sejam elas com ou sem intervalos de classes, e gráficos de diversos tipos, como
     os histogramas, os polígonos de frequências, a Ogiva de Galton, além dos gráficos
     de barras, colunas, setores e linhas, associando o uso de tecnologia para essa ela-
     boração. Esses conhecimentos e habilidades são fundamentais para o desenvol-
     vimento de competências associadas à resolução de problemas que envolvam a
     tomada de decisão a partir da organização, apresentação e análise de dados.
     • Tópico 3: medidas descritivas, divididas em medidas de posição, que são a média,
     a moda e a mediana, e as medidas de dispersão, que são a amplitude, a variância, o
     desvio-padrão e o coeficiente de variação, fundamentais para resumir as informações.




                                         OBJETIVO

        Nesta unidade você será capaz de:

        • Realizar a coleta, a organização, a descrição e a análise dos dados referentes a
        uma pesquisa.




                                                9

## Página 10

A estatística no Brasil e no mundo

Podemos pensar que os conceitos de Estatística nasceram no mundo contemporâneo, em
que valorizamos cada vez mais a rapidez e a quantidade de informações, quando o avan-
ço tecnológico é incessante. Entretanto, sabemos que a utilização dos conceitos da Esta-
tística como suporte para a tomada de decisões é verificada também no mundo antigo.


O primeiro registro disponível acerca de um levantamento estatístico refere-se à solicita-
ção de Heródoto que, em 3050 a.C., no Egito, promoveu um estudo populacional com a
intenção de quantificar a disponibilidade de recursos humanos e econômicos para a cons-
trução das pirâmides. Determinados autores apontam ainda que anos antes, em 5000
a.C., já havia ocorrido um levantamento para quantificar os egípcios presos de guerra.


Podemos dividir a evolução da Estatística em três fases. São elas:




                            Ocorre na Idade Média, quando as coletas de informações ti-
         1ª fase
                            nham finalidades tributárias ou bélicas.


                            Por volta do século XVI iniciam-se as primeiras análises siste-
         2ª fase            máticas dos fatos sociais, tais como casamentos, óbitos, nas-
                            cimentos, migração, entre outros.


                            No século XVIII a Estatística foi batizada por Godofredo Achen-
         3ª fase            wall como nova ciência, estabelecendo, assim, seus objetivos e
                            relacionamentos com as demais ciências




Agora veremos a evolução da estatística no Brasil até o momento atual. Observe a tabela
a seguir.



                                  Destaques no Brasil


 1585       Padre José de Anchieta registra os habitantes de algumas capitanias.




                                            10

## Página 11

       Primeiro censo geral da população brasileira feito por José Maria da Silva
1872
       Paranhos, conhecido como Visconde do Rio Branco.




       Criação do Conselho Superior de Estatística, com vistas à padronização de
1907
       conceitos e apuração de resultados em todo o território nacional.




       Criação do Instituto Nacional de Estatística, que só passou a existir de fato em
       1936, mudando em 1938 para Instituto Brasileiro de Geografia e Estatística –
1934
       IBGE, que traz uma série de informações referentes à população, economia e
       uma variedade de indicadores sociais.




       Criação do Instituto Nacional de Estudos e Pesquisas Educacionais Anísio
       Teixeira – INEP (www.inep.gov.br), que apresenta uma vasta quantidade de
1937
       informações referentes à área educacional, trazendo resultados de censos
       escolares, avaliações institucionais; sinopses estatísticas e microdados.




       Iniciaram os “modernos censos” decenais, não ocorrendo apenas o de 1990
1940   (foi adiado para 1991), devido à “falta de recursos” alegada pelo governo Collor.
       Antes disso ocorreram os de 1872, 1890, 1900 e 1920.




1947   Primeiro curso de Inferência dado no Brasil baseado no livro de Cramer.




       Duas escolas iniciaram o Ensino de Estatística no Brasil: uma no Rio de Janeiro,
1953   a Escola Nacional de Ciências Estatística (ENCE) e a outra conhecida como
       Escola de Estatística da Bahia.




                                         11

## Página 12

            O Brasil teve a honra de receber Sir Ronald Aylmer Fisher para participar do 2°
 1955
            Congresso Internacional de Biometria, realizado em Campinas.




            Criação do Instituto de Pesquisa Econômica Aplicada – IPEA (www.ipea.gov.br),
 1964
            que possui muitos indicadores e dados referentes à área econômica e financeira.




            Iniciou-se a formação de grupos de pesquisa em probabilidades, propiciando
 1970
            um dos grandes passos para a criação de outros cursos nessa área.




            Registram-se 25 universidades em todo o país com cursos de graduação e
 1992
            pós-graduação em Estatística.




            Podemos notar um aumento significativo na oferta de cursos. Existem 56
 2020       cursos de graduação, 115 pós-graduações Lato Sensu e nove cursos de
            mestrado e doutorado em Estatística no Brasil.




Como podemos observar os conceitos da Estatística estão presentes na vida do homem
desde a Antiguidade, no entanto, atualmente a utilização da Estatística expandiu-se para
muito além de suas origens. Indivíduos e organizações usam a estatística para compreen-
der dados e tomar decisões bem informadas em ciências biológicas, ciências exatas, ciên-
cias sociais, ciências humanas, em negócios e em inúmeras áreas do conhecimento. Com
o surgimento dos computadores e de softwares estatísticos específicos para trabalhar
com dados, o estudo ficou muito mais dinâmico e interessante, de forma exata e precisa.




                                             12

## Página 13

Conceitos fundamentais

Agora, vamos conhecer alguns conceitos fundamentais da Estatística.


1. Variáveis


É a característica que será observada em uma pesquisa.



           Exemplo


       Sexo, idade, massa corporal, grau de instrução.




2. Dados


São as observações coletadas a respeito de uma determinada variável.



           Exemplo


       Feminino, masculino, 20 anos, 65 Kg, superior completo.




3. População e Amostra


Em alguns casos seria impossível entrevistar todos os elementos para coletarmos os
dados, pois levaria muito tempo para concluir o trabalho ou até mesmo seria inviável fi-
nanceiramente. Dessa forma, com técnicas robustas de amostragem, escolhemos uma
parte significativa para fazer a coleta desses dados.


Você já imaginou se, ao fazer uma pesquisa eleitoral em nosso país, ti-
véssemos que entrevistar todos os eleitores do Brasil?

Quando fazemos a coleta dos dados em situações em que todos os elementos são ana-
lisados, estamos trabalhando com uma “população”.




                                             13

## Página 14

Quando fazemos a coleta dos dados em situações em que uma parte ou subgrupo é
considerado, estamos trabalhando com uma “amostra”.


Entenda melhor esses conceitos.




        População:
        É o conjunto de todos os elementos sobre o qual desejamos obter informação.

        Amostra:
        É um subconjunto de elementos retirados da população para obter-se a infor-
        mação desejada.




Ao referirmo-nos a uma população fazemos um recenseamento e o dado estatístico é con-
siderado parâmetro. No caso de uma amostra, fazemos uma pesquisa por amostragem
e o dado estatístico é denominado estatística, como podemos observar na figura a seguir.




                   Amostra               Amostragem         Estatística




                  População                Censo            Parâmetro




         Ampliando o foco


                              O Censo é uma pesquisa realizada no Brasil pelo IBGE,
                              em que todos os domicílios brasileiros são visitados
                              para que se possa saber o número de brasileiros,
                              quem são, onde vivem e como vivem.


                              Para saber mais sobre o recenseamento visite o Portal
                              do IBGE.




                                            14

## Página 15

Ao realizar uma pesquisa com amostras é importante ter muito cuidado com a repre-
sentatividade delas. É importante assegurar que a parte escolhida da população seja
suficientemente adequada para estimar o comportamento do todo.


Podemos minimizar a possibilidade de erro melhorando a qualidade dos instrumentos de
pesquisa, determinando criteriosamente o tamanho da amostra e melhorando a habilida-
de dos pesquisadores.


No planejamento da pesquisa é essencial estudar as técnicas de amostragens. São elas:


     1. Amostragens probabilísticas: aleatória simples, sistemática, estratificada, por
     conglomerado e múltipla.


     2. Não probabilísticas: inacessibilidade, a esmo, por material contínuo, intencionais,
     por voluntários, por quotas, bola de neve.


Ainda é cedo para apresentarmos as técnicas de determinação do tamanho da amostra,
mas, ao final desse curso, você já terá condições de compreender as fórmulas utilizadas
para o cálculo do tamanho de uma amostra representativa.


O uso inadequado da estatística

O uso inadequado da estatística induz a erro. Isso ocorre quando:


    • Utilizamos amostras tendenciosas ou de tamanho pequeno.
    • Há uso ambíguo de conceitos.
    • Há apresentação equivocada de variáveis.
    • Há erros na representação gráfica (intervalos em escalas desiguais).
    • Há omissão de dados reveladores.


Veja algumas dessas situações a seguir.


Situação 1


Afirmativa: Cerca de 68% dos entrevistados defendem a pena de morte como solução
para a violência no país. Isso equivale a mais da metade dos brasileiros.


Pensamento correto: Se não for informado o número de entrevistados isso pode ser fal-
so. Suponhamos que apenas 3 pessoas deram essa opinião, em que 2 foram favoráveis
e 1 contra. O tamanho da amostra não foi suficiente para ser representativa.

                                            15

## Página 16

Situação 2


Afirmativa: Melhoramos o nosso serviço de delivery em 100%.


Pensamento correto: Se o serviço de entrega era péssimo, melhorar em 100% significa
que ele ficou pior.


Situação 3


Afirmativa: O quadro de funcionários de nossa empresa é composto de 100 funcionários
com o salário médio de R$ 5.000,00.


Pensamento correto: Essa informação não é suficiente para saber se nessa empresa
todos recebem bons salários. Suponhamos que o presidente da empresa receba R$
401.000,00 e 99 funcionários recebam R$ 1.000,00.


Situação 4


Afirmativa: A pesquisa revelou que 9 em cada 10 pessoas preferem o nosso produto.


Pensamento correto: Essa pesquisa pode ter sido feita baseada em amostras de 10 pes-
soas que já tinham afinidade com a marca e foram especialmente selecionadas para dar
esse resultado.


Tipos de erros

    1. Erros amostrais: diferença entre o resultado da amostra e o verdadeiro valor da
    população. Ocorre quando as amostras são aleatórias.


    2. Erros não amostrais: ocorrem quando os dados amostrais são coletados incor-
    retamente, devido a uma amostra tendenciosa, instrumento de medida defeituoso,
    anotações erradas, erro do pesquisador. Não devem existir ou devem ser minimiza-
    dos com cuidado, atenção e planejamento.




                                         16

## Página 17

Tipos de variáveis

Observe o esquema a seguir, que ilustra os tipos de variáveis de maneira geral.




                                                                      Nominal

                                   Qualitativa
                                   (atributo)

                                                                      Ordinal

          Variável

                                                                      Discreta

                                  Quantitativa
                                   (números)

                                                                     Contínua




Agora, com base na figura anterior, vamos apresentar cada uma delas. Preste atenção!


Qualitativas

A resposta a ser anotada é uma qualidade ou atributo, ou seja, será anotada com uma
palavra ou um código numérico.


As variáveis qualitativas podem dividir-se em duas categorias:


    • Qualitativa nominal: não existe uma hierarquia ou uma ordenação nas possíveis
    respostas.
      Exemplo: profissão, sexo, estado civil, CPF, RG, senha.


    • Qualitativa ordinal: existe certa ordem nas possíveis respostas.
      Exemplo: grau de instrução, classe social.




                                            17

## Página 18

Quantitativas

A resposta a ser anotada é uma medição ou contagem, ou seja, será anotada com
um número.


As variáveis quantitativas podem dividir-se em duas categorias:


    • Quantitativa discreta: os números a serem anotados são inteiros.
      Exemplo: número de filhos, idade.


    • Quantitativa contínua: os números a serem anotados são reais.
      Exemplo: massa corporal, estatura.


Vamos a um exemplo prático.


Exemplo
O trecho a seguir foi extraído da página do IBOPE na internet.




           Inquérito domiciliar para monitorar a soroprevalência da infecção pelo
                 vírus SARS-CoV-2 em adultos no município de São Paulo

        Estudo transversal com amostragem probabilística realizado no Município de
        São Paulo entre os dias 15 e 24 de junho de 2020 (16 semanas após o primeiro
        caso registrado na cidade).

        Para medir a soroprevalência no Município de São Paulo foram analisadas
        1.183 coletas de sangue dos participantes em 115 setores censitários sendo
        12 residências foram sorteadas em cada setor censitário.

        Sumário da Metodologia: o Município de São Paulo tem uma população de
        8.407.202 habitantes com 18 anos ou mais. Foram criados dois estratos na
        cidade: distritos com maior renda e distritos com menor renda, sendo que
        cada um deles corresponde a cerca de metade da população pesquisada.
        Após responderem um questionário, uma amostra de sangue foi colhida por
        punção venosa dos participantes. A quantidade de anticorpos anti-SARS-
        -CoV-2 (IgG e IgM) foi medida usando um método de quimiluminescência.




                                            18

## Página 19

        Tabulação dos dados coletados:




                               Fonte: ibopeinteligencia.com




Observe a metodologia da pesquisa e poderemos verificar que:


População e Amostra
População: 8.407.202 habitantes com 18 anos ou mais do município de São Paulo.
Amostra: 1.183 indivíduos com 18 anos ou mais moradores no município de São Paulo.


Variáveis pesquisadas, classificando-as como qualitativas nominais, qualitativas or-
dinais, quantitativas discretas, quantitativas contínuas
Sexo: qualitativa nominal.
Idade: quantitativa discreta.



                                           19

## Página 20

Escolaridade: qualitativa ordinal.
Raça/cor: qualitativa nominal.
Nº de pessoas no domicílio: quantitativa discreta.


Considerações gerais:


    • As diferentes prevalências de anticorpos anti-SARS-CoV-2 refletem a desigualda-
    de social do município de São Paulo.
    • A soroprevalência diminui com o aumento do nível educacional, sendo 4,5 vezes
    maior entre os indivíduos que não completaram o ensino fundamental quando com-
    parada com os que terminaram o ensino superior (22,9% versus 5,1%),
    • A soroprevalência é 2,5 vezes maior entre os participantes que se identificam
    como pretos do que nos brancos (19,7% versus 7,9%),
    • Participantes que vivem em habitações com 5 ou mais indivíduos apresentam
    uma soroprevalência quase 2 vezes maior do que aqueles que habitam com 1 ou
    dois indivíduos (15,8% versus 8,1%),


Exemplo 2
Observe a tabela com as informações e veja o posicionamento das variáveis envolvidas
no esquema apresentado, de acordo com a classificação a que ela pertence.


Informações sobre nome, estado civil, grau de instrução, número de filhos, salário, idade,
procedência, altura e massa corporal de 10 funcionários da Universidade Veiga de Almeida.


            Estado       Grau de     Nº de      Salário       Idade                   Altura Massa
  Nome                                                                Procedência
             Civil      Instrução    filhos       R$         (anos)                    (m)    (kg)


                        Superior
 Eduardo   Casado                       3      12.534,72       30          capital    1,87    85
                        completo


                        Superior
 Cláudia   Casada                       2      18.145,78       56          interior   1,65    62
                        completo
                         Ensino
  Carlos   Solteiro                     0      3.456,78        18          capital    1,91    95
                         médio
                        Superior
  Paola    Solteira                     0      4.567,89        20          Interior   1,68    55
                      Incompleto
                        Superior
  Maria    Casada                       1      6.789,10        48          capital    1,60    65
                        completo


                          Fonte: Elaborada pela autora. Dados fictícios.



                                               20

## Página 21

  • Nome
  • Estado civil                                                               • Grau de instrução
                                 Qualitativa            Qualitativa
  • Procedência                   nominal                ordinal




                                                                                            • Salário
  • Número de filhos            Quantitativa           Quantitativa
                                 discreta               contínua                            • Altura
  • Idade
                                                                                            • Massa




Exemplo 3
Observe possíveis respostas de uma pesquisa sobre os tipos de amostragens, tanto pro-
babilísticas quanto não probabilísticas, com um exemplo para cada tipo pesquisado.


      Tipo          Amostragem                 Descrição                         Exemplos

                                                                       Consideremos uma população
                                                                       de 1.000 elementos. Assim, po-
                                                                       demos numerá-la de 000 a 999.
                                       Processo mais elementar
                                                                         Em seguida, escolhemos um
                                       e muito utilizado. Equivale
                                                                      ponto de uma tabela de números
                                        a um sorteio. Numera-se
                                                                      aleatórios e agrupamos de 3 em
                        Aleatória        a população de 1 a n e,
                                                                      3; assim, vamos selecionando os
                         Simples       em seguida, realiza-se um
                                                                           elementos da amostra.
                       (ou casual)       sorteio aleatório de ele-
                                       mentos dessa numeração
                                                                      Considerando a hipotética linha
                                        que serão os elementos
                                                                        123456789012345678901,
                                        pertencentes à amostra.
                                                                      selecionaríamos os elementos:
  Amostragens                                                          123 – 456 – 789 – 012 – 345
 Probabilísticas                                                               -678 – 901.


                                                                         Seja uma população formada
                                                                       por 100 estudantes, sendo 40%
                                       Classifica a população em
                                                                       do sexo feminino e 60% do sexo
                                        pelo menos duas subpo-
                                                                       masculino. Resolve-se selecionar
                                       pulações (ou estratos) que
                     Estratificada                                    uma amostra de 10 elementos. De
                                       possuem as mesmas ca-
                   (ou proporcional)                                    acordo com essa técnica deve-
                                        racterísticas, para, assim,
                                                                       mos manter a mesma proporção
                                       extrair proporcionalmente
                                                                        de representatividade de cada
                                       amostras de cada estrato.
                                                                       grupo, portanto a amostra deve
                                                                        conter 4 mulheres e 6 homens.




                                                  21

## Página 22

                                                                 Deseja-se obter uma amostra
                                 Quando a população em           composta de 80 famílias resi-
                                    estudo já encontra-se      dentes em determinada rua, que
                                 ordenada de acordo com         conta com 2.000 casas. Assim,
                                 algum critério específico,     nosso intervalo seria dado por
                                 como fichários, prontuá-     2.000/80, que resulta em 25. Con-
                                   rios, listas, calculamos    siderando os critérios aleatórios,
                                 um intervalo de amostra-       devemos escolher um número
                   Sistemática   gem, também conhecido          entre as casas 0 e 25 e, a partir
                                 com intervalo de seleção,       daí, contar de 25 em 25 casas
                                    que definirá quais ele-    para inclusão na amostra. Consi-
                                  mentos participarão da       derando que o número sorteado
                                  pesquisa. O cálculo dele      foi 5, selecionaremos a família
 Amostragens                     é dado pelo tamanho da         da referida casa para participar
Probabilísticas                   população dividido pelo        da amostra, e depois aqueles
                                    tamanho da amostra.       residentes nas casas 5, 30, 55, 80,
                                                                 105, até atingirmos 80 casas.


                                   Divide-se a área da po-
                                  pulação em seções (ou
                                  conglomerados), depois      Ex.: pesquisa pré-eleitoral. Selecio-
                                  selecionam-se aleatoria-    nam-se aleatoriamente 30 zonas
                       Por
                                  mente algumas dessas         eleitorais e realiza-se a pesquisa
                  conglomerado
                                    seções e, finalmente,        com todos os elementos das
                                  consideram-se todos os              zonas selecionadas.
                                 elementos dos conglome-
                                    rados selecionados.



                                 Os indivíduos escolhidos
                                   nessa pesquisa estão
                    Acidental    prontamente disponíveis          Pesquisas de opinião em
                    ou casual    e não porque foram sele-            supermercados.
                                 cionados por meio de um
                                    critério estatístico.



                                   Intencionalmente é
 Amostragens                                                    Para um estudo sobre auto-
                                 escolhido o grupo de ele-
     não           Intencional                                 móveis, o pesquisador procura
                                 mentos que irá compor a
probabilísticas                                                      apenas oficinas.
                                         amostra.



                                 Assemelha-se ao método
                                  de amostragem estrati-        Deseja-se entrevistar apenas
                                  ficada, diferindo apenas    indivíduos da classe A, que repre-
                   Por quotas
                                    pelo fato de a escolha     sentam 12% da população. Esta
                                   não seguir critérios de      será a quota para o trabalho.
                                        aleatoriedade.




                                           22

## Página 23

                                                                 Ao entrevistar imigrantes de um
                                                                  país específico, pode-se iniciar
                                                                buscando pessoas conhecidas e
                                     Cada indivíduo entrevis-    pedir novos contatos de conter-
  Amostragens
                                    tado indica uma ou mais      râneos conhecidos delas. Esse
      não          Bola de neve
                                     pessoas para a próxima        processo continua até que o
 probabilísticas
                                           entrevista.           pesquisador tenha todas as en-
                                                                trevistas de que necessita ou até
                                                                 que todos os contatos tenham
                                                                          sido atingidos.




Indicadores e Índices

O indicador é uma medida quantitativa que auxilia na tomada de decisões. É obtido re-
lacionando-se as variáveis. Muito usado para avaliar tendências e performances. É um
excelente feedback, já que permite demonstrar, de forma clara e objetiva, o desempenho
de um processo em um determinado intervalo de tempo.


Índice é o valor numérico assumido por um indicador em determinado momento.



         Exemplo


       Para entender melhor esses conceitos vamos supor que uma organização utili-
       ze como indicador o número de colaboradores com nível superior completo di-
       vidido pelo número total de colaboradores. Se a empresa apurar esse indicador
       em um determinado período e alcançar 0,5 ou 50% esse será o seu índice de
       escolaridade de nível superior dos funcionários.




Os indicadores e índices são muito usados nas áreas de sustentabilidade, meio ambiente,
economia, entre outras.




                                             23

## Página 24

Observe a tabela a seguir com os indicadores e índices mais usados.


        Indicadores mais utilizados                          Índices mais utilizados


                                                    IDH – Índice de desenvolvimento Hu-
                                                    mano - Proposto pela ONU, calculado no
                                                    nosso país a partir dos dados censitários
  PIB – Produto Interno Bruto - É um dos            do IBGE. Calculado como sendo a média
  indicadores mais utilizados em macroe-            geométrica de três índices, educação,
  conomia. Seu objetivo é quantificar a ati-        renda e saúde. É mais representativo do
  vidade econômica de uma região.                   que o PIB, já que este só se importa com
                                                    a economia, sem preocupação com as
                                                    condições da população; por isso, pode
                                                    mascarar a desigualdade existente.


  Tabela Fipe - Expressa preços médios de
                                                    IBOVESPA – Índice da Bolsa de Valores
  veículos anunciados por vendedores, no
                                                    de São Paulo – É um índice de desempe-
  mercado nacional, servindo como parâ-
                                                    nho das ações negociadas na Bovespa.
  metro para negociações.


                                                    IPC - Índice de Preços ao Consumidor,
  IQA - Índice de qualidade do ar - É um in-
                                                    apurado pela FGV (Fundação Getúlio Var-
  dicador padronizado do nível de poluição
                                                    gas). É um índice para avaliação do poder
  do ar em uma determinada região.
                                                    de compra do consumidor.


                                                    IPCA – Índice de Preços ao Consumidor
                                                    Amplo - Medido mensalmente pelo IBGE
  Indicador social - Usado para informar
                                                    (Instituto Brasileiro de Geografia e Estatís-
  algo sobre determinado aspecto da reali-
                                                    tica), foi criado com o objetivo de oferecer
  dade social, para fins de pesquisa ou ava-
                                                    a variação dos preços no comércio para o
  liação de políticas públicas.
                                                    público final. O IPCA é considerado o índi-
                                                    ce oficial de inflação do país.

                                                    IGP-M – Índice Geral de Preços do Mer-
                                                    cado - Medido pela FGV (Fundação Ge-
                                                    túlio Vargas), é um indicador de variação
                                                    dos preços na economia brasileira. Como
                                                    o Brasil é um país fortemente impactado
                                                    pela inflação, ele serve para verificar as
                                                    mudanças quanto ao valor da moeda e
                                                    nas alterações de preços.




                                               24

## Página 25

                                                  INCC - Índice Nacional de Custo da
                                                  Construção - Mede a variação de preços
                                                  que faz parte do setor da construção civil
                                                  na economia brasileira.




          Ampliando o foco


        Tenha em mente que tanto os indicadores quanto os índices são de grande utili-
        dade para o planejamento estratégico de uma empresa e também para o nosso
        dia a dia. Neste material apresentaremos apenas alguns desses indicadores e
        índices, contudo existe uma variedade deles.


        Para saber mais é importante que você pesquise sobre indicadores e índices
        ligados à sua área de interesse.




Divisões da Estatística

Observe o esquema a seguir para conhecer as divisões da Estatística.


                                            Ferramentas para coleta, organização, apresenta-
                        Estatística          ção, análise, interpretação dos dados, bem como
                        descritiva          para o cálculo de medidas de posição e dispersão.


                                              Análise de situações, que lidam com incertezas
                       Probabilidade
                                                              ou com o acaso.

  Estatística
                                             Ferramentas usadas para estimar, induzir ou inferir
                        Estatística         leis de comportamento de uma população, basean-
                         indutiva           do-se em resultados obtidos em uma pesquisa feita
                                                com uma amostra retirada dessa população.


                        Estatística          A partir da segunda metade do século XX teve um
                      computacional                forte impacto na prática da estatística.




                                             25

## Página 26

Softwares estatísticos

Muitos softwares fazem integração entre informática e estatística. Contudo, neste mate-
rial listaremos os softwares mais usados. Veja a seguir uma breve descrição sobre cada
um deles.


1. SPSS – Software comercial de análises estatísticas. Fornece um conjunto robusto de
recursos que permitem extrair informações dos dados. Possui versão demo.


2. MINITAB – Software comercial de análises estatísticas. Possui um ambiente comple-
to para a análise de dados. Sua interface intuitiva permite que o usuário trabalhe, simulta-
neamente, com planilhas eletrônicas de dados, tabelas estatísticas, gráficos e textos em
forma de “janelas”. Possui versão demo.


3. R for Windows – É um dos softwares estatísticos mais usados no mundo. Gratuito.
Disponível para Windows e Unix.


4. STATISTICA – Software comercial de análises estatísticas. Possui versão demo.


5. Excel – Na planilha eletrônica da Microsoft encontramos várias funções estatísticas,
de fácil utilização e amplamente comercializadas.


6. Bioestat – Software para estudantes de fácil uso para iniciantes, voltado sobretudo
para a área de Ciências Biológicas, mas pode ser usado em qualquer área. É gratuito e
tem versão em português.




                                      MIDIATECA

        Na midiateca você encontra os links com os endereços para fazer o download
        da versão “demo” dos softwares comerciais.




                                            26

## Página 27

Elaboração e análise de gráficos e tabelas

Um dos objetivos da Estatística é fazer a síntese e a organização dos valores que a variável
de estudo pode assumir. Para alcançar esse propósito lançamos mão das tabelas e dos
gráficos que viabilizarão informações rápidas e seguras a respeito da variável em estudo.


Neste tópico abordaremos como fazer a coleta dos dados, a organização em tabelas de
distribuição de frequências e algumas representações gráficas.


Coleta dos dados

Dentre diversas maneiras para se coletarem dados, a amostragem é a mais frequente.
Pode ser feita a partir da aplicação de um questionário, que deverá ser respondido pelo
entrevistado fisicamente, por meio do telefone ou, ainda, pela internet. Podem também
ser obtidos por meio de gravação de entrevistas ou por um levantamento dos dados que
já tenham sido disponibilizados por outro pesquisador.


São importantes alguns cuidados para coletar os dados. São eles:


     • Identificação: o entrevistado só deve ser identificado nos casos em que realmen-
     te seja necessário, pois sem identificação ele ficará mais à vontade para responder
     às questões propostas.
     • Palavras adequadas: a comunicação deve ser feita de acordo com o público-alvo,
     tomando-se o cuidado com palavras muito rebuscadas ou termos vulgares.
     • Perguntas impróprias: somente perguntas relevantes devem ser feitas ao entre-
     vistado.
     • Perguntas com duplo sentido: averiguar se o que foi perguntado está sendo
     compreendido.
     • Perguntas sem sentido estatístico: evitar perguntas que não serão necessárias
     ao estudo. Questionários muito longos cansam o entrevistado e ele tende a não res-
     ponder ou a responder de qualquer maneira.
     • Testar antes de aplicar: faça um projeto piloto antes de sair para sua entrevista.
     Aplique o questionário a uma pequena quantidade de pessoas e certifique-se de que
     realmente o que você está perguntando está sendo bem compreendido. Só depois
     desse teste é que o pesquisador deve ir a campo.




                                             27

## Página 28

Organização dos dados coletados em tabelas

A elaboração de tabelas obedece à Resolução nº 886, de 26 de outubro de 1966, do
Conselho Nacional de Estatística, sendo que, a partir de 1993, o IBGE também passou a
normatizar a apresentação de tabelas.


E quais elementos uma tabela deve conter?

Observe os elementos a seguir. São eles:


    1. Título: conjunto de informações, as mais completas possíveis, respondendo às
    seguintes perguntas:


       • O quê (referente ao fato).
       • Onde (relativo ao lugar).
       • Quando (corresponde à época).


    2. Cabeçalho: parte superior da tabela que especifica o conteúdo das colunas.


    3. Coluna indicadora: parte da tabela que especifica o conteúdo das linhas.


    4. Classes: são as linhas do corpo da tabela.


    5. Corpo: conjunto de linhas e colunas que contêm informações sobre a variável
    em estudo.


    6. Casa ou célula: espaço destinado a um só número.


    7. Fonte e notas: colocadas, preferencialmente, no rodapé.




                                           28

## Página 29

                           Exemplo de uma tabela com os elementos.


                 Os 10 números mais sorteados na Mega-sena desde o concurso            Título
                  nº 1 (de 11/03/1996) até o concurso nº 2281 (de 18/07/2020)

                                                      Quantidade de vezes
                            Número                                                 Cabeçalho
                                                        que foi sorteado
    1ª Classe                 53                              261        Casa ou
                              10                              260        célula
                              23                              254
                               5                              254
                              33                              248                     Corpo
                               4                              248
                              27                              245
                              54                              244
    9ª Classe                 24                              244
                              42                              244

                             Total                            2.502
                  Fonte: https://www.somatematica.com.br/megasenaFrequentes.php      Rodapé




Considerações:


- A tabela não é delimitada à esquerda e nem à direita (deve ficar aberta lateralmente).
- A tabela é limitada superiormente e inferiormente por uma linha grossa ou dupla.
- Os elementos “fonte” e “título” são obrigatórios.
- Devemos colocar um traço horizontal (–) quando o valor é zero.
- Quando não temos os dados sinalizamos com três pontos (...).
- Um ponto de interrogação (?) deve ser usado quando temos dúvida quando à exatidão
de determinado valor.
- O zero (0) deve ser usado quando o valor é muito pequeno para ser expresso pela uni-
dade utilizada.


Elaboração de uma tabela de distribuição de frequências

Uma tabela de distribuição de frequências é uma tabela-resumo que mostra classes,
frequências absolutas, frequências relativas e frequências acumuladas.


    • Dados brutos: são os dados originais na forma como foram coletados, sem ne-
    nhuma organização.


    • Rol estatístico: é a organização dos dados brutos em ordem crescente, decres-
    cente ou alfabética.


                                               29

## Página 30

      • Frequência absoluta ou frequência simples ou simplesmente frequência: é a
      quantidade de vezes que cada valor é observado.


      • Frequências relativas: obtidas pela divisão entre a frequência e o total ou pela
      frequência acumulada e o total. Geralmente são expressas em %.


      • Frequências acumulada: é a soma de cada frequência com as anteriores ou a
      soma de cada frequência relativa com as anteriores.


Vamos a um exemplo prático.


Exemplo
O SINCOM realizou uma pesquisa sobre os preços de um modelo de computador em 20
lojas de informática, na cidade de Juiz de Fora, em julho de 2020. Foram coletados os
valores a seguir em reais:



         2000    2500   2000   2600   2000      2600   2600   2500   2500   2000
         2000    2000   2500   2600   2600      2600   2600   2600   2600   2600



Observe como organizam-se os dados brutos em uma tabela de distribuição de frequências.


1º passo: confeccionar o rol, ou seja, precisamos organizar os dados brutos em ordem
crescente ou decrescente.



         2000    2000   2000   2000    2000     2000   2500   2500   2500 2500
         2600    2600   2600   2600    2600     2600   2600   2600   2600 2600



2º Passo: com o rol confeccionado iniciamos a contagem para preencher a coluna da
frequência absoluta:


Rol



         2000    2000   2000   2000    2000     2000   2500   2500   2500 2500
         2600    2600   2600   2600    2600     2600   2600   2600   2600 2600




                                           30

## Página 31

           Tabela: Preço do microcomputador modelo X, em 20 lojas de JF – Julho 2020.



            Frequência          Frequência             Frequência              Frequência
   Preço     absoluta             relativa             acumulada           relativa acumulada
                (fi)               (fri%)                  (Fi)                    (Fri%)

   2000           6

   2500           4

   2600          10

   Total         20



                                     Fonte: SINCOM (2020).




A frequência relativa será obtida dividindo-se a frequência absoluta pelo total. Para ex-
pressarmos o resultado em porcentagem temos que multiplicar o resultado por 100.



           Tabela: Preço do microcomputador modelo X, em 20 lojas de JF – Julho 2020.



             Frequência         Frequência             Frequência              Frequência
   Preço      absoluta            relativa             acumulada           relativa acumulada
                 (fi)              (fri%)                  (Fi)                    (Fri%)

   2000           6         6/20 = 0,30 ou 30%

   2500           4          4/20=0,2 ou 20%

   2600          10         10/20 = 0,5 = 50%

   Total         20                100%



                                     Fonte: SINCOM (2020).




A frequência acumulada será a soma de todas as frequências absolutas que estão na
classe (linha) ou acima dela. Veja a tabela a seguir:




                                               31

## Página 32

           Tabela: Preço do microcomputador modelo X, em 20 lojas de JF – Julho 2020.



             Frequência         Frequência             Frequência              Frequência
  Preço       absoluta            relativa             acumulada           relativa acumulada
                 (fi)              (fri%)                  (Fi)                    (Fri%)

   2000           6         6/20 = 0,30 ou 30%               6

   2500           4          4/20=0,2 ou 20%            6 + 4 = 10

   2600          10         10/20 = 0,5 = 50%         6 + 4 + 10 = 20

   Total         20                100%




                                     Fonte: SINCOM (2020).




A frequência acumulada relativa será a soma de todas as frequências relativas que estão
na classe(linha) ou acima dela:



           Tabela: Preço do microcomputador modelo X, em 20 lojas de JF – Julho 2020.



             Frequência         Frequência             Frequência              Frequência
   Preço      absoluta            relativa             acumulada           relativa acumulada
                 (fi)              (fri%)                  (Fi)                   (Fri%)

   2000           6         6/20 = 0,30 ou 30%               6                     30%

   2500           4          4/20=0,2 ou 20%            6 + 4 = 10          30% + 20% = 50%

   2600          10         10/20 = 0,5 = 50%         6 + 4 + 10 = 20       50% + 50% = 100%

   Total         20                100%



                                     Fonte: SINCOM (2020).




                                               32

## Página 33

Tabela de distribuição de frequência COM intervalos de
classe

Quando os dados obtidos assumem muitos valores diferentes, é costume, em vez de lis-
tar as respostas uma a uma, apresentar as tabelas em intervalos, chamados de classes.


Veja um exemplo de uma tabela de distribuição de frequências com intervalo de classe:



                  Tabela: População feminina residente no Brasil por idade, em 2019.


                                                                                       Frequência
       Idade              Frequência          Frequência         Frequência
                                                                                       Acumulada
      (anos)               Absoluta            Relativa          Acumulada
                                                                                        Relativa

      0 I---- 4              4991                4,6%                4991                4,6%

      4 I---- 6              2638                2,4%                7629                7,0%

      6 I--- 10              5516                5,1%               13145                12,1%

     10 I--- 15              7167                6,6%               20312                18,7%

     15 I--- 18              4624                4,3%               24936                23,0%

     18 I--- 25             11063                10,2%              35999                33,2%

     25 I--- 30              7674                7,0%               43673                40,2%

     30 I--- 40             17011                15,7%              60684                55,9%

     40 I--- 60             29168                26,9%              89852                82,8%

  60 anos ou mais           18658                17,2%              108510              100,0%

       Total                108510              100,0%



         Fonte: IBGE - Pesquisa Nacional por Amostra de Domicílios Contínua anual (2020).




                                                 33

## Página 34

Elementos de uma distribuição de frequências com intervalos de classe:
Uma distribuição de frequência com intervalos de classe apresenta alguns elementos.
São eles:


                              Limites               Intervalo         Amplitude
        Classe
                            das classes            das classes        amostral


       Amplitude de um                 Ponto médio                Frequências
      intervalo de classe             de uma classe              de cada classe



Agora vamos conhecer mais um pouco sobre eles.


1. Classes


São os intervalos da variável quantitativa.


Exemplo:


    Na tabela 2, temos:


    1ª classe: 0 I---- 4
    2ª classe: 4 I---- 6
    3ª classe: 6 I---- 10
    4ª classe: 10 I---- 15
    5ª classe: 15 I---- 18
    6ª classe: 18 I---- 25
    7ª classe: 25 I---- 30
    8ª classe: 30 I---- 40
    9ª classe: 40 I---- 60
    10ª classe: 60 anos ou mais


2. Como calcular o número de classes: (k)


Temos três formas que podem ser usadas para calcular o número de classes de uma
tabela de distribuição de frequências em função da quantidade de dados (n):


    Sejam:
    k = número de classes
    n = quantidade de dados


                                              34

## Página 35

                                           ~
   • Primeira forma: Fórmula de Sturges: k = 1 + 3,3 · log n


   • Segunda forma: k = n


   • Terceira forma: o bom senso do pesquisador. A familiaridade do pesquisador com
   os dados é que deve indicar quantas classes devem ser construídas. As fórmulas são
   apenas sugestões. Cabe ao pesquisador analisar o resultado e tomar a decisão de
   quantas classes deverão ser adotadas. A orientação é que o número de classes deve
   estar entre 5 e 20. Tabelas com poucas classes perdem a precisão dos dados e com
   muitas classes ficam cansativas.


3. Limites da classe


São os valores extremos de cada classe. O menor número é o limite inferior da classe (Li)
e o maior número, o limite superior (LS) da classe.


Exemplo:


   Na tabela 2 temos:
   Na primeira classe: Li = 0 LS = 4
   Na quinta classe: Li = 15 LS = 18
   Na décima classe: Li = 60 LS = -


4. Tipos de intervalos e representações mais usadas


   a) Fechado à esquerda e fechado à direita: I----I ou -------- ou [ , ]
   b) Fechado à esquerda e aberto à direita: : I---- ou -------- ou [ , [
   c) Aberto à esquerda e fechado à direita: : ----I ou --------  ou ] , ]
   d) Aberto à esquerda e aberto à direita: ---- ou --------     ou ] , [


É muito importante entender essa simbologia para determinar corretamente as frequên-
cias absolutas.


Veja o exemplo a seguir.




                                           35

## Página 36

Exemplo
1º) Considere o seguinte levantamento sobre o número de salários mínimos recebidos
pelos funcionários de uma empresa:


                       Funcionário                          Nº de S.M

                 Funcionário 1                                    1

                 Funcionário 2                                    4

                 Funcionário 3                                    4,2

                 Funcionário 4                                    3,9

                 Funcionário 5                                    5

                 Funcionário 6                                    2,5

                 Funcionário 7 (voluntário)                       0

                 Funcionário 8                                    3

                 Funcionário 9                                    4,1


2º) Organize os dados em ordem crescente, ou seja, confeccione o ROL:


                              0; 1; 2,5; 3; 3,9; 4; 4,1; 4,2; 5


3º) Suponha que vamos preencher a coluna da frequência absoluta da seguinte tabela
de distribuição:


      Nº de          Frequência
                                                              Explicação
     salários         Absoluta


                                          Como o intervalo é fechado à esquerda, o limite
                                           inferior será considerado e, como o intervalo é
                                          fechado à direita, o limite superior também deve
     0 I----I5
                                                           ser considerado.
         ou
    0 ---- 5              9
                                              Então, fazendo a contagem no rol temos:
        ou
                                                    0; 1; 2,5; 3; 3,9; 4; 4,1; 4,2; 5
      [0 , 5]

                                           Portanto, 9 funcionários recebem de zero até,
                                                inclusive, cinco salários mínimos.




                                              36

## Página 37

4º) Modifique o intervalo e faça a contagem da frequência absoluta:


      Nº de           Frequência
                                                          Explicação
     salários          Absoluta


                                        Como o intervalo é fechado à esquerda, o limite
                                        inferior será considerado e, como o intervalo é
                                        aberto à direita, o limite superior NÃO deve ser
     0 I----5
                                                          considerado.
        ou
    0 ---- 5              8
                                           Então, fazendo a contagem no rol temos:
        ou
                                                   0; 1; 2,5; 3; 3,9; 4; 4,1; 4,2
      [0,5[

                                         Portanto, 8 funcionários recebem de zero até
                                                 antes de 5 salários mínimos.




5º) Modifique, novamente o intervalo e faça a contagem:


      Nº de           Frequência
                                                          Explicação
     salários          Absoluta


                                         Como o intervalo é aberto à esquerda, o limite
                                       inferior NÃO será considerado e, como o interva-
                                        lo é fechado à direita, o limite superior deve ser
     0 ----I5                                             considerado.
        ou
    0 ---- 5              8                Então, fazendo a contagem no rol temos:
        ou                                         1; 2,5; 3; 3,9; 4; 4,1; 4,2; 5
      ]0 , 5]
                                       Portanto, 8 valores satisfazem essa condição, ou
                                        seja, ganham mais de 0 salários até inclusive 5
                                                       salários mínimos.




                                          37

## Página 38

Por fim, modificando novamente o intervalo e fazendo a contagem teremos:


       Nº de            Frequência
                                                              Explicação
      salários           Absoluta

                                             Como o intervalo é aberto à esquerda, o limite
                                           inferior NÃO será considerado e, como o intervalo
                                            é aberto à direita, o limite superior também NÃO
      0 ----5                                             deve ser considerado.
         ou
     0 ---- 5                7                 Então, fazendo a contagem no rol temos:
         ou                                             1; 2,5; 3; 3,9; 4; 4,1; 4,2
       ]0 , 5[
                                           Portanto, 7 funcionários satisfazem essa condi-
                                           ção, ou seja, recebem entre zero e cinco salários
                                                               mínimos.



5. Amplitude amostral: (AA)


É a diferença entre o maior valor e o menor valor do rol.


                                     AA = xmáximo – xmínimo


6. Amplitude de um intervalo de classe: (hi)
É a diferença entre o limite superior e o limite inferior da classe.


                                         hi = Lsi – Lii


Como vimos na tabela População feminina residente no Brasil por idade, em 2019, que
apresenta a população feminina residente no Brasil por idade em 2019, temos:



         Amplitude da primeira classe (0 I---- 4) → h1 = Ls1 – Li1 = 4 – 0 = 4

         Amplitude da quarta classe (10 I---- 15) → h4 = Ls4 – Li4 = 15– 10 = 5

         Amplitude da décima classe (60 anos ou mais) → h10 = Ls10 – Li10 = ???
         (não pode ser determinada, já que o limite superior não foi especificado)




                                              38

## Página 39

Quando a tabela ainda não foi criada, para calcular o intervalo de cada classe devemos
dividir a amplitude total (AT) pelo número de classes(k). Então:

                                                 AT
                                      h=
                                                  k


A amplitude de um intervalo de classe pode ser igual em toda tabela (que é preferível,
sempre que possível) ou diferente em algumas classes.



7. Ponto Médio de uma classe: (Xi)
É o valor que representa a classe, calculado como sendo a média aritmética entre os
limites da classe.

                                             Lii + Lsi
                                     Xi =
                                                 2


Na tabela que apresenta a população feminina residente no Brasil por idade em 2019,
temos:




                                                           Li1 + Ls1       0+4
       Ponto médio da primeira classe (0 I---- 4) → X1 =               =         =2
                                                              2             2


                                                           Li2 + Ls2       4+6
       Ponto médio da segunda classe (4 I---- 6) → X2 =                =         =5
                                                              2             2




Calculamos da mesma maneira para todas as classes que possuem limite inferior e limi-
te superior.


Ponto médio da décima classe (60 anos ou mais) → (não pode ser determinado, já que o
limite superior não foi especificado)




                                            39

## Página 40

8. Frequências de cada classe
As frequências absolutas, relativas e acumuladas são calculadas exatamente iguais ao
cálculo das frequências para tabelas de distribuição de frequência sem intervalos de
classe. São elas:


    • Frequência absoluta ou frequência simples ou simplesmente frequência: é a
    quantidade de vezes que cada valor é observado.


    • Frequências relativas: obtida pela divisão entre a frequência e o total ou pela fre-
    quência acumulada e o total. Geralmente são expressas em %.


    • Frequências acumulada: é a soma de cada frequência com as anteriores ou a
    soma de cada frequência relativa com as anteriores.


Vamos lembrar o cálculo das frequências relativas?

Para isso, é preciso efetuar uma divisão e o resultado deve ser arredondado.


Critério de arredondamento de dados

De acordo com a Resolução nº 886/1966 do IBGE:


I) Se o primeiro algarismo a ser abandonado for menor do que 5, ou seja quando o pri-
meiro algarismo a ser abandonado for 0,1,2,3 ou 4, ficará inalterado o último algarismo
que permanece.


Exemplo: Arredondar com 1 casa decimal os números seguintes:
53,24 passa para 53,2.
24,13 passa para 24,1.


II) Se o primeiro algarismo a ser abandonado for maior do que 5, ou seja, quando o pri-
meiro algarismo a ser abandonado é o 6,7,8, ou 9, aumenta-se em uma unidade o alga-
rismo que permanece.


Exemplos:
13,87 passa para 13,9.
24,08 passa para 24,1.
34,99 passa para 35,0.




                                           40

## Página 41

III) Quando o algarismo a ser abandonado for igual a 5 existem dois critérios e você deve
adotar o que achar mais conveniente, sem nenhum problema.


Se, após o 5 seguir em qualquer casa um algarismo diferente de zero, aumenta-se uma
unidade ao algarismo que permanece. (Esse critério é adotado por calculadoras e com-
putadores.)


Exemplos:
7,352 passa para 7,4.
45,6501 passa para 45,7.
86,250002 passa para 86,3.


Outro critério quando o dígito a ser abandonado for o 5:


Se o 5 for o último algarismo ou após o 5 só se seguirem zeros, o último algarismo a ser
conservado só será aumentando de uma unidade se for ímpar.


Exemplos:
18,75 passa para 18,8
29,65 passa para 29,6
37,75000 passa para 37,8
49,8500 passa para 49,8


Observação: Nunca devemos fazer arredondamentos de sucessivos. O arredondamento
deve ser feito apenas na resposta final.



Veja alguns exemplos práticos para entender melhor. Vamos lá!


Exemplo 1
Em julho de 2020 foi feita uma pesquisa sobre a idade de 50 funcionários da UVA. Os
dados estão apresentados a seguir:


                     31    43   44   44   45        48   45   46   47   45
                     49    50   51   53   54        54   56   58   26   65
                     18    20   20   21   22        24   25   25   62   27
                     29    34   30   30   41        31   32   33   29   35
                     36    36   37   37   40        37   38   38   38   37




                                               41

## Página 42

Como devemos construir a tabela completa de distribuição de frequências, com o intervalo
fechado à esquerda?


Resolução:
Sete passos devem ser seguidos para confeccionarmos a tabela de distribuição de fre-
quências com intervalos de classe:


1- Confeccionar o Rol Estatístico.
2- Determinar o número de classes (k).
3- Calcular a amplitude dos intervalos de classe (h).
4- Determinar o limite inferior da primeira classe (Li).
5- Determinar o limite superior da primeira classe (Ls).
6- Determinar os dos limites inferiores e superiores das demais classes.
7- Determinar a frequência absoluta de cada classe e, a partir daí, calcular as frequências
relativas e acumuladas.
1º passo: confecção do Rol.


                     18   20   20   21   22        24   25   25   26 27
                     29   29   30   30   31        31   32   33   34 35
                     36   36   37   37   37        37   38   38   38 40
                     41   43   44   44   45        45   45   46   47 48
                     49   50   51   53   54        54   56   58   62 65



2º passo: determinação do número de classes (k).

  ~
k=  1 + 3,3 · log n
  ~
k = 1 + 3,3 · log 50
  ~
k=  1 + 3,3 · 1,6989970004
  ~
k = 1 + 5,606601014
  ~
k=  6,606601014
  ~
k = 7 classes


Se tivéssemos optado pelo método da raiz quadrada de determinação das classes teríamos:


k= n
k = 50
                ~
k = 7,071067812 = 7 classes




                                              42

## Página 43

3º passo: cálculo da amplitude dos intervalos de classe (h).

          AA
  h=
           k
         65 – 18                   ~
  h=                 = 6,714285714 = 7
            7



4º passo: determinação do limite inferior da primeira classe (Li).


Normalmente, o limite inferior da primeira classe será o menor valor da amostra, ou um
número menor do que ele.


No nosso caso, adotaremos Li = 18



5º passo: determinar o limite superior da primeira classe (Ls).


Para determinar o limite superior da primeira classe basta adicionar a amplitude da clas-
se ao limite inferior, ou seja:


1ª classe: Li = 18    Ls = 18 + 7 = 25


6º passo: determinar os dos limites inferiores e superiores das demais classes.


O limite superior da primeira classe será o limite inferior da segunda classe e assim su-
cessivamente, até que o maior valor pesquisado esteja contido na última classe.


2ª classe: Li = 25   Ls = 25 + 7 = 32
3ª classe: Li = 32   Ls = 32 + 7 = 39
4ª classe: Li = 39   Ls = 39 + 7 = 46
5ª classe: Li = 46   Ls = 46 + 7 = 53
6ª classe: Li = 53   Ls = 53 + 7 = 60
7ª classe: Li = 60   Ls = 60 + 7 = 67 (67 > 65 → ok!)


7º passo: determinar a frequência absoluta de cada classe e, a partir daí, calcular as
frequências relativas e acumuladas da mesma maneira que foram calculadas para as
distribuições de frequências sem intervalos de classe.




                                            43

## Página 44

       Tabela: Idade de 50 funcionários da Universidade Veiga de Almeida em Julho de 2020.

                Idade (anos)                 fi                fri %          Fi        Fri%

           18       | ----     25             6                12%             6         12%
           25       | ----     32            10                20%            16         32%
           32       | ----     39            13                26%            29        58%
           39       | ----     46             8                16%            37         74%
           46       | ----     53             6                12%            43         86%
           53       | ----     60             5                10%            48         96%
           60       | ----     67             2                4%             50        100%

                   Total                     50                 1
                                    Fonte: Elaborada pela autora. Dados fictícios.


Interpretação da tabela
Uma amostra com 50 dados, coletados em julho de 2020, informam as idades de cola-
boradores da Universidade Veiga de Almeida – UVA.
Após a organização desses dados pode-se observar que aproximadamente 26% dos fun-
cionários apresentam idade igual ou superior a 32 anos e inferior a 39 anos. Em seguida,
com um percentual de 20%, estão os colaboradores que apresentam idade de 25 até
antes de 32 anos. Percebe-se que 4% dos empregados estão acima de 60 anos e 12%
estão abaixo de 25 anos. Mais da metade dos colaboradores, aproximadamente 58%,
têm idade abaixo de 39 anos.


Você pode perceber que outras informações ainda podem ser retiradas da tabela?


Exemplo 2
Os dados coletados pela Secretaria do Meio Ambiente e já ordenados referem-se aos
níveis de ruído em decibéis de algumas áreas residenciais da cidade do Rio de Janeiro.

   55,00           55,89            56,03         56,67         59,36      60,14     60,22     60,32
   60,74           60,96            61,49         61,89         61,92      62,57     62,69     63,14
   63,29           64,00            64,17         64,43         64,71      64,78     65,08     65,70
   65,81           66,01            66,16         66,84         70,00      71,46     71,52     72,99


Como poderíamos organizá-los em uma distribuição de frequências com intervalos de
classes?




                                                          44

## Página 45

Resposta:
                   Tabela: Níveis de ruído de algumas áreas residenciais da cidade
                                 do Rio de Janeiro, em julho de 2020.
                                                                                      Frequência
  Nível de ruido             Frequência             Frequência           Frequência
                                                                                      Acumulada
      (dB)                    absoluta               Relativa            Acumulada
                                                                                       Relativa

   55   I----    58                  4                 12,5%                   4         12,5%
   58   I----    61                  6                 18,8%                  10         31,3%
   61   I----    64                  7                 21,9%                  17         53,2%
   64   I----    67                 11                 34,3%                  28         87,5%
   67   I----    70                  0                 0,0%                   28         87,5%
   70   I----    73                  4                 12,5%                  32        100,0%

        Total                       32                100,0%

            Fonte: Secretaria do Verde e do Meio Ambiente do Rio de Janeiro (2020).


Representação gráfica de uma distribuição de frequências

As tabelas de distribuição de frequências podem ser representadas graficamente. Sua
finalidade principal é fornecer informações analíticas de maneira mais rápida e verificar o
comportamento da distribuição. Apresentaremos três tipos de representações gráficas
das distribuições: histogramas, polígonos de frequências e polígonos de frequências
acumuladas (Ogiva de Galton).


Histograma

Antes de começarmos observe a tabela a seguir.


         Tabela: Idade de 50 funcionários da Universidade Veiga de Almeida em Julho de 2020.

                Idade (anos)                 fi               fri %           Fi      Fri%

          18        | ----     25             6               12%              6      12%
          25        | ----     32            10               20%             16      32%
          32        | ----     39            13               26%             29      58%
          39        | ----     46             8               16%             37      74%
          46        | ----     53             6               12%             43      86%
          53        | ----     60             5               10%             48      96%
          60        | ----     67             2               4%              50      100%
                   Total                     50                1

                                    Fonte: Elaborada pela autora. Dados fictícios.




                                                         45

## Página 46

Para representá-la graficamente vamos utilizar um tipo de gráfico que se chama Histograma.


Para construir um polígono de frequência acumulada é necessário:


1- Traçar um eixo horizontal e marcar em escala os limites inferiores e superiores das
classes.
2- Traçar um eixo horizontal e marcar as frequências (podem ser frequência absoluta,
acumulada ou relativa).
3- Traçar um retângulo em que a base corresponde à classe e a altura seja igual à fre-
quência.
4- Colocar o título no gráfico e identificar as variáveis nos eixos.

                  Gráfico: Idade de 50 funcionários da UVA, em Jul/2020

                 fi
                14
                12
                10
                 8
                 6
                 4
                 2
                 0
                      18     25     32     39     46     53     60     67 Idade
                                                                          (anos)


                           Fonte: Elaborado pela autora. Dados fictícios.



         Ampliando o foco


       Na planilha eletrônica Excel, versão 2016, existe o comando que gera automa-
       ticamente o histograma. Basta selecionar a base de dados e clicar em Inserir
       – Gráficos -Histograma.




Polígono de frequência


Para construir um polígono de frequência é necessário:


a. Marcar os pontos no gráfico, sendo que no eixo horizontal você marca os pontos mé-
dios das classes e no eixo vertical as frequências.


                                                46

## Página 47

b. Relacionar cada ponto médio com sua respectiva frequência.
c. Marcar metade das distâncias dos pontos médios no eixo horizontal antes da primeira
classe e depois da última classe.
d. Traçar os segmentos de retas que ligam os pontos marcados.
e. Colocar o título e o nome das variáveis nos eixos.



      Idade de 50 funcionários da UVA, em Jul/2020                    Idade de 50 funcionários da UVA, em Jul/2020

 fi                                                              fi
14                                                              14
12                                                              12
10                                                              10
 8                                                               8
 6                                                               6
 4                                                               4
 2                                                               2
 0                                                               0
        18   25    32   39    46     53   60   67 Idade                 18   25    32   39    46   53    60   67 Idade
                                                  (anos)                                                         (anos)




                                   Fonte: Elaborados pela autora. Dados fictícios.




Polígono de frequência acumulada (Ogiva de Galton)

Para construir um polígono de frequência acumulada é necessário:


1- Marcar os limites das classes no eixo horizontal.
2- Marcar os pontos que correspondem aos limites superiores das classes e as respecti-
vas frequências acumuladas de cada classe.
3- Traçar os segmentos de retas que ligam os pontos marcados.
4- Ligar os pontos marcados.
5- Colocar o título e o nome das variáveis nos eixos.




                                                           47

## Página 48

           Figura: Ogiva de Galton da Idade de 50 funcionários da UVA, em julho de 2020.


                                    Idade de 50 funcionários da UVA, em Jul/2020


                Frequência
                Acumulada
                       50
                       48
                        43

                        37

                        29



                        16


                         6

                         0              18   25    32   39    46   53   60    67 Idade
                                                                                 (anos)




                             Fonte: Elaborado pela autora. Dados fictícios.



Outros tipos de gráficos

Os gráficos têm a capacidade de comunicar os dados de forma visual eficaz e atraen-
te. Os requisitos fundamentais para um bom gráfico devem ser simplicidade, clareza
e veracidade.


Existem diversos tipos de gráficos: colunas, barras, linhas ou segmentos, setores ou piz-
za, pictogramas e muitos outros tipos. Com auxílio de computadores podemos represen-
tá-los em terceira dimensão e com movimentos, o que enriquece muito a visualização.


Observe na tabela a seguir os gráficos mais utilizados.




                                                   48

## Página 49

Tipo/Características        Representação


   1. Gráfico de
      colunas

    Representação
  de uma série por
    meio de retân-
   gulos de mesma
    base e alturas
  proporcionais aos
 respectivos dados.

 Utilizado quando
 apresentamos as
  categorias com
  palavras curtas.




     2. Gráfico
     de barras

    Representação
  de uma série por
    meio de retân-
   gulos de mesma
    altura e bases
  proporcionais aos
 respectivos dados.

 Utilizado quando
 apresentamos as
  categorias com
 palavras extensas




                       49

## Página 50

    3. Colunas
   justapostas

Descreve simulta-
neamente duas ou
 mais categorias
para uma variável.




    4. Setores

Também conheci-
 do como gráfico
  de pizza. É utili-
zado sempre que
desejamos ressal-
tar a participação
 do dado no total.

 Cada setor é ob-
 tido por meio de
uma regra de três
 simples e direta,
  lembrando que
100% corresponde
      a 360º.




   5. Gráfico de
 linha ou gráfico
  de segmentos

 Marcamos todos
 os pares ordena-
dos corresponden-
tes à série estatís-
tica e os unimos a
partir de uma linha
 poligonal traceja-
  da ou contínua.




                       50

## Página 51

    6. Diagrama
    de dispersão
   (scatterplot) e
linha de tendência

O diagrama de dis-
  persão, também
  conhecido como
   scatterplot, é a
 representação do
  relacionamento
  entre duas variá-
 veis quantitativas.
 A linha de tendên-
  cia é a linha que
faz uma aproxima-
ção dessa relação.


    7. Box Plot

   É um gráfico
  que possibilita
 interpretar rapi-
 damente o com-
  portamento da
 distribuição em
  cada ¼ ou 25%
(quartis). É muito
útil para observar-
mos a existência
de valores discre-
 pantes (outliers)
  e a simetria da
    distribuição.




                                  MIDIATECA

     Na midiateca encontram-se alguns links que direcionam para apresentações
     com gráficos dinâmicos. São gráficos interessantes que se movimentam. Vale
     a pena conferir!




                                         51

## Página 52

Para construir um gráfico é preciso saber que:


    • Todo gráfico deve ter título e escala.
    • O título deve ser escrito acima do gráfico.
    • As variáveis devem ser identificadas nos eixos coordenados, inclusive com as uni-
    dades convenientes.
    • As linhas auxiliares (grade) são opcionais, mas ajudam na leitura.
    • A fonte é opcional nos gráficos.


         Ampliando o foco


       Para colocar em prática o conteúdo visto, faça os exercícios da seção 2.1 do
       livro LARSON, R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010,
       disponível na Biblioteca Virtual.


       Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
       com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                             52

## Página 53

Medidas de posição e de dispersão

Neste tópico apresentaremos as medidas de posição e as medidas de dispersão, que
também nos auxiliam na tarefa de resumir os dados.


Medidas de posição

São medidas usadas para resumir um conjunto de dados em um único valor. As medidas
de posição mais importantes são as medidas de tendência central, que recebem esse
nome pelo fato de que os dados observados tendem, em geral, a se agrupar em torno de
um valor localizado no centro da distribuição de uma série de observações.


    • Medidas de tendência central: a média, a mediana e a moda.
    • Medidas de posição (separatrizes): os quartis, os decis e os percentis e a própria
    mediana.


Para calcularmos as medidas de posição poderemos encontrar os dados organizados de
três maneiras, a saber:


                                   Dados agrupados               Dados agrupados
   Dados não agrupados.             em tabelas SEM               em tabelas COM
                                 intervalos de classes.        intervalos de classe.


Para cada situação acima temos uma metodologia de cálculo diferente. Por isso, antes
de calcularmos as medidas de posição, primeiro temos que identificar como os dados
foram apresentados.


Agora vamos conhecer cada metodologia.


1. Média Aritmética: (x ou )
É a medida de posição mais conhecida e mais utilizada, embora nem sempre seja a mais
representativa.


Se os dados referirem-se a uma amostra, indicamos a média por x quando trabalhamos
com a população indicamos pela letra grega μ.




                                          53

## Página 54

Nas fórmulas estatísticas é comum representar a primeira observação da variável “x” por
x1, o valor da segunda observação por x2 e assim sucessivamente. Em geral o valor da
i-ésima observação é indicado por xi.


E quais as vantagens e desvantagens na utilização da média?




                                                     É fortemente influenciada por valores
          Facilidade de se calcular.
                                                    extremos e por isso só deve ser utilizada
                                                          em distribuições simétricas.
     É a mais conhecida e mais utilizada.

                                                          Não pode ser calculada para
    É usada para compararmos conjuntos
                                                        distribuições de frequências com
               semelhantes.
                                                                limites indefinidos.




                               Tabela: Cálculo da média aritmética.


                                                    Em que:
                                       Σxi
                            x =
                                       n            x – média aritmética amostral.
                                                    xi – valores da variável.
   Para dados                                       n – número de entradas em uma amostra
                                 Ou
  não agrupados
                                                                       ou
                                       Σxi          μ – média aritmética populacional.
                            μ=
                                       N            xi – valores da variável.
                                                    N – número de entradas em uma população.

                                                    Em que:
                                   Σxi . fi
                          x =
                                       n            x – média aritmética amostral.
                                                    xi – valores da variável.
     Para dados
   agrupados em                   ou                fi – frequência da classe.
    tabelas sem                                     n – número de entradas em uma amostra
     intervalos                                                        ou
     de classes
                                   Σxi . fi
                          μ=                        μ – média aritmética populacional.
                                       N
                                                    xi – valores da variável.
                       (média ponderada)            fi – frequência da classe.
                                                    N – número de entradas em uma população.




                                               54

## Página 55

                                 Σxi . fi         Em que:
                        x =
    Para dados                      n
                                                  x – média aritmética amostral.
  agrupados em
                               ou                 μ – média aritmética populacional.
   tabelas com
    intervalos                                    Xi – pontos médios das classes.
    de classes                                    fi – a frequência da classe.
                                 Σxi . fi         n – número de entradas em uma amostra.
                        μ=
                                    N             N – número de entradas em uma população.




                             Fonte: Elaborada pela autora (2020).




Observe alguns exemplos práticos.


Exemplo 1
Para dados não agrupados.


Suponha que a Central de Estágios da nossa universidade tenha enviado um questionário
em julho de 2020, a uma amostra de 10 recém-graduados no curso de Engenharia Civil,
solicitando-lhes informações acerca dos salários mensais iniciais. Os dados coletados
foram os seguintes:




        R$ 5.800,00 R$ 6.800,00      R$ 8.000,00        R$ 8.200,00   R$ 8.000,00

        R$ 7.300,00 R$ 7.300,00      R$ 7.900,00         R$ 7.900,00 R$ 7.900,00




Vamos calcular a média aritmética dos salários mensais dos engenheiros.




                                             55

## Página 56

Podemos observar que os dados não estão agrupados em tabelas. Utilize a fórmula de
cálculo de média para amostras de dados não agrupados:




                                                    Σxi
                                         x=
                                                    n


          5800 + 6800 + 8000 + 8200 + 8000 + 7300 + 7300 + 7900 + 7900 + 7900
    x=
                                                    10


                                           75100
                                    x=                  = 7510
                                              10




Interpretação do resultado: os salários mensais dos engenheiros civis pesquisados gi-
ram em torno de R$ 7.510,00.


Exemplo 2
Para dados agrupados em tabelas sem intervalo de classes.


Suponha que as anotações dos salários vieram organizadas em uma tabela sem interva-
los de classes. Vamos calcular a média dos salários dos engenheiros.



                Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                          Salário                  Quantidade de pessoas (fi)

                           5800                                  1

                           6800                                  1

                           7300                                  2

                           7900                                  3

                           8000                                  2

                           8200                                  1

                           Total                                 10


                       Fonte: Central de Estágios da UVA. Dados fictícios.



                                               56

## Página 57

Utilizando a fórmula para cálculo da média para dados agrupados em tabelas sem in-
tervalo de classes temos:



                                                  Σxi . fi
                                        x=
                                                     n


               5800 . 1 + 6800 . 1 + 7300 . 2 + 7900 . 3 + 8000 . 2 + 8200 . 1
         x=
                                                    10



                       5800 + 6800 + 14600 + 23700 + 16000 + 8200
         x=
                                                    10


                                          75100
                                   x=                    = 7510
                                             10



Exemplo 3
Para dados agrupados em tabelas com intervalo de classes.


Agora, vamos supor que as anotações dos salários vieram organizadas em uma tabela
com intervalos de classes. Vamos calcular a média dos salários dos engenheiros.



               Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                         Salário                   Quantidade de pessoas (fi)

                     5000 I---- 6000                              1

                      6000 I--- 7000                              1

                      7000 I--- 8000                              5

                      8000 I--- 9000                              3

                          Total                                   10


                      Fonte: Central de Estágios da UVA. Dados fictícios.




                                              57

## Página 58

Primeiramente, antes de calcular a média, temos que calcular o ponto médio de cada
classe.



                   Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


          Salário                 Quantidade de pessoas (fi)          Ponto médio da classe (Xi)

                                                                         5000 + 6000
       5000 I---- 6000                           1                                     = 5500
                                                                                2

                                                                        6000 + 7000
       6000 I--- 7000                            1                                     = 6500
                                                                                2

                                                                         7000 + 8000
       7000 I--- 8000                            5                                     = 7500
                                                                                2

                                                                        8000 + 9000
       8000 I--- 9000                            3                                     = 8500
                                                                                2
           Total                                 10


                          Fonte: Central de Estágios da UVA. Dados fictícios.




Agora podemos calcular a média dos salários:



                                                      Σxi . fi
                                            x=
                                                         n


                               5500 . 1 + 6500 . 1 + 7500 . 5 + 8500 . 3
                       x=
                                                        10


                                     5500 + 6500 + 37500 + 25500
                       x=
                                                        10


                                             75000
                                       x=                    = 7500
                                                 10




                                                  58

## Página 59

Note que, ao agruparmos os dados com intervalos de classes, perdemos um pouco a
precisão das medidas. Por isso, devemos, sempre que possível, usar entre 5 e 20 classes.


Na midiateca você encontra o endereço para pesquisar mais sobre os salários médios e
medianos das categorias profissionais.


1. Mediana

Muitas vezes encontramos nas distribuições assimétricas valores discrepantes (valores
muito grandes ou muito pequenos) e que faz com que a média seja muito influenciada.
Veja um exemplo a seguir.


         Exemplo


       Suponha que as notas dos alunos sejam as seguintes: 0; 5; 5; 5.


       Sem fazer nenhum cálculo podemos observar que as notas giram em torno de
       5, porém se calcularmos a média obtemos como resposta o valor 3,75.


       Nesses casos, em que temos alguns valores muito altos ou muito baixos em
       relação aos outros valores, é mais conveniente usarmos outra medida de posi-
       ção, a mediana.


       A mediana é o valor central do rol, ou seja, o valor que divide um grupo de valo-
       res ordenados em duas partes com o mesmo número de termos, como mostra
       a imagem a seguir:


                                       50%         50%

                              Mínimo      Mediana        Máximo




E quais as vantagens e desvantagens na utilização da mediana?




                                              59

## Página 60

                                                A mediana flutua mais de amostra para
                                                  amostra do que a média aritmética,
                                                    portanto é menos confiável.
   Sua determinação é rápida e fácil,
 sem necessitar de cálculos complexos.
                                                      Não utiliza todos os dados.

Não é influenciada por valores extremos.
                                                  Não é possível para todos os dados.

   Pode ser calculada quando temos
                                                   Não é possível calcular a mediana
   limites de classes indefinidos, pois
                                                de um grupo total a partir das medianas
  só depende do tamanho da amostra
                                                          de dois subgrupos.
            ou da população.

                                                   Não é levada em consideração na
                                                    maioria dos testes estatísticos.




                               Tabela: Cálculo da Mediana.




                     1 – Ordenar os dados numéricos, ou seja, confeccionar o rol.

                     2 – Se o tamanho da amostra ou da população for um número ím-
Para dados não
                     par, a mediana é o valor que estiver no centro do rol.
  agrupados

                     3 – Se o tamanho da amostra ou da população for um número par, a
                     mediana é a média aritmética dos dois valores centrais do rol.




                                           60

## Página 61

                 1 – Calcular a posição ocupada mediana, que denominamos de
                 Posto da Mediana.

                 a) Se a quantidade de dados for “par” usamos:


                         Σfi
                  P=
                          2
  Para dados
  agrupados      b) Se a quantidade de dados for “ímpar” usamos:
  em tabelas
sem intervalos
  de classes             1 + Σfi
                  P=
                              2

                 2 – Calcula-se as frequências acumuladas.

                 3 – Identifique na coluna das frequências acumuladas, a primeira
                 classe que contenha o posto da mediana. O valor dessa classe, será
                 o valor da mediana.



                 1) Calcular o posto da mediana
                                                     Em que:
                         Σfi
                  P=
                          2                          Md = mediana.
                                                     Li = limite inferior do Posto da
                 2) Identificar a primeira classe    Mediana.
                 que contenha o posto da me-         h = amplitude da classe que
 Para dados
  agrupados      diana.                              contém o posto da mediana.
  em tabelas                                         Σfi = tamanho da amostra ou
com intervalos   3) Utilizar a fórmula de igualda-   da população.
  de classes     de dos quocientes:                  Fa.ant = frequência acumulada
                                                     da classe anterior a classe que
                                                     contém o posto da mediana.
                                   Σfi
                                          – Fa.ant   fi = frequência absoluta da
                                   2
                  Md = Li + h ·                      classe que contém o posto da
                                                     mediana.
                                          fi




                                         61

## Página 62

Observe alguns exemplos práticos.


Exemplo 1
Para dados não agrupados


Voltemos aos dados dos salários dos engenheiros recém-formados coletados pela Cen-
tral de Estágios em julho de 2020:



         R$ 5.800,00 R$ 6.800,00        R$ 8.000,00       R$ 8.200,00       R$ 8.000,00

         R$ 7.300,00 R$ 7.300,00        R$ 7.900,00       R$ 7.900,00      R$ 7.900,00




Vamos determinar a mediana dos salários mensais dos engenheiros. Para isso:


1º ) Confeccione o Rol.


  5800     6800      7300      7300       7900         7900    7900     8000         8000   8200


Como o tamanho da amostra é igual a 10 (par) não encontramos um valor no centro do
rol e sim dois valores. Devemos, então, fazer a média aritmética desses dois valores:


                                         7900 + 7900
                                 Md =                         7900
                                                 2


Exemplo 2
Para dados agrupados em tabelas sem intervalo de classe.


Agora vamos determinar a mediana para dados agrupados em tabelas sem intervalo
de classe:



                  Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                            Salário                   Quantidade de pessoas (fi)

                             5800                                 1

                             6800                                 1



                                                 62

## Página 63

                                  7300                                 2

                                  7900                                 3

                                  8000                                 2

                                  8200                                 1

                                  Total                               10


                              Fonte: Central de Estágios da UVA. Dados fictícios.


1º) Calcule o posto da mediana:



         Σfi
   P=
         2

         10
   P=             =5
         2



2º) Determine as frequências acumuladas:



                       Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



          Salário                     Quantidade de pessoas (fi)            Frequência Acumulada

               5800                                  1                                    1

               6800                                  1                                    2

               7300                                  2                                    4

               7900                                  3                                    7

               8000                                  2                                    9

               8200                                  1                                10

               Total                                10


                              Fonte: Central de Estágios da UVA. Dados fictícios.




                                                      63

## Página 64

3º) Procure na coluna da frequência acumulada a primeira classe que contém o posto da
mediana, que nesse exemplo foi igual a 5.



                Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



                                                       Frequência Acumulada
                     Quantidade de
      Salário                                  (é nessa coluna que faremos a análise,
                      pessoas (fi)
                                            mas não é aqui que estará o valor da mediana)


                                                                     1
       5800                 1
                                                    (não contém, pois é menor do que 5)

                                                                     2
       6800                 1
                                                    (não contém, pois é menor do que 5)

                                                                    4
       7300                 2
                                                    não contém, pois é menor do que 5)

                                                                    7
                                             (ok! A frequência acumulada 7 é maior do que
       7900                 3
                                                                    5)
                                                       essa é a classe da mediana

       8000                 2                                         9

       8200                 1                                        10

       Total                10


                       Fonte: Central de Estágios da UVA. Dados fictícios.




Logo, a mediana será igual ao valor da variável que estiver na classe selecionada, a pri-
meira classe em que encontramos o valor da frequência acumulada maior do que o valor
do posto. Então: Md = 7900


Exemplo 3
Para os dados agrupados em tabelas com intervalos de classes.


Finalmente, vamos calcular a mediana para os dados agrupados em tabelas com inter-
valos de classes.



                                               64

## Página 65

Vamos continuar trabalhando com o exemplo dos salários dos engenheiros, porém ago-
ra agrupados em uma tabela com intervalos. Observe a tabela a seguir.

                   Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                             Salário                     Quantidade de pessoas (fi)

                         5000 I---- 6000                             1

                          6000 I--- 7000                             1

                          7000 I--- 8000                             5

                          8000 I--- 9000                             3

                              Total                                 10


                          Fonte: Central de Estágios da UVA. Dados fictícios.




1º) Calcule o posto da mediana:


                                                         Σfi
                                              P=
                                                          2

                                                   10
                                           P=                  =5
                                                     2


2º) Determine as frequências acumuladas:


                   Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



          Salário                 Quantidade de pessoas (fi)             Frequência Acumulada

       5000 I---- 6000                           1                                    1

       6000 I--- 7000                            1                                    2

       7000 I--- 8000                            5                                    7

       8000 I--- 9000                            3                                    10

           Total                                10


                          Fonte: Central de Estágios da UVA. Dados fictícios.



                                                   65

## Página 66

3º) Procure na coluna da frequência acumulada a primeira classe que contém o posto da
mediana, que nesse exemplo foi igual a 5.



                    Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.

          Salário                   Quantidade de pessoas (fi)            Frequência Acumulada
       5000 I---- 6000                            1                                 1 (1<5)
       6000 I--- 7000                             1                                2 (2<5)
       7000 I--- 8000                             5                               7 (7>5 ok!)
       8000 I--- 9000                             3                                   10
           Total                                  10

                           Fonte: Central de Estágios da UVA (dados fictícios).


Observando a classe do Posto da Mediana, temos:



        Limite inferior da classe: Li = 7000
        Amplitude da classe: h = 8000 – 7000 = 1000

        Frequência absoluta da classe: fi = 5
        Frequência acumulada da classe anterior: Fa.ant = 2
        Metade dos dados:


          Σfi         10
                =            =5
          2            2

        Substituindo os valores na fórmula, temos:


                              Σfi
                                    – Fa.ant
          Md = Li + h ·       2

                                    fi

                                     5–2
         Md = 7000 + 1000 ·
                                         5

        Md = 7000 + 1000 · 0,6


        Md = 7000 + 600 = 7600




                                                   66

## Página 67

         Ampliando o foco


        Os fractis são números que dividem um conjunto de dados ordenados em par-
        tes iguais, ou seja, em partes que contêm a mesma quantidade de elementos
        da série.


        A mediana é um fractil porque divide o rol em duas partes iguais.


        Os quartis Q1, Q2, e Q3 dividem, aproximadamente, um conjunto de dados orde-
        nados em quatro partes iguais.


        Os percentis dividem, aproximadamente, um conjunto de dados ordenados em
        100 partes iguais.


        E assim por diante: Decis (10 partes) Quintis (5 partes).




2. MODA

É o valor, ou valores, da distribuição que ocorrem com a maior frequência, ou seja, o valor,
ou valores que mais se repetem dentro de uma série de observações. De maneira geral:


     • Se todos os valores repetem-se a mesma quantidade de vezes, dizemos que não
     há moda, ou seja, a distribuição é amodal.
     • Se um valor ocorre com maior frequência, chamamos a distribuição de unimodal.
     • Se dois valores repetem-se com a mesma quantidade e com maior frequência,
     chamamos a distribuição de bimodal.
     • Três valores, então trimodal.
     • Mais de três valores, dizemos que a distribuição é multimodal ou polimodal.


Vantagens e desvantagens na utilização da moda:


  Pode ser usada quando trabalhamos com variáveis qualitativas (categóricas).
  Não é influenciada por valores extremos.
  Pode ser calculada para distribuições com limites indefinidos.
  Não leva em consideração todos os valores do conjunto de dados.




                                                67

## Página 68

                               Tabela: Cálculo da Moda.




   Para dados não
                         Basta verificar qual é o valor que mais se repete.
     agrupados



Para dados agrupados     Basta observar a classe, ou classes, de maior frequência ab-
   em tabelas sem
                         soluta.
intervalos de classes



                                                          Em que:

                                                          Mo = moda.

                                                          Classe modal= classe de maior
                         - Moda bruta:                    frequência.
                         É o ponto médio da clas-
                         se de maior frequência.          Li = limite inferior da classe mo-
                                                          dal.
                         - Moda pelo processo
                         de Czuber:                       D1 = diferença entre a frequên-
                                                          cia absoluta da classe modal e
                                         D1               a frequência da classe anterior
                         Mo = Li +                  ·h    a ela.
Para dados agrupados                  D1 + D 2
    em tabelas com
                                                          D2 = diferença entre a frequên-
 intervalos de classes   - Moda pelo processo             cia absoluta da classe modal e
(Vamos abordar quatro    de King:
 processos diferentes)                                    a frequência da classe poste-
                                                          rior a ela.
                                        fpost
                         Mo = Li +                  ·h    h = amplitude da classe modal.
                                     fant + fpost
                                                          fant =frequência simples da clas-
                         - Moda pelo processo
                                                          se anterior à classe modal.
                         de Pearson
                                                          fpost =frequência simples da
                             Mo = 3 · Md – 2 · x          classe posterior à classe mo-
                                                          dal.

                                                          Md = mediana.

                                                          x ou μ =média.




                                            68

## Página 69

Exemplo 1
Para dados não agrupados.


Voltemos aos dados dos salários dos engenheiros recém-formados coletados pela Cen-
tral de Estágios em julho de 2020:



        R$ 5.800,00 R$ 6.800,00       R$ 8.000,00     R$ 8.200,00         R$ 8.000,00

        R$ 7.300,00 R$ 7.300,00       R$ 7.900,00     R$ 7.900,00         R$7.900,00




Vamos determinar a moda dos salários mensais dos engenheiros. Observamos no con-
junto de dados que o valor que mais se repetiu foi R$ 7.900,00. Portanto, esse valor é o
valor da moda.


Exemplo 2
Para dados agrupados em tabelas sem intervalo de classe.


Agora vamos determinar a moda para dados agrupados em tabelas sem intervalo de
classe.


                Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                          Salário                Quantidade de pessoas (fi)

                           5800                                1

                           6800                                1

                           7300                                2

                           7900                                3

                           8000                                2

                           8200                                1


Observamos que a 4ª classe apresenta a maior frequência absoluta. Logo, a média será
o valor da variável nessa classe, ou seja, R$ 7.900,00.




                                              69

## Página 70

Exemplo 3
Para dados agrupados em tabelas com intervalo de classe


Vamos continuar trabalhando com o exemplo dos salários dos engenheiros, porém ago-
ra agrupados em uma tabela com intervalos.


                 Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                           Salário                Quantidade de pessoas (fi)

                       5000 I---- 6000                          1

                        6000 I--- 7000                          1

                        7000 I--- 8000                          5

                        8000 I--- 9000                          3

                            Total                               10


                       Fonte: Central de Estágios da UVA (dados fictícios).


Classe Modal → classe de maior frequência: 3ª classe: 7000 I--- 8000


Li = 7000

D1 = 5 – 1 = 4

D2 = 5 – 3 = 2

h = 8000 – 7000 = 1000

fant = 1

fpost = 3

Md = 7600

x = 7500

                      7000 + 8000
– Moda bruta: Mo =                       = 7500
                             2




                                               70

## Página 71

Moda pelo processo de Czuber

               D1
Mo = Li +                  ·h
            D1 + D 2

                  4
 Mo = 7000 +         · 1000
                 4+2
Mo = 7000 + 0,66667 · 1000

Mo = 7000 + 666,67

Mo = 7666,67

Moda pelo processo de King

               fpost
Mo = Li +                  ·h
            fant + fpost


                       3
 Mo = 7000 +                · 1000
                  1+3


Mo = 7000 + 0,75 · 1000

Mo = 7000 + 750

Mo = 7750


Moda pelo processo de Pearson

Mo = 3 · Md – 2 · x

Mo = 3 · 7600 – 2 · 7500

Mo = 22800 – 15000


Mo = 7800




                                     71

## Página 72

Relações entre média, mediana e moda

Ao traçar o histograma e o polígono de frequências de uma distribuição podemos verifi-
car, visualmente, qual é o comportamento dos dados.


Agora vamos relacionar a imagem dos gráficos às medidas de posição.


 Se média = mediana = moda       Se média > mediana > moda            Se média < mediana < moda

         Distribuição               Distribuição assimétrica            Distribuição assimétrica
          simétrica                          positiva                           negativa




                               Fonte: Elaborada pela autora (2020).




A forma da distribuição pode interferir na escolha da medida de tendência central. Ou seja:


    • Se a distribuição for aproximadamente simétrica, a média aritmética é a mais in-
    dicada, mesmo porque esta poderá ser utilizada em estatística mais avançada e é
    uma medida mais estável.
    • Se a pretensão for uma medida descritiva, rápida e simples ainda que grosseira, e
    se a distribuição for unimodal, o pesquisador deve escolher a moda.
    • Se a pretensão for uma medida exata, ele poderá optar entre a média e a mediana.



         Ampliando o foco


       Para colocar em prática o conteúdo visto, faça os exercícios da seção 2.3 do
       livro LARSON, R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010,
       disponível na biblioteca virtual.


       Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
       com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                               72

## Página 73

Medidas de dispersão ou de variabilidade

Suponha que temos três turmas diferentes, cada uma com 10 alunos, para as quais foi
aplicada uma avaliação com valor de 10 pontos.


As notas foram as seguintes:


    Notas
               N1     N2       N3     N4    N5    N6       N7   N8       N9   N10   Total
   Turmas

  Turma “A”    5       5       5      5     5      5       5    5        5     5     50


 Turma “B”     0       0       0      5     5      5       5    10       10   10     50


 Turma “C”     1       2       3      4     5      5       6    7        8     9     50



Se calcularmos a média, a moda e a mediana para cada turma teremos:


                   Turmas           Média        Mediana        Moda

               Turma “A”              5             5                5

               Turma “B”              5             5                5

               Turma “C”              5             5                5


O que podemos concluir?


Analisando somente as medidas de posição, concluímos que essas três turmas são idên-
ticas, o que seria um grande erro.


Para verificar o comportamento dos dados e o grau de dispersão em torno da média,
precisamos de outras medidas estatísticas, que são as Medidas de Dispersão ou Varia-
bilidade, ou seja:


    • A Amplitude.
    • O Desvio Médio.
    • A Variância, o Desvio-Padrão e o Coeficiente de Variação.




                                            73

## Página 74

Agora veremos com mais detalhes cada uma delas. Vamos lá!


Amplitude (Range)

É a diferença entre o maior e o menor valor do conjunto.


Voltando ao exemplo das notas das três turmas, temos:


    • Turma A: 5 – 5 = 0 (nenhuma dispersão, turma homogênea).
    • Turma B: 10 – 0 = 10 (dados muito dispersos, turma heterogênea).
    • Turma C: 9 -1 = 8 (dados muito dispersos, turma heterogênea, porém mais homo-
    gênea do a turma B).


Para dados agrupados com intervalos de classe a amplitude total é:


A diferença entre o limite superior da última classe e o limite inferior da primeira classe.


A amplitude é muito pouco usada, pois tem uma grande desvantagem que a é de levar
em conta apenas dois valores, desprezando todos os outros.


Desvio: (D)

É a diferença entre cada valor observado e a média destes valores.


                                         D = xi – x


Propriedade importante: a soma dos desvios em relação à média é sempre igual a zero.


Desvio médio: (Dm)

É a média aritmética dos módulos dos desvios.

                                               Σ|xi – x|
    • Para dados não agrupados: Dm =
                                                   n


                                        Σ|xi – x| · fi
    • Para dados agrupados: Dm =
                                             n




                                              74

## Página 75

Variância: (s2 ou σ2)

É a média aritmética dos quadrados dos desvios em relação à média da distribuição.



 Tabela: Cálculo da Variância para dados não agrupados, dados agrupados em tabelas de distribuição
           sem intervalos de classes e em tabelas de distribuição com intervalos de classes.




                                  Amostras                         População                 Em que:



  Para dados não                     Σ(xi – x)2                       Σ(xi – μ)2       s2 ou σ2 = variância
                           s =2
                                                           σ = 2
    agrupados.                         n–1                                 N           n = tamanho da
                                                                                       amostra
     Para dados                                                                        N = tamanho da
  agrupados em                      Σ(xi – x)2 . fi                  Σ(xi – μ)2 . fi   população
    tabelas sem          s2 =                         σ2 =
                                       n–1                                 N           x = média amostral
   intervalos de
      classes.                                                                         μ = média popula-
                                                                                       cional
     Para dados
                                                                                       xi = valor observado
  agrupados em                     Σ(Xi – x)2 . fi                   Σ(Xi – μ)2 . fi
    tabelas com          s =
                          2
                                                      σ =  2                           Xi = ponto médio da
   intervalos de                       n–1                                 N           classe
      classes.




Observe:


    • Os denominadores são diferentes. Para amostras usamos “n - 1” e para população
    usamos “n”.
    • Como no cálculo da variância eleva-se o desvio ao quadrado, a unidade de medi-
    da resultante sempre será a unidade de medida original elevada ao quadrado. Por
    exemplo, seriam reais (dinheiro) ao quadrado.
    • Como esse tipo de unidade de medida não faz sentido, o que ocorre em muitos
    casos é extrair a raiz quadrada da variância, obtendo um valor que tem a mesma
    unidade de medida que os dados originais — o desvio-padrão.




                                                      75

## Página 76

Desvio-padrão: (s ou σ)

É a raiz quadrada da variância.


s = s2   para amostras
ou
σ = σ2    para população


Coeficiente de Variação

É o quociente entre o desvio-padrão e a média de uma distribuição.


Normalmente, multiplicamos o resultado por 100 para expressarmos essa medida em
porcentagem.


Para saber se uma dispersão é muito grande em relação à média ou para comparar o
grau de dispersão relativa entre as distribuições de duas ou mais variáveis é preciso utili-
zar uma medida de dispersão relativa, que se chama Coeficiente de Variação.
O coeficiente de variação é útil para comparar a variabilidade de observações com dife-
rentes unidades de medida, como:



                                       s
                              CV =           para amostras
                                       x
                                            ou
                                       σ
                              CV =          para população
                                       μ



Veja a interpretação dos valores dos coeficientes de variação. A saber:


     • Dispersão baixa: quando CV inferior a 10%.
     • Dispersão média: quando CV estiver entre 10% e 20%.
     • Dispersão alta: quando CV estiver entre 20% e 30%.
     • Dispersão muito alta: quando CV for superior a 30% (pequena representatividade
     da média).




                                             76

## Página 77

Exemplo 1
Para dados não agrupados


Voltemos aos dados dos salários dos engenheiros recém-formados coletados pela Cen-
tral de Estágios em julho de 2020.



           R$ 5.800,00 R$ 6.800,00    R$ 8.000,00   R$ 8.200,00      R$ 8.000,00

           R$ 7.300,00 R$ 7.300,00    R$ 7.900,00   R$ 7.900,00      R$7.900,00




Vamos determinar a variância, o desvio-padrão e o coeficiente de variação dos salários
mensais dos engenheiros.


1º) Calcule a média aritmética da distribuição.

      Σxi
 x=
       n


      5800 + 6800 + 8000 + 8200 + 8000 + 7300 + 7300 + 7900 + 7900 + 7900
 x=
                                            n


      75100
 x=             = 7510
       10


2º) Calcule os desvios em relação à média e depois eleve cada desvio ao quadrado.


                             desvio                        desvio2


                    5800 - 7510 =      -1710               2924100


                    6800 - 7510 =       -710               504100


                    7300 - 7510 =       -210                44100


                    7300 - 7510 =       -210                44100




                                               77

## Página 78

                  7900 - 7510 =     390              152100


                  7900 - 7510 =     390              152100


                  7900 - 7510 =     390              152100


                  8000 - 7510 =     490              240100


                  8200 - 7510 =     690              476100



3º) Some de todos os desvios elevados ao quadrado.


                           desvio                    desvio2


                  5800 - 7510 =     -1710            2924100


                  6800 - 7510 =     -710             504100


                  7300 - 7510 =     -210              44100


                  7300 - 7510 =     -210              44100


                  7900 - 7510 =     390              152100


                  7900 - 7510 =     390              152100


                  7900 - 7510 =     390              152100


                  8000 - 7510 =     490              240100


                  8000 - 7510 =     490              240100


                  8200 - 7510 =     690              476100


                                                     4929000




                                            78

## Página 79

Como estamos trabalhando com uma amostra, para calcular a variância devemos
dividir esse somatório por “n-1”:

        Σ(xi – x)2         4929000
 s2 =                  =                = 547666,67
         n–1                10 – 1


O desvio-padrão será a raiz quadrada da variância:


s = 547666,67 = 740,05


O coeficiente de variação é a divisão entre o desvio-padrão e a média.

         740,05
CV =                   = 0,0985
          7510


Expressando em porcentagem temos:


CV = 0,0985 . 100 = 9,85% (baixa dispersão)


Exemplo 2
Para dados agrupados em tabelas sem intervalo de classe.


Agora vamos determinar as medidas de dispersão para dados agrupados em tabelas
sem intervalo de classe.



                     Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



                              Salário                   Quantidade de pessoas (fi)


                               5800                                   1


                               6800                                   1


                               7300                                   2


                               7900                                   3




                                                   79

## Página 80

                                  8000                                    2


                                  8200                                    1


                                  Total                                  10


                              Fonte: Central de Estágios da UVA (dados fictícios).


1º) Calcule a média.

         Σxi . fi
  x=
            n


       5800 . 1 + 6800 . 1 + 7300 . 2 + 7900 . 3 + 8000 . 2 + 8200 . 1
  x=
                                               10


       5800 + 6800 + 14600 + 23700 + 16000 + 8200
  x=
                                       10


        75100
  x=                   = 7510
           10



2º) Determine os desvios em relação à média.


                     Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                                       Quantidade de
                    Salário                                         desvio = xi-média
                                        pessoas (fi)

                     5800                     1                5800 - 7510 =         -1710


                     6800                     1                6800 - 7510 =         -710


                     7300                     2                7300 - 7510 =         -210


                     7900                     3                7900 - 7510 =         390




                                                      80

## Página 81

                  8000                    2                8000 - 7510 =           490


                  8200                    1                8200 - 7510 =           690


                  Total                   10



                          Fonte: Central de Estágios da UVA (dados fictícios).


3º)Eleve o desvio ao quadrado.


                  Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



                            Quantidade de
        Salário                                         desvio = xi-média            desvio²
                             pessoas (fi)

         5800                      1                5800 - 7510 =          -1710     2924100


         6800                      1                6800 - 7510 =          -710      504100


         7300                      2                7300 - 7510 =          -210          44100


         7900                      3                7900 - 7510 =           390      152100


         8000                      2                8000 - 7510 =           490      240100


         8200                      1                8200 - 7510 =           690      476100


         Total                    10



                          Fonte: Central de Estágios da UVA (dados fictícios).




                                                  81

## Página 82

4º) Multiplique os quadrados dos desvios pelas respectivas frequências absolutas e fazer
o somatório.


                       Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



              Quantidade de
 Salário                               desvio = xi-média          desvio²             desvio² . fi
               pessoas (fi)

  5800                  1           5800 - 7510 =      -1710     2924100      2924100 . 1 =    2924100


  6800                  1           6800 - 7510 =         -710    504100       504100 . 1 =     504100


  7300                  2           7300 - 7510 =         -210     44100        44100 . 2 =      88200


  7900                  3           7900 - 7510 =         390     152100       152100 . 3 =     456300


  8000                  2           8000 - 7510 =         490     240100       240100 . 2 =     480200


  8200                  1           8200 - 7510 =         690     476100       476100 . 1 =     476100


  Total                10                                                                      4929000



                             Fonte: Central de Estágios da UVA (dados fictícios).




Como estamos trabalhando com uma amostra, para calcular a variância, devemos
dividir esse somatório por “n-1”:

          Σ(xi – x)2         4929000
 s2 =                    =               = 547666,67
           n–1                10 – 1


O desvio-padrão será a raiz quadrada da variância:


s = 547666,67 = 740,05


O coeficiente de variação é a divisão entre o desvio-padrão e a média.

           740,05
 CV =                    = 0,0985
            7510


                                                     82

## Página 83

Expressando em porcentagem temos:


CV = 0,0985 . 100 = 9,85% (baixa dispersão)


Exemplo 3
Para dados agrupados em uma tabela de distribuição com intervalos de classes.


Finalmente vamos calcular as medidas de dispersão para os dados agrupados em uma
tabela de distribuição com intervalos de classes.


Voltemos ao exemplo.


                    Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.



            Salário            Quantidade de pessoas (fi)        Ponto médio da classe (Xi)

                                                                    5000 + 6000
       5000 I---- 6000                       1                                         = 5500
                                                                          2

                                                                    6000 + 7000
        6000 I--- 7000                       1                                         = 6500
                                                                          2

                                                                    7000 + 8000
        7000 I--- 8000                       5                                         = 7500
                                                                          2

                                                                    8000 + 9000
        8000 I--- 9000                       3                                         = 8500
                                                                          2

             Total                          10



                          Fonte: Central de Estágios da UVA (dados fictícios).




1º) Calcule a média dos salários com a fórmula dos dados agrupados com intervalos.

         Σxi . fi
  x=
            n




                                                  83

## Página 84

          5500 . 1 + 6500 . 1 + 7500 . 5 + 8500 . 3
  x=
                                10


                 5500 + 6500 + 37500 + 25500
  x=
                                10


          75000
  x=                 = 7500
            10


2º) Calcule o desvio de cada ponto médio em relação à média, fazendo a diferença entre
o valor do ponto médio e a média.

                    Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                       Quantidade de          Ponto médio da                  Desvio =
       Salário
                        pessoas (fi)            classe (Xi)             = Ponto Médio - Média

 5000 I---- 6000               1                    5500               5500 - 7500 =     -2000


  6000 I--- 7000               1                    6500               6500 - 7500 =     -1000


  7000 I--- 8000               5                    7500               7500 - 7500 =       0


  8000 I--- 9000               3                    8500               8500 - 7500 =     1000

        Total                 10


                          Fonte: Central de Estágios da UVA (dados fictícios).


3º) Eleve os desvios ao quadrado.


                    Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                                                                      Desvio =
                        Quantidade de        Ponto médio
       Salário                                                     = Ponto Médio -      Desvio²
                         pessoas (fi)        da classe (Xi)
                                                                       Média

  5000 I---- 6000              1                  5500           5500 - 7500 =-2000    4000000

  6000 I--- 7000               1                  6500           6500 - 7500 = -1000   1000000




                                                  84

## Página 85

  7000 I--- 8000                   5                 7500               7500 - 7500 =          0


  8000 I--- 9000                   3                 8500               8500 - 7500 =      1000000


        Total                      10



                             Fonte: Central de Estágios da UVA (dados fictícios).


4º) Multiplique o quadrado do desvio pela respectiva frequência absoluta e fazer o soma-
tório dessa coluna.


                       Tabela: Salário mensal de 10 engenheiros civis em julho de 2020.


                                             Ponto
                       Quantidade de                        Desvio = Ponto
    Salário                                 médio da                            Desvio²   Desvio² . fi
                        pessoas (fi)                        Médio - Média
                                           classe (Xi)
                                                            5500 - 7500 =                 4000000 . 1
 5000 I---- 6000               1              5500                              4000000
                                                               -2000                      = 4000000

                                                            6500 - 7500 =
 6000 I--- 7000                1              6500                              1000000    1000000
                                                               -1000

                                                            7500 - 7500 =
 7000 I--- 8000                5              7500                                  0      0.5=0
                                                                  0

                                                            8500 - 7500 =                 1000000 . 3
 8000 I--- 9000                3              8500                              1000000
                                                               1000                       = 3000000

        Total                 10                                                           8000000



                             Fonte: Central de Estágios da UVA (dados fictícios).


Como estamos trabalhando com uma amostra, para calcular a variância devemos
dividir esse somatório por “n-1”:

          Σ(xi – x)2         8000000
 s2 =                    =                 = 888888,89
           n–1                 10 – 1




                                                     85

## Página 86

O desvio-padrão será a raiz quadrada da variância:


s = 888888,89 = 942,81


O coeficiente de variação é a divisão entre o desvio-padrão e a média.

        942,81
CV =                = 0,126
         7500


Expressando em porcentagem, temos:


CV = 0,126 . 100 = 12,5% (média dispersão)


Mais uma vez notamos que os dados agrupados perdem um pouco a precisão, principal-
mente se forem agrupados em poucas classes.


         Ampliando o foco


       Para praticar o conteúdo visto, faça os exercícios da seção 2 do livro LARSON,
       R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010, disponível na
       Biblioteca Virtual.


       Confira as respostas que estão no final do livro e, se tiver alguma dúvida, entre
       em contato com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




Estatísticas descritivas usando a planilha eletrônica Excel

O Excel oferece funções para calcular as medidas estatísticas. Contudo, serve apenas
para dados não agrupados.


Suponha que os dados estejam digitados nas células B1 até a célula B50, como mostra
a tabela a seguir.


                                                              Comando

                 Frequência absoluta                         =CONT.SE
                         Média                            =MEDIA(B1:B50)



                                              86

## Página 87

                 Mediana                          =MED(B1:B50)

                  Moda                           =MODO(B1:B50)

          Variância amostral                     =VAR.A(B1:B50)

        Variância populacional                   =VAR.P(B1:B50)

        Desvio-padrão amostral                =DESVPAD.A(B1:B50)

      Desvio-padrão populacional              = DESVPAD.P(B1:B50)




                                 MIDIATECA

Para ampliar o seu conhecimento veja o material complementar da Unidade 1,
disponível na midiateca.




                               NA PRÁTICA

No portal do Conselho Regional de Estatística da 3ª Região (SP), publicado no
dia 23 de maio de 2020 por Camila Soares, encontramos uma série de entre-
vistas com profissionais que falam sobre o uso da estatística no cotidiano e
mostram como ela é fundamental na tomada de decisões, além de contribuir
com outras ciências e com o mercado de trabalho dessa área.


Veja alguns trechos dessa reportagem:


Exemplos da utilização da Estatística em Customer Relationship Manage-
ment CRM
O que é Customer Relationship Management (CRM)? O CRM é um termo que
designa o gerenciamento do relacionamento com clientes, em geral por meio
de um sistema.




                                     87

## Página 88

Como funciona na prática? Não basta ter um banco de dados ou conjunto de
dados com informações dos clientes. Também não basta fazer alguns cálculos
em cima desses bancos de dados. É preciso entender a estratégia de negócios
da empresa e como opera o mercado. Além disso, é necessário dispor de técni-
cas para analisar os dados. É nessa parte que a formação em Estatística é mui-
to relevante quanto ao uso de técnicas para analisar esses bancos de dados.
Isso ajuda a empresa a alcançar melhores resultados.


Exemplo da utilização em Analytics
O que é Analytics? Habilidade de utilizar dados, análises e raciocínio sistemáti-
co para conduzir um processo de tomada de decisão mais eficiente.
Como funciona na prática? Uma unidade hospitalar que cuida de bebês pre-
maturos utiliza uma análise em tempo real com base em gravação de cada
respiração e cada batimento cardíaco de todos os bebês em sua unidade. Em
seguida, analisa os dados para identificar padrões. Com base na análise do sis-
tema é possível prever infecções 24 horas antes de o bebê apresentar sintomas
visíveis. Isso permite uma intervenção precoce e tratamento, o que é vital em
bebês prematuros.


Exemplos da utilização da Estatística na área de Meio Ambiente
Como funciona na prática? Por meio dos conhecimentos é possível fazer uma
avaliação do efeito do uso do ar-condicionado no consumo de combustível e
nas emissões dos automóveis. Também são os estatísticos que auxiliam na
criação de escore para divulgação de dados de qualidade da areia das praias.


Exemplos da utilização da Estatística na área de Qualidade
Como funciona na prática? Toda empresa, seja pequena ou grande, indústria
ou serviço quer oferecer um trabalho de qualidade. Nesse ponto, o estatístico
pode atuar em diversas etapas dos processos de uma empresa para garantir
a qualidade e para isso é fundamental conhecer o negócio e suas metas. A
empresa deve ter claro o que considera qualidade, qual o nível de qualidade
esperado e a qual custo. O estatístico pode auxiliar nestas decisões por meio
de indicadores de negócio, pesquisas de satisfação, entre outros. Além disso,
pode medir a qualidade no processo de fabricação do produto, na qualidade
dos fornecedores, na logística, nos sistemas de medição a partir de técnicas
específicas para cada tipo de atividade. Dessa maneira, os gestores poderão
fazer os ajustes necessários nos processos em questão. Alguns exemplos de
metodologias são: CEP (controle estatístico do processo), MSA (Análise do sis-




                                       88

## Página 89

tema de medição), modelos de otimizações na logística da empresa, os proje-
tos de seis sigma, planejamentos e experimentos. Os testes de confiabilidade
de produtos também têm papel importante na qualidade. Muitas empresas se
certificam em normas técnicas de qualidade e o estatístico pode tornar-se au-
ditor, aplicando metodologias e raciocínio estatístico para testar se a empresa
está atuando conforme a norma estabelece.

Fonte: http://www.conre3.org.br/portal/dia-do-estatistico-profissionais-falam-sobre-
-o-uso-da-estatistica-no-cotidiano/. Acesso em: 5 jul. 2020. Acesso em: 5 jul. 2020.




                                        89

## Página 90

Resumo da Unidade 1

Iniciamos o Tópico 1 com um histórico sobre a Estatística no mundo e no Brasil. A seguir
apresentamos conceitos importantes para o bom entendimento da disciplina.


No Tópico 2 abordamos as formas de coletar, organizar e apresentar os dados pesqui-
sados por meio de tabelas e gráficos. Conhecemos dois tipos importantes de tabelas
estatísticas, com intervalos e sem intervalos de classes e suas representações gráficas
nos histogramas, polígonos de frequência e Ogiva d Galton.


No Tópico 3 aprendemos a calcular as medidas estatísticas, tanto as medidas de posição
média, mediana e moda, quanto as medidas de posição amplitude, desvio médio, variân-
cia, desvio-padrão e coeficiente de variação. Fizemos os mesmos cálculos para as três
situações possíveis: dados agrupados, dados agrupados em tabelas sem intervalo de
classes e dados agrupados em tabelas com intervalos de classes.


Também vimos os softwares estatísticos mais usados e algumas funções do Excel que
auxiliam na Estatística descritiva.


Por fim, mostramos que tabelas e gráficos de dados podem ser encontrados com gran-
de frequência no nosso dia a dia, relatórios anuais, artigos de jornais, revistas e estudos
de pesquisa. Sendo assim, é muito importante entender como eles são organizados e,
principalmente, como são interpretados.




                                            90

## Página 91

Referências

ATIVOS DIGITAIS. Envato Elements. Disponível em: https://elements.envato.com/pt-br/.
Acesso em: 28 jul. 2020.


BRASILEIRÃO de 1959 a 2019. Esporte Interativo. Disponível em: https://www.esporte-
interativo.com.br/futebolbrasileiro/GRAFICO-So-Paulo-lideraria-ranking-de-Brasileiro-de-
-todos-os-tempos-20200407-0040.html. Acesso em: 28 jul. 2020.


BOLETIM da Receita Federal – Impactos da Covid-19. Disponível em: http://www.receita.
economia.gov.br/dados/boletim-da-receita-federal_impactos-da-covid-19/boletim-da-
-receita-federal-impactos-da-covid-19z. Acesso em: 28 jul. 2020.


CONHEÇA o Brasil – População – Quantidade de homens e mulheres. IBGE Educa –
Jovens. Disponível em: https://educa.ibge.gov.br/jovens/conheca-o-brasil/populacao/
18320-quantidade-de-homens-e-mulheres.html. Acesso em: 28 jul. 2020.


GLOBAL Coronavirus cases. YouTube. Disponível em: https://youtu.be/Ovb2zc5M21g.
Acesso em: 30 jul. 2020.


INQUÉRITO domiciliar para monitorar a soroprevalência da infecção pelo vírus SARS-
-CoV-2 em adultos no município de São Paulo. Ibobe. Disponível em: https://www.ibo-
peinteligencia.com/arquivos/Apresentação%20SoroEpi%20-%20resultados%20fase%20
2%20-%20FINAL.pdf. Acesso em: 18 jul. 2020.


KOKOSKA, S. Introdução à estatística: uma abordagem por resolução de problemas.
Rio de Janeiro: LTC, 2013.


LARSON, R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010.


MOORE, D. A Estatística Básica e sua prática. Rio de Janeiro: Ed. LTC, 2000.


NORMAS de apresentação tabular. Fundação Instituto Brasileiro de Geografia e Estatísti-
ca . Rio de Janeiro: IBGE, 1979. Disponível em: https://biblioteca.ibge.gov.br/visualizacao/
livros/liv82498.pdf. Acesso em: 18 jul. 2020.




                                             91

## Página 92

NÚMERO de matrículas na Educação Superior: 2008 – 2018. Inep. Disponível em: http://
download.inep.gov.br/educacao_superior/censo_superior/documentos/2019/censo_da_
educacao_superior_2018-notas_estatisticas.pdf. Acesso em: 28 jul. 2020.


NÚMEROS mais sorteados da Mega-Sena. Só Matemática. Disponível em: https://www.
somatematica.com.br/megasenaFrequentes.php. Acesso em: 20 jul. 2020.


PAÍSES por números de homicídios. YouTube. Disponível em: https://youtu.be/Hyuv-
jXHoUi4. Acesso em: 20 jul. 2020.


PERCENTUAL do número de funções docentes em exercício, por organização acadêmi-
ca, segundo o grau de formação. Brasil, 2017. Disponível em: http://download.inep.gov.br/
educacao_superior/censo_superior/resumo_tecnico/resumo_tecnico_censo_da_educa-
cao_superior_2017.pdf. Acesso em: 28 jul. 2020.


PORTAL do IBGE: Disponível em: https://www.ibge.gov.br/. Acesso em: 20 jul. 2020.


RANKING de desenvolvimento humano – 2015. Folha de São Paulo. Disponível em:
http://arte.folha.uol.com.br/graficos/vunXC/. Acesso em: 28 jul. 2020.


SALÁRIOS: cargos e salários. Salário. Disponível em: https://www.salario.com.br. Acesso
em: 20 jul. 2020.


SOARES, C. Profissionais falam sobre o uso da Estatística no Cotidiano. CONRE-3.
23/05/2020. Disponível em: http://www.conre3.org.br/portal/dia-do-estatistico-profis-
sionais-falam-sobre-o-uso-da-estatistica-no-cotidiano/. Acesso em: 28 jul. 2020.


SPIEGEL, Murray R.; STEPHENS, Larry J. Estatística. 4. ed. Porto Alegre: Bookman, 2009


SWEENEY, Dennis J.; WILLIAMS, Thomas A.; ANDERSON, David R. Estatística aplicada
à administração e economia. 3. ed. São Paulo: Cengage Learning, 2013.




                                           92

## Página 93

UNIDADE 2

Probabilidade

## Página 94

                                               INTRODUÇÃO


O mundo em que vivemos exige tomada de decisão, capacidade de lidar com o impre-
visto e respostas para perguntas ainda desconhecidas. O estudo das teorias das pro-
babilidades é de grande importância como suporte para tomadas de decisão, quando
operamos com incertezas.


Se algo pode ter vários resultados e não sabemos ao certo qual ocorrerá, podemos usar
a probabilidade para descrever a chance numérica de cada um dos resultados possíveis.
Se soubermos que amanhã terá 95% de chance de chover, não vamos nos programar
para passar o dia na praia. Seja na vida cotidiana ou nos negócios, ter meios de quanti-
ficar numericamente possibilidades é uma ferramenta extremamente importante para
decidirmos os passos apropriados a serem tomados.


A teoria das probabilidades possui aplicação nos estudos relacionados às ciências
exatas, biológicas e humanas, bem como em qualquer área que envolva o conceito de
chance. Nesta unidade vamos estudar a definição e os conceitos básicos para o estudo
das probabilidades.


O que veremos nos tópicos, a saber:


    • Tópico 1 – Os conceitos fundamentais, como experimentos aleatórios, experi-
    mentos determinísticos, espaço amostral e os tipos de eventos mais usuais. Tam-
    bém vamos recordar algumas técnicas de contagem da análise combinatória.
    • Tópico 2 – O conceito e os cálculos das probabilidades abordando três situações:
    a probabilidade teórica ou clássica para eventos equiprováveis, a probabilidade para
    eventos não equiprováveis e a visão frequencialista.
    • Tópico 3 – Os conceitos da probabilidade condicional e o notável Teorema de Bayes.


O estudo das probabilidades também será muito importante para entendermos os con-
ceitos da Estatística Inferencial, que iremos estudar nas próximas unidades.




                                          94

## Página 95

                                OBJETIVO

Nesta unidade você será capaz de:

• Utilizar o conceito de probabilidade e seus axiomas e teoremas na análise de
situações de cunho prático.




                                     95

## Página 96

Introdução à probabilidade

No dia a dia frequentemente tomamos decisões quando temos dúvidas.

             Como será aceitação de um produto lançado no mercado?
                 Qual será a previsão do tempo na próxima semana?
             Qual a chance de acertar os seis números da Mega-Sena?
         Qual a possibilidade de obter bons lucros no mercado de ações?
        Qual o risco que vou correr ao efetuar certo procedimento cirúrgico?

É conveniente, então, possuirmos uma medida numérica. Sim, um número que revele a
incerteza presente em cada um destes e de muitos outros questionamentos. Essa medi-
da numérica é a probabilidade.



         Ampliando o foco


       A palavra probabilidade deriva do latim probare, que significa “testar”.




Iniciaremos o tópico apresentando os conceitos básicos que auxiliam nos cálculos
das probabilidades.

Experimento

É todo o fenômeno que acontece ou toda ação que iremos realizar.

De acordo com a natureza do fenômeno que pretendemos estudar, podemos classificá-lo
como aleatório ou determinístico.

    1. Experimento aleatório

    • Cada experimento poderá ser repetido, sob as mesmas condições, indefinidamente.
    • O resultado particular de cada experimento aparecerá ao acaso, mas pode-se
    descrever todos os possíveis resultados.
    • Quando o experimento for repetido muitas vezes, aparecerá regularidade
    nas respostas.


                                               96

## Página 97

    Exemplo


  Sejam os experimentos:


  a) Lançamento de um produto no mercado.
  Podemos prever quais são os possíveis resultados: ser bem aceito, não ser
  bem aceito, ser indiferente.



      Porém, não sabemos antecipadamente se esse produto será bem
      aceito ou não.



  Se fizermos o lançamento desse produto no mercado inúmeras vezes, pode-
  mos perceber regularidade na resposta.
  b) Fazer um contato de vendas: possíveis resultado  Vender ou não vender.
  c) Jogar uma partida de futebol: possíveis resultados  Ganhar, perder
  ou empatar.
  d) Lançar um dado e observar a face de cima: possíveis resultados  1, 2, 3, 4, 5, 6.




2. Experimento determinístico

• É exatamente o contrário do experimento aleatório.
• Nos experimentos determinísticos já conhecemos o resultado da experiência, pois
ele será sempre o mesmo, desde que as mesmas condições e parâmetros iniciais
sejam mantidos.
• Os experimentos determinísticos não são estudados pela Teoria das Probabilidades,
já que não se trata de incertezas.




    Exemplo


  Soma dos ângulos internos de um triângulo  Sabemos que o resultado será
  sempre 180º. Se repetirmos a experiência de escolher vários triângulos, medir-
  mos os três ângulos internos e efetuarmos a soma, o resultado será sempre
  igual a 180º.




                                           97

## Página 98

Espaço amostral: (s ou Ω)

São todos os resultados possíveis de acontecer, quando se realiza um experimento.
É representado pela letra S (space) ou pela letra grega Ω (ômega). Cada resultado do
espaço amostral é considerado um ponto amostral.

O conceito de espaço amostral é muito importante, pois, na maioria dos casos, será
a ferramenta principal para efetuarmos o cálculo das probabilidades.

Observe alguns exemplos na tabela a seguir.

Veja o experimento na coluna esquerda da tabela e especifique todos os elementos do
espaço amostral. Depois confira a sua resposta na coluna à direita.



                                                       Espaço amostral (s ou Ω)
                    Experimento
                                                         Possíveis resultados


                                                   Ser defeituosa.
  Selecionar uma peça para ser inspecionada.
                                                   Não ser defeituosa.


                                                   Céu ensolarado.
                                                   Céu nublado.
                                                   Ventania.
  Verificar a previsão do tempo.                   Chovendo.
                                                   Chuva com trovões.
                                                   Granizo.
                                                   Neve.


                                                   Ser bem-sucedido.
  Realizar um procedimento cirúrgico.
                                                   Não ser bem-sucedido.



                                                   Congestionado.
                                                   Com lentidão.
  Verificar as condições do trânsito.
                                                   Com retenções.
                                                   Boas condições de tráfego.




                                           98

## Página 99

                                                    Ganhar.
  Jogar uma partida de futebol.                     Perder.
                                                    Empatar.


                                                    Efetuar a venda.
  Fazer um contato de vendas.
                                                    Não efetuar a venda.


                                                    Sair o número 1.
                                                    Sair o número 2.
                                                    Sair o número 3.
  Lançar um dado e observar a face superior.
                                                    Sair o número 4.
                                                    Sair o número 5.
                                                    Sair o número 6.




Evento

Todas as perguntas que formulamos a respeito do experimento são denominadas
“eventos”, que são denotados por letras maiúsculas.

Vejamos alguns tipos de eventos. São eles:

    • Evento elementar ou evento simples: é aquele formado por um único elemento
    do espaço amostral.
    • Evento composto: é aquele formado por dois ou mais elementos do
    espaço amostral.
    • Evento certo: é aquele que ocorre sempre, ou seja, os resultados favoráveis ao
    evento apresentam os mesmos elementos do espaço amostral.
    • Evento impossível: é aquele que nunca ocorre, ou seja, os resultados favoráveis
    ao evento serão um conjunto vazio.
    • Evento soma ou união: é aquele em que pelo menos um dos eventos ocorre, “Um
    ou outro”.
    • Evento produto ou interseção: é aquele em que os eventos ocorrem simultanea-
    mente, “Um e outro”.
    • Eventos mutuamente exclusivos, ou mutuamente excludentes: são eventos em
    que "Um ou outro" não poderá acontecer.


                                               99

## Página 100

    • Evento complementar: são eventos mutuamente exclusivos, que unidos têm
    os mesmos elementos do espaço amostral. É denotado com uma barra acima do
    nome do evento.
    • Eventos equiprováveis: são eventos em que todos os elementos do espaço
    amostral têm a mesma probabilidade de ocorrência.
    • Eventos não equiprováveis: são eventos em que os elementos do espaço amos-
    tral não têm a mesma probabilidade de ocorrência.
    • Eventos independentes: dois eventos são independentes quando a ocorrência de
    um evento não influência a ocorrência do outro.
    • Eventos condicionados: são aqueles em que o acontecimento de um está condi-
    cionado ao acontecimento de outro, ou seja, acontece um se o outro já aconteceu.

Agora que já conhecemos os conceitos básicos da probabilidade, apresentaremos alguns
exemplos ligando esses quatro conceitos na tabela a seguir.



 Experimento     Espaço Amostral                   Evento               Tipo de Evento



                                     Quais são as chances, ao lançar
                                     um dado, de observarmos na
                                     face superior um número maior
                                     do que 5?
                                                                        Evento
                 Ω = {1,2,3,4,5,6}   A = face superior maior do que 5   Elementar
                                     (Escolhemos no espaço amos-        ou simples
                                     tral os elementos que são favo-
                                     ráveis ao evento).


   Jogar um                          A={6}
     dado

                                     Quais são as chances, ao lan-
                                     çar um dado, de observarmos
                                     na face superior um número
                                     primo?
                                                                        Evento
                 Ω = {1,2,3,4,5,6}
                                                                        composto
                                     B = face superior ser um nú-
                                     mero primo

                                     B = {2,3,5}




                                         100

## Página 101

                               Quais são as chances, ao lan-
                               çar um dado, de observarmos
                               na face superior um número
                               menor ou igual a 6?
           Ω = {1,2,3,4,5,6}                                   Evento certo
                               C=face superior ser um núme-
                               ro menor ou igual a 6

                               C={1,2,3,4,5,6}




                               Quais são as chances, ao lan-
                               çar um dado, de observarmos
                               na face superior um número
                               maior do que 6?
                                                               Evento
           Ω = {1,2,3,4,5,6}
                                                               impossível
                               D=face superior ser um núme-
                               ro maior do que 6

Jogar um
                               D={ }
  dado




                               (a partir daqui vamos omitir
                               a pergunta e já escrever o
                               evento)

                               E=observar na face superior
                               um número maior do que 4

                               E={5,6}
           Ω = {1,2,3,4,5,6}
                                                               Evento união
           Ω = {1,2,3,4,5,6}
                               ou

                               F=observar na face superior
                               um número menor do que 3

                               F={1,2}

                               G=E ∪ F = {1,2,5,6}




                                    101

## Página 102

                               H=observar na face superior
                               um número maior do que 2

                               H={3,4,5,6}

                               e
           Ω = {1,2,3,4,5,6}                                   Evento
           Ω = {1,2,3,4,5,6}                                   interseção
                               I=observar na face superior
                               um número menor do que 5

                               I={1,2,3,4}

                               J=H ∩ I = {3,4}



                               K= observar na face superior
                               um número menor do que 3
                                                               Evento
                                                               mutuamente
                               K={1,2}
                                                               exclusivos

                               L= observar na face superior
Jogar um   Ω = {1,2,3,4,5,6}                                   (a interseção
                               uma face maior do que 3
  dado     Ω = {1,2,3,4,5,6}                                    é nula,
                                                               mas juntos
                               L={4,5,6}
                                                               não formam
                                                               o espaço
                               K ∩ L ={ }
                                                               amostral).

                               K ∪ L = {1,2,4,5,6}



                               M= observar na face superior
                               um número par
                                                               Eventos
                                                               complemen-
                               M={2,4,6}
                                                               tares

                                 = observar na face superior
           Ω = {1,2,3,4,5,6}                                   (a interseção
                               uma face ímpar
           Ω = {1,2,3,4,5,6}                                   é nula,
                                                               e juntos
                                   = {1,3,5}
                                                               formam
                                                               o espaço
                               M∩ ={ }
                                                               amostral).

                               M ∪ = {1,2,3,4,5,6} = Ω




                                    102

## Página 103

Para apresentarmos exemplos de eventos equiprováveis, não equiprováveis, independen-
tes e condicionados precisamos primeiro estudar como calculamos as probabilidades, o
que veremos a seguir.


Regras de contagem, combinações e permutações

Para calcularmos as probabilidades, muitas vezes será necessário identificar e contar o
número de elementos do espaço amostral.


Vamos apresentar três modos de contagem muito usados, que são: experimentos em
múltiplas etapas, arranjos, combinação e permutação.


     1. Experimentos em múltiplas etapas


Se um experimento pode ser descrito como uma sequência de dois estágios sucessivos
e independentes, em que o primeiro estágio pode ocorrer de “m” modos distintos, em
seguida o segundo estágio pode acontecer de “n” modos distintos. Nessas condições,
dizemos que “o número de maneiras distintas de ocorrer esse acontecimento é igual ao
produto m . n”.


Veja um exemplo:


Um estudante recebeu o resultado do seu desempenho no Enem e deve escolher um cur-
so e o tipo de faculdade que deseja cursar. Sabe-se que ele tem preferência por cinco cur-
sos: Medicina (M), Nutrição (N), Odontologia (O), Biomedicina (B) e Veterinária (V). Cada
curso pode ser feito em três tipos de faculdades: particular (P), estadual (E) ou federal (F).
Qual é o número de opções que o estudante poderá escolher para fazer sua matrícula?


Resolução:


De acordo com o princípio fundamental da contagem, o número total de opções que o
estudante pode fazer é 5 . 3 = 15 .


Se quisermos ilustrar as 15 opções podemos lançar mão de um recurso muito útil, que
se chama “árvore de possibilidades”.




                                             103

## Página 104

                              Figura: Árvore das Possibilidades.


                             Curso         Faculdade       Resultado




                                                P              MP

                               M                E              ME

                                                F              MF

                                                P              NP

                                N               E              NE

                                                F              NF

                                                P              OP

                                O               E              OE

                                                F              OF

                                                P              BP

                                B               E              BE

                                                F              BF

                                                P              VP

                                V               E              VE

                                                F              VF


                             Fonte: Elaborada pela autora (2020).




Fazendo a contagem das respostas anteriores temos também as 15 possibilidades.



        Podemos generalizar o princípio fundamental da contagem. Quando um
        acontecimento é composto por “k” estágios sucessivos e independentes,
        com, respectivamente, n1, n2, n3, n4, ..... , nk modos distintos de ocorrên-
        cia, o número total de maneiras distintas de ocorrer esse acontecimento é
        n1. n2 . n3 . n4 . ..... . nk.




                                             104

## Página 105

Veja mais alguns exemplos:


Exemplo 1


Um casal planeja ter três filhos.
    a) De quantas maneiras diferentes a família poderá ser formada, considerando-se
    que nascerá um filho em cada gestação.
    b) Quais são essas possibilidades?


Resolução:


    a) Para responder de quantas maneiras diferentes serão, basta usar o princípio fun-
    damental da contagem:
    1ª gestação: 2 opções – O filho ser do sexo masculino ou do sexo feminino.
    2ª gestação: 2 opções – O filho ser do sexo masculino ou do sexo feminino.
    3ª gestação: 2 opções – O filho ser do sexo masculino ou do sexo feminino.
    Logo: 2 . 2. 2 = 8 maneiras.
    b) Para responder quais são as possibilidades vamos desenhar a árvore
    de possibilidades:
    Vamos simbolizar: M  sexo masculino e F  sexo feminino


                                Figura: Árvore das Possibilidades

                  1ª Gestação      2ª Gestação      3ª Gestação     Resultado




                                                        M            MMM
                                        M
                                                        F            MMF
                      M
                                                        M            MFM
                                        F
                                                        F             MFF

                                                        M            FMM
                                        M
                                                        F             FMF
                       F
                                                        M             FFM
                                        F
                                                        F             FFF

                           Fonte: Elaborada pela autora (2020).



                                              105

## Página 106

Exemplo 2


Quantos números diferentes e de três algarismos distintos existem no sistema decimal
de numeração?


Resolução:



      Classes                    C                          D                         U




                                                      Supondo que              Supondo que
                                                       escolhemos               escolhemos
                         1, 2, 3, 4, 5, 6, 7,
                                                     o algarismo “1”          o algarismo “2”
                                8, 9
                                                   para as centenas,         para as dezenas,
                         Lembrando que
                                                   ele não poderá ser       ele não poderá ser
                         o algarismo das
                                                       usado aqui,              usado aqui,
                        centenas tem que
                                                    já que os algaris-       já que os algaris-
                         ser diferente de
                                                   mos são distintos.       mos são distintos
                          zero para que
                                                   Logo, na casa das        Logo, na casa das
                         o número tenha
                                                  dezenas podem ser          unidades podem
                           três dígitos.
                                                  os dígitos: 0, 2, 3, 4,      ser os dígitos:
                                                        5, 6, 7, 8, 9        0, 3, 4, 5, 6, 7, 8, 9


      Total de
                                 9                          9                         8
   possibilidades


Pelo princípio fundamental da contagem temos: 9 . 9 . 8 = 648 possibilidades.


Exemplo 3


Em uma indústria, uma peça deve passar por três estações para receber sua classificação.


Estação 1: classifica em ruim, bom ou excelente.


Estação 2: classifica em atrasado ou no prazo.


Estação 3: classifica em esteira 1, esteira 2, esteira 3 ou esteira 4.


Calcule o número de classificações completas que a peça pode receber.


                                                106

## Página 107

Resolução:


Pelo princípio fundamental da contagem temos: 3 . 2 . 4 = 24


Cálculos que envolvam fatorial


O fatorial de um número natural “n” é representado por n! (lê-se n fatorial) e definido por:
     1) n! = n . (n – 1) . (n – 2) . (n – 3) .... . 2 . 1, para n ≥ 2
     2) 1! = 1
     3) 0! = 1


Para ilustrar, veja um exemplo.


           Exemplo


         a) 4! = 4 . 3 . 2 . 1 = 24

                       8!               8.7.6.5!
         b)                      =                     como estamos multiplicando os fatores no
                     5!.3!              5!.3.2.1

         numerador e no denominador, podemos parar de escrever a qualquer momento,
         colocar o símbolo de fatorial (!) e depois efetuar a simplificação, não havendo
         necessidade de desenvolver até o 1.

                8!                   8.7.6.5!
                             =                     =   56
               5!.3!                 5!.3.2.1




     2. Arranjos simples


Usados em problemas de contagem em que os agrupamentos são considerados dis-
tintos quanto a ordem e natureza de seus elementos. Por exemplo: vamos considerar o
número formado pelos algarismos 1 e 2. Seja o número 12. Se invertermos a ordem o
número será 21, que é diferente de 12, ou seja, formamos agrupamentos diferentes quan-
do invertemos a ordem dos elementos.


Fórmula a ser usada:

                 n!
An,k =
              (n – k)!



                                                            107

## Página 108

Veja alguns exemplos a seguir:



        Exemplo 1


       Cinco cavalos disputam um páreo. Quantos são os possíveis resultados para as
       três primeiras colocações.


       Resolução: Vamos formar um agrupamento com três cavalos escolhidos em
       um total de cinco cavalos.


           Como a ordem importa nesse agrupamento, pois, se invertemos a posição
           dos cavalos mudamos o sentido do agrupamento, vamos usar a fórmula
           dos arranjos.


                           n!
           An,k   =
                       (n – k)!


                           5!            5.4.3.2.1
           A5,3   =                 =                     =   60
                       (5 – 3)!             2.1




        Exemplo 2


       A senha de um cofre é composta por uma sequência de três dígitos diferentes.
       Calcule o número máximo de tentativas para que a senha seja descoberta.


       Resolução: Vamos formar um agrupamento de três dígitos distintos, escolhidos
       nos 10 dígitos disponíveis no nosso sistema de numeração.


           Como a ordem importa nesse agrupamento, já que ao invertemos os dígi-
           tos, as senhas ficam diferentes, vamos usar a fórmula do arranjo simples.


                           n!
           An,k   =
                      (n – k)!
                           10!          10 . 9 . 8 . 7!
           A10,3 =                  =                     = 720
                      (10 – 3)!               7!




                                               108

## Página 109

            Importante



           1 - Quando o tamanho do agrupamento é igual ao número de elementos, em
           vez de arranjo chamamos de Permutação.


                                                       n!
                                      An,k   =
                                                   (n – k)!


           2 - Qualquer problema que envolva permutações ou arranjo simples pode ser
           resolvido diretamente pelo princípio fundamental da contagem.




       3. Combinação simples


Usadas em problemas de contagem em que os agrupamentos são considerados dis-
tintos quanto a ordem e natureza. A ordem dos elementos não altera o agrupamento.
Vamos selecionar seis números para jogar na Mega-Sena. Se alterarmos a ordem que
vamos marcar no cartão de jogos, a aposta será a mesma, ou seja, formamos agrupa-
mentos iguais quando invertemos a ordem dos elementos.


Fórmula a ser usada:

                 n!
Cn,k   =
              (n – k)!




                                                 109

## Página 110

  Exemplo


Quantos cartões diferentes podemos marcar na Mega-Sena?


Sabemos que a ordem que você marca no cartão dos números escolhidos não
importa nesse agrupamento. Logo, dos 60 números disponíveis, vamos esco-
lher seis deles e calcular a combinação, já que a ordem não importará.


                 n!
Cn,k    =
            k! . (n – k)!


                   60!
C60,6   =
             6! . (60 – 6)!


                   60!              60.59.58.57.56.55.54
C60,6   =                       =
                 6! . 54!              6.5.4.3.2.1.54!


                60.59.58.57.56.55
C60,6   =                             = 50.063.860
                      6.5.4.3.2.1




  Ampliando o foco


Para colocar em prática o conteúdo visto nesta unidade visite a Biblioteca Vir-
tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
para reforçar o conhecimento.


Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                       110

## Página 111

Conceitos de probabilidade

Neste tópico vamos apresentar conceitos importantes para efetuarmos o cálculo
das probabilidades.


Seja um experimento aleatório e “Ω” um espaço amostral associado a esse experimento.
A cada evento “A” associamos um número real, representado por P(A) e denominado
probabilidade de A.


Podemos efetuar o cálculo das probabilidades considerando três cenários diferentes.
São eles:


       • 1º cenário: probabilidade teórica ou clássica com eventos equiprováveis.
       • 2º cenário: probabilidade teórica com eventos não equiprováveis.
       • 3º cenário: probabilidade empírica ou estatística, usando frequência relativa.


Probabilidade teórica ou clássica com eventos equiprováveis

Na definição clássica de probabilidade trabalhamos apenas com eventos equiprováveis,
que, como vimos, são os eventos em que todos os elementos do espaço amostral têm
a mesma chance de ocorrer.


Para calcular a probabilidade basta efetuar a divisão entre o número de elementos dos
casos favoráveis ao evento “A” pelo número de elementos possíveis representados no
espaço amostral “Ω”.

               números de casos favoráveis             n(A)
P(A)     =                                         =
               números de casos possíveis              n (Ω)


Podemos apresentar o resultado das probabilidades de três maneiras, a saber:


       1. Como uma fração.
       2. Como um número racional, se efetuarmos a divisão entre o numerador e o deno-
       minador da fração.
       3. Como uma porcentagem, se multiplicarmos por 100 o resultado da divisão.




                                             111

## Página 112

Exemplo 1


Em um lote de 12 peças, quatro são defeituosas. Sendo retirada uma peça para inspeção, calcu-
le a probabilidade de essa peça ser defeituosa.



  Antes de iniciar o cálculo
     precisamos seguir                                       Respostas
    as seguintes etapas:


   1 - Qual é o
                                 Avaliar defeitos em peças produzidas.
   experimento?


                                 Ω = { D, D, D, D, D,D,D,D,D,D,D,D}
                                 onde:
   2 - Qual é o espaço           D = peça defeituosa
   amostral?
                                 D = eça não defeituosa
                                 12 peças, sendo 4 defeituosas e 8 perfeitas
                                 12 elementos, então: n (Ω) = 12


   3 - Qual é o evento?          A = retirar uma peça defeituosa


   4 - Retire do espaço          D, D, D, D 4 peças defeituosas; 4 elementos, então:
   amostral os elementos
   favoráveis ao evento.         n (A) = 4


   5 - Cada elemento
   do espaço amostral
   tem a mesma chance
   de ocorrência?

   (Se a resposta for
                                 Sim, pois não há nenhuma informação de que alguma peça
   “Sim”, então vamos
                                 tenha maior chance de ser selecionada do que outra.
   calcular
   a fórmula da
   probabilidade clássica.
   Se for “Não”, iremos
   usar o cálculo empírico
   ou frequencial.)

   6 - Fazer o cálculo
                                          números de casos favoráveis       n(A)        4
   usando o método               P(A) =                                 =           =        = 0,333 = 33%
                                          números de casos possíveis        n (Ω)       12
   apropriado.




                                                112

## Página 113

Exemplo 2


No lançamento de um dado “honesto”, qual a probabilidade de obter um número par na
face superior?



           Etapas                                          Respostas


   1 - Qual é o
                               Lançamento de um dado.
   experimento?




   2 - Qual é o espaço         As faces {1, 2, 3, 4, 5, 6}, portanto são seis elementos, então:
   amostral?                   n (Ω) = 6




   3 - Qual é o evento?        A = observar na face superior um número par.


   4 - Retirem do espaço
   amostral os elementos       A = {2, 4, 6}, portanto são três elementos, então: n (A) = 3
   favoráveis ao evento.


   5 - Todos os elemen-
   tos do espaço amostral
   têm a mesma chance
   de ocorrência?

   (Se a resposta for “Sim”,   Sim, pois temos a informação de que o dado é honesto, ou
   então vamos calcular        seja, todas as faces têm a mesma probabilidade de sair.
   a fórmula da
   probabilidade clássica.
   Se for “Não”, iremos
   usar o cálculo empírico
   ou frequencial.)


   6 - Fazer o cálculo usan-
                                        números de casos favoráveis       n(A)        3
   do o método                 P(A) =                                 =           =       = 0,5= 50%
                                        números de casos possíveis        n (Ω)       6
   apropriado.




                                              113

## Página 114

Exemplo 3


Dois dados honestos são lançados e a face superior é observada. Determine a probabili-
dade de a soma das duas faces superiores ser igual a oito.



          Etapas                                              Respostas


  1 - Qual é o
                              Lançar dois dados.
  experimento?


                              Ω = {(1,1), (1,2),         (1,3),   (1,4),   (1,5),      (1,6),

                                       (2,1),   (2,2),   (2,3),   (2,4),   (2,5),      (2,6),
                                       (3,1),   (3,2),   (3,3),   (3,4),   (3,5),      (3,6),
  2 - Qual é o espaço
                                       (4,1),   (4,2),   (4,3),   (4,4),   (4,5),      (4,6),
  amostral?
                                       (5,1),   (5,2),   (5,3),   (5,4),   (5,5),      (5,6),
                                       (6,1),   (6,2),   (6,3),   (6,4),   (6,5),      (6,6)}

                              n (Ω) = 36


                              A = soma dos números das duas faces superiores
  3 - Qual é o evento?
                              ser igual a 8.

  4 - Retirem do espaço
                              A = {(2,6), (3,5), (4,4), (5,3), (6,2)}, portanto são cinco elemen-
  amostral os elementos
                              tos, então: n (A) = 5
  favoráveis ao evento.


  5 - Todos os elemen-
  tos do espaço amostral
  têm a mesma chance
  de ocorrência?

  (Se a resposta for “Sim”,
                              Sim, pois os dados são honestos.
  então vamos calcular
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)


  6 - Fazer o cálculo usan-
                                       números de casos favoráveis             n(A)             5
  do o método                 P(A) =                                       =             =           = 0,139 = 13,9%
                                        números de casos possíveis             n (Ω)            36
  apropriado.



                                                114

## Página 115

Exemplo 4


Determine a probabilidade de pelo menos uma face “cara” aparecer, na face superior, no lança-
mento de duas moedas “não viciadas”.



           Etapas                                           Respostas



   1 - Qual é o
                                Lançamento de duas moedas .
   experimento?


   2 - Qual é o espaço          Ω = {(cara, cara), (cara, coroa), (coroa, cara), (coroa, coroa)}
   amostral?                    n (Ω) = 4


   3 - Qual é o evento?         A = observar pelo menos uma cara na face superior


   4 - Retirem do espaço
                                A = {(cara, cara), (cara, coroa), (coroa, cara)}, portanto são 3
   amostral os elementos
                                elementos, então: n (A) = 3
   favoráveis ao evento.



   5 - Todos os elemen-
   tos do espaço amostral
   têm a mesma chance
   de ocorrência?

                                Sim, pois temos a informação que as moedas não
   (Se a resposta for “Sim”,
                                são “viciadas”, ou seja, as duas faces têm a mesma
   então vamos calcular
                                probabilidade de sair.
   a fórmula da
   probabilidade clássica.
   Se for “Não”, iremos
   usar o cálculo empírico
   ou frequencial.)



   6 - Fazer o cálculo usan-
                                         números de casos favoráveis       n(A)        3
   do o método                  P(A) =                                 =           =       = 0,75 = 75%
                                         números de casos possíveis        n (Ω)       4
   apropriado.




                                               115

## Página 116

Exemplo 5


A equipe de professores do Departamento de Estatística é composta de 12 pessoas, sendo 9
mulheres e 3 homens. Dessas pessoas, vamos selecionar aleatoriamente duas para participar do
Congresso de Estatística. Qual a probabilidade de a comissão ser formada por duas mulheres.




 Etapas                      Respostas



   1 - Qual é o                Selecionar dois professores do sexo feminino para partici-
   experimento?                par do Congresso de Estatística.



                               Para calcular o número de elementos do espaço amostral,
                               devemos considerar um grupo de 12 pessoas, das quais se-
                               rão retirados dois elementos, sem importar a ordem, o que
                               condiz com a combinação de 12 elementos, tomados dois
                               a dois.


   2 - Qual é o espaço                         n!
   amostral?                   Cn,k   =
                                          k! . (n – k)!

                                              12!
                               C12,2 =
                                            2! . 10!

                                           12.11.10!
                               C12,2 =                    = 66
                                            2.1.10!



   3 - Qual é o evento?        A = escolher duas mulheres



                               Para calcular o número de elementos favoráveis, vamos re-
                               tirar das nove mulheres possíveis, um agrupamento de dois,
                               logo, como não importa a ordem, vamos calcular a combi-
   4 - Retirem do espaço       nação de nove tomados dois a dois.
   amostral os elementos
   favoráveis ao evento.
                                               9!                9.8.7!
                               C9,2   =                   =               = 36
                                             2!.7!               2.1.7!




                                              116

## Página 117

  5 - Todos os elemen-
  tos do espaço amostral
  têm a mesma chance
  de ocorrência?

                               Sim, pois todas as mulheres têm a mesma chance de ser
  (Se a resposta for “Sim”,
                               selecionadas. Não há nenhuma informação sobre uma pes-
  então vamos calcular
                               soa ser predileta para a escolha.  probabilidade clássica.
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)


  6 - Fazer o cálculo usan-
                                            números de casos favoráveis       n(A)        36
  do o método                  P(A) =                                     =           =        = 0,5455 = 54,55%
                                            números de casos possíveis        n (Ω)       66
  apropriado.




Exemplo 6


Calcule a probabilidade de um jogador acertar os seis números da Mega-Sena marcando um
cartão com seis números.



Etapas                        Respostas



  1 - Qual é o
                               Jogar um cartão com 6 números na Mega-Sena .
  experimento?


                               Para calcular o número de elementos do espaço amostral,
                               devemos considerar um grupo de números, dos quais se-
                               rão escolhidos seis, sem importar a ordem em que serão
                               marcados no cartão, o que condiz com a combinação de 60
                               elementos, tomados seis a seis.
  2 - Qual é o espaço
  amostral?
                                                    n!
                               Cn,k     =
                                               k! . (n – k)!

                                                   60!
                               C60,6 =
                                                 6! . 54!




                                                   117

## Página 118

                                            60.59.58.57.56.55.54!
                              C12,2 =                                          = 50.063.860
                                                 6.5.4.3.2.1.54!


                              Sim, pode acreditar! Existem 50.063.860 cartões diferen-
                              tes que podem ser marcados escolhendo seis números de
                              00 a 60.



3 - Qual é o evento?          A = marcar os seis números que serão sorteados.



4 - Retirem do espaço
amostral os elementos         P1 cartão.
favoráveis ao evento.



5 - Todos os elemen-
tos do espaço amostral
têm a mesma chance
de ocorrência?

(Se a resposta for “Sim”,     Sim, pois todos os números têm a mesma chance de serem
então vamos calcular          sorteados.  probabilidade clássica.
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)



6 - Fazer o cálculo usan-            números de casos favoráveis       n(A)            1
                            P(A) =                                 =           =                = 0,00000002 = 0,00
do o método                          números de casos possíveis        n (Ω)       50.063.860
apropriado.




                                                118

## Página 119

Probabilidade teórica com eventos não equiprováveis

São usadas quando pelo menos um elemento do espaço amostral tiver diferentes proba-
bilidades de ocorrência dos demais.


Veja um exemplo:


Considere a roleta indicada na figura.




                                  1


                                                     2

                                  3




Vamos supor que vamos dar um impulso na seta e desejamos conhecer a probabilidade
de ela vir parar no setor circular com o número 2.


Podemos visualizar que a área do setor circular com os números 2 é diferente da área
dos setores 1 e 3. Logo, essa probabilidade deverá ser calculada para eventos com espa-
ços amostrais não equiprováveis.


Seja Ω o espaço amostral, então: Ω = {1, 2, 3} . Este espaço amostral para não será ade-
quado para resolver esse tipo de problema, já que os elementos não são equiprováveis.


Uma boa estratégia para calcular a probabilidade seria redefinir um novo espaço amos-
tral adequado, dessa vez com eventos equiprováveis.


Seja Ω1 = {1, 2, 2, 3} o novo espaço amostral, desta vez com todos os elementos equipro-
váveis e n(Ω1) = 4 elementos




                                          119

## Página 120

Agora sim, poderemos utilizar o conceito clássico da probabilidade para eventos
equiprováveis. Então:


Seja o evento A = seta parar no número 2  n(A)=2

           n (A)        2
P (A) =             =        = 0,5 = 50%
          n (Ω1 )       4


Probabilidade     empírica,                      ou    estatística,          usando
frequência relativa

A frequência relativa de uma tabela de distribuição de frequências, que apresentamos na
Unidade 1, pode ser usada para determinarmos a probabilidade de um evento.


Se um experimento for repetido um número muito grande de vezes, percebemos uma
tendência nos resultados. A proporção de ocorrência dos resultados tenderá a probabili-
dade teórica. Isso é chamado de Lei dos Grandes Números.


Logo, para calcular a probabilidade de o evento “A” ocorrer temos:

          número de vezes que o evento ocorreu
P (A) =
          número de repetições do experimento


Essa fórmula é exatamente o que fizemos em estatística descritiva ao organizar os da-
dos em uma tabela de distribuição de frequências. A coluna da frequência relativa indica
a probabilidade de ocorrência de cada resultado observado.


Vejamos um exemplo.


Seja um experimento de lançar um dado “honesto” várias vezes e observar a face supe-
rior. A tabela a seguir apresenta os resultados do número de vezes em que o número 6
foi observado na face superior.




                                           120

## Página 121

                                Lançamento do dado



                             Quantidade de vezes em
                                                            Cálculo da frequência
  Número de repetições       que apareceu o número 6
                                                                   relativa
                                 na face superior


                                                               0
            1                              0                        =   0%
                                                               1

                                                               0
            2                              0                        =   0%
                                                               2

                                                               0
            3                              0                        =   0%
                                                               3

                                                               1
            4                              1                        =   25%
                                                               4

                                                               1
            5                              1                        =   20%
                                                               5

                                                               2
            6                              2                        =   33,3%
                                                               6

                                                               3
           20                              3                        =   15%
                                                              20

                                                              16
           100                           16                         =   16%
                                                              100

                                                              165
          1000                        165                           =   16,5%
                                                             1000

                                                             1677
          10000                       1667                          =   16,67%
                                                             1000


Seja o evento A=observar a face superior igual a 6 no lançamento de um dado.


      Pela probabilidade clássica temos:

           1
P (A) =           = 0,1667    = 16,67%
           6


                                               121

## Página 122

Pela probabilidade empírica, observamos que, à medida que o número de repetições au-
menta, a frequência relativa aproxima-se da probabilidade clássica.


Quando n=10000  fr = 16,77%



         Importante


       Se o número de repetições for pequeno, haverá grandes oscilações da frequên-
       cia relativa. À medida que o número de repetições aumenta consideravelmente,
       as oscilações tendem a ser menores e tendem a estabilizar em um valor cons-
       tante, que pode ser avaliado como probabilidade. A dificuldade existente nessa
       definição é que o número limite real pode não existir na realidade. Sendo assim,
       a definição obedece à teoria matemática do limite.




Relações básicas de probabilidades

Veremos a seguir algumas relações básicas de probabilidades. Aqui, vamos estudar as
probabilidades complementares e a probabilidade na união de eventos.


Probabilidade complementar

Dois eventos, A e B, são complementares se sua intersecção é vazia e sua união é o es-
paço amostral Ω. Isto é:
A∩B=Ø
e
A ∪ B = Ω.


O complementar de um evento A é representado por A ou AC


    • Cálculo das probabilidades complementares


Se A é o complementar de A, então a soma das probabilidades de um evento com seu
evento complementar é sempre igual a 100%, ou seja, igual a 1:


P(A) + P(A) = 1      ...     P(A) = 1 - P(A)         ou     P(A) = 1- P(A)




                                               122

## Página 123

Exemplo 1


Qual a probabilidade de não sair a face 4 no lançamento de um dado?


          Etapas:                                        Respostas


  1 - Qual é o
                              Lançamento de um dado.
  experimento?

  2 - Qual é o espaço         As faces {1, 2, 3, 4, 5, 6}, portanto são seis elementos.
  amostral?                   Então: n (Ω) = 6

                              Qual a probabilidade de não sair a face 4 no lançamento de
                              um dado?
                              Evento A= sair a face 4
  3 - Qual é o evento?
                              Evento A = não sair a face 4
                              (Lembrando que a barra acima do nome do evento significa
                              a negação.)

  4 - Retirem do espaço
  amostral os elementos       A = {4}     n(A) = 1
  favoráveis ao evento.


  5 - Cada elemento
  do espaço amostral
  tem a mesma chance
  de ocorrência?

  (Se a resposta for “Sim”,
                              Sim      probabilidade clássica.
  então vamos calcular
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)


                              Usando o conceito de eventos complementares:

  6 - Fazer o cálculo usan-                               1       5
                              P ( A ) = 1 − P(A) = 1 −        =       = 0,8333 = 83,33%
  do o método                                             6       6
  apropriado.
                              Logo, existe uma probabilidade de aproximadamente 83%
                              de não sair a face 4 no lançamento de um dado.




                                            123

## Página 124

Exemplo 2


A probabilidade de uma dona de casa escolher uma determinada marca de café é de 65%. Qual
a probabilidade que em um determinado dia ela escolha outra marca?


          Etapas:                                        Respostas

  1 - Qual é o
                               Dona de casa escolher uma marca de café.
  experimento?

  2 - Qual é o espaço
                               Vamos supor 100 pessoas.
  amostral?

                               Evento A= escolher determinada marca de café.
                               Evento A = não escolher essa determinada marca, ou seja,
  3 - Qual é o evento?         escolher outra marca
                               (lembrando que a barra acima do nome do evento, significa
                               a negação)

  4 - Retirem do espaço
  amostral os elementos        n(A) = 65
  favoráveis ao evento.

  5 - Cada elemento
  do espaço amostral
  tem a mesma chance
  de ocorrência?

  (Se a resposta for “Sim”,
                               Sim        probabilidade clássica.
  então vamos calcular
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)

  6 - Fazer o cálculo usan-    Undo o conceito de eventos complementares:
  do o método
  apropriado.                  P( A ) = 1 − P(A) = 1 − 0,65 = 0,35 = 35%



A propriedade complementar é muito usada quando precisamos calcular a probabilidade
de “pelo menos” alguma ocorrência.




                                              124

## Página 125

Exemplo 3


Em um lote de 12 peças, quatro são defeituosas e oito são boas. A probabilidade de retirarmos
duas peças boas em uma inspeção é igual a 42%. Qual a probabilidade de pelo menos uma apre-
sentar defeito quando retiramos duas peças para inspeção?



          Etapas:                                         Respostas


  1 - Qual é o
                                Retirar duas peças para inspeção
  experimento?


  2 - Qual é o espaço           Não será preciso definir, pois vamos usar o conceito de pro-
  amostral?                     babilidade complementar.


                                A= retirar pelo menos uma peça defeituosa

  3 - Qual é o evento?          A = não retirar alguma peça defeituosa, ou seja, retirar as
                                duas peças boas. Se não retiramos duas peças boas, então
                                pelo menos uma será defeituosa.

  4 - Retirem do espaço
                                Não será preciso, pois vamos usar o conceito de probabili-
  amostral os elementos
                                dade complementar.
  favoráveis ao evento.


  5 - Cada elemento
  do espaço amostral
  tem a mesma chance
  de ocorrência?

  (Se a resposta for “Sim”,
                                Sim
  então vamos calcular
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)



  6 - Fazer o cálculo usan-     Undo o conceito de eventos complementares:
  do o método
  apropriado.                   P( A ) = 1 − P(A) = 1 − 0,42 = 0,58 = 58%




                                              125

## Página 126

União de eventos (μ)

Sejam dois eventos A e B:


             U

                 A                                                      B




Na união, consideramos os elementos da parte rosa, da parte verde e da parte branca.


União  U  palavra usada OU  operação (+)


P(A) ∪ (B) = P(A) + P(B)  eventos mutuamente exclusivos


P(A) ∪ (B) = P(A) + P(B) − P(A ∩ B)  eventos não exclusivos


P(A ∪ B ∪ C) = P(A) + P(B) + P(C) − P(A ∩ B) − P(A ∩ C) − P(B ∩ C) + P(A ∪ B ∪ C)  3 eventos
                                                                             não exclusivos




                                             126

## Página 127

Exemplo 1


Retirando uma carta de um baralho comum de 52 cartas, qual é a probabilidade de ocor-
rer uma dama ou uma carta de copas?



          Etapas:                                       Respostas



  1 - Qual é o
                              Retirar uma carta de um baralho de 52 cartas.
  experimento?



                              W = {A♣, 2♣, 3♣, 4♣, 5♣, 6♣, 7♣, 8♣, 9♣, 10♣, J♣, Q♣, K♣,
                                  A♠, 2♠, 3♠, 4♠, 5♠, 6♠, 7♠, 8♠, 9♠,10♠, J♠, Q♠, K♠,
  2 - Qual é o espaço
                                  A♦, 2♦, 3♦, 4♦, 5♦, 6♦, 7♦, 8♦, 9♦, 10♦, J♦, Q♦, K♦,
  amostral?
                                  A♥, 2♥, 3♥, 4♥, 5♥, 6♥, 7♥, 8♥, 9♥, 10♥, J♥, Q♥, K♥}; n (W)
                                  = 52



                              Evento A: retirar uma dama.
  3 - Qual é o evento?
                              Evento B: retirar uma carta de copas.




  4 - Retirem do espaço       A= { Q♣, Q♠, Q♦, Q♥}; n(A) = 4
  amostral os elementos       B={ A♥, 2♥, 3♥, 4♥, 5♥, 6♥, 7♥, 8♥, 9♥, 10♥, J♥, Q♥, K♥}; n(B)=13
  favoráveis ao evento.       Elementos comuns em A e B  A ∩ B = { Q♥} ; n (A ∩ B) = 1




  5 - Cada elemento
  do espaço amostral
  tem a mesma chance
  de ocorrência?

  (Se a resposta for “Sim”,
                              Sim
  então vamos calcular
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)




                                            127

## Página 128

                                  Calculando a probabilidade de A ou B temos:

                                  P(A) ∪ (B) = P(A ∪ B) = P(A) + P(B) − P(A ∩ B)

                                                    n (A)          n (B)               n(A ∩ B)
                                    P(A ∪ B)   =              =                =
                                                    n (Ω)          n (Ω)                n (Ω)
  6 - Fazer o cálculo usan-
  do o método apropriado.
                                                     4        13           1           16         4
                                    P(A ∪ B)   =          +         -              =        =         = 0,3077 = 30,77%
                                                    52        52        52             52       13

                                  Logo, temos aproximadamente 31% de chance de retirar
                                  uma dama de copas no baralho.




Exemplo 2


Observando a cor dos olhos e a cor do cabelo de um grupo de 100 pessoas temos:


                              Cabelo louro               Cabelo castanho                                Total


     Olho azul                    10                               20                                     30


  Olho castanho                   30                               40                                     70


       Total                      40                               60                                    100



Escolhendo uma pessoa ao acaso, qual a probabilidade de ser uma pessoa com cabelo
louro e olhos castanhos ou cabelo castanho e olhos azuis?


Etapas:                         Respostas


  1 - Qual é o
                                  Escolher uma pessoa em um grupo de 100 pessoas.
  experimento?


                                  Como o número é elevado, não precisamos enumerar cada
                                  ponto amostral, basta determinar o número de elementos
  2 - Qual é o espaço
                                  do espaço amostral:
  amostral?
                                  n(W) = 100




                                                   128

## Página 129

                            Evento A: escolher uma pessoa com cabelo louro e olhos
                            castanhos.
3 - Qual é o evento?
                            Evento B: escolher uma pessoa com cabelo castanho e
                            olhos azuis.



                            n(A)=30
4 - Retirem do espaço       n(B)= 20
amostral os elementos
favoráveis ao evento.       Elementos comuns de A e B 
                            A ∩ B = { } ; n(A ∩ B) = 0




5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,
                            Sim  probabilidade clássica.
então vamos calcular
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)




                            Calculando a probabilidade de A ou B temos:

                            P(A ∪ B) = P(A) + (B) − P(A ∩ B)

                                               n (A)            n (B)            n(A ∩ B)
                              P(A ∪ B)   =                 =             =
                                              n (Ω)             n (Ω)                n (Ω)
6 - Fazer o cálculo usan-
do o método
                                               3               20            0         50
apropriado.                   P(A ∪ B)   =             +             -           =           = 0,5 = 50%
                                              100              100       100           100

                            Logo, temos 50% de chance de escolher uma pessoa
                            com cabelo louro e olhos castanhos ou cabelo castanho
                            e olhos azuis.




                                             129

## Página 130

Exemplo 3


Em um grupo de 60 pessoas, 10 são torcedoras do Botafogo, cinco do Fluminense e o
restante torce para o Flamengo. Escolhendo ao acaso um torcedor, qual a probabilidade
de ser torcedor do Botafogo ou do Fluminense? Aproveite esse resultado e determine a
probabilidade de o torcedor ser flamenguista.



Etapas:                       Respostas



  1 - Qual é o
                               Escolher um torcedor em um grupo de 60 pessoas.
  experimento?



  2 - Qual é o espaço
                               60 pessoas  n(W) =60
  amostral?



                               Experimento A: ser torcedor do Botafogo.
  3 - Qual é o evento?
                               Experimento B: ser torcedor do Fluminense.



                               n(A) = 10
  4 - Retirem do espaço        n(B) = 5
  amostral os elementos
  favoráveis ao evento.        Elementos comuns de A e B 
                               A ∩ B = { } ; n(A ∩ B) = 0




  5 - Cada elemento
  do espaço amostral
  tem a mesma chance
  de ocorrência?

  (Se a resposta for “Sim”,
                               Sim
  então vamos calcular
  a fórmula da
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)




                                            130

## Página 131

                                Calculando a probabilidade de A ou B temos:

                                P(A ∪ B) = P(A) + (B) − P(A ∩ B)

                                                     n (A)                n (B)        n(A ∩ B)
                                  P(A ∪ B)    =                   =               =
                                                     n (Ω)             n (Ω)               n (Ω)

                                                     10               5           0         15
                                  P(A ∪ B)    =               +              -         =           = 0,25 = 25%
                                                     60               60          60        60

                                Como os eventos dos torcedores do flamengo, do Botafogo
                                ou do Fluminense são complementares, para determinar a
  6 - Fazer o cálculo usan-
                                probabilidade de selecionar um torcedor do flamengo, pode-
  do o método
  apropriado.                   mos usar o conceito de probabilidades complementares, ou
                                seja, basta diminuir 100% - 25% = 75%.

                                Se quisermos confirmar podemos fazer:

                                C= torcedor do Flamengo  n(C)=45

                                               45
                                  P(C)    =               =       0,75 = 75%
                                               60

                                Logo, a probabilidade de selecionar um torcedor do Botafo-
                                go ou do Fluminense é igual a 25%. Já a chance do torcedor
                                ser flamenguista é igual a 75%.




Exemplo 4

Uma pesquisa, em uma agência de turismo, sobre o tipo de viagem nos últimos 12 meses, re-
velou que 45,8% dos clientes viajaram por razões comerciais, 54% por razões pessoais e 30%
viajaram por razões pessoais e comerciais.
       a) Qual é a probabilidade de que um viajante, selecionado aleatoriamente, tenha viajado
nos últimos 12 meses por razões comerciais ou pessoais?
       b) Qual é a probabilidade de que um viajante, selecionado aleatoriamente, não tenha via-
jado nos últimos 12 meses por razões comerciais ou pessoais?



          Etapas:                                                     Respostas


  1 - Qual é o                  Selecionar um cliente de uma agência de turismo que tenha
  experimento?                  viajado nos últimos 12 meses.




                                               131

## Página 132

                            Vamos considerar 100 pessoas, em que 45,8% dos clientes
2 - Qual é o espaço
                            viajaram por razões comerciais, 54% por razões pessoais e
amostral?
                            30% viajaram por razões pessoais e comerciais.


                            A = tenha viajado nos últimos 12 meses, por razões comer-
                            ciais.
                            B= tenha viajado nos últimos 12 meses, por razões pes-
                            soais.
                            A ∩ B= tenha viajado nos últimos 12 meses, por razões co-
3 - Qual é o evento?
                            merciais e pessoais.
                            A ∩ B= tenha viajado nos últimos 12 meses, por razões co-
                            merciais ou pessoais.
                            A ∪ B = não tenha viajado nos últimos 12 meses, por razões
                            comerciais ou pessoais.



                              A                                                    B



                                     15,8%            30%              24%
4 - Retirem do espaço
amostral os elementos
favoráveis ao evento.
                                                                                30,2%

                            n(A) = 45,8
                            n(B) = 54
                            n(A ∩ B) = 30



5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,
                            Sim, pois o cliente foi escolhido aleatoriamente.
então vamos calcular
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)




                                          132

## Página 133

                                 a)

                                                       n (A)            n (B)          n(A ∩ B)
                                      P(A ∪ B)   =                 =             =
                                                       n (Ω)            n (Ω)              n (Ω)
   6 - Fazer o cálculo usan-
                                                     45,8              54        30         69,8       0,5 =
   do o método apropriado.            P(A ∪ B)   =             +             -         =           =
                                                     100               100       100        100        50%

                                 b)

                                 P(A ∪ B) = 1 − P(A ∪ B) = 1 − 0,698 = 30,2%




Exemplo 5


Em um levantamento com 1.000 pessoas sobre a aquisição de um produto lançado no mercado,
em três versões, A, B e C, constatou-se que:
470 compraram o produto A.
420 compraram o produto B.
315 compraram o produto C.
110 compraram os produtos A e B.
220 compraram os produtos A e C.
140 compraram os produtos B e C.
75 compraram os três produtos.


Escolhendo-se ao acaso uma pessoa entrevistada, qual é a probabilidade de que ela tenha ad-
quirido pelo menos uma versão do produto?



           Etapas:                                                 Respostas




   1 - Qual é o                  Selecionar uma pessoa e perguntar qual foi a versão do pro-
   experimento?                  duto adquirido.




   2 - Qual é o espaço
                                 1000 entrevistados  n(Ω) = 1000
   amostral?




                                                 133

## Página 134

                            E= escolher pelo menos uma versão do produto.
                            Podemos dividir o evento E em três partes:
                            A = escolher o produto A.
3 - Qual é o evento?
                            B = escolher o produto B.
                            C = escolher o produto C.
                            Logo, E= A ∪ B ∪C



4 - Retirem do espaço       n(A)=470
amostral os elementos       n(B)=420
favoráveis ao evento.       n(C)=315



5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,
                            Sim  probabilidade clássica.
então vamos calcular
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)




                            P(E) = P(A ∪ B ∪ C) = P(A) + P(B) + P(C) − P(A ∩ B) − P(A ∩ C) − P(B
                            ∩ C) + P(A ∪ E)



                                                            470          420              315
                              P(E) = P(A ∪ B ∪ C)     =             +               +            -
6 - Fazer o cálculo usan-                                   1000         1000            1000
do o método
                              110         220        140           75        810
apropriado.                          -           -          +            =              = 0,81 = 81%
                             1000        1000        1000         1000       1000



                            Logo, haverá 81% de chance de uma pessoa comprar pelo
                            menos uma das versões do produto lançado no mercado.




                                           134

## Página 135

Poderíamos resolver esse problema de outra maneira utilizando o conceito da
probabilidade complementar.


Uma boa estratégia para resolver esse problema é representar as informações
do exercício no diagrama de Veen.

                                                                    U

          A                                                   B


                                    35
                     215                        245

                                    75

                           145             65



                                    30




                                    C
                                                                  190



Vamos começar pela interseção das 3 versões: A ∩ B ∩ C = 75


Depois, passamos para a interseção de duas versões:
A ∩ B = 110 (como já colocamos 75 na interseção das 3 versões, restam agora
110–75=35)
A ∩ C = 220 (como já colocamos 75 na interseção das 3 versões, restam agora
220-75=145)
B ∩ C = 140 (como já colocamos 75 na interseção das 3 versões, restam agora
140–75=65)


Agora vamos descontar as respostas que já foram posicionadas.
A = 470 – 35 – 75 – 145 = 215
B= 420 – 35 – 75 – 65 = 245
C= 315 – 145 – 75 – 65 = 30




                                    135

## Página 136

        Por fim, calculamos das 1.000 pessoas aquelas que não compraram nenhuma
        versão do produto:
        1000 – 215 - 35 – 75 – 145 – 245 – 65 – 30 = 190


        Se fizermos o complementar das pessoas que não compraram nenhuma ver-
        são do produto, vamos obter como resultado as pessoas que compraram pelo
        menos uma versão do produto. Logo:
        C= pessoas que compraram pelo menos uma versão do produto
        C = pessoas que não compraram o produto  n ( C ) = 190


        P (C) = 1 − P ( C )


                          190        810
          P(C) = 1 −            =            = 0,81 = 81%
                         1000       1000




Exemplo 6


Um jogo de dominó é composto por 28 peças, sendo que em cada peça são representadas duas
quantidades de bolinhas, que variam de zero a seis.


Se selecionarmos uma peça aleatoriamente, qual é a probabilidade de possuir ao menos um 3
ou 5 em sua face?




           Etapas:                                             Respostas




   1 - Qual é o
                                Retirar uma peça do dominó.
   experimento?




                                Ω = {(0,0), (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (1,1), (1,2), (1,3),
   2 - Qual é o espaço
                                (1,4), (1,5), (1,6), (2,2), (2,3), (2,4), (2,5), (2,6), (3,3), (3,4), (3,5),
   amostral?
                                (3,6), (4,4), (4,5), (4,6), (5,5), (5,6), (6,6)}  n (Ω) = 28




                                                136

## Página 137

                              Evento A: Saírem 3 bolinhas.
3 - Qual é o evento?          Evento B: Saírem 5 bolinhas.
                              A ∪ B : sair 3 OU 5 bolinhas


                              A={(0,3), (1,3), (2,3), (3,3), (3,4), (3,5), (3,6)}  n(A) = 7
4 - Retirem do espaço
                              B={(0,5), (1,5), (2,5), (3,5), (4,5), (5,5), (5,6)}  n(B) = 7
amostral os elementos
favoráveis ao evento.         Verificando a existência da interseção  A ∩ B = {(3,5)} 
                              n (A ∩.B) =1


5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,
                              Sim
então vamos calcular
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)


                              Calculando a probabilidade do evento A ou B ocorrem:

                              P(A ∪ B) = P(A) + P(B) − P(A ∩ B)
6 - Fazer o cálculo usan-
                                                   7        7         1         13
do o método                     P(A ∪ B)     =          +         –        =         = 0,464=46,4%
                                                   28       28        28        28
apropriado.
                              Logo, haverá 46% de chance de uma peça do dominó pos-
                              suir ao menos um 3 ou um 5 em sua face.




       Ampliando o foco


     Para colocar em prática o conteúdo visto nesta unidade visite a Biblioteca Vir-
     tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
     para reforçar o conhecimento.


     Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
     com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                             137

## Página 138

Probabilidade condicional

Neste tópico continuaremos os estudos abordando mais algumas relações básicas das
probabilidades: a probabilidade para eventos condicionados, a probabilidade para eventos
independentes e a interseção de eventos. Também apresentaremos o Teorema de Bayes.


Probabilidade condicional

Dados dois eventos “A” e “B” de um espaço amostral Ω ≠ Ø, chamamos de probabilidade
de “B” condicionada a “A”, a probabilidade de ocorrer “B”, sabendo que “A” já ocorreu.


Representação: P(B|A) ou P(B/A)


Dizemos: “probabilidade de “B” ocorrer, sabendo que “A” já ocorreu.


Cálculo:

           n (A∩B)             P (A∩B)
P(B|A) =           ou P(B|A) =
             n(A)                P(A)


Exemplo1:

Para oferecer uma promoção do tipo “fale grátis de seu móvel para seu fixo”, escolhe-se
aleatoriamente uma pessoa de uma base de dados com 15.000 usuários de telefonia,
dos quais 10.000 possuem telefones fixos, 8.000 telefones móveis e 3.000 têm telefones
fixos e móveis.


Pergunta-se:


    a) Já sabendo que a pessoa selecionada tem um telefone móvel, qual a probabilida-
    de de ela ter telefone fixo também?




                                           138

## Página 139

        Etapas:                                        Respostas


1 - Qual é o                Escolher aleatoriamente uma pessoa em uma base
experimento?                de dados.


                            15.000 usuários de telefonia, dos quais 10.000 possuem te-
2 - Qual é o espaço
                            lefones fixos, 8.000 telefones móveis e 3.000 têm telefones
amostral?
                            fixos e móveis.

                            A= selecionar uma pessoa com telefone móvel.
                            B= selecionar uma pessoa com telefone fixo.
3 - Qual é o evento?
                            B|A= selecionar uma pessoa com telefone fixo sabendo que
                            ela tem um telefone móvel.


4 - Retirem do espaço       n(A) = 8.000
amostral os elementos       n(B) = 10.000
favoráveis ao evento.       n(A ∩ B) = 3.000


5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?
                            Sim, pois a pessoa é escolhida aleatoriamente. Temos
                            que observar que uma condição já aconteceu nesse
(Se a resposta for “Sim”,
                            evento, ou seja, já sabemos que a pessoa possui uma li-
então vamos calcular
                            nha de telefonia móvel, por isso vamos usar o conceito
a fórmula da
                            de probabilidade condicional.
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)



                                       n (A∩B)
                            P(B|A) =
                                           n(A)

6 - Fazer o cálculo usan-              3000
do o método                 P(B|A) =              = 0,375 = 37,5%
                                       8000
apropriado.
                            Portanto, é provável que 37,5% dos usuários de telefonia da
                            base consultada tenham um telefone fixo, sabendo que ela
                            já possuía um telefone móvel.



                                           139

## Página 140

  b) Já sabendo que ela tem um telefone fixo, qual a probabilidade de ela ter telefone
  móvel também?


        Etapas:                                       Respostas

1 - Qual é o                Escolher aleatoriamente uma pessoa em uma base
experimento?                de dados.

                            15.000 usuários de telefonia, dos quais 10.000 possuem te-
2 - Qual é o espaço
                            lefones fixos, 8.000 telefones móveis e 3.000 têm telefones
amostral?
                            fixos e móveis.

                            A= selecionar uma pessoa com telefone móvel.
                            B= selecionar uma pessoa com telefone fixo.
3 - Qual é o evento?
                            A|B= selecionar uma pessoa com telefone móvel sabendo
                            que ela tem um telefone fixo.

4 - Retirem do espaço       n(A) = 8.000
amostral os elementos       n(B) = 10.000
favoráveis ao evento.       n(A ∩ B) = 3.000


5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?
                            Sim, pois a pessoa é escolhida aleatoriamente. Temos
                            que observar que uma condição já aconteceu nesse
(Se a resposta for “Sim”,
                            evento, ou seja, já sabemos que a pessoa possui uma li-
então vamos calcular
                            nha de telefonia fixo, por isso vamos usar o conceito
a fórmula da
                            de probabilidade condicional.
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)


                                       n (A∩B)
                            P(B|A) =
                                           n(B)

6 - Fazer o cálculo usan-              3000
do o método                 P(B|A) =              = 0,3 = 30%
                                       10000
apropriado.
                            Portanto, é provável que 30% dos usuários de telefonia da
                            base consultada tenham um telefone móvel, sabendo que
                            ela já possuía um telefone fixo.



                                           140

## Página 141

Exemplo 2:


Em uma escola com 100 alunos, 60 estudam Estatística, 50 estudam Inglês e 20 es-
tudam Estatística e Inglês. Sabendo que um aluno escolhido aleatoriamente já estuda
Estatística, qual a probabilidade de ele estudar Inglês?


          Etapas:                                     Respostas

  1 - Qual é o
                              Escolher aleatoriamente um aluno .
  experimento?

  2 - Qual é o espaço         100 alunos, 60 estudam Estatística, 50 estudam Inglês e 20
  amostral?                   estudam Estatística e Inglês.

                              A= escolher um aluno que estude Estatística.
                              B= escolher um aluno que estude Inglês.
  3 - Qual é o evento?
                              B|A= escolher um aluno que estude Inglês, sabendo que
                              esse aluno já estuda Estatística.


                                E                                    I


                                     40          20           30
  4 - Retirem do espaço
  amostral os elementos
  favoráveis ao evento.                                            10
                              n(A) = 60
                              n(B) = 50
                              n (A ∩ B)= 20


  5 - Cada elemento
  do espaço amostral
  tem a mesma chance
  de ocorrência?
                              Sim, pois a pessoa é escolhida aleatoriamente. Temos que
  (Se a resposta for “Sim”,   observar que uma condição já aconteceu nesse evento, ou
  então vamos calcular        seja, já sabemos que a pessoa escolhida estuda Estatística,
  a fórmula da                por isso vamos usar o conceito de probabilidade condicional.
  probabilidade clássica.
  Se for “Não”, iremos
  usar o cálculo empírico
  ou frequencial.)




                                           141

## Página 142

                                          n (A∩B)
                               P(B|A) =
                                            n(A)

                                          20
  6 - Fazer o cálculo usan-    P(B|A) =         = 0,33 = 33%
  do o método apropriado.                 60

                               Portanto, é provável que 30% dos usuários de telefonia da
                               base consultada tenham um telefone móvel, sabendo que
                               ela já possuía um telefone fixo.




Probabilidade para eventos independentes

Dois eventos A e B são independentes se, e somente se:



        P(A|B) = P(A) ou P(B|A) = P(B)

                                                     P (A∩B)                 P (A∩B)
        Substituindo P(B|A) = P(B) em: P(B|A) =                temos: P(B)
                                                      P(A)                    P(A)

        Logo: P(A ∩ B) = P(A). P(B)

        Analogamente,
                                                     P (A∩B)                 P (A∩B)
        Substituindo P(B|A) = P(A) em: P(B|A) =                temos: P(A)
                                                      P(B)                    P(B)

        Logo: P(A ∩ B) = P(A). P(B)




Exemplo 1


O quadro a seguir apresenta o resultado de uma pesquisa sobre a primeira razão para a
escolha de uma determinada Instituição de Ensino Superior – IES:




                                               142

## Página 143

                             Custo da             Qualidade
                                                                      Outros      Total
                            mensalidade           do ensino

Ensino presencial              393                  421                76         890

Ensino a distância             593                  400                46         1039

       Total                   986                  821                122        1929



  a) Se selecionarmos um aluno aleatoriamente e observarmos que ele optou por en-
  sino presencial, qual a probabilidade de a qualidade de ensino ser a primeira razão
  para a escolha da IES?


        Etapas:                                           Respostas

1 - Qual é o
                               Selecionar aleatoriamente um aluno entrevistado.
experimento?

2 - Qual é o espaço
                               Está representado no quadro no enunciado da questão.
amostral?

                               A= aluno ser do ensino presencial.
3 - Qual é o evento?           B= aluno responder que a qualidade de ensino é a primeira
                               razão para a escolha da IES.


4 - Retirem do espaço          n(A) = 890
amostral os elementos          n(B) = 821
favoráveis ao evento.          n (A ∩ B)= 421


5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?                 Sim, pois o aluno foi escolhido aleatoriamente.

(Se a resposta for “Sim”,      Porém, temos que prestar atenção, pois uma condição já
então vamos calcular           aconteceu nesse evento, ou seja, já sabemos que a pessoa
a fórmula da                   escolhida frequenta o ensino presencial e por isso vamos
probabilidade clássica.        usar o conceito de probabilidade condicional.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)




                                            143

## Página 144

                                         n (A∩B)
                            P(B|A) =
                                          n(A)

                                         421
6 - Fazer o cálculo usan-   P(B|A) =             = 0,473 = 47,3%
do o método                              890
apropriado.                 Logo, existe aproximadamente 47,3% de probabilidade
                            de que o aluno que estuda presencialmente tenha esco-
                            lhido a qualidade do ensino como primeira razão para
                            escolher a IES.



  b) Se um aluno selecionado aleatoriamente optou por ensino a distância, qual a pro-
  babilidade de o custo da mensalidade ser a primeira razão para escolha da IES?


        Etapas:                                        Respostas

1 - Qual é o
                            Selecionar aleatoriamente um aluno entrevistado.
experimento?

2 - Qual é o espaço
                            Está representado no quadro, no enunciado da questão.
amostral?

                            A= aluno ser do ensino a distância.
3 - Qual é o evento?        B= aluno responder que o custo da mensalidade é a primei-
                            ra razão para escolha da IES.

4 - Retirem do espaço       n(A) = 1039
amostral os elementos       n(B) = 986
favoráveis ao evento.       n (A ∩ B) = 593

5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?              Sim, pois o aluno foi escolhido aleatoriamente.

(Se a resposta for “Sim”,   Porém, temos que prestar atenção, pois uma condição já
então vamos calcular        aconteceu nesse evento, ou seja, já sabemos que a pessoa
a fórmula da                escolhida é do EAD e por isso vamos usar o conceito de
probabilidade clássica.     probabilidade condicional.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)




                                           144

## Página 145

                                          n (A∩B)
                              P(B|A) =
                                           n(A)

                                           593
  6 - Fazer o cálculo usan-   P(B|A) =               = 0,571 = 57,1%
  do o método apropriado.                 1039

                              Logo, existe aproximadamente 57,1% de probabilidade de
                              que o aluno que estuda a distância tenha escolhido o custo
                              da mensalidade como primeira razão para escolher a IES.




    c) Considere os eventos: A=estudante ser do ensino presencial e B=qualidade do en-
    sino como a primeira razão de escolha da IES. Os eventos A e B são independentes?


A= ser do ensino presencial  n((A) = 890

B = qualidade de ensino  n(B) = 821


n(A ∩ B) = 421


n (Ω) =1929

         ser do presencial   890         = 0,461 =
P(A) =                     =
          total de alunos    1929          46,1%

         qualidade de ensino    821
P(B) =                       =            = 0,426 = 42,6,%
           total de alunos     1929

            qualidade de ensino        n(A ∩ B)            421
P(A∩B) =                          =                 =                  = 0,218 = 21,8,%
              total de alunos            n (Ω)            1929


Se os eventos forem independentes, temos: P(A ∩ B) = P(A) . P(B)


Comparando 0,218 com 0,461. 0,426


0,218 ≠ 0,196


Como P(A ∩ B) ≠ P(A) . P(B) os eventos não são independentes.



                                            145

## Página 146

Interseção de eventos (U)


Sejam dois eventos A e B:



        U
             A                            B




        Na interseção, consideramos somente os elementos da parte branca.

        Interseção ∩ palavra usada “E”  operação (x)

        P(A ∩ B) = P(A). P(B)  eventos independentes

        P(A ∩ B) = P(A). P(B/A)  eventos condicionados

        P(A ∩ B) = P(A). P(A/B)  eventos condicionados



Vejamos alguns exemplos.


Exemplo 1


Suponha que a probabilidade de a fábrica de respiradores entregar a mercadoria no prazo
seja 99,4%. Suponha que duas entregas de respiradores sejam selecionadas ao acaso.
Qual a probabilidade de:
     a) Ambos os respiradores serem entregues no prazo.
     b) Exatamente um dos respiradores for entregue no prazo.
     c) Ambos os respiradores forem entregues com atraso.


Resolução:


    Sejam:
    Evento A: entregar o respirador 1 no prazo      P(A) = 0,994
    Evento A: não entregar o respirador 1 no prazo  P(A) = 1-0,994 = 0,006


                                          146

## Página 147

    Evento B: entregar o respirador 2 no prazo  P(B) = 0,994
    Evento B : não entregar o respirador 2 no prazo  P(B) = 1-0,994 = 0,006


    a) O respirador 1 foi entregue no prazo E o respirador 2 foi entregue no prazo.


    A palavra-chave aqui é o “E”, significando a interseção (multiplicação) dos eventos.
    Logo:

    P(A ∩ B) = P(A). P(B)

    P(A ∩ B) = 0,994 . 0,994 = 0,988 = 98,8%


    b) O respirador 1 foi entregue no prazo E o respirador 2 não foi entregue no prazo
    OU o respirador 1 não foi entregue no prazo E o respirador 2 foi entregue no prazo.


    A palavra-chave aqui é o “E”, significando a interseção (multiplicação) dos eventos e
    também o ”OU” significando a união (soma) dos eventos. Logo:


    P(A ∩ B) ou P(A ∩ B) = P(A) . P(B) + P(A) . P(B) = 0,994 . 0,006 + 0,006 . 0,994 = 0,012 = 1,2%


    c) Ambos os respiradores forem entregues com atraso, logo o respirador 1 foi entre-
    gue com atraso E o respirador 2 foi entregue com atraso.


    P(A ∩ B) = P(A) . P(B)


    P(A ∩ B) = 0,006 . 0,006 = 0,000036 = 0,0036%


Exemplo 2


Em um lote de 12 peças selecionadas para inspeção, quatro apresentam defeitos. Três
peças são retiradas ao acaso, uma após a outra, sem reposição. Calcule a probabilidade
de as três peças selecionadas não apresentarem defeitos.


Etapas:                       Respostas

  1 - Qual é o
                                 Retirar três peças para serem inspecionadas.
  experimento?

  2 - Qual é o espaço
                                 12 peças, sendo quatro defeituosas e oito sem defeitos.
  amostral?



                                               147

## Página 148

                                  A= selecionar para inspeção a primeira peça sem defeito.
                                  B= selecionar para inspeção a primeira peça sem defeito.

3 - Qual é o evento?              C= selecionar para inspeção a primeira peça sem defeito.
                                  D= selecionar a primeira peça sem defeito para inspeção
                                  E selecionar a segunda peça sem defeito para inspeção E
                                  selecionar a terceira peça sem defeito para inspeção.


4 - Retirem do espaço             n(A) = 8
amostral os elementos             n(B) = 7
favoráveis ao evento.             n(C) = 6


5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,         Sim, pois todas as peças têm a mesma chance de serem
então vamos calcular              selecionadas para inspeção.
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)



                                  P(D) = P(A) . P(B) . P(C)
6 - Fazer o cálculo usan-
do o método                                  8    .     7     .   6         336
                                  P(D) =                               =          = 0,25 = 25%
apropriado.                                  12         11        10       1320




                                             Resumindo


Símbolo                     Palavra                                        Operação

   ∪                          ou                                            adição

   ∩                          e                                        Multiplicação


   A                         não                                       P(A) = 1 - P(A)



                                                  148

## Página 149

                                                        P(A ∪ B) = P(A) + P(B) 
                 É o evento que ocorre se A          eventos mutuamente exclusivos
    A∪B          ocorre, ou se B ocorrer ou
                     se A e B ocorrerem             P(A ∪ B) = P(A) + P(B) - P(A ∩ B) 
                                                         eventos não exclusivos


                                                         P(A ∩ B) = P(A) . P(B/A)
                  É o evento que ocorre se
    A∩B
                       A e B ocorrem
                                                         P(A ∩ B) = P(A) . P(B/A)



Axiomas das probabilidades

A probabilidade clássica P(A) satisfaz aos seguintes axiomas:


A1) A probabilidade é um número maior ou igual a zero e menor ou igual a 1.
0 ≤ P(A) ≤ 1


A2) A soma das probabilidades de todos os resultados possíveis em um espaço amostral
é igual a 1 ou 100%.
P(Ω) = 1


A3) Se os eventos forem, dois a dois mutuamente exclusivos então:
P(E) = P(E1) + P(E2) +...+ P(En)


Teoremas do cálculo das probabilidades para eventos
equiprováveis

T1) Se “A” e “B” são eventos equiprováveis quaisquer, então:
P(A ∪ B) = P(A) + P(B) − P(A ∩ B)

Para três eventos, temos:
P(A ∪ B ∪ C) = P(A) + P(B) + P(C) − P(A ∩ B) − P(A ∩ C) − P(B ∩ C) + P(A ∪ B ∪ C)


T2) Se A é o complementar de A, então a soma das probabilidades de um evento com seu
evento complementar é sempre igual a 100%, ou seja, igual a 1:
P(A) + P(A) = ∴ P(A) = 1− P(A) ou P(A) = 1 − P(A)


T3) Se A ⊂ B, então:
P(A) ≤ P(B).


                                              149

## Página 150

Vejamos um exemplo que ilustra o conceito apresentado.


Exemplo


Um número é escolhido entre os cinco primeiros números naturais. Considere os eventos
a seguir e encontre a probabilidade de que o número escolhido seja:


    a) A= Menor do que 5.
    b) B= Maior do que 5.
    c) C= Igual a zero.
    d) D= Igual a 1.
    e) E= Igual a 2.
    f) F= Igual a 3.
    g) G= Igual a 4.
    h) H= Igual a 1 ou igual a 4 ou igual a um número primo.
    i) I= Par ou primo.
    j) J= Diferente de 1.
    k) K=um número par.
    l) L=um número ímpar.
    m) M= um número par ou ímpar.


Resolução:


Primeiramente vamos definir:


Experimento: escolher um número entre os cinco primeiros números naturais
Espaço Amostral: Ω ={0,1,2,3,4} , logo o número de elementos do espaço amostral será:
n(Ω) = 5


A seguir vamos calcular as probabilidades solicitadas:


    a) A= nº menor do que 5  A={0,1,2,3,4}, logo, o número de elementos do evento A
    será: n(A) = 5 então:

             n(A)       5
    P(A) =          =       = 1 = 100%
             n(Ω)       5




                                          150

## Página 151

    Observamos que o número de elementos do evento A é igual ao número de
    elementos do espaço amostral. Esse é o maior número de elementos que po-
    demos retirar do espaço amostral, portanto essa será a maior probabilidade
    que poderá ocorrer. É um exemplo do evento certo.



b) B= Maior do que 5  B={ }, não existe nenhum elemento no espaço amostral
maior do que 5, logo o número de elementos do evento B será: n(B) = 0 então:

         n(B)       0
P(B) =          =       = 0 = 0%
         n(Ω)       5



    Observamos que o número de elementos do evento é o menor número de
    elementos que podemos escolher no espaço amostral, portanto essa será a
    menor probabilidade que poderá ocorrer. É um exemplo do evento impossível.




    Concluímos então, observando os eventos A e B, que a probabilidade de ocor-
    rência de um evento, é um número maior ou igual a zero e menor ou igual a 1,
    conforme afirma o axioma 1:
    0 ≤ P(A) ≤ 1



c) C= Igual a zero  C={0}, logo, o número de elementos do evento C será: n(C)=1 então:

         n(C)       1
P(C) =          =       = 0,2 = 20%
         n(Ω)       5


d) D= Igual a 1  D={1}, logo, o número de elementos do evento D será: n(D)=1 então:

         n(D)       1
P(D) =          =       = 0,2 = 20%
         n(Ω)       5


e) E= Igual a 2  E={2}, logo, o número de elementos do evento E será: n(E)=1 então:

         n(E)       1
P(E) =          =       = 0,2 = 20%
         n(Ω)       5




                                        151

## Página 152

    f) F= Igual a 3  F={3}, logo, o número de elementos do evento F será: n(F)=1 então:

               n(F)       1
    P(F) =            =        = 0,2 = 20%
               n(Ω)       5


    g) G= Igual a 4  G={4}, logo, o número de elementos do evento G será: n(G)=1 então:

               n(G)       1
    P(G) =            =        = 0,2 = 20%
               n(Ω)       5



           Se somarmos as probabilidades dos eventos C, D, E, F e G estamos somando
           as probabilidades de todos os elementos do espaço amostral. Logo, vamos
           obter: 0,2+0,2+0,2+0,2+0,2 = 1 ou 100%, o que afirma o axioma 2, ou seja: a
           soma das probabilidades de todos os resultados possíveis de um espaço
           amostral é igual a 1 ou 100%.



    h) H= o número igual a 1 ou o número igual a 4 ou um número primo.  H={1,4,2,3},
    logo, o número de elementos do evento H será: n(H)=4 então:

               n(H)       4
    P(H) =            =        = 0,8 = 80%
               n(Ω)       5


2ª Temos outra maneira de calcular essa probabilidade. Pode ser a seguinte:


Observamos que podemos dividir o evento H em 3 eventos: H1= o número igual a 1, H2=
o número igual a 4 e H3= um número primo. Vamos buscar no espaço amostral os ele-
mentos dos eventos.
H1={1}
H2={4}
H3={2,3}


Vamos observar, dois a dois, as interseções dos elementos dos eventos:
H1 ∩ H2={ }
H1 ∩ H3={ }
H2 ∩ H3={ }


Como a interseção entre os elementos dos eventos dois a dois é vazia, concluímos que
são eventos mutuamente exclusivos.



                                               152

## Página 153

Logo, para calcular a probabilidade H= o número igual a 1 ou o número igual a 4 ou um
número primo, basta calcular cada probabilidade separadamente e efetuar a soma das
probabilidades.


Então:

         n(H1)   n(H2)   n(H3)              1         1       2      4
P(H) =         =       =       =              +           +     =        = 0,8 = 80%
         n(Ω)    n(Ω)    n(Ω)               5         5       5      5



          Com esse item podemos verificar o que diz no axioma 3: se os eventos forem
          dois a dois mutuamente exclusivos, então: P(E) = P(E1) + P(E2) +...+ P(En)




     i) I=Par ou primo  I={0,2,3,4}, logo, o número de elementos do evento I será: n(I)=4 então:

               n(I)         4
     P(H) =            =         = 0,8 = 80%
               n(Ω)         5


3ª Temos outra maneira de calcular essa probabilidade. Pode ser a seguinte:


Observamos que podemos dividir o evento I em 2 eventos: I1= número par, I2= número
primo. Vamos buscar no espaço amostral os elementos dos eventos:
I1={0,2,4} n(I1) = 3
I2={2,3} n(I2) = 2


Vamos observar, dois a dois, as interseções dos elementos dos eventos:
I1 ∩ I2={ 2 } ∴ n(I1 ∩ I2) =


Nesse caso, observamos um elemento na interseção dos eventos. Logo, os eventos não
são mutuamente excludentes. Precisamos de, em vez de usar o axioma 3 para calcular a
probabilidade, devemos usar o teorema 1, que diz:


(A ∪ B) = P(A) + P(B) − P(A ∩ B)

Logo:


P(I1 ∪ I2) = P(I1) + P( I2) - P(I1 ∩ I2)




                                                153

## Página 154

               n(I1)       n(I2)        n(I1 ∩ I2)
P(I1 ∪ I2) =           =            =
               n(Ω)        n(Ω)           n(Ω)

               3       2       1        4
P(I1 ∪ I2) =     +         -     =          = 0,8 = 80%
               5       5       5        5


     j) J= Diferente de 1  I={0,2,3,4}, logo, o número de elementos do evento I será: n(I)=4 então:

               n(I)        4
     P(I) =            =           = 0,8 = 80%
               n(Ω)        5


4ª Temos outra maneira de calcular essa probabilidade. Pode ser a seguinte:


Outra maneira de calcular a probabilidade pedida seria utilizando o teorema 2, que diz:
se A é o complementar de A, então a soma das probabilidades de um evento com seu
evento complementar é sempre igual a 100%, ou seja, P(I) + P(I ) = 1 ∴ P(I ) + 1 - P(I) ou
P(I) = 1 - P(I )


Se “J” é o evento de escolha do número diferente de 1, então seria escolha do número
igual a 1. Nós já calculamos, na letra “d” deste exercício, que a probabilidade de retirar o
número 1 cujo resultado foi igual a 0,2, ou seja, P(I ) = 0,2 então, a probabilidade de não
retirar o número 1 seria: P(I) = 1 - P(I ) = 1- 0,2 = 0,8


Teorema de Bayes


Também conhecido como Teorema da Probabilidade das Causas ou dos Antecedentes,
ou ainda Teorema da Probabilidade a posteriori.


Esse teorema preocupa-se em responder à seguinte pergunta:

Supondo que o evento B já tenha ocorrido, qual a probabilidade que este
evento tenha provindo do evento AI?

O Teorema de Bayes é importante porque inverte probabilidades condicionais. Ele nos dá
a probabilidade de uma causa ocorrer, desde que um evento já tenha ocorrido.


O teorema diz: “Se “n” eventos A1, A2, A3, ... , são dois a dois mutuamente exclusivos, tais
que, A1 ∪ A2 ∪ A3 ∪ ... An = Ω, e P(Ai) > 0 então:




                                                     154

## Página 155

             P(Ai) . P(B\ Ai)
P(Ai\B) =
            Σi P(Ai) . P(B\ Ai)


Para entendermos melhor o assunto, vejamos alguns exemplos práticos.


Exemplo 1


Em uma fábrica de produção de aparelhos celulares, as linhas de montagem I, II e III
respondem respectivamente por 50, 40 e 10 por cento da produção. Alguns aparelhos
saem destas linhas com defeitos. A porcentagem de celulares defeituosos é de 1%, 2%
e 3%, respectivamente, para as linhas I, II e III. Para evitar que os aparelhos defeituosos
saiam da empresa e cheguem ao mercado, o controle de qualidade realiza inspeções
individuais em todos os celulares fabricados e os que apresentam algum defeito são
enviados para uma linha especial de recuperação. Você, como responsável pelo controle
de qualidade, precisa intervir na linha de montagem mais problemática. Por qual linha de
montagem você iniciaria seus trabalhos?


Para ajudar nessa decisão será preciso conhecer:


    a) Qual a probabilidade de o celular defeituoso ter sido produzido na linha de
    montagem I?
    b) Qual a probabilidade de o celular defeituoso ter sido produzido na linha de
    montagem II?
    c) Qual a probabilidade de o celular defeituoso ter sido produzido na linha de
    montagem III?


Etapas:                           Respostas

  1 - Qual é o
                                   Retirar um aparelho defeituoso para inspeção.
  experimento?


                                   Linha de               Linha de     Linhade
                                   produção 1             produção 2   produção 3

                                           50%                40%       10%
  2 - Qual é o espaço
  amostral?
                                                1%            2%       3%
                                                       DEFEITOS




                                                 155

## Página 156

                       Determinar a probabilidade de um aparelho celular defei-
                       tuoso encontrado na inspeção final ter sido produzido na
                       linha de produção I, na linha de produção II e na linha de
                       produção III.


                       A1 = aparelho ter sido produzido na linha de produção I.
                       A2 = aparelho ter sido produzido na linha de produção II.
                       A3 = aparelho ter sido produzido na linha de produção III.


                       B= aparelho com defeito.


                       B\A1= aparelho defeituoso sabendo que foi produzido na li-
                       nha de produção I
                       B\A2 = aparelho defeituoso sabendo que foi produzido na
                       linha de produção II
                       B\A3= aparelho defeituoso sabendo que foi produzido na li-
3 - Qual é o evento?   nha de produção III


                       O que queremos investigar:


                       A1\B= aparelho ter sido produzido na linha de montagem I
                       sabendo que ele é defeituoso
                       A2\B = aparelho ter sido produzido na linha de montagem II
                       sabendo que ele é defeituoso
                       A3\B = aparelho ter sido produzido na linha de montagem III,
                       sabendo que ele é defeituoso.


                       Ou seja, precisamos usar o teorema de Bayes para calcular
                       a probabilidade condicional invertida.


                                    P(Ai) . P(B\ Ai)
                       P(Ai\B) =
                                   Σ i P(Ai) . P(B\ Ai)




                                    156

## Página 157

                            P(A1) = 50% = 0,5
                            P(A2) = 40% = 0,4
                            P(A3) = 10% = 0,1
                            P(B\A1) = 1% = 0,01
4 - Retirem do espaço
amostral os elementos       P(B\A2) = 2% = 0,02
favoráveis ao evento.       P(B\A3) = 3% = 0,03

                            P(A1\B) = ??? é o que queremos determinar

                            P(A2\B) = ??? é o que queremos determinar

                            P(A3\B) = ??? é o que queremos determinar



5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,   Sim, pois todos os aparelhos têm a mesma chance de se-
então vamos calcular        rem selecionados para inspeção.
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)



                                              P(Ai) . P(B\ Ai)
                            P(Ai\B) =
                                            Σ i P(Ai) . P(B\ Ai)

                            Cálculo da probabilidade da peça defeituosa ter sido produzida
                            na linha I:

                                                             P(A1) . P(B\ A1)
                            P(A1\B) =
6 - Fazer o cálculo usan-                 P(A1) . P(B\ A1) + P(A2) . P(B\ A2) + P(A3) . P(B\ A3)
do o método
apropriado.
                                                                 0,5 . 0,01
                            P(A1\B) =
                                                   0,5 . 0,01 + 0,4 . 0,02 + 0,1 . 0,03



                                                   0,005                  0,005
                            P(A1\B) =                                 =               = 0,3125 = 31,5%
                                          0,005 + 0,008 + 0,003               0,016




                                             157

## Página 158

                    P(Ai) . P(B\ Ai)
P(Ai\B) =
                  Σ i P(Ai) . P(B\ Ai)

Cálculo da probabilidade da peça defeituosa ter sido produzida
na linha II:

                                  P(A2) . P(B\ A2)
P(A2\B) =
                P(A1) . P(B\ A1) + P(A2) . P(B\ A2) + P(A3) . P(B\ A3)



                                       0,4 . 0,02
P(A2\B) =
                         0,5 . 0,01 + 0,4 . 0,02 + 0,1 . 0,03



                         0,005                      0,008
P(A2\B) =                                   =               = 0,5 = 50%
                0,005 + 0,008 + 0,003               0,016



Cálculo da probabilidade da peça defeituosa ter sido produzida
na linha III:



                                  P(A3) . P(B\ A3)
P(A3\B) =
                P(A1) . P(B\ A1) + P(A2) . P(B\ A2) + P(A3) . P(B\ A3)



                                       0,1 . 0,03
P(A3\B) =
                         0,5 . 0,01 + 0,4 . 0,02 + 0,1 . 0,03



                         0,005                  0,003
P(A3\B) =                                   =               = 0,1875 = 18,75%
                0,005 + 0,008 + 0,003               0,016



Logo, você deve escolher a linha de produção II para fazer
a intervenção.




                   158

## Página 159

Exemplo 2


Um viajante usa os serviços da companhia aérea “A” em 20% das viagens, 30% da com-
panhia “B” e no restante do tempo ele utiliza a companhia “C”. Sabe-se que a porcen-
tagem dos voos que decolam com atraso é de 10%, 6% e 3% respectivamente para as
companhias “A”, “B” e “C”. Suponha que o voo do viajante teve atraso.


Qual a probabilidade de que ele tenha utilizado a companhia “C” nessa viagem?


          Etapas:                                      Respostas


  1 - Qual é o               Verificar a probabilidade de um viajante ter utilizado a com-
  experimento?               panhia aérea "A', sabendo que ela teve um atraso no voo.

                               CIA "A"    CIA "B'          CIA "C"
                               20%        30%               50%

  2 - Qual é o espaço
  amostral?
                                  10%      6%               3%
                                                 ATRASOS




                             A1 = viajante ter escolhido a cia aérea “A” para viajar.
                             A2 = viajante ter escolhido a cia aérea “B” para viajar.
                             A3 = viajante ter escolhido a cia aérea “C” para viajar.


                             B= o voo decolou atrasado.


                             B\A1 = voo atrasado sabendo que a cia escolhida foi a “A”

  3 - Qual é o evento?       B\A2 = voo atrasado sabendo que a cia escolhida foi a “B”
                             B\A3 = voo atrasado sabendo que a cia escolhida foi a “C”


                             O que queremos investigar:
                             A3\B = a cia escolhida foi “C” sabendo que o voo decolou
                             com atraso.


                             Ou seja, precisamos usar o teorema de Bayes para calcular
                             a probabilidade condicional invertida.




                                           159

## Página 160

                            P(A1) = 30% = 0,2
                            P(A2)= 20% = 0,3

4 - Retirem do espaço       P(A3)= 50% = 0,5
amostral os elementos       P(B\A1 )= 10% = 0,1
favoráveis ao evento.       P(B\A2 )= 6% = 0,06
                            P(B\A3 ) = 3% = 0,03
                            P(A3\B) = ??? é o que queremos determinar




5 - Cada elemento
do espaço amostral
tem a mesma chance
de ocorrência?

(Se a resposta for “Sim”,
                            Sim.
então vamos calcular
a fórmula da
probabilidade clássica.
Se for “Não”, iremos
usar o cálculo empírico
ou frequencial.)



                                              P(Ai) . P(B\ Ai)
                            P(Ai\B) =
                                            Σ i P(Ai) . P(B\ Ai)

                            Cálculo da probabilidade da peça defeituosa ter sido produzida
                            na linha I:

                                                            P(A3) . P(B\ A3)
                            P(A3\B) =
6 - Fazer o cálculo usan-                 P(A1) . P(B\ A1) + P(A2) . P(B\ A2) + P(A3) . P(B\ A3)
do o método
apropriado.
                                                                 0,5 . 0,03
                            P(A3\B) =
                                                   0,2 . 0,01 + 0,3 . 0,06 + 0,5 . 0,03



                                                   0,015                      0,015
                            P(A3\B) =                                 =               = 0,283 = 28,3%
                                          0,002 + 0,018 + 0,015           0,053




                                             160

## Página 161

 Ampliando o foco


Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca Vir-
tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
para reforçar o conhecimento.


Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                       161

## Página 162

                                  MIDIATECA

Para ampliar o seu conhecimento veja o material complementar da Unidade 2,
disponível na midiateca.




                                NA PRÁTICA

Foi realizada uma pesquisa de mercado com 500 consumidores com o objetivo
de conhecer a aceitação de um novo produto — chamado Sampler — lançado
no mercado.

 Resultado da pesquisa de satisfação com 500 consumidoresdo produto Sampler


       Sexo                Aprovou           Não aprovou          Total

    Masculino               140                   100              240

     Feminino               200                    60              260

       Total                340                   160              500


                              Fonte: Dados fictícios.


Você precisa fazer um relatório para enviar ao CEO da empresa em que traba-
lha dando a ele feedback sobre a aceitação do produto do mercado. Pense em
como o conteúdo desta unidade pode lhe auxiliar na elaboração do relatório.


É interessante apresentar no relatório algumas probabilidades calculadas a par-
tir dos dados pesquisados. Por exemplo:


Se uma pessoa for selecionada ao acaso qual a probabilidade de:
a) Ser do sexo masculino?
b) Ser do sexo feminino?
c) Não tenha aprovado o produto.




                                       162

## Página 163

d) Ser do sexo feminino ou ter aprovado o produto.
e) Sabendo que a pessoa selecionada aprovou o produto, qual a probabilidade
de ser do sexo masculino?
f) Dado que a pessoa selecionada seja do sexo feminino, qual a probabilidade
de que ela não tenha aprovado o produto?


Vamos considerar:


A = a pessoa selecionada é do sexo masculino.
A =a pessoa selecionada não é do sexo masculino, ou seja, é do sexo feminino.
B = a pessoa selecionada aprovou o produto lançado.
B = a pessoa selecionada não aprovou o produto lançado.


              240
a) P(A) =            = 0,48 = 48%
              500


b) P(A) = 1- P(A) = 1 - 0,48 = 0,52 = 52%


              160
c) P(B) =            = 0,32 = 32%
              500


                                            260         340       340       400
d) P(A ∪ B) = P(A ) + P(B) - P(A ∩ B) =             +         -         =         = 0,8= 80%
                                            500         500       500       500


                P(A ∩ B)        140
e) P(A\B) =                 =         = 0,41= 41%
                    P(B)        340



                P(B ∩ A)        60
e) P(B\A) =                 =         = 0,23= 23%
                    P(A)        260




                                            163

## Página 164

Resumo da Unidade 2

Nesta unidade vimos os conceitos básicos da Teoria das probabilidades e determinamos
situações práticas às quais ela se aplica. Também revisamos as técnicas de contagem
e abordamos algumas definições importantes, necessárias para o entendimento do cál-
culo de probabilidades.


Apresentamos o cálculo das probabilidades considerando cenários diferentes. Estuda-
mos as relações básicas de probabilidades, como probabilidades complementares e pro-
babilidade na união de eventos.


Por fim, abordamos a Probabilidade Condicional e a Probabilidade na Interseção de even-
tos. Tratarmos igualmente de alguns axiomas e teoremas de probabilidade — probabili-
dade condicional e teorema de Bayes — com a utilização de exemplos práticos e desen-
volvidos passo a passo.




                                          164

## Página 165

Referências

KOKOSKA, S. Introdução à estatística: uma abordagem por resolução de problemas.
Rio de Janeiro: LTC, 2013.


LARSON, R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010.


MEGA-SENA: a loteria que paga milhões para o acertador. CEF – Loterias. Disponível
em: http://loterias.caixa.gov.br/wps/portal/loterias/landing/megasena. Acesso em: 18
ago. 2020.


O PROBLEMA de Monty Hall. YouTube. Disponível em: < https://www.youtube.com/wat-
ch?v=6_mMLdAzUCE. Acesso em: 12 ago. 2020.


PROBABILIDADES Teóricas e Experimentais. Khan Academy. Disponível em: < https://
pt.khanacademy.org/math/probability/probability-geometry/probability-basics/v/compa-
ring-theoretical-to-experimental-probabilites. Acesso em: 12 ago. 2020.


SPIEGEL, M. R.; STEPHENS, L. J. Estatística. 4. ed. Porto Alegre: Bookman, 2009


SWEENEY, D. J.; WILLIAMS, T. A.; ANDERSON, D. R. Estatística aplicada à administra-
ção e economia. 3. ed. São Paulo: Cengage Learning, 2013.




                                         165

## Página 166

                UNIDADE 3

Distribuições de Probabilidade

## Página 167

                                                    INTRODUÇÃO


Nesta unidade continuaremos o estudo da teoria das probabilidades, apresentando os
conceitos de variáveis aleatórias e de distribuições de probabilidades.


As distribuições teóricas de probabilidades são aquelas que foram submetidas a estudos
prévios e têm propriedades conhecidas. Por isso, nos ajudam na modelagem matemá-
tica de fenômenos aleatórios. Sendo assim, é mais uma ferramenta poderosa e muito
utilizada para os cálculos de probabilidades.


O que veremos nos tópicos, a saber:
    • Tópico 1: variáveis aleatórias, discretas e continuas e a função de distribuição de
    probabilidades.
    • Tópico 2: algumas distribuições discretas de probabilidades — adistribuição bino-
    mial e a distribuição de Poisson.
    • Tópico 3: algumas distribuições contínuas de probabilidades — a distribuição Nor-
    mal e a distribuição Normal padronizada.




                                        OBJETIVO

       Nesta unidade você será capaz de:

       • Utilizar as distribuições de probabilidades em situações inerentes à inferência
       estatística.




                                              167

## Página 168

Variáveis aleatórias e distribuições de
probabilidade

Ao determinarmos os elementos de um espaço amostral eles podem ter uma natureza
quantitativa ou qualitativa, como podemos observar no exemplo a seguir.


         Exemplo


       Notas dos alunos de Estatística  Ω =[0 ; 10]  quantitativa
       Sexo dos alunos de Estatística  Ω ={feminino, masculino}  qualitativa




Quando trabalharmos com variáveis aleatórias qualitativas ficamos impossibilitados de
operar matematicamente em alguns casos, porque seus valores não são numéricos. As
variáveis aleatórias vêm auxiliar-nos, principalmente quando necessitamos atribuir um
valor numérico para todos os elementos do espaço amostral, mesmo que esses elemen-
tos sejam qualitativos.


Variável aleatória

Uma variável aleatória é uma função que associa a cada elemento do espaço amostral
um único número real. Normalmente escolhemos letras maiúsculas do nosso alfabeto
para representá-las, sendo as mais usuais o “X” e o “Y”.


Apesar da terminologia “variável aleatória”, ela é uma função cujo domínio sempre é for-
mado pelos elementos do espaço amostral e o contradomínio o conjunto dos números
reais. Tais funções são chamadas aleatórias, porque seus valores não podem ser deter-
minados com certeza antes que o experimento seja realizado.


Para entendermos melhor a aplicabilidade deste conceito vejamos o exemplo a seguir.




                                            168

## Página 169

          Exemplo


         A produção de aparelhos celulares de uma determinada fábrica pode ter duas
         classificações quanto à qualidade: Defeituoso ou Perfeito, ou seja, temos um
         experimento em que os elementos do espaço amostral não são números e sim
         atributos, que vamos representar como “D”=defeituoso e “P”=perfeito.




Exemplo


Suponha que três aparelhos sejam retirados aleatoriamente para inspeção. Seja a variá-
vel aleatória “X” definida como o número de aparelhos danificados.


Então:


    1) O experimento consiste em observar as condições da qualidade de três
    aparelhos celulares.
    Espaço amostral: {PPP, PPD, PDP, PDD, DPP, DPD, DDP, DDD}, lembrando que
    “D” = defeituoso e “P” = perfeito.


    2) A variável aleatória “X” representa o número de aparelhos danificados, podendo
    assumir os seguintes valores, como mostra a tabela a seguir.


                      Resultado          X=nº de aparelhos danificados

                         PPP                            0
                         PPD                            1
                         PDP                            1
                         PDD                            2
                         DPP                            1
                         DPD                            2
                         DDP                            2
                         DDD                            3


    3) O importante é que agora não estamos interessados na ordem e sim na quantidade.




                                              169

## Página 170

Uma variável aleatória pode ser classificada como discreta ou contínua, dependendo do
número de possíveis valores que a variável pode assumir.


Variável aleatória discreta

A variável aleatória é discreta quando o conjunto de todos os valores que ela pode as-
sumir são associadas aos números naturais, 0, 1, 2, ... Normalmente, são oriundas
de contagens.



         Exemplo



              Experimento           Variável aleatória    Valores possíveis

          Inspecionar uma re-      X=nº de celulares      0,1,2,3,4,5
          messa de 5 celulares.    defeituosos.           (finito)

          Classificar os 160
                                   X=nº de alunos do      0,1,2,..,160
          alunos de Estatística
                                   sexo feminino.         (finito)
          quanto ao sexo.

          Observar o fluxo de      X=nº de carros que
          carros que passam        passam no pedá-        0,1,2,3,...
          no pedágio, num in-      gio, num intervalo     (infinito numerável)
          tervalo de 1 hora.       de 1 hora.




Variável aleatória contínua

A variável aleatória é contínua quando os conjuntos de todos os valores que ela pode
assumir pertencem a um intervalo real ou a um conjunto de intervalos não numeráveis.
Normalmente, são oriundas de medições.




                                          170

## Página 171

         Exemplo



               Experimento            Variável aleatória         Valores possíveis

             Observar o tempo     X=tempo, em minutos, en-
             de espera na fila    tre a chegada e a saída do      X≥0
             do banco.            cliente na agência bancária.

             Altura dos alunos    X= altura, em metros, dos
                                                                  0≤X≤3
             de Estatística.      alunos de Estatística.

             Pesar um carrega-
                                  X= número de quilos do
             mento de um de-                                      X≥0
                                  carregamento.
             terminado produto.
                                  X= Porcentagem de con-
             Construção de um
                                  clusão do projeto depois        0 ≤ X ≤ 100
             apartamento.
                                  de 1 ano.




Função ou Distribuição de Probabilidade

Seja “X” uma variável aleatória discreta, sejam x1, x2, x3, … , xn os possíveis valores que
a variável pode assumir.


Associamos a cada resultado “x” a probabilidade de ocorrência de tal forma que:


    a) P(xi) ≥ 0 , ou seja, a probabilidade de ocorrência de resultado xi é maior ou
    igual a zero.

         n
    b) ∑ i=1, P(xi)=1, ou seja, a soma das probabilidades de todos os valores possíveis de
    “X” é igual a 1.



        A Função de Probabilidade pode ser apresentada na forma de uma lista, de
        uma tabela, de um gráfico ou ainda de uma função matemática.




                                            171

## Página 172

Para entendermos melhor a explicação, vamos retomar o exemplo anterior usado na
variável aleatória.


Exemplo


A produção de aparelhos celulares de uma determinada fábrica pode ter duas classifica-
ções quanto à qualidade: Defeituoso ou Perfeito, ou seja, temos um experimento em que
os elementos do espaço amostral não são números e sim atributos, que vamos repre-
sentar como “D”=defeituoso e “P”=perfeito.


Suponha que três aparelhos sejam retirados aleatoriamente para inspeção. Seja a variá-
vel aleatória “X” definida como o número de aparelhos danificados.


Determine a função de probabilidade para a variável aleatória X.


Então:


    1) O experimento consiste em observar as condições da qualidade de três aparelhos
    celulares.


    Espaço Amostral: {PPP, PPD, PDP, PDD, DPP, DPD, DDP, DDD}, sendo “D”=defeituoso
    e “P”=perfeito
    2) A variável aleatória “X” representa o número de aparelhos danificados, podendo
    assumir os seguintes valores: 0,1,2 ou 3


    3) Distribuição de probabilidade: associação, a cada resultado “x” a probabilidade
    de ocorrência.




                                         172

## Página 173

Devemos, então, calcular a probabilidade de ocorrência de cada valor que é variável aleatória:


     PPP
     PPD
                      0                               Calculando as probabilidades, temos:
     PDP
                      1
     PDD
                      2                                                1
     DPP                                              P (x = 0) =
                      3                                                8
     DPD
     DDP
     DDD                                                               3
                                                      P (x = 1) =
                                                                       8
   Número de elementos do espaço
   amostral  n(Ω)= 8
                                                                       3
                                                      P (x = 2) =
   Se X=0 1 ocorrência                                                8

   Se X=1 3 ocorrências
                                                                       1
                                                      P (x = 3) =
   Se X=2  3 ocorrências                                              8


   Se X=3  1 ocorrência


                                Fonte: Elaborada pela autora (2020).




                    Distribuição de Probabilidade, expressa por uma tabela:


   Valores de “X”                   0                  1                   2           3

   Probabilidade de                 1                  3                   3           1
   ocorrência p(X=xi)               8                  8                   8           8


                                Fonte: Elaborada pela autora (2020).




                                                173

## Página 174

                         Distribuição de Probabilidade expressa por um gráfico:

             P(X = xi)                  Distribuição de Probabilidade

                  1/2

                  3/8

                  1/8

                    0
                                    0            1             2            3     xi
                                  Fonte: Elaborado pela autora (2020).



Esperança Matemática ou Média: E(X)

A Esperança Matemática, também chamada de média, é o valor em torno do qual estão
concentrados os resultados da variável aleatória em uma amostra suficientemente grande.


Seu cálculo é obtido pela fórmula:


E(X) = μ =    ∑ x i. P(xi)
Onde:
xi = valor da variável aleatória
P(xi) = respectiva probabilidade


         Ampliando o foco


        Em Estatística Descritiva, o valor em torno do qual os dados estão concentra-
        dos são chamamos de média e os representamos pelo símbolo X.


        Em Estatística Indutiva, o valor em torno do qual os resultados da variável alea-
        tória estão concentrados é chamado de Esperança Matemática e os represen-
        tamos pelo símbolo μ.




                                                  174

## Página 175

Variância (σ2)

Avalia o grau de dispersão dos valores da variável aleatória em torno da média. Seu cál-
culo é obtido pela fórmula:σ


σ2 =   ∑ (x – μ) · p(xi)
                i
                        2




Em que:
xi = valor da variável aleatória
μ = Valor esperado
P(xi) = respectiva probabilidade


Desvio-padrão (σ)

É a raiz quadrada da variância.


σ = σ2



          Ampliando o foco


         Em Estatística Descritiva, a variância é representada por s² e o desvio-
         -padrão por “s”.


         Em Estatística Indutiva, a variância é representada por σ 2 e o desvio-padrão por σ.




Coeficiente de Variação (CV)

É o quociente entre o desvio-padrão e a esperança matemática. Normalmente, é
expresso em %.


          σ
CV =                · 100
         E(X)




                                                 175

## Página 176

Se:


  CV < 10%                            Baixa dispersão.

  10% ≤ CV < 20%                      Média dispersão.

  20% ≤ CV < 30%                      Alta.

  CV ≥ 30%                            Muito alta (pequena representatividade da média).


Agora, vejamos um exemplo para entendermos a aplicação dos conceitos abordados
na prática.


Exemplo


O número de chamadas telefônicas recebidas por minuto em uma empresa de telemar-
keting e suas respectivas probabilidades está representado na tabela a seguir:


      Número de chamadas              0          1          2          3        4          5

         Probabilidades             0,18       0,39       0,24       0,14      0,04       0,01


      a) Qual o número esperado de chamadas recebidas em um minuto?


      E(X) = 0 · 0,18 + 1 · 0,39 + 2 · 0,24 + 3 · 0,14 + 4 · 0,04 + 5 · 0,01

      E(X) = 0 + 0,39 + 0,48 + 0,42 + 0,16 + 0,05 = 1,5


      Espera-se, aproximadamente, 1,5 chamadas por minuto.


      b) A média é uma medida confiável para essa distribuição?


      Para responder essa questão devemos verificar o coeficiente de variação, pois, se
      ele for maior do que 30%, há uma alta dispersão, então não devemos confiar na mé-
      dia, já que ela é afetada por valores discrepantes.


      - Calculando a variância:


       σ2 =   ∑ (x – μ) · p(xi)
                    i
                          2




                                                176

## Página 177

Número de
                                  0         1        2      3       4       5       ∑
chamadas (xi)


Probabilidades P(xi)             0,18      0,39     0,24   0,14    0,04    0,01



(xi – μ)2                        2,25      0,25     0,25   2,25    6,25   12,25



(xi – μ)2 · p(xi)                0,405    0,0975    0,06   0,315   0,25   0,1225   1,25




   Como podemos observar na tabela, (xi – μ)2 · p(xi) = 1,25, logo:


    σ2 =    ∑ (x – μ) · p(xi)
                     i
                             2




   então, σ 2 = 1,25


   - Calculando o desvio-padrão:


   σ = σ2


   σ = 1,25         1,118


   - Calculando o Coeficiente de Variação:


               σ
    CV =                 · 100
             E(X)


             1,118
    CV =                 · 100 = 74,53%
              1,5


   Como o coeficiente de variação é muito elevado, pois é maior do que 30%, os dados
   estão muito dispersos, então a média não é uma boa medida representativa para
   essa distribuição.




                                                   177

## Página 178

Função Densidade de Probabilidade (ou                                          simplesmente
Função Densidade)

Seja “X” uma variável aleatória contínua pertencente a um intervalo numérico.


Define-se a função densidade, a função cuja área sob a curva entre dois valores “a” e “b”
nos revela a probabilidade de que a variável aleatória X assuma um valor entre “a” e “b”
(a<b), de tal forma que:


     a) f(x) ≥ ∀ x ∈ , ou seja, a função assume apenas valores positivos ou nulo, assim o
     gráfico fica sobre o eixo-x, ou acima dele.
           +∞
     b) ∫- ∞ f(x) dx = 1, ou seja, a área total sob a curva de densidade é sempre igual a 1
           +∞
     c) ∫- ∞ f(x) dx = P(a ≤ X ≤ b) ∀ a < b, ou seja, a probabilidade de ocorrência de um evento
     no intervalo [a,b] é a área entre “a” e “b”.


                                                         A forma do gráfico
                                                         de uma função desnidade
                                                         pode variar consideravelmente,
                                                         mas a área sob a curva
                                                         sempre será igual a 1




            O gráfico da função desnidade se estende infinitamente para - ∞ e + ∞    x


                                 Fonte: Elaborado pela autora (2020).


No caso das variáveis aleatórias contínuas, não estaremos mais interessados em saber
a probabilidade de um determinado resultado, como fazemos com as variáveis aleatórias
discretas, mas sim em determinar a probabilidade de o resultado estar em um determi-
nado intervalo.


Como não há probabilidade associada a um ponto único, é indiferente o uso dos símbo-
los < ou ≤ para menor que ou > ou ≥ para maior que, ou seja:

∫- ∞+∞f(x) dx = P(a ≤ X ≤ b) = P(a < X < b) = P(a ≤ X < b) = P(a < X ≤ b)




                                                   178

## Página 179

            Curva densidade de probabilidade e a determinação das probabilidades



               P(a ≤ X ≤ b)                       P(X ≤ a)                         P(X ≥ b)




        a       b               x             a                  x          b                  x

             Figura 1                             Figura 1                      Figura 1
       a área sombreada é                   a área sombreada é            a área sombreada é
           P(a ≤ X ≤ b)                           P(X ≤ a)                      P(X ≥ b)



                                Fonte: Elaborado pela autora (2020).


Então, como vimos, para encontrar a probabilidade precisamos determinar a área sob
uma curva e isso pode ser feito por integral, geometria, tabelas e o uso de tecnologias
computacionais.


Medidas estatísticas

As medidas estatísticas Esperança Matemática, também chamada de Valor Esperado ou
Média e variância, são definidas de forma semelhante ao que estudamos para as variá-
veis aleatórias discretas, porém substituímos o somatório por integral.

                                                                     +∞
Esperança Matemática, Valor Esperado ou Média: E(X) = ∫- ∞ x · f(x)dx

                    +∞
Variância: σ2 = ∫- ∞ (x - E(X))2 · f(x)dx



            Ampliando o foco praticando


         Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca Vir-
         tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
         para reforçar o conhecimento.


         Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
         com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                                    179

## Página 180

Distribuições de probabilidades discretas

No Tópico 1 apresentamos a definição geral das distribuições de probabilidade. Porém,
existem variáveis aleatórias que apresentam certos padrões de comportamento. Para es-
sas variáveis foram desenvolvidos modelos de distribuições específicos, com a finalidade
de auxiliar no cálculo das probabilidades, bem como a esperança matemática e a variância.


Para determinarmos as probabilidades, a esperança e a variância, basta identificarmos
qual é a distribuição teórica que a variável aleatória pertence e usar as fórmulas pró-
prias da distribuição identificada.


Vamos iniciar o estudo pelos modelos de distribuições discretas, apresentando dois
modelos entre vários existentes e que são muito utilizados: a distribuição binomial e a
distribuição de Poison.


Modelo de Distribuição Binomial

A distribuição de probabilidade binomial é uma distribuição discreta de probabilidade que
tem inúmeras aplicações no mundo real. A principal característica desse modelo é que
conhecemos o número finito de repetições do experimento.


Condições para modelagem:

    1. O experimento consiste em um número finito de tentativas idênticas.
    2. Cada tentativa pode resultar apenas um de dois resultados possíveis: o “sucesso”
    (se acontece o evento) ou um “fracasso” (se o evento não acontece).
    3. A probabilidade de sucesso de cada prova é constante e igual a “p” e a probabili-
    dade do fracasso “q=1-p”.
    4. Os resultados das provas são independentes.


Exemplos:


          Experimento              Variável aleatória X       Resultados possíveis

                                                           Sucesso  a face
  Lançar uma moeda seis                                    observada é cara
  vezes e observar a face          X = sair a face cara
  superior.                                                Fracasso  a face
                                                           observada é coroa



                                           180

## Página 181

                                                            Sucesso  a criança
  Observar o sexo de 100           X= a criança ser do      é menina
  partos de uma maternidade.       sexo feminino            Fracasso  a criança
                                                            é menino

                                                            Sucesso  não
  Perfurar uma série de poços      X= não encontrar         se encontra petróleo
  de petróleo.                     petróleo                 Fracasso  encontra-se
                                                            petróleo

                                                            Sucesso  o aluno
  Responder aleatoriamente                                  acerta a questão
                                   X=acertar
  10 questões de uma
                                   a resposta correta       Fracasso  o aluno
  prova objetiva.
                                                            erra a questão

                                                            Sucesso  o aluno
  Responder aleatoriamente                                  erra a questão
  10 questões de uma               X=errar a resposta
  prova objetiva.                                           Fracasso  o aluno
                                                            acerta a questão

  Observar 10 animais                                       Sucesso  o animal apre-
  que foram submetidos a           X= apresentar a          senta a doença
  uma injeção com efeitos          doença                   Fracasso  o animal não
  cancerígenos.                                             apresenta a doença


Parâmetros de uma distribuição binomial: X ~ B(n,p)

São valores definidores de uma distribuição binomial. Uma variável aleatória binomial
fica completamente determinada quando conhecemos o número de tentativas e a pro-
babilidade de um sucesso.
Em que:
n = número total de ensaios.
p = probabilidade de sucesso em cada ensaio.


Fórmulas da Distribuição Binomial

Se soubermos os parâmetros “n” e “p” seremos capazes de responder qualquer pergunta
sobre a probabilidade de X, o valor esperado e a variância. Para isso, basta aplicarmos as
fórmulas da distribuição binomial, que são:




                                           181

## Página 182

  Cálculo das Probabilidades              Cálculo do Valor Esperado      Cálculo da Variância


  P(X=xi) = ( xn ) · px · q(n-x)          μ=n·p                          σ2 = n · p · q


  Em que:

  X= número de eventos discretos em um número finito de repetições.

  n = número de repetições do experimento.

  x = número de sucessos.
  p = a probabilidade de sucesso em cada prova.

  q = a probabilidade de fracasso em cada prova.

                   n!
   ( xn ) =                    = número de combinações de “n” elementos tomados x a x.
               x! (n - x)!



Vejamos um exemplo prático.


Exemplo 1


Um professor de Estatística elaborou uma avaliação de múltipla escolha, que consiste em
10 questões, cada uma com cinco alternativas para resposta e apenas uma dessas alter-
nativas tem a resposta correta. Suponha que os estudantes que irão a fazer a prova não
frequentaram as aulas e não estudaram para a avaliação. Sendo assim, eles irão marcar
a resposta aleatoriamente, o que informalmente chamamos de “chute”. Para ser aprova-
do, o aluno deve acertar no mínimo sete questões. Um estudante é selecionado ao acaso.


    a) Qual é a probabilidade de ele “chutar” as 10 questões e acertar 7?
    b) Qual é a probabilidade de ele ser aprovado?
    c) O valor esperado de acertos nessas condições.
    d) O que podemos concluir de um aluno que não estuda e “chuta” as questões
    da avaliação.


Resolução:


1º) Definir a variável aleatória, pois será essa definição que vai determinar quem terá o
sucesso e o fracasso.



                                                  182

## Página 183

Seja a variável aleatória X=número de acertos na avaliação.


Logo: Sucesso  acertar a questão
      Fracasso  errar a questão


2º) Verificar as hipóteses a serem atendidas para o uso das fórmulas da distribuição
binomial. São elas:


1 - Existe um número finito de repetições?
Sim  o aluno “chuta” 10 vezes, logo, n=10


2 - Só há 2 possíveis resultados para cada tentativa?
Sim acerto ou erro


3- Cada “chute” é independente?
Sim, pois um “chute” não influencia no resultado do outro.


4 - A probabilidade de sucesso em cada tentativa é constante?
Sim, em cada questão a probabilidade de acerto é igual a:


         número de opções corretas em cada questão       1
  p=                                                 =       = 0,2
              número de opções em cada questão           5


Logo, a probabilidade do fracasso é q = 1 - p = 1 - 0,2 = 0,8

Como todas as hipóteses foram verificadas, então: X ~ B(10 ; 0,2), ou seja, a variável alea-
tória X obedece a uma distribuição binomial, com parâmetros n=10 e p=0,2.


Com isso, podemos fazer o cálculo das probabilidades pedidas, usando as fórmulas da
distribuição binomial, como mostramos a seguir.


     a) Um estudante é selecionado ao acaso. Qual é a probabilidade de ele “chutar”
     as 10 questões e acertar 7?


     P(X=xi) = ( xn ) · px · q(n–x)


     P(X=7) = ( 10
                7 ) · (0,2) · (0,8)
                           7       (10-7)




                                             183

## Página 184

                   10!
     P(X=7) =                · (0,2)7 · (0,8)(10-7)
                  7! 3!


                   10!
     P(X=7) =                · (0,2)7 · (0,8)3
                7! (10-x)!


    P(X=7) = 120 · 0,0000128 · 0,512 = 0,000078643 0,0786%


    b) Um estudante é selecionado ao acaso. Qual é a probabilidade de ele ser aprovado?
    Para ser aprovado, o aluno tem que acertar pelo menos sete questões. Logo:


    P(X ≥ 7) = P(X=7) + P(X=8) + P(X=9) + P(X=10)


    Analogamente ao cálculo de P(X=7), vamos fazer P(X=8), P(X=9) e P(X=10), obtendo
    como resultado:


    P(X ≥ 7) = 0,00078643 + 0,00007373 + 0,00000410 + 0,00000010 = 0,00086436 =
    0,086436%


    c) E(x)=n . p = 10 . 0,2 = 2 questões
    Espera-se que um aluno que “chuta” 10 questões em uma avaliação, acerta 2.


Logo, com base nas respostas anteriores podemos concluir que vai ser muito difícil um es-
tudante ser aprovado em Estatística sem assistir às aulas e sem estudar para avaliações.


Modelo de Distribuição de Poisson

Esse modelo será utilizado para modelar experimentos discretos em que o número de
repetições não é conhecido.


A variável aleatória de Poisson é definida como o número de sucessos em certo intervalo
fixo e contínuo, que pode ser de tempo, comprimento, área, volume, massa e outros, sen-
do essa a principal característica desse modelo.


Exemplos:
    1. Número de carros que passam em um pedágio em uma hora.
    2. Número de vezes em que o corpo de bombeiros é chamado por dia para comba-
    ter incêndios em uma cidade.
    3. Número de acidentes de trabalho por semana em uma empresa industrial.


                                                  184

## Página 185

    4. Número de reparos em uma rodovia em um trecho de 100 km.
    5. Número de automóveis que chegam ao campus entre 7 e 10 horas da manhã.
    6. Número de microrganismos por cm³ de água contaminada.
    7. Número de vazamentos em 100 Km de tubulação.


Condições para modelagem

    1. Acertos individuais ocorrerão aleatoriamente e independentemente dentro um in-
    tervalo fixo de observação.


    2. O número de ocorrências de certo evento de interesse neste intervalo é uma variável
    discreta com valores possíveis 0, 1, 2, 3... . Observe que x não possui limite máximo.


    3. Uma distribuição de Poisson modela bem eventos “raros”. Fenômenos raros são
    aqueles que ocorrem com pouca frequência para qualquer intervalo de observação.
    Por exemplo, o número de automóveis da linha Gol que passam por um radar na
    estrada em um intervalo de uma hora e o número de automóveis Porsche Cayenne
    que passam nesse mesmo radar, nesse mesmo intervalo de uma hora. Certamente,
    o modelo de Poisson deverá ser mais adequado para o automóvel Porsche.


Parâmetros de uma distribuição de Poisson: X ~ P(μ)

A média μ é o único parâmetro para a distribuição de Poisson.
μ=λ.t
Em que:
μ = média de eventos discretos em “t” unidades de medida.
λ = taxa de frequência ou coeficiente de proporcionalidade por unidade de medida.
t = unidade de medida.


Fórmulas da Distribuição de Poisson

Se soubermos a taxa de frequência por unidade de medida (λ) e a unidade de medida (t),
seremos capazes de responder qualquer pergunta sobre a probabilidade de X, o valor es-
perado e a variância. Basta para isso aplicarmos as fórmulas da distribuição de Poisson,
que são:


  Cálculo das Probabilidades      Cálculo do Valor Esperado       Cálculo da Variância

               e -μ · μx
   P(X=xi) =                      μ=λ·t                           σ2 = λ · t
                  x!



                                           185

## Página 186

  Em que:
  X: número de eventos discretos em t unidades de medida.
  λ: taxa de frequência ou coeficiente de proporcionalidade por unidade de medida.
  t: unidade de medida.
  μ = λ t: média de eventos discretos em t unidades de medida.
  e = constante de Euler 2,71828.


Exemplo 1


Intervalo de tempo


A administradora de pedágios de uma rodovia está interessada em conhecer o número
de carros que chegam a uma determinada cabine em um intervalo de tempo de 15 mi-
nutos. Sabe-se que:


    • Em qualquer período de igual duração a probabilidade de chegar um carro no pe-
    dágio é a mesma.
    • O evento chegada de um carro na cabine do pedágio é independente da chegada
    de outro carro em qualquer período.
    • Observando dados históricos da rodovia, podemos estimar o número médio de
    carros que chegam em uma cabine desse pedágio como 10 carros no período de
    15 minutos.


Pede-se:
    a) Determine a probabilidade de chegarem exatamente 5 carros nessa cabine em
    um período de 15 minutos.
    b) Determine a probabilidade de chegar exatamente 1 carro nessa cabine em um
    período de 3 minutos.
.
Resolução:


Letra a)
Podemos verificar no enunciado que os pressupostos para a modelagem por Poisson
foram satisfeitos. Logo, usaremos esse modelo para responder às questões propostas.


    1) Definição da variável aleatória X:
    Seja X= número de carros que chegam em uma cabine de pedágio de uma rodovia
    em um intervalo de tempo de 15 minutos.
    Nesse caso, buscamos determinar a probabilidade que o número de carros que


                                           186

## Página 187

    chega em uma cabine de um pedágio de uma rodovia, em um intervalo qualquer de
    tempo de 15 minutos, seja exatamente igual a 5, ou seja, X=5


    2) Intervalo de tempo: t = 15 minutos.

                                                                                10
    3) Frequência nesse intervalo de tempo: λ = 10 carros a cada 15 minutos =
                                                                                15
                            10
    4) Média: μ = λ · t =      · 15 = 10 carros
                            15

Então,
X ~ P(10), ou seja, a variável aleatória X obedece à distribuição de Poisson, com média
igual a 10.
Estando com o parâmetro definido, vamos substituí-lo no modelo de Poisson:


           e-10 · 105       4,539992976
P(X=5) =                =                  = 0,0378 = 3,78%
              5!                120


Letra b)
Observamos que o intervalo de tempo agora modificou-se para três minutos. Logo:


    1) Definição da variável aleatória X:
    Seja X= número de carros que chegam em uma cabine de pedágio de uma rodovia
    em um intervalo de tempo qualquer de três minutos.
    Nesse caso, buscamos determinar a probabilidade que o número de carros que che-
    ga em uma cabine de pedágio de uma rodovia, em um intervalo qualquer de tempo
    de três minutos seja exatamente igual a 1, ou seja, X=1.


    2) Intervalo de tempo: t = 3 minutos.

                                                                                10
    3) Frequência nesse intervalo de tempo: λ= 10 carros a cada 15 minutos =
                                                                                15
                            10
    4) Média: μ = λ · t =      · 3 = 2 carros
                            15

    5) Então: X ~ P(2)


Então,
X ~ P(2), ou seja, a variável aleatória X obedece à distribuição de Poisson, com média
igual a 2. Estando com o parâmetro definido, vamos substituí-lo no modelo de Poisson:




                                                  187

## Página 188

             e-μ · μx
 P(X=xi) =
                x!


             e-2 · 21       0,2707
 P(X=1) =               =            = 0,2797 = 27,07%
               1!             1


Portanto, a probabilidade de que exatamente cinco carros cheguem em um período de
15 minutos nessa cabine é aproximadamente igual a 3,78%, enquanto a probabilidade
de que exatamente um arro chegue em um período de três minutos nessa cabine é
igual a 27,07%.


Exemplo 2


Intervalo de comprimento

Suponhamos que, após um mês de obras de recapeamento de um trecho de uma rodo-
via, a administradora dessa rodovia esteja interessada em verificar o número de defeitos
graves na pavimentação desse trecho. Sabe-se que:


    • Em quaisquer dois intervalos de mesma extensão nessa rodovia a probabilidade
    de ocorrência de ocorrer um defeito grave é a mesma.
    • Os eventos defeitos graves em uma rodovia são independentes, ou seja, a ocor-
    rência de um defeito grave em um intervalo não tem nenhuma dependência com a
    existência ou não de outro defeito em outro intervalo qualquer.
    • A quantidade de defeitos graves por Km pertence ao intervalo [0, +∞[.
    • Sabe-se que defeitos graves ocorrem um mês após as obras de recapeamento à
    taxa de dois defeitos por quilômetro.


Pede-se:
    a) A probabilidade de não haver nenhum defeito grave em um trecho de 3 km
    de rodovia.
    b) A probabilidade de existir pelo menos um defeito grave em um trecho de 3 km de
    rodovia.


Resolução:


Letra a)
Pelo enunciado verificamos que as hipóteses para a modelagem com a Distribuição de
Poisson estão satisfeitas.


                                              188

## Página 189

Primeiramente, vamos definir a variável aleatória de Poisson X.
X= número de defeitos graves em uma rodovia que aconteceram um mês após o reca-
peamento em um trecho de 3 km.
Podemos notar que estamos trabalhando em um intervalo de comprimento de 3km.
t = 3 km


Agora, vamos determinar a taxa de frequência nesse intervalo de tempo.

                              2 defeitos
λ= 2 defeitos a cada km =
                                1 km

                                    2
Logo, a média será: μ = λ · t =       · 3 = 6 defeitos
                                    1

Para calcular a probabilidade pedida basta usar a fórmula da distribuição de Poisson.


             e-μ · μx
 P(X=xi) =
                x!


             e-6 · 20       0,0025 · 1
 P(X=1) =               =                  = 0,0025 = 25%
               0!               1


Logo, existe uma probabilidade bem pequena, em torno de 0,25%, de não se encontrar ne-
nhum defeito grave em um trecho de 3Km de uma rodovia, um mês após o recapeamento.


Letra B)
Qual a probabilidade de existir pelo menos um defeito grave em um trecho de 3 km
de rodovia?


P(X ≥ 1) = P(X=1) + P(X=2) + P(X=3) + …


Como não sabemos o número final de defeitos, temos que determinar essa probabilida-
de utilizando o conceito de probabilidade complementar, ou seja:


P(X ≥ 1) = 1 - P(X=0)


Sabemos que P(X=0) = 0,0025, pois calculamos na letra a) desse exemplo.


Logo:
P(X ≥ 1) = 1 - 0,0025 = 0,9975 = 99,75%




                                                 189

## Página 190

Então, existe uma probabilidade de 99,75% de ocorrer pelo menos um defeito grave em
um trecho de 3 km da rodovia.



        Ampliando o foco


       Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca Vir-
       tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
       para reforçar o conhecimento.


       Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
       com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                              190

## Página 191

Distribuição de probabilidade Normal

Neste tópico apresentaremos duas distribuições contínuas de probabilidades — distribui-
ção Normal e distribuição Normal Padronizada, que nos auxiliam nos cálculos das pro-
babilidades, oferecendo-nos, já tabelado, o cálculo das áreas sob a curva. É fundamental
entendermos os conceitos que serão apresentados, pois assim teremos a alternativa
de resolver os cálculos de probabilidades sem que seja preciso resolver as integrais que
aparecem quando trabalhamos com as variáveis aleatórias contínuas.


Distribuição de probabilidade Normal ou Gausseana

A distribuição de probabilidade é:


    • Considerada a distribuição de probabilidade mais importante em toda Estatística
    para descrever uma variável aleatória contínua.
    • Usada na modelagem dos dados provenientes de fenômenos naturais, industriais
    e pesquisas em geral.
    • Amplamente usada em inferência estatística.
    • Desenvolvida por Moivre em 1733 e posteriormente por Laplace, usada por Gauss
    na derivação das equações do estudo de erros em repetições de medidas de um
    mesmo objeto, por isso é também conhecida como gausseana.


Temos algumas propriedades da distribuição Normal de probabilidade.


                           Propriedades da distribuição Normal


                                      Modelo Matemático

                 A curva Normal apresenta o seguinte modelo matemático:

                                                    1       (x – μ)2 ,
                                                            –
                                       f(x) =             e 2 σ2
                                                  σ 2π


                                     x μ ∈ ( - ∞; + ∞) e σ2 > 0


                                                Em que:




                                                  191

## Página 192

                                   μ = média
                               σ = desvio padrão
                                 σ 2 = variância
                                  e = 2,71828...
                                 π = 3,14159...


                               Formato da curva


         O formato da curva normal assemelha-se à forma de um sino.


                                                        σ




                                          μ
                            média = mediana = moda



                  Abrangência em função do desvio-padrão


Cerca de 68,26% da probabilidade total está entre a média e 1 desvio-padrão.
Cerca de 95,44% da probabilidade total está entre a média e 2 desvios-padrão.
Cerca de 99,73% da probabilidade total está entre a média e 3 desvios-padrão.


                             Observe graficamente:

                                        99, 73%

                                        95,44%




                                        68,26%




                     - 3σ   - 2σ - 1σ     μ      + 1σ + 2σ + 3σ

Normal




                                         192

## Página 193

                              Área sob a curva


   Quando calculamos a área total sob a curva normal sempre encontramos
                             resultado igual a 1.
     Isso significa 100% quando estamos investigando as probabilidades.




                                área total = 1



                                       μ




                                  Simetria


    A curva é simétrica em relação à média, portanto 50% da probabilidade
          está antes da média, que chamamos de ramos esquerdo,
              e 50% depois da média, denominado ramo direito.
                           Observe graficamente:




                                0,5         0,5



                                       μ




                                Parâmetros


Lembrando que a distribuição fica completamente definida por seus parâmetros
   e a distribuição Normal tem como parâmetros a média μ e a variância σ2.




                                      193

## Página 194

                                     Notação


                                   X ~(N μ σ2 )


        Lê-se: A variável aleatória “X” obedece a uma distribuição Normal,
           com parâmetros μ e σ2, sendo μ = média e σ2 = variância.




                                     Simetria


As medidas de posição, média, mediana e moda são coincidentes na curva Normal.
                              Observe graficamente:




                            média = mediana = moda




                                   Assíntodas


            A curva normal é assintótica em relação ao eixo-x, ou seja,
       tende a zero para valores tendendo a mais infinito ou menos infinito.




                            média = mediana = moda




                                       194

## Página 195

                                    Pontos de Inflexão


                     A curva normal apresenta dois pontos de inflexão,
                     invertendo a concavidade quando x = +1σ e x = -1σ.




                                    - 1σ        μ       + 1σ




Como calcular as probabilidades com o modelo de distribuição Normal?


Seja uma variável aleatória normal com média igual a 5 e a variância igual a 4. Suponha-
mos que desejamos calcular a probabilidade de a variável aleatória estar entre 3 e 6.


Como vimos, para determinar essa probabilidade basta determinarmos o valor da área
sob a curva no intervalo considerado.

                                                        σ=2




                                    3       5       6

                            Fonte: Elaborada pela autora (2020).

                                                               1
                                                             e- (x-5) .
                                                          6              2
Para calcular essa área e se resolvermos a integral: ∫3
                                                        2 2π    2·4
E coloquemos como reposta 0.532807.
Logo, P(3<X<6) 53,28%


Como o cálculo dessa integral não é nada trivial, já que ela não pode ser resolvida analiti-
camente, temos outra opção para determinarmos a área sob a curva, que irá fornecer-nos
a probabilidade. Vamos, então, fazer uma padronização na curva Normal e determinar a


                                            195

## Página 196

área sob a curva usando simplesmente uma tabela, em vez de calcularmos essa integral
pelo desenvolvimento em série. Para isso, vamos fazer uma transformação linear da va-
riável aleatória Normal para variável aleatória padronizada.


Distribuição Normal Padrão ou Curva Normal Padronizada

Também chamada de Standartizada ou reduzida, é a distribuição Normal com média
igual a zero (μ = 0), variância igual a 1, (σ2= 1) ,e consequentemente o desvio-padrão igual
a 1 (σ =1).


Seja X uma variável aleatória com distribuição N (μ, σ2).


Consideremos a transformação linear de X para Z-score:


          X-μi
Z=
           σ


Com isso, pode-se demonstrar que a média é sempre igual a zero e a variância
sempre igual a 1.
μ = 0 e σ 2= 1


Logo, o modelo matemático ficha reduzido a:
               1            Z2
 f(x) =             e   -

               2π           2
Z ∈ (-∞ + ∞)


Sendo assim, o formato da curva:


                                 Figura: Curva Normal Padronizada.

                                                           σ=1




                                                μ= 0

                                 Fonte: Elaborada pela autora (2020).




                                                 196

## Página 197

Com o intuito de auxiliar no cálculo das áreas sob a curva, que, como já vimos, são de difí-
cil resolução, usaremos a distribuição Normal Padrão. Nessa distribuição as áreas sob a
curva já foram calculadas e estão apresentadas em tabelas. Serão essas as tabelas que
nos auxiliarão nos cálculos das probabilidades.


Existem vários tipos de tabelas, porém as mais utilizadas são as que oferecem a área
para valores de z-score até a média e também que mostram a área cumulativa. Para esse
                         Tabela
estudo escolhemos utilizar         dadisponibilizada
                            a tabela   Distribuiçãoa seguir.
                                                     Normal

       DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
 z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
           0	
           0,0040	
       0,0080	
       0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
 0	
  
           0,0398	
      0,0438	
       0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
 0,1	
  
 0,2	
     0,0793	
      0,0832	
       0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

           0,1179	
      0,1217	
       0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
 0,3	
  
           0,1554	
      0,1591	
       0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
 0,4	
  
           0,1915	
      0,1950	
       0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
 0,5	
  
           0,2257	
      0,2291	
       0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
 0,6	
  
           0,2580	
      0,2611	
       0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
 0,7	
  
           0,2881	
      0,2910	
       0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
 0,8	
  
           0,3159	
      0,3186	
       0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
 0,9	
  
           0,3413	
      0,3438	
       0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
 1	
  
 1,1	
     0,3643	
      0,3665	
       0,3686	
       0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

           0,3849	
      0,3869	
       0,3888	
       0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
 1,2	
  
           0,4032	
      0,4049	
       0,4066	
       0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
 1,3	
  
           0,4192	
      0,4207	
       0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
 1,4	
  
           0,4332	
      0,4345	
       0,4357	
       0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
       0,4429	
       0,4441	
  
 1,5	
  
 1,6	
     0,4452	
      0,4463	
       0,4474	
       0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
       0,4535	
       0,4545	
  

           0,4554	
      0,4564	
       0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
 1,7	
  
           0,4641	
      0,4649	
       0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
 1,8	
  
           0,4713	
      0,4719	
       0,4726	
       0,4732	
       0,4738	
       0,4744	
       0,4750	
       0,4756	
       0,4761	
       0,4767	
  
 1,9	
  
           0,4772	
      0,4778	
       0,4783	
       0,4788	
       0,4793	
       0,4798	
       0,4803	
       0,4808	
       0,4812	
       0,4817	
  
 2	
  
 2,1	
     0,4821	
      0,4826	
       0,4830	
       0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

           0,4861	
      0,4864	
       0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
 2,2	
  
           0,4893	
      0,4896	
       0,4898	
       0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916	
  
 2,3	
  
           0,4918	
      0,4920	
       0,4922	
       0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
       0,4934	
       0,4936	
  
 2,4	
  
           0,4938	
      0,4940	
       0,4941	
       0,4943	
       0,4945	
       0,4946	
       0,4948	
       0,4949	
       0,4951	
       0,4952	
  
 2,5	
  
 2,6	
     0,4953	
      0,4955	
       0,4956	
       0,4957	
       0,4959	
       0,4960	
       0,4961	
       0,4962	
       0,4963	
       0,4964	
  

           0,4965	
      0,4966	
       0,4967	
       0,4968	
       0,4969	
       0,4970	
       0,4971	
       0,4972	
       0,4973	
       0,4974	
  
 2,7	
  
           0,4974	
      0,4975	
       0,4976	
       0,4977	
       0,4977	
       0,4978	
       0,4979	
       0,4979	
       0,4980	
       0,4981	
  
 2,8	
  
           0,4981	
      0,4982	
       0,4982	
       0,4983	
       0,4984	
       0,4984	
       0,4985	
       0,4985	
       0,4986	
       0,4986	
  
 2,9	
  
           0,4987	
      0,4987	
       0,4987	
       0,4988	
       0,4988	
       0,4989	
       0,4989	
       0,4989	
       0,4990	
       0,4990	
  
 3	
  
           0,4990	
      0,4991	
       0,4991	
       0,4991	
            197
                                                                      0,4992	
       0,4992	
       0,4992	
       0,4992	
       0,4993	
       0,4993	
  
 3,1	
  
           0,4993	
      0,4993	
       0,4994	
       0,4994	
       0,4994	
       0,4994	
       0,4994	
       0,4995	
       0,4995	
       0,4995	
  
 3,2

## Página 198

           0,4938	
                 0,4940	
             0,4941	
            0,4943	
         0,4945	
      0,4946	
         0,4948	
            0,4949	
            0,4951	
          0,4952	
  
 2,5	
  
 2,6	
     0,4953	
                 0,4955	
             0,4956	
            0,4957	
         0,4959	
      0,4960	
         0,4961	
            0,4962	
            0,4963	
          0,4964	
  

           0,4965	
                 0,4966	
             0,4967	
            0,4968	
         0,4969	
      0,4970	
         0,4971	
            0,4972	
            0,4973	
          0,4974	
  
 2,7	
  
           0,4974	
                 0,4975	
             0,4976	
            0,4977	
         0,4977	
      0,4978	
         0,4979	
            0,4979	
            0,4980	
          0,4981	
  
 2,8	
  
           0,4981	
                 0,4982	
             0,4982	
            0,4983	
         0,4984	
      0,4984	
         0,4985	
            0,4985	
            0,4986	
          0,4986	
  
 2,9	
  
           0,4987	
                 0,4987	
             0,4987	
            0,4988	
         0,4988	
      0,4989	
         0,4989	
            0,4989	
            0,4990	
          0,4990	
  
 3	
  
           0,4990	
                 0,4991	
             0,4991	
            0,4991	
         0,4992	
      0,4992	
         0,4992	
            0,4992	
            0,4993	
          0,4993	
  
 3,1	
  
           0,4993	
                 0,4993	
             0,4994	
            0,4994	
         0,4994	
      0,4994	
         0,4994	
            0,4995	
            0,4995	
          0,4995	
  
 3,2	
  
           0,4995	
                 0,4995	
             0,4995	
            0,4996	
         0,4996	
      0,4996	
         0,4996	
            0,4996	
            0,4996	
          0,4997	
  
 3,3	
  
           0,4997	
                 0,4997	
             0,4997	
            0,4997	
         0,4997	
      0,4997	
         0,4997	
            0,4997	
            0,4997	
          0,4998	
  
 3,4	
  
 3,5	
     0,4998	
                 0,4998	
             0,4998	
            0,4998	
         0,4998	
      0,4998	
         0,4998	
            0,4998	
            0,4998	
          0,4998	
  

           0,4998	
                 0,4998	
             0,4999	
            0,4999	
         0,4999	
      0,4999	
         0,4999	
            0,4999	
            0,4999	
          0,4999	
  
 3,6	
  
           0,4999	
                 0,4999	
             0,4999	
            0,4999	
         0,4999	
      0,4999	
         0,4999	
            0,4999	
            0,4999	
          0,4999	
  
 3,7	
  
           0,4999	
                 0,4999	
             0,4999	
            0,4999	
         0,4999	
      0,4999	
         0,4999	
            0,4999	
            0,4999	
          0,4999	
  
 3,8	
  
           0,5000	
                 0,5000	
             0,5000	
            0,5000	
         0,5000	
      0,5000	
         0,5000	
            0,5000	
            0,5000	
          0,5000	
  
 3,9	
  
 4	
       0,5000	
                 0,5000	
             0,5000	
            0,5000	
         0,5000	
      0,5000	
         0,5000	
            0,5000	
            0,5000	
          0,5000	
  


Fonte: http://ead.uva.br///recurso/DEF/EST/u3_c3_r1/anexo/Tabela-da-Distribuicao-Normal.pdf



                      Importante


             Ao escolher uma tabela precisamos estar atentos ao tipo de probabilidade que
             ela apresenta.
             Existem tabelas que fornecem a área:
                          1- de 0 a k
                          2- de - ∞ a k
                          3- de k a + + ∞
             Normalmente, essa informação vem no cabeçalho da tabela.


             Os nossos exercícios Tabela
                                  serão resolvidos com a tabela
                                          da Distribuição       do tipo 1, conforme pode-
                                                           Normal
             mos ver no cabeçalho:

                      DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
              z	
         0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
                            0	
                  0,0040	
       0,0080	
         0,0120	
      0,0160	
     0,0199	
     0,0239	
         0,0279	
      0,0319	
          0,0359	
  
              0	
  
                            0,0398	
             0,0438	
       0,0478	
         0,0517	
      0,0557	
     0,0596	
     0,0636	
         0,0675	
      0,0714	
          0,0753	
  
              0,1	
  
      0,2	
                 0,0793	
             0,0832	
       0,0871	
         0,0910	
      0,0948	
     0,0987	
     0,1026	
         0,1064	
      0,1103	
          0,1141	
  
Como devemos  0,1179	
  
                         usar  a tabela
                           0,1217	
   0,1255	
  
                                                 de0,1293	
  
                                                    distribuição
                                                              0,1331	
  
                                                                         Normal
                                                                         0,1368	
  
                                                                                       padrão?
                                                                                    0,1406	
   0,1443	
                                                 0,1480	
          0,1517	
  
              0,3	
  
                            0,1554	
             0,1591	
       0,1664	
         0,1664	
      0,1700	
     0,1736	
     0,1772	
         0,1808	
      0,1844	
          0,1879	
  
              0,4	
  
1º Calcular
        0,5	
   o0,1915	
  
                  valor de      Z com
                            0,1950	
       a fórmula
                                       0,1985	
   0,2019	
                                     0,2054	
     0,2088	
     0,2123	
         0,2157	
      0,2190	
          0,2224	
  

           X 0,6	
  
             -μ             0,2257	
             0,2291	
       0,2324	
         0,2357	
      0,2389	
     0,2422	
     0,2454	
         0,2486	
      0,2517	
          0,2549	
  

 Z=           0,7	
         0,2580	
             0,2611	
       0,2642	
         0,2673	
      0,2704	
     0,2734	
     0,2764	
         0,2794	
      0,2823	
          0,2852	
  
        σ 0,2881	
   0,2910	
   0,2939	
   0,2967	
   0,2995	
   0,3023	
   0,3051	
   0,3078	
   0,3106	
   0,3133	
  
        0,8	
  
arredondando        com a
        0,9	
   0,3159	
       precisão
                           0,3186	
     0,3212	
   de0,3238	
  
                                                       duas 0,3264	
  
                                                                 casas 0,3289	
  
                                                                              decimais.  0,3315	
  Algumas
                                                                                                      0,3340	
   tabelas 0,3389	
  
                                                                                                                  0,3365	
    não apresen-
tam valores
        1	
     negativos
                0,3413	
   0,3438	
  para   Z, porém
                                        0,3461	
     0,3485	
   como
                                                                 0,3508	
   a 0,3531	
  
                                                                               curva 0,3554	
  
                                                                                         Normal0,3577	
  é simétrica
                                                                                                                  0,3599	
   em relação à
                                                                                                                             0,3621	
  

                0,3643	
   0,3665	
     0,3686	
     0,3708	
    0,3729	
     0,3749	
   0,3770	
     0,3790	
    0,3810	
   0,3830	
  
média, o sinal não terá nenhuma relevância e poderá ser desprezado.
        1,1	
  
                            0,3849	
             0,3869	
       0,3888	
         0,3907	
      0,3925	
     0,3944	
     0,3962	
         0,3980	
      0,3997	
          0,4015	
  
              1,2	
  
                            0,4032	
             0,4049	
       0,4066	
         0,4082	
      0,4099	
     0,4115	
     0,4131	
         0,4147	
      0,4162	
          0,4177	
  
              1,3	
  
                            0,4192	
             0,4207	
       0,4222	
         0,4236	
      0,4251	
     0,4265	
     0,4279	
         0,4292	
      0,4306	
          0,4319	
  
              1,4	
  
                            0,4332	
             0,4345	
       0,4357	
         0,4370	
      0,4382	
     0,4394	
     0,4406	
         0,4418	
      0,4429	
          0,4441	
  
              1,5	
                                                                                  198
              1,6	
         0,4452	
             0,4463	
       0,4474	
         0,4484	
      0,4495	
     0,4505	
     0,4515	
         0,4525	
      0,4535	
          0,4545	
  

                            0,4554	
             0,4564	
       0,4573	
         0,4582	
      0,4591	
     0,4599	
     0,4608	
         0,4616	
      0,4625	
          0,4633	
  
              1,7

## Página 199

2º Identificar a parte inteira e a primeira casa decimal, na primeira coluna da tabela.
3º Identificar a segunda casa decimal, na primeira linha com valores na tabela.
4º Fazer o cruzamento da linha com a coluna e buscar o valor da área sob a curva de
zero até z.


Vejamos um exemplo a seguir.


Exemplo
Considere X ≥ 800, μ =850, σ = 45


1 - Calculando o Z-score:
         X-μ                     800 - 850
 Z=                         =  = -1,111111
        σ             45
pela simetria da curva Normal = 1,111111. Com precisão de duas casas decimais vamos
considerar Z=1,11


2 - Vamos buscar na primeira coluna da tabela o valor 1,1

         Primeira                                   Tabela da Distribuição Normal
          coluna


                DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
          z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
                    0	
           0,0040	
       0,0080	
       0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
          0	
  
                    0,0398	
      0,0438	
       0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
          0,1	
  
          0,2	
     0,0793	
      0,0832	
       0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

                    0,1179	
      0,1217	
       0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
          0,3	
  
                    0,1554	
      0,1591	
       0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
          0,4	
  
                    0,1915	
      0,1950	
       0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
          0,5	
  
                    0,2257	
      0,2291	
       0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
          0,6	
  
                    0,2580	
      0,2611	
       0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
          0,7	
  
                    0,2881	
      0,2910	
       0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
          0,8	
  
                    0,3159	
      0,3186	
       0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
          0,9	
  
                    0,3413	
      0,3438	
       0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
          1	
  
          1,1	
     0,3643	
      0,3665	
       0,3686	
       0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

                    0,3849	
      0,3869	
       0,3888	
       0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
          1,2	
  
                    0,4032	
      0,4049	
       0,4066	
       0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
          1,3	
  
                    0,4192	
      0,4207	
       0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
          1,4	
  
3 - Na 1,5	
  
       primeira0,4332	
  linha  com valores
                             0,4345	
   0,4357	
   na   tabela,
                                                   0,4370	
       identificamos
                                                              0,4382	
   0,4394	
      a segunda
                                                                                    0,4406	
   0,4418	
   casa   decimal,
                                                                                                            0,4429	
   0,4441	
   no

caso 0,01
       1,6	
   0,4452	
      0,4463	
   0,4474	
   0,4484	
   0,4495	
   0,4505	
   0,4515	
   0,4525	
     0,4535	
   0,4545	
  

                    0,4554	
      0,4564	
       0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
          1,7	
  
                    0,4641	
      0,4649	
       0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
          1,8	
  
                    0,4713	
      0,4719	
       0,4726	
       0,4732	
       0,4738	
       0,4744	
       0,4750	
       0,4756	
       0,4761	
       0,4767	
  
          1,9	
  
                    0,4772	
      0,4778	
       0,4783	
       0,4788	
       0,4793	
       0,4798	
       0,4803	
       0,4808	
       0,4812	
       0,4817	
  
          2	
  
          2,1	
     0,4821	
      0,4826	
       0,4830	
       0,4834	
       0,4838	
  
                                                                               199            0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

                    0,4861	
      0,4864	
       0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
          2,2	
  
                    0,4893	
      0,4896	
       0,4898	
       0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916

## Página 200

                                                                                Tabela da Distribuição Normal


Primeira                              DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
linha                           z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
de valores                      0	
        0	
             0,0040	
          0,0080	
        0,0120	
     0,0160	
     0,0199	
        0,0239	
      0,0279	
          0,0319	
        0,0359	
  

                                           0,0398	
        0,0438	
          0,0478	
        0,0517	
     0,0557	
     0,0596	
        0,0636	
      0,0675	
          0,0714	
        0,0753	
  
                                0,1	
  
                                0,2	
      0,0793	
        0,0832	
          0,0871	
        0,0910	
     0,0948	
     0,0987	
        0,1026	
      0,1064	
          0,1103	
        0,1141	
  

                                           0,1179	
        0,1217	
          0,1255	
        0,1293	
     0,1331	
     0,1368	
        0,1406	
      0,1443	
          0,1480	
        0,1517	
  
                                0,3	
  
                                           0,1554	
        0,1591	
          0,1664	
        0,1664	
     0,1700	
     0,1736	
        0,1772	
      0,1808	
          0,1844	
        0,1879	
  
                                0,4	
  
                                           0,1915	
        0,1950	
          0,1985	
        0,2019	
     0,2054	
     0,2088	
        0,2123	
      0,2157	
          0,2190	
        0,2224	
  
                                0,5	
  
                                           0,2257	
        0,2291	
          0,2324	
        0,2357	
     0,2389	
     0,2422	
        0,2454	
      0,2486	
          0,2517	
        0,2549	
  
                                0,6	
  
                                           0,2580	
        0,2611	
          0,2642	
        0,2673	
     0,2704	
     0,2734	
        0,2764	
      0,2794	
          0,2823	
        0,2852	
  
                                0,7	
  
                                           0,2881	
        0,2910	
          0,2939	
        0,2967	
     0,2995	
     0,3023	
        0,3051	
      0,3078	
          0,3106	
        0,3133	
  
                                0,8	
  
                                           0,3159	
        0,3186	
          0,3212	
        0,3238	
     0,3264	
     0,3289	
        0,3315	
      0,3340	
          0,3365	
        0,3389	
  
                                0,9	
  
                                           0,3413	
        0,3438	
          0,3461	
        0,3485	
     0,3508	
     0,3531	
        0,3554	
      0,3577	
          0,3599	
        0,3621	
  
                                1	
  
                                1,1	
      0,3643	
        0,3665	
          0,3686	
        0,3708	
     0,3729	
     0,3749	
        0,3770	
      0,3790	
          0,3810	
        0,3830	
  

                                           0,3849	
        0,3869	
          0,3888	
        0,3907	
     0,3925	
     0,3944	
        0,3962	
      0,3980	
          0,3997	
        0,4015	
  
                                1,2	
  
                                           0,4032	
        0,4049	
          0,4066	
        0,4082	
     0,4099	
     0,4115	
        0,4131	
      0,4147	
          0,4162	
        0,4177	
  
                                1,3	
  
                                           0,4192	
        0,4207	
          0,4222	
        0,4236	
     0,4251	
     0,4265	
        0,4279	
      0,4292	
          0,4306	
        0,4319	
  
                                1,4	
  
Logo, temos este        Tabela
                 resultado:
               1,5	
      0,4345	
   da         Distribuição
                                           0,4332	
  
                                     0,4357	
     0,4370	
   0,4382	
   Normal
                                                                        0,4394	
   0,4406	
                                                          0,4418	
          0,4429	
        0,4441	
  

                                1,6	
      0,4452	
        0,4463	
          0,4474	
        0,4484	
     0,4495	
     0,4505	
        0,4515	
      0,4525	
          0,4535	
        0,4545	
  

                                           0,4554	
        0,4564	
          0,4573	
        0,4582	
     0,4591	
     0,4599	
        0,4608	
      0,4616	
          0,4625	
        0,4633	
  
                                1,7	
  
                             0,4641	
  
        DISTRIBUIÇÃO	
  N
                    1,8	
  ORMAL	
      –	
  Á0,4649	
      0,4656	
  
                                                REA	
  DE	
   0	
  até	
  K0,4664	
             0,4671	
   0,4678	
  
                                                                            	
  =>	
  	
  	
  probabilidade	
         da	
  0,4686	
   0,4693	
  
                                                                                                                             variável	
   z	
  estar	
  0,4699	
  
                                                                                                                                                          entre	
  00,4706	
  
                                                                                                                                                                     	
  e	
  k	
  
                             0,4713	
         0,4719	
      0,4726	
       0,4732	
             0,4738	
   0,4744	
     0,4750	
       0,4756	
      0,4761	
       0,4767	
  
  z	
       0	
     1,9	
  
                   0,01	
         0,02	
                 0,03	
        0,04	
                   0,05	
        0,06	
             0,07	
           0,08	
           0,09	
  
                                         0,4772	
   0,4778	
       0,4783	
     0,4788	
                  0,4793	
     0,4798	
        0,4803	
      0,4808	
          0,4812	
        0,4817	
  
             0	
               2	
  
                              0,0040	
        0,0080	
         0,0120	
       0,0160	
                    0,0199	
        0,0239	
           0,0279	
             0,0319	
          0,0359	
  
  0	
  
                               2,1	
   0,4821	
     0,4826	
       0,4830	
     0,4834	
                  0,4838	
     0,4842	
        0,4846	
      0,4850	
          0,4854	
        0,4857	
  
             0,0398	
         0,0438	
             0,0478	
             0,0517	
          0,0557	
        0,0596	
        0,0636	
           0,0675	
             0,0714	
          0,0753	
  
  0,1	
                         2,2	
      0,4861	
        0,4864	
          0,4868	
        0,4871	
     0,4875	
     0,4878	
        0,4881	
      0,4884	
          0,4887	
        0,4890	
  

  0,2	
      0,0793	
         0,0832	
   0,4893	
  
                                              0,0871	
  0,4896	
   0,0910	
  
                                                                       0,4898	
   0,0948	
  
                                                                                    0,4901	
              0,0987	
  
                                                                                                          0,4904	
        0,1026	
   0,4909	
  
                                                                                                                       0,4906	
           0,1064	
  0,4911	
   0,1103	
  
                                                                                                                                                                   0,4913	
   0,1141	
  
                                                                                                                                                                                0,4916	
  
                                2,3	
  
             0,1179	
         0,1217	
      0,1255	
  0,4920	
   0,1293	
  
                                                                     0,4922	
   0,1331	
                  0,1368	
        0,1406	
   0,4931	
  
                                                                                                                                          0,1443	
  0,4932	
   0,1480	
             0,1517	
  
  0,3	
                        2,4	
   0,4918	
                                   0,4925	
                0,4927	
     0,4929	
                                    0,4934	
           0,4936	
  

             0,1554	
         0,1591	
   0,4938	
  
                               2,5	
          0,1664	
  0,4940	
   0,1664	
  
                                                                       0,4941	
   0,1700	
  
                                                                                    0,4943	
              0,4945	
  
                                                                                                          0,1736	
     0,4946	
  
                                                                                                                          0,1772	
   0,4948	
  
                                                                                                                                          0,1808	
  0,4949	
   0,1844	
  
                                                                                                                                                                   0,4951	
           0,4952	
  
                                                                                                                                                                                    0,1879	
  
  0,4	
  
                               2,6	
       0,4953	
        0,4955	
          0,4956	
        0,4957	
     0,4959	
     0,4960	
        0,4961	
      0,4962	
          0,4963	
        0,4964	
  
             0,1915	
         0,1950	
             0,1985	
             0,2019	
          0,2054	
        0,2088	
        0,2123	
           0,2157	
             0,2190	
          0,2224	
  
  0,5	
                                    0,4965	
        0,4966	
          0,4967	
        0,4968	
     0,4969	
     0,4970	
        0,4971	
      0,4972	
          0,4973	
        0,4974	
  
                                2,7	
  
             0,2257	
         0,2291	
             0,2324	
             0,2357	
          0,2389	
        0,2422	
        0,2454	
           0,2486	
             0,2517	
          0,2549	
  
  0,6	
                         2,8	
      0,4974	
        0,4975	
          0,4976	
        0,4977	
     0,4977	
     0,4978	
        0,4979	
      0,4979	
          0,4980	
        0,4981	
  
             0,2580	
         0,2611	
   0,4981	
  
                                              0,2642	
  0,4982	
   0,2673	
       0,2704	
                0,2734	
        0,2764	
   0,4985	
  
                                                                                                                                          0,2794	
  0,4985	
   0,2823	
       0,2852	
  
  0,7	
                         2,9	
                                  0,4982	
     0,4983	
              0,4984	
     0,4984	
                                    0,4986	
     0,4986	
  

             0,2881	
         0,2910	
        0,2939	
  0,4987	
   0,2967	
  
                                                                       0,4987	
   0,2995	
                0,3023	
        0,3051	
   0,4989	
  
                                                                                                                                          0,3078	
  0,4989	
   0,3106	
             0,3133	
  
  0,8	
                        3	
       0,4987	
                                   0,4988	
              0,4988	
     0,4989	
                                    0,4990	
           0,4990	
  

             0,3159	
         0,3186	
   0,4990	
  
                                3,1	
         0,3212	
  0,4991	
   0,3238	
  
                                                                       0,4991	
             0,4991	
  
                                                                                          0,3264	
        0,4992	
  
                                                                                                          0,3289	
     0,4992	
  
                                                                                                                          0,3315	
   0,4992	
  
                                                                                                                                          0,3340	
  0,4992	
   0,3365	
  
                                                                                                                                                                   0,4993	
           0,4993	
  
                                                                                                                                                                                    0,3389	
  
  0,9	
  
                                         0,4993	
   0,4993	
       0,4994	
     0,4994	
                  0,4994	
     0,4994	
        0,4994	
      0,4995	
          0,4995	
        0,4995	
  
             0,3413	
          3,2	
  
                              0,3438	
        0,3461	
         0,3485	
       0,3508	
                    0,3531	
        0,3554	
           0,3577	
             0,3599	
          0,3621	
  
  1	
  
                               3,3	
   0,4995	
     0,4995	
       0,4995	
     0,4996	
                  0,4996	
     0,4996	
        0,4996	
      0,4996	
          0,4996	
        0,4997	
  
  1,1	
      0,3643	
         0,3665	
             0,3686	
             0,3708	
          0,3729	
        0,3749	
        0,3770	
           0,3790	
             0,3810	
          0,3830	
  
                                           0,4997	
        0,4997	
          0,4997	
        0,4997	
     0,4997	
     0,4997	
        0,4997	
      0,4997	
          0,4997	
        0,4998	
  
                                3,4	
  
             0,3849	
         0,3869	
   0,4998	
  
                                              0,3888	
  0,4998	
   0,3907	
       0,3925	
                0,3944	
        0,3962	
   0,4998	
  
                                                                                                                                          0,3980	
  0,4998	
   0,3997	
       0,4015	
  
  1,2	
                         3,5	
                                  0,4998	
     0,4998	
              0,4998	
     0,4998	
                                    0,4998	
     0,4998	
  

             0,4032	
         0,4049	
      0,4066	
  0,4998	
   0,4082	
  
                                                                     0,4999	
   0,4099	
                  0,4115	
        0,4131	
   0,4999	
  
                                                                                                                                          0,4147	
  0,4999	
   0,4162	
             0,4177	
  
  1,3	
                        3,6	
   0,4998	
                                   0,4999	
                0,4999	
     0,4999	
                                    0,4999	
           0,4999	
  

             0,4192	
         0,4207	
   0,4999	
  
                               3,7	
          0,4222	
  0,4999	
   0,4236	
  
                                                                       0,4999	
   0,4251	
  
                                                                                    0,4999	
              0,4999	
  
                                                                                                          0,4265	
     0,4999	
  
                                                                                                                          0,4279	
   0,4999	
  
                                                                                                                                          0,4292	
  0,4999	
   0,4306	
  
                                                                                                                                                                   0,4999	
           0,4999	
  
                                                                                                                                                                                    0,4319	
  
  1,4	
  
                                           0,4999	
        0,4999	
          0,4999	
        0,4999	
     0,4999	
     0,4999	
        0,4999	
      0,4999	
          0,4999	
        0,4999	
  
             0,4332	
          3,8	
  
                              0,4345	
             0,4357	
             0,4370	
          0,4382	
        0,4394	
        0,4406	
           0,4418	
             0,4429	
          0,4441	
  
  1,5	
                                    0,5000	
        0,5000	
          0,5000	
        0,5000	
     0,5000	
     0,5000	
        0,5000	
      0,5000	
          0,5000	
        0,5000	
  
                                3,9	
  
  1,6	
      0,4452	
         0,4463	
             0,4474	
             0,4484	
          0,4495	
        0,4505	
        0,4515	
           0,4525	
             0,4535	
          0,4545	
  
                                4	
        0,5000	
        0,5000	
          0,5000	
        0,5000	
     0,5000	
     0,5000	
        0,5000	
      0,5000	
          0,5000	
        0,5000	
  
             0,4554	
         0,4564	
             0,4573	
             0,4582	
          0,4591	
        0,4599	
        0,4608	
           0,4616	
             0,4625	
          0,4633	
  
  1,7	
  
             0,4641	
         0,4649	
             0,4656	
             0,4664	
          0,4671	
        0,4678	
        0,4686	
           0,4693	
             0,4699	
          0,4706	
  
  1,8	
  
             0,4713	
         0,4719	
             0,4726	
             0,4732	
          0,4738	
        0,4744	
        0,4750	
           0,4756	
             0,4761	
          0,4767	
  
  1,9	
  
             0,4772	
         0,4778	
             0,4783	
             0,4788	
          0,4793	
        0,4798	
        0,4803	
           0,4808	
             0,4812	
          0,4817	
  
  2	
  
  2,1	
      0,4821	
         0,4826	
             0,4830	
             0,4834	
          0,4838	
        0,4842	
        0,4846	
           0,4850	
             0,4854	
          0,4857	
  

             0,4861	
         0,4864	
             0,4868	
             0,4871	
          0,4875	
        0,4878	
        0,4881	
           0,4884	
             0,4887	
          0,4890	
  
  2,2	
  
             0,4893	
         0,4896	
             0,4898	
             0,4901	
          0,4904	
  
                                                                                              200         0,4906	
        0,4909	
           0,4911	
             0,4913	
          0,4916	
  
  2,3	
  
             0,4918	
         0,4920	
             0,4922	
             0,4925	
          0,4927	
        0,4929	
        0,4931	
           0,4932	
             0,4934	
          0,4936	
  
  2,4

## Página 201

4 - Fazemos o cruzamento da linha com a coluna e identificamos o valor da área sob a
                      Tabela da Distribuição Normal
curva de zero até z.


        DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
  z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
               0	
           0,0040	
     0,0080	
      0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
  0	
  
               0,0398	
      0,0438	
     0,0478	
      0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
  0,1	
  
  0,2	
        0,0793	
      0,0832	
     0,0871	
      0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

               0,1179	
      0,1217	
     0,1255	
      0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
  0,3	
  
               0,1554	
      0,1591	
     0,1664	
      0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
  0,4	
  
               0,1915	
      0,1950	
     0,1985	
      0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
  0,5	
  
               0,2257	
      0,2291	
     0,2324	
      0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
  0,6	
  
               0,2580	
      0,2611	
     0,2642	
      0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
  0,7	
  
               0,2881	
      0,2910	
     0,2939	
      0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
  0,8	
  
               0,3159	
      0,3186	
     0,3212	
      0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
  0,9	
  
               0,3413	
      0,3438	
     0,3461	
      0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
  1	
  
  1,1	
        0,3643	
      0,3665	
     0,3686	
      0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

               0,3849	
      0,3869	
     0,3888	
      0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
  1,2	
  
               0,4032	
      0,4049	
     0,4066	
      0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
  1,3	
  
               0,4192	
      0,4207	
     0,4222	
      0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
  1,4	
  
Logo a0,4332	
  
       área de 0,4345	
  
                 zero até0,4357	
  
                          z=1,11 será
                                    0,4370	
  a mesma
                                                 0,4382	
   área  de zero
                                                              0,4394	
       até Z=-1,11
                                                                         0,4406	
   0,4418	
   que é  0,3665
                                                                                                 0,4429	
   0,4441	
  
  1,5	
  
  1,6	
        0,4452	
      0,4463	
     0,4474	
      0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
       0,4535	
       0,4545	
  
Como     calcular a
 1,7	
   0,4554	
       probabilidade
                    0,4564	
   0,4573	
   0,4582	
  qualquer
                                            de         0,4591	
   distribuição
                                                                    0,4599	
       Normal?
                                                                               0,4608	
   0,4616	
                                 0,4625	
       0,4633	
  

               0,4641	
      0,4649	
     0,4656	
      0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
  1,8	
  
Para    determinar a
 1,9	
   0,4713	
        probabilidade
                    0,4719	
   0,4726	
     utilizando
                                          0,4732	
        a 0,4744	
  
                                                     0,4738	
   distribuição0,4750	
  
                                                                              Normal,0,4756	
  
                                                                                        devemos      seguir
                                                                                                0,4761	
      os se-
                                                                                                           0,4767	
  

guintes
 2	
       passos:0,4778	
   0,4783	
   0,4788	
   0,4793	
   0,4798	
   0,4803	
   0,4808	
   0,4812	
   0,4817	
  
         0,4772	
  

  2,1	
        0,4821	
      0,4826	
     0,4830	
      0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

  2,2	
   1º)0,4861	
  
               Determine0,4864	
  os valores
                                      0,4868	
   da  média0,4875	
  
                                                  0,4871	
   μ e do 0,4878	
  
                                                                     desvio-padrão
                                                                               0,4881	
   σ0,4884	
  
                                                                                            e nomeie      a distribuição
                                                                                                      0,4887	
   0,4890	
  

  2,3	
   Normal:
           0,4893	
   X~N(μ,
                       0,4896	
   σ 2)0,4898	
          0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916	
  

               0,4918	
      0,4920	
     0,4922	
      0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
       0,4934	
       0,4936	
  
  2,4	
  
  2,5	
   2º)0,4938	
  
               Enuncie  0,4940	
   0,4941	
  
                           a probabilidade    0,4943	
  
                                                pedida.0,4945	
                       0,4946	
       0,4948	
       0,4949	
       0,4951	
       0,4952	
  

  2,6	
        0,4953	
      0,4955	
     0,4956	
      0,4957	
       0,4959	
       0,4960	
       0,4961	
       0,4962	
       0,4963	
       0,4964	
  

  2,7	
   3º)0,4965	
   0,4966	
   0,4967	
   0,4968	
   0,4969	
   0,4970	
   0,4971	
  
                Transforme a variável aleatória Normal para Normal padronizada através
                                                                                          0,4972	
   0,4973	
   0,4974	
  

  2,8	
   da0,4974	
  
              fórmula:
                       0,4975	
           0,4976	
      0,4977	
       0,4977	
       0,4978	
       0,4979	
       0,4979	
       0,4980	
       0,4981	
  

               0,4981	
      0,4982	
     0,4982	
      0,4983	
       0,4984	
       0,4984	
       0,4985	
       0,4985	
       0,4986	
       0,4986	
  
  2,9	
  
                        X-μ
  3	
        Z=
              0,4987	
       0,4987	
     0,4987	
      0,4988	
       0,4988	
       0,4989	
       0,4989	
       0,4989	
       0,4990	
       0,4990	
  

               0,4990	
  
                          σ 0,4991	
      0,4991	
      0,4991	
       0,4992	
       0,4992	
       0,4992	
       0,4992	
       0,4993	
       0,4993	
  
  3,1	
  
               0,4993	
      0,4993	
     0,4994	
      0,4994	
       0,4994	
       0,4994	
       0,4994	
       0,4995	
       0,4995	
       0,4995	
  
  3,2	
  
            4º)0,4995	
  
                 Esboce0,4995	
  
                            o gráfico    das curvas
                                     0,4995	
   0,4996	
   e destaque
                                                             0,4996	
     a região0,4996	
  
                                                                        0,4996	
    cuja área    será investigada
                                                                                             0,4996	
   0,4996	
        para
                                                                                                                   0,4997	
  
  3,3	
  
            determinar
              0,4997	
      a probabilidade
                          0,4997	
   0,4997	
    pedida.0,4997	
   0,4997	
   0,4997	
   0,4997	
   0,4997	
   0,4998	
  
                                                0,4997	
  
  3,4	
  
  3,5	
        0,4998	
      0,4998	
     0,4998	
      0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
  

               0,4998	
      0,4998	
     0,4999	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
  3,6	
  
               0,4999	
      0,4999	
     0,4999	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
  3,7	
  
                                                                             201
               0,4999	
      0,4999	
     0,4999	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
  3,8	
  
               0,5000	
      0,5000	
     0,5000	
      0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  
  3,9

## Página 202

         Importante


        Este passo é o mais importante, pois com o desenho não precisamos ficar de-
        corando se devemos somar 0,5, diminuir 0,5, ou não fazer nada. Apenas com-
        paramos o desenho da área obtida na tabela e o desenho do que queremos
        determinar para concluirmos o que temos que fazer.




    5º) Utilize a tabela da distribuição Normal padronizada e busque o valor da área.


    6º) Finalize com o valor da área sob a curva e o desenho do item 4, faça os ajustes
    necessários e determine a probabilidade solicitada.


Vejamos alguns exemplos com os passos apresentados. É importante observar que
cada exemplo mostra uma região da curva diferente. E, em cada região, os cálculos são
diferentes. Por isso, há um número grande de exemplos apresentados.


Essa parte é importante, pois a maioria dos problemas de probabilidades poderão ser
resolvidos ou aproximados pela distribuição Normal.


Exemplo 1


Exemplifica o cálculo da probabilidade na qual a variável aleatória Normal padrão (z-score)
é menor ou igual a um determinado valor negativo.


Enunciado: Em agosto de 2020, um brasileiro passou em média 77 horas conectado à
internet navegando em redes sociais. Suponha que os tempos estejam normalmente
distribuídos e que o desvio-padrão seja de 20 horas.


Pergunta-se:
Qual é a probabilidade de um brasileiro, escolhido aleatoriamente, ter passado menos de
50 horas conectado à internet em agosto de 2020?


Resolução:


    1º) Determine os valores da média μ e do desvio-padrão σ e nomeie a distribuição
    Normal: neste caso  X~N(77,202).




                                            202

## Página 203

        2º) Enuncie a probabilidade pedida: P(X ≤ 50) = ?


        3º) Transforme a variável aleatória Normal para Normal padronizada a partir
        da fórmula:
                   X-μ                   50 - 77
        Z=                      =                       = - 1,35
                        σ                  20


        4º) Desenhe as curvas e destaque a região cuja área será investigada para determinar
        a probabilidade pedida.

                                                    σ=20                                                                                        σ=1




                   50         μ=77                                                                          -1,35              0
                   Distribuição Normal                                                           Distribuição Normal Padronizada

                                                           P(X ≤ 50) = P(Z ≤ -1,35)

                                                  Fonte: Elaborada pela autora (2020).


        5º) Utilize a tabela da distribuição Normal padronizada e busque o valor da área.
        Para Z=1,35  A=0,4115 Tabela da Distribuição Normal

      DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
           0	
              0,0040	
       0,0080	
      0,0120	
     0,0160	
      0,0199	
       0,0239	
       0,0279	
         0,0319	
     0,0359	
  
0	
  
           0,0398	
         0,0438	
       0,0478	
      0,0517	
     0,0557	
      0,0596	
       0,0636	
       0,0675	
         0,0714	
     0,0753	
  
0,1	
  
0,2	
      0,0793	
         0,0832	
       0,0871	
      0,0910	
     0,0948	
      0,0987	
       0,1026	
       0,1064	
         0,1103	
     0,1141	
  

           0,1179	
         0,1217	
       0,1255	
      0,1293	
     0,1331	
      0,1368	
       0,1406	
       0,1443	
         0,1480	
     0,1517	
  
0,3	
  
           0,1554	
         0,1591	
       0,1664	
      0,1664	
     0,1700	
      0,1736	
       0,1772	
       0,1808	
         0,1844	
     0,1879	
  
0,4	
  
           0,1915	
         0,1950	
       0,1985	
      0,2019	
     0,2054	
      0,2088	
       0,2123	
       0,2157	
         0,2190	
     0,2224	
  
0,5	
  
           0,2257	
         0,2291	
       0,2324	
      0,2357	
     0,2389	
      0,2422	
       0,2454	
       0,2486	
         0,2517	
     0,2549	
  
0,6	
  
           0,2580	
         0,2611	
       0,2642	
      0,2673	
     0,2704	
      0,2734	
       0,2764	
       0,2794	
         0,2823	
     0,2852	
  
0,7	
  
           0,2881	
         0,2910	
       0,2939	
      0,2967	
     0,2995	
      0,3023	
       0,3051	
       0,3078	
         0,3106	
     0,3133	
  
0,8	
  
           0,3159	
         0,3186	
       0,3212	
      0,3238	
     0,3264	
      0,3289	
       0,3315	
       0,3340	
         0,3365	
     0,3389	
  
0,9	
  
           0,3413	
         0,3438	
       0,3461	
      0,3485	
     0,3508	
      0,3531	
       0,3554	
       0,3577	
         0,3599	
     0,3621	
  
1	
  
1,1	
      0,3643	
         0,3665	
       0,3686	
      0,3708	
     0,3729	
      0,3749	
       0,3770	
       0,3790	
         0,3810	
     0,3830	
  

           0,3849	
         0,3869	
       0,3888	
      0,3907	
     0,3925	
      0,3944	
       0,3962	
       0,3980	
         0,3997	
     0,4015	
  
1,2	
  
           0,4032	
         0,4049	
       0,4066	
      0,4082	
     0,4099	
      0,4115	
  
                                                                                    0,4115         0,4131	
       0,4147	
         0,4162	
     0,4177	
  
1,3	
  
           0,4192	
         0,4207	
       0,4222	
      0,4236	
     0,4251	
      0,4265	
       0,4279	
       0,4292	
         0,4306	
     0,4319	
  
1,4	
  
           0,4332	
         0,4345	
       0,4357	
      0,4370	
     0,4382	
      0,4394	
       0,4406	
       0,4418	
         0,4429	
     0,4441	
  
1,5	
  
1,6	
      0,4452	
         0,4463	
       0,4474	
      0,4484	
         203
                                                                      0,4495	
      0,4505	
       0,4515	
       0,4525	
         0,4535	
     0,4545	
  

           0,4554	
         0,4564	
       0,4573	
      0,4582	
     0,4591	
      0,4599	
       0,4608	
       0,4616	
         0,4625	
     0,4633	
  
1,7

## Página 204

                                                       σ=1
                       Área tabelada


       Área a ser determinada



                                   -1,35    0
                             Distribuição Normal Padronizada


                           Fonte: Elaborada pela autora (2020).


    6º) Faça alguns ajustes para chegar à reposta correta, porque a área a ser determi-
    nada é diferente da área tabelada. Como vimos, a área total sob a curva Normal é
    igual a 1 e, pela simetria da curva, 0,5 antes da média e 0,5 depois da média.


    Verificamos que:


    Área a ser determinada + área tabelada =0,5
    Área a ser determinada = 0,5 – área tabelada
    Área a ser determinada = 0,5 – 0,4115 = 0,0885


    Logo, a probabilidade pedida será igual à área a ser determinada:


    P(X ≤ 50) = P(Z ≤ -1,35)= 0,0885 = 8,85%


    Podemos concluir que a probabilidade de que uma pessoa selecionada ao aca-
    so tenha ficado menos de 50 horas nas redes sociais no mês de agosto de 2020
    é igual a 8,85%.


Exemplo 2


Exemplifica a probabilidade na qual z-score é menor ou igual a um determinado
valor positivo.


Enunciado: Suponhamos que o tempo médio que os alunos da Universidade Veiga de
Almeida levam para sair de casa e chegar à universidade é igual a 58 minutos. Considere
que a distribuição do tempo gasto nesse percurso seja normal, com desvio-padrão igual
a 12 minutos. Determine a probabilidade de um aluno selecionado aleatoriamente ter
demorado menos de 76 minutos para chegar à faculdade.




                                           204

## Página 205

Resolução:


    1º) Determine os valores da média μ e do desvio-padrão σ e nomeie a distribuição
    Normal: X~N(58;122)


    2º) Enuncie a probabilidade pedida: P(X ≤ 76) = ?


    3º) Transforme a variável aleatória Normal para Normal padronizada a partir
    da fórmula:


             X-μ       76 - 58
     Z=            =              = 1,5
              σ           12


    4º) Desenhe as curvas e destaque a região cuja área será investigada para determi-
    nar a probabilidade pedida.

                               σ=12                                               σ=1




                   μ=58          76                                     0          1,5
          Distribuição Normal                             Distribuição Normal Padronizada

                                      P(X ≤ 76) = P(Z ≤ 1,5)


                                 Fonte: Elaborada pela autora (2020).


    5º) Utilize a tabela da distribuição Normal padronizada e busque o valor da área.
    Para Z=1,50 temos A=0,4332




                                               205

## Página 206

                                          Tabela da Distribuição Normal

      DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
            0	
          0,0040	
      0,0080	
       0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
0	
  
            0,0398	
     0,0438	
      0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
0,1	
  
0,2	
       0,0793	
     0,0832	
      0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

            0,1179	
     0,1217	
      0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
0,3	
  
            0,1554	
     0,1591	
      0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
0,4	
  
            0,1915	
     0,1950	
      0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
0,5	
  
            0,2257	
     0,2291	
      0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
0,6	
  
            0,2580	
     0,2611	
      0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
0,7	
  
            0,2881	
     0,2910	
      0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
0,8	
  
            0,3159	
     0,3186	
      0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
0,9	
  
            0,3413	
     0,3438	
      0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
1	
  
1,1	
       0,3643	
     0,3665	
      0,3686	
       0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

            0,3849	
     0,3869	
      0,3888	
       0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
1,2	
  
            0,4032	
     0,4049	
      0,4066	
       0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
1,3	
  
            0,4192	
     0,4207	
      0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
1,4	
  
            0,4332	
     0,4345	
      0,4357	
       0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
       0,4429	
       0,4441	
  
1,5	
  
1,6	
       0,4452	
     0,4463	
      0,4474	
       0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
       0,4535	
       0,4545	
  

            0,4554	
     0,4564	
      0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
1,7	
  
            0,4641	
     0,4649	
      0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
1,8	
  
1,9	
       0,4713	
     0,4719	
      0,4726	
       0,4732	
       0,4738	
       0,4744	
           Área a0,4756	
  
                                                                                                   0,4750	
    ser determinada
                                                                                                                        0,4761	
   0,4767	
  

            0,4772	
     0,4778	
      0,4783	
   0,4788	
   0,4793	
               0,4798	
       0,4803	
       0,4808	
       0,4812	
      0,4817	
  
2	
                                       Área tabelada                                                                                       σ=1
2,1	
       0,4821	
     0,4826	
           A = 0,4332
                                       0,4830	
   0,4834	
   0,4838	
               0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

            0,4861	
     0,4864	
      0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
2,2	
  
            0,4893	
     0,4896	
      0,4898	
       0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
   0,4332
                                                                                                                  0,5          0,4913	
         0,4916	
  
2,3	
  
            0,4918	
     0,4920	
      0,4922	
       0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
       0,4934	
       0,4936	
  
2,4	
  
            0,4938	
     0,4940	
      0,4941	
       0,4943	
       0,4945	
       0,4946	
       0,4948	
       0,4949	
       0,4951	
       0,4952	
  
2,5	
  
                            μ=0                  1,5 0,4957	
                                                          μ=0 0,4963	
            1,5
2,6	
   0,4953	
         0,4955	
      0,4956	
                      0,4959	
       0,4960	
       0,4961	
       0,4962	
                      0,4964	
  
 Distribuição 0,4966	
  
                   Normal 0,4967	
  
                         Padronizada                                                             Distribuição    Normal0,4973	
  
                                                                                                                         Padronizada
2,7	
   0,4965	
                     0,4968	
                        0,4969	
       0,4970	
       0,4971	
   0,4972	
            0,4974	
  

            0,4974	
     0,4975	
      0,4976	
       0,4977	
       0,4977	
       0,4978	
       0,4979	
       0,4979	
       0,4980	
       0,4981	
  
2,8	
  
            0,4981	
     0,4982	
      0,4982	
       0,4983	
       0,4984	
       0,4984	
       0,4985	
       0,4985	
       0,4986	
       0,4986	
  
2,9	
                                               Fonte: Elaborada pela autora (2020).
            0,4987	
     0,4987	
      0,4987	
       0,4988	
       0,4988	
       0,4989	
       0,4989	
       0,4989	
       0,4990	
       0,4990	
  
3	
  
        0,4990	
   0,4991	
   0,4991	
   0,4991	
   0,4992	
   0,4992	
   0,4992	
   0,4992	
   0,4993	
   0,4993	
  
3,1	
  
  6º) Como a área a ser determinada é diferente da área tabelada precisamos fazer
            0,4993	
     0,4993	
      0,4994	
       0,4994	
       0,4994	
       0,4994	
       0,4994	
       0,4995	
       0,4995	
       0,4995	
  
3,2	
  
        alguns ajustes para chegar à reposta correta.
            0,4995	
     0,4995	
      0,4995	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4997	
  
3,3	
  
            0,4997	
     0,4997	
      0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4998	
  
3,4	
  
        Verificamos
           0,4998	
     que:
                      0,4998	
         0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
  
3,5	
  
            0,4998	
     0,4998	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
3,6	
  
  Área
3,7	
      a ser 0,4999	
  
        0,4999	
   determinada
                            0,4999	
   = 0,5  + área0,4999	
  
                                          0,4999	
    tabelada 0,4999	
                            0,4999	
       0,4999	
       0,4999	
       0,4999	
  

  Área
3,8	
      a ser 0,4999	
  
        0,4999	
   determinada
                            0,4999	
   = 0,5  +0,4332
                                          0,4999	
   0,4999	
                       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  

  Área
3,9	
      a ser determinada
        0,5000	
   0,5000	
   0,5000	
   = 0,9332
                                            0,5000	
                 0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  

4	
         0,5000	
     0,5000	
      0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  



                                                                           206

## Página 207

    Logo, a probabilidade pedida será igual à área a ser determinada:


    P( X ≤ 76) = P(Z ≤ 1,5) = 0,9332 = 93,32%


    Podemos concluir que a probabilidade de um aluno levar menos de 76 minutos para
    chegar na universidade é igual a 93,32%


Exemplo 3


Exemplifica o cálculo da probabilidade em que a z-score é maior ou igual a um determi-
nado valor negativo.


Enunciado: Os “pesos” (massa corporal) dos frequentadores de uma academia de ginás-
tica são normalmente distribuídos, com média de 65,3kg e desvio-padrão 5,5.


Um frequentador é selecionado ao acaso. Determine a probabilidade de seu peso ser
maior do que 63,2 kg.


Resolução:


    1º) Determine os valores da média μ e do desvio-padrão σ e nomeie a distribuição
    Normal: X~N(65,3;5,52)


    2º) Enuncie a probabilidade pedida: P(X ≥ 63,2) = ?


    3º) Transforme a variável aleatória Normal para Normal padronizada a partir da fórmula:
             X-μ       63,2 - 65,3
     Z=            =                 = - 0,38
              σ           5,5


    4º Desenhe as curvas e destaque a região cuja área será investigada para determi-
    nar a probabilidade pedida.




                                                207

## Página 208

                                                        σ=5,5                                                                                      σ=1




                            63,2 μ=65,3                                                                             - 0,38 μ=0
                       Distribuição Normal                                                           Distribuição Normal Padronizada

                                                          P( X ≥ 63,2) = P(Z ≥ -0,38)

                                                  Fonte: Elaborada pela autora (2020).


            5º) Utilize a tabela da distribuição Normal padronizada e busque o valor da área.


            Para Z = -0,38 usaremos z = 0,38, pois as áreas são idênticas para os dois valores.
            A = 0,1480
                                Tabela da Distribuição Normal

          DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
    z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
               0	
            0,0040	
     0,0080	
       0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
        0,0319	
      0,0359	
  
    0	
  
               0,0398	
       0,0438	
     0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
        0,0714	
      0,0753	
  
    0,1	
  
    0,2	
      0,0793	
       0,0832	
     0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
        0,1103	
      0,1141	
  

               0,1179	
       0,1217	
     0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
        0,1480	
  
                                                                                                                                      0,1480        0,1517	
  
    0,3	
  
               0,1554	
       0,1591	
     0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
        0,1844	
      0,1879	
  
    0,4	
  
   0,5	
   0,1915	
           0,1950	
     0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
        0,2190	
      0,2224	
  

Área tabelada
   0,6	
   0,2257	
           0,2291	
       σ=1
                                           0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
           σ=1
                                                                                                                                      0,2517	
      0,2549	
  

A =0,7	
  
    0,1480 0,2580	
           0,2611	
     0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
        0,2823	
     Área
                                                                                                                                                   0,2852	
   a ser

               0,2881	
       0,2910	
     0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
        0,3106	
   determinada
                                                                                                                                                   0,3133	
  
    0,8	
  
               0,3159	
       0,3186	
     0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
        0,3365	
      0,3389	
  
    0,9	
  
               0,3413	
       0,3438	
     0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
  0,1480
                                                                                                                     0,3577	
      0,5
                                                                                                                                     0,3599	
       0,3621	
  
    1	
  
    1,1	
      0,3643	
       0,3665	
     0,3686	
       0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
        0,3810	
      0,3830	
  
                        - 0,38 μ=0                                                                              - 0,38 μ=0
    1,2	
   0,3849	
   0,3869	
   0,3888	
   0,3907	
                    0,3925	
       0,3944	
       0,3962	
       0,3980	
        0,3997	
      0,4015	
  
      Distribuição     Normal     Padronizada                                                 Distribuição    Normal Padronizada
    1,3	
   0,4032	
   0,4049	
   0,4066	
   0,4082	
                    0,4099	
       0,4115	
   0,4131	
   0,4147	
   0,4162	
   0,4177	
  

               0,4192	
       0,4207	
     0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
        0,4306	
      0,4319	
  
    1,4	
  
               0,4332	
       0,4345	
     0,4357	
       0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
        0,4429	
      0,4441	
  
    1,5	
  
    1,6	
      0,4452	
       0,4463	
     0,4474	
       0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
        0,4535	
      0,4545	
  
                                                  Fonte: Elaborada pela autora (2020).
               0,4554	
       0,4564	
     0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
        0,4625	
      0,4633	
  
    1,7	
  
               0,4641	
       0,4649	
     0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
        0,4699	
      0,4706	
  
    1,8	
  
      6º) Faça alguns             ajustes       para chegar     à reposta     correta,     porque   a área    a ser0,4767	
  
                                                                                                                     determi-
    1,9	
   0,4713	
   0,4719	
      0,4726	
     0,4732	
   0,4738	
   0,4744	
   0,4750	
   0,4756	
   0,4761	
  
      nada      é diferente
            0,4772	
   0,4778	
  
                                  da0,4783	
  
                                      área tabelada.
                                                  0,4788	
   0,4793	
   0,4798	
   0,4803	
   0,4808	
   0,4812	
   0,4817	
  
    2	
  
    2,1	
      0,4821	
       0,4826	
     0,4830	
       0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
        0,4854	
      0,4857	
  

               0,4861	
       0,4864	
     0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
        0,4887	
      0,4890	
  
    2,2	
  
               0,4893	
       0,4896	
     0,4898	
       0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
        0,4913	
      0,4916	
  
    2,3	
  
               0,4918	
       0,4920	
     0,4922	
       0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
        0,4934	
      0,4936	
  
    2,4	
                                                                    208
               0,4938	
       0,4940	
     0,4941	
       0,4943	
       0,4945	
       0,4946	
       0,4948	
       0,4949	
        0,4951	
      0,4952	
  
    2,5

## Página 209

    Verificamos que:


    Área a ser determinada = 0,5 + área tabelada
    Área a ser determinada = 0,5 +0,1480
    Área a ser determinada = 0,6480


    Logo, a probabilidade pedida será igual à área a ser determinada:


    P( X ≥ 63,2) = P(Z ≥-0,38) = 0,6480 = 64,8%


    Podemos concluir que a probabilidade que um frequentador da academia seleciona-
    do ao acaso tenha seu peso superior a 63,2kg é igual a 64,8%.


Exemplo 4


Exemplifica o cálculo da probabilidade na qual z-score é maior ou igual a um determi-
nado valor positivo.


Enunciado: Em 2019 o valor médio de gastos das famílias brasileiras é de R$ 5.700,00
por mês. Suponha que os gastos mensais tenham uma distribuição normal e que o des-
vio-padrão seja igual a R$ 1.500,00. Uma família é selecionada ao acaso. Determine a
probabilidade dessa família ter um gasto mensal superior a R$ 7.000,00.


Resolução:


    1º) Determine os valores da média μ e do desvio-padrão σ e nomeie a distribuição
    Normal: X~N(5700;15002)


    2º) Enuncie a probabilidade pedida: P(X ≥ 7000) = ?


    3º) Transforme a variável aleatória Normal para Normal padronizada a partir da fórmula:
             X-μ       7000 - 5700
     Z=            =                 = 0,87
              σ           1500


    4º) Desenhe as curvas e destaque a região cuja área será investigada para determi-
    nar a probabilidade pedida.




                                              209

## Página 210

                                               σ=1500                                                                                         σ=1




                        μ=5700 7000                                                                                       μ=0         0,87
                   Distribuição Normal                                                           Distribuição Normal Padronizada

                                                     P( X ≥ 7000) = P(Z ≥ 0,87)




                                              Fonte: Elaborada pela autora (2020).


        5º) Utilize a tabela da distribuição Normal padronizada e busque o valor da área.

                                          Tabela da Distribuição Normal
        Para Z=0,87  A=0,3078


      DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
           0	
          0,0040	
       0,0080	
       0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
0	
  
           0,0398	
     0,0438	
       0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
0,1	
  
0,2	
      0,0793	
     0,0832	
       0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

           0,1179	
     0,1217	
       0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
0,3	
  
           0,1554	
     0,1591	
       0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
0,4	
  
           0,1915	
     0,1950	
       0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
0,5	
  
           0,2257	
     0,2291	
       0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
0,6	
  
           0,2580	
     0,2611	
       0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
0,7	
  
           0,2881	
     0,2910	
       0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
0,8	
  
           0,3159	
     0,3186	
       0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
0,9	
  
           0,3413	
     0,3438	
       0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
1	
  
1,1	
      0,3643	
     0,3665	
       0,3686	
       0,3708	
           Área 0,3749	
  
                                                                     0,3729	
   tabelada =  0,30780,3790	
  
                                                                                         0,3770	
                                0,3810	
       0,3830	
  

           0,3849	
     0,3869	
       0,3888	
       0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
1,2	
  
                                                                                               Área a ser
           0,4032	
     0,4049	
       0,4066	
       0,4082	
       0,4099	
       0,4115	
    0,4131	
   0,4147	
              0,4162	
       0,4177	
  
1,3	
                                                                                     determinada
           0,4192	
     0,4207	
       0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
1,4	
  
           0,4332	
     0,4345	
       0,4357	
       0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
       0,4429	
       0,4441	
  
1,5	
  
1,6	
      0,4452	
     0,4463	
       0,4474	
       0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
       0,4535	
       0,4545	
  

           0,4554	
     0,4564	
       0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
1,7	
  
                                                                          μ=0         0,87
           0,4641	
     0,4649	
       0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
1,8	
  
           0,4713	
     0,4719	
       0,4726	
   Distribuição
                                                     0,4732	
    Normal 0,4744	
  
                                                                0,4738	
   Padronizada
                                                                                   0,4750	
                       0,4756	
       0,4761	
       0,4767	
  
1,9	
  
           0,4772	
     0,4778	
       0,4783	
       0,4788	
       0,4793	
       0,4798	
       0,4803	
       0,4808	
       0,4812	
       0,4817	
  
2	
                                           Fonte: Elaborada pela autora (2020).
2,1	
      0,4821	
     0,4826	
       0,4830	
       0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

           0,4861	
     0,4864	
       0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
2,2	
  
2,3	
      0,4893	
     0,4896	
       0,4898	
       0,4901	
           210
                                                                     0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916	
  

           0,4918	
     0,4920	
       0,4922	
       0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
       0,4934	
       0,4936	
  
2,4

## Página 211

    6º) Como a área a ser determinada é diferente da área tabelada precisamos fase
    alguns ajustes para chegar à reposta correta.


    Verificamos que:


    Área a ser determinada+ área tabelada = 0,5
    Area a ser determinada = 0,5 – área tabelada
    Área a ser determinada = 0,5 – 0,3078 = 0,1922


    Logo, a probabilidade pedida será igual a área a ser determinada:
    P( X ≥ 7000) = P(Z ≥ 0,87) = 0,1922= 19,22%


    Podemos concluir que a probabilidade que uma família selecionada ao acaso tenha
    gasto mensal superior a R$ 7.000,00 é igual a 19,22%.


Exemplo 5


Exemplifica o cálculo da probabilidade em que a z-score está entre dois valores determi-
nados antes da média.


Enunciado: A quantia média anual que os moradores da cidade do Rio de Janeiro gastam
por ano em aplicativos de transportes individuais R$ 1.649,00. Suponha que a quantia
gasta tenha uma distribuição normal com desvio-padrão igual a R$ 843,00.


Determine a probabilidade de uma pessoa selecionada ao acaso gastar entre R$ 1.042,00
e R$ 1.568,00


Resolução:


    1º) Determine os valores da média μ e do desvio-padrão σ e nomeie a distribuição
    Normal: X~N(1649;8432)


    2º) Enuncie a probabilidade pedida: P(1042 < X <1 568) = ?


    3º) Transforme a variável aleatória Normal para Normal padronizada a partir da fórmula:


                              X-μ        1042 - 1649
     Para X = 1042  Z1 =            =                 = - 0,72
                                σ            843




                                           211

## Página 212

                                                    X-μ                  1568 - 1649
        Para X=1568                   Z2 =                        =                               = - 0,10
                                                       σ                            843


        4º) Desenhe as curvas e destaque a região cuja área será investigada para determi-
        nar a probabilidade pedida.


                                               σ=843                                                                                          σ=1




                   1042 1568                                                                                     - 0,72 - 0,10
                        μ=1649                                                                                            μ=0


                   Distribuição Normal                                                             Distribuição Normal Padronizada

                                         P(1042 < X < 1568) = P(-0,72 < Z <-0,10)


                                              Fonte: Elaborada pela autora (2020).


        5º) Utilize a tabela da distribuição Normal padronizada e busque o valor das áreas.


        Primeiramente para Z=-0,72 ou Z=0,72 já que as duas áreas são idênticas para esses
        valores  A = 0,2642Tabela da Distribuição Normal


      DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
           0	
          0,0040	
       0,0080	
       0,0120	
         0,0160	
       0,0199	
      0,0239	
      0,0279	
       0,0319	
       0,0359	
  
0	
  
           0,0398	
     0,0438	
       0,0478	
       0,0517	
         0,0557	
       0,0596	
      0,0636	
      0,0675	
       0,0714	
       0,0753	
  
0,1	
  
0,2	
      0,0793	
     0,0832	
       0,0871	
       0,0910	
         0,0948	
       0,0987	
      0,1026	
      0,1064	
       0,1103	
       0,1141	
  

           0,1179	
     0,1217	
       0,1255	
       0,1293	
         0,1331	
       0,1368	
      0,1406	
      0,1443	
       0,1480	
       0,1517	
  
0,3	
  
           0,1554	
     0,1591	
       0,1664	
       0,1664	
         0,1700	
       0,1736	
      0,1772	
      0,1808	
       0,1844	
       0,1879	
  
0,4	
  
           0,1915	
     0,1950	
       0,1985	
       0,2019	
         0,2054	
       0,2088	
      0,2123	
      0,2157	
       0,2190	
       0,2224	
  
0,5	
  
           0,2257	
     0,2291	
       0,2324	
       0,2357	
         0,2389	
       0,2422	
      0,2454	
      0,2486	
       0,2517	
       0,2549	
  
0,6	
  
           0,2580	
     0,2611	
       0,2642	
       0,2673	
         0,2704	
       0,2734	
      0,2764	
      0,2794	
       0,2823	
       0,2852	
  
0,7	
                                  0,2642

           0,2881	
     0,2910	
       0,2939	
       0,2967	
         0,2995	
       0,3023	
      0,3051	
      0,3078	
       0,3106	
       0,3133	
  
0,8	
  
           0,3159	
     0,3186	
       0,3212	
       0,3238	
         0,3264	
       0,3289	
      0,3315	
      0,3340	
       0,3365	
       0,3389	
  
0,9	
  
           0,3413	
     0,3438	
       0,3461	
       0,3485	
         0,3508	
       0,3531	
      0,3554	
      0,3577	
       0,3599	
       0,3621	
  
1	
  
1,1	
      0,3643	
     0,3665	
       0,3686	
       0,3708	
         0,3729	
       0,3749	
      0,3770	
      0,3790	
       0,3810	
       0,3830	
  

           0,3849	
     0,3869	
       0,3888	
       0,3907	
         0,3925	
       0,3944	
      0,3962	
      0,3980	
       0,3997	
       0,4015	
  
1,2	
  
           0,4032	
     0,4049	
       0,4066	
       0,4082	
         0,4099	
       0,4115	
      0,4131	
      0,4147	
       0,4162	
       0,4177	
  
1,3	
                                                                        212
           0,4192	
     0,4207	
       0,4222	
       0,4236	
         0,4251	
       0,4265	
      0,4279	
      0,4292	
       0,4306	
       0,4319	
  
1,4	
  
           0,4332	
     0,4345	
       0,4357	
       0,4370	
         0,4382	
       0,4394	
      0,4406	
      0,4418	
       0,4429	
       0,4441

## Página 213

            Agora para Z = -0,10 ou Z = 0,10 já que as duas áreas são idênticas para esses valo-
                                 Tabela da Distribuição Normal
            res A = 0,0398

            DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
    z	
         0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
                  0	
            0,0040	
       0,0080	
      0,0120	
       0,0160	
       0,0199	
      0,0239	
       0,0279	
       0,0319	
      0,0359	
  
    0	
  
                  0,0398	
       0,0438	
       0,0478	
      0,0517	
       0,0557	
       0,0596	
      0,0636	
       0,0675	
       0,0714	
      0,0753	
  
    0,1	
         0,0398

    0,2	
         0,0793	
       0,0832	
       0,0871	
      0,0910	
       0,0948	
       0,0987	
      0,1026	
       0,1064	
       0,1103	
      0,1141	
  

                  0,1179	
       0,1217	
       0,1255	
      0,1293	
       0,1331	
       0,1368	
      0,1406	
       0,1443	
       0,1480	
      0,1517	
  
    0,3	
  
Área tabelada                                                  Área tabelada                              0,1772	
  Área a  ser
   0,4	
   0,1554	
              0,1591	
       0,1664	
      0,1664	
   0,1700	
           0,1736	
                    0,1808	
        0,1844	
      0,1879	
  
para Z = 0,72                                                  para Z = 0,10                                       determinada
           0,1915	
              0,1950	
       0,1985	
      0,2019	
   0,2054	
           0,2088	
      0,2123	
       0,2157	
       0,2190	
      0,2224	
  
A =0,5	
  
   0,2642                                                      A = 0,0398
                  0,2257	
       0,2291	
       0,2324	
      0,2357	
       0,2389	
       0,2422	
      0,2454	
       0,2486	
       0,2517	
      0,2549	
  
    0,6	
                                 σ=1                                                  σ=1                                                    σ=1
                  0,2580	
       0,2611	
       0,2642	
      0,2673	
       0,2704	
       0,2734	
      0,2764	
       0,2794	
       0,2823	
      0,2852	
  
    0,7	
  
                  0,2881	
       0,2910	
       0,2939	
      0,2967	
       0,2995	
       0,3023	
      0,3051	
       0,3078	
       0,3106	
      0,3133	
  
    0,8	
  
                  0,3159	
       0,3186	
       0,3212	
      0,3238	
       0,3264	
       0,3289	
      0,3315	
       0,3340	
       0,3365	
      0,3389	
  
    0,9	
  
            0,3413	
       0,3438	
             0,3461	
      0,3485	
       0,3508	
       0,3531	
      0,3554	
       0,3577	
       0,3599	
      0,3621	
  
    1	
  
       - 0,72          - 0,10                                     - 0,72            - 0,10                                - 0,72            - 0,10
    1,1	
   0,3643	
       0,3665	
             0,3686	
      0,3708	
       0,3729	
       0,3749	
      0,3770	
       0,3790	
       0,3810	
      0,3830	
  
                   μ=0                                                           μ=0                                                      μ=0
    1,2	
   0,3849	
       0,3869	
             0,3888	
      0,3907	
       0,3925	
       0,3944	
      0,3962	
       0,3980	
       0,3997	
      0,4015	
  

              0,4032	
   0,4049	
   0,4066	
                  0,4082	
       0,4099	
       0,4115	
      0,4131	
       0,4147	
       0,4162	
      0,4177	
  
    1,3	
  Distribuição   Normal                                    Distribuição Normal                                     Distribuição Normal
    1,4	
          Padronizada
                  0,4192	
   0,4207	
           0,4222	
      0,4236	
   Padronizada
                                                                          0,4251	
   0,4265	
             0,4279	
       0,4292	
   Padronizada
                                                                                                                                      0,4306	
   0,4319	
  

                  0,4332	
       0,4345	
       0,4357	
      0,4370	
       0,4382	
       0,4394	
      0,4406	
       0,4418	
       0,4429	
      0,4441	
  
    1,5	
  
                                                       Fonte: Elaborada pela autora (2020).
    1,6	
         0,4452	
       0,4463	
       0,4474	
      0,4484	
       0,4495	
       0,4505	
      0,4515	
       0,4525	
       0,4535	
      0,4545	
  

                  0,4554	
       0,4564	
       0,4573	
      0,4582	
       0,4591	
       0,4599	
      0,4608	
       0,4616	
       0,4625	
      0,4633	
  
    1,7	
  
      6º) 0,4641	
  
            Observe o         colorido
                         0,4649	
   0,4656	
  a área   A1. Se
                                                  0,4664	
      apagarmos
                                                             0,4671	
     0,4678	
   em  A1 a área
                                                                                      0,4686	
   0,4693	
  A2, 0,4699	
  
                                                                                                                vamos ficar
                                                                                                                          0,4706	
   exa-
    1,8	
  
      tamente
    1,9	
   0,4713	
   com   a área
                         0,4719	
      a ser determinada.
                                    0,4726	
      0,4732	
   0,4738	
   Portanto:
                                                                          0,4744	
    0,4750	
   0,4756	
      0,4761	
   0,4767	
  

                  0,4772	
       0,4778	
       0,4783	
      0,4788	
       0,4793	
       0,4798	
      0,4803	
       0,4808	
       0,4812	
      0,4817	
  
    2	
  
      Área
    2,1	
      a ser 0,4826	
  
            0,4821	
   determinada=
                                0,4830	
   área   tabelada
                                             0,4834	
   0,4838	
  1 - área
                                                                      0,4842	
   tabelada
                                                                                    0,4846	
   2 0,4850	
                               0,4854	
      0,4857	
  

      Area
    2,2	
      a ser determinada
            0,4861	
   0,4864	
   0,4868	
   = 02642
                                                0,4871	
   – 0,4875	
  
                                                             0,0398 0,4878	
                              0,4881	
       0,4884	
       0,4887	
      0,4890	
  

      Área
    2,3	
      a ser determinada
            0,4893	
   0,4896	
   0,4898	
  
                                             = 0,2244
                                                0,4901	
                     0,4904	
       0,4906	
      0,4909	
       0,4911	
       0,4913	
      0,4916	
  

                  0,4918	
       0,4920	
       0,4922	
      0,4925	
       0,4927	
       0,4929	
      0,4931	
       0,4932	
       0,4934	
      0,4936	
  
    2,4	
  
            0,4938	
   0,4940	
   0,4941	
   0,4943	
   0,4945	
   0,4946	
   0,4948	
   0,4949	
   0,4951	
                                          0,4952	
  
    2,5	
  
      Logo, a probabilidade pedida será igual a área a ser determinada:
    2,6	
         0,4953	
       0,4955	
       0,4956	
      0,4957	
       0,4959	
       0,4960	
      0,4961	
       0,4962	
       0,4963	
      0,4964	
  

                  0,4965	
       0,4966	
       0,4967	
      0,4968	
       0,4969	
       0,4970	
      0,4971	
       0,4972	
       0,4973	
      0,4974	
  
    2,7	
  
            P(1042 < X < 1568) = P(-0,72 < Z<-0,10) = 0,2244 = 22,44%
                  0,4974	
       0,4975	
       0,4976	
      0,4977	
       0,4977	
       0,4978	
      0,4979	
       0,4979	
       0,4980	
      0,4981	
  
    2,8	
  
                  0,4981	
       0,4982	
       0,4982	
      0,4983	
       0,4984	
       0,4984	
      0,4985	
       0,4985	
       0,4986	
      0,4986	
  
    2,9	
  
          Então,  a probabilidade
             0,4987	
   0,4987	
  
                                         de uma
                                    0,4987	
  
                                                    pessoa
                                               0,4988	
  
                                                                 selecionada
                                                             0,4988	
   0,4989	
  
                                                                                   ao  acaso0,4989	
  
                                                                                   0,4989	
  
                                                                                                gastar entre
                                                                                                         0,4990	
  
                                                                                                                    R$0,4990	
  
                                                                                                                       1.042,00
    3	
  
          e R$  1.568,00
             0,4990	
        em 0,4991	
  
                        0,4991	
   aplicativos     de 0,4992	
  
                                               0,4991	
   transporte é      igual a0,4992	
  
                                                                        0,4992	
      22,44%  0,4992	
   0,4993	
     0,4993	
  
    3,1	
  
                  0,4993	
       0,4993	
       0,4994	
      0,4994	
       0,4994	
       0,4994	
      0,4994	
       0,4995	
       0,4995	
      0,4995	
  
    3,2	
  
Exemplo
  3,3	
     6
          0,4995	
               0,4995	
       0,4995	
      0,4996	
       0,4996	
       0,4996	
      0,4996	
       0,4996	
       0,4996	
      0,4997	
  

                  0,4997	
       0,4997	
       0,4997	
      0,4997	
       0,4997	
       0,4997	
      0,4997	
       0,4997	
       0,4997	
      0,4998	
  
    3,4	
  
Exemplifica
  3,5	
   0,4998	
  o cálculo
                        0,4998	
   da0,4998	
  
                                       probabilidade
                                                0,4998	
   em    que a0,4998	
  
                                                            0,4998	
    z-score0,4998	
  
                                                                                 está entre
                                                                                          0,4998	
  dois0,4998	
  
                                                                                                          valores0,4998	
  
                                                                                                                   determi-
nados
  3,6	
   que  contenham
           0,4998	
   0,4998	
  
                                 a0,4999	
  
                                    média.0,4999	
                           0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
  

                  0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
  
    3,7	
  
                  0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
  
    3,8	
  
                  0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
  
    3,9	
                                                                          213
    4	
           0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000

## Página 214

Vamos resolver o exemplo que apresentamos no início do estudo da distribuição Normal,
que foi determinado com a resolução da integral. Agora, vamos resolvê-lo com o uso da
tabela da distribuição normal padronizada.


Enunciado: Supondo que uma variável aleatória Normal apresente a média igual a 5 e a
variância igual a 4, determine a probabilidade de a variável aleatória estar entre 3 e 6.


Resolução:


    1º) Determine os valores da média μ e do desvio-padrão σ e nomeie a distribuição
    Normal: X~N(5;22)


    2º) Enuncie a probabilidade pedida: P(3 ≤ X ≤ 6) = ?


    3º) Transforme a variável aleatória Normal para Normal padronizada a partir da fórmula:


                           X-μ             3-5
     Para X = 3  Z1 =            =                =-1
                            σ               2


                           X-μ             6-5
     Para X = 6  Z2 =            =                = - 0,5
                            σ               2


    4º) Desenhe as curvas e destaque a região cuja área será investigada para determi-
    nar a probabilidade pedida.


                                                             σ=1




                                      -1        μ=0 0,5
                             Distribuição Normal Padronizada


                           Fonte: Elaborada pela autora (2020).


    5º) Utilize a tabela da distribuição Normal padronizada e busque o valor das áreas.
    Primeiramente para Z=-1 ou Z=1. Com duas casas decimais Z = 1,00  A=0,3413




                                                 214

## Página 215

                                                     Tabela da Distribuição Normal

      DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
            0	
               0,0040	
           0,0080	
           0,0120	
          0,0160	
           0,0199	
          0,0239	
           0,0279	
           0,0319	
          0,0359	
  
0	
  
            0,0398	
          0,0438	
           0,0478	
           0,0517	
          0,0557	
           0,0596	
          0,0636	
           0,0675	
           0,0714	
          0,0753	
  
0,1	
  
0,2	
       0,0793	
          0,0832	
           0,0871	
           0,0910	
          0,0948	
           0,0987	
          0,1026	
           0,1064	
           0,1103	
          0,1141	
  

            0,1179	
          0,1217	
           0,1255	
           0,1293	
          0,1331	
           0,1368	
          0,1406	
           0,1443	
           0,1480	
          0,1517	
  
0,3	
  
            0,1554	
          0,1591	
           0,1664	
           0,1664	
          0,1700	
           0,1736	
          0,1772	
           0,1808	
           0,1844	
          0,1879	
  
0,4	
  
            0,1915	
          0,1950	
           0,1985	
           0,2019	
          0,2054	
           0,2088	
          0,2123	
           0,2157	
           0,2190	
          0,2224	
  
0,5	
  
            0,2257	
          0,2291	
           0,2324	
           0,2357	
          0,2389	
           0,2422	
          0,2454	
           0,2486	
           0,2517	
          0,2549	
  
0,6	
  
            0,2580	
          0,2611	
           0,2642	
           0,2673	
          0,2704	
           0,2734	
          0,2764	
           0,2794	
           0,2823	
          0,2852	
  
0,7	
  
            0,2881	
          0,2910	
           0,2939	
           0,2967	
          0,2995	
           0,3023	
          0,3051	
           0,3078	
           0,3106	
          0,3133	
  
0,8	
  
            0,3159	
          0,3186	
           0,3212	
           0,3238	
          0,3264	
           0,3289	
          0,3315	
           0,3340	
           0,3365	
          0,3389	
  
0,9	
  
            0,3413	
  
            0,3413            0,3438	
           0,3461	
           0,3485	
          0,3508	
           0,3531	
          0,3554	
           0,3577	
           0,3599	
          0,3621	
  
1	
  
1,1	
       0,3643	
          0,3665	
           0,3686	
           0,3708	
          0,3729	
           0,3749	
          0,3770	
           0,3790	
           0,3810	
          0,3830	
  

            0,3849	
          0,3869	
           0,3888	
           0,3907	
          0,3925	
           0,3944	
          0,3962	
           0,3980	
           0,3997	
          0,4015	
  
1,2	
  
1,3	
  
        Agora  para0,4049	
  
          0,4032	
   Z=0,5 . Com Tabela
                              0,4066	
   duas  da
                                               casas
                                           0,4082	
   Distribuição
                                                         decimais
                                                       0,4099	
              Normal
                                                                       Z= 0,50
                                                                  0,4115	
          A=0,1915
                                                                              0,4131	
   0,4147	
   0,4162	
                                                                       0,4177	
  

            0,4192	
          0,4207	
           0,4222	
           0,4236	
          0,4251	
           0,4265	
          0,4279	
           0,4292	
           0,4306	
          0,4319	
  
1,4	
  
1,5	
   0,4332	
        0,4345	
    0,4357	
       0,4370	
                0,4382	
                         0,4394	
     0,4406	
         0,4418	
            0,4429	
       0,4441	
  
      DISTRIBUIÇÃO	
      N ORMAL	
    – 	
   Á REA	
   D E	
   0 	
   a té	
   K 	
   = >	
   	
   	
   p robabilidade	
    d   a	
   v ariável	
   z 	
   e star	
   e ntre	
   0	
  e	
  k	
  
1,6	
   0,4452	
        0,4463	
    0,4474	
       0,4484	
                0,4495	
                         0,4505	
     0,4515	
         0,4525	
            0,4535	
       0,4545	
  
z	
       0	
           0,01	
      0,02	
         0,03	
                  0,04	
                           0,05	
       0,06	
           0,07	
              0,08	
         0,09	
  
1,7	
   0,4554	
  
          0	
  
                        0,4564	
  
                        0,0040	
  
                                    0,4573	
  
                                    0,0080	
  
                                                   0,4582	
  
                                                   0,0120	
  
                                                                           0,4591	
  
                                                                           0,0160	
  
                                                                                                            0,4599	
  
                                                                                                            0,0199	
  
                                                                                                                         0,4608	
  
                                                                                                                         0,0239	
  
                                                                                                                                          0,4616	
  
                                                                                                                                          0,0279	
  
                                                                                                                                                              0,4625	
  
                                                                                                                                                              0,0319	
  
                                                                                                                                                                             0,4633	
  
                                                                                                                                                                             0,0359	
  
0	
  
1,8	
   0,4641	
        0,4649	
    0,4656	
       0,4664	
                0,4671	
                         0,4678	
     0,4686	
         0,4693	
            0,4699	
       0,4706	
  
0,1	
   0,0398	
        0,0438	
    0,0478	
       0,0517	
                0,0557	
                         0,0596	
     0,0636	
         0,0675	
            0,0714	
       0,0753	
  
1,9	
   0,4713	
        0,4719	
    0,4726	
       0,4732	
                0,4738	
                         0,4744	
     0,4750	
         0,4756	
            0,4761	
       0,4767	
  
0,2	
   0,0793	
  
          0,4772	
  
                        0,0832	
  
                        0,4778	
  
                                    0,0871	
  
                                    0,4783	
  
                                                   0,0910	
  
                                                   0,4788	
  
                                                                           0,0948	
  
                                                                           0,4793	
  
                                                                                                            0,0987	
  
                                                                                                            0,4798	
  
                                                                                                                         0,1026	
  
                                                                                                                         0,4803	
  
                                                                                                                                          0,1064	
  
                                                                                                                                          0,4808	
  
                                                                                                                                                              0,1103	
  
                                                                                                                                                              0,4812	
  
                                                                                                                                                                             0,1141	
  
                                                                                                                                                                             0,4817	
  
2	
  
0,3	
   0,1179	
  
          0,4821	
  
                        0,1217	
  
                        0,4826	
  
                                    0,1255	
  
                                    0,4830	
  
                                                   0,1293	
  
                                                   0,4834	
  
                                                                           0,1331	
  
                                                                           0,4838	
  
                                                                                                            0,1368	
  
                                                                                                            0,4842	
  
                                                                                                                         0,1406	
  
                                                                                                                         0,4846	
  
                                                                                                                                          0,1443	
  
                                                                                                                                          0,4850	
  
                                                                                                                                                              0,1480	
  
                                                                                                                                                              0,4854	
  
                                                                                                                                                                             0,1517	
  
                                                                                                                                                                             0,4857	
  
2,1	
   0,1554	
        0,1591	
    0,1664	
       0,1664	
                0,1700	
                         0,1736	
     0,1772	
         0,1808	
            0,1844	
       0,1879	
  
0,4	
   0,4861	
        0,4864	
    0,4868	
       0,4871	
                0,4875	
                         0,4878	
     0,4881	
         0,4884	
            0,4887	
       0,4890	
  
2,2	
   0,1915
0,5	
   0,1915	
        0,1950	
    0,1985	
       0,2019	
                0,2054	
                         0,2088	
     0,2123	
         0,2157	
            0,2190	
       0,2224	
  
2,3	
   0,4893	
        0,4896	
    0,4898	
       0,4901	
                0,4904	
                         0,4906	
     0,4909	
         0,4911	
            0,4913	
       0,4916	
  
0,6	
   0,2257	
        0,2291	
    0,2324	
       0,2357	
                0,2389	
                         0,2422	
     0,2454	
         0,2486	
            0,2517	
       0,2549	
  
2,4	
   0,4918	
        0,4920	
    0,4922	
       0,4925	
                0,4927	
                         0,4929	
     0,4931	
         0,4932	
            0,4934	
       0,4936	
  
0,7	
   0,2580	
  
          0,4938	
  
                        0,2611	
  
                        0,4940	
  
                                    0,2642	
  
                                    0,4941	
  
                                                   0,2673	
  
                                                   0,4943	
  
                                                                           0,2704	
  
                                                                           0,4945	
  
                                                                                                            0,2734	
  
                                                                                                            0,4946	
  
                                                                                                                         0,2764	
  
                                                                                                                         0,4948	
  
                                                                                                                                          0,2794	
  
                                                                                                                                          0,4949	
  
                                                                                                                                                              0,2823	
  
                                                                                                                                                              0,4951	
  
                                                                                                                                                                             0,2852	
  
                                                                                                                                                                             0,4952	
  
2,5	
   Área     tabelada                                                                                                                 0,3078	
   Área         tabelada
0,8	
   0,2881	
  
          0,4953	
  
                        0,2910	
  
                        0,4955	
  
                                    0,2939	
  
                                    0,4956	
  
                                                   0,2967	
  
                                                   0,4957	
  
                                                                           0,2995	
  
                                                                           0,4959	
  
                                                                                                            0,3023	
  
                                                                                                            0,4960	
  
                                                                                                                         0,3051	
  
                                                                                                                         0,4961	
         0,4962	
  
                                                                                                                                                              0,3106	
  
                                                                                                                                                              0,4963	
  
                                                                                                                                                                             0,3133	
  
                                                                                                                                                                             0,4964	
  
2,6	
   para     z = -10,3186	
                                                                                                                               para
                                                                                                                                                              0,3365	
  z = 0,5
0,9	
   0,3159	
  
          0,4965	
      0,4966	
  
                                    0,3212	
  
                                    0,4967	
  
                                                   0,3238	
  
                                                   0,4968	
  
                                                                           0,3264	
  
                                                                           0,4969	
  
                                                                                                            0,3289	
  
                                                                                                            0,4970	
  
                                                                                                                         0,3315	
  
                                                                                                                         0,4971	
  
                                                                                                                                          0,3340	
  
                                                                                                                                          0,4972	
            0,4973	
  
                                                                                                                                                                             0,3389	
  
                                                                                                                                                                             0,4974	
  
2,7	
   0,3413	
        0,3438	
    0,3461	
       0,3485	
                0,3508	
                         0,3531	
     0,3554	
         0,3577	
            0,3599	
       0,3621	
  
1	
  
2,8	
   0,4974	
        0,4975	
    0,4976	
       0,4977	
                0,4977	
                         0,4978	
     0,4979	
         0,4979	
            0,4980	
       0,4981	
  
1,1	
   0,3643	
        0,3665	
    0,3686	
       0,3708	
                0,3729	
                         0,3749	
     0,3770	
         0,3790	
            0,3810	
       0,3830	
  
2,9	
   0,4981	
        0,4982	
    0,4982	
       0,4983	
                0,4984	
                         0,4984	
     0,4985	
         0,4985	
            0,4986	
       0,4986	
  
1,2	
   0,3849	
        0,3869	
  
          0,4987	
  0,3413
                        0,4987	
  
                                    0,3888	
  
                                    0,4987	
  
                                                   0,3907	
  
                                                   0,4988	
  
                                                                           0,3925	
  
                                                                           0,4988	
  
                                                                                                            0,3944	
  
                                                                                                            0,4989	
  
                                                                                                                         0,3962	
  
                                                                                                                         0,4989	
  
                                                                                                                                          0,3980	
  
                                                                                                                                          0,4989	
   0,1915
                                                                                                                                                              0,3997	
  
                                                                                                                                                              0,4990	
  
                                                                                                                                                                             0,4015	
  
                                                                                                                                                                             0,4990	
  
3	
  
1,3	
   0,4032	
        0,4049	
    0,4066	
       0,4082	
                0,4099	
                         0,4115	
     0,4131	
         0,4147	
            0,4162	
       0,4177	
  
3,1	
   0,4990	
        0,4991	
    0,4991	
       0,4991	
                0,4992	
                         0,4992	
     0,4992	
         0,4992	
            0,4993	
       0,4993	
  
1,4	
   0,4192	
        0,4207	
    0,4222	
       0,4236	
                0,4251	
                         0,4265	
     0,4279	
         0,4292	
            0,4306	
       0,4319	
  
3,2	
   0,4993	
  
                -1
                        0,4993	
  
                          μ=0 0,5
                                    0,4994	
       0,4994	
                0,4994	
                         0,4994	
     0,4994	
         0,4995	
  
                                                                                                                         0,4406	
   -1 0,4418	
  
                                                                                                                                               μ=0 0,4429	
  
                                                                                                                                                              0,4995	
       0,4995	
  
                                                                                                                                                                 0,5 0,4441	
  
1,5	
   0,4332	
        0,4345	
    0,4357	
       0,4370	
                0,4382	
                         0,4394	
  
3,3	
   0,4995	
        0,4995	
    0,4995	
       0,4996	
                0,4996	
                         0,4996	
     0,4996	
         0,4996	
            0,4996	
       0,4997	
  
1,6	
   0,4452	
  
          0,4997	
  
                        0,4463	
  
                        0,4997	
  
                                    0,4474	
  
                                    0,4997	
  
                                                   0,4484	
  
                                                   0,4997	
  
                                                                           0,4495	
  
                                                                           0,4997	
  
                                                                                                            0,4505	
  
                                                                                                            0,4997	
  
                                                                                                                         0,4515	
  
                                                                                                                         0,4997	
  
                                                                                                                                          0,4525	
  
                                                                                                                                          0,4997	
  
                                                                                                                                                              0,4535	
  
                                                                                                                                                              0,4997	
  
                                                                                                                                                                             0,4545	
  
                                                                                                                                                                             0,4998	
  
    Distribuição Normal
3,4	
                              Padronizada                                                              0,4599	
   Distribuição          Normal0,4625	
     Padronizada
1,7	
   0,4554	
        0,4564	
    0,4573	
       0,4582	
                0,4591	
                                      0,4608	
         0,4616	
                           0,4633	
  
3,5	
   0,4998	
        0,4998	
    0,4998	
       0,4998	
                0,4998	
                         0,4998	
     0,4998	
         0,4998	
            0,4998	
       0,4998	
  
1,8	
   0,4641	
        0,4649	
    0,4656	
       0,4664	
                0,4671	
                         0,4678	
     0,4686	
         0,4693	
            0,4699	
       0,4706	
  
3,6	
   0,4998	
        0,4998	
    0,4999	
  
                                            Fonte:
                                                   0,4999	
  
                                                         Elaborada
                                                                           0,4999	
  
                                                                           0,4738	
  pela 0,4744	
  
                                                                                                            0,4999	
     0,4999	
  
                                                                                                             autora (2020).
                                                                                                                                          0,4999	
            0,4999	
       0,4999	
  
1,9	
   0,4713	
        0,4719	
    0,4726	
       0,4732	
                                                              0,4750	
         0,4756	
            0,4761	
       0,4767	
  
3,7	
   0,4999	
  
          0,4772	
  
                        0,4999	
  
                        0,4778	
  
                                    0,4999	
  
                                    0,4783	
  
                                                   0,4999	
  
                                                   0,4788	
  
                                                                           0,4999	
  
                                                                           0,4793	
  
                                                                                                            0,4999	
  
                                                                                                            0,4798	
  
                                                                                                                         0,4999	
  
                                                                                                                         0,4803	
  
                                                                                                                                          0,4999	
  
                                                                                                                                          0,4808	
  
                                                                                                                                                              0,4999	
  
                                                                                                                                                              0,4812	
  
                                                                                                                                                                             0,4999	
  
                                                                                                                                                                             0,4817	
  
2	
  
3,8	
   0,4999	
        0,4999	
    0,4999	
       0,4999	
                0,4999	
                         0,4999	
     0,4999	
         0,4999	
            0,4999	
       0,4999	
  
2,1	
   0,4821	
        0,4826	
    0,4830	
       0,4834	
                0,4838	
                         0,4842	
     0,4846	
         0,4850	
            0,4854	
       0,4857	
  
3,9	
   0,5000	
        0,5000	
    0,5000	
       0,5000	
                0,5000	
                         0,5000	
     0,5000	
         0,5000	
            0,5000	
       0,5000	
  
2,2	
   0,4861	
  
          0,5000	
  
                        0,4864	
  
                        0,5000	
  
                                    0,4868	
  
                                    0,5000	
  
                                                   0,4871	
  
                                                   0,5000	
  
                                                                           0,4875	
  
                                                                           0,5000	
  
                                                                                                            0,4878	
  
                                                                                                            0,5000	
  
                                                                                                                         0,4881	
  
                                                                                                                         0,5000	
  
                                                                                                                                          0,4884	
  
                                                                                                                                          0,5000	
  
                                                                                                                                                              0,4887	
  
                                                                                                                                                              0,5000	
  
                                                                                                                                                                             0,4890	
  
                                                                                                                                                                             0,5000	
  
4	
  
2,3	
   0,4893	
        0,4896	
    0,4898	
       0,4901	
                0,4904	
                         0,4906	
     0,4909	
         0,4911	
            0,4913	
       0,4916	
  

            0,4918	
          0,4920	
           0,4922	
           0,4925	
          0,4927	
           0,4929	
          0,4931	
           0,4932	
           0,4934	
          0,4936	
  
2,4	
  
                                                                                              215
            0,4938	
          0,4940	
           0,4941	
           0,4943	
          0,4945	
           0,4946	
          0,4948	
           0,4949	
           0,4951	
          0,4952	
  
2,5	
  
2,6	
       0,4953	
          0,4955	
           0,4956	
           0,4957	
          0,4959	
           0,4960	
          0,4961	
           0,4962	
           0,4963	
          0,4964

## Página 216

                                  Área a ser determinada




                                         0,3413 0,1915



                                    -1       μ=0    0,5
                             Distribuição Normal Padronizada

                               P( 3 ≤ X ≤ 6) = P(-1 ≤ Z ≤ 0,5)

                            Fonte: Elaborada pela autora (2020).


     6º) Como a área a ser determinada é exatamente a soma das duas áreas tabeladas
     temos:


     Área a ser determinada= área tabelada 1 + área tabelada 2
     Area a ser determinada = 0,3413 +0,1915
     Área a ser determinada = 0,5328


     Logo, a probabilidade pedida será igual a área a ser determinada.


     P(3 ≤ X ≤ 6) = P(-1 ≤ Z ≤ 0,5) = 0,5328 = 53,28%


Obtivemos o mesmo valor encontrado resolvendo a integral definida,

 6    1        (x-5)2
∫3          e– 2 · 4
     2 2π

porém com o cálculo de apenas uma adição de duas áreas. As tabelas simplificam sig-
nificativamente os cálculos.


Esse mesmo raciocínio do exemplo 6 será usado, caso seja preciso fazer o cálculo da
probabilidade em que a z-score está entre dois valores determinados depois da média.


Exemplo 7


É um exemplo inverso. Nesse caso, sabemos a probabilidade pela qual queremos deter-
minar o valor da variável aleatória.




                                              216

## Página 217

Enunciado: As notas dos alunos de Estatística estão normalmente distribuídas com mé-
dia igual a 80 pontos e desvio-padrão igual a 5 pontos. A universidade vai premiar 10%
dos alunos que obtiverem as maiores notas na disciplina. Qual deve nota um aluno deve
obter para que seja premiado?


Resolução:


Observe que agora conhecemos a área sob a curva e precisamos determinar o z-score e
consequentemente o valor da variável aleatória.


Observe o desenho da situação:


                          σ=5                                                σ=1


                                10% ou 0,1                                           10% ou 0,1




                 μ=80                                               μ=0
Distribuição Normal                                Distribuição Normal Padronizada
                                 ???????                                              ???????



                          Fonte: Elaborada pela autora (2020).


Como sabemos que a curva tem 50% das probabilidades antes da média e 50% depois
da média, então:

                        40% ou 0,4                                         40% ou 0,4

                          σ=5                                                σ=1

                                10% ou 0,1                                           10% ou 0,1




                 μ=80                                               μ=0
Distribuição Normal                                Distribuição Normal Padronizada
                                 ???????                                              ???????



                          Fonte: Elaborada pela autora (2020).




                                             217

## Página 218

Vamos buscar no corpo tabela de distribuição Normal padronizada o valor de z que gera
uma área igual a 0,4.
                       Tabela da Distribuição Normal

          DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
    z	
       0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
              0	
              0,0040	
     0,0080	
      0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
    0	
  
              0,0398	
         0,0438	
     0,0478	
      0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
    0,1	
  
    0,2	
     0,0793	
         0,0832	
     0,0871	
      0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

              0,1179	
         0,1217	
     0,1255	
      0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
    0,3	
  
              0,1554	
         0,1591	
     0,1664	
      0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
    0,4	
  
              0,1915	
         0,1950	
     0,1985	
      0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
    0,5	
  
              0,2257	
         0,2291	
     0,2324	
      0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
    0,6	
  
              0,2580	
         0,2611	
     0,2642	
      0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
    0,7	
  
              0,2881	
         0,2910	
     0,2939	
      0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
    0,8	
  
              0,3159	
         0,3186	
     0,3212	
      0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
    0,9	
  
              0,3413	
         0,3438	
     0,3461	
      0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
    1	
  
    1,1	
     0,3643	
         0,3665	
     0,3686	
      0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

              0,3849	
         0,3869	
     0,3888	
      0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
    1,2	
  
              0,4032	
         0,4049	
     0,4066	
      0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
    1,3	
  
              0,4192	
         0,4207	
     0,4222	
      0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
Com
  1,4	
  isso, descobrimos o valor do z-score, Z=1,28
              0,4332	
         0,4345	
     0,4357	
      0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
       0,4429	
       0,4441	
  
    1,5	
  
              0,4452	
         0,4463	
     0,4474	
      0,4484	
       0,4495	
       0,4505	
  
  1,6	
  determinarmos o valor da variável aleatória X, basta substituir os valores na fórmula:        0,4515	
       0,4525	
       0,4535	
       0,4545	
  
Para
              0,4554	
         0,4564	
     0,4573	
      0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
    1,7	
  
              0,4641	
         0,4649	
     0,4656	
      0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
    1,8	
  
                X-μ
  Z1,9	
  
    =         0,4713	
         0,4719	
     0,4726	
      0,4732	
       0,4738	
       0,4744	
       0,4750	
       0,4756	
       0,4761	
       0,4767	
  

    2	
  
                  σ
              0,4772	
         0,4778	
     0,4783	
      0,4788	
       0,4793	
       0,4798	
       0,4803	
       0,4808	
       0,4812	
       0,4817	
  

    2,1	
     0,4821	
         0,4826	
     0,4830	
      0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

    2,2	
         X - 800,4864	
  
              0,4861	
                      0,4868	
      0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
 1,28 =
    2,3	
     0,4893	
  
                           5   0,4896	
     0,4898	
      0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916	
  

              0,4918	
         0,4920	
     0,4922	
      0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
       0,4934	
       0,4936	
  
    2,4	
  
              0,4938	
         0,4940	
     0,4941	
      0,4943	
       0,4945	
       0,4946	
       0,4948	
       0,4949	
       0,4951	
       0,4952	
  
   2,5	
  
1,28 · 5 = X - 80
    2,6	
     0,4953	
         0,4955	
     0,4956	
      0,4957	
       0,4959	
       0,4960	
       0,4961	
       0,4962	
       0,4963	
       0,4964	
  

              0,4965	
         0,4966	
     0,4967	
      0,4968	
       0,4969	
       0,4970	
       0,4971	
       0,4972	
       0,4973	
       0,4974	
  
6,42,7	
  
    = X - 80
              0,4974	
         0,4975	
     0,4976	
      0,4977	
       0,4977	
       0,4978	
       0,4979	
       0,4979	
       0,4980	
       0,4981	
  
    2,8	
  
              0,4981	
         0,4982	
     0,4982	
      0,4983	
       0,4984	
       0,4984	
       0,4985	
       0,4985	
       0,4986	
       0,4986	
  
    2,9	
  
6,4 + 800,4987	
  
         =X                    0,4987	
     0,4987	
      0,4988	
       0,4988	
       0,4989	
       0,4989	
       0,4989	
       0,4990	
       0,4990	
  
    3	
  
              0,4990	
         0,4991	
     0,4991	
      0,4991	
       0,4992	
       0,4992	
       0,4992	
       0,4992	
       0,4993	
       0,4993	
  
    3,1	
  
86,4 = X0,4993	
  
   3,2	
                       0,4993	
     0,4994	
      0,4994	
       0,4994	
       0,4994	
       0,4994	
       0,4995	
       0,4995	
       0,4995	
  

              0,4995	
         0,4995	
     0,4995	
      0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4997	
  
    3,3	
  
Logo,      podemos 0,4997	
  
   3,4	
   0,4997	
     concluir0,4997	
  
                                   que os 0,4997	
  
                                              estudantes0,4997	
  com  notas 0,4997	
  
                                                                     0,4997	
   iguais ou0,4997	
  
                                                                                             superiores
                                                                                                      0,4997	
   a 86,4  serão
                                                                                                                    0,4998	
  

premiados
   3,5	
          pela 0,4998	
  
             0,4998	
   universidade.
                                  0,4998	
   0,4998	
   0,4998	
     0,4998	
   0,4998	
   0,4998	
   0,4998	
      0,4998	
  

              0,4998	
         0,4998	
     0,4999	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
    3,6	
  
              0,4999	
         0,4999	
     0,4999	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
    3,7	
  
              0,4999	
         0,4999	
     0,4999	
      0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
    3,8	
  
    3,9	
     0,5000	
         0,5000	
     0,5000	
      0,5000	
            218
                                                                         0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  

    4	
       0,5000	
         0,5000	
     0,5000	
      0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000

## Página 219

Distribuições de Probabilidades com o Excel

A planilha eletrônica Excel tem a capacidade de calcular probabilidades de diversas dis-
tribuições de probabilidade. Apresentaremos, na tabela a seguir, os comandos das distri-
buições que foram estudados nesta unidade.


  Distribuição                Sintaxe                              Descrição


                                                       x: número de tentativas
                                                       bem-sucedidas.

                                                       n: número de repetições.

                                                       p = probabilidade de sucesso
   Binomial      = DISTRBIONOM (x; n; p; cumulativo)   em cada tentativa.

                                                       Cumulativo: usado para acumular
                                                       as probabilidades. Tem valor
                                                       lógico 0 ou 1:
                                                       0  probabilidade pontual
                                                       1  probabilidade acumulada.


                                                       x: número de eventos.

                                                       μ: valor numérico esperado.

    Poisson         = POISSON (x; μ; cumulativo)       Cumulativo: usado para acumular
                                                       as probabilidades. Tem valor
                                                       lógico 0 ou 1:
                                                       0 probabilidade pontual.
                                                       1  probabilidade acumulada.


                                                       x: valor cuja distribuição você
                                                       deseja obter.

                                                       μ: média aritmética da
                                                       distribuição.

    Normal       = DIST.NORM.N (x; μ; σ; cumulativo)   σ = desvio-padrão da distribuição

                                                       Cumulativo: usado para acumular
                                                       as probabilidades. Tem valor
                                                       lógico 0 ou 1:
                                                       0 probabilidade não acumulada.
                                                       1  probabilidade acumulada.




                                             219

## Página 220

Agora, com o auxílio da planilha eletrônica Excel resolveremos alguns dos exemplos prá-
ticos apresentados anteriormente.


Exemplo 1


Um professor de Estatística elaborou uma avaliação de múltipla escolha, que consiste
em 10 questões, cada uma com cinco alternativas para resposta e apenas uma dessas
alternativas tem a resposta correta. Suponha que os estudantes que irão a fazer a prova
não frequentaram as aulas e não estudaram para a avaliação. Sendo assim, eles irão
marcar a resposta aleatoriamente, o que informalmente chamamos de “chute”. Para ser
aprovado, o aluno deve acertar no mínimo sete questões. Um estudante é selecionado
ao acaso.


    a) Qual é a probabilidade de ele “chutar” as 10 questões e acertar 7?


    Sabemos que: X ~B(10,2)
    Desejamos saber: P(X=7)


       COMANDO                                                 RESPOSTA

       = DISTRBINOM (7;10;0,2;0)                               0,00078643



    b) Qual é a probabilidade de ele ser aprovado?


    Desejamos saber P(X ≥ 7)


       COMANDO                                                 RESPOSTA

       = DISTRBINOM (7;10;0,2;0) + DISTRBINOM (8;10;0,2;0) +
                                                               0,000864
       DISTRBINOM (9;10;0,2;0) + DISTRBINOM (10;10;0,2;0)

       Outra opção (usando a opção acumulada)

       = 1- DISTRBINOM (6;10;0,2;1)                            0,000864



    c) Qual é o valor esperado de acertos nessas condições.


       COMANDO                                                 RESPOSTA

       = 10.0,2                                                2




                                              220

## Página 221

    d) O que podemos concluir de um aluno que não estuda e “chuta” as questões
    da avaliação?


       COMANDO                                                 RESPOSTA

       Isso o Excel não faz. As conclusões ainda dependem da
                                                               ?????
       interpretação humana.



Exemplo 2


Intervalo de tempo


Uma concessionária de rodovias está interessada em modelar o número de carros que
chegam em uma determinada cabide de um pedágio em um período de 15 minutos. Para
isso, verifica-se que a chegada dos carros satisfaz às seguintes hipóteses:
      • A probabilidade de um carro chegar ao pedágio é a mesma para dois períodos
      quaisquer de igual duração.
      • O fato de carros chegarem ou não em qualquer período independe da chegada ou
      não de outro carro em qualquer período.
      • Segundo dados históricos o número médio de carros que passam por esse trecho
      da estrada é de 10 carros no período de 15 minutos.


Determine a probabilidade de:


    a) Exatamente cinco carros chegarem em um período de 15 minutos nessa cabine.
    b) Exatamente um carro chegar em um período de três minutos nessa cabine.


    a) Como vimos na resolução analítica do exercício: X ~ P(10)
    Desejamos saber P(X=5)


       COMANDO                                                 RESPOSTA

       = POISSON (5;10;0)                                      0,03783327



    b) Como vimos na resolução analítica do exercício: X ~ P(2)
    Desejamos saber P(X=1)


       COMANDO                                                 RESPOSTA

       = POISSON (1;2;0)                                       0,270671




                                            221

## Página 222

Exemplo 3


Em agosto de 2020, um brasileiro passou em média 77 horas conectado à internet na-
vegando em redes sociais. Suponha que os tempos estejam normalmente distribuídos e
que o desvio-padrão seja de 20 horas.


Qual é a probabilidade de um brasileiro, escolhido aleatoriamente, ter passado menos
de 50 horas conectado à internet em agosto de 2020?


Sabemos que: X ~ N (77,202)
Desejamos saber P(X ≤ 50)


   COMANDO                                                     RESPOSTA

   = DIST.NORM.N (50;77;20;1)                                  0,08850799



Exemplo 4


Os “pesos” (massa corporal) dos frequentadores de uma academia de ginástica são
normalmente distribuídos, com média de 65,3kg e desvio-padrão 5,5. Um frequentador é
selecionado ao acaso.


Determine a probabilidade de seu peso ser maior do que 63,2 kg.


Sabemos que: X ~ N (65,3;5,52)
Desejamos saber P(X ≥ 63,2)


   COMANDO                                                     RESPOSTA

   = 1 - DIST.NORM.N (63,2;65,3;5,5;1)                         0,64870188



Exemplo 5


Supondo que uma variável aleatória Normal apresente a média igual a 5 e a variância
igual a 4.


Determine a probabilidade de a variável aleatória estar entre 3 e 6.


Sabemos que X ~ N (5;22)
Desejamos saber: P(3 ≤ X ≤ 6) = ?


                                            222

## Página 223

COMANDO                                                        RESPOSTA

= DIST.NORM.N (6;5;2;1) - DIST.NORM.N (3;5;2;1)                0,53280721




       Ampliando o foco praticando


     Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca
     Virtual, escolha um livro de Estatística, selecione e faça alguns exercícios do
     livro para reforçar o conhecimento.


     Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
     com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                             223

## Página 224

                               MIDIATECA

Para ampliar o seu conhecimento veja o material complementar da Unidade 3,
disponível na midiateca.




                              NA PRÁTICA

O problema da Grear Tire Company


Suponha que a Grear Tire Company tenha desenvolvido um pneu radial com
cinturão de aço, que será vendido por meio de uma cadeia nacional de discount
stores. Uma vez que esse tipo de pneu é um novo produto, os gerentes da Grear
acreditam que a durabilidade (em termos de milhas rodadas) oferecida com o
pneu será um fator importante na aceitação do produto. Antes de definirem os
termos do contrato de garantia de durabilidade do pneu, os gerentes da Grear
desejam obter informações de probabilidade a respeito do número de milhas
que os pneus durarão.


A partir dos testes reais de estrada com os pneus, a equipe de engenharia da
Grear estima que a durabilidade média dos pneus é μ = 36.500 milhas (58.741
km) e que o desvio-padrão é σ = 5000 milhas. Além disso, os dados coletados
indicam que a distribuição Normal é uma suposição razoável.


Qual a porcentagem de os pneus possivelmente durarem mais de 40.000
milhas (64.373 km)?


Utilizando os conceitos da distribuição Normal chegamos facilmente à reposta,
ou seja 24,2% dos pneus terão durabilidade maior do que 40.000 milhas.


Imagine que a Grear esteja considerando a possibilidade de dar uma garantia
que concede um desconto na troca de pneus se os originais não resistirem ao
número de milhas estipulado na garantia.




                                    224

## Página 225

Qual deve ser o número de milhas coberto pela garantia, levando-se em conta
que a Grear quer que não mais do que 10% dos pneus sejam habilitados à
garantia do desconto?


Utilizando os conceitos da distribuição Normal chegamos à reposta: uma ga-
rantia de 30.100 milhas cumprirá o requisito de que aproximadamente 10% dos
pneus se habilitem à garantia. Talvez com essa informação, a empresa possa
fixar a garantia de durabilidade de seis pneus em 30 mil milhas. Logo, consta-
tamos o importante papel que as distribuições de probabilidade desempenham
em termos de produzir informações para a tomada de decisões. Ou seja, assim
que uma distribuição de probabilidade é estabelecida para uma aplicação em
particular, ela pode ser usada rápida e facilmente para se obter informações a
respeito do problema.


A probabilidade não determina a recomendação de uma decisão diretamente,
mas fornece informações que ajudam o tomador de decisão a entender melhor
os riscos e as incertezas associadas ao problema. Por fim, essas informações
podem auxiliá-lo a tomar uma boa decisão.


(Texto extraído do livro citado na fonte a seguir.)


Fonte:
SWEENEY, D. J.; WILLIAMS, T. A.; ANDERSON, D. R. Estatística aplicada à ad-
ministração e economia. 3. ed. São Paulo: Cengage Learning, 2013.




                                        225

## Página 226

Resumo da Unidade 3

Nesta unidade ampliamos o estudo da Teoria das probabilidades. Apresentamos os mo-
delos matemáticos que auxiliam na determinação das probabilidades de maneira fácil e
metódica.


Abordamos as distribuições de probabilidades para variáveis aleatórias discretas e para
variáveis aleatórias continuas. A principal diferença conceitual entre as distribuições dis-
cretas e continuas é que as distribuições discretas nos revelam a probabilidade de a
variável aleatória assumir valores pontuais. Já as distribuições continuas fornecem a pro-
babilidade a partir do cálculo da área sob a curva.


Também estudamos as distribuições discretas: a Binomial e a de Poisson, enquanto as
distribuições continuas apresentadas foram a Normal e a Normal Padronizada. Vimos
que a distribuição Normal tem grande importância para a Estatística e será utilizada tam-
bém em inferência estatística.


Por fim, apresentamos os comandos para utilizar as distribuições de probabilidades com
a planilha eletrônica Excel, que facilitam ainda mais a determinação das probabilidades.




                                            226

## Página 227

Referências

COMO usar as tabelas da Distribuição Normal. YouTube. 08/10/2013. Disponível em:
https://youtu.be/ec9HWoY2kt8. Acesso em: 6 set. 2020.


CURVA Normal. Disponível em: https://www.geogebra.org/m/whcbqg3r Acesso em:
3 set. 2020.


DISTRIBUIÇÃO binomial de probabilidade. YouTube. 25/09/2013. Disponível em: https://
www.youtube.com/watch?v=V2sfnVikFXA. Acesso em: 6 set. 2020.


KOKOSKA, S. Introdução à estatística: uma abordagem por resolução de problemas. Rio
de Janeiro: LTC, 2013.


LARSON, R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010.


SWEENEY, D. J.; WILLIAMS, T. A.; ANDERSON, D. R. Estatística aplicada à administração
e economia. 3. ed. São Paulo: Cengage Learning, 2013.


TABELA da Distribuição Normal padronizada. UVA. Disponível em: http://ead.uva.br///recur-
so/DEF/EST/u3_c3_r1/anexo/Tabela-da-Distribuicao-Normal.pdf Acesso em: 5 set. 2020.


VARIÁVEIS aleatórias. Khan Academy. Disponível em: https://pt.khanacademy.org/math/
statistics-probability/random-variables-stats-library. Acesso em: 4 set. 2020.




                                           227

## Página 228

           UNIDADE 4

Intervalos de confiança e
  Relação entre variáveis

## Página 229

                                               INTRODUÇÃO

Iniciamos os estudos desta disciplina falando da Estatística Descritiva, com suas técni-
cas para resumir um conjunto de dados por meio de tabelas, criar e apresentar gráficos
e ainda para calcular as medidas de posição e dispersão. Depois, vimos a teoria da pro-
babilidade e os modelos probabilísticos, discretos ou contínuos.


Agora, nesta unidade, esses conhecimentos serão utilizados no estudo de um significativo
ramo da Estatística, denominado de Inferência Estatística. A Inferência tem como objetivo
fornecer as ferramentas necessárias para fazermos generalizações sobre uma população,
a partir de evidências fornecidas por uma amostra representativa, retirada desta população.


O que veremos nos tópicos, a saber:


Tópico 1 – Intervalos de Confiança para médias e proporções e cálculo do tamanho da
amostra.
Tópico 2 – Conceitos de Correlação, medindo o grau de associação entre duas variáveis.
Tópico 3 – Concepções da Regressão Linear buscando a determinação de um modelo
matemático que descreva o relacionamento entre duas variáveis aleatórias.


Com os conceitos de Intervalos de Confiança, Correlação e Regressão Linear seremos
capazes de aplicar os conhecimentos adquiridos em projetos de pesquisa científica e
na solução de problemas de nossa área de atuação com mais eficiência.




                                       OBJETIVO

       Nesta unidade você será capaz de:

       • Utilizar os conceitos da inferência estatística para a resolução de problemas
       práticos.




                                            229

## Página 230

Intervalos de confiança

Antes de apresentarmos os princípios dos Intervalos de Confiança, definiremos alguns
conceitos importantes para um melhor entendimento do conteúdo, que são: parâmetros,
estimadores e estimativas.


Parâmetros

São as características numéricas de uma população. Geralmente são representadas por
letras gregas, tais como, θ, μ , σ, ρ entre outras. Em nossos estudos adotaremos “θ” para
simbolizarmos parâmetros, “μ” para representarmos a média populacional, “σ” quando
nos referirmos ao desvio-padrão populacional, “σ²” para variância populacional e “ρ” para
o coeficiente de correlação populacional, entre outros.


Estimadores

Também chamados de “estatística” de um parâmetro, são as características numéricas
determinada na amostra. Geralmente representamos por θ̂ (teta chapéu). São exemplos
de estimadores: a média amostral (( X̅ )) , a variância amostral (s²), o desvio-padrão amos-
tral (s), o coeficiente de correlação amostral (r), entre outros.


Entre várias características que os estimadores possuem devemos escolher estimado-
res que sejam:


     • Estimadores Não viciados, ou não tendenciosos ou, como dizemos em estatís-
     tica, não viesados: são aqueles em que o seu valor esperado é igual ao parâmetro
     estimado.
     • Estimadores consistentes: são aqueles que à medida que o tamanho da amostra
     aumenta, o estimador aproxima-se cada vez mais do parâmetro da população.
     • Estimadores eficientes: são aqueles que são não enviesados e possuem variân-
     cia mínima.


Estimativa

É valor numérico assumido por um estimador. Pode ser por ponto (um único valor) ou in-
tervalar (um intervalo de valores possíveis) de um parâmetro populacional desconhecido.




                                            230

## Página 231

Agora que já entendemos sobre os conceitos basilares falaremos de Intervalo de Confiança.


Intervalo de confiança

É um intervalo numérico aberto, de modo que podemos estar seguramente certos ou
confiantes de que o verdadeiro valor do parâmetro pertencerá a esse intervalo. Para ini-
ciarmos a determinação de um intervalo de confiança devemos escolher qual o nível de
confiança que iremos trabalhar. Temos dois níveis. São eles:



                           É a porcentagem de que o intervalo de confiança contenha o
        Nível de
                           parâmetro em amostragens repetidas. Os níveis de confiança
       confiança
                           mais utilizados são: 90%, 95% e 99%.


                           Representado pela letra grega a, é o complementar do nível de
        Nível de           confiança, ou seja, se o nível de confiança for 90%, o nível de
      significância        significância será 10%; para 95% de confiança, 5% de significân-
                           cia e 99% de confiança, 1% de significância.



Intervalos de confiança para Média

Temos dois casos a considerar quando queremos construir um intervalo de confiança
para a média populacional.


1º caso: Intervalo de confiança para média populacional quando o desvio-padrão po-
pulacional é conhecido.


Para construirmos esse intervalo de confiança devemos seguir cinco passos. Vejamos:


1º) Verificar se a população obedece aos princípios de uma distribuição normal, ou asse-
gurar que o tamanho da amostra seja suficientemente grande, ou seja, n > 30.


2º) Identificar o valor numérico do desvio-padrão populacional (σ); da média amostral
( X̅ ) e o tamanho da amostra (n).


3º) Calcular, então, o desvio-padrão da média com a utilização da fórmula:




                                           231

## Página 232

                                                   σ
                                      σx =
                                                   n


Em que:
σ x = desvio-padrão da média
σ = desvio-padrão populacional
n = tamanho da amostra

4º) Determinar com auxílio da tabela de distribuição normal padronizada o valor de Z-s-
core em função do nível de confiança. Como vimos na Unidade 3, existem vários tipos
de tabela normal padronizada e por isso temos que ter bastante cuidado nessa etapa.
Na tabela disponibilizada em nosso curso, buscamos a metade no nível de confiança, no
corpo da tabela, para determinar Z-score.

5º) Substituir os valores já conhecidos na fórmula do intervalo de confiança:



                                   IC = X̅ ± Zscore ∙ σ x



Vejamos alguns exemplos práticos a seguir.

Exemplo 1:
Considere que a altura dos alunos de Estatística estejam normalmente distribuídas com
o desvio-padrão populacional igual a 15 cm. Foi retirada uma amostra aleatoriamente,
composta de 100 alunos, e obteve-se a média amostral igual a 1,75 cm.

Determine o intervalo com 95% de confiança que contenha a verdadeira altura média
dos alunos de Estatística.

Resolução:
Vamos, então, determinar os cinco passos para construir esse intervalo de confiança, para
média μ, ao nível de confiança de 95%, com o desvio-padrão populacional conhecido:


                   Etapas                                        Respostas

    1º) Verificar se a população obedece
                                                   No enunciado verificamos que a popula-
    aos princípios de uma distribuição
                                                   ção formada pela medida da altura dos
    normal, ou assegurar que o tama-
                                                   alunos de Estatística está normalmente
    nho da amostra seja suficientemente
                                                   distribuída.
    grande, ou seja, n > 30;



                                             232

## Página 233

                                                    σ = 15 cm
2º) Identificar o valor numérico do desvio-pa-
drão populacional (σ); da média amostral ( X̅ )     X̅ = 175 m
e o tamanho da amostra (n);
                                                    n = 100

    3º) Calcular o desvio-padrão da mé-
    dia com a utilização da fórmula:                                                    15
                         σ                            σx =                                                      =1,5
                σx =                                                                  100
                         n

                                                    Se o desvio-padrão populacional é co-
                                                    nhecido e o tamanho da amostra é maior
                                                    do que 30, vamos usar a tabela normal
                                                    padronizada para determinar o valor de
                                                    Z-score.

                                                    Verificamos que o nível de confiança soli-
                                                    citado foi de 95%.

                                                    Como nossa tabela oferece os valores de
                                                    zero até z, temos que dividir esse nível de
                                                    confiança por 2, ou seja,
    4º) Determinar com o auxílio da ta-
    bela de distribuição normal padroni-
    zada o valor de Z-score em função                        95%                                                                          47,5%
                                                                                        = 47,5% =                                                                       = 0,475
    do nível de confiança. Como vimos                                2                                                                         100
    na Unidade 3, existem vários tipos de
    tabela normal padronizada e por isso            Buscamos então, no corpo da tabela de
    temos que ter bastante cuidado nes-             distribuição Tabela
                                                                 normal    padronizada
                                                                        da Distribuição Normal esse valor:

    sa etapa. Na tabela disponibilizada                     DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  

    em nosso curso, buscamos a meta-                z	
  
                                                    0	
  
                                                                0	
  
                                                                  0	
  
                                                                       0,01	
   0,02	
  
                                                                                 0,0040	
  
                                                                                               0,03	
  
                                                                                                0,0080	
  
                                                                                                                 0,04	
  
                                                                                                              0,0120	
       0,0160	
  
                                                                                                                                          0,05	
   0,06	
  
                                                                                                                                            0,0199	
  
                                                                                                                                                                 0,07	
  
                                                                                                                                                          0,0239	
  
                                                                                                                                                                                 0,08	
  
                                                                                                                                                                         0,0279	
  
                                                                                                                                                                                              0,09	
  
                                                                                                                                                                                        0,0319	
      0,0359	
  


    de no nível de confiança, no corpo da           0,1	
  
                                                    0,2	
  
                                                                  0,0398	
  

                                                                  0,0793	
  
                                                                                 0,0438	
  

                                                                                 0,0832	
  
                                                                                                0,0478	
  

                                                                                                0,0871	
  
                                                                                                              0,0517	
  

                                                                                                              0,0910	
  
                                                                                                                             0,0557	
  

                                                                                                                             0,0948	
  
                                                                                                                                            0,0596	
  

                                                                                                                                            0,0987	
  
                                                                                                                                                          0,0636	
  

                                                                                                                                                          0,1026	
  
                                                                                                                                                                         0,0675	
  

                                                                                                                                                                         0,1064	
  
                                                                                                                                                                                        0,0714	
  

                                                                                                                                                                                        0,1103	
  
                                                                                                                                                                                                      0,0753	
  

                                                                                                                                                                                                      0,1141	
  


    tabela, para determinar Z-score.                0,3	
  
                                                    0,4	
  
                                                                  0,1179	
  

                                                                  0,1554	
  
                                                                                 0,1217	
  

                                                                                 0,1591	
  
                                                                                                0,1255	
  

                                                                                                0,1664	
  
                                                                                                              0,1293	
  

                                                                                                              0,1664	
  
                                                                                                                             0,1331	
  

                                                                                                                             0,1700	
  
                                                                                                                                            0,1368	
  

                                                                                                                                            0,1736	
  
                                                                                                                                                          0,1406	
  

                                                                                                                                                          0,1772	
  
                                                                                                                                                                         0,1443	
  

                                                                                                                                                                         0,1808	
  
                                                                                                                                                                                        0,1480	
  

                                                                                                                                                                                        0,1844	
  
                                                                                                                                                                                                      0,1517	
  

                                                                                                                                                                                                      0,1879	
  

                                                                  0,1915	
       0,1950	
       0,1985	
      0,2019	
       0,2054	
       0,2088	
      0,2123	
       0,2157	
       0,2190	
      0,2224	
  
                                                    0,5	
  
                                                                  0,2257	
       0,2291	
       0,2324	
      0,2357	
       0,2389	
       0,2422	
      0,2454	
       0,2486	
       0,2517	
      0,2549	
  
                                                    0,6	
  
                                                                  0,2580	
       0,2611	
       0,2642	
      0,2673	
       0,2704	
       0,2734	
      0,2764	
       0,2794	
       0,2823	
      0,2852	
  
                                                    0,7	
  
                                                                  0,2881	
       0,2910	
       0,2939	
      0,2967	
       0,2995	
       0,3023	
      0,3051	
       0,3078	
       0,3106	
      0,3133	
  
                                                    0,8	
  
                                                                  0,3159	
       0,3186	
       0,3212	
      0,3238	
       0,3264	
       0,3289	
      0,3315	
       0,3340	
       0,3365	
      0,3389	
  
                                                    0,9	
  
                                                                  0,3413	
       0,3438	
       0,3461	
      0,3485	
       0,3508	
       0,3531	
      0,3554	
       0,3577	
       0,3599	
      0,3621	
  
                                                    1	
  
                                                    1,1	
         0,3643	
       0,3665	
       0,3686	
      0,3708	
       0,3729	
       0,3749	
      0,3770	
       0,3790	
       0,3810	
      0,3830	
  

                                                                  0,3849	
       0,3869	
       0,3888	
      0,3907	
       0,3925	
       0,3944	
      0,3962	
       0,3980	
       0,3997	
      0,4015	
  
                                                    1,2	
  
                                                                  0,4032	
       0,4049	
       0,4066	
      0,4082	
       0,4099	
       0,4115	
      0,4131	
       0,4147	
       0,4162	
      0,4177	
  
                                                    1,3	
  
                                                                  0,4192	
       0,4207	
       0,4222	
      0,4236	
       0,4251	
       0,4265	
      0,4279	
       0,4292	
       0,4306	
      0,4319	
  
                                                    1,4	
  
                                                                  0,4332	
       0,4345	
       0,4357	
      0,4370	
       0,4382	
       0,4394	
      0,4406	
       0,4418	
       0,4429	
      0,4441	
  
                                                    1,5	
  
                                                    1,6	
         0,4452	
       0,4463	
       0,4474	
      0,4484	
       0,4495	
       0,4505	
      0,4515	
       0,4525	
       0,4535	
      0,4545	
  

                                                                  0,4554	
       0,4564	
       0,4573	
      0,4582	
       0,4591	
       0,4599	
      0,4608	
       0,4616	
       0,4625	
      0,4633	
  
                                                    1,7	
  
                                                                  0,4641	
       0,4649	
       0,4656	
      0,4664	
       0,4671	
       0,4678	
      0,4686	
       0,4693	
       0,4699	
      0,4706	
  
                                                    1,8	
  
                                                                  0,4713	
       0,4719	
       0,4726	
      0,4732	
       0,4738	
       0,4744	
      0,4750	
       0,4756	
       0,4761	
      0,4767	
  
                                                    1,9	
  
                                                                  0,4772	
       0,4778	
       0,4783	
      0,4788	
       0,4793	
       0,4798	
      0,4803	
       0,4808	
       0,4812	
      0,4817	
  
                                                    2	
  
                                                    2,1	
         0,4821	
       0,4826	
       0,4830	
      0,4834	
       0,4838	
       0,4842	
      0,4846	
       0,4850	
       0,4854	
      0,4857	
  

                                                                  0,4861	
       0,4864	
       0,4868	
      0,4871	
       0,4875	
       0,4878	
      0,4881	
       0,4884	
       0,4887	
      0,4890	
  
                                                    2,2	
  
                                                                  0,4893	
       0,4896	
       0,4898	
      0,4901	
       0,4904	
       0,4906	
      0,4909	
       0,4911	
       0,4913	
      0,4916	
  
                                                    2,3	
  
                                                      Fonte: http://ead.uva.br///recurso/DEF/EST/u3_
                                                    2,4	
   0,4918	
   0,4920	
   0,4922	
   0,4925	
   0,4927	
   0,4929	
   0,4931	
   0,4932	
   0,4934	
   0,4936	
  


                                                     c3_r1/anexo/Tabela-da-Distribuicao-Normal.pdf
                                                    2,5	
   0,4938	
   0,4940	
   0,4941	
   0,4943	
   0,4945	
   0,4946	
   0,4948	
   0,4949	
   0,4951	
   0,4952	
  

                                                    2,6	
   0,4953	
   0,4955	
   0,4956	
   0,4957	
   0,4959	
   0,4960	
   0,4961	
   0,4962	
   0,4963	
   0,4964	
  

                                                                  0,4965	
       0,4966	
       0,4967	
      0,4968	
       0,4969	
       0,4970	
      0,4971	
       0,4972	
       0,4973	
      0,4974	
  
                                                    2,7	
  
                                                                  0,4974	
       0,4975	
       0,4976	
      0,4977	
       0,4977	
       0,4978	
      0,4979	
       0,4979	
       0,4980	
      0,4981	
  
                                                    2,8	
  
                                                                  0,4981	
       0,4982	
       0,4982	
      0,4983	
       0,4984	
       0,4984	
      0,4985	
       0,4985	
       0,4986	
      0,4986	
  
                                                    2,9	
  
                                                                  0,4987	
       0,4987	
       0,4987	
      0,4988	
       0,4988	
       0,4989	
      0,4989	
       0,4989	
       0,4990	
      0,4990	
  
                                                    3	
  
                                                                  0,4990	
       0,4991	
       0,4991	
      0,4991	
       0,4992	
       0,4992	
      0,4992	
       0,4992	
       0,4993	
      0,4993	
  
                                                    3,1	
  
                                                                  0,4993	
       0,4993	
       0,4994	
      0,4994	
       0,4994	
       0,4994	
      0,4994	
       0,4995	
       0,4995	
      0,4995	
  
                                                    3,2	
  
                                              233   3,3	
         0,4995	
       0,4995	
       0,4995	
      0,4996	
       0,4996	
       0,4996	
      0,4996	
       0,4996	
       0,4996	
      0,4997	
  

                                                                  0,4997	
       0,4997	
       0,4997	
      0,4997	
       0,4997	
       0,4997	
      0,4997	
       0,4997	
       0,4997	
      0,4998	
  
                                                    3,4	
  
                                                    3,5	
         0,4998	
       0,4998	
       0,4998	
      0,4998	
       0,4998	
       0,4998	
      0,4998	
       0,4998	
       0,4998	
      0,4998	
  

                                                                  0,4998	
       0,4998	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
  
                                                    3,6

## Página 234

                                                    Depois de identificado o valor da metade do
                                                    nível de confiança no corpo da tabela, de-
                                                    terminamos o Z-score usando na primeira
                                                    coluna correspondente o valor da parte in-
                                                    teira e da primeira casa decimal (1,9) e na
                                                    primeira linha o valor da segunda casa de-
                                                    cimal (0,06): Tabela da Distribuição Normal
                                                            DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
                                                    z	
         0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
                                                                  0	
            0,0040	
       0,0080	
      0,0120	
       0,0160	
       0,0199	
      0,0239	
       0,0279	
       0,0319	
      0,0359	
  
                                                    0	
  
                                                                  0,0398	
       0,0438	
       0,0478	
      0,0517	
       0,0557	
       0,0596	
      0,0636	
       0,0675	
       0,0714	
      0,0753	
  
                                                    0,1	
  
                                                    0,2	
         0,0793	
       0,0832	
       0,0871	
      0,0910	
       0,0948	
       0,0987	
      0,1026	
       0,1064	
       0,1103	
      0,1141	
  

                                                                  0,1179	
       0,1217	
       0,1255	
      0,1293	
       0,1331	
       0,1368	
      0,1406	
       0,1443	
       0,1480	
      0,1517	
  
                                                    0,3	
  
                                                                  0,1554	
       0,1591	
       0,1664	
      0,1664	
       0,1700	
       0,1736	
      0,1772	
       0,1808	
       0,1844	
      0,1879	
  
                                                    0,4	
  
                                                                  0,1915	
       0,1950	
       0,1985	
      0,2019	
       0,2054	
       0,2088	
      0,2123	
       0,2157	
       0,2190	
      0,2224	
  
                                                    0,5	
  
                                                                  0,2257	
       0,2291	
       0,2324	
      0,2357	
       0,2389	
       0,2422	
      0,2454	
       0,2486	
       0,2517	
      0,2549	
  
                                                    0,6	
  
                                                                  0,2580	
       0,2611	
       0,2642	
      0,2673	
       0,2704	
       0,2734	
      0,2764	
       0,2794	
       0,2823	
      0,2852	
  
                                                    0,7	
  
                                                                  0,2881	
       0,2910	
       0,2939	
      0,2967	
       0,2995	
       0,3023	
      0,3051	
       0,3078	
       0,3106	
      0,3133	
  
                                                    0,8	
  
                                                                  0,3159	
       0,3186	
       0,3212	
      0,3238	
       0,3264	
       0,3289	
      0,3315	
       0,3340	
       0,3365	
      0,3389	
  
                                                    0,9	
  
                                                                  0,3413	
       0,3438	
       0,3461	
      0,3485	
       0,3508	
       0,3531	
      0,3554	
       0,3577	
       0,3599	
      0,3621	
  
                                                    1	
  
                                                    1,1	
         0,3643	
       0,3665	
       0,3686	
      0,3708	
       0,3729	
       0,3749	
      0,3770	
       0,3790	
       0,3810	
      0,3830	
  

                                                                  0,3849	
       0,3869	
       0,3888	
      0,3907	
       0,3925	
       0,3944	
      0,3962	
       0,3980	
       0,3997	
      0,4015	
  
                                                    1,2	
  
                                                                  0,4032	
       0,4049	
       0,4066	
      0,4082	
       0,4099	
       0,4115	
      0,4131	
       0,4147	
       0,4162	
      0,4177	
  
                                                    1,3	
  
                                                                  0,4192	
       0,4207	
       0,4222	
      0,4236	
       0,4251	
       0,4265	
      0,4279	
       0,4292	
       0,4306	
      0,4319	
  
                                                    1,4	
  
                                                                  0,4332	
       0,4345	
       0,4357	
      0,4370	
       0,4382	
       0,4394	
      0,4406	
       0,4418	
       0,4429	
      0,4441	
  
                                                    1,5	
  
                                                    1,6	
         0,4452	
       0,4463	
       0,4474	
      0,4484	
       0,4495	
       0,4505	
      0,4515	
       0,4525	
       0,4535	
      0,4545	
  

                                                                  0,4554	
       0,4564	
       0,4573	
      0,4582	
       0,4591	
       0,4599	
      0,4608	
       0,4616	
       0,4625	
      0,4633	
  
                                                    1,7	
  
                                                                  0,4641	
       0,4649	
       0,4656	
      0,4664	
       0,4671	
       0,4678	
      0,4686	
       0,4693	
       0,4699	
      0,4706	
  
                                                    1,8	
  
                                                                  0,4713	
       0,4719	
       0,4726	
      0,4732	
       0,4738	
       0,4744	
      0,4750	
       0,4756	
       0,4761	
      0,4767	
  
                                                    1,9	
  
                                                                  0,4772	
       0,4778	
       0,4783	
      0,4788	
       0,4793	
       0,4798	
      0,4803	
       0,4808	
       0,4812	
      0,4817	
  
                                                    2	
  
                                                    2,1	
         0,4821	
       0,4826	
       0,4830	
      0,4834	
       0,4838	
       0,4842	
      0,4846	
       0,4850	
       0,4854	
      0,4857	
  

                                                                  0,4861	
       0,4864	
       0,4868	
      0,4871	
       0,4875	
       0,4878	
      0,4881	
       0,4884	
       0,4887	
      0,4890	
  
                                                    2,2	
  
                                                                  0,4893	
       0,4896	
       0,4898	
      0,4901	
       0,4904	
       0,4906	
      0,4909	
       0,4911	
       0,4913	
      0,4916	
  
                                                    2,3	
  
                                                                  0,4918	
       0,4920	
       0,4922	
   Logo,
                                                                                                            0,4925	
   Z0,4927	
   = 1,960,4931	
  
                                                    2,4	
                                                                  score 0,4929	
                                0,4932	
       0,4934	
      0,4936	
  

                                                                  0,4938	
       0,4940	
       0,4941	
      0,4943	
       0,4945	
       0,4946	
      0,4948	
       0,4949	
       0,4951	
      0,4952	
  
                                                    2,5	
  
                                                    2,6	
         0,4953	
       0,4955	
       0,4956	
      0,4957	
       0,4959	
       0,4960	
      0,4961	
       0,4962	
       0,4963	
      0,4964	
  

                                                                  0,4965	
       0,4966	
       0,4967	
      0,4968	
       0,4969	
       0,4970	
      0,4971	
       0,4972	
       0,4973	
      0,4974	
  
                                                    2,7	
  
                                                                  0,4974	
       0,4975	
       0,4976	
      0,4977	
       0,4977	
       0,4978	
      0,4979	
       0,4979	
       0,4980	
      0,4981	
  
                                                    2,8	
  
                                                                  0,4981	
       0,4982	
       0,4982	
      0,4983	
       0,4984	
       0,4984	
      0,4985	
       0,4985	
       0,4986	
      0,4986	
  
                                                    2,9	
  
                                                                  0,4987	
       0,4987	
       0,4987	
      0,4988	
       0,4988	
       0,4989	
      0,4989	
       0,4989	
       0,4990	
      0,4990	
  
                                                    3	
  
                                                    3,1	
  
                                                    3,2	
  
                                                                  0,4990	
  

                                                                  0,4993	
  
                                                                                 0,4991	
  

                                                                                 0,4993	
  
                                                                                                    IC = X̅ ± Zscore ∙ σx
                                                                                                0,4991	
  

                                                                                                0,4994	
  
                                                                                                              0,4991	
  

                                                                                                              0,4994	
  
                                                                                                                             0,4992	
  

                                                                                                                             0,4994	
  
                                                                                                                                            0,4992	
  

                                                                                                                                            0,4994	
  
                                                                                                                                                          0,4992	
  

                                                                                                                                                          0,4994	
  
                                                                                                                                                                         0,4992	
  

                                                                                                                                                                         0,4995	
  
                                                                                                                                                                                        0,4993	
  

                                                                                                                                                                                        0,4995	
  
                                                                                                                                                                                                      0,4993	
  

                                                                                                                                                                                                      0,4995	
  

                                                                  0,4995	
       0,4995	
       0,4995	
      0,4996	
       0,4996	
       0,4996	
      0,4996	
       0,4996	
       0,4996	
      0,4997	
  
                                                    3,3	
  
                                                                  0,4997	
       0,4997	
       0,4997	
      0,4997	
       0,4997	
       0,4997	
      0,4997	
       0,4997	
       0,4997	
      0,4998	
  
                                                    3,4	
  
                                                    3,5	
         0,4998	
  

                                                                  0,4998	
  
                                                                                        IC = 175 ± 1,96 ∙ 1,5
                                                                                 0,4998	
  

                                                                                 0,4998	
  
                                                                                                0,4998	
  

                                                                                                0,4999	
  
                                                                                                              0,4998	
  

                                                                                                              0,4999	
  
                                                                                                                             0,4998	
  

                                                                                                                             0,4999	
  
                                                                                                                                            0,4998	
  

                                                                                                                                            0,4999	
  
                                                                                                                                                          0,4998	
  

                                                                                                                                                          0,4999	
  
                                                                                                                                                                         0,4998	
  

                                                                                                                                                                         0,4999	
  
                                                                                                                                                                                        0,4998	
  

                                                                                                                                                                                        0,4999	
  
                                                                                                                                                                                                      0,4998	
  

                                                                                                                                                                                                      0,4999	
  

    5º) Substituir os valores já conhecidos
                                                    3,6	
  
                                                    3,7	
         0,4999	
       0,4999	
  
                                                                                           IC = 175 ± 2,94
                                                                                                0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
  



    na fórmula do intervalo de confiança:
                                                                  0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
       0,4999	
       0,4999	
      0,4999	
  
                                                    3,8	
  
                                                    3,9	
         0,5000	
  
                                                                                      IC = 175 - 2,94 = 172,06
                                                                                 0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
  

                                                    4	
           0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
       0,5000	
       0,5000	
      0,5000	
  

                                                                                     IC = 175 + 2,94 = 177,94
              IC = X̅ ± Zscore ∙ σx
                                                    Podemos afirmar, com 95% de confian-
                                                    ça, que o intervalo ]172,06;177,94[ con-
                                                    tém o verdadeiro valor da altura média
                                                    dos alunos.



Exemplo 2:
Deseja-se estimar a quantia que cada cliente gasta para jantar em um famoso restauran-
te do Rio de Janeiro. Para isso, foram coletados dados de uma amostra de 40 clientes,
cuja média de gastos foi igual a R$ 173,88. Sabe-se que há dados históricos que nos
permitem obter uma boa estimativa do desvio-padrão populacional, que, neste caso, é
igual a R$ 10,00.


                                              234

## Página 235

Construa um intervalo de confiança de 99% para o verdadeiro valor dos gastos mé-
dios dos clientes desse famoso restaurante por jantar, sabendo que esses gastos
estão normalmente distribuídos.


                   Etapas                                          Respostas


    1º) Verificar se a população obedece
                                                  Sim, os gastos estão normalmente distri-
    aos princípios de uma distribuição
                                                  buídos.
    normal, ou assegurar que o tama-
                                                  A amostra é igual a 40 clientes, logo,
    nho da amostra seja suficientemente
                                                  maior do que 30.
    grande, ou seja, n > 30;


    2º) Identificar o valor numérico do
                                                  σ = 10
    desvio-padrão populacional (σ); da
                                                  X̅ = 173,88
    média amostral (X̅ ) e o tamanho da
                                                  n = 40
    amostra (n);


    3º) Calcular o desvio-padrão da mé-
    dia com a utilização da fórmula:
                                                                  10
                                                           σx =        = 1,58113883
                        σ                                         40
                 σx =
                        n




                                                  Se o desvio-padrão populacional é co-
                                                  nhecido e o tamanho da amostra é maior
                                                  do que 30, vamos usar a tabela normal
    4º) Determinar com auxílio da tabela
                                                  padronizada para determinar o valor de
    de distribuição normal padronizada o
                                                  Z-score.
    valor de Z-score em função do nível
    de confiança. Como vimos na Unida-
                                                  Verificamos que o nível de confiança soli-
    de 3, existem vários tipos de tabela
                                                  citado foi de 99%.
    normal padronizada e por isso temos
    que ter bastante cuidado nessa eta-
                                                  Como nossa tabela oferece os valores de
    pa. Na tabela disponibilizada em nos-
                                                  zero até z, temos que dividir esse nível de
    so curso, buscamos a metade no ní-
                                                  confiança por 2, ou seja,
    vel de confiança, no corpo da tabela,
    para determinar Z-score.
                                                  99%                    49,5%
                                                           = 49,5% =             = 0,495
                                                    2                     100




                                            235

## Página 236

      Buscamos, então, no corpo da tabela de
      distribuição Tabela
                   normal    padronizada
                          da Distribuição Normal esse valor:

                 DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
         z	
         0	
    0,01	
   0,02	
         0,03	
            0,04	
                   0,05	
   0,06	
        0,07	
          0,08	
       0,09	
  
                       0	
            0,0040	
       0,0080	
       0,0120	
       0,0160	
       0,0199	
       0,0239	
       0,0279	
       0,0319	
       0,0359	
  
         0	
  
                       0,0398	
       0,0438	
       0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
         0,1	
  
         0,2	
         0,0793	
       0,0832	
       0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

                       0,1179	
       0,1217	
       0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
         0,3	
  
                       0,1554	
       0,1591	
       0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
         0,4	
  
                       0,1915	
       0,1950	
       0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
         0,5	
  
                       0,2257	
       0,2291	
       0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
         0,6	
  
                       0,2580	
       0,2611	
       0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
         0,7	
  
                       0,2881	
       0,2910	
       0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
         0,8	
  
                       0,3159	
       0,3186	
       0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
         0,9	
  
                       0,3413	
       0,3438	
       0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
         1	
  
         1,1	
         0,3643	
       0,3665	
       0,3686	
       0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

                       0,3849	
       0,3869	
       0,3888	
       0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
         1,2	
  
                       0,4032	
       0,4049	
       0,4066	
       0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
         1,3	
  
                       0,4192	
       0,4207	
       0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
         1,4	
  
                       0,4332	
       0,4345	
       0,4357	
       0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
       0,4429	
       0,4441	
  
         1,5	
  
         1,6	
         0,4452	
       0,4463	
       0,4474	
       0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
       0,4535	
       0,4545	
  

                       0,4554	
       0,4564	
       0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
         1,7	
  
                       0,4641	
       0,4649	
       0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
         1,8	
  
                       0,4713	
       0,4719	
       0,4726	
       0,4732	
       0,4738	
       0,4744	
       0,4750	
       0,4756	
       0,4761	
       0,4767	
  
         1,9	
  
                       0,4772	
       0,4778	
       0,4783	
       0,4788	
       0,4793	
       0,4798	
       0,4803	
       0,4808	
       0,4812	
       0,4817	
  
         2	
  
         2,1	
         0,4821	
       0,4826	
       0,4830	
       0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

                       0,4861	
       0,4864	
       0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
         2,2	
  
                       0,4893	
       0,4896	
       0,4898	
       0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916	
  
         2,3	
  
         2,4	
   Fonte:
                   0,4918	
   http://ead.uva.br///recurso/DEF/EST/u3_
                              0,4920	
   0,4922	
   0,4925	
   0,4927	
   0,4929	
   0,4931	
   0,4932	
   0,4934	
   0,4936	
  

                       0,4938	
       0,4940	
       0,4941	
       0,4943	
       0,4945	
       0,4946	
       0,4948	
       0,4949	
       0,4951	
       0,4952	
  
         2,5	
  
         2,6	
   c3_r1/anexo/Tabela-da-Distribuicao-Normal.pdf
                       0,4953	
       0,4955	
  
                                          0,4960	
   0,4956	
  
                                                     0,4961	
       0,4957	
  
                                                                0,4962	
           0,4959	
  
                                                                           0,4963	
   0,4964	
  

                       0,4965	
       0,4966	
       0,4967	
       0,4968	
       0,4969	
       0,4970	
       0,4971	
       0,4972	
       0,4973	
       0,4974	
  
         2,7	
  
                       0,4974	
       0,4975	
       0,4976	
       0,4977	
       0,4977	
       0,4978	
       0,4979	
       0,4979	
       0,4980	
       0,4981	
  
         2,8	
  

      Não
       2,9	
  
       3	
  
               encontramos o valor exato 0,495. En-
                       0,4981	
  

                       0,4987	
  
                                      0,4982	
  

                                      0,4987	
  
                                                     0,4982	
  

                                                     0,4987	
  
                                                                    0,4983	
  

                                                                    0,4988	
  
                                                                                   0,4984	
  

                                                                                   0,4988	
  
                                                                                                  0,4984	
  

                                                                                                  0,4989	
  
                                                                                                                 0,4985	
  

                                                                                                                 0,4989	
  
                                                                                                                                0,4985	
  

                                                                                                                                0,4989	
  
                                                                                                                                               0,4986	
  

                                                                                                                                               0,4990	
  
                                                                                                                                                              0,4986	
  

                                                                                                                                                              0,4990	
  

      tão, vamos escolher os valores mais pró-
       3,1	
           0,4990	
  

                       0,4993	
  
                                      0,4991	
  

                                      0,4993	
  
                                                     0,4991	
  

                                                     0,4994	
  
                                                                    0,4991	
  

                                                                    0,4994	
  
                                                                                   0,4992	
  

                                                                                   0,4994	
  
                                                                                                  0,4992	
  

                                                                                                  0,4994	
  
                                                                                                                 0,4992	
  

                                                                                                                 0,4994	
  
                                                                                                                                0,4992	
  

                                                                                                                                0,4995	
  
                                                                                                                                               0,4993	
  

                                                                                                                                               0,4995	
  
                                                                                                                                                              0,4993	
  

                                                                                                                                                              0,4995	
  
       3,2	
  
      ximos pela falta e pelo excesso, que são
       3,3	
           0,4995	
       0,4995	
       0,4995	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4996	
       0,4997	
  



      0,4949 e 0,4951.
                       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4997	
       0,4998	
  
       3,4	
  
       3,5	
  
                     Tabela da Distribuição Normal
                       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
       0,4998	
  

                       0,4998	
       0,4998	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
         3,6	
  
                       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
       0,4999	
  
                                                                                                                                                 0,4999	
  
         3,7	
  
           DISTRIBUIÇÃO	
  NORMAL	
  –	
  ÁREA	
  DE	
  0	
  até	
  K	
  =>	
  	
  	
  probabilidade	
  da	
  variável	
  z	
  estar	
  entre	
  0	
  e	
  k	
  
                        0,4999	
     0,4999	
     0,4999	
     0,4999	
     0,4999	
       0,4999	
     0,4999	
     0,4999	
     0,4999	
       0,4999	
  
      z	
  3,8	
   0	
             0,01	
       0,02	
       0,03	
       0,04	
         0,05	
       0,06	
       0,07	
       0,08	
       0,09	
  
                        0,5000	
     0,5000	
     0,5000	
     0,5000	
     0,5000	
       0,5000	
     0,5000	
     0,5000	
     0,5000	
       0,5000	
  
      0	
  3,9	
   0	
             0,0040	
     0,0080	
     0,0120	
     0,0160	
       0,0199	
     0,0239	
     0,0279	
     0,0319	
     0,0359	
  

       4	
             0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  
                     0,0398	
       0,0438	
       0,0478	
       0,0517	
       0,0557	
       0,0596	
       0,0636	
       0,0675	
       0,0714	
       0,0753	
  
      0,1	
  
      0,2	
          0,0793	
       0,0832	
       0,0871	
       0,0910	
       0,0948	
       0,0987	
       0,1026	
       0,1064	
       0,1103	
       0,1141	
  

                     0,1179	
       0,1217	
       0,1255	
       0,1293	
       0,1331	
       0,1368	
       0,1406	
       0,1443	
       0,1480	
       0,1517	
  
      0,3	
  
                     0,1554	
       0,1591	
       0,1664	
       0,1664	
       0,1700	
       0,1736	
       0,1772	
       0,1808	
       0,1844	
       0,1879	
  
      0,4	
  
                     0,1915	
       0,1950	
       0,1985	
       0,2019	
       0,2054	
       0,2088	
       0,2123	
       0,2157	
       0,2190	
       0,2224	
  
      0,5	
  
                     0,2257	
       0,2291	
       0,2324	
       0,2357	
       0,2389	
       0,2422	
       0,2454	
       0,2486	
       0,2517	
       0,2549	
  
      0,6	
  
                     0,2580	
       0,2611	
       0,2642	
       0,2673	
       0,2704	
       0,2734	
       0,2764	
       0,2794	
       0,2823	
       0,2852	
  
      0,7	
  
                     0,2881	
       0,2910	
       0,2939	
       0,2967	
       0,2995	
       0,3023	
       0,3051	
       0,3078	
       0,3106	
       0,3133	
  
      0,8	
  
                     0,3159	
       0,3186	
       0,3212	
       0,3238	
       0,3264	
       0,3289	
       0,3315	
       0,3340	
       0,3365	
       0,3389	
  
      0,9	
  
                     0,3413	
       0,3438	
       0,3461	
       0,3485	
       0,3508	
       0,3531	
       0,3554	
       0,3577	
       0,3599	
       0,3621	
  
      1	
  
      1,1	
          0,3643	
       0,3665	
       0,3686	
       0,3708	
       0,3729	
       0,3749	
       0,3770	
       0,3790	
       0,3810	
       0,3830	
  

                     0,3849	
       0,3869	
       0,3888	
       0,3907	
       0,3925	
       0,3944	
       0,3962	
       0,3980	
       0,3997	
       0,4015	
  
      1,2	
  
                     0,4032	
       0,4049	
       0,4066	
       0,4082	
       0,4099	
       0,4115	
       0,4131	
       0,4147	
       0,4162	
       0,4177	
  
      1,3	
  
                     0,4192	
       0,4207	
       0,4222	
       0,4236	
       0,4251	
       0,4265	
       0,4279	
       0,4292	
       0,4306	
       0,4319	
  
      1,4	
  
                     0,4332	
       0,4345	
       0,4357	
       0,4370	
       0,4382	
       0,4394	
       0,4406	
       0,4418	
       0,4429	
       0,4441	
  
      1,5	
  
      1,6	
          0,4452	
       0,4463	
       0,4474	
       0,4484	
       0,4495	
       0,4505	
       0,4515	
       0,4525	
       0,4535	
       0,4545	
  

                     0,4554	
       0,4564	
       0,4573	
       0,4582	
       0,4591	
       0,4599	
       0,4608	
       0,4616	
       0,4625	
       0,4633	
  
      1,7	
  
                     0,4641	
       0,4649	
       0,4656	
       0,4664	
       0,4671	
       0,4678	
       0,4686	
       0,4693	
       0,4699	
       0,4706	
  
      1,8	
  
                     0,4713	
       0,4719	
       0,4726	
       0,4732	
       0,4738	
       0,4744	
       0,4750	
       0,4756	
       0,4761	
       0,4767	
  
      1,9	
  
                     0,4772	
       0,4778	
       0,4783	
       0,4788	
       0,4793	
       0,4798	
       0,4803	
       0,4808	
       0,4812	
       0,4817	
  
      2	
  
      2,1	
          0,4821	
       0,4826	
       0,4830	
       0,4834	
       0,4838	
       0,4842	
       0,4846	
       0,4850	
       0,4854	
       0,4857	
  

                     0,4861	
       0,4864	
       0,4868	
       0,4871	
       0,4875	
       0,4878	
       0,4881	
       0,4884	
       0,4887	
       0,4890	
  
      2,2	
  
                     0,4893	
       0,4896	
       0,4898	
       0,4901	
       0,4904	
       0,4906	
       0,4909	
       0,4911	
       0,4913	
       0,4916	
  
      2,3	
  
                     0,4918	
       0,4920	
       0,4922	
       0,4925	
       0,4927	
       0,4929	
       0,4931	
       0,4932	
       0,4934	
       0,4936	
  
      2,4	
  
                     0,4938	
       0,4940	
       0,4941	
       0,4943	
       0,4945	
       0,4946	
       0,4948	
       0,4949	
       0,4951	
       0,4952	
  
      2,5	
  
      2,6	
          0,4953	
       0,4955	
       0,4956	
       0,4957	
       0,4959	
       0,4960	
       0,4961	
       0,4962	
       0,4963	
       0,4964	
  

                     0,4965	
       0,4966	
       0,4967	
       0,4968	
       0,4969	
       0,4970	
       0,4971	
       0,4972	
       0,4973	
       0,4974	
  
      2,7	
  
                     0,4974	
       0,4975	
       0,4976	
       0,4977	
       0,4977	
       0,4978	
       0,4979	
       0,4979	
       0,4980	
       0,4981	
  
      2,8	
  
                     0,4981	
       0,4982	
       0,4982	
       0,4983	
       0,4984	
       0,4984	
       0,4985	
       0,4985	
       0,4986	
       0,4986	
  
      2,9	
  
                     0,4987	
       0,4987	
       0,4987	
       0,4988	
       0,4988	
       0,4989	
       0,4989	
       0,4989	
       0,4990	
       0,4990	
  
      3	
  

      Depois de identificado o valor da metade
      3,1	
  
      3,2	
  
                     0,4990	
  

                     0,4993	
  
                                    0,4991	
  

                                    0,4993	
  
                                                   0,4991	
  

                                                   0,4994	
  
                                                                  0,4991	
  

                                                                  0,4994	
  
                                                                                 0,4992	
  

                                                                                 0,4994	
  
                                                                                                0,4992	
  

                                                                                                0,4994	
  
                                                                                                               0,4992	
  

                                                                                                               0,4994	
  
                                                                                                                              0,4992	
  

                                                                                                                              0,4995	
  
                                                                                                                                             0,4993	
  

                                                                                                                                             0,4995	
  
                                                                                                                                                            0,4993	
  

                                                                                                                                                            0,4995	
  


      do nível de confiança no corpo da tabela,
      3,3	
          0,4995	
  

                     0,4997	
  
                                    0,4995	
  

                                    0,4997	
  
                                                   0,4995	
  

                                                   0,4997	
  
                                                                  0,4996	
  

                                                                  0,4997	
  
                                                                                 0,4996	
  

                                                                                 0,4997	
  
                                                                                                0,4996	
  

                                                                                                0,4997	
  
                                                                                                               0,4996	
  

                                                                                                               0,4997	
  
                                                                                                                              0,4996	
  

                                                                                                                              0,4997	
  
                                                                                                                                             0,4996	
  

                                                                                                                                             0,4997	
  
                                                                                                                                                            0,4997	
  

                                                                                                                                                            0,4998	
  
      3,4	
  
      determinamos o Z-score buscando na pri-
      3,5	
          0,4998	
  

                     0,4998	
  
                                    0,4998	
  

                                    0,4998	
  
                                                   0,4998	
  

                                                   0,4999	
  
                                                                  0,4998	
  

                                                                  0,4999	
  
                                                                                 0,4998	
  

                                                                                 0,4999	
  
                                                                                                0,4998	
  

                                                                                                0,4999	
  
                                                                                                               0,4998	
  

                                                                                                               0,4999	
  
                                                                                                                              0,4998	
  

                                                                                                                              0,4999	
  
                                                                                                                                             0,4998	
  

                                                                                                                                             0,4999	
  
                                                                                                                                                            0,4998	
  

                                                                                                                                                            0,4999	
  
      3,6	
  
      meira coluna correspondente o valor da
      3,7	
          0,4999	
  

                     0,4999	
  
                                    0,4999	
  

                                    0,4999	
  
                                                   0,4999	
  

                                                   0,4999	
  
                                                                  0,4999	
  

                                                                  0,4999	
  
                                                                                 0,4999	
  

                                                                                 0,4999	
  
                                                                                                0,4999	
  

                                                                                                0,4999	
  
                                                                                                               0,4999	
  

                                                                                                               0,4999	
  
                                                                                                                              0,4999	
  

                                                                                                                              0,4999	
  
                                                                                                                                             0,4999	
  

                                                                                                                                             0,4999	
  
                                                                                                                                                            0,4999	
  

                                                                                                                                                            0,4999	
  
      3,8	
  
      parte inteira e da primeira casa decimal
      3,9	
          0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  

      4	
            0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
       0,5000	
  




236

## Página 237

                                                  (2,5) e na primeira linha os dois valores
                                                  da segunda casa decimal e calculamos a
                                                  média desses dois valores. (0,08 e 0,07,
                                                  cuja média é 0,075):

                                                  Logo, Zscore = 2,575.




                                                               IC = X̅ ± Zscore ∙ σx

                                                      IC = 173,88 ± 2,575 ∙ 1,58113883
    5º) Substituir os valores já conheci-                   IC = 173,88 ± 4,0714
    dos na fórmula do intervalo de con-                 IC = 173,88 - 4,0714 = 169,81
    fiança:                                            IC = 173,88 + 4,0714 = 177,95


              IC = X̅ ± Zscore ∙ σx
                                                  Podemos afirmar, com 95% de confiança
                                                  que o intervalo (169,81;177,95) contém o
                                                  verdadeiro valor dos gastos médios dos
                                                  clientes.




2º caso: Intervalo de confiança para média populacional quando o desvio-padrão po-
pulacional é desconhecido.


Temos duas situações a considerar:


1ª Situação: se o desvio-padrão populacional não é conhecido, a população está nor-
malmente distribuída e estamos trabalhamos com grandes amostras, ou seja, se n > 30,
para determinar o intervalo de confiança para a média, procedemos da mesma forma
que no 1º caso.


2ª Situação: se o desvio-padrão populacional não é conhecido, a população está normal-
mente distribuída e estamos trabalhamos com pequenas amostras, ou seja, se n ≤ 30, para
determinar o intervalo de confiança para a média, devemos seguir os seguintes passos:


1º) Verificar se a população obedece aos princípios de uma distribuição normal, ou asse-
gurar que o tamanho da amostra seja suficientemente pequeno, ou seja, n ≤ 30.



                                            237

## Página 238

2º) Identificar o valor numérico do desvio-padrão amostral (s); da média amostral ( X̅ ) e
o tamanho da amostra (n);


3º) Calcular o desvio-padrão da média com a utilização da fórmula:


                                                     s
                                        Sx =
                                                     n


Em que:


Sx = desvio-padrão amostral da média
s = desvio-padrão amostral
n = tamanho da amostra

4º) Determinar, com auxílio da tabela de distribuição t-Student, o valor de t-score em
função do nível de significância. Na tabela t-Student buscamos a metade no nível de
significância, juntamente com (n-1) graus de liberdade, sendo “n” o tamanho da pequena
amostra para determinar t-score.


Vejamos aqui, uma de muitas configurações da tabela t-Student.


      t‧α          25%          10%             5%        2,5%       1%          0,5%
        2
    Grau de
   liberdade

       1          1,0000       3,0777          6,3138    12,7062   31,8207     63,6574

       2          0,8165       1,8856          2,9200    4,3027     6,9646      9,9248

       3          0,7649       1,6377          2,3534    3,1824     4,5407      5,8409

       4          0,7407       1,5332          2,1318    2,7764     3,7469      4,6041

       5          0,7267       1,4759          2,0150    2,5706     3,3649      4,0322

       6          0,7176       1,4398          1,9432    2,4469     3,1427      3,7074

       7          0,7111       1,4149          1,8946    2,3646     2,9980      3,4995

       8          0,7064       1,3968          1,8595    2,3060     2,8965      3,3554




                                               238

## Página 239

9    0,7027   1,3830   1,8331   2,2622   2,8214   3,2498

10   0,6998   1,3722   1,8125   2,2281   2,7638   3,1693

11   0,6974   1,3634   1,7959   2,2010   2,7181   3,1058

12   0,6955   1,3562   1,7823   2,1788   2,6810   3,0545

13   0,6938   1,3502   1,7709   2,1604   2,6503   3,0123

14   0,6924   1,3450   1,7613   2,1448   2,6245   2,9768

15   0,6912   1,3406   1,7531   2,1315   2,6025   2,9467

16   0,6901   1,3368   1,7459   2,1199   2,5835   2,9208

17   0,6892   1,3334   1,7396   2,1098   2,5669   2,8982

18   0,6884   1,3304   1,7341   2,1009   2,5524   2,8784

19   0,6876   1,3277   1,7291   2,0930   2,5395   2,8609

20   0,6870   1,3253   1,7247   2,0860   2,5280   2,8453

21   0,6864   1,3232   1,7207   2,0796   2,5177   2,8314

22   0,6858   1,3212   1,7171   2,0739   2,5083   2,8188

23   0,6853   1,3195   1,7139   2,0687   2,4999   2,8073

24   0,6848   1,3178   1,7109   2,0639   2,4922   2,7969

25   0,6844   1,3163   1,7081   2,0595   2,4851   2,7874

26   0,6840   1,3163   1,7056   2,0555   2,4786   2,7787

27   0,6837   1,3137   1,7033   2,0518   2,4727   2,7707

28   0,6834   1,3125   1,7011   2,0484   2,4671   2,7633

29   0,6830   1,3114   1,6991   2,0452   2,4620   2,7564

30   0,6828   1,3104   1,6973   2,0423   2,4573   2,7500




                       239

## Página 240

         Student é o pseudônimo do químico e matemático inglês William Sealy Gos-
         set (1876-1937), funcionário da cervejaria irlandesa Guinness Brewing Com-
         pany, em Dublin, no início do século XX, criador da Distribuição t.



                  Fonte: http://ead.uva.br////recurso/DEF/EST/u4_c1_r1/index.htm


Observe que, para utilizarmos essa tabela, precisamos da metade do nível de signifi-
cância e dos graus de liberdade.


Para determinarmos metade do nível de significância basta fazer 100% - Nível de Con-
fiança e dividir por 2.


Quanto aos graus de liberdade, basta diminuirmos 1 do tamanho da amostra.


5º) Substituir os valores já conhecidos na fórmula do intervalo de confiança:



                                        IC = X̅ + t score ∙ Sx




          Importante


        Chamamos de “margem de erro” o produto: Zscore ∙ σ x ou Tscore ∙ Sx .




Vejamos um exemplo que aborda o conteúdo visto em questão de concurso público.


Exemplo
Concurso: Colégio Pedro II - 2017
(Fonte: https://www.tecconcursos.com.br/questoes/720037).


A taxa de propagação de mensagens via torpedo eletrônico é uma importante característica
da velocidade de uma operadora. Suponha que a taxa de envio seja considerada uma variá-
vel aleatória com distribuição Normal. Certa operadora garante que sua taxa de transmis-
são média é de 54 mensagens por segundo. Para checar a validade da informação alegada
pela operadora, a agência controladora de telefonia decide, então, realizar um experimento.


                                                240

## Página 241

Para isso, ela coleta uma amostra com 25 mensagens e observa uma média de 52,4
mensagens por segundo e um desvio-padrão de 2,1 mensagens por segundo. O intervalo
de confiança de 95% para a taxa média e a conclusão da agência controladora foram:


a) [51,577;53,223] - não existem evidências de que a operadora esteja falando a verdade.
b) [51,53;53,27] - não existem evidências de que a operadora esteja falando a verdade.
c) [53,18;54,82] - não se pode rejeitar a afirmação da operadora.
d) [53,13;54,87] - não se pode rejeitar a afirmação da operadora.


                   Etapas                                            Respostas


    1º) Verificar se a população obedece
    aos princípios de uma distribuição            Sim, no enunciado verificamos que que a
    normal, ou assegurar que o tama-              taxa de envio é considerada uma variável
    nho da amostra seja suficientemente           aleatória com distribuição Normal.
    grande, ou seja, n > 30;



                                                  σ = desconhecido
                                                  S = 2,1

    2º) Identificar o valor numérico do           X̅ = 52,4
    desvio-padrão populacional (σ) ou             n = 25

    amostral (S), da média amostral (X̅ )
                                                  Se o desvio-padrão populacional não é
    e o tamanho da amostra (n);
                                                  conhecido e o tamanho da amostra é me-
                                                  nor do que 30, vamos usar a tabela t-S-
                                                  tudent para determinar o valor de t-score.



    3º) Calcular o desvio-padrão da mé-
    dia com a utilização da fórmula:
                                                                      2,1
                                                              Sx =          = 0,42
                        S                                             25
                 Sx =
                        n



                                                  Verificamos que o nível de confiança soli-
                                                  citado foi de 95%.
    4º) Determinar, com auxílio da tabela
    t-Student, o valor de t-score.
                                                  Logo, o nível de significância será: α =
                                                  100% - 95% = 5%




                                            241

## Página 242

      Então, a metade do nível de significância
      é igual a

                              5%
                                         = 2,5%
                                2

      Para determinarmos os graus de liberda-
      de, basta diminuir 1 no tamanho da amos-
      tra. Então: GL = 25 - 1 = 24

      Procuramos a metade do nível de signi-
      ficância na primeira linha da tabela e os
      graus de liberdade na primeira coluna da
      tabela e fazemos o cruzamento desses
      dois dados, obtendo o valor de t-score.


         t‧α        25%      10%      5%       2,5%       1%       0,5%
           2
        Grau de
       liberdade
          1        1,0000   3,0777   6,3138   12,7062   31,8207   63,6574

          2        0,8165   1,8856   2,9200   4,3027    6,9646    9,9248

          3        0,7649   1,6377   2,3534   3,1824    4,5407    5,8409

          4        0,7407   1,5332   2,1318   2,7764    3,7469    4,6041

          5        0,7267   1,4759   2,0150   2,5706    3,3649    4,0322

          6        0,7176   1,4398   1,9432   2,4469    3,1427    3,7074

          7        0,7111   1,4149   1,8946   2,3646    2,9980    3,4995

          8        0,7064   1,3968   1,8595   2,3060    2,8965    3,3554

          9        0,7027   1,3830   1,8331   2,2622    2,8214    3,2498

          10       0,6998   1,3722   1,8125   2,2281    2,7638    3,1693

          11       0,6974   1,3634   1,7959   2,2010    2,7181    3,1058

          12       0,6955   1,3562   1,7823   2,1788    2,6810    3,0545

          13       0,6938   1,3502   1,7709   2,1604    2,6503    3,0123

          14       0,6924   1,3450   1,7613   2,1448    2,6245    2,9768

          15       0,6912   1,3406   1,7531   2,1315    2,6025    2,9467

          16       0,6901   1,3368   1,7459   2,1199    2,5835    2,9208

          17       0,6892   1,3334   1,7396   2,1098    2,5669    2,8982

          18       0,6884   1,3304   1,7341   2,1009    2,5524    2,8784

          19       0,6876   1,3277   1,7291   2,0930    2,5395    2,8609

          20       0,6870   1,3253   1,7247   2,0860    2,5280    2,8453

          21       0,6864   1,3232   1,7207   2,0796    2,5177    2,8314

          22       0,6858   1,3212   1,7171   2,0739    2,5083    2,8188

          23       0,6853   1,3195   1,7139   2,0687    2,4999    2,8073

          24       0,6848   1,3178   1,7109   2,0639    2,4922    2,7969

          25       0,6844   1,3163   1,7081   2,0595    2,4851    2,7874




      Logo, tscore = 2,0639.




242

## Página 243

                                                           IC = X̅ ± tscore ∙ Sx

                                                       IC = 52,4 ± 2,0639 ∙ 0,42
                                                     IC = 52,4 - 0,866838 = 51,53
                                                    IC = 52,4 + 0,866838 = 53,27

                                              [51,53;53,27] -
5º) Substituir os valores já conheci-
dos na fórmula do intervalo de con-           Podemos afirmar, com 95% de confiança,
fiança:                                       que o intervalo (51,53;53,27) contém o
                                              verdadeiro valor da taxa média de propa-
                                              gação das mensagens por segundo.
          IC = X̅ ± tscore ∙ ∙ Sx
                                              Verificamos que esse intervalo não con-
                                              tém a taxa média de 54 mensagens envia-
                                              das por segundo que foi anunciada pela
                                              operadora. Logo, não existem evidências
                                              de que essa afirmação seja verdadeira.

                                              Portanto, a alternativa correta é a letra B.




    Importante


   O nível de confiança é a frequência com a qual esperamos que o intervalo ob-
   servado contenha o valor correto para o parâmetro de interesse quando o expe-
   rimento é repetido várias vezes.


   Logo, um nível de confiança de 90% significa que 90% dos intervalos de confian-
   ça, construídos a partir das amostras aleatórias, contêm o valor verdadeiro do
   parâmetro, ou seja, se retirarmos inúmeras amostras da população, em 90% des-
   sas amostras encontraremos o verdadeiro valor do parâmetro nesse intervalo.


   Um intervalo de confiança de 90% não significa que há a probabilidade de 90%
   do parâmetro da população pertencer ao intervalo.




                                        243

## Página 244

       Não é somente para as médias que podemos estabelecer estimativas intervalares.




Agora, vejamos o Intervalo de Confiança para proporção:


Muito utilizados, os intervalos de confiança para proporção são construídos de maneira
muito semelhante aos intervalos de confiança para médias.


As etapas continuam as mesmas, entretanto, no caso das proporções, utilizaremos sem-
pre como referência a Distribuição Normal e o cálculo do desvio-padrão das proporções,
que é dado por:



                                             p ∙ (1 – p)
                                     px =
                                                   n



Sendo, então, o intervalo de confiança:


                                     IC = p ± Zscore ∙ px


Em que:


p = proporção
n = tamanho da amostra
px = desvio-padrão da proporção

Podemos construir ainda intervalos de confiança para a variância, para o desvio-padrão
e para operações como soma, subtração e quociente de medidas estatísticas. São cons-
truídos de maneira muito similar aos que foram apresentados aqui.




                                             244

## Página 245

          Ampliando o foco


       Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca Vir-
       tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
       para reforçar o conhecimento sobre a construção de intervalos de confiança.


       Confira as respostas que estão no final do livro e, se tiver alguma dúvida, fale
       com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.



Tamanho da amostra

É importante em qualquer pesquisa determinar o tamanho da amostra para que ela seja
representativa da população.


    • Amostra pequena: corremos o risco de errar fortemente na inferência.
    • Amostra grande: caímos no desperdício de tempo e dinheiro.


Como já vimos a tabela normal, agora podemos apresentar os conceitos para o cálculo
do tamanho da amostra.


- Cálculo do tamanho da amostra para estimar a média de uma população infinita
Uma população é considerada infinita quando 5% do número de seus elementos for
maior do que 1.000. Nesse caso, se a variável for quantitativa, o tamanho da amostra é
dado por:


                                              Zscore ∙ σ
                                        n=
                                                    ε


Em que:


n = tamanho da amostra aleatória simples a ser selecionada da população em estudo.
Zscore= valor encontrado na tabela normal padronizada em função do nível de confiança.
σ = desvio-padrão populacional. Que pode ser obtido, pelo menos, de quatro maneiras:

      1. A partir de especificações técnicas.
      2. A partir de conjecturas com base em amostras-piloto.




                                              245

## Página 246

       3. Comparação com estudos semelhantes.
       4. A partir de dados históricos.


ε = erro amostral. É a máxima diferença que o pesquisador admite que ocorra entre a
média populacional (μ) e a média amostral (X). É expresso em decimais.


- Cálculo do tamanho da amostra para estimar a média de uma população finita
Uma população é considerada finita quando 5% do número de seus elementos for menor
do que 1.000. Nesse caso, se a variável for quantitativa, o tamanho da amostra é dado por:


                                          Zscore2 ∙ σ2 ∙ ∙ N
                              n=
                                     ε2 ∙ (N – 1) + Zscore2 ∙ σ2



Em que:


n = tamanho da amostra aleatória simples a ser selecionada da população em estudo.
Zscore = valor encontrado na tabela normal padronizada em função do nível de confiança.
σ = desvio-padrão populacional.
ε = erro amostral.
N = tamanho da população

- Cálculo do tamanho da amostra para estimar a proporção (p) de uma população infinita
Se a variável escolhida for qualitativa, ordinal ou nominal, e a população considerada infi-
nita, o tamanho da amostra será calculado por meio da fórmula:


                                              Z 2 ∙p̂ ∙ q̂
                                    n=
                                                   ε2



Em que:


n = tamanho da amostra aleatória simples a ser selecionada da população.
Zscore = valor encontrado na tabela normal padronizada.
ε = erro amostral. É a máxima diferença que o pesquisador admite que ocorra entre a
média populacional (μ) e a média amostral (X). É expresso em decimais.




                                             246

## Página 247

p̂ = estimativa da verdadeira proporção, expresso em decimais. Caso não se tenham es-
timativas prévias para p, admitimos, então, p̂ = 0,5.
q̂ = 1 - p̂

- Cálculo do tamanho da amostra para estimar a proporção (p) de uma população finita
Se a variável escolhida for qualitativa, ordinal ou nominal e a população considerada fini-
ta, o tamanho da amostra será calculado por meio da fórmula:


                                           Z 2 ∙ p̂ ∙ q̂ ∙ N
                               n=
                                      ε2 (N – 1) + Z 2 ∙ p̂ ∙ q̂



Em que:


n = tamanho da amostra aleatória simples a ser selecionada da população.
Zscore = valor encontrado na tabela normal padronizada .
ε = erro amostral.
p̂ = estimativa da verdadeira proporção, expresso em decimais. Caso não se tenham es-
timativas prévias para p̂, admitimos, então, p̂ = 0,5
 q̂ = 1 - p̂
N = tamanho da população.

Vejamos alguns exemplos práticos.


Exemplo 1
Determine o tamanho da amostra que devemos obter para estimar a média da altura
de todos os estudantes da Universidade Veiga de Almeida, com 95% de confiança e a
tolerância de 2cm para mais ou para menos. Sabe-se, por estudos anteriores, que o des-
vio-padrão da altura dos estudantes é igual a 20cm.


Resolução:
Sabemos que:
n=?
Zscore – buscando no corpo da tabela a metade do nível de confiança, ou seja, 0,95/2 =
0,475.




                                             247

## Página 248

Obtemos:
Zscore= 1,96
σ =20 cm
ε = 2cm

Logo, substituindo na fórmula do cálculo do tamanho da amostra para estimar a média
de uma população infinita, temos:

                                            2                       2
                              Zscore ∙ σ            1,96 ∙ 20
                     n=                         =                       = 385 alunos
                                    ε                    2



Exemplo 2
Uma empresa administradora de cartões de crédito deseja estimar a proporção de clien-
tes que não fizeram o pagamento integral da fatura no final de um mês. Suponha que a
margem de erro desejada seja de três pontos percentuais, com 98% de confiança.


Determine o tamanho da amostra: deve ser selecionado para estimar essa proporção?


Resolução:
Sabemos que:
n=?
Zscore = buscando no corpo da tabela a metade do nível de confiança, ou seja, 0,98/2 =
0,49,


obtemos, Zscore ≅ 2,33
ε = 3% = 0,03
p̂ = vamos admitir p̂ =0,5, já que não temos estimativas prévias dessa proporção.
q̂ = 1 - p̂ = 1 – 0,5 = 0,5

Logo, substituindo na fórmula do cálculo do tamanho da amostra para estimar a propor-
ção (p) de uma população infinita, temos:


                     Z2 ∙ p̂ ∙ q̂               2,332 ∙ 0,5 ∙ 0,5
               n=                       =                                 ≅ 1509 clientes
                         ε2                         (0,03)2




                                                     248

## Página 249

Correlação

É um termo estatístico que indica a relação entre duas ou mais variáveis. Um dos propó-
sitos dos pesquisadores é verificar a associação entre as variáveis estudadas. A possível
existência de um relacionamento entre variáveis é fundamental para análises e conclu-
sões do estudo que está sendo realizado. Intuitivamente, muitas vezes consideramos
que há uma relação de associação entre o comportamento de variáveis, como:


                           Tempo de estudo x nota na avaliação.


                         Investimento em publicidade x vendas.


                     Manutenção preventiva x manutenção corretiva.



Neste tópico vamos apresentar um indicador que nos revela o quão forte ou fraca é essa
associação entre as variáveis, chamado de coeficiente de correlação do produto de
momentos de Pearson ou simplesmente coeficiente de correlação de Pearson. Além
de representar graficamente o relacionamento entre duas variáveis, por meio do diagra-
ma de dispersão, para visualmente inferirmos sobre a correlação existente.


Coeficiente de correlação de Pearson

Representado pela letra “r” tendo como uma das suas fórmulas:

                                    n ∙ ∑(xi ∙ y i) – (∑xi) ∙ (∑y i)
                      r=
                              [n ∙ ∑xi2 – (∑xi)2 ] ∙ [n ∙ ∑y i2 – (∑y i)2 ]


Em que:

n = tamanho da amostra.
∑(xi ∙ y i) = somatório do produto das variáveis.
∑xi = somatório dos valores da variável independente x.
∑y i = somatório dos valores da variável dependente y.
(∑x i)2 = quadrado do somatório dos valores da variável independente x.
(∑yi)2 = quadrado do somatório dos valores da variável dependente y.




                                               249

## Página 250

∑x i2 = somatório dos quadrados dos valores da variável independente x.
∑yi2 = somatório dos quadrados dos valores da variável independente y.


Considerações sobre o coeficiente de correlação de Pearson

    • O valor do coeficiente de correlação de Pearson pertence ao intervalo [-1;1].
    • O coeficiente de correlação de Pearson é adimensional, ou seja, não tem unidade
    de medida.
    • Nos casos em que o coeficiente de correlação linear for igual a zero não indica
    ausência de correlação e sim ausência de correlação linear, mas as variáveis podem
    estar correlacionadas exponencialmente, polinomialmente ou logaritmicamente.


Como interpretar o coeficiente de correlação de Pearson:

- Observando o sinal de “r”, temos:

    • Se r > 0, correlação positiva, indicando uma reta crescente, isto é, à medida que os
    valores de uma variável aumentam, os valores da outra também aumentam.
    • Se r < 0, correlação negativa, indicando uma reta decrescente, isto é, à medida que
    os valores de uma variável aumentam, os valores da outra variável diminuem.


- Observando o valor de “r”, temos:


    • 0 ≤ |r| < 0,3 → a correlação entre as variáveis é muito fraca.
    • 0,3 < |r|< 0,6 → a correlação entre as variáveis é média.
    • 0,6 < |r|≤ 1 → a correlação entre as variáveis é forte.


         Importante


       É importante que saibamos que a existência de correlação não necessariamen-
       te implica uma relação de causalidade. São coisas distintas, sendo responsabi-
       lidade do coeficiente de correlação apenas sinalizar o quanto associados estão
       os comportamentos de duas variáveis.




                                            250

## Página 251

Vejamos:


Exemplo 1
horas de estudo X notas


Queremos verificar o quanto estão associadas as variáveis notas das avaliações em Es-
tatística e o tempo de estudo em horas dos alunos. Para isso, coletamos as informações
com uma amostra composta de 22 alunos, que estão apresentadas na tabela a seguir:


                     Aluno         Horas de Estudo         Notas

                       1                 7,5                8,5

                       2                  2                  1

                       3                  9                  5

                       4                 1,5                 3

                       5                  6                  7

                       6                 8,5                 9

                       7                  3                  2

                       8                 4,5                 5

                       9                 3,5                 4

                      10                 10                 9,5

                      11                  8                  9

                      12                  4                  5

                      13                 5,5                 7

                      14                 6,5                 7

                      15                 5,5                5,5




                                         251

## Página 252

                        16                   1             2

                        17                   3             2

                        18                   5             8

                        19                   2             1

                        20                   4             3

                        21                   5             6

                        22                   7             8




Resolução


Vamos definir:
X = número de horas de estudo.
Y= rendimento na avaliação.


Devemos, então, calcular os somatórios presentes na fórmula do coeficiente de cor-
relação linear de Pearson:


                Horas de
    Aluno                        Notas (Y)         X.Y         X²         Y²
               Estudo (X)

      1            7,5              8,5            63,75   56,25        72,25

      2             2               2               4          4          4

      3             9               9               81         81        81

      4            1,5              2               3      2,25           4

      5             6               7               42         36        49

      6            8,5              8               68     72,25         64

      7             3               3               9          9          9




                                             252

## Página 253

   8        4,5              5                22,5            20,25      25

   9        3,5              4                 14             12,25      16

   10       10              9,5                95              100      90,25

   11       8                9                 72                  64    81

   12       4                5                 20                  16    25

   13       5,5              5                27,5            30,25      25

   14       6,5              7                45,5            42,25      49

   15       5,5              6                 33             30,25      36

   16       1                1                  1                  1      1

   17       3                2                  6                  9      4

   18       5                7                 35                  25    49

   19       2                1                  2                  4      1

   20       4                3                 12                  16     9

   21       5               5,5               27,5                 25   30,25

   22       7                8                 56                  49    64

Total (∑)   112           117,5             739,75             705      788,75




                           n ∙ ∑(xi ∙ y i) – (∑xi) ∙ (∑y i)
             r=
                    [n ∙ ∑xi2 – (∑xi)2 ] ∙ [n ∙ ∑y i2 – (∑y i)2]


                           22 ∙ 739,75 – 112 ∙ 117,5
            r=
                  [22 ∙ 705 – 1122 ] ∙ [22 ∙ 788,75 – 117,52]


                                 16274,50 – 13160
                      r=
                                  2966 ∙ 3546,25


                                     253

## Página 254

                                           3114,5
                                r=
                                        10518177,5


                                           3114,5
                                  r=
                                       3243,173985


                                       r ≅ 0,96


Exemplo 2
massa muscular X idade


Acredita-se que, com o passar dos anos, as mulheres vão perdendo massa muscular sig-
nificativamente. Para avaliar essa hipótese foi feita uma pesquisa em uma amostra de 30
pacientes do sexo feminino, quando foi observada a massa muscular e a idade. O cálculo
da massa muscular pode ser feito pela avaliação das dobras cutâneas e também pela
fórmula da área muscular do braço. O resultado está apresentado na tabela seguinte.


Calcule o coeficiente de correlação de Pearson e verifique se realmente o índice de
massa muscular está correlacionado com a idade da paciente.


                   Paciente             Idade          Massa muscular

                       1                  45                 97

                       2                  56                 90

                       3                  65                 74

                       4                  58                 80

                       5                  49                 105

                       6                  62                 68

                       7                  51                 83

                       8                  53                 79

                       9                  71                 68




                                          254

## Página 255

10   69    73

11   58    95

12   73    63

13   53    100

14   45    116

15   76    65

16   69    68

17   74    65

18   67    65

19   56    82

20   63    92

21   44    102

22   72    65

23   78    77

24   68    78

25   67    88

26   39    105

27   51    102

28   64    71

29   60    87

30   40    119




     255

## Página 256

Resolução


Vamos definir:
X = idade da paciente.
Y= massa muscular.


Devemos, então, calcular os somatórios presentes na fórmula do coeficiente de correla-
ção linear de Pearson, como mostra a tabela a seguir.


                                Massa
   Paciente        Idade                        x. y           x²            y²
                               muscular

      1             45            97            4365          2025          9409

      2             56            90            5040          3136          8100

      3             65            74            4810          4225          5476

      4             58            80            4640          3364          6400

      5             49            105           5145          2401         11025

      6             62            68            4216          3844          4624

      7             51            83            4233          2601          6889

      8             53            79            4187          2809          6241

      9             71            68            4828          5041          4624

      10             69           73            5037          4761          5329

      11             58           95            5510          3364          9025

      12             73           63            4599          5329          3969

      13             53           100           5300          2809         10000

      14             45           116           5220          2025         13456

      15             76           65            4940          5776          4225




                                          256

## Página 257

  16             69            68               4692               4761   4624

  17             74            65               4810               5476   4225

  18             67            65               4355               4489   4225

  19             56            82               4592               3136   6724

  20             63            92               5796               3969   8464

  21             44            102              4488               1936   10404

  22             72            65               4680               5184   4225

  23             78            77               6006               6084   5929

  24             68            78               5304               4624   6084

  25             67            88               5896               4489   7744

  26             39            105              4095               1521   11025

  27             51            102              5202               2601   10404

  28             64            71               4544               4096   5041

  29             60            87               5220               3600   7569

  30             40            119              4760               1600   14161

Total (∑)    1796              2522            146510            111076   219640




                               n ∙ ∑(xi ∙ y i)–(∑xi) ∙ (∑y i)
                  r=
                        [n ∙ ∑xi2 – (∑xi)2 ] ∙ [n ∙ ∑y i2 – (∑y i)2]


                              30 ∙ 146510 – 2522 ∙ 1796
            r=
                  [30 ∙ 111076 – (1796)2 ] ∙ [30 ∙ 219640 – (2522)2]


                                        –134212
                          r=
                                    106664 ∙ 228716


                                        257

## Página 258

                                          –134212
                                 r=
                                        24395763424


                                       r ≅ –0,86



Como o valor de “r” foi maior do que 0,6 podemos concluir que há forte correlação li-
near entre as variáveis massa muscular e idade. Ainda observando o sinal negativo de
“r”, podemos dizer que à medida que a idade da paciente aumenta, sua massa muscu-
lar diminui.


Exemplo 3


Pesquisas sugerem que o tamanho do cérebro de uma pessoa pode estar relacionado
com seu grau de inteligência. Para investigar essa hipótese foi feita uma pesquisa com
uma amostra aleatória de 40 alunos de um curso autodidata de matemática. A tabela
a seguir mostra o tamanho do cérebro em cm³, obtido por ressonância magnética, que
será representada pela variável independente (x) e seu respectivo escore de Quociente de
Inteligência (QI), que será representado pela variável dependente (y).



Determine o coeficiente de correlação linear de Pearson e verifique se há alguma
evidência que possa corroborar essa hipótese. Observe a tabela a seguir.


                                    Volume cerebral
                  Observação                             Escores de QI
                                        (cm³)

                       1                 1270                131

                       2                 1341                138

                       3                 1265                137

                       4                 1233                131

                       5                 1310                135

                       6                 1242                 97

                       7                 1251                136




                                          258

## Página 259

8    1337   90

9    1302   87

10   1347   131

11   1238   130

12   1339   139

13   1346   133

14   1344   138

15   1228   94

16   1340   81

17   1278   130

18   1303   98

19   1336   99

20   1326   78

21   1200   81

22   1329   95

23   1282   133

24   1348   137

25   1347   89

26   1267   139

27   1237   83

28   1318   101




     259

## Página 260

                        29                  1238            75

                        30                  1329            128

                        31                  1240            131

                        32                  1308            142

                        33                  1320            101

                        34                  1306            88

                        35                  1341            81

                        36                  1280            131

                        37                  1348            138

                        38                  1241            86

                        39                  1266            79

                        40                  1299            87




Resolução


Vamos definir:
X = volume cerebral.
Y = Coeficiente de inteligência.


Devemos, então, calcular os somatórios presentes na fórmula do coeficiente de correla-
ção linear de Pearson, como mostra a tabela a seguir.


                   volume
 Observação                        QI (Y)           X.Y          X²           Y²
                 cerebral (X)

       1            1270            131            166370   1612900         17161

       2            1341            138            185058   1798281         19044




                                            260

## Página 261

3    1265   137         173305   1600225   18769

4    1233   131         161523   1520289   17161

5    1310   135         176850   1716100   18225

6    1242   97          120474   1542564   9409

7    1251   136         170136   1565001   18496

8    1337   90          120330   1787569   8100

9    1302   87          113274   1695204   7569

10   1347   131         176457   1814409   17161

11   1238   130         160940   1532644   16900

12   1339   139         186121   1792921   19321

13   1346   133         179018   1811716   17689

14   1344   138         185472   1806336   19044

15   1228   94          115432   1507984   8836

16   1340   81          108540   1795600   6561

17   1278   130         166140   1633284   16900

18   1303   98          127694   1697809   9604

19   1336   99          132264   1784896   9801

20   1326   78          103428   1758276   6084

21   1200   81          97200    1440000   6561

22   1329   95          126255   1766241   9025

23   1282   133         170506   1643524   17689




                  261

## Página 262

  24         1348            137             184676             1817104   18769

  25         1347             89             119883             1814409   7921

  26         1267            139             176113             1605289   19321

  27         1237             83             102671             1530169   6889

  28         1318            101             133118             1737124   10201

  29         1238             75             92850              1532644   5625

  30         1329            128             170112             1766241   16384

  31         1240            131             162440             1537600   17161

  32         1308            142             185736             1710864   20164

  33         1320            101             133320             1742400   10201

  34         1306             88             114928             1705636   7744

  35         1341             81             108621             1798281   6561

  36         1280            131             167680             1638400   17161

  37         1348            138             186024             1817104   19044

  38         1241             86             106726             1540081   7396

  39         1266             79             100014             1602756   6241

  40         1299             87             113013             1687401   7569

Total (∑)    51820          4458             5780712           67207276   519462



                             n ∙ ∑(x i ∙ yi)–(∑x i) ∙ (∑yi)
                 r=
                       [n ∙ ∑x i2 – (∑x i)2 ] ∙ [n ∙ ∑yi2 – (∑yi)2]


                           40 ∙ 5780712 – 51820 ∙ 4458
            r=
                  [40 ∙ 67207276-518202 ] ∙ [40 ∙ 519462-44582 ]



                                       262

## Página 263

                                             214920
                               r=
                                         2978640 ∙ 904716


                                             214920
                                  r=
                                         2694823266240


                                             214920
                                    r=
                                           1641591,687


                                          r = 0,13

Observamos que existe uma fraca correlação entre o volume cerebral e a inteligência,
pois o valor de r < 0,3.


        Ampliando o foco



       Philipp Koellinger destaca que, além da questão do tamanho, que conta apenas
       2% para a inteligência, existirão vários fatores que explicam os outros 98% na
       variação de desempenho.


       Com a colaboração de três investigadores, Gideon e Philipp incluíram também
       outras variáveis no estudo, como sexo, idade, altura, estatuto socioeconômico.
       A altura, por exemplo, está associada a melhores resultados nos testes, mas
       também a um cérebro maior.


       A correlação estudada pelos investigadores já tem vindo a ser investigada há
       algum tempo, mas com amostras pequenas. Ao incluir mais participantes, am-
       bos esperavam ir além dos estudos já feitos. Dessa forma, recorreram aos da-
       dos do Biobank, um banco de dados que contém informações de mais de meio
       milhão de pessoas do Reino Unido.


       As conclusões acabam por ir ao encontro do senso comum: se a pessoa tem
       mais neurônios, isso permite-lhe ter uma memória melhor, ou realizar mais ta-
       refas em simultâneo.


       Existem, no entanto, muitos outros fatores associados, pelo que os investigado-
       res reconhecem que são necessários mais estudos. Relativamente ao sexo, por




                                             263

## Página 264

             exemplo, parece existir uma diferença significativa no tamanho do cérebro en-
             tre homens e mulheres, mas isso não se traduz em diferenças no desempenho.

             Fonte: https://www.dn.pt/vida-e-futuro/tamanho-do-cerebro-conta-pouco-para-a-in-
             teligencia-10267115.html




Cálculo do coeficiente de correlação de Pearson com auxílio da planilha
eletrônica Excel


=PEARSON(INÍCIO DA VARIÁVEL INDEPENDENTE : FIM DA VARIÁVEL INDEPENDENTE;
INÍCIO DA VARIÁVEL DEPENDENTE : FIM DA VARIÁVEL DEPENDENTE)


Gráfico de Dispersão (Scatter Plot)

Podemos verificar visualmente o relacionamento entre duas variáveis a partir dos gráfi-
cos de dispersão, também conhecidos como Scatter Plot.


Para traçar um gráfico de dispersão, vamos marcar, no eixo das abscissas, as horas de
estudo e, no eixo das ordenadas, a nota na avaliação. Marcamos um ponto cartesiano
para cada aluno, relacionando suas horas de estudo com seu rendimento na avaliação.


Vejamos alguns tipos de correlação a seguir.


                 Correlação linear positiva                                                  Correlação linear negativa


          Dispersão "Horas de Estudo" x "Nota"                                               Dispersão "demanda" x "preço"
   10
                                                                                    25
    9

    8                                                                               20
    7

    6                                                                               15

    5

    4                                                                               10

    3
                                                                                    5
    2

    1
                                                                                    0
    0                                                                                    0      5      10      15      20    25
        0 0,5 1 1,5 2 2,5 3 3,5 4 4,5 5 5,5 6 6,5 7 7,5 8 8,5 9 9,5 10 10,5




                                                                              264

## Página 265

   Podemos verificar que os dados apresen-
                                                               Podemos verificar que os dados apresen-
   tam uma tendência de alinhamento em
                                                               tam uma tendência de alinhamento em
   torno de uma reta crescente, indicando
                                                               torno de uma reta decrescente, indicando
   que, à medida que o valor de uma variável
                                                               que, à medida que o valor de uma variável
   aumenta, o valor da outra variável tam-
                                                               aumenta, o valor da outra variável diminui.
   bém aumenta.




                 Correlação não linear                                         Não há correlação


    Dispersão "unidade" x "grau de compactação"                Dispersão "estatura" x "sucesso profissional"
                                                               12

  16                                                           10

 12,8                                                           8

  9,6                                                           6


  6,4                                                           4


  3,2                                                           2

                                                                0
                                                                 1,45   1,55      1,65   1,75   1,85   1,95
        0   5   10   15   20   25   30    35   40   45




   Não podemos observar uma tendência
   de alinhamento em torno de uma reta,                        Não podemos observar nenhuma ten-
   mas observamos um alinhamento em                            dência de alinhamento entre os dados,
   torno de uma parábola, o que indica que a                   sugerindo não haver correlação entre as
   correlação entre as variáveis não é linear                  variáveis.
   e sim parabólica.




                                         Fonte: Elaborado pela autora (2020).




Agora, usando os exemplos anteriores, traçaremos os gráficos de dispersão com base
no estudo de correlação.




                                                         265

## Página 266

Exemplo 1: horas de estudo x notas


                                      Horas de
                   Aluno                          Notas
                                     Estudo (X)

                     1                  7,5        8,5

                     2                   2         2

                     3                   9         9

                     4                  1,5        2

                     5                   6         7

                     6                  8,5        8

                     7                   3         3

                     8                  4,5        5

                     9                  3,5        4

                     10                 10         9,5

                     11                  8         9

                     12                  4         5

                     13                 5,5        5

                     14                 6,5        7

                     15                 5,5        6

                     16                  1         1

                     17                  3         2

                     18                  5         7

                     19                  2         1

                     20                  4         3

                     21                  5         5,5

                     22                  7         8




                                        266

## Página 267

                              Dispersão "Horas de Estudo" x "Nota"
              10

               9

               8

               7

               6

               5

               4

               3

               2

               1

               0
                   0 0,5 1 1,5 2 2,5 3 3,5 4 4,5 5 5,5 6 6,5 7 7,5 8 8,5 9 9,5 10 10,5



Analisando o comportamento do gráfico podemos verificar que os pontos estão prati-
camente alinhados em uma reta crescente, indicando que, à medida que uma variável
aumenta, a outra também aumenta.


Exemplo 2: idade x massa muscular


                                                                      Massa
                     Paciente                  Idade
                                                                     muscular

                          1                      45                      97

                          2                      56                      90

                          3                      65                      74

                          4                      58                      80

                          5                      49                     105

                          6                      62                      68

                          7                      51                      83

                          8                      53                      79

                          9                      71                      68

                         10                      69                      73

                         11                      58                      95




                                                 267

## Página 268

12                          73                63

13                          53                100

14                          45                116

15                          76                65

16                          69                68

17                          74                65

18                          67                65

19                          56                82

20                          63                92

21                          44                102

22                          72                65

23                          78                77

24                          68                78

25                          67                88

26                          39                105

27                          51                102

28                          64                71

29                          60                87

30                          40                119


          Dispersão Idade x Massa Muscular
90

80

70
60

50

40
30
20

10

0
     40        60      80         100   120   140



                            268

## Página 269

Analisando o comportamento do gráfico podemos verificar que os pontos estão alinha-
dos em uma reta decrescente, o que indica que, `s medida que uma variável aumenta, a
outra irá diminuir.


Exemplo 3: volume cerebral x QI


                                  Volume cerebral
                 Observação                           Escores de QI
                                     (cm³) (X)

                      1                1270                131

                      2                1341                138

                      3                1265                137

                      4                1233                131

                      5                1310                135

                      6                1242                97

                      7                1251                136

                      8                1337                90

                      9                1302                87

                      10               1347                131

                      11               1238                130

                      12               1339                139

                      13               1346                133

                      14               1344                138

                      15               1228                94

                      16               1340                81

                      17               1278                130

                      18               1303                98

                      19               1336                99

                      20               1326                78

                      21               1200                81

                      22               1329                95

                      23               1282                133



                                        269

## Página 270

                        24               1348                 137

                        25               1347                  89

                        26               1267                 139

                        27               1237                  83

                        28               1318                 101

                        29               1238                  75

                        30               1329                 128

                        31               1240                 131

                        32               1308                 142

                        33               1320                 101

                        34               1306                  88

                        35               1341                  81

                        36               1280                 131

                        37               1348                 138

                        38               1241                  86

                        39               1266                  79

                        40               1299                  87



                             Dispersão volume cerebral x QI
                  160
                  140
                  120
                  100
                   80
                   60
                   40
                   20
                    0
                    1180 1200 1220 1240 1260 1280 1300 1320 1340 1360




Analisando o comportamento do gráfico podemos verificar que os pontos não estão ali-
nhados, o que indica que as variáveis não estão correlacionadas linearmente.


                                          270

## Página 271

         Ampliando o foco


       Para construir um gráfico de dispersão com auxílio da planilha eletrônica Ex-
       cel, você deve selecionar os valores da variável independente (x) e da variável
       dependente (y), nessa ordem, e simultaneamente ir em Inserir – Gráfico – Dis-
       persão X Y.


       Verifique na midiateca desta unidade um tutorial para construir um gráfico de
       dispersão com auxílio da planilha eletrônica Excel, adicionando a linha de ten-
       dência e o quadrado do coeficiente de correlação de Pearson.




Verificação da Normalidade dos Dados

Encontramos nos softwares estatísticos testes que verificam a hipótese de normalida-
de dos dados, como os testes de Kolmogorov-Sminorv, Shapiro-Wilk, Anderson-Darling,
Ryan-Joiner, entre outros. Entretanto, vamos apresentar aqui três métodos simples que
também nos auxiliam na verificação da normalidade dos dados. São eles:


    1) Análise do histograma: basta examinar a forma da distribuição, que deve ser
    semelhante à forma de um sino para que a normalidade pareça razoável.


    2) Regra empírica ao contrário: com a média e o desvio-padrão, calcule a propor-
    ção real das observações em cada intervalo e compare com os percentuais:


                                            ∑fi
                     [X̅ – S ; X̅ + S] →          ≅ 0 ,68
                                            n
                                            ∑fi
              [X̅ – 2 ∙ S ; X̅ + 2 ∙ S] →         ≅ 0,95     para amostras
                                            n
                                            ∑fi
              [X̅ – 3 ∙ S ; X̅ + 3 ∙ S] →         ≅ 0,997
                                            n
                        ou
                                            ∑fi
                     [μ – σ ; μ + σ] →            ≅ 0,68
                                            n
                                            ∑fi
             [μ – 2 ∙ σ ; μ + 2 ∙ σ] →            ≅ 0,95    para população
                                            n
                                            ∑fi
             [μ – 3 ∙ σ ; μ + 3 ∙ σ] →            ≅ 0,997
                                            n




                                                  271

## Página 272

Se os resultados estiverem próximos desses valores, então a normalidade parece razoável.


      3) Gráfico de Probabilidade Normal: diagrama de dispersão de cada observação
      versus seu escore normal padronizado correspondente. Se os pontos ficarem ali-
      nhados sobre uma reta, então a normalidade parece razoável.


Vejamos um exemplo prático com base nos conceitos apresentados.


Exemplo:


Feita uma pesquisa sobre os gastos mensais com transporte com 20 alunos da Universi-
dade Veiga de Almeida, as respostas já ordenadas estão apresentadas a seguir. Verifique
se os dados obtidos obedecem a uma distribuição normal.


 23   29   30   31   35   39   41    42   42     46    47      47   49   50   50   51   56   56   64   67

 5     5   0    0    6    9    6     1       7   5         6   7    8    3    9    9    0    8    5    0



Resolução:


1) Análise do histograma:
Vamos agrupar os dados em uma tabela de distribuição de frequências:


                                   Gastos R$                   Frequência


                                200 I--- 300                        2
                                300 I--- 400                        4
                                400 I--- 500                        7
                                500 I--- 600                        5
                                600 I--- 700                        2


                                     Total                          20




                                                     272

## Página 273

                                        Histograma
                  8
                  7
                  6
                  5
                  4
                  3
                  2
                  1
                  0
                      200      300        400       500     600     700


                            Fonte: Elaborado pela autora (2020).


Observando o histograma podemos verificar que se aproxima da forma de um sino.


                                        Histograma
                  8
                  7
                  6
                  5
                  4
                  3
                  2
                  1
                  0
                      200      300        400       500     600     700



                            Fonte: Elaborado pela autora (2020).




Pelo formato do histograma parece razoável dizer que os dados seguem uma distribui-
ção normal.


2) Regra empírica ao contrário: com a média e o desvio-padrão, calcule as diferenças:
Para a amostra apresentada, podemos calcular o valor da média e do desvio-padrão
amostral, obtendo como resultados: X̅ = 452 e S = 116. Logo,


                                                    ∑fi
                              [X̅ – S ; X̅ + S] →         ≅ 0 ,68
                                                    n




                                             273

## Página 274

                                                         ∑fi
                         [X̅ – 2 ∙ S ; X̅ + 2 ∙ S] →             ≅ 0,95
                                                          n
                                                         ∑fi
                         [X̅ – 3 ∙ S ; X̅ + 3 ∙ S] →             ≅ 0,997
                                                          n


                                                                       14
                 [452 – 116 ; 452 + 116] = [336;568] →                        = 0,7 ≅ 0,68
                                                                       20
                                                                       20
             [452-2 ∙116 ; 452 + 2 ∙ 116] = [220;684] →                       = 1 ≅ 0,95
                                                                       20
                                                                       20
            [452 – 3 ∙ 116 ; 452 + 3 ∙ 116] = [104;800] →                     = 1≅ 0,997
                                                                       20



Como os quocientes estão próximos dos valores desejáveis, podemos dizer que parece
razoável admitir a normalidade dos dados.


3) Gráfico de Probabilidade Normal:


                            Observação                 Zscore = (X – X̅ )/σ
                             (eixo-y)                       (eixo-x)

                                 235                           -1,89

                                 295                           -1,37

                                 300                           -1,32

                                 310                           -1,23

                                 356                           -0,83

                                 399                           -0,46

                                 416                           -0,31

                                 421                           -0,27

                                 427                           -0,22

                                 465                           0,11




                                                274

## Página 275

                                   476                             0,21

                                   477                             0,22

                                   498                             0,40

                                   503                             0,44

                                   509                             0,50

                                   519                             0,58

                                   560                             0,94

                                   568                             1,01

                                   645                             1,68

                                   670                             1,90


                               Dispersão Z-score X observação
                                             800
                                             700
                                             600
                                             500
                                             400
                                             300
                                             200
                                             100
                                                 0
               -2,50 -2,00 -1,50   -1,00 -0,50       0,00   0,50   1,00   1,50   2,00   2,50



                              Fonte: Elaborado pela autora (2020).


Observando o gráfico concluímos que não há evidência de não normalidade.


Podemos concluir que todos os três métodos nos mostram que não há evidências para
rejeitar a normalidade. Isso não significa que podemos dizer com certeza que os dados
sejam provenientes de uma distribuição normal.




                                                     275

## Página 276

 Ampliando o foco


Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca Vir-
tual, escolha um livro de Estatística, selecione e faça alguns exercícios do livro
para reforçar o conhecimento sobre o cálculo do coeficiente de correlação li-
near de Pearson.


Confira as respostas que estão no final do livro e se tiver alguma dúvida fale
com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                       276

## Página 277

Regressão linear

A análise de regressão é uma ferramenta estatística usada para prever os valores fu-
turos com base no comportamento de valores passados. Para isso, vamos construir
um modelo matemático capaz de descrever o relacionamento entre duas ou mais
variáveis quantitativas, de maneira que possamos estimar o valor de uma variável a
partir das outras.


Podemos aplicar os modelos de regressão em diversas áreas: engenharia, administra-
ção, economia e finanças, marketing, ciências físicas, biologia, ciência ambiental, geogra-
fia, saúde pública, nutrição, entre muitas outras áreas de estudo e pesquisa.


Variável independente (X) e variável dependente (Y)

Um conceito importante na análise de regressão é o de variável dependente ou variável
independente.




          A variável dependente é aquela por meio da qual desejamos realizar a previsão
          e que, portanto, sofre influência do comportamento da variável independente.




Regressão Linear Simples

O caso mais simples de regressão é quando temos duas variáveis e a relação entre elas
pode ser representada por uma linha reta. Essa reta é chamada de modelo de Regres-
são Linear Simples, cujo modelo matemático será representado por:


                                         y = ax + b + εi



Em que:


y = variável dependente, ou seja, o valor que queremos estimar.
x = variável independente, ou seja, aquela que influencia a previsão da variável y.
a = coeficiente angular da reta de regressão.



                                               277

## Página 278

b = também chamado de intercepto com o eixo-y, é o coeficiente linear da reta.
εi = erro aleatório de “y” para a observação i. É uma variável aleatória com valor esperado
igual a zero.



          Importante


        Antes de determinar os coeficientes para o modelo de regressão linear, é mui-
        to conveniente traçar o diagrama de dispersão para verificar visualmente se o
        relacionamento entre as variáveis aproxima-se de uma reta e também fazer o
        cálculo do coeficiente de correlação de Pearson para garantir que as variáveis
        estão correlacionadas e poderão ser ajustadas por um modelo linear.




Determinação dos coeficientes “a” e “b”:

Para construirmos o modelo de regressão linear devemos determinar os valores dos
coeficientes “a” e “b” de modo que a reta ajuste-se ao conjunto de pontos visualizados
no gráfico de dispersão.


O método dos mínimos quadrados utiliza dados amostrais para determinar os valores de
“a” e “b”, que minimizam a soma dos quadrados dos desvios entre os valores observados
e os valores estimados da variável dependente.


Usando os conceitos do cálculo diferencial pode-se demonstrar que:




                 n ∙ ∑(x i ∙ yi) – (∑x i) ∙ (∑yi)
           a=                                        b = y̅ - a ∙ x̅   εi = vamos admitir = 0
                       n ∙ ∑x i2 – (∑x i)2

          Logo, substituindo-se os valores de “a” e “b” e “εi “ vamos obter o modelo de
          regressão linear simples: y = ax + b




Em que:


n = tamanho da amostra.
∑(x i ∙ yi) = somatório do produto das variáveis.



                                                    278

## Página 279

∑x i= somatório dos valores da variável independente x.
∑yi = somatório dos valores da variável dependente y.
(∑x i)2 = quadrado do somatório dos valores da variável independente x.
∑x i2 = somatório dos quadrados dos valores da variável independente x.
y̅ = média dos valores da variável dependente.
x̅ = média dos valores da variável independente.




        Uma forma alternativa para o coeficiente angular “a”, temos:


                                       ∑(x i – x̅ ) ∙ (∑yi – y̅ )
                              a=
                                             ∑(x i – x̅ )2




Como verificar se um modelo de Regressão Linear está adequado?

Algumas ferramentas estatísticas nos auxiliam na verificação da adequação de um mo-
delo de regressão linear, como o gráfico dos resíduos e o coeficiente de determinação
(r²), que nos indica o percentual de eficácia do modelo. Quanto mais próximo de 1 (100%)
mais bem ajustado poderá estar o modelo. Para maior confiança, é preciso analisar o
valor de r² juntamente com o gráfico de resíduos.


Determinação do modelo de regressão linear com auxílio do Excel

Para determinar rapidamente o modelo de regressão linear com auxílio da planilha ele-
trônica Excel, basta clicar nos pontos do gráfico de dispersão; com o botão direito do
mouse clicar em “adicionar linha de tendência” e no menu “formatar linha de tendência”
selecionar “exibir equação no gráfico”.


Vejamos alguns exemplos práticos a seguir.


Exemplo 1


Voltemos ao Exemplo 1 usado no Tópico 2: horas de estudo X notas.


Queremos verificar o quanto estão associadas as variáveis notas das avaliações em Es-
tatística e o tempo de estudo em horas dos alunos. Para isso, coletamos as informações
com uma amostra composta de 22 alunos, que estão apresentadas na tabela a seguir:



                                            279

## Página 280

                     Aluno         Horas de Estudo         Notas

                       1                 7,5                8,5

                       2                  2                  2

                       3                  9                  9

                       4                 1,5                 2

                       5                  6                  7

                       6                 8,5                 8

                       7                  3                  3

                       8                 4,5                 5

                       9                 3,5                 4

                      10                 10                 9,5

                      11                  8                  9

                      12                  4                  5

                      13                 5,5                 5

                      14                 6,5                 7

                      15                 5,5                 6

                      16                  1                  1

                      17                  3                  2

                      18                  5                  7

                      19                  2                  1

                      20                  4                  3

                      21                  5                 5,5

                      22                  7                  8


Determine o modelo de regressão linear para as variáveis notas das avaliações em Esta-
tística e o tempo de estudo em horas dos alunos.




                                         280

## Página 281

Resolução


a) Como já verificamos, as variáveis estão fortemente relacionadas pois, ao calcularmos
o coeficiente de correlação de Pearson, encontramos o valor de r = 0,96, portanto r>0,6
indicando uma forte correlação entre as variáveis.


Logo, podemos determinar o modelo de regressão linear y = ax + b.


Precisamos, então, calcular o coeficiente angular e o coeficiente linear da reta. Pelo mé-
todo dos mínimos quadrados temos:


                                   n ∙ ∑(x i ∙ yi) – (∑x i) ∙ (∑yi)
                              a=
                                          n ∙ ∑x i 2 – (∑x i)2


                                         b = y̅ – a ∙ x̅


Calculando o coeficiente angular “a”:


                  Horas de
    Aluno                       Notas (Y)             X.Y              X²        Y²
                 Estudo (X)

       1             7,5           8,5                63,75           56,25     72,25

       2             2              2                      4           4          4

       3             9              9                      81          81        81

       4             1,5            2                      3          2,25        4

       5             6              7                      42          36        49

       6             8,5            8                      68         72,25      64

       7             3              3                      9           9          9

       8             4,5            5                  22,5           20,25      25

       9             3,5            4                      14         12,25      16

      10             10            9,5                     95         100       90,25




                                              281

## Página 282

      11             8                9                       72               64       81

      12             4                5                       20               16       25

      13            5,5               5                   27,5                30,25     25

      14            6,5               7                   45,5                42,25     49

      15            5,5               6                       33              30,25     36

      16             1                1                       1                 1        1

      17             3                2                       6                 9        4

      18             5                7                       35               25       49

      19             2                1                       2                 4        1

      20             4                3                       12               16        9

      21             5                5,5                 27,5                 25      30,25

      22             7                8                       56               49       64

   Total (∑)        112             117,5               739,75                705      788,75



                                      n ∙ ∑(x i ∙ yi) – (∑x i) ∙ (∑yi)
                               a=
                                             n ∙ ∑x i 2 – (∑x i)2


                          22 ∙ 739,75 – 112 ∙ 117,5                3114,5
                   a=                                         =             = 1,0501
                              22 ∙ 705 – 1122                       2966




Calculando o coeficiente linear “b”.


                                            b = y̅ – a ∙ x̅


                                       117,5                        112
                                 b=               – 1,0501 ∙
                                          22                        22




                                                 282

## Página 283

                            b = 5,34090909 – 1,0501 ∙ 5,09090909
                       b = 5,34090909 – 5,345797734 = -0,049


Logo o modelo de regressão y = ax + b será y = 1,0501x – 0,049.


Exemplo 2


Voltemos ao Exemplo 2 do Tópico 2: massa muscular X idade.


Acredita-se que, com o passar dos anos, as mulheres vão perdendo massa muscular
significativamente. Para avaliar essa hipótese foi feita uma pesquisa em uma amostra de
30 pacientes do sexo feminino, quando foram observadas a massa muscular e a idade. O
cálculo da massa muscular pode ser feito pela avaliação das dobras cutâneas e também
pela fórmula da área muscular do braço. O resultado está apresentado na tabela a seguir.


Calcule o coeficiente de correlação de Pearson e verifique se realmente o índice de
massa muscular está correlacionado com a idade da paciente.


                    Paciente               Idade         Massa muscular

                       1                    45                     97

                       2                    56                     90

                       3                    65                     74

                       4                    58                     80

                       5                    49                     105

                       6                    62                     68

                       7                    51                     83

                       8                    53                     79

                       9                    71                     68

                       10                   69                     73

                       11                   58                     95

                       12                   73                     63

                       13                   53                     100

                       14                   45                     116




                                            283

## Página 284

                       15                    76                       65

                       16                    69                       68

                       17                    74                       65

                       18                    67                       65

                       19                    56                       82

                       20                    63                       92

                       21                    44                       102

                       22                    72                       65

                       23                    78                       77

                       24                    68                       78

                       25                    67                       88

                       26                    39                       105

                       27                    51                       102

                       28                    64                       71

                       29                    60                       87

                       30                    40                       119




Resolução


b) Como já verificamos, as variáveis estão fortemente relacionadas pois, ao calcularmos
o coeficiente de correlação de Pearson, encontramos o valor de r = -0,86, portanto r>0,6
indicando uma forte correlação entre as variáveis.


Logo, podemos determinar o modelo de regressão linear y = ax + b.


Precisamos, então, calcular o coeficiente angular e o coeficiente linear da reta. Pelo mé-
todo dos mínimos quadrados temos:


                                   n ∙ ∑(x i ∙ yi) – (∑x i) ∙ (∑yi)
                              a=
                                         n ∙ ∑x i 2 – (∑x i)2


                                        b = y̅ – a ∙ x̅



                                             284

## Página 285

Calculando o coeficiente angular “a”:


                                 Massa
   Paciente        Idade                         x. y    x²     y²
                                muscular

       1             45             97           4365   2025   9409

       2             56             90           5040   3136   8100

       3             65             74           4810   4225   5476

       4             58             80           4640   3364   6400

       5             49            105           5145   2401   11025

       6             62             68           4216   3844   4624

       7             51             83           4233   2601   6889

       8             53             79           4187   2809   6241

       9             71             68           4828   5041   4624

      10             69             73           5037   4761   5329

      11             58             95           5510   3364   9025

      12             73             63           4599   5329   3969

      13             53            100           5300   2809   10000

      14             45            116           5220   2025   13456

      15             76             65           4940   5776   4225

      16             69             68           4692   4761   4624

      17             74             65           4810   5476   4225

      18             67             65           4355   4489   4225

      19             56             82           4592   3136   6724




                                           285

## Página 286

      20            63              92                 5796             3969     8464

      21            44             102                 4488             1936    10404

      22            72              65                 4680             5184     4225

      23            78              77                 6006             6084     5929

      24            68              78                 5304             4624     6084

      25            67              88                 5896             4489     7744

      26            39             105                 4095             1521    11025

      27            51             102                 5202             2601    10404

      28            64              71                 4544             4096     5041

      29            60              87                 5220             3600     7569

      30            40             119                 4760             1600    14161

   Total (∑)       1796            2522              146510            111076   219640



                                    n ∙ ∑(x i ∙ yi) – (∑x i) ∙ (∑yi)
                             a=
                                           n ∙ ∑x i 2 – (∑x i)2


                                    30 ∙ 146510 – 1796 ∙ 2522
                              a=
                                       30 ∙ 111076 – (1796)2


                                       –134212
                                a=                    = -1,2583
                                         106664




Calculando o coeficiente linear “b”.

                                          b = y̅ – a ∙ x̅




                                               286

## Página 287

                                     2522                     1796
                              b=               –(–1,2583) ∙
                                      30                        30


                        b = 84,066666667 + 1,2583 ∙ 59,8666666667


                             b = 84,0666666667+75,33022667


                                           b = 159,40


Logo o modelo de regressão y = ax + b será: y = -1,2583x + 159,40.


         Ampliando o foco


       Para colocar em prática o conteúdo visto nesta unidade, visite a Biblioteca Virtual,
       escolha um livro de Estatística, selecione e faça alguns exercícios do livro para
       reforçar o conhecimento sobre a determinação do modelo de Regressão Linear.


       Confira as respostas que estão no final do livro e se tiver alguma dúvida fale
       com o seu tutor pelo Fórum de Dúvidas – Fale com o Tutor.




                                         MIDIATECA

       Para ampliar o seu conhecimento veja o material complementar da Unidade 4,
       disponível na midiateca.




                                               287

## Página 288

                                 NA PRÁTICA


Vamos refletir: Os moinhos de vento são muito barulhentos?


Os moinhos de vento modernos estão se tornando uma alternativa eficiente
e limpa para produção de energia. Embora um moinho de vento eficaz possa
utilizar vento de baixa velocidade, os locais ideais para parques eólicos são ao
longo da costa oceânica ou em áreas montanhosas, onde há uma velocidade
de vento consistente de pelo menos 24 km/h.


Os moinhos de vento não podem ser construídos próximos a cidades devido às
leis que regulam o nível de ruído. Um moinho de vento típico produz aproxima-
damente 56dB a 60 metros de distância. Isso é mais suave do que o som da
fala humana (que está em cerca de 70dB).


Suponha que uma pequena comunidade costeira esteja considerando a cons-
trução de um moinho de vento para gerar eletricidade para a prefeitura. Um ex-
perimento é conduzido para medir o nível de ruído do moinho de vento (em dB)
a várias distâncias (em metros) do local proposto. Os dados estão resumidos
na tabela a seguir.




  Distância (m)       3    15    23     37    46   49     61    76   122 150

  Nível de Ruído
                      75   110   73     52    58   77     56    57    28     4
       (dB)



Com as técnicas apresentadas nesta unidade podemos verificar facilmente que
há uma relação linear significativa entre a variável independente “distância” e a
variável dependente “nível de ruído”.




                                        288

## Página 289

                                 Título do Gráfico          y = -0,542x + 90,546
                                                                 R2 = 0,7777
        120

        100

         80

         60

         40

         20

          0
              0     20     40      60      80      100    120      140       160


                         Fonte: Elaborado pela autora (2020).


Também podemos usar a análise de regressão que estudamos nesta unidade
para prever um valor do nível de ruído para uma determinada distância do moi-
nho de vento. Utilizando os dados levantados sobre o nível de ruído e a distância
chegamos ao seguinte modelo matemático:


                                y = -0,542x + 90,546


Suponhamos que um moinho de vento esteja sendo construído a 100m de sua
residência. Podemos prever, com auxílio do modelo, que, para essa distância, o
nível de ruído está próximo de 36 dB (o que está entre um suave sussurro e um
escritório tranquilo).

Fonte: Adaptado de Kokoska (2013).




                                         289

## Página 290

Resumo da Unidade 4

Nesta unidade aprofundamos nossos conhecimentos na Estatística Indutiva. Iniciamos
apresentando os conceitos de Intervalos de Confiança, em que a partir dos dados cole-
tados em uma amostra foi possível inferir um intervalo sob um certo grau de confiança,
que continha o verdadeiro valor do parâmetro, como se tivéssemos feito nossa pesquisa
com todos os elementos da população.


Já familiarizados com o uso da tabela normal padronizada, conhecemos, então, o tão
esperado cálculo do tamanho de uma amostra. Também apresentamos os conceitos
do coeficiente de correlação linear de Pearson, que nos auxiliou na verificação da força
com que uma variável está relacionada com a outra. Apresentamos, ainda, o modelo de
Regressão Linear, por meio do qual foi possível estimar a equação de uma reta que me-
lhor descreveria a relação entre uma variável dependente e uma variável independente.


Acreditamos, fortemente, que ao chegar até aqui você esteja familiarizado com os con-
ceitos fundamentais, tanto da Estatística Descritiva quanto da Estatística Indutiva. Ainda
temos uma vasta gama de conceitos, mas com as ferramentas adquiridas nesse curso
você conseguirá facilmente assimilar os novos conhecimentos.


Esse foi o pontapé inicial para mergulharmos no fascinante mundo da Estatística.


Saudações!




                                           290

## Página 291

Referências

KOKOSKA, S. Introdução à estatística: uma abordagem por resolução de problemas.
Rio de Janeiro: LTC, 2013.


LARSON, R.; FARBER, B. Estatística aplicada. São Paulo: Pearson, 2010.


NEXO Jornal. Como funciona uma pesquisa eleitoral. YouTube. Disponível em: https://
youtu.be/igi7E1OY7gs. Acesso em: 19 set. 2020.


SWEENEY, D. J.; WILLIAMS, T. A.; ANDERSON, D. R. Estatística aplicada à administra-
ção e economia. 3. ed. São Paulo: Cengage Learning, 2013.


TABELA de Distribuição Normal Padronizada. Disponível em: http://ead.uva.br///recurso/
DEF/EST/u3_c3_r1/anexo/Tabela-da-Distribuicao-Normal.pdf. Acesso em: 5 set. 2020.


TABELA T-student. Disponível em: http://ead.uva.br////recurso/DEF/EST/u4_c1_r1/index.
htm. Acesso em: 19 set. 2020.


TEC Concursos. Intervalo de confiança para a média. #720037 Pró-Reitoria GP CP2 - Esta-
tístico (CP II)/2017. Disponível em: https://www.tecconcursos.com.br/questoes/720037.
Acesso em: 5 out. 2020.




                                          291

## Página 292

Sem texto selecionável nesta página; consulte a página correspondente no PDF original.
