# Modelagem UML para o sistema de empréstimos e reservas de biblioteca universitária

> Paradigmas de Programação · AVA 2. Conversão textual do documento Word; nome completo e matrícula removidos.

## 1 INTRODUÇÃO

A automação do controle de empréstimos e reservas em uma biblioteca universitária exige um modelo de análise capaz de representar, de forma clara e consistente, as entidades envolvidas, seus relacionamentos e o fluxo de eventos do sistema. A linguagem UML (Unified Modeling Language) é amplamente consagrada na engenharia de software para essa finalidade, por oferecer visões complementares que descrevem tanto a estrutura estática (como o diagrama de classes) quanto a dinâmica comportamental (diagramas de estados e de sequência) de um sistema de informação (BOOCH; RUMBAUGH; JACOBSON, 2012; SOMMERVILLE, 2018).

Além da representação gráfica rigorosa dos componentes, o desenvolvimento de trabalhos acadêmicos requer atenção estrita às normas da Associação Brasileira de Normas Técnicas (ABNT), em especial às diretrizes de apresentação de trabalhos acadêmicos (NBR 14724), de citações em documentos (NBR 10520) e de elaboração de referências bibliográficas (NBR 6023). Nesse contexto, o presente relatório descreve e fundamenta as decisões de modelagem adotadas para o cenário de biblioteca universitária proposto na atividade AVA 2, articulando as escolhas conceituais com os princípios estudados na disciplina Paradigmas de Programação, com ênfase no paradigma orientado a objetos e suas estruturas fundamentais (SEBESTA, 2018).

## 2 DIAGRAMA DE CLASSES DO SISTEMA DE BIBLIOTECA

O diagrama de classes, no contexto de análise de sistemas orientados a objetos, representa as principais entidades do domínio, seus atributos, operações e relacionamentos, constituindo a espinha dorsal da visão estrutural da aplicação (BOOCH; RUMBAUGH; JACOBSON, 2012). Para o cenário da biblioteca universitária em processo de automação, as classes centrais modeladas são: Bibliotecária, Usuário (com as especializações Aluno, Professor e FuncionárioAdministrativo), Publicação (com subclasses Livro e Periódico), Exemplar, Empréstimo e Reserva (LARMAN, 2007).

A classe Bibliotecária centraliza o atendimento e a gestão operacional da biblioteca, sendo responsável pelo registro de empréstimos, devoluções, controle de reservas e baixa de materiais danificados ou obsoletos. Seus atributos principais englobam matrícula, nome e e-mail institucional, assegurando a rastreabilidade e a responsabilização dos operadores em cada movimentação realizada no acervo.

A classe Usuário é modelada como uma generalização abstrata, originando três subclasses concretas: Aluno (que adiciona o atributo curso), Professor (que adiciona titulação acadêmica) e FuncionárioAdministrativo (que incorpora o atributo setor de lotação). Os atributos comuns a todos os usuários — matrícula, nome completo, data de nascimento, endereço residencial, telefone e e-mail — residem na superclasse Usuário, promovendo reutilização de código e aderência ao princípio do reaproveitamento na orientação a objetos.

A classe Publicação representa o conceito genérico dos títulos disponíveis no acervo, contendo atributos como título, valorAtualEmprestimo e valorRessarcimento, este último aplicado no cálculo de indenizações por perda definitiva ou avaria material do item. A classe Publicação especializa-se em Livro (com atributos próprios: autor, ISBN, editora e número da edição) e Periódico (com volume/número, ISSN e periodicidade/data de publicação). Cada Publicação vincula-se a uma coleção de um ou mais Exemplares (relação de composição 1 para 1..\*), refletindo as cópias físicas existentes no depósito da biblioteca.

A classe Exemplar materializa as unidades físicas emprestáveis e possui identificadorExemplar (número de tombo), datas de aquisição e de baixa, e o atributo status, modelado por meio de um tipo enumerado (StatusExemplar) que assume os valores Disponível, Emprestado, Reservado, Bloqueado e BaixadoDoAcervo. A classe Empréstimo gerencia as transações de retirada, vinculando um Usuário, uma Bibliotecária e uma coleção de 1 a 3 Exemplares de publicações distintas, conforme exigência de negócio, contendo idEmprestimo, dataEmprestimo, dataPrevistaDevolucao, valorTotalEmprestimo e métodos de controle financeiro e prazos. Por fim, a classe Reserva gerencia a fila de espera para retirada futura de até 3 exemplares, monitorando prazos limites e seu ciclo de confirmação.

**Figura 1 –** Diagrama de Classes do Sistema de Biblioteca Universitária

**Figura (texto Mermaid):** Diagrama de classes do sistema de biblioteca — [diagramas/diag_classes.md](diagramas/diag_classes.md)

Fonte: Elaborado pelo autor (2026), gerado via PlantUML com base no cenário proposto.

