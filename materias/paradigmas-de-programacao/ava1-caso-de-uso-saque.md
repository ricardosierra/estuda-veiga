# Descrição de caso de uso CSU01: realizar saque bancário em caixa eletrônico

> Paradigmas de Programação · AVA 1. Conversão textual do documento Word; nome completo e matrícula removidos.

## 1 INTRODUÇÃO

A documentação de casos de uso é uma prática fundamental na engenharia de software, pois descreve de forma estruturada as interações entre os atores envolvidos e a aplicação, permitindo que requisitos funcionais e regras de negócio sejam corretamente compreendidos e implementados. Essa descrição auxilia a comunicação entre analistas, desenvolvedores e clientes e reduz ambiguidades ao longo do desenvolvimento e da manutenção de sistemas de informação (SOMMERVILLE, 2018; LARMAN, 2007).

No contexto de serviços bancários, o caso de uso "Realizar saque bancário" em caixa eletrônico constitui um exemplo de interação entre usuário e sistema, envolvendo validação de credenciais, verificação de saldo, aplicação de regras de negócio e tratamento de falhas. Este trabalho tem como objetivo apresentar a descrição do caso de uso CSU01 – Realizar saque bancário, associado ao requisito funcional RF01 e à regra de negócio RN01, especificando fluxo principal, fluxos alternativos e fluxos de exceção da operação de saque em caixa eletrônico.

Além de enfatizar o papel da documentação de casos de uso na análise orientada a objetos, o trabalho estabelece um diálogo com a disciplina Paradigmas de Programação, destacando como diferentes paradigmas influenciam a modelagem e a implementação dos comportamentos descritos em casos de uso (SEBESTA, 2018).

## 2 DESCRIÇÃO DO CASO DE USO CSU01 – REALIZAR SAQUE BANCÁRIO

### 2.1 Cabeçalho do caso de uso

**Nome do caso de uso: CSU01 – Realizar saque bancário**

**Ator principal: Cliente do banco (autor da operação de saque)**

**Atores secundários: Sistema de caixa eletrônico (aplicação), sistema bancário central**

**Requisito funcional atendido: RF01 – Permitir que o cliente realize saque em dinheiro em sua conta corrente por meio de caixa eletrônico**

**Regra de negócio relacionada: RN01 – O saque será autorizado apenas se houver valor disponível na conta corrente do cliente**

**Objetivo:**

Permitir que o cliente retire um determinado valor em dinheiro de sua conta corrente, utilizando um caixa eletrônico bancário, mediante validação de cartão, verificação de senha pessoal e confirmação de saldo disponível, registrando adequadamente a transação no sistema bancário (LARMAN, 2007).

**Pré-condições:**

- O cliente possui cartão bancário válido e ativo.

- O caixa eletrônico encontra-se em operação e conectado ao sistema bancário central.

- A conta corrente do cliente apresenta saldo suficiente para o valor pretendido, em conformidade com a regra de negócio RN01.

**Pós-condições:**

- Em caso de sucesso, o valor solicitado é debitado da conta corrente do cliente, as cédulas são entregues pelo caixa eletrônico e a transação é registrada no sistema bancário.

- Em caso de falha ou exceção, nenhum valor é debitado da conta, o motivo da não realização é comunicado ao cliente e os registros necessários são armazenados para auditoria e manutenção.

### 2.2 Fluxo principal de eventos

O fluxo principal descreve a sequência típica na qual o caso de uso é executado com sucesso, sem ocorrência de erros ou condições excepcionais relevantes (LARMAN, 2007).

1.  O cliente aproxima-se do caixa eletrônico e insere o cartão bancário no leitor do equipamento.

2.  O sistema de caixa eletrônico reconhece o cartão, exibe a tela inicial e solicita a digitação da senha pessoal.

3.  O cliente informa a senha utilizando o teclado do caixa eletrônico.

4.  O sistema valida a senha por meio de consulta ao sistema bancário central, verificando sua correção e o status da conta corrente associada ao cartão.

5.  Após a validação da senha, o sistema apresenta o menu de opções de transações, incluindo "Saque", "Consulta de saldo" e outras operações disponíveis.

6.  O cliente seleciona a opção "Saque" no menu exibido pelo sistema.

7.  O sistema solicita que o cliente informe o valor desejado para o saque.

8.  O cliente digita o valor pretendido utilizando o teclado do caixa eletrônico.

9.  O sistema verifica a disponibilidade de saldo na conta corrente do cliente, aplicando a regra RN01, e avalia se há cédulas suficientes no caixa eletrônico para o valor informado.

