# Modelagem UML para sistema de empréstimos e reservas de biblioteca universitária — versão textual alternativa

> Versão derivada do arquivo de texto preexistente. Ela difere do documento Word e do PDF preservados localmente; não há informação suficiente para determinar qual versão é mais recente. Identificadores pessoais da capa foram removidos.

## 1 INTRODUÇÃO

A automação do controle de empréstimos e reservas em uma biblioteca universitária exige um modelo de análise capaz de representar, de forma clara e consistente, as entidades envolvidas, seus relacionamentos e o fluxo de eventos do sistema. A linguagem UML (Unified Modeling Language) é amplamente utilizada para essa finalidade, por oferecer diagramas que descrevem tanto a estrutura estática (como o diagrama de classes) quanto o comportamento dinâmico (diagramas de estados e de sequência) de um sistema de informação (BOOCH; RUMBAUGH; JACOBSON, 2012; SOMMERVILLE, 2018).

Além da representação gráfica dos componentes, o desenvolvimento de trabalhos acadêmicos requer atenção às normas da Associação Brasileira de Normas Técnicas (ABNT), em especial às normas de apresentação de trabalhos acadêmicos (NBR 14724), de citações em documentos (NBR 10520) e de referências bibliográficas (NBR 6023). Nesse contexto, este texto descreve e justifica as decisões de modelagem para o cenário proposto, articulando as escolhas conceituais com a disciplina Paradigmas de Programação e destacando o papel dos princípios da orientação a objetos na estruturação de sistemas computacionais (SEBESTA, 2018).

## 2 DIAGRAMA DE CLASSES DO SISTEMA DE BIBLIOTECA

O diagrama de classes, no contexto de análise de sistemas orientados a objetos, representa as principais entidades do domínio, seus atributos, operações e relacionamentos, constituindo a visão estrutural do sistema (BOOCH; RUMBAUGH; JACOBSON, 2012). Para o cenário da biblioteca universitária em processo de automação, as classes centrais consideradas são: Bibliotecária, Usuário (com subclasses Aluno, Professor e Funcionário Administrativo), Publicação (com subclasses Livro e Periódico), Exemplar, Empréstimo e Reserva (LARMAN, 2007).

A classe Bibliotecária é responsável pelas operações de empréstimo, devolução, registros de reserva e baixa de acervo, possuindo como atributos principais matrícula, nome e e-mail, que permitem a identificação da profissional responsável pelas transações realizadas no sistema. Essa classe se relaciona com Empréstimo e Reserva, de modo que cada operação registra qual bibliotecária a executou, garantindo rastreabilidade e responsabilidade sobre o atendimento ao usuário.

A classe Usuário é generalizada em três subclasses: Aluno, Professor e FuncionárioAdministrativo, refletindo a regra de negócio de que diferentes perfis podem utilizar os serviços da biblioteca, desde que previamente cadastrados. A classe abstrata Usuário inclui atributos comuns como matrícula, nome, data de nascimento, endereço, telefone e e-mail, enquanto Aluno adiciona o atributo curso, Professor adiciona titulação e FuncionárioAdministrativo adiciona setor, de acordo com as informações exigidas no cadastro de cada tipo de usuário.

A classe Publicação representa, de forma genérica, os materiais disponíveis para empréstimo e reserva, contendo atributos como título, valorAtualEmprestimo e valorRessarcimento, este último utilizado para calcular o valor a ser cobrado em caso de não devolução ou dano ao exemplar. Publicação é especializada em Livro e Periódico: Livro possui atributos específicos como autor, ISBN, editora e edição, enquanto Periódico inclui volumeNumero, ISSN e dataPublicacao, refletindo a diferenciação entre obras de natureza bibliográfica distinta.

