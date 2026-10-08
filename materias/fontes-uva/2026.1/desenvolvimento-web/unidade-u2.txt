Análise de Requisitos para
Projetos Front-End



    Introdução                              02
    Tema 1                                  04
      Conhecendo projetos Front-End


    Tema 2                                  12
      Conceito de análise de requisitos


    Tema 3                                  19
      Exemplos de análise de requisitos e
      aplicações práticas


    Tema 4                                  23
      Conceitos de Prototipação


    Encerramento                            28
    Referências                             30
Introdução
Olá, estudante! Bem vindo (a) mundo da análise de requisitos para projetos front-end.
Talvez você já tenha ouvido falar sobre o papel crucial dos requisitos em um projeto,
mas sabe realmente o que isso significa para o desenvolvimento front-end? Imagine
que você está começando um novo projeto, cheio de ideias inovadoras para um site
ou uma aplicação web. Por onde começar? Quais são os primeiros passos para garantir
que o projeto atenda às expectativas do cliente e dos usuários?
Para começarmos, vamos explorar o que são os projetos front-end. Basicamente,
front-end é a parte de um aplicativo ou site que interage diretamente com o usuário.
Tudo aquilo que você vê e com o que você interage em uma página web é resultado
do trabalho de um desenvolvedor front-end.
O conceito de análise de requisitos é fundamental para qualquer projeto de softwa-
re. No contexto do front-end, isso envolve entender as necessidades e expectativas
dos usuários, bem como as funcionalidades e restrições que o site ou aplicação deve
ter. Por exemplo, qual deve ser a aparência do site? Quais funcionalidades ele deve
oferecer? Como ele deve se comportar em diferentes dispositivos?
Vamos ver alguns exemplos práticos de análise de requisitos e como aplicá-los. Isso
inclui criar documentos detalhados que descrevem todas as funcionalidades espera-
das, wireframes que esboçam a estrutura do site e protótipos que oferecem uma
visão mais tangível do produto final. Prototipação, é uma parte crucial desse proces-
so. Ferramentas como Figma, Sketch e Adobe XD permitem criar modelos interativos
que ajudam a visualizar e refinar o design antes de iniciar a programação.
Ao longo deste material, vamos detalhar cada um desses aspectos, proporcionando
uma compreensão clara e prática de como conduzir uma análise de requisitos eficiente.
Objetivos
                      Ao final desta unidade, você deverá ser capaz de:
                        •    Aprender técnicas avançadas de prototipagem para
                        validar e iterar designs.




Conteúdo programático
                        Esta unidade está organizada de acordo com os seguintes
                        temas:
                        •   Tema 1 – Conhecendo projetos Front-End;
                        •   Tema 2 – Conceito de análise de requisitos;
                        •   Tema 3 – Exemplos de análise de requisitos e aplicações
                        práticas;
                        •   Tema 4 – Conceitos de Prototipação.




Objeto motivacional
Imagine-se como um arquiteto digital, proje-
tando portais que fazem os olhos dos usuários
brilharem e suas vidas ficarem mais simples.
Esse é o poder do design centrado no usuário.
Ao explorar a análise de requisitos para pro-
jetos front-end, você desvendará os segredos
para entender as verdadeiras necessidades dos
usuários e transformar essas descobertas em interfaces mágicas e intuitivas. Com
exemplos práticos e conceitos de prototipação, você aprenderá a combinar funciona-
lidade técnica e estética visual, criando experiências que são não apenas eficientes,
mas também envolventes e adaptáveis.
Tema 1
    Conhecendo Projetos Front-End




Por que Front-End e Back-End são importantes para a experiência do
usuário?


O desenvolvimento de software envolve uma série de etapas e disciplinas essenciais
para a criação de sistemas eficientes e funcionais. Dentro desse contexto, o desenvol-
vimento Front-End tem ganhado destaque por sua importância na interação direta
com o usuário final.
Mas, por que o Front-End é importante para a experiência do usuário? Observe a
figura abaixo que mostra um desenvolvedor escrevendo código em um ambiente de
trabalho. Este processo envolve transformar as necessidades dos usuários ou do mer-
cado em um produto de software que seja funcional e atrativo.




    Figura 2.1 - Na computação, o desenvolvimento de software
    Fonte: Jedchela / Wikimedia Commons.


                                               4
A importância do Front-End reside no fato de que ele é o ponto de contato direto en-
tre o usuário e o sistema. A experiência do usuário (UX) depende diretamente da qua-
lidade dele. Um bom design de Front-End pode significar a diferença entre um sistema
que é fácil de usar e um que é frustrante e confuso. Dennis, et al. (2014) afirmam que
um dos objetivos principais do Front-End é facilitar a usabilidade, o que inclui a faci-
lidade de navegação, a clareza das informações apresentadas e a responsividade do
sistema.
Para entender melhor essa importância, é útil compararmos o Front-End com o Back-
-End. Observe o quadro abaixo que destaca as diferenças e responsabilidades entre
eles, mostrando como cada um contribui de maneira única para o desenvolvimento e
funcionalidade de um sistema.



          Aspecto                        Front-End                      Back-End


                                                                 Processamento, armazena-
    Função Principal              Apresentação e interação
                                                                  mento e gestão de dados



                                                                 Lógica de negócios e mani-
   Responsabilidade                  Interface do usuário
                                                                     pulação de dados



                                                                 Funcionalidade interna do
             Foco                   Experiência do usuário
                                                                         sistema



       Descrições                  Visível para o usuário, in-   Cérebro do sistema, opera-
      Comparativas                      terface gráfica                ções internas



                                                                   Node.js, Python, Java,
       Tecnologias                  HTML, CSS, JavaScript,
                                                                  Ruby on Rails, SQL, Mon-
         Comuns                     React, Angular, Vue.js
                                                                           goDB

Quadro 2.1 - Comparação entre Front-End e Back-End
Fonte: elaborado pelo autor.


O uso das tecnologias no desenvolvimento de Front-End permite que os desenvolve-
dores criem interfaces que não são apenas atraentes, mas também funcionais e aces-
síveis. Segundo Dennis et al. (2014), o objetivo do uso dessas ferramentas é melhorar
a eficiência do desenvolvimento e a qualidade final do produto.



                                                5
