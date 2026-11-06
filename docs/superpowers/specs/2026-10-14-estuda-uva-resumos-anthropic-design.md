# Design: resumos de estudo com Anthropic no Estuda UVA

**Estado:** aprovado pela usuária em 2026-10-14.  
**Data:** 2026-10-14

## Entendimento do pedido

Adicionar ao Estuda UVA uma forma de criar um resumo curto e um resumo longo de cada matéria para a usuária ouvir e usar no estudo, além das aulas guiadas que já existem. A geração deve usar a API da Anthropic e a chave já configurada no projeto local `gerenciador-de-conteudo`.

## Evidências do app atual

- O app é Tauri com frontend React e backend Rust; já usa `reqwest` no backend.
- A tela de matéria tem abas para aulas, materiais, histórico e desempenho. Os materiais locais são associados à matrícula ativa por matéria e semestre.
- O app já lê Markdown/texto local e converte materiais importados do Canvas. A aula guiada é conteúdo local determinístico; a nova geração será identificada separadamente como conteúdo gerado por IA.
- O arquivo `.env` do projeto `gerenciador-de-conteudo` contém uma chave Anthropic e está com permissão `0600`. O valor não foi exibido nem copiado.

## Desenho proposto

### Experiência

1. Adicionar uma aba **Resumos** à tela de cada matéria.
2. Mostrar os materiais locais legíveis que serão usados, com título/arquivo e semestre. Incluir materiais importados já salvos no acervo; não buscar nem enviar conteúdo direto do Canvas.
3. Mostrar aviso de que o texto selecionado será enviado à Anthropic e que a chamada pode consumir créditos. A geração acontece somente quando a usuária acionar **Gerar resumos**.
4. Gerar em português, numa única chamada, dois textos separados: resumo curto (alvo de aproximadamente 2 minutos de leitura) e resumo longo (alvo de aproximadamente 8 minutos), com explicações didáticas, conceitos-chave, exemplos presentes na fonte e referências ao nome/página quando a fonte trouxer essa informação.
5. Exibir e salvar os textos localmente para consulta offline. Incluir controles de ouvir, pausar/retomar e parar para cada resumo, usando a voz disponível no dispositivo com idioma `pt-BR`. Não criar nem enviar arquivos de áudio.
6. Exibir data, modelo e materiais de origem; marcar o conjunto como desatualizado quando os textos de origem mudarem e permitir regenerar manualmente.

### Integração e dados

- A chamada à Messages API e a chave ficam exclusivamente no backend Rust/Tauri. O frontend nunca recebe a chave.
- Ler `ANTHROPIC_API_KEY` e `ANTHROPIC_MODEL` sob demanda do arquivo `~/Dev/Projetos/gerenciador-de-conteudo/.env`, sem alterar esse arquivo, copiar o segredo para outro local, imprimir o valor ou gravá-lo no SQLite/Markdown/JSON/Git. O modelo configurado atualmente é `claude-haiku-5-5`; se não houver configuração de modelo, usar esse mesmo identificador como padrão.
- A fonte da geração é somente o material didático legível associado à matrícula ativa e ao semestre da tela. Não incluir notas, frequência, registros oficiais, credenciais ou cookies.
- Depois de confirmado pela usuária, enviar o material selecionado à Anthropic. Não há geração automática em segundo plano.
- Persistir os resumos e metadados numa tabela própria do banco SQLite local do app, sem editar os originais ou tratar os resumos como registros acadêmicos oficiais. A chave para detectar conteúdo desatualizado usa os caminhos e hashes do material de origem.
- Não registrar chave, cabeçalhos de autenticação, texto integral dos materiais, requisição ou resposta da API nos logs. Mostrar erros sanitizados para chave ausente, indisponibilidade, limite/credito, resposta inválida e timeout.
- Configuração ausente ou erro da API não deve impedir o restante do Estuda UVA nem a abertura das aulas existentes.

### Leitura em voz alta

Usar a Web Speech API disponível no WebView para sintetizar localmente o texto salvo. O idioma inicial será `pt-BR`; a disponibilidade e qualidade da voz dependem do dispositivo. O estado de reprodução não altera progresso oficial ou histórico de frequência.

### Fora do escopo

- Gerar áudio com um serviço de voz da Anthropic ou criar arquivos de áudio.
- Enviar automaticamente materiais, dados acadêmicos oficiais ou conteúdo do Canvas que ainda não foi importado localmente.
- Alterar arquivos originais, registros acadêmicos ou outras integrações do projeto `gerenciador-de-conteudo`.

## Critérios de aceite propostos

1. Uma matéria com material local legível permite gerar os dois resumos em uma ação confirmada; ambos ficam disponíveis offline depois da geração.
2. Uma matéria sem fonte legível informa isso e não chama a API.
3. A chave nunca chega ao frontend, aos logs ou a arquivos rastreados; a ausência da chave gera orientação compreensível sem quebrar o app.
4. A confirmação informa que os materiais selecionados serão enviados à Anthropic antes da chamada.
5. Os controles de voz leem em português, permitem pausar/retomar e parar, e reportam se a síntese não estiver disponível.
6. Uma alteração no material de origem marca o resumo guardado como desatualizado sem apagar o conteúdo anterior nem sobrescrever o material.
7. Respostas inválidas, erros HTTP e timeout não substituem resumos locais existentes.

## Decisões para revisão

- O escopo recomendado é um par de resumos para toda a matéria e semestre, usando os materiais locais legíveis associados à matrícula ativa.
- A voz será a voz instalada no dispositivo; a API da Anthropic será usada apenas para gerar texto.
- O texto só será enviado depois de uma ação explícita, com aviso de envio e possível consumo de créditos.

## Referências técnicas consultadas

- [Anthropic Messages API](https://platform.claude.com/docs/en/api/messages/create)
- [Anthropic modelos disponíveis](https://platform.claude.com/docs/en/models/overview)
- [MDN SpeechSynthesis](https://developer.mozilla.org/en-US/docs/Web/API/SpeechSynthesis)
