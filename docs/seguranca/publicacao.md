# Publicação de conteúdo

**Revisado em:** 2026-11-06 · **Estado:** o repositório é **local, sem remoto** e **nada foi publicado**. Este documento define o processo; o processo automatizado é **Planejado**.

Rótulos: **Implementado**, **Parcial**, **Planejado**, **Não verificado**.

## 1. O que "publicar" significa aqui

Publicar é tornar um material acessível a pessoas além do mantenedor: subir o repositório para um remoto, distribuir um pacote da biblioteca, sincronizar com pares ou enviar um colaborador a conteúdo. Uma vez publicado, o conteúdo pode ser copiado e indexado; retirar depois não o apaga das cópias.

## 2. Estado atual

| Item | Estado |
|---|---|
| Remoto Git | **Nenhum** (README raiz e `docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md`) |
| Camada pública separada do privado | **Parcial**: separação por pasta dentro do mesmo repositório (`privado/` é versionado e, pela decisão de 6/11/2026, publicável) |
| Contrato `estuda-domain::publication` (nega por padrão) | Implementado como biblioteca com testes, **não integrado** a nenhum fluxo |
| Licença | **Nenhuma licença declarada** ([../biblioteca/fontes-e-licencas.md](../biblioteca/fontes-e-licencas.md)) |
| Histórico Git sem item proibido | **Não verificado**: contém versões antigas do registro acadêmico, que estavam na raiz ([privacidade.md](privacidade.md) §6) |
| Hash de integridade e assinatura | **Planejado** (`docs/ARQUITETURA_ACERVO.md` §6, fases 0 e 1) |

**Consequência:** pela política de 6/11/2026, `privado/` versionado e o histórico com registros pessoais **não bloqueiam mais** a publicação. Ainda assim, não faça `git remote add` nem `git push` sem concluir o checklist abaixo e a confirmação explícita do mantenedor.

## 3. Checklist antes de qualquer publicação

Marque cada item e registre quem revisou e quando.

**Escopo**

1. Definir exatamente quais caminhos serão publicados. Padrão: **nada** (negar por padrão).
2. Classificar cada caminho com o contrato de camadas ([../biblioteca/estrutura.md](../biblioteca/estrutura.md) §3). Excluir `LocalOnly` e `Generated`; `Personal` só com a política do dono (`allow_personal`); revisar `NeedsReview` arquivo a arquivo.
3. Confirmar que nenhum `originais/`, `build/`, `materias/catalogo/` (gerado), perfil do Chromium, `.env` nem banco `.sqlite3*` está no conjunto.

**Dado pessoal**

4. Rodar `node tools/acervo/build.mjs --validar` e `node tools/acervo/build.mjs --verificar-texto`; ambos devem terminar sem erro.
5. Buscar manualmente no conjunto: CPF, RG, RA, nomes de docentes, e-mails, URLs com `?token=`, `signature`, caminhos `/Users/…`. O validador **não** cobre Markdown nem texto livre.
6. Se `privado/` não entrar no conjunto, remover links relativos para `privado/` em arquivos públicos (hoje, por exemplo, `materias/README.md` e `periodos/README.md`) e `source.file` que apontem para `privado/` em `dados/`. Se `privado/` entrar no conjunto, esses links continuam válidos.
7. Verificar o histórico Git quanto a itens proibidos. Se houver algum, decidir: reescrever o histórico (por exemplo, `git filter-repo`) com backup, ou publicar a partir de um repositório novo com o conjunto aprovado. Também cabe ao dono decidir sobre as versões antigas de `REGISTRO_ACADEMICO.md`, que estavam na raiz, fora de `privado/`. Nenhuma das ações foi feita.

**Direitos**

8. Para cada texto de terceiro (`fontes-uva/`, citações, imagens, e-books): licença conhecida ou permissão registrada; senão, não publicar ([../biblioteca/fontes-e-licencas.md](../biblioteca/fontes-e-licencas.md) §3).
9. Escolher e adicionar licença para código, documentação e conteúdo próprio.
10. Conferir que cada contribuição de terceiros tem consentimento de publicação registrado (§5).