As bibliotecas e frameworks modernas oferecem componentes reutilizáveis, gestão de
estados e facilidades para a integração com APIs (Application programming Interface -
Interface de programação de aplicações), o que acelera o processo de desenvolvimento
e garante maior consistência e manutenção do código.
Portanto, ao falarmos de projetos de Front-End, estamos nos referindo a uma área
que exige uma combinação de habilidades de design e programação. A criação de
interfaces eficazes requer não apenas um entendimento profundo das necessidades
dos usuários, mas também conhecimento técnico sobre as tecnologias disponíveis.
Observe a figura abaixo, que mostra um fluxograma ilustrando como Front-end e
back-end se conectam por meio de uma API.




    Figura 2 2 - Front-end e back-end se conectam por meio de uma API
    Fonte: Diego Mariano / Wikimedia Commons.



A análise e modelagem eficazes de sistemas são fundamentais para o sucesso de
qualquer projeto de software, e isso inclui um foco especial no desenvolvimento de
Front-End (MARINHO, 2016).
A arquitetura de um projeto Front-End é fundamental para garantir a escalabilida-
de, manutenção e eficiência do desenvolvimento. Entender a estrutura típica de um
projeto Front-End e a importância da modularidade e reutilização de componentes é
essencial para criar aplicações robustas e fáceis de gerenciar.
Um projeto Front-End é organizado em uma estrutura hierárquica que facilita a gestão
dos diferentes componentes e recursos. Confira as principais partes dessa estrutura,
essencial para um desenvolvimento eficiente e modular:




                                                6
 Componentes são blocos reutilizáveis de
 código que encapsulam funcionalidades
 específicas da interface do usuário. Em
 frameworks como React, os componentes
 são a unidade fundamental de construção
 da interface.




 Páginas representam
 diferentes vistas ou telas
 que o usuário verá ao
 navegar pela aplicação.
 Cada página pode ser
 composta por vários com-
 ponentes.




 A navegação gerên-
 cia como os usuários
 se movem entre as
 páginas da aplicação,
 podendo ser feita
 através de menus de
 navegação, links e ro-
 teamento. Em React, a
 biblioteca React Rou-
 ter é frequentemente
 usada para configurar
 rotas e gerenciar a
 navegação.


A modularidade é um princípio chave na arquitetura de projetos Front-End. Ela per-
mite dividir a aplicação em partes menores e gerenciáveis, chamadas de módulos ou
componentes, cada um responsável por uma funcionalidade específica. Isso traz vários
benefícios. Conheça quais são eles:



                                           7
    •     Facilidade de Manutenção: Componentes modulares são mais fáceis
    de manter e atualizar, pois qualquer mudança necessária pode ser feita de
    forma isolada sem impactar outras partes da aplicação.
    •     Reutilização: Componentes reutilizáveis promovem a eficiência do
    desenvolvimento. Uma vez criados, os componentes podem ser usados em
    diferentes partes da aplicação, reduzindo a duplicação de código.
    •      Testabilidade: Componentes independentes são mais fáceis de testar,
    pois cada um pode ser testado isoladamente antes de ser integrado ao siste-
    ma maior.
    •     Colaboração: Em equipes de desenvolvimento, a modularidade permi-
    te que diferentes membros da equipe trabalhem em componentes distintos
    simultaneamente, acelerando o desenvolvimento.
    •      Escalabilidade: Projetos modulares são mais fáceis de escalar, permi-
    tindo a adição de novas funcionalidades sem a necessidade de reestruturar a
    aplicação inteira.




O desenvolvimento Front-End é uma área dinâmica e essencial no processo de criação
de aplicações web. Através de práticas bem estabelecidas, como o uso de frameworks
modernos e metodologias ágeis, os desenvolvedores podem criar interfaces de usuá-
rio eficientes, responsivas e acessíveis. A análise de requisitos é uma fase crítica que
deve ser abordada com rigor para garantir que o produto final atenda às necessidades
dos usuários e aos objetivos do negócio (SOMMERVILLE, 2007).
Uma das melhores práticas no desenvolvimento Front-End é a adoção de frameworks
e bibliotecas atualizados, como React, Angular e Vue.js. Esses frameworks permitem
a criação de componentes reutilizáveis e a gestão eficiente do estado da aplicação,
facilitando a manutenção e evolução do projeto. Segundo Marinho (2016), a utilização
de frameworks modernos não apenas acelera o desenvolvimento, mas também impõe
padrões de qualidade e consistência ao código.
As tendências atuais no desenvolvimento Front-End refletem uma adaptação contí-
nua, onde a combinação de tecnologias como PWAs (Aplicativos da Web Progressivos)
e ferramentas de automação se torna crucial para atender às expectativas crescen-
tes dos usuários em relação a desempenho, funcionalidade e experiência do usuário
(DENNIS et al., 2014). Conheça essas tendências:




                                           8
                                     PWAs combinam características de web
                                     apps com funcionalidades de apps móveis,
    Aplicativos da Web
                                     oferecendo experiência robusta, mesmo
    Progressivos
                                     offline, usando Service Workers, caching e
                                     instalação na tela inicial.




                                     Ferramentas como Webpack e Gulp otimi-
                                     zam o build e deployment de apps, auto-
      Ferramentas de                 matizando tarefas como minificação de
      Automação                      código, compilação de CSS e otimização de
                                     imagens, melhorando a eficiência e de-
                                     sempenho.




                                     Ferramentas de automação simplificam o
                                     desenvolvimento e aprimoram a experiên-
        Garantia de
                                     cia do usuário, garantindo que apps sejam
        Desempenho
                                     rápidas, responsivas e eficientes em diver-
                                     sos dispositivos.




