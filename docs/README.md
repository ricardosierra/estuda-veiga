# Documentação do Faculdade e do Estuda UVA

**Revisado em:** 2026-11-06 · **Base:** repositório em movimento durante a revisão. O `HEAD` passou de `3811300` para `e29f27b` enquanto as páginas eram escritas, e a árvore de trabalho tinha mudanças não commitadas, inclusive de outra sessão. Cada página cita a base que usou.

Esta documentação descreve o código e os arquivos que existem. O que é só plano está rotulado **Planejado**. Os rótulos **Implementado**, **Parcial**, **Planejado** e **Não verificado** são definidos no [README da raiz](../README.md#como-ler-os-rótulos-de-estado).

**Antes de confiar em qualquer página, leia [lacunas e contradições](referencia/lacunas-e-contradicoes.md).** Ela lista o que está desatualizado, o que se contradiz e o que não foi verificado.

## Por onde começar

| Quem é você | Leia |
|---|---|
| Novo usuário | [Instalação](getting-started/instalacao.md) → [Primeiros passos](getting-started/primeiros-passos.md) → [Telas](aplicativo/telas.md) |
| Estudante procurando material | [Estrutura da biblioteca](biblioteca/estrutura.md) → [Tipos de conteúdo](biblioteca/tipos-de-conteudo.md) |
| Desenvolvedor | [Visão geral](arquitetura/visao-geral.md) → [Crates](arquitetura/crates.md) → [Ambiente e compilação](desenvolvimento/ambiente-e-compilacao.md) → [Testes](desenvolvimento/testes.md) |
| Colaborador de conteúdo | [Contribuir com conteúdo](contribuindo/conteudo.md) → [Catalogação](biblioteca/catalogacao.md) |
| Mantenedor dos bancos | [Dados: visão geral](dados/visao-geral.md) → [Banco privado](dados/banco-privado.md) → [Sincronização](dados/sincronizacao.md) → [Migrações](dados/migracoes.md) |
| Auditor de privacidade | [Privacidade](seguranca/privacidade.md) → [Credenciais e sessões](seguranca/credenciais-e-sessoes.md) → [Ameaças](seguranca/ameacas.md) |

## Índice

### Primeiros passos
- [Instalação](getting-started/instalacao.md): requisitos, verificação.
- [Primeiros passos e configuração](getting-started/primeiros-passos.md): uso do acervo e do app, ajustes.
- [Resolução de problemas](getting-started/resolucao-problemas.md)

### Aplicativo
- [Telas](aplicativo/telas.md): o que cada tela mostra, de onde vêm os dados, estados vazios, acessibilidade.

### Arquitetura
- [Visão geral](arquitetura/visao-geral.md): camadas e caminhos de execução.
- [Crates e módulos](arquitetura/crates.md): `estuda-domain`, `estuda-core`, `estuda-gpui`, shell Tauri.
- [Migração Tauri para GPUI](arquitetura/migracao-gpui.md): o que foi convertido e o que é legado.
- [Eventos e estado](arquitetura/eventos-e-estado.md)
- [Decisões pendentes](arquitetura/decisoes-pendentes.md)
- Propostas de arquitetura-alvo, de outra sessão de trabalho: [README](arquitetura/README.md), [01 diagnóstico](arquitetura/01-diagnostico.md), [02 arquitetura-alvo](arquitetura/02-arquitetura-alvo.md), [03 dados e sincronização](arquitetura/03-dados-e-sincronizacao.md), [04 segurança e publicação](arquitetura/04-seguranca-e-publicacao.md), [05 estrutura atual](arquitetura/05-decisao-estrutura-atual.md). Descrevem o alvo, não o estado atual.

### Dados
- [Visão geral](dados/visao-geral.md)
- [Banco privado (do aplicativo)](dados/banco-privado.md): esquema real, versão 8 (baseline única, sem `ALTER`).
- [Banco público](dados/banco-publico.md): não existe; contém o projeto futuro.
- [Sincronização](dados/sincronizacao.md): três superfícies, conflitos, outbox.
- [Migrações](dados/migracoes.md)
- [Proveniência e confiança](dados/proveniencia-e-confianca.md)

### Biblioteca
- [Estrutura](biblioteca/estrutura.md)
- [Catalogação](biblioteca/catalogacao.md): contrato **proposto** de metadados.
- [Tipos de conteúdo](biblioteca/tipos-de-conteudo.md)
- [Fontes e licenças](biblioteca/fontes-e-licencas.md)

### Automações
- [Visão geral](automacoes/visao-geral.md)
- [Portal e Lyceum](automacoes/portal-lyceum.md) · [Canvas](automacoes/canvas.md) · [Calendário](automacoes/calendario.md)
- [Extração textual](automacoes/extracao-textual.md)
- [Resumos com a API da Anthropic](automacoes/resumos-anthropic.md)
- [Limites e segurança](automacoes/limites-e-seguranca.md)

### Desenvolvimento
- [Ambiente e compilação](desenvolvimento/ambiente-e-compilacao.md) · [Testes](desenvolvimento/testes.md) · [Performance](desenvolvimento/performance.md)
- [Convenções](desenvolvimento/convencoes.md) · [Contribuir com código](desenvolvimento/contribuindo-codigo.md)

### Contribuição de conteúdo
- [Contribuir com conteúdo](contribuindo/conteudo.md)

### Segurança
- [Privacidade](seguranca/privacidade.md) · [Credenciais e sessões](seguranca/credenciais-e-sessoes.md) · [Publicação](seguranca/publicacao.md) · [Ameaças](seguranca/ameacas.md)

### Referência
- [Glossário](referencia/glossario.md) · [Formatos e convenções](referencia/formatos.md) · [Comandos](referencia/comandos.md)
- [Lacunas e contradições](referencia/lacunas-e-contradicoes.md)

## Documentos anteriores a esta revisão

Continuam no lugar. Partes estão desatualizadas; os desvios conhecidos estão em [lacunas e contradições](referencia/lacunas-e-contradicoes.md).

- [Arquitetura do acervo](ARQUITETURA_ACERVO.md) · [Estado de implementação do Estuda UVA](ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md) · [Navegação no Portal do Aluno](NAVEGACAO_PORTAL_UVA.md)
- Ideias e planos de IA, **não** implementados salvo o que a página indica: [features](features/README.md)
- Planos e especificações históricas: [`superpowers/plans/`](superpowers/plans/) e [`superpowers/specs/`](superpowers/specs/). São registro de decisões de outubro de 2026, não descrição do estado atual, e o app não os indexa.
- Guias específicos: [app](../app/README.md) · [gerador do acervo](../tools/acervo/README.md) · [dados](../dados/LEIA-ME.md)

## Convenções desta documentação

- Cada afirmação técnica cita o caminho do código que a fundamenta, relativo à raiz do repositório.
- Exemplos usam dados sintéticos e placeholders. Nenhuma página deve conter dado pessoal real, credencial, cookie ou token.
- Diagramas são Mermaid em texto, sempre acompanhados de explicação textual. Não há imagens.
- Links são relativos. A verificação dos links é descrita em [lacunas e contradições](referencia/lacunas-e-contradicoes.md#verificações-desta-revisão).