Como ilustrado na Figura 1, as associações e multiplicidades expressam com fidelidade os limites impostos pelas regras de negócio: a cardinalidade 1..3 entre Empréstimo/Reserva e Exemplar garante a restrição quantitativa de itens por transação, enquanto as heranças de Usuário e Publicação garantem extensibilidade e polimorfismo estrutural.

## 3 DIAGRAMA DE TRANSIÇÃO DE ESTADO DA CLASSE EXEMPLAR

O diagrama de máquinas de estados descreve o ciclo de vida dinâmico de um objeto ao longo do tempo, especificando as respostas e mutações de seu estado interno disparadas por eventos externos ou transições temporais (BOOCH; RUMBAUGH; JACOBSON, 2012; PRESSMAN; MAXIM, 2016). Na gestão de uma biblioteca universitária, a classe Exemplar possui alta relevância comportamental, uma vez que suas operações dependem diretamente da condição em que o item físico se encontra no acervo.

No modelo desenvolvido, o ciclo de vida do Exemplar inicia no estado Disponível logo após o seu cadastramento no acervo. Ao ser selecionado para uma retirada por usuário regular, transita para o estado Emprestado via evento realizarEmprestimo(). No ato da devolução (evento devolverExemplar()), caso não haja reserva concorrente para a publicação e o material esteja íntegro, o exemplar retorna ao estado Disponível. Havendo reserva pendente de outro usuário, o sistema direciona o exemplar imediatamente ao estado Bloqueado, impedindo novo empréstimo no balcão até que o usuário prioritário venha retirá-lo ou o prazo expire.

O estado Reservado ocorre quando o exemplar é associado diretamente a uma solicitação de reserva pendente no balcão. Do estado Reservado ou Bloqueado, a confirmação de retirada conduz o item para Emprestado; por outro lado, se a reserva for cancelada ou o prazo limite expirar sem manifestação do interessado, o exemplar é reincorporado ao acervo ativo no estado Disponível.

Por fim, o estado terminal BaixadoDoAcervo representa a eliminação definitiva do exemplar do catálogo ativo em conformidade com as três regras de descarte do cenário: perda por atraso não devolvido superior a 90 dias (disparado a partir de Emprestado), inutilização física por dano ou deterioração severa de conservação (a partir de Disponível ou Bloqueado) e inatividade de empréstimo prolongada superior a 5 anos (a partir de Disponível).

**Figura 2 –** Diagrama de Transição de Estados da Classe Exemplar

**Figura (texto Mermaid):** Diagrama de transição de estados da classe Exemplar — [diagramas/diag_estados.md](diagramas/diag_estados.md)

Fonte: Elaborado pelo autor (2026), gerado via PlantUML.

A Figura 2 evidencia que todas as transições respeitam invariantes do domínio, garantindo que nenhum item extraviado permaneça indefinidamente listado como passível de reserva e assegurando que exemplares devolvidos com reserva pendente fiquem bloqueados de forma determinística.

## 4 DIAGRAMA DE SEQUÊNCIA DO MÓDULO REALIZAR EMPRÉSTIMO DE EXEMPLAR

O diagrama de sequência é a representação dinâmica por excelência para demonstrar a colaboração temporal e a troca de mensagens ordenada entre atores e instâncias de objetos em um determinado caso de uso (LARMAN, 2007; SOMMERVILLE, 2018). Para o caso de uso 'Realizar empréstimo de exemplar', participam da interação o ator Usuário, a mediadora Bibliotecária, o controlador SistemaBiblioteca e as instâncias de domínio Usuario, Exemplar, Publicacao e Emprestimo.

O fluxo interativo se inicia quando o Usuário comparece ao balcão com até 3 exemplares desejados e apresenta sua identificação. A Bibliotecária aciona a operação iniciarEmprestimo no sistema. O controlador valida preliminarmente a regularidade cadastral e financeira do usuário perante a entidade Usuario (verificando ausência de bloqueios por atrasos prévios ou multas pendentes).

Em seguida, o sistema itera sobre cada exemplar selecionado (bloco loop de 1 a 3 itens): consulta a instância Exemplar para confirmar se o status é rigorosamente Disponivel e consulta a classe Publicacao associada para recuperar o valorAtualEmprestimo. Com todas as checagens bem-sucedidas, o controlador instancia o objeto Emprestimo, vinculando os participantes e calculando o montante consolidado da transação.

Na sequência, o sistema atualiza em cascata o status de cada exemplar para Emprestado e solicita à Bibliotecária o recebimento do valor correspondente. Uma vez efetuado o pagamento pelo usuário e confirmada a quitação no sistema, o comprovante formal é emitido e impresso, encerrando o fluxo com a entrega material dos livros ao usuário.