A acessibilidade é outro aspecto fundamental no desenvolvimento Front-End. Criar
interfaces que sejam acessíveis a todos, incluindo pessoas com deficiências, é não ape-
nas uma responsabilidade ética, mas também uma exigência legal em muitos países.
Conforme destaca Sommerville (2007), acessibilidade deve ser considerada desde o
início do desenvolvimento, integrando técnicas como o uso correto de ARIA (Accessib-
le Rich Internet Applications) e a validação contínua com ferramentas específicas.
O SEO (Search Engine Optimization) também desempenha um papel crucial na visibi-
lidade e sucesso de uma aplicação web. Práticas de SEO, como a correta utilização de
tags HTML, otimização de imagens e melhoria da velocidade de carregamento das
páginas, garantem que os motores de busca possam indexar e ranquear a aplicação de
maneira eficaz. Marinho (2016) ressalta que uma boa estratégia de SEO começa com a
estruturação semântica do HTML e a escolha cuidadosa das palavras-chave, que refle-
tem as intenções de busca dos usuários. observe a figura abaixo que mostra os méto-
dos comuns usados para a otimização.




                                          9
    Figura 2.3 - Front-end e back-end se conectam por meio de uma API
    Fonte: Giulia Forsythe / Wikimedia Commons.


A integração dessas práticas no fluxo de trabalho do desenvolvimento Front-End
resulta em aplicações mais robustas, acessíveis e otimizadas para motores de busca.
A combinação de técnicas avançadas de desenvolvimento com uma análise de requisi-
tos bem elaborada é a chave para o sucesso de projetos Front-End, garantindo que as
soluções não apenas atendam às expectativas dos usuários, mas também se adaptem
às constantes evoluções tecnológicas.



                                             Para refletir
                Por que o Front-End é crucial para a experiência do usuário? Pen-
                se sobre como a interface visual influencia a interação e satisfa-
                ção do usuário com o sistema. Considerando a comparação entre
                Front-End e Back-End, como cada um contribui para a funcionali-
                dade e usabilidade de um sistema? Reflita sobre a importância da
                modularidade no desenvolvimento Front-End e como isso impac-
                ta a manutenção e escalabilidade de um projeto.
                Fonte: elaborado pelo autor




                                                  10
Portanto, conhecer as melhores práticas de desenvolvimento Front-End, estar atento
às tendências e compreender a importância de acessibilidade e SEO são elementos
essenciais para o sucesso de qualquer projeto. Estes princípios garantem não apenas
a criação de interfaces atraentes e funcionais, mas também a satisfação do usuário e a
sustentabilidade a longo prazo da aplicação.




                                         11
Tema 2
    Conceito de Análise de Requisitos




Qual a importância de uma análise de requisitos bem-feita para o
sucesso de um projeto de software?

Antes de falar sobre a análise de requisitos, vamos entender o que é um requisito.
Ele é simplesmente uma declaração do que o sistema deve fazer ou de quais caracte-
rísticas ele precisa possuir. Durante um projeto de desenvolvimento de sistemas, são
criados diferentes tipos de requisitos. Conheça cada um deles:




 Requisitos do    Requisitos dos        Requisitos       Requisitos não      Requisitos do
   negócio          usuários            funcionais         funcionais          sistema



                                                           Definem as
 Descrevem as                      Especificam as
                   Indicam o que                         características    Explicam como
 necessidades                      funções que o
                  os usuários pre-                        e qualidades      o sistema deve
 e objetivos da                    software deve
                   cisam realizar.                       que o sistema      ser construído
  organização                      desempenhar
                                                            deve ter


Embora essa lista de requisitos possa parecer extensa, as categorias refletem apenas o
objetivo dos requisitos e o estágio do ciclo de vida do desenvolvimento de sistemas –
CVDS (System Development Life Cycle ou SDLC) é o processo de determinação do modo
como um sistema de informações pode fornecer suporte para as necessidades da em-
presa, projetar o sistema, construí-lo e entregá-lo aos usuários (DENNIS et al., 2014).
A análise de requisitos é um processo essencial na engenharia de software que envol-
ve a identificação, documentação e gerenciamento das necessidades e expectativas
dos stakeholders para um projeto.

           “Durante a fase de análise, os requisitos são escritos sob a perspectiva do negócio;
           eles enfocam o que o sistema precisa fazer para satisfazer as necessidades do
           usuário do negócio. Um bom ponto de partida é concentrar-se no que o usuário re-




                                             12
           almente precisa realizar com o sistema a fim de cumprir um trabalho ou uma ta-
           refa necessária. Esses requisitos do usuário descrevem as tarefas que os usuários
           realizam como uma parte integral das operações comerciais, tais como: ‘Agendar
           reunião com um cliente’; ‘Remeter novo pedido de um cliente’; ‘Reorganizar o
           estoque’; ‘Determinar o crédito disponível’ e ‘Examinar os saldos das contas’.”
           (DENNIS et al., 2014, p. 100)

Os casos de uso são ferramentas utilizadas para esclarecer as etapas desenvolvidas na
realização dessas tarefas dos usuários. Entendendo o que o usuário precisa fazer em
termos de tarefas a realizar, é possível ao analista, a seguir, determinar as maneiras
pelas quais o novo sistema pode fornecer suporte a essas necessidades dos usuários.



                                           Saiba mais
               Você sabia? Na engenharia de software, os requisitos se dividem
               em funcionais e não funcionais. Os requisitos funcionais especifi-
               cam o que o sistema deve fazer, como as funcionalidades e tare-
               fas principais. Já os requisitos não funcionais descrevem como
               o sistema deve operar, abordando características de qualidade
               como segurança, desempenho e usabilidade. A distinção é crucial,
               pois garante que o software atenda tanto às necessidades prá-
               ticas quanto às expectativas de qualidade, proporcionando uma
               experiência eficiente e segura para os usuários.
               Para saber mais, acesse a seguir: Requisito Funcional e Não Fun-
               cional de Software: entenda a diferença.
               Fonte: elaborado pelo autor



