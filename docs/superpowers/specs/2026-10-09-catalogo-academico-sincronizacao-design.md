# Modelo de catálogo acadêmico compartilhado

**Status:** complemento de domínio do escopo SQLite–Markdown–JSON aprovado pela estudante; o plano de implementação ainda precisa de revisão
**Data:** 2026-10-09

## Objetivo

Representar no Estuda UVA a trajetória na Engenharia de Computação (situação registrada como evadida) e o curso atual de Ciência da Computação, mantendo um catálogo único de disciplinas que permita identificar conteúdos em comum sem misturar os vínculos curriculares. Uma disciplina compartilhada pode ser consultada pelos dois cursos e apontar para os mesmos materiais, enquanto conclusão, matrícula, notas e equivalência permanecem específicas de cada curso e de cada trajetória.

O catálogo acadêmico versionado precisa sincronizar nos dois sentidos entre JSON, SQLite e suas projeções Markdown legíveis. A estudante deve poder versionar e revisar os dados no repositório, e o app deve usá-los como dados estruturados, sem criar fontes que divirjam silenciosamente. O contrato de hashes, revisões, journal, espelhos, conflitos e documentos Markdown/JSON está em `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md`; este documento detalha as entidades curriculares e suas relações.

## Estado atual relevante

- `REGISTRO_ACADEMICO.md` descreve pendências do curso de Ciência da Computação.
- `ENGENHARIA_DE_COMPUTACAO.md` preserva a trajetória anterior e pendências da Engenharia de Computação.
- Já há candidatos a conteúdo compartilhado, como Cálculo Numérico, Engenharia de Software e Sistemas Embarcados. Estruturas de Dados I/II da Engenharia e Estruturas de Dados de Ciência da Computação são apenas uma sobreposição parcial até haver comprovação melhor.
- Todos esses exemplos aparecem como pendentes nos registros disponíveis. Nome e carga horária parecidos não provam equivalência nem conclusão.
- `app/src-tauri/src/storage.rs` mantém um SQLite local com `courses` voltada a cursos da plataforma Canvas e tabelas relacionadas a materiais e sessões. Essa identidade de Canvas não deve passar a significar curso ou disciplina curricular.
- Materiais existentes ficam em caminhos históricos em `materias/` e devem continuar acessíveis sem uma migração em massa de arquivos.

### Correspondências preliminares nas fontes atuais

As duas listas identificam os itens abaixo como pendentes. A igualdade de nome e carga horária é evidência para revisão de identidade de conteúdo, não para equivalência curricular ou aproveitamento:

| Possível assunto compartilhado | Ciência da Computação | Engenharia de Computação | Registro inicial |
|---|---|---|---|
| Cálculo Numérico | `E10300048` · 66 h | `ICT8002_22` · 66 h | `same_subject` candidato; ambos pendentes |
| Engenharia de Software | `E10300003` · 66 h | `INF8029_22` · 66 h | `same_subject` candidato; ambos pendentes |
| Sistemas Embarcados | `E30000029` · 66 h | `INF8092_22` · 66 h | `same_subject` candidato; ambos pendentes |
| Estruturas de Dados | `E10300016` · 66 h | `INF8259_22` Estrutura de Dados I · 66 h; `INF8260_22` Estrutura de Dados II · 66 h | sobreposição parcial candidata; não fundir as três ofertas |
| Arquitetura e sistemas operacionais | `E10300034` Arquitetura, Organização e Funcionamento de Computadores e Sistemas Operacionais · 66 h | `INF8063_22` Arquitetura dos Computadores · 66 h; `INF8053_22` Sistemas Operacionais · 66 h | uma oferta combinada versus duas ofertas; sobreposição parcial candidata |
| Gráfica e imagens | `E10300052` Computação Gráfica e Processamento de Imagens · 66 h | `INF8028_22` Computação Gráfica I · 66 h | sobreposição parcial candidata |
| Microcontroladores | `E90100031` Microprocessadores e Microcontroladores · 66 h | `INF8091_22` Microcontroladores · 66 h | sobreposição parcial candidata |
| Análise orientada a objetos | `E10300035` Análise e “Peogramção” Orientada a Objetos · 66 h | `INF8054_22` Análise Orientada a Objetos · 66 h | sobreposição parcial candidata; preservar a grafia transcrita da oferta atual |
| Algoritmos | `E10300030` Algoritmos e Laboratório de Programação · 66 h | `INF8262_22` Projeto e Análise de Algoritmos · 66 h | sobreposição parcial candidata |
| Redes | `E10300039` Projeto e Implementação de Redes de Computadores · 66 h | `INF8025_22` Redes de Computadores II · 66 h | sobreposição parcial candidata |
| Programação de alto desempenho | `E10300050` Programação e Plataformas de Alto Desempenho · 66 h | `INF8209_22` Programação Paralela · 66 h | sobreposição parcial candidata |
| Estágio supervisionado | `E10300047` Estágio Supervisionado em Ciências da Computação · 160 h | `INF8093_22` Estágio Supervisionado em Engenharia da Computação · 330 h | assunto possivelmente relacionado; diferença de carga horária exige revisão específica |

