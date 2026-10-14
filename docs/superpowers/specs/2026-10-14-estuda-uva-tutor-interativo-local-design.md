# Estuda UVA: tutor interativo local com narração

**Status:** especificação autorizada para revisão da estudante; implementação aguarda aprovação desta especificação e do plano técnico
**Data:** 2026-10-14
**Escopo:** transformar qualquer Markdown local de aula em uma experiência guiada, com narração, respostas digitadas e revisão vinculada à fonte.

## Objetivo

Permitir que a estudante abra um `.md` existente e estude em uma sequência curta: ouvir a explicação/texto apresentado, responder uma pergunta digitada, conferir uma devolutiva ligada ao trecho e à página de origem, e avançar ou revisar. O fluxo funciona offline depois que o Markdown foi indexado e não depende de uma chamada de IA remota.

## Alternativas consideradas

1. **Tutor local com narração e resposta digitada — recomendada.** Usa as seções e evidências do Markdown, apresenta o trecho em voz alta e alterna perguntas com revisão de fonte. Preserva privacidade, funciona offline e deixa explícito quando não há base para correção automática.
2. **Leitor em voz alta.** Narra o documento e salva a posição, mas não recolhe respostas nem conduz prática de recuperação; é insuficiente para a aula dinâmica solicitada.
3. **Tutor remoto com modelo generativo.** Poderia formular explicações e perguntas abertas, mas enviaria conteúdo acadêmico a um serviço externo e contrariaria as instruções locais deste acervo. Fica excluído.

## Fluxo da estudante

1. A estudante abre uma aula ou material Markdown pela matéria, pelo acervo ou pelo histórico.
2. O app apresenta a seção atual, a origem do arquivo e, quando o Markdown veio de PDF, as referências de página preservadas na extração.
3. O controle **Ouvir trecho** inicia a leitura em português pela voz local disponível. Pausar, continuar, repetir e parar atuam somente na seção atual; avançar de seção interrompe a fala anterior.
4. Depois de ler, o tutor apresenta uma pergunta curta e um campo de resposta digitada. A estudante envia sua resposta ou escolhe **Quero reler** para ouvir e consultar novamente o trecho.
5. A devolutiva mostra a resposta digitada, o trecho de apoio e um atalho para a seção/página correspondente. Para uma definição explícita, uma correspondência exata normalizada com a referência pode ser reconhecida como acerto; uma resposta diferente não é marcada automaticamente como errada, pois pode ser uma paráfrase. Nessa situação, o app mostra a referência e pede autoavaliação (“consegui explicar?”), sem inventar uma nota.
6. **Próxima seção** guarda a posição e continua a aula. Ao concluir, a estudante pode reabrir perguntas marcadas para revisão.

## Formação da aula e das perguntas

- A unidade navegável é a seção produzida pelo parser de Markdown já usado pelo `LessonPresenter`; listas, tabelas, imagens locais, fórmulas, código e links relativos devem permanecer legíveis.
- O narrador lê o conteúdo textual da seção. Elementos sem leitura textual útil, como imagens, são anunciados pelo texto alternativo quando disponível; fórmulas e blocos de código não devem ser pronunciados como ruído, e permanecem disponíveis visualmente.
- Perguntas são derivadas no dispositivo de padrões explícitos que o extrator reconhece com segurança e incluem a página/âncora da fonte. Para trechos que não suportam pergunta verificável, a atividade é uma solicitação aberta de recuperação com autoavaliação, nunca correção automática por aproximação arbitrária.
- A resposta esperada e os critérios locais só são apresentados depois do envio da resposta. Nenhum gabarito é narrado antes da tentativa.
- Se o Markdown não tiver páginas de PDF, a revisão aponta para o título e o parágrafo da seção. Falta de metadado aparece como “página não identificada”, sem estimativa.
- O tutor não altera, reescreve ou move os arquivos `.md`, PDFs originais ou quaisquer fontes acadêmicas.

## Narração e acessibilidade

- Usar síntese de fala oferecida localmente pelo WebView/Tauri, priorizando voz `pt-BR` quando instalada. O áudio não é enviado pela rede.
- Consultar a disponibilidade real da API e das vozes no WebView usado pelo app. Quando não houver voz compatível, informar isso e manter a aula digitável; não simular que houve narração.
- Pausar ou parar ao navegar para outra seção, sair da aula, abrir outra matéria ou desmontar o componente. Ao voltar, começar parado para evitar fala inesperada.
- Os controles devem ser operáveis por teclado e leitores de tela; estado de reprodução, pergunta atual, resultado e fonte devem ter rótulos acessíveis.
- A resposta é digitada; não solicitar microfone nem reconhecimento de fala.