A análise de requisitos mais simples e comum é a análise do problema. Ela envolve
pedir aos usuários e gerentes que identifiquem problemas no sistema atual e sugiram
soluções para o sistema futuro (DENNIS et al., 2014). Os usuários geralmente têm
boas ideias de modificações e estão dispostos a sugeri-las. Essas modificações tendem
a resolver problemas existentes e resultam em melhorias incrementais, como adicio-
nar um campo para o telefone celular do cliente ou criar um novo relatório (DENNIS et
al., 2014).
Esse tipo de melhoria é eficiente para aumentar a eficiência do sistema e a facilidade
de uso, mas geralmente oferece apenas pequenas melhorias no valor do negócio. O
novo sistema pode ser melhor que o antigo, mas pode ser difícil identificar vantagens
financeiras significativas.
Segundo Sommerville (2007), a análise de requisitos é uma etapa crítica no ciclo de
vida do desenvolvimento de software, onde os requisitos são coletados, analisados e
especificados para garantir que todos os envolvidos compreendam claramente o que
o sistema deve realizar.

                                            13
   Fonte: VG Stock




 Importância da Análise de Requisitos

 A análise de requisitos é crucial para o sucesso de um projeto. Dennis, Wixom e Roth
 (2014) destacam que uma boa análise garante que o produto final atenda às necessidades
 dos usuários e cumpra os objetivos do negócio. Requisitos bem definidos permitem que as
 equipes de desenvolvimento trabalhem com mais eficiência, evitando retrabalho e redu-
 zindo custos. Por outro lado, uma análise mal feita pode ser desastrosa. Marinho (2016)
 aponta que problemas na definição de requisitos são uma das principais causas de falhas
 em projetos de software, levando a atrasos, aumento de custos e insatisfação do cliente.




A satisfação do usuário final está diretamente ligada à qualidade da análise de requi-
sitos. Quando os requisitos são bem definidos e compreendidos, o resultado é um sis-
tema que atende às expectativas dos usuários. Dennis, Wixom e Roth (2014) afirmam
que a satisfação do usuário é um dos principais indicadores de sucesso de um projeto
de software. Um sistema que não atende às necessidades do usuário final pode com-
prometer a reputação da empresa e resultar na perda de clientes.
A análise de requisitos, portanto, é uma atividade que requer habilidades de comunica-
ção eficazes, atenção aos detalhes e uma compreensão profunda das necessidades do
negócio e dos usuários. Sommerville (2007) enfatiza que a participação ativa dos stake-
holders durante o processo de coleta e análise de requisitos é crucial para garantir que
todas as necessidades sejam capturadas e que o projeto siga na direção correta.
A elicitação de requisitos é o processo de coletar informações dos stakeholders para
identificar e compreender as necessidades do sistema. Existem várias técnicas que
podem ser utilizadas.

                                           14
A entrevista é uma técnica comum de coleta de requisitos em projetos, sendo condu-
zida normalmente de forma personalizada. Dennis et al. (2014) destacam cinco etapas
básicas dentro do processo de entrevista. Conheça quais são elas:



        •     Agenda com entrevistados selecionados de diferentes níveis,
        proporcionando visão completa do sistema.
        •      Perguntas em ordem lógica, top-down ou bottom-up conforme
        familiaridade e conforto.
        •     Planejamento detalhado das perguntas e avisos prévios aos
        entrevistados.
        •     Criação de afinidade, registro cuidadoso e esclarecimento de
        dúvidas durante a entrevista. Separação de fatos e opiniões.
        •      Relatório resumindo informações enviado ao entrevistado
        para revisão antes da distribuição.



O Desenvolvimento Conjunto de Aplicações (JAD) é uma técnica colaborativa criada
pela IBM nos anos 1970 para identificar requisitos de sistemas através de sessões
estruturadas. Nessas sessões, participam a equipe de projeto, usuários e gerência. Um
facilitador conduz a sessão de forma neutra, enquanto auxiliares anotam e utilizam
ferramentas CASE para registrar informações (DENNIS et al., 2014).
As sessões JAD ajudam a evitar problemas de requisitos vagos ou específicos demais,
podendo reduzir em 50% o scope creep. No entanto, enfrentam desafios como a do-
minância de alguns participantes e a relutância de outros em compartilhar opiniões.




          O JAD eletrônico (e-JAD) resolve esses problemas com software
         de colaboração que permite o envio anônimo de ideias e votações,
                       acelerando o processo em até 80%.




A seleção de participantes é crucial, envolvendo aqueles que podem contribuir signi-
ficativamente e necessitando de suporte da direção para liberar os melhores funcio-
nários. O facilitador deve ter experiência em técnicas de JAD e, muitas vezes, é um
consultor externo (DENNIS et al., 2014).
As sessões JAD podem durar de meio dia a várias semanas, dependendo do projeto,
e devem ser cuidadosamente planejadas, com uma agenda estruturada e preparação


                                         15
prévia dos participantes. O facilitador deve manter o foco da agenda, esclarecer ter-
mos técnicos e registrar contribuições sem influenciar o grupo.
Ferramentas como casos de uso, protótipos e modelos de processos são utilizadas
para definir completamente o novo sistema (DENNIS et al., 2014). Após as sessões, um
relatório é elaborado e distribuído para documentar as informações coletadas.
Um questionário é um conjunto de perguntas escritas usadas para obter informações
de indivíduos, sendo especialmente útil para alcançar um grande número de pesso-
as, como clientes, fornecedores ou funcionários em várias localizações geográficas.
Atualmente, muitos questionários são distribuídos eletronicamente por e-mail ou pela
web, economizando custos em relação aos questionários em papel. Confira as técnicas
eficientes para administração de questionários:



  •      Selecionar uma amostra representativa, com taxas de retorno de 30% a
  50% para questionários de papel e eletrônicos, e de 5% a 30% para questioná-
  rios online, é o primeiro passo. Formular perguntas claras e específicas, sepa-
  rando fatos de opiniões e evitando ambiguidades, é crucial. Antes da distribui-
  ção, testar o questionário com um grupo pequeno garante consistência.
  •      Explicar claramente o propósito do questionário, estabelecer prazos
  para devolução, oferecer incentivos e supervisionar a administração duran-
  te reuniões são fundamentais para obter respostas significativas. Processar
  rapidamente os questionários devolvidos e elaborar um relatório detalhado
  garantem a continuidade da análise e a entrega oportuna dos resultados aos
  participantes.