Cada Publicação possui um ou mais Exemplares associados, representando as cópias físicas existentes na biblioteca; assim, no diagrama de classes estabelece-se uma associação de 1 para muitos entre Publicação e Exemplar, indicando que uma publicação deve possuir pelo menos um exemplar para ser emprestada. A classe Exemplar inclui atributos como identificadorExemplar (ou número de tombo), status e, opcionalmente, datas de aquisição e baixa, sendo o status modelado como um tipo enumerado com os valores Disponível, Emprestado, Reservado, Bloqueado e BaixadoDoAcervo, conforme descrito no cenário.

A classe Empréstimo representa a operação de retirada de exemplares pelo usuário, devendo registrar o vínculo com um único Usuário, uma única Bibliotecária e entre um e três Exemplares, de acordo com a regra de que o empréstimo contempla de 1 a 3 exemplares de diferentes publicações. Entre os atributos de Empréstimo destacam-se idEmprestimo, dataEmprestimo, dataPrevistaDevolucao e valorTotalEmprestimo, calculado com base nos valores de empréstimo das publicações envolvidas, além de operações como registrarDevolucao(), calcularValor() e verificarAtraso() (LARMAN, 2007).

A classe Reserva modela o processo pelo qual o usuário solicita exemplares para futura retirada, respeitando o limite de, no máximo, três exemplares diferentes por reserva. Reserva possui atributos como idReserva, dataReserva, dataLimiteEmprestimo e statusReserva, sendo este último um tipo enumerado com os estados Pendente, Cancelada e Confirmada, o que permite ao sistema acompanhar a evolução das reservas. O relacionamento de Reserva com Usuário indica que um usuário pode possuir várias reservas, e o relacionamento com Exemplar pode ser modelado como associação para até três exemplares por reserva, conforme a regra de negócio.

As operações de manutenção de acervo (como baixarExemplarPorPerda() e avaliarEstadoConservacao()) podem ser modeladas como métodos de Bibliotecária ou de uma eventual classe de controle AcervoBiblioteca, mas, conceitualmente, são implementadas por meio de alterações no estado dos objetos da classe Exemplar e atualização das coleções de publicações disponíveis.

## 3 DIAGRAMA DE TRANSIÇÃO DE ESTADO DA CLASSE EXEMPLAR

O diagrama de estados da classe Exemplar descreve o ciclo de vida de um exemplar dentro da biblioteca, explicitando os estados possíveis e as transições disparadas por eventos do sistema e decisões de negócio. A utilização de diagramas de estados é recomendada sempre que uma classe apresenta mudanças significativas de comportamento em função de seu estado interno, como ocorre com itens de acervo sujeitos a empréstimos, reservas, bloqueios e baixas (BOOCH; RUMBAUGH; JACOBSON, 2012; PRESSMAN; MAXIM, 2016).

No cenário analisado, os principais estados do Exemplar são: Disponível, Emprestado, Reservado, Bloqueado e BaixadoDoAcervo, ainda que BaixadoDoAcervo represente uma condição de saída do sistema, decorrente da perda de acervo. O estado Disponível representa a situação em que o exemplar pode ser emprestado de imediato; a transição de Disponível para Emprestado ocorre quando um empréstimo é registrado para aquele exemplar, após verificação de que o usuário está cadastrado e apto a utilizar o serviço.

Do estado Emprestado, o exemplar retorna ao estado Disponível quando ocorre a devolução dentro das condições normais, isto é, sem reserva pendente para a publicação e sem dano que impeça novos empréstimos. Caso exista reserva cadastrada para a publicação correspondente no momento da devolução, o exemplar transita de Emprestado para Bloqueado, permanecendo indisponível até que a reserva seja confirmada ou cancelada pelo sistema.

O estado Reservado representa a situação em que o sistema associa um exemplar específico a uma reserva pendente, por exemplo, quando o usuário solicita reserva e um exemplar é destacado para ser emprestado assim que se tornar disponível. Nesse caso, a transição de Disponível para Reservado ocorre quando a reserva é registrada e o exemplar é vinculado a essa reserva, enquanto a transição de Reservado para Emprestado ocorre quando a reserva é confirmada e o empréstimo efetivamente realizado.

