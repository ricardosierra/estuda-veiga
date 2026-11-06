# Estuda UVA

Aplicativo desktop local para organizar materiais acadêmicos do Canvas UVA, converter PDFs em Markdown por página e consultar prazos sem alterar atividades ou notas.

O que está implementado, o que ainda depende de sessão autenticada e as verificações mais recentes está em [Estado de implementação](../docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md).

## Desenvolvimento

Requisitos: Rust estável compatível com a versão indicada no `Cargo.toml`, Node.js/npm apenas para o frontend e Chromium ou Chrome instalado.
O app usa `chromiumoxide`/CDP em Rust, sem Playwright; Node serve apenas para compilar a interface. Extração, regras e pontuação rodam localmente em Rust, sem geração por IA.

```sh
npm install
npm run build
cargo check --manifest-path src-tauri/Cargo.toml
cargo test --manifest-path src-tauri/Cargo.toml --lib
npm run tauri build
npm run tauri dev
```

O login do Canvas acontece manualmente numa janela Chromium visível e num perfil separado do navegador pessoal. O app não solicita nem guarda senha. A automação navega e consulta conteúdo em modo de leitura.
O botão de cancelamento interrompe um download ativo antes de gravar o material. Cookies, URLs assinadas e parâmetros temporários são usados apenas em memória; não aparecem no acervo, no banco ou nas mensagens de erro.

## Agenda

A agenda persiste eventos no SQLite local em `app_data_dir`, separados por semestre e com link de origem. A sincronização consulta atribuições publicadas e prazos aplicáveis ao estudante nas matérias atuais exibidas pelo Canvas. Ela não abre páginas de entrega nem modifica atividades.

Em 9 de outubro de 2026, o próximo semestre registrado no planejamento é 2027.1. As fontes públicas consultadas ainda não apresentaram seu calendário oficial. Por isso, a tela mostra o período sem datas e oferece o link oficial da UVA; nenhum prazo é estimado a partir de semestres anteriores.

## Arquivos e privacidade

O app escreve textos e aulas em Markdown na pasta local do acervo configurada pelo estudante. PDFs brutos são temporários durante a conversão; dependências, builds, perfil do Chromium, banco SQLite e dados locais permanecem ignorados pelo Git. Cookies e credenciais não são gravados no banco ou nos arquivos versionados.

## Fluxo de uso documentado

### Preparar ou atualizar o acervo

1. Abra `app/` no terminal e instale as dependências do frontend com `npm install`.
2. Execute `npm run tauri dev` para abrir o Estuda UVA. O app usa como acervo padrão a pasta raiz deste repositório; confirme o caminho em **Dados e sincronização**.
3. Use **Sincronizar agora** para conciliar os arquivos Markdown/JSON com o SQLite. O monitor local também tenta sincronizar depois de alterações em arquivos acompanhados.
4. Consulte **Estado do Git** na mesma área. O app apenas mostra os arquivos rastreados, novos e ignorados; não adiciona, commita ou publica mudanças.
5. Abra **Acervo** para pesquisar e ler notas, materiais textuais e documentação. Os documentos originais permanecem nas pastas em que já estavam; arquivos sob `originais/` ficam fora da sincronização.

### Registrar o Portal do Aluno e o Lyceum

O caminho de cada tela do Portal e dos seus 5 subsistemas, e o procedimento Rust de cada tarefa (implementado, a implementar ou manual), estão em [docs/NAVEGACAO_PORTAL_UVA.md](../docs/NAVEGACAO_PORTAL_UVA.md).

1. Em **Acadêmico**, selecione **Abrir janela do Lyceum**.
2. Se o portal pedir autenticação, digite CPF/senha e conclua MFA apenas na janela visível do Estuda UVA. O app não lê senha de arquivos nem a grava.
3. Depois do acesso, selecione **Capturar histórico e notas**. A navegação segue links acadêmicos oficiais visíveis, em modo de leitura, até o limite configurado de páginas.
4. Revise o estado e as seções listadas na tela. Uma captura pode ser parcial quando uma área não abre, a sessão expira ou o limite é alcançado.
5. O app grava arquivos novos em `materias/catalogo/capturas/`: um JSON com a captura estruturada e um Markdown com títulos, campos, tabelas, links oficiais e avisos. Ambos entram na sincronização local; o SQLite em uso é `.estuda-uva.sqlite3` na raiz do acervo. Espelhos Markdown/JSON gerados pelo sincronizador ficam em `.estuda-uva/mirrors/`.
6. Confira a captura em **Acervo** ou abra os caminhos exibidos no resultado. Cada atualização ganha arquivos próprios para manter o histórico; o app não substitui uma captura anterior.