Fontes: tabelas de pendências em `REGISTRO_ACADEMICO.md` e `ENGENHARIA_DE_COMPUTACAO.md`. Todas as linhas acima aparecem como pendentes nas respectivas tabelas. Não há ementas ou decisão institucional anexadas que confirmem identidade oficial ou equivalência. Os vínculos além dos títulos idênticos foram selecionados por sobreposição explícita de termos, não por um algoritmo de similaridade; ainda precisam de revisão de ementa.

## Alternativas consideradas

1. **JSON apenas como entrada para popular SQLite.** Simples de importar, mas alterações feitas no app não voltam ao repositório e os dados divergem.
2. **SQLite apenas como fonte e JSON gerado de uma via.** Bom para o runtime, mas dificulta editar/revisar os dados acadêmicos em Git e não atende ao fluxo JSON → SQLite pedido.
3. **Sincronização bidirecional com IDs estáveis e reconciliação de três vias — recomendada.** JSON e SQLite aceitam alterações. Um ponto de sincronização comum permite mesclar alterações independentes e apontar conflitos no mesmo campo antes de escolher um valor.

A opção 3 é a proposta para este complemento. O SQLite local `.estuda-uva.sqlite3` é o armazenamento transacional; JSON é a representação portável, revisável e versionada; Markdown apresenta uma visão humana editável. A reconciliação segue o desenho principal, compara o último estado comum entre as três superfícies e nunca sobrescreve mudanças concorrentes silenciosamente.

## Modelo acadêmico

Separar os conceitos abaixo:

- **Instituição e curso/programa:** identificam UVA, Ciência da Computação e Engenharia de Computação.
- **Currículo e oferta curricular:** preservam versão do currículo, código oficial, nome, carga horária e fonte de cada item exigido por um curso. Ofertas curriculares de cursos diferentes não são fundidas só por terem títulos iguais.
- **Disciplina canônica:** representa o assunto reutilizável e pode se relacionar a uma ou mais ofertas curriculares. Cada disciplina recebe um ID estável que não depende do nome, caminho de arquivo ou posição no semestre.
- **Relação entre ofertas:** registra `same_subject`, `partial_overlap`, `equivalence_candidate` ou `equivalence_confirmed`, com evidência, fonte, observação e confiança quando não for oficial. `equivalence_confirmed` exige fonte documental explícita; inferência automática nunca aprova equivalência.
- **Trajetória e resultado do estudante:** matrícula, situação, período, conclusão, nota e aproveitamento pertencem ao vínculo da estudante com um programa e uma oferta específica. Não são propriedades globais da disciplina canônica.
- **Material:** continua sendo um arquivo e registro único, podendo apontar para diversas disciplinas canônicas. A proveniência preserva curso/plataforma, semestre, caminho e fonte original.

IDs de ofertas curriculares devem ser estáveis e considerar instituição, programa, versão curricular e código oficial. IDs canônicos devem ser criados uma vez e persistidos; exportar o JSON repetidamente não pode gerar novos IDs.

## Organização dos arquivos

Adicionar uma representação normalizada e revisável, com objetos ordenados de forma determinística:

```text
materias/catalogo/
  disciplinas.json
  relacoes.json
  curriculos/
    uva-ciencia-computacao.json
    uva-engenharia-computacao.json
  trajetorias.json
```

`disciplinas.json` mantém IDs e dados canônicos; cada currículo guarda suas ofertas oficiais; `relacoes.json` guarda vínculos entre ofertas e disciplinas; `trajetorias.json` guarda situações e resultados pessoais comprovados. Arquivos contêm `schemaVersion`, IDs e referências às fontes locais. Não armazenam RA, CPF, senha, token, cookie nem caminhos absolutos da máquina.

