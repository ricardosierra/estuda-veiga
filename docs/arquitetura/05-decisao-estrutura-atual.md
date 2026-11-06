# 05 — Decisão: estrutura atual, tudo no mesmo repositório

**Estado:** decidido pelo usuário em 6 de novembro de 2026.

## Decisão

A estrutura de diretórios **permanece a atual**. Nada de repositório novo e nada de reorganização de pastas: acervo, dados do aluno (`privado/`), núcleo Rust e interface continuam no mesmo repositório. Isso substitui a recomendação de mover `crates/estuda-domain` para `app/crates/` (02 §4), que fica **suspensa**.

## Arquitetura alvo (resumo aprovado)

```text
Estuda UVA — GPUI nativo
  Interface Rust: pesquisa, estudo, progresso, sincronização
        │
Core Rust
  domínio, serviços, cache RAM, indexação, regras de negócio
        │
  ┌─────┴─────────────────────────────┐
Base privada                      Base pública
  alunos, cursos vinculados,        disciplinas, ementas, conteúdos,
  matrículas, notas, frequência,    exercícios, provas, trabalhos e
  avaliações individuais,           soluções autorizadas
  progresso, anotações
  SQLite local por perfil           SQLite + Markdown + JSON
        │
Automações Rust
  Portal UVA, Canvas, Sistema de Provas, Calendário, CDP/HTTP
```

## Onde cada peça mora hoje

| Peça | Local atual | Observação |
|---|---|---|
| Interface GPUI | `app/crates/estuda-gpui` | binário `estuda-uva` |
| Core (domínio, SQLite, catálogo, cache) | `app/crates/estuda-core` | cache RAM e indexação ainda por medir (02 §6) |
| Automações: Portal UVA, Canvas, Calendário, CDP/HTTP | `app/crates/estuda-core` (`lyceum`, `canvas*`, `calendar`, `browser`) | isolar em `estuda-portal` só se o tempo de `cargo check` justificar |
| Automações: Sistema de Provas, leitura somente-leitura | `app/crates/estuda-sync` | sem UI e sem SQLite |
| Tipos de domínio e trava de publicação | `crates/estuda-domain` (raiz) | `estuda-sync` o consome por caminho relativo |
| Base privada | `privado/` e SQLite por perfil | pode ficar em `dados/`; o `.sqlite3` em si não vai ao Git |
| Base pública | `materias/`, `fontes-uva/`, `periodos/`, `cursos/`, `dados/` | SQLite + Markdown + JSON sincronizados |
| App legado Tauri/React | `app/src-tauri`, `app/src` | congelado, fora do workspace, só para paridade |

## Consequências e pontos em aberto

- **Duas pastas `crates/`** (`crates/` na raiz e `app/crates/`) coexistem. Foi uma escolha consciente aqui, apesar do alerta em 02 §4. Se incomodar depois, é um `git mv` mais um ajuste de path em `app/crates/estuda-sync/Cargo.toml`.
- **Fronteira privado/público** segue pela trava de `estuda-domain` e pelo manifesto de publicação (04). Política de 6/11/2026, confirmada pelo dono: o conteúdo privado pode ir ao Git e pode ficar em `dados/`. Seguem proibidos: senhas, cookies, tokens, `.env`, `*.sqlite3*`, perfil do Chromium e `originais/`; RA, CPF e RG ficam fora até decisão do dono.
- **Direção futura (dono, 6/11/2026): `periodos/` não deve ser atrelado a um único curso e aluno.** O sistema deve poder se conectar por protocolo pela internet e, via blockchain, compartilhar dados entre instâncias, criando repositórios públicos e seus forks. Isso é só direção: não há desenho, protocolo nem mudança de layout aprovados. Antes de implementar, é preciso definir identidade de dado entre instâncias (`estuda-domain`), o que sai de cada instância (RA, CPF e RG seguem fora) e como conflitos entre forks são resolvidos. `periodos/` fica onde está até haver esse plano.
- **Nenhuma alteração de banco** foi feita ou implícita nesta decisão. A divisão física dos bancos (03 §4) e qualquer migration continuam dependendo de aprovação prévia com o SQL exato.
