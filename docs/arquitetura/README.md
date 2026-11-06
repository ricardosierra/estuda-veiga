# Arquitetura do Estuda UVA (reorganização proposta)

**Estado:** proposta e primeiro incremento seguro. Nada aqui foi migrado, e nenhum banco SQLite foi alterado.
**Política vigente (6/11/2026, decisão do dono):** todo conteúdo pode ser público, inclusive notas, matrículas e trabalhos, desde que **separado** em camada própria. Nunca publicados: credenciais, bancos SQLite, perfis do navegador, binários originais.
**Medido em:** 6 de novembro de 2026, HEAD `3811300`, branch `master`.

| Documento | Conteúdo |
|---|---|
| [01-diagnostico.md](01-diagnostico.md) | O que existe de fato, com números, e os problemas encontrados |
| [02-arquitetura-alvo.md](02-arquitetura-alvo.md) | Árvore, mapa de módulos, regras de dependência, migração incremental, métricas |
| [03-dados-e-sincronizacao.md](03-dados-e-sincronizacao.md) | Entidades, bancos pessoal/acervo (separados, ambos publicáveis em texto), contrato de sincronização, SQL proposto (não executado) |
| [04-seguranca-e-publicacao.md](04-seguranca-e-publicacao.md) | Camadas e política de publicação, manifesto de publicação, riscos |
| [05-decisao-estrutura-atual.md](05-decisao-estrutura-atual.md) | Decisão: estrutura atual, tudo no mesmo repositório; arquitetura alvo aprovada |

O único código novo é `crates/estuda-domain/`: identificadores tipados, estado de confiança do dado e a política de publicação por camada. Não tem I/O, banco, interface nem dependências.
