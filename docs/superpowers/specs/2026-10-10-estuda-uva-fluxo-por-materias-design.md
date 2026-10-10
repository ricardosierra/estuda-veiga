# Estuda UVA: fluxo de estudo centrado em matérias

**Status:** implementação em andamento a partir do fluxo explicitamente solicitado pela estudante
**Data:** 2026-10-10
**Escopo:** reorganização da experiência de consulta e estudo; complemento às especificações aprovadas de catálogo, sincronização local e sincronização entre dispositivos.

## Objetivo

Levar a estudante da matéria à aula em poucos passos: abrir o app, escolher uma matéria, encontrar suas aulas, pedir para o app apresentar uma aula e consultar o histórico daquela matéria. O acervo continua vindo da pasta local configurada e do Canvas/portal quando disponíveis, mas a navegação principal passa a representar o percurso acadêmico, não a estrutura de diretórios.

O login do Canvas controla apenas a atualização de conteúdo que dependa do Canvas. Ele não bloqueia a leitura de materiais, registros e aulas já presentes na pasta local ou no SQLite.

## Alternativas consideradas

1. **Matérias primeiro — recomendada.** A tela inicial mostra as matérias ativas; cada matéria reúne aulas, materiais, informações acadêmicas e histórico. Atende diretamente ao fluxo descrito pela estudante.
2. **Agenda primeiro.** A tela inicial destaca prazos e próximos eventos, com acesso secundário às matérias. Ajuda no planejamento diário, mas não torna a consulta de aulas o caminho principal.
3. **Acervo e busca primeiro.** Preserva a navegação por pasta, arquivo e busca global. É a organização que levou à interface confusa atual e não atende ao fluxo pedido.

## Navegação proposta

- **Estudar:** entrada principal com as matérias do período ativo, atividade recente e próximos eventos conhecidos. Cada cartão mostra o nome da matéria, período/turma quando disponível, quantidade ou estado dos materiais associados e um atalho para continuar a aula recente.
- **Matérias:** lista completa agrupada por período. O período ativo vem da matrícula registrada localmente; períodos anteriores permanecem acessíveis sem se misturarem às matérias atuais.
- **Agenda:** prazos e eventos por matéria, mantendo visível a origem e a data da última atualização.
- **Histórico:** sessões de aula e resultados de prática, filtráveis por período e matéria.
- **Acadêmico:** trajetória, currículo, notas e frequência com suas fontes. Matrículas ativas, disciplinas concluídas e pendências curriculares mantêm estados distintos.
- **Dados e sincronização:** acervo geral sem classificação, busca global, estado SQLite–Markdown–JSON, conflitos, Git e dispositivos pareados. São ferramentas de apoio, não o primeiro destino ao abrir o app.

## Matérias e associação de conteúdo

- A lista de matérias usa primeiro os vínculos ativos da trajetória local. Canvas pode atualizar módulos e conteúdo da matéria, mas não é a fonte exclusiva da lista.
- Os materiais da pasta configurada são associados por identificadores e metadados já existentes; relações explícitas do catálogo acadêmico; curso, período, módulo e proveniência registrados no material; e correspondências de título/alias que possam ser explicadas.
- Não associar materiais a uma matéria apenas porque os nomes de pastas parecem semelhantes. Ambiguidades aparecem em “Materiais sem matéria” para revisão, mantendo o caminho e o conteúdo originais.
- A página de uma matéria apresenta suas unidades/aulas em ordem acadêmica, com estados como **disponível localmente**, **atualização do Canvas necessária**, **associação a revisar** ou **sem material associado**. Um estado não confirmado não deve aparecer como uma contagem zero conclusiva.
- O acervo completo continua pesquisável por nome e conteúdo em Dados e sincronização. A árvore de diretórios pode ser acessada como detalhe técnico, sem ser a interface normal de estudo.
- Material de 2026.1 e de cursos anteriores continua consultável pelo respectivo período. Não será incorporado automaticamente às sete matrículas locais de 2026.2. Associações que dependem de identidade de disciplina ainda não confirmada ficam para revisão.

## Página da matéria

Ao abrir uma matéria, a estudante encontra:

1. **Aulas:** unidades/módulos ordenados, com andamento de leitura e estado de disponibilidade.
2. **Materiais:** Markdown, trabalhos e referências da matéria, incluindo material local e origem oficial quando aplicável.
3. **Desempenho:** notas oficiais e frequência apenas quando constarem em fonte registrada, separadas dos resultados de exercícios do app.
4. **Histórico:** aulas iniciadas/retomadas/concluídas e práticas realizadas naquela matéria.

Cada item com conteúdo mostra sua origem e quando foi capturado ou sincronizado. Abrir um item apresenta o conteúdo dentro do app, sem exigir que a estudante navegue por nomes de arquivos ou caminhos.

## Apresentação da aula