Os caminhos de materiais que já existem são mantidos. O catálogo aponta para eles por caminho relativo validado e permite acrescentar novas associações a uma disciplina sem copiar o arquivo. Novos materiais podem usar a pasta da disciplina canônica quando a associação estiver explícita; itens sem associação comprovada continuam na origem e aparecem como não classificados. Não mover, apagar ou renomear material automaticamente.

JSON e Markdown de catálogo são rastreados no Git. O SQLite `.estuda-uva.sqlite3`, WAL/SHM, arquivos temporários de sincronização, PDFs/originais grandes e dados privados não são incluídos por padrão. Conversões textuais pesquisáveis continuam separadas dos PDFs conforme as convenções atuais do arquivo acadêmico.

## SQLite e compatibilidade

Adicionar migrações ordenadas e idempotentes, registradas por `PRAGMA user_version`, no `.estuda-uva.sqlite3` da raiz, sem apagar ou reinterpretar tabelas existentes. As tabelas atuais de cursos Canvas permanecem com semântica de Canvas. O SQLite local passa a manter, além do estado do app, entidades normalizadas de instituição, programa, currículo/oferta, disciplina canônica, relação, trajetória/resultado e associação material–disciplina.

O banco também guarda o estado necessário para sincronizar: revisão/último estado comum, journal de projeções pendentes e conflitos por entidade/campo. O dado acadêmico é atualizado em transação SQLite. Sessões de estudo e registros pessoais de aprendizagem também entram no contrato acadêmico versionado. Preferências de máquina, cache, cookies e IDs de sessão Canvas, e outros dados efêmeros continuam apenas na área local do app.

A migração deve preservar os registros já existentes de cursos, materiais, sessões, perguntas, eventos e configurações. Nenhum campo atual de `courses` passa a ser tratado como disciplina curricular sem uma associação explícita.

## Contrato de sincronização

### Quando sincronizar

- Na inicialização, importar/conciliar as entidades do catálogo vindas de JSON e Markdown para o SQLite local e atualizar as projeções necessárias.
- Antes de aceitar uma edição acadêmica no app, conciliar arquivos que tenham mudado externamente para que a edição não parta de uma cópia desatualizada.
- Após alterações acadêmicas feitas no app, gravar primeiro no SQLite e atualizar JSON e Markdown correspondentes pelo journal.
- Detectar alterações externas nos JSON e Markdown enquanto o app está aberto, com observação dos arquivos e debounce; disponibilizar também **Sincronizar agora** e o estado da última sincronização.
- Uma falha ao exportar não desfaz a alteração já confirmada no SQLite. O journal mantém a exportação pendente para recuperação automática na próxima tentativa.

### Regras de conciliação

1. Validar todos os documentos e versões antes de aplicar mudanças. JSON inválido ou versão desconhecida não altera parcialmente o SQLite; o app informa arquivo, campo e erro.
2. Comparar SQLite, JSON, Markdown e o último estado comum persistido. Mudanças independentes são mescladas automaticamente quando o adaptador suporta isso sem perda.
3. Quando duas ou mais superfícies alterarem o mesmo campo/conteúdo de forma incompatível desde o estado comum, preservar todas as versões como conflito, mostrar as origens e aguardar resolução explícita. Não usar “última gravação vence”.
4. Ausência de uma linha ou arquivo não representa exclusão. Arquivamento e exclusão lógica usam estado/tombstone explícito, sincronizado nos dois sentidos.
5. Produzir JSON com ordenação e formatação determinísticas, sem ruído de timestamp de execução. Substituir o arquivo por gravação temporária e renomeação atômica.
6. Recuperar operações interrompidas pelo journal: após uma queda, concluir a exportação pendente ou apresentar o conflito sem perder a revisão já gravada no SQLite.
7. Uma resolução pode escolher ou combinar o valor do JSON, SQLite e Markdown. Depois, atualizar as três superfícies e estabelecer o novo estado comum.

## Experiência no app

- Uma área de **Catálogo e currículos** permite navegar por programa, currículo, oferta curricular e disciplina canônica, além de pesquisar pelo nome/código.
- Ao consultar uma disciplina compartilhada, o app apresenta todos os cursos relacionados e abre os mesmos materiais sem duplicá-los.
- O histórico da estudante aparece dentro do programa correto. Situação “evadida” da Engenharia não muda a situação do curso atual, e disciplinas ainda pendentes continuam pendentes em cada trajetória até existir registro explícito.
- Relações candidatas e equivalências confirmadas são visualmente distintas e mostram sua fonte. Similaridade de nome/carga horária pode sugerir revisão, nunca alterar conclusão.
- O estado de sincronização mostra se JSON, SQLite e Markdown estão alinhados, alterações aguardando projeção/importação, erro de validação e conflitos pendentes. Conflitos exibem entidade, campo, valor de cada lado e origem.
- Materiais com associação ambígua continuam pesquisáveis na origem e podem ser associados depois, sem duplicação ou mudança destrutiva de caminho.