10. Estando todas as condições satisfeitas, o sistema debita o valor correspondente na conta corrente do cliente, registra a transação de saque e autoriza a liberação das cédulas.

11. O caixa eletrônico libera as cédulas referentes ao valor solicitado e apresenta ao cliente a opção de emissão de comprovante da operação.

12. O sistema encerra a operação, exibe mensagem de finalização, ejeta o cartão do cliente e aguarda a retirada do cartão para retornar à tela inicial.

### 2.3 Fluxos alternativos

Os fluxos alternativos representam caminhos legítimos que o caso de uso pode seguir em função de decisões do cliente ou de condições normais de operação, sem configurar falha grave, mas alterando a sequência originalmente prevista no fluxo principal (LARMAN, 2007).

**Alternativo A1 – Cancelamento da operação antes da conclusão**

1.  Este fluxo alternativo inicia a partir do passo 5 do fluxo principal, quando o sistema apresenta o menu de opções de transações.

2.  Em vez de selecionar "Saque", o cliente escolhe cancelar a operação, pressionando o botão "Cancelar" ou selecionando a opção equivalente no menu.

3.  O sistema exibe mensagem informando que a operação foi cancelada, não realiza qualquer débito na conta corrente e ejeta o cartão do cliente, retornando à tela inicial após a retirada do cartão.

**Alternativo A2 – Emissão de comprovante de saque**

1.  Este fluxo é uma continuação do passo 11 do fluxo principal, depois da liberação das cédulas.

2.  Após entregar o dinheiro, o sistema pergunta ao cliente se deseja emitir comprovante da operação de saque.

3.  Caso o cliente confirme, o sistema imprime comprovante contendo data, hora, número da conta, valor sacado e informações básicas da transação; em seguida, a operação é finalizada e o cartão é ejetado.

4.  Se o cliente optar por não emitir comprovante, o sistema encerra a operação sem gerar impressão, mantendo o registro da transação apenas no sistema bancário.

**Alternativo A3 – Alteração do valor de saque pelo cliente**

1.  Este fluxo alternativo inicia a partir do passo 9 do fluxo principal, quando o sistema verifica saldo e disponibilidade de cédulas.

2.  Se, por limitação operacional, o valor informado exceder a quantidade de cédulas que pode ser disponibilizada naquele momento, mesmo havendo saldo suficiente, o sistema exibe mensagem informando a restrição e sugere a digitação de outro valor.

3.  O cliente informa um novo valor de saque compatível com o limite de cédulas do caixa eletrônico, e o sistema repete a verificação de saldo e disponibilidade.

4.  Se o novo valor for aceito, o fluxo retorna ao passo 10 do fluxo principal; caso contrário, o cliente pode optar por cancelar a operação, encerrando o caso de uso sem movimentação financeira.

### 2.4 Fluxos de exceção

Os fluxos de exceção tratam situações de erro ou anomalia que impedem a conclusão bem-sucedida da operação de saque, como senha incorreta, saldo insuficiente ou falhas técnicas (LARMAN, 2007).

**Exceção E1 – Senha incorreta**

1.  Este fluxo de exceção inicia a partir do passo 4 do fluxo principal, quando o sistema valida a senha digitada pelo cliente.

2.  Se a senha estiver incorreta, o sistema exibe mensagem de erro e solicita nova digitação, contabilizando a tentativa.

3.  Em caso de três tentativas consecutivas de senha incorreta, o sistema bloqueia o cartão por segurança, exibe mensagem orientando o cliente a procurar a agência bancária para regularização e retém o cartão no interior do caixa eletrônico, encerrando a operação sem debitar qualquer valor.

4.  O sistema registra o evento de bloqueio e retenção do cartão para fins de auditoria e acompanhamento pela instituição financeira.

**Exceção E2 – Saldo insuficiente para o saque (violação da RN01)**

1.  Este fluxo inicia a partir do passo 9 do fluxo principal, durante a verificação do saldo disponível.

2.  Se o valor solicitado para saque for superior ao saldo disponível na conta corrente, o sistema identifica violação da regra RN01 e exibe mensagem informando que não há saldo suficiente para o valor desejado.

3.  O sistema oferece ao cliente a opção de informar um novo valor de saque ou cancelar a operação, deixando claro que não houve débito.

4.  Caso o cliente informe um novo valor compatível com o saldo disponível, o sistema reinicia a verificação a partir do passo 9 do fluxo principal; se o cliente optar por cancelar, a operação é encerrada sem movimentação financeira.