O estado Bloqueado é acionado em situações nas quais o exemplar não pode ser emprestado temporariamente, embora ainda faça parte do acervo ativo, como quando há reserva pendente que deve ser atendida antes de qualquer novo empréstimo. A transição de Bloqueado para Emprestado ocorre quando a reserva correspondente é confirmada e convertida em empréstimo, enquanto a transição de Bloqueado para Disponível ocorre quando a reserva é cancelada ou perde a validade sem que o empréstimo seja realizado.

Finalmente, o estado BaixadoDoAcervo representa a perda de acervo descrita nas regras do sistema: exemplares não devolvidos até noventa dias após o empréstimo, exemplares considerados inutilizáveis devido ao estado de conservação e exemplares não utilizados em empréstimos nos últimos cinco anos são baixados do acervo. As transições para BaixadoDoAcervo ocorrem, portanto, a partir de estados como Emprestado, Disponível, Reservado ou Bloqueado, em resposta a eventos de controle periódico realizados pelos funcionários da biblioteca ou por rotinas automatizadas do sistema de gestão da biblioteca (PRESSMAN; MAXIM, 2016).

## 4 DIAGRAMA DE SEQUÊNCIA DO MÓDULO REALIZAR EMPRÉSTIMO DE EXEMPLAR

O diagrama de sequência, na UML, modela a interação temporal entre objetos e atores, evidenciando a ordem das mensagens trocadas para realizar uma operação específica do sistema (LARMAN, 2007; SOMMERVILLE, 2018). No módulo "Realizar empréstimo de exemplar", os principais participantes são: Usuário (ator externo), Bibliotecária (ator responsável pelo atendimento), SistemaBiblioteca (objeto controlador), além das instâncias de Exemplar, Publicação e Empréstimo envolvidas na transação.

O fluxo básico pode ser descrito como segue: inicialmente, o Usuário escolhe, na própria biblioteca, entre um e três Exemplares disponíveis de diferentes publicações e dirige-se ao balcão de atendimento. A Bibliotecária envia ao SistemaBiblioteca uma solicitação contendo a identificação do usuário, e o sistema verifica se o cadastro existe e se o usuário está apto a realizar empréstimos, por exemplo, não estando suspenso ou com pendências cadastrais ou financeiras.

Para cada exemplar apresentado, a Bibliotecária solicita ao SistemaBiblioteca a operação de empréstimo, informando o identificador do exemplar e o usuário; o sistema consulta a classe Exemplar para verificar se o status atual é Disponível e consulta a Publicação correspondente para obter o valorAtualEmprestimo, necessário ao cálculo do valor total. Caso todos os exemplares estejam disponíveis e o usuário esteja regular, o SistemaBiblioteca cria uma instância da classe Empréstimo, associando o usuário, a bibliotecária e os exemplares, definindo a dataEmprestimo, a dataPrevistaDevolucao e o valorTotalEmprestimo, conforme as regras de negócio.

Após o registro do empréstimo, o sistema atualiza o status de cada Exemplar de Disponível para Emprestado e registra a operação de pagamento do valor do empréstimo, efetuado pelo usuário junto à bibliotecária, seja por meio de uma classe específica de Pagamento ou por um atributo de controle no próprio Empréstimo. Finalmente, o SistemaBiblioteca confirma o sucesso da operação à bibliotecária, que comunica ao usuário a conclusão do empréstimo e entrega fisicamente os exemplares, encerrando o fluxo de sequência para o caso de uso principal (LARMAN, 2007).

## 5 RELAÇÃO COM PARADIGMAS DE PROGRAMAÇÃO