O catálogo em **Acadêmico** lê currículos, relações entre ofertas e trajetórias do SQLite, após reconciliar as fontes JSON. Ciência da Computação e Engenharia de Computação continuam separadas; relações candidatas não confirmam equivalência nem aproveitamento. Divergências do registro são preservadas junto com suas fontes.

### Ler notas, fóruns, avisos e Caixa de Entrada do Canvas

1. Conecte o Canvas (**Matérias** → **Abrir Canvas para entrar**) e faça o login manual na janela Chromium do app.
2. Em **Currículos**, no painel **Notas, fóruns, avisos e Caixa de Entrada do Canvas**, marque as partes que quer ler. A Caixa de Entrada vem desmarcada por ter mensagens privadas.
3. Selecione **Ler agora**. O app lê pela API do Canvas, sem IA, todas as disciplinas listadas em `/courses`, incluindo o Espaço Curso. Nada é postado, enviado ou marcado como lido; conversas são lidas com `auto_mark_as_read=false`.
4. A captura é gravada como arquivos novos em `materias/catalogo/capturas/canvas-<id>.json` e `.md`, fora do Git, e sincronizada no SQLite. Falhas em um item aparecem como aviso e marcam a captura como parcial.
5. As notas do Canvas são das atividades online (A1). A nota oficial continua sendo a do boletim do Lyceum, e a captura não altera `dados/notas/`.

No mesmo painel, **Nota final ponderada** calcula localmente a NFp (A1 × 0,4 + maior entre A2 e A3 × 0,6; especiais com A4 e aprovação com 7,0) e a nota mínima que falta na prova. É uma estimativa.

### Consultar e estudar materiais do Canvas

1. Abra **Matérias** e selecione **Abrir Canvas para entrar**. O Chromium do Estuda UVA usa um perfil próprio; faça o login manualmente nessa janela.
2. Volte à lista **Matérias** e selecione **Capturar todas as disciplinas**. A varredura percorre em sequência os cursos e materiais publicados que a conta autenticada consegue ver, exclui “Espaço Curso” e ignora avaliações, fóruns, notas e frequência.
3. Acompanhe o progresso por curso e por item. Você pode cancelar com segurança, retomar uma execução parcial ou repetir somente um item com falha. Se a sessão expirar, entre novamente no Chromium e retome a execução.
4. PDFs acessíveis são validados e convertidos para Markdown por página em memória; o PDF bruto temporário é descartado pelo importador em lote. Páginas de aula compatíveis também viram Markdown. Arquivos já existentes e edições locais são preservados; versões alteradas ficam sinalizadas para revisão.
5. Ao concluir, abra a matéria e escolha um material capturado para iniciar a aula local. A narração usa a voz pt-BR local disponível, quando suportada, e as respostas digitadas ficam apenas no SQLite local.
6. Materiais com período confirmado são associados à matrícula do semestre. Semestre desconhecido aparece sem inferência; cursos antigos, páginas inacessíveis, PDFs sem texto e links não suportados continuam identificados no resultado.

A captura começa por ação do estudante e navega somente pelo conteúdo publicado visível. O acesso pode aparecer no histórico comum do Canvas; não marca presença, conclusão, notas, entrega, fórum ou matrícula. O painel informa “captura concluída/parcial” do acervo local, nunca conclusão acadêmica oficial. A varredura autenticada precisa ser iniciada pelo botão no app; uma compilação ou teste local não comprova que todos os cursos foram baixados.

Para abrir um item específico que a varredura não reconheceu, use **Canvas e importação** para consultar o módulo e importar aquele material individualmente. Sessões locais registram material e progresso de leitura para retomar o estudo.

O fluxo de captura não tenta baixar vídeo, avaliação, fórum, nota, presença ou entrega. A frequência oficial continua sendo a exibida pelo Portal/Lyceum. Abrir um material ou registrar uma sessão de estudo no app não altera frequência, notas, matrícula ou entregas na UVA.

### Verificações documentadas

Na raiz do repositório, as verificações focadas do backend são:

```sh
cargo fmt --manifest-path app/src-tauri/Cargo.toml -- --check
cargo test --offline --manifest-path app/src-tauri/Cargo.toml --lib
npm run build --prefix app
```

Esses comandos verificam formatação, testes locais de Rust e compilação do frontend. Não comprovam uma captura real no portal, a cobertura de um semestre no Canvas nem a frequência oficial. Esses fluxos dependem de uma sessão autenticada e devem ser conferidos na janela do app; registre páginas indisponíveis como cobertura parcial.