A análise de documentos ajuda as equipes de projeto a entenderem o sistema exis-
tente. Idealmente, a documentação deve ser continuamente atualizada, mas muitas
vezes isso não acontece, resultando em pouca ou nenhuma documentação técnica.
Documentos úteis incluem relatórios em papel, memorandos, manuais de políticas
e treinamento, organogramas e formulários. Relatórios de problemas dos usuários
também são valiosos. Esses documentos mostram o sistema formal da empresa, mas
o sistema “real” pode ser diferente, indicando áreas para mudanças. Formulários e re-
latórios não utilizados devem ser eliminados, e campos raramente preenchidos devem
ser revisados.
Para melhorar a compreensão dos sistemas existentes, é essencial revisar tanto os for-
mulários vazios quanto preenchidos para identificar necessidades de aprimoramento.
A observação direta oferece uma perspectiva mais concreta, indo além das descrições
fornecidas em entrevistas ou sessões JAD. Estudos mostram que muitos gerentes têm
dificuldade em recordar com precisão suas rotinas e alocação de tempo.




                                          16
Assim, a observação valida as informações obtidas por outras fontes, como entrevis-
tas e questionários. Para evitar influenciar o comportamento dos observados, o analis-
ta deve agir discretamente, como um antropólogo, durante a observação. No entanto,
a presença do analista pode modificar as rotinas, pois as pessoas tendem a ser mais
cuidadosas sob observação, o que pode não refletir seu comportamento habitual.
Essa abordagem complementa os dados coletados em entrevistas, pois aspectos
como a localização do escritório e o mobiliário podem indicar poder e influência, cor-
roborando ou desafiando as informações obtidas previamente.
A priorização de requisitos é o processo de determinar a ordem de implementação
dos requisitos com base em sua importância e impacto no projeto. Um dos métodos
mais conhecidos e utilizados para a priorização de requisitos é o MoSCoW. Esse méto-
do é uma técnica de priorização que classifica os requisitos em quatro categorias.
Segundo Dennis et al. (2014), o método MoSCoW ajuda a garantir que os recursos do
projeto sejam direcionados para as funcionalidades mais importantes, otimizando a
utilização do tempo e dos recursos disponíveis.
Mudanças nos requisitos durante o desenvolvimento de um projeto são comuns e,
em muitos casos, inevitáveis. A gestão eficaz dessas mudanças é fundamental para o
sucesso do projeto. Leia mais sobre a temática:



  •      Monitoramento e Controle: Implementar um sistema de gerenciamen-
  to de mudanças é crucial para registrar, analisar e aprovar alterações de requi-
  sitos de maneira estruturada.
  •     Análise de Impacto: Antes de aprovar mudanças, é essencial realizar
  uma análise para avaliar seu impacto no projeto, incluindo programação, orça-
  mento e recursos.
  •     Comunicação e Colaboração: Manter uma comunicação aberta com
  stakeholders é fundamental para gerenciar mudanças. Todos devem ser infor-
  mados para facilitar a adaptação.




A classificação e priorização dos requisitos são etapas fundamentais no processo de
desenvolvimento de sistemas, permitindo uma melhor organização e gestão das ne-
cessidades do projeto. Métodos como o MoSCoW ajudam a focar nos requisitos mais
importantes, enquanto a gestão eficaz das mudanças de requisitos assegura que o
projeto permaneça no caminho certo, mesmo diante de ajustes necessários. A combi-
nação dessas práticas contribui significativamente para a entrega de um sistema que
atenda às expectativas dos stakeholders e aos objetivos do negócio.




                                         18
Tema 3
    Exemplos de Análise de Requisitos e Aplicações Práticas




Como a priorização de requisitos utilizando o método MoSCoW pode
influenciar o sucesso de um projeto Front-End?

A análise de requisitos é uma etapa fundamental no desenvolvimento de projetos
front-end, pois garante que todas as necessidades dos stakeholders sejam identifica-
das e atendidas. Ferramentas específicas são amplamente utilizadas para organizar,
rastrear e priorizar esses requisitos, oferecendo uma visão clara do progresso e facili-
tando a comunicação entre a equipe. Ferramentas como Trello, Jira e Microsoft Pro-
ject se destacam por sua eficácia e versatilidade, contribuindo significativamente para
o sucesso dos projetos.



 Trello é uma ferramenta de gerenciamen-
 to baseada em Kanban, permitindo criar
 quadros com listas e cartões para organizar
 tarefas. Cada cartão representa um requi-
 sito, podendo ser categorizado, priorizado
 e atualizado conforme o desenvolvimento
 avança. Sua interface intuitiva facilita o
 acompanhamento do progresso e a comu-
 nicação entre a equipe (SOMMERVILLE,
 2007).
                                                      Fonte: Trello / Wikimedia Commons




 Jira, por sua vez, oferece funcionalidades
 avançadas de rastreamento de bugs e
 gestão de tarefas, além de integrar-se com
 outros sistemas. É possível criar stories,
 definir prioridades e acompanhar requisi-
 tos até a entrega final (DENNIS et al. 2014).
 Relatórios e dashboards personalizados
 permitem um monitoramento detalhado
 do progresso do projeto.
                                                      Fonte: Jira / Wikimedia Commons




                                                 19
 Microsoft Project é ideal para projetos
 complexos, permitindo criar cronogramas
 detalhados e gerenciar recursos de forma
 rigorosa (MARINHO, 2016). Sua integração
 com o Excel facilita a importação/exporta-
 ção de dados, melhorando a eficiência no
 gerenciamento de informações.

                                                   Fonte: Microsoft / Wikimedia Commons