A disciplina Paradigmas de Programação estuda diferentes formas de organizar e estruturar programas, como os paradigmas imperativo, funcional, orientado a objetos e lógico, entre outros (SEBESTA, 2018). A modelagem orientada a objetos realizada por meio de diagramas da UML mantém relação direta com os conceitos fundamentais da programação orientada a objetos, tais como abstração, encapsulamento, herança, polimorfismo e troca de mensagens entre componentes de software.

No diagrama de classes, a generalização de Usuário em Aluno, Professor e FuncionárioAdministrativo e de Publicação em Livro e Periódico exemplifica o mecanismo de herança e especialização, permitindo que atributos e comportamentos comuns sejam definidos na superclasse e reaproveitados pelas subclasses, reduzindo duplicações e facilitando a manutenção do código (SEBESTA, 2018). Além disso, o encapsulamento assegura que os atributos internos dos exemplares e das transações financeiras sejam acessados e modificados apenas por meio de métodos específicos, protegendo o estado dos objetos contra alterações indevidas (LARMAN, 2007).

A dinâmica apresentada nos diagramas de estados e de sequência também ilustra o modelo computacional orientado a objetos. No diagrama de estados, a resposta de um exemplar a eventos depende do seu estado interno atual, refletindo a natureza reativa dos objetos em tempo de execução. Já no diagrama de sequência, a execução do empréstimo é estruturada como uma troca coordenada de mensagens entre atores e objetos autônomos, conectando a análise conceitual às práticas de implementação ensinadas na disciplina (SEBESTA, 2018).

## 6 CONSIDERAÇÕES FINAIS

A combinação de diagrama de classes, diagrama de estados e diagrama de sequência permite representar o sistema de automação da biblioteca sob perspectivas complementares, aumentando a compreensão do domínio e reduzindo a probabilidade de erros na etapa de implementação. O diagrama de classes organiza as entidades e relacionamentos essenciais; o diagrama de estados da classe Exemplar explicita o ciclo de vida dos itens de acervo; e o diagrama de sequência da operação de empréstimo detalha o fluxo de mensagens entre atores e objetos, alinhando as regras de negócio à implementação futura (BOOCH; RUMBAUGH; JACOBSON, 2012; LARMAN, 2007).

Sob a perspectiva dos paradigmas de desenvolvimento de software, a modelagem analisada evidencia a importância de fundamentar a construção de aplicações em princípios sólidos de engenharia e programação orientada a objetos, assegurando coesão, rastreabilidade e manutenibilidade (SOMMERVILLE, 2018; SEBESTA, 2018). Dessa forma, o trabalho integra a prática de modelagem de sistemas com as diretrizes acadêmicas e normas vigentes da ABNT, proporcionando uma visão consistente e profissional do processo de desenvolvimento de software.

## Referências

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. ABNT NBR 6023: Informação e documentação — Referências — Elaboração. Rio de Janeiro: ABNT, 2018.

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. ABNT NBR 10520: Informação e documentação — Citações em documentos — Apresentação. Rio de Janeiro: ABNT, 2023.

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. ABNT NBR 14724: Informação e documentação — Trabalhos acadêmicos — Apresentação. Rio de Janeiro: ABNT, 2011.

BOOCH, Grady; RUMBAUGH, James; JACOBSON, Ivar. UML: guia do usuário. 2. ed. Rio de Janeiro: Elsevier, 2012.

LARMAN, Craig. Utilizando UML e padrões: uma introdução à análise e ao projeto orientados a objetos e ao desenvolvimento iterativo. 3. ed. Porto Alegre: Bookman, 2007.

PRESSMAN, Roger S.; MAXIM, Bruce R. Engenharia de software: uma abordagem profissional. 8. ed. Porto Alegre: AMGH, 2016.

SEBESTA, Robert W. Conceitos de linguagens de programação. 11. ed. Porto Alegre: Bookman, 2018.

SOMMERVILLE, Ian. Engenharia de software. 10. ed. São Paulo: Pearson, 2018.