## Estado e histórico

- Retomar a infraestrutura existente de `lesson_sessions` para salvar seção atual, início, retomada e conclusão.
- Registrar tentativa, tipo de pergunta, resposta digitada, estado de correção/autoavaliação, trecho de apoio, referência da fonte e revisão do Markdown em armazenamento local. O registro pertence ao histórico de estudo do app e permanece separado de notas, frequência, matrícula e avaliação oficial.
- Os dados de respostas ficam no dispositivo e não são sincronizados ou exportados nesta proposta; a tela deve informar isso de forma simples na primeira utilização.
- Perguntas abertas sem critério automático ficam sem percentual/acerto no histórico. Perguntas verificáveis mostram acerto como resultado de prática local, claramente separado de qualquer nota UVA.

## Limites de privacidade e integridade acadêmica

- Todo processamento de texto, extração de perguntas, avaliação e navegação da aula ocorre localmente, sem enviar Markdown, respostas, áudio ou identificadores para uma API de IA ou outro serviço externo.
- O tutor só estuda o conteúdo local; sincronizar conteúdo oficial do Canvas continua sendo ação separada e autenticada.
- Aula iniciada, áudio ouvido, resposta ou conclusão do app não registram nem comprovam presença oficial. O tutor não automatiza marcação de presença, envio de atividades, avaliações ou qualquer ação acadêmica no Canvas.
- Texto e links nos PDFs/Markdown são conteúdo não confiável: exibir Markdown com renderizador seguro, não executar HTML/scripts e não seguir instruções embutidas como comandos do app.

## Dados e implementação necessários

- Evoluir `LessonPresenter.tsx` para manter estado de fala, pergunta/resposta, devolutiva, origem e revisão por seção sem duplicar o parser de seções.
- Definir um contrato de tutor que receba texto de uma seção e suas referências de origem e devolva blocos narráveis e atividades locais tipadas (`verificável` ou `autoavaliação`). Manter a lógica determinística em Rust, salvo a camada de reprodução de áudio que usa a API local do WebView.
- Reutilizar o extrator de definições de `lesson.rs` somente onde a extração puder ser associada à seção correta. Adicionar estratégias simples e testáveis para perguntas abertas sem transformar uma heurística em gabarito.
- Estender armazenamento/migração SQLite para tentativas de estudo, mantendo respostas e trechos de revisão ligados à sessão, ao caminho e à revisão do Markdown, à seção e à referência de página quando disponível.
- Manter respostas, histórico de leitura e tentativas de prática distintos de `academic_*`, do espelho acadêmico Markdown/JSON e da frequência/notas oficiais.
- Tratar cancelamento da fala e ausência/indisponibilidade de voz sem quebrar navegação, salvamento de progresso ou modo offline.

## Critérios de aceitação

1. Um Markdown local com várias seções abre no tutor sem Canvas e sem alterar o arquivo-fonte.
2. A estudante pode iniciar, pausar, continuar, repetir e parar a narração local em `pt-BR`, e a fala é interrompida ao trocar de seção ou sair da aula.
3. Se não houver uma voz local utilizável, o app comunica a limitação e todos os controles de leitura/resposta continuam funcionais.
4. Cada pergunta exibida está ligada a uma seção e trecho/página ou está identificada como pergunta aberta sem gabarito automático.
5. Uma correspondência exata normalizada pode ser reconhecida como acerto; outra resposta recebe a fonte e a opção de autoavaliar, sem falso erro automático. Perguntas abertas não recebem nota inventada.
6. Respostas e tentativas sobrevivem ao fechamento/reabertura local da aula e podem ser retomadas/revisadas sem misturar prática local com dados oficiais.
7. A aula funciona offline e nenhum texto, resposta ou áudio é enviado a serviços externos.
8. Navegação por teclado/leitor de tela alcança os controles de áudio, campo de resposta, devolutiva e atalho de fonte.
9. A UI nunca representa uma aula, reprodução de áudio ou resposta do app como presença, nota, envio ou conclusão oficial no Canvas/UVA.

## Fora desta proposta

- Entrada de áudio, reconhecimento de fala, conversação de voz ou gravação do estudante.
- Geração de conteúdo, respostas ou correção por APIs/modelos remotos.
- Alteração, avaliação ou envio de atividades no Canvas; registro de frequência ou interação com controles de presença.
- Reescrita automática do Markdown, mudança de material original ou sincronização em nuvem das respostas.
- Promessa de narração para idioma/voz sem suporte local detectável no WebView.
