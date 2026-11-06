# Arquitetura do acervo

**Estado:** descreve a organização atual, o que a reorganização de novembro de 2026 mudou e o que ainda falta para compartilhar o acervo com outras pessoas.

## 1. Princípios

1. **Um conteúdo, um lugar.** Cada matéria tem uma pasta em `materias/` e um registro em `dados/disciplinas/`. Cada texto oficial tem uma pasta em `fontes-uva/`. Quem precisa do conteúdo aponta para ele; não copia.
2. **Conteúdo compartilhável e dado pessoal são camadas separadas.** Nada pessoal entra na camada compartilhada.
3. **Ligação por identificador estável.** Os registros usam IDs como `uva:discipline:…` e `uva:program:…`. Os caminhos de fonte (`source.file`) são validados: o gerador recusa fonte inexistente.
4. **Texto é a forma versionada.** PDF e Word ficam fora do Git. O texto extraído entra em `fontes-uva/` com página e fonte.

## 2. Camadas

| Camada | Onde está | Compartilhável | Conteúdo típico |
|---|---|---|---|
| Conteúdo | `materias/`, `fontes-uva/`, `docs/`, `app/`, `tools/` | sim, se o texto for próprio ou público | atividades, textos de unidade, código |
| Estrutura acadêmica | `cursos/`, `periodos/`, `dados/instituicao`, `dados/programas`, `dados/disciplinas`, `dados/ofertas`, `dados/relacoes`, `dados/periodos`, `dados/canvas` | sim, sem dado de pessoa | código do curso, currículo, nome da disciplina |
| Pessoal | `dados/matriculas/`, `dados/notas/`, `dados/trajetorias/`, `REGISTRO_ACADEMICO.md`, `.estuda-uva.sqlite3`, `materias/catalogo/capturas/` | não | notas, frequência, situação, nomes de docentes |
| Gerado | `build/`, `materias/catalogo/*.json`, `.estuda-uva/mirrors/` | não versionar | saídas do gerador e do app |

Nomes de professores são dados de terceiros. Por isso ficam na camada pessoal, junto das matrículas.

## 3. Como evitar duplicação

- **Matéria:** uma pasta canônica por matéria. Nomes alternativos são links simbólicos. Hoje há três: `automatos`, `calculo-1` e `estatistica`.
- **Textos oficiais:** uma cópia em `fontes-uva/`. Em novembro de 2026 foram achados dois calendários com o mesmo conteúdo (mesmo SHA-256); o segundo virou um ponteiro para o original.
- **Outra pessoa na mesma matéria:** cria só os registros pessoais (matrícula, notas), apontando para `uva:discipline:<slug>`. Não copia materiais.
- **Pendente:** o gerador ainda não verifica duplicação por hash. Sugestão: um comando `--verificar-duplicados` que compara SHA-256 de todos os textos.

## 4. Como os links funcionam

- **Markdown:** caminhos relativos, como `../../periodos/2026.2/README.md`. O verificador de links roda sobre todos os `.md`.
- **Dados:** `source.file` aponta para um arquivo que precisa existir. `arquivo` em `dados/trabalhos` aponta para uma pasta de `materias/`.
- **Aplicativo:** o escaneador do catálogo (`app/src-tauri/src/catalog/scanner.rs`) indexa `materias/`, `fontes-uva/`, `periodos/`, `cursos/`, `docs/` e `originais/`. Pastas fora dessa lista não aparecem no app. Há um teste de regressão para isso.

## 5. Escala para novas pessoas

Cada estudante deve ter uma área de dados pessoais fora do conteúdo compartilhado, ou um repositório privado próprio, que referencia os IDs compartilhados. O conteúdo compartilhado não muda quando uma pessoa entra. Pendente: decidir onde fica a área pessoal de cada estudante e como o gerador lê dois repositórios.

## 6. Rede P2P e "blockchain": recomendação

Objetivo declarado: pessoas trocam conteúdo entre si com protocolo próprio, em cadeia de registros, sem depender de um servidor central.

A parte que traz valor é o **endereçamento por conteúdo** (cada texto identificado pelo seu hash) e o **registro assinado e encadeado** (cada entrada aponta para a anterior). Isso é o modelo do Git e do IPFS. Consenso de blockchain não é necessário para materiais de estudo, e não há token ou pagamento envolvido.

Proposta em fases:

- **Fase 0 (próximo passo):** objetos identificados por SHA-256 e um manifesto JSON por período e matéria, listando os objetos. O gerador passaria a verificar a integridade; hoje ainda não faz isso.
- **Fase 1:** cada manifesto é assinado pelo autor (Ed25519) e cada entrada guarda o hash da anterior. Continua local, sem rede.
- **Fase 2:** sincronização entre pares, com troca de manifestos e de objetos que faltam. A escolha de stack (libp2p, Iroh ou um protocolo simples sobre HTTPS com lista de pares) é uma decisão sua.
- **Fase 3, só se houver necessidade:** consenso. O currículo oficial continua sendo responsabilidade da UVA; consenso entre estudantes não o substitui.

Limites que valem antes de qualquer fase de rede:

- **Dado pessoal fica fora do log.** Notas, frequência, matrícula e nomes de docentes não são replicados. Blockchain torna o dado imutável, o que conflita com correção e exclusão de dados pessoais previstas na LGPD.
- **Direitos autorais.** Textos extraídos de materiais da UVA e de e-books de terceiros não devem ser replicados entre usuários sem permissão. Antes da fase 2, é preciso decidir o que é compartilhável.
- **Identidade.** Sem chave por pessoa não há autoria verificável. Cada estudante teria um par de chaves Ed25519.

Decisões pendentes:

1. Quais conteúdos entram no compartilhamento (só código e textos próprios, ou também textos extraídos de materiais oficiais).
2. Stack da rede (fase 2).
3. Estrutura de repositórios: um compartilhado mais um privado por estudante, ou outra forma.

## 7. Pendências conhecidas

- `cursos/engenharia-de-computacao/README.md` mistura informação do curso com dados pessoais (situação, notas, frequência). Precisa ser separado.
- `REGISTRO_ACADEMICO.md` é pessoal e já está no histórico Git. Se o repositório for publicado, esse histórico vai junto.
- `materias/paradigmas-de-programacao/` usa nome diferente do registro canônico `paradigmas-de-linguagens-de-programacao`.
- Dois diagramas referenciados por `materias/README.md` (`diagrama-afd-termina-em-1.png` e `automato_termina_em_1.jff`) não existem no acervo. A referência virou texto.
- `fontes-uva/2026.2/bases-de-programacao/capitulo-01.md` e `BPR_Cap_01.md` têm conteúdos diferentes; é preciso conferir qual é a versão correta.
- `materias/catalogo/*.json` são gerados e ficam fora do Git. Para regenerar: `node tools/acervo/build.mjs`. Esse comando também recria `build/acervo.sqlite3`, por isso não foi executado nesta reorganização.
- O `.estuda-uva.sqlite3` do aplicativo guarda os caminhos antigos de `materias/fontes-uva/`. Na próxima sincronização, o app reconcilia esses caminhos. Essa mudança no banco precisa de aprovação prévia.
- Os PDFs de `fontes-uva/2026.2/` continuam locais. O `PLANEJAMENTO.md` pede o descarte deles depois da extração; a decisão de descartar fica com você.