**Figura 3 –** Diagrama de Sequência do Caso de Uso Realizar Empréstimo

**Figura (texto Mermaid):** Diagrama de sequência do empréstimo de exemplar — [diagramas/diag_sequencia.md](diagramas/diag_sequencia.md)

Fonte: Elaborado pelo autor (2026), gerado via PlantUML.

A representação sequencial apresentada na Figura 3 destaca o desacoplamento de responsabilidades entre interface, controle e domínio de dados, mitigando inconsistências de estoque concorrente e consolidando as operações contábeis da biblioteca.

## 5 RELAÇÃO COM PARADIGMAS DE PROGRAMAÇÃO

A disciplina Paradigmas de Programação investiga as diferentes abordagens e modelos conceituais subjacentes à formulação de soluções computacionais, contrapondo paradigmas imperativo, declarativo, funcional, orientado a objetos e concorrente (SEBESTA, 2018). A modelagem orientada a objetos realizada por meio da UML expressa com exatidão os pilares fundamentais da Programação Orientada a Objetos (POO): abstração, encapsulamento, hierarquia de tipos (herança) e polimorfismo.

Na estrutura estática do sistema de biblioteca, o princípio da herança e da generalização é evidenciado quando a classe abstrata Usuario transfere seus atributos e operações basilares às subclasses Aluno, Professor e FuncionarioAdministrativo. Essa modelagem promove polimorfismo, pois métodos de cobrança de taxas de empréstimo ou limites diferenciados podem ser sobrescritos especificamente para cada perfil acadêmico sem impactar a arquitetura unificada do sistema (LARMAN, 2007). Da mesma forma, o encapsulamento é estritamente resguardado na classe Exemplar, cujos atributos de status e histórico de circulação somente sofrem mutação por meio de métodos autorizados, garantindo integridade de estado.

Nos diagramas dinâmicos, observa-se a essência da computação orientada a objetos: programas vistos não como listas lineares de comandos imperativos soltos, mas como uma comunidade colaborativa de objetos autônomos que interagem e reagem por meio do envio síncrono de mensagens e manutenção de seus estados internos (SEBESTA, 2018). Dessa maneira, o projeto de software conecta a modelagem conceitual à implementação em linguagens modernas de mercado (como Java, C# ou TypeScript).

## 6 CONSIDERAÇÕES FINAIS

A construção integrada do Diagrama de Classes, Diagrama de Transição de Estados e Diagrama de Sequência proporcionou uma visão completa e multifacetada das regras de negócio do sistema de empréstimos e reservas da biblioteca universitária. Enquanto a visão estrutural mapeou a taxonomia de classes e associações quantitativas, a visão dinâmica delimitou os ciclos operacionais de conservação e circulação do acervo físico e o passo a passo da interação entre atores e sistema.

O alinhamento da modelagem com os princípios de Paradigmas de Programação comprova que uma análise orientada a objetos criteriosa minimiza refatorações onerosas no ciclo de vida de desenvolvimento, gerando bases sólidas para a escrita de código legível, coeso e desacoplado. Ademais, a formatação deste relatório em estrita observância às normas da ABNT (NBR 14724, NBR 10520 e NBR 6023) confere rigor científico e excelência acadêmica à atividade proposta.

## Referências

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. **ABNT NBR 6023: Informação e documentação — Referências — Elaboração**. Rio de Janeiro: ABNT, 2018.

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. **ABNT NBR 10520: Informação e documentação — Citações em documentos — Apresentação**. Rio de Janeiro: ABNT, 2023.

ASSOCIAÇÃO BRASILEIRA DE NORMAS TÉCNICAS. **ABNT NBR 14724: Informação e documentação — Trabalhos acadêmicos — Apresentação**. Rio de Janeiro: ABNT, 2011.

BOOCH, Grady; RUMBAUGH, James; JACOBSON, Ivar. **UML: guia do usuário**. 2. ed. Rio de Janeiro: Elsevier, 2012.

LARMAN, Craig. **Utilizando UML e padrões: uma introdução à análise e ao projeto orientados a objetos e ao desenvolvimento iterativo**. 3. ed. Porto Alegre: Bookman, 2007.

OBJECT MANAGEMENT GROUP (OMG). **OMG Unified Modeling Language (OMG UML): Version 2.5.1**. Needham: OMG, 2017. Disponível em: https://www.omg.org/spec/UML/2.5.1/. Acesso em: 08 set. 2026.

PRESSMAN, Roger S.; MAXIM, Bruce R. **Engenharia de software: uma abordagem profissional**. 8. ed. Porto Alegre: AMGH, 2016.

SEBESTA, Robert W. **Conceitos de linguagens de programação**. 11. ed. Porto Alegre: Bookman, 2018.

SOMMERVILLE, Ian. **Engenharia de software**. 10. ed. São Paulo: Pearson, 2018.