**Exceção E3 – Falha na liberação das cédulas**

1.  Este fluxo é derivado do passo 11 do fluxo principal, quando o sistema autoriza a liberação das cédulas.

2.  Se ocorrer falha mecânica que impeça a saída do dinheiro, o sistema registra o erro, impede a confirmação definitiva do débito na conta corrente e exibe mensagem de indisponibilidade temporária do serviço.

3.  O sistema orienta o cliente a procurar atendimento na agência bancária ou a tentar nova operação em outro caixa eletrônico, informando que o valor não foi debitado e que a transação foi cancelada.

4.  A instituição financeira utiliza os registros gerados para monitorar a qualidade do serviço e programar manutenção no equipamento.

**Exceção E4 – Falha de comunicação com o sistema bancário central**

1.  Este fluxo pode iniciar em qualquer ponto da operação em que seja necessária comunicação com o sistema bancário central, especialmente nos passos 4 e 9 do fluxo principal.

2.  Em caso de perda de conexão ou indisponibilidade do sistema central, o caixa eletrônico não consegue validar a senha nem verificar o saldo, motivo pelo qual exibe mensagem de falha técnica e impossibilidade de concluir o saque.

3.  O sistema cancela automaticamente o processo, ejeta o cartão do cliente e registra o evento de falha de comunicação, sem realizar qualquer débito na conta corrente.

4.  A instituição financeira utiliza esses registros para acompanhar a disponibilidade do serviço e realizar ações corretivas visando a continuidade operacional.

## 3 RELAÇÃO COM PARADIGMAS DE PROGRAMAÇÃO

A disciplina Paradigmas de Programação estuda diferentes formas de organizar e estruturar programas, como os paradigmas imperativo, funcional, orientado a objetos e lógico, entre outros (SEBESTA, 2018). A descrição de casos de uso, embora seja uma técnica de análise e projeto, mantém relação direta com paradigmas de programação, sobretudo com o paradigma orientado a objetos, ao enfatizar atores, responsabilidades e mensagens trocadas entre componentes de um sistema.

No caso de uso CSU01 – Realizar saque bancário, a interação entre cliente, caixa eletrônico e sistema bancário central pode ser modelada em termos de objetos, classes e métodos, em que cada componente assume responsabilidades específicas, como validar credenciais, consultar saldo e registrar transações (LARMAN, 2007). Em linguagens orientadas a objetos, essa divisão favorece encapsulamento e coesão, enquanto em paradigmas funcionais a operação poderia ser tratada como uma composição de funções puras e efeitos controlados, evidenciando como a escolha de paradigma influencia a implementação das etapas descritas no caso de uso (SEBESTA, 2018).

Ao analisar fluxos alternativos e de exceção, observa-se também a importância de estruturas de controle de fluxo e de tratamento de erros previstos pelo paradigma de programação adotado, seja por meio de exceções, padrões de projeto ou outras abstrações, conectando a modelagem conceitual do caso de uso a decisões concretas de implementação (SEBESTA, 2018).

## 4 CONSIDERAÇÕES FINAIS

A descrição detalhada do caso de uso CSU01 – Realizar saque bancário em caixa eletrônico demonstra a relevância de documentar, de forma sistemática, as interações entre o ator principal e a aplicação, incluindo pré-condições, pós-condições, regras de negócio e variações de fluxo. Essa documentação contribui para reduzir ambiguidades, orientar o desenvolvimento de software e facilitar atividades de teste e manutenção, especialmente em sistemas bancários que exigem alta confiabilidade e segurança (SOMMERVILLE, 2018; LARMAN, 2007).

Sob a perspectiva dos paradigmas de programação, o caso de uso analisado destaca como decisões de modelagem afetam a forma de implementar operações de saque em diferentes paradigmas, reforçando a importância de alinhar a análise orientada a casos de uso com princípios de programação ensinados na disciplina (SEBESTA, 2018). Dessa maneira, o trabalho integra a prática de documentação de requisitos com a reflexão sobre paradigmas de desenvolvimento, contribuindo para uma visão mais abrangente da construção de sistemas de informação.

## Referências

LARMAN, Craig. **Utilizando UML e padrões**: uma introdução à análise e ao projeto orientados a objetos e ao desenvolvimento iterativo. 3. ed. Porto Alegre: Bookman, 2007.

SEBESTA, Robert W. **Conceitos de linguagens de programação.** 11. ed. Porto Alegre: Bookman, 2018.

SOMMERVILLE, Ian. **Engenharia de software.** 10. ed. São Paulo: Pearson, 2018.