A análise de requisitos utilizando ferramentas como Trello, Jira e Microsoft Project
não apenas organiza e rastreia os requisitos de forma eficiente, mas também promove
uma comunicação clara e contínua entre todos os stakeholders. Sommerville (2007)
destaca que uma boa gestão de requisitos é essencial para o sucesso de qualquer pro-
jeto de software, pois ajuda a evitar mal-entendidos e a garantir que as expectativas
dos clientes sejam atendidas.
Além disso, Dennis, Wixom e Roth (2014) enfatizam a importância da priorização dos re-
quisitos para garantir que as funcionalidades mais críticas sejam desenvolvidas primeiro.
Para ilustrar a teoria da análise de requisitos em projetos front-end, apresentaremos
um estudo de caso detalhado de um projeto real de desenvolvimento de um aplicativo
de gestão de tarefas. Este exemplo cobre todas as etapas, desde a coleta até a imple-
mentação dos requisitos, discutindo os desafios enfrentados e como foram superados.
O primeiro passo no projeto foi a coleta de requisitos. A equipe de desenvolvimento
realizou várias reuniões com os stakeholders para entender suas necessidades e ex-
pectativas. Utilizaram-se técnicas como entrevistas, workshops e questionários.




                     1                                                        2


Funcionalidades principais:                    Requisitos não funcionais:
•      Adicionar, editar e excluir tarefas.    •           Interface amigável e intuitiva.
•      Organizar tarefas em listas.            •     Alta performance e rapidez de
•      Definir prazos e lembretes.             resposta.

•      Atribuir prioridades às tarefas.        •           Segurança dos dados dos usuários.

•      Sincronização entre dispositivos.




                                              20
Os requisitos foram documentados no Confluence, divididos em funcionais e não
funcionais. Cada requisito incluiu descrições claras, critérios de aceitação e mockups
de tela. Utilizamos o método MoSCoW para priorizar as funcionalidades, com as es-
senciais marcadas como “Must have”, como adicionar e organizar tarefas. As “Should
have”, como a sincronização entre dispositivos, foram prioritárias também.
O projeto foi dividido em sprints de duas semanas, com “user stories” criadas no Jira
para cada requisito. Monitoramos o progresso com quadros Kanban no Jira. Durante
o desenvolvimento, adicionamos uma nova funcionalidade de integração com calen-
dários externos após reavaliar o backlog e ajustar o cronograma em uma reunião de
planejamento.
Para melhorar a comunicação entre equipe, implementamos reuniões diárias de
stand-up. O Confluence centralizou a documentação do projeto. Durante os testes,
otimizamos a sincronização de tarefas entre dispositivos para resolver problemas de
performance identificados.
Após o desenvolvimento, o aplicativo foi testado por usuários para feedback. Imple-
mentamos sugestões viáveis antes do lançamento em ambiente de produção. A ges-
tão eficaz e comunicação constante superaram desafios como mudanças nos requisi-
tos, dificuldades de comunicação e problemas técnicos.
Essa experiência destaca a importância da coleta detalhada e documentação de re-
quisitos, priorização adequada e uso de ferramentas como Confluence e Jira para o
sucesso do projeto.
Os desafios enfrentados, como mudanças nos requisitos, dificuldades de comunicação
e problemas técnicos, foram superados através de uma gestão eficaz e comunicação
constante entre os membros da equipe. Este exemplo oferece insights valiosos sobre
a gestão de projetos e a resolução de problemas, ilustrando como aplicar os conceitos
teóricos na prática.
A coleta e documentação detalhadas de requisitos, a definição de prioridades apro-
priadas e o uso de ferramentas de gerenciamento como Confluence e Jira são essen-
ciais para o sucesso do projeto.
Os desafios encontrados, como mudanças de requisitos, dificuldades de comunicação
e problemas técnicos, foram superados através de uma gestão eficaz e comunicação
contínua entre os membros da equipe.




                                               Vídeo
                                 Assista ao vídeo clicando aqui




                                          21
A coleta e documentação detalhadas de requisitos, a definição de prioridades apro-
priadas e o uso de ferramentas de gerenciamento como Confluence e Jira são essen-
ciais para o sucesso do projeto.



                                           Leitura
               Nome do livro: Engenharia de software: teoria e prática
               Autor: Shari Lawrence Pfleeger
               Editora: Pearson
               Capítulo: 9
               Ano: 2004
               ISBN: 8587918311
               Comentário: No capítulo 9 do livro “Engenharia de Software: Te-
               oria e Prática”, serão abordados o processo de teste de sistema,
               seus propósitos, etapas, participantes, técnicas e ferramentas.
               Testar um sistema difere dos testes de unidade e integração. Nos
               testes de unidade, há controle total, com criação de dados de tes-
               te e execução de casos pessoalmente. Nos testes de integração,
               pode-se trabalhar independentemente ou em colaboração com
               parte da equipe. No teste de sistema, há colaboração com toda a
               equipe de desenvolvimento para alcançar objetivos coletivos.
               Disponível em: https://plataforma.bvirtual.com.br/Leitor/Publica-
               cao/476/epub/332




Os desafios encontrados, como mudanças de requisitos, dificuldades de comunicação
e problemas técnicos, foram superados através de uma gestão eficaz e comunicação
contínua entre os membros da equipe.




                                         22
Tema 4
    Conceitos de Prototipação




Qual o papel da prototipação no desenvolvimento de projetos Front-End?


Um processo de proteção fundamental que não desenvolve os projetores front-end,
remete algo crucial para uma validação e refinanciamento dessas ideias na implemen-
tação final. Segundo Sommerville (2007), a prototipação envolve a criação de uma
versão simplificada do sistema, que deverá ser utilizada para demonstrar conceitos e
testar funcionalidades. Observe a figura abaixo que mostra estênceis para protótipos
de software em um quadro branco:




    Figura 2.4 - Protótipos de software em um quadro branco
    Fonte: Jan Dittrich (WMDE) / Wikimedia Commons.




                                               23