- A ação principal é **Apresentar aula**. Ela abre o texto Markdown ou a aula montada localmente em uma tela de leitura dedicada, com índice das seções, próxima/anterior e retomada do último ponto lido.
- Preservar títulos, listas, tabelas, imagens locais, links relativos seguros, exemplos e referências de página/fonte. Não executar HTML, scripts ou conteúdo remoto embutido.
- A apresentação reorganiza e navega o material existente; não inventa explicações, notas ou fatos acadêmicos. Conteúdo extraído de PDF continua sendo Markdown versionável; PDFs novos são temporários de extração e não são adicionados ao acervo.
- A prática de recuperação permanece opcional ao fim da leitura. O resultado é identificado como prática local, e não como nota ou avaliação oficial da UVA.

## Histórico de estudo

- Cada matéria mostra o histórico com data/hora, aula/material, ação (início, retomada, conclusão), progresso de leitura e, se houve prática, acertos e percentual.
- Leitura e prática são eventos diferentes. Retomar uma aula atualiza o progresso da sessão apropriada; cada prática concluída permanece como um resultado próprio.
- O histórico é salvo primeiro no SQLite da pasta local, projetado no contrato existente de Markdown/JSON e sincronizado entre instalações pareadas conforme as regras de sincronização aprovadas. Eventos têm identificadores estáveis e são mesclados como eventos, evitando que uma sessão substitua outra.
- Histórico do app nunca é apresentado como histórico oficial do portal. Notas, frequência e conclusões oficiais permanecem identificadas pela fonte institucional.

## Estados e disponibilidade

- **Aguardando login no Canvas:** permite continuar a estudar conteúdo local e informa que atualização do Canvas está indisponível até a sessão visível ser autenticada.
- **Conteúdo local disponível:** abre as aulas e registros salvos sem rede.
- **Nenhum material associado:** indica que a matéria existe, mas que a indexação/associação não encontrou material; oferece atualização do Canvas ou revisão do acervo quando apropriado.
- **Acervo ainda sendo indexado**, **indexação parcial** e **erro de leitura/sincronização** são estados separados; nenhum deles deve ser exibido como acervo vazio.
- Notas/práticas vazias e histórico vazio significam que não há registro correspondente. Não substituir esses estados por um painel genérico de zeros quando os dados ainda não foram carregados ou não puderam ser associados.

## Dados e implementação necessários

- Reutilizar a trajetória, o catálogo curricular, as relações de disciplinas, o scanner local, o SQLite e a sincronização descritos nas especificações existentes.
- Criar uma consulta local de matérias do período e seus materiais associados que funcione sem Canvas. Reutilizar identificadores e relações estáveis para evitar duplicar um material em matérias relacionadas.
- Acrescentar consulta paginada/limitada do histórico por matéria e registro de progresso de leitura de aula. A migração SQLite preserva sessões de prática já existentes.
- Separar componentes de navegação, início de estudo, lista/detalhe da matéria, apresentação de aula e histórico; a tela `App.tsx` mantém o shell e os estados compartilhados.
- Migrar a árvore do acervo para Dados e sincronização sem eliminar a busca global ou a abertura/edição sincronizada dos arquivos.
- Manter credenciais, cookies, perfil de navegador, configuração por dispositivo, banco SQLite vivo, PDFs temporários e originais excluídos fora do espelho peer-to-peer. Somente dados e materiais elegíveis seguem o contrato de sincronização já aprovado.

## Critérios de aceitação

1. Ao abrir o app sem login no Canvas, a estudante vê as matrículas locais ativas, não uma solicitação de login como substituta do conteúdo local.
2. É possível abrir uma matéria ativa e consultar suas aulas/materiais sem visualizar uma árvore de pastas.
3. Cada material é mostrado como local, pendente de atualização, sem associação confirmada ou com erro, conforme evidência disponível; um material não é atribuído silenciosamente a uma disciplina errada.
4. A ação **Apresentar aula** abre o texto no app, permite navegar pelas seções e retoma do ponto salvo.
5. O histórico por matéria distingue leitura, prática, progresso, data e origem sem misturar resultados do app com notas oficiais.
6. Matérias e conteúdo de semestres anteriores continuam acessíveis sob seus próprios períodos.
7. Busca global, dados acadêmicos, conflitos e estado de sincronização permanecem acessíveis na área secundária de Dados e sincronização.
8. Toda a navegação de materiais já locais funciona offline; login apenas governa operações que precisam buscar novos dados do Canvas/portal.

## Fora desta proposta

- Não mudar o contrato de sincronização de três vias, regras de conflito, pareamento entre dispositivos ou armazenamento acadêmico já aprovado; a nova experiência consome esses serviços.
- Não inferir equivalência curricular, notas ou associação material–disciplina sem evidência suficiente.
- Não guardar cópias de PDFs importados apenas para alimentar a interface.
- Não alterar ações de matrícula, financeiro, perfil ou outras operações no portal institucional.