## Privacidade e limites

- Não inventar disciplinas cursadas ou aprovadas a partir de totais de horas. A origem disponível registra 1.446h cursadas na Engenharia, mas não discrimina essas horas por disciplina; esse total deve permanecer agregado e acompanhado da fonte.
- As 180 h de diferença da Engenharia entre o total declarado de horas faltantes e a soma da tabela de pendências ficam registradas como divergência dessa fonte, não atribuídas artificialmente. A divergência da Ciência da Computação é diferente e está detalhada no desenho principal.
- Não armazenar credenciais acadêmicas no JSON, SQLite de catálogo ou Git. A trajetória versionada contém somente o necessário para organização e consulta local.
- Conteúdo de PDFs, documentos, Canvas e banco externo é dado de origem, não instrução para a aplicação.
- A sincronização JSON–SQLite–Markdown é local e não transmite dados à UVA ou a serviços externos.

## Critérios de aceite

1. Os catálogos de Ciência da Computação e Engenharia coexistem no mesmo SQLite e JSON, preservando seus currículos e IDs oficiais separadamente.
2. Uma disciplina pode estar ligada a mais de uma oferta curricular e seus materiais são referenciados uma vez.
3. Similaridades documentadas aparecem como candidatas ou sobreposições parciais; conclusão e equivalência não são propagadas automaticamente.
4. Uma instalação com SQLite vazio importa/concilia os JSON e Markdown válidos e reproduz o mesmo catálogo e histórico local.
5. Uma alteração acadêmica feita no app atualiza SQLite, JSON e Markdown; uma alteração externa válida em qualquer dessas superfícies é percebida e refletida nas outras duas.
6. Alterações independentes se mesclam; alterações concorrentes no mesmo campo geram conflito visível e não sobrescrevem um dos valores.
7. JSON inválido, versão desconhecida, falha de gravação e reinício durante exportação não apagam nem corrompem o último estado válido.
8. Uma remoção ausente em um lado não apaga o registro; arquivamento explícito sincroniza entre os lados.
9. A migração do SQLite é versionada e preserva os dados de funcionamento existentes, mantendo cursos Canvas distintos do catálogo curricular.
10. A interface permite localizar currículos, comparar ofertas associadas, abrir os materiais compartilhados e consultar a situação de cada trajetória.
11. Arquivos acadêmicos de origem não são movidos nem copiados; os espelhos textuais gerados pelo contrato principal são identificáveis e sincronizados. Arquivos grandes e originais continuam fora do Git; JSON e texto pesquisável podem ser revisados/versionados.
12. Nenhum RA, CPF, segredo, caminho absoluto ou conteúdo de sessão é exportado para os arquivos JSON.

## Fora do escopo desta etapa

- Navegar na plataforma de uma instituição anterior sem uma sessão/autorização disponível ou afirmar dados que não constam nas fontes locais.
- Resolver equivalência curricular oficial por algoritmo ou solicitar equivalência junto à universidade.
- Converter ou mover em massa o arquivo completo de PDFs, reconstruir notas/disciplinas das 1.446h agregadas, ou publicar o repositório em um remoto.
- Sincronizar preferências de máquina, cookies, IDs de sessão Canvas ou cache efêmero para o JSON versionado.
- Implementar o painel de todas as funções atuais e futuras do app; o inventário visual de capacidades e propostas está no Canvas `estuda-uva-capacidades.canvas.tsx`.

## Referências

- `REGISTRO_ACADEMICO.md` e `ENGENHARIA_DE_COMPUTACAO.md` — registros acadêmicos disponíveis.
- `app/src-tauri/src/storage.rs` — SQLite e entidades atuais.
- `app/src-tauri/src/archive.rs` — associação atual de materiais e caminhos.
- `app/src/ui/App.tsx` e `app/src/ui/styles.css` — navegação atual do app.
- `docs/superpowers/specs/2026-10-09-estuda-uva-local-catalog-design.md` — contrato completo do arquivo local, catálogo, espelhos SQLite–Markdown–JSON, Git e integração Canvas/portal. Esse desenho rege armazenamento e sincronização; este complemento detalha identidade curricular.