Este processo permite identificar problemas e coletar feedback dos usuários de ma-
neira antecipada, reduzindo o risco de erros na fase de desenvolvimento.
Existem diferentes níveis de fidelidade na prototipação, sendo os mais comuns os
protótipos de baixa e alta fidelidade.
Protótipos de baixa fidelidade são representações simplificadas do produto final, fei-
tas com esboços em papel ou ferramentas digitais básicas. São rápidos e econômicos,
ideais para testar e iterar ideias no início do projeto, quando os requisitos ainda estão
em definição e mudanças são comuns.
Em contraste, os protótipos de alta fidelidade são mais detalhados, semelhantes ao
produto final em aparência e funcionalidade. Utilizam ferramentas avançadas e po-
dem incluir interatividade, proporcionando uma experiência de usuário mais realista.
Esses protótipos são cruciais para validar o design e funcionalidades antes da imple-
mentação, conforme destacado por Marinho (2016).
Um protótipo de design da interface é um modelo ou simulação de uma tela de um
computador, formulário ou relatório. É preparado um protótipo para cada interface do
sistema a fim de mostrar aos usuários e programadores como o sistema se comportará.

           “Antigamente, um protótipo de design da interface em geral era especificado em
           um formulário em papel, que mostrava o que seria exibido em cada parte da tela.
           Os formulários em papel ainda são usados hoje em dia, mas cada vez mais pro-
           tótipos de design das interfaces estão sendo construídos usando ferramentas de
           computador, em vez de papel” (DENNIS et al., 2014 p. 316).

As três abordagens mais comuns para a prototipagem do design da interface são os
esboços sequenciais (storyboards), protótipos HTML e protótipos de linguagem, Co-
nheça mais sobre elas:




   •    Esboço Sequencial (storyboards): É um esboço em papel mostrando
   imagens das telas e como elas se conectam, como em um desenho animado.
   Requer papel e caneta, sendo simples e acessível artisticamente.
   •      Protótipo HTML: Feito com páginas web em HTML, permite interação
   através de cliques e entrada de dados fictícios, mas não realiza processamen-
   to real. Oferece navegação simulada, sendo adequado para sistemas web,
   apesar de não replicar exatamente as telas finais.
   •      Protótipo em Linguagem: Construído na linguagem ou ferramenta
   final do sistema, permite navegação entre telas sem processar dados. Mais
   demorado de desenvolver, mas mostra as telas como serão no produto final,
   sem necessidade de adivinhar elementos e layout.




                                           24
A prototipação no Front-End traz benefícios significativos: feedback precoce dos
stakeholders e usuários finais, validação de ideias antes da implementação, e melhora
na comunicação entre equipes. Essa prática, diferenciando entre protótipos de baixa
e alta fidelidade, fortalece a criação de produtos alinhados com as necessidades dos
usuários.
No desenvolvimento de projetos Front-End, a escolha da ferramenta de prototipação
pode influenciar significativamente a eficiência e a qualidade do produto final. Exis-
tem diversas ferramentas populares que facilitam a criação de protótipos interativos,
cada uma com suas particularidades e vantagens específicas. Conheça mais sobre
alguns deles:




 Adobe XD: Ferramenta integrada ao
 Adobe Creative Cloud para prototipagem
 de interfaces, destacando-se por protóti-
 pos interativos e colaboração eficaz entre
 designers e desenvolvedores.


                                                    Fonte: Adobe Systems / Wikimedia Commons




 Sketch: Popular entre designers macOS,
 oferece plugins para protótipos interati-
 vos, símbolos reutilizáveis e consistência
 no design digital.



                                                    Fonte: Bohemian Coding / Wikimedia Commons




 Figma: Baseada na nuvem, permite colabo-
 ração em tempo real e prototipagem com
 transições interativas, ideal para feedback
 instantâneo de stakeholders.



                                                    Fonte: Figma / Wikimedia Commons




                                               25
Escolher a ferramenta de prototipagem adequada é fundamental para o êxito no
desenvolvimento de projetos Front-End. Ao considerar os objetivos do projeto, neces-
sidades de colaboração, integrações disponíveis, curva de aprendizado e funcionalida-
des específicas de cada ferramenta, as equipes podem selecionar a opção que melhor
se adapte às suas necessidades.
Utilizar a ferramenta correta não apenas otimiza o processo de design e desenvolvi-
mento, mas também garante a criação de produtos mais eficientes e alinhados com as
expectativas dos usuários.
O processo de prototipagem é iterativo, começando com esboços iniciais que evoluem
para protótipos detalhados. Esta abordagem contínua permite ajustar o design com
base no feedback dos usuários, garantindo que o produto final atenda às expectativas
e necessidades do público-alvo. Conheça as etapas desse processo:




   •      Esboço Inicial: Começa com esboços ou wireframes que delineiam a
   estrutura básica da interface. Eles são rápidos de criar e oferecem uma visão
   geral sem focar em detalhes estéticos. São feitos com papel e caneta.
   •     Protótipo de Baixa Fidelidade: Após os esboços, criam-se protóti-
   pos de baixa fidelidade que adicionam mais detalhes. Podem ser feitos digi-
   talmente ou em papel, testando a navegabilidade básica sem considerar o
   design final. Feedback inicial dos usuários é coletado.
   •     Protótipo de Média Fidelidade: Com o avanço do projeto, criam-se
   protótipos de média fidelidade, oferecendo uma representação mais realis-
   ta com detalhes visuais e interatividade básica. Ferramentas como Figma e
   Adobe XD são usadas. O feedback dos usuários ajusta a navegação e usabili-
   dade.
   •      Protótipo de Alta Fidelidade: Antes da implementação, criam-se pro-
   tótipos de alta fidelidade, detalhados com elementos visuais, animações e
   interações finais. Ferramentas como InVision e Axure RP simulam o compor-
   tamento real. Feedback dos usuários ajuda nos ajustes finais.




Iterar sobre os protótipos com base no feedback dos usuários é uma prática essencial
para o sucesso de qualquer projeto de design. Cada rodada de feedback proporciona
insights valiosos sobre a usabilidade e eficácia do design, permitindo ajustes antes de
avançar para a próxima etapa. Este ciclo de feedback e iteração ajuda a identificar e
corrigir problemas desde cedo, economizando tempo e recursos no longo prazo.
Como afirmam Dennis et al. (2014), a prototipagem permite envolver os usuários
no processo de desenvolvimento, garantindo que suas necessidades e expectativas



                                          26
sejam atendidas. A iteração contínua assegura que o produto final não só atenda aos
requisitos técnicos, mas também ofereça uma experiência de usuário positiva.



                                    Indicação de filme
               Nome: Homem de Ferro (Iron Man)
               Ano: 2008
               Comentário: Embora seja um filme de super-herói, mostra o
               processo de prototipação e iteração na criação da armadura do
               Homem de Ferro, ilustrando conceitos de design iterativo e pro-
               totipagem rápida.
               Para conhecer mais sobre o filme, acesse o trailer disponível em:
               < TRAILER DO FILME Homem de Ferro (DUBLADO)>.