**Integridade**

11. Gerar o hash SHA-256 do conteúdo de cada arquivo publicado e registrá-lo em manifesto (**Planejado**; o gerador ainda não faz). Até lá, registrar o hash do commit e conferir `git status` limpo.
12. Garantir que as fichas de metadados ([../biblioteca/catalogacao.md](../biblioteca/catalogacao.md)) tenham `publication = approved` e `verification = reviewed` (contrato **proposto**).

**Operação**

13. Branch de produção `master` (regra do mantenedor); sem `Co-Authored-By` nos commits.
14. Primeiro lançamento em `v0.1.0`; `v1.0.0` fica reservado.
15. Nunca publicar sem confirmação explícita do mantenedor.

## 4. Revisão de soluções e provas

Soluções, provas e trabalhos podem violar regras de integridade acadêmica se divulgados enquanto a avaliação estiver aberta.

1. **Avaliação em aberto:** não publicar enunciado nem solução.
2. **Autorização:** a solução tem autor identificado (pode ser pseudônimo) e consentimento por escrito para a licença escolhida.
3. **Origem:** enunciado de terceiros só com licença da origem; senão, descrever o problema com palavras próprias.
4. **Revisão por segunda pessoa:** confere correção, fontes, ausência de dado de terceiro e de trecho copiado.
5. **Registro:** a revisão fica registrada (quem, quando, resultado), por exemplo no campo `verification` da ficha ou no commit.
6. **Retirada:** se houver pedido do autor, do docente ou da instituição, o material volta a `withdrawn` e é removido da próxima publicação.

## 5. Consentimento de colaboradores

- Contribuições só são aceitas com declaração de que o colaborador é autor ou tem direito de publicar, e de que aceita a licença do conteúdo (a escolher).
- A contribuição não pode conter dados de terceiros (colegas, docentes) sem autorização.
- O consentimento é registrado por escrito (issue ou pull request, ou campo `authorship.consent`).
- O colaborador pode pedir retirada futura; o mantenedor deve poder cumprir (ver §4.6). Isto é um dos motivos para cuidar de dado pessoal em estruturas imutáveis ou replicadas (`docs/ARQUITETURA_ACERVO.md` §6; LGPD). A decisão de 6/11/2026 trata dos dados do dono do acervo; dados de colaboradores não foram decididos.

## 6. Integridade

- **Hoje (Parcial):** o Git garante integridade do histórico local por hash de commit. O banco do app guarda `sha256` do texto de cada material importado (`materials.sha256`, `text_sha256`) e o sincronizador guarda `content_hash` por documento (`sync_documents`). Não há manifesto público de hashes.
- **Planejado:** objetos identificados por SHA-256, manifesto JSON por período e matéria, verificação no gerador (fase 0); manifestos assinados com Ed25519 e encadeados (fase 1); troca entre pares (fase 2). O mesmo documento (`docs/ARQUITETURA_ACERVO.md` §6) lista decisões pendentes: quais conteúdos entram, qual pilha de rede e a estrutura de repositórios.
- **Limite:** hash prova que o conteúdo não mudou; não prova que seja autorizado nem correto.

## 7. Decisões pendentes para o mantenedor

1. Estrutura: a decisão de 6/11/2026 mantém `privado/` no mesmo repositório. Falta definir se a publicação será do repositório inteiro ou de um subconjunto.
2. Reescrever o histórico atual ou publicar a partir de repositório novo, conforme o resultado da verificação (checklist, item 7).
3. Licença de cada camada.
4. Destino de `fontes-uva/` (publicar, restringir ou substituir por links e resumos próprios).
5. Quando integrar `estuda-domain::publication` ao gerador como verificação obrigatória antes de qualquer publicação.