A integração dos protótipos com a equipe de desenvolvimento é crucial para garantir
uma transição suave do design para a implementação. Para isso, é essencial que os
designers e desenvolvedores colaborem estreitamente desde o início do projeto. Uti-
lizar ferramentas que permitem o compartilhamento de protótipos interativos, como
Figma e InVision, facilita a comunicação e o alinhamento entre as equipes.
Uma prática recomendada é envolver os desenvolvedores nas fases iniciais de prototi-
pagem para que eles possam fornecer feedback sobre a viabilidade técnica das solu-
ções propostas. Além disso, manter uma documentação detalhada e atualizada dos
protótipos ajuda a equipe de desenvolvimento a entender o contexto e os objetivos
do design, minimizando mal-entendidos e retrabalhos.
O processo de criação de protótipos, desde os esboços iniciais até os protótipos finais
de alta fidelidade, é essencial para garantir que o produto final atenda às necessida-
des dos usuários e seja tecnicamente viável. A iteração contínua com base no feedba-
ck dos usuários e a integração próxima com a equipe de desenvolvimento são práticas
fundamentais para o sucesso de qualquer projeto de design de interface.




                                          27
    Encerramento




Por que Front-End e Back-End são importantes para a experiência do
usuário?
Resposta da questão norteadora 1

Front-End e Back-End são essenciais para a experiência do usuário porque o Front-End
é responsável pela interface e interatividade, tornando a navegação intuitiva e agra-
dável, enquanto o Back-End garante o funcionamento correto, seguro e eficiente das
funcionalidades, assegurando que a experiência seja fluida e confiável.


Qual a importância de uma análise de requisitos bem-feita para o
sucesso de um projeto de software?
Resposta da questão norteadora 2

Uma análise de requisitos bem-feita é crucial para o sucesso de um projeto de software,
pois garante que o produto final atenda às necessidades dos usuários, evite retrabalho,
reduza custos e minimize riscos de falhas, resultando em maior satisfação do cliente e
eficiência do processo de desenvolvimento.


Como a priorização de requisitos utilizando o método MoSCoW pode
influenciar o sucesso de um projeto Front-End?
Resposta da questão norteadora 3

O método MoSCoW (Must have, Should have, Could have, Won’t have) ajuda a focar
nos requisitos mais críticos primeiro. Isso garante que as funcionalidades essenciais
sejam entregues no prazo, melhorando a satisfação do cliente e o sucesso do projeto.


Qual o papel da prototipação no desenvolvimento de projetos Front-End?
Resposta da questão norteadora 4

A prototipação desempenha um papel fundamental no desenvolvimento de projetos
Front-End ao permitir a validação precoce de ideias, identificar problemas e coletar
feedback dos usuários antes da implementação final. Isso ajuda a garantir que o pro-
duto final atenda às expectativas e necessidades do público-alvo de maneira eficiente.




                                          28
Resumo da unidade

   Chegamos ao fim de nossa jornada sobre análise de requisitos para proje-
   tos front-end. Espero que agora você tenha uma visão clara da importância
   desse processo e das melhores práticas para aplicá-lo em seus projetos. Mas
   antes de nos despedirmos, vamos fazer uma breve reflexão sobre o que
   aprendemos e como essas informações podem impactar seu trabalho futuro.
   Ao longo deste material, vimos como a análise de requisitos não é apenas
   uma etapa inicial, mas um alicerce que sustenta todo o desenvolvimento
   de um projeto front-end. Lembre-se de que compreender profundamente
   as necessidades dos usuários e dos clientes é o que diferencia um projeto
   mediano de um projeto de sucesso. A partir dessa compreensão, você pode
   criar interfaces que não apenas atendem às expectativas, mas também pro-
   porcionam uma experiência de uso intuitiva e agradável.
   Também exploramos a importância de documentar esses requisitos de ma-
   neira clara e detalhada. Esse passo é fundamental para garantir que todos
   os envolvidos no projeto tenham uma visão alinhada e possam trabalhar de
   maneira coesa. E não se esqueça das ferramentas de prototipação! Utilizá-las
   pode economizar tempo e esforço, permitindo testar e ajustar designs antes
   de partir para a implementação final.
   Agora, é hora de aplicar esse conhecimento em seus projetos. Comece pe-
   queno, talvez com um projeto pessoal ou um exercício prático. A prática
   constante ajudará a consolidar esses conceitos e a desenvolver suas habi-
   lidades. Lembre-se de que a análise de requisitos é um processo iterativo:
   quanto mais você pratica, melhor você se torna em identificar e atender às
   necessidades de seus usuários.
   Além disso, fique atento às tendências e inovações no campo do desenvol-
   vimento front-end. Novas ferramentas e metodologias estão sempre sur-
   gindo, e estar atualizado pode oferecer vantagens competitivas e aprimorar
   ainda mais a qualidade de seus projetos.
   Participe de comunidades, leia artigos, e nunca pare de aprender, e lembre-se: um
   projeto bem planejado é meio caminho andado para o sucesso. Até a próxima!




                                     29
Referências
 SOMMERVILLE, Ian. Engenharia de software. 8. ed. São Paulo: Pearson,
 2007. E-book. Disponível em: https://plataforma.bvirtual.com.br. Acesso em:
 27 maio 2024.

 DENNIS, Alan; WIXOM, Barbara H.; ROTH, Roberta M. Análise e Projeto de
 Sistemas: Grupo GEN, 2014. E-book. ISBN 978-85-216-2634-3. Disponível
 em: https://integrada.minhabiblioteca.com.br/#/books/978-85-216-2634-3/.
 Acesso em: 27 mai. 2024.

 MARINHO, Antonio Lopes. Análise e modelagem de sistemas. São Paulo:
 Pearson, 2016. E-book. Disponível em: https://plataforma.bvirtual.com.br.
 Acesso em: 27 maio 2024.
Até a próxima unidade!
