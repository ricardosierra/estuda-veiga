# 04 — Segurança e publicação

## 1. Trava de publicação (implementada)

`crates/estuda-domain/src/publication.rs` classifica caminhos relativos em cinco camadas e aplica uma `PublicationPolicy`. **Política do dono (6/11/2026): `PublicationPolicy::OWNER`, dado pessoal publicável.** O padrão da biblioteca (`Default`) continua conservador. Desconhecido nega por padrão:

| Camada | Exemplos | Publicável |
|---|---|---|
| `LocalOnly` | `.estuda-uva*`, `originais/**`, perfil do Chromium, qualquer `*.sqlite3*`, `.env*` | nunca |
| `Personal` | `privado/**`, `dados/{notas,matriculas,trajetorias,trabalhos}/**`, `REGISTRO_ACADEMICO.md`, `cursos/**`, `materias/catalogo/capturas/**` | sim com `OWNER`; não com a política padrão |
| `Generated` | `build/**`, `materias/catalogo/**`, `app/dist/**`, `app/node_modules/**` | nunca (reconstruível) |
| `Shareable` | `fontes-uva/**`, `docs/arquitetura/**`, `crates/**`, `tools/**`, `dados/{instituicao,programas,ofertas,relacoes,periodos}/**` | sim |
| `NeedsReview` | `dados/disciplinas/**`, `dados/canvas/**`, `materias/**`, qualquer caminho desconhecido | só se uma pessoa o listar como revisado |

Revisão humana desbloqueia apenas `NeedsReview`; nunca `LocalOnly` nem `Generated` (testes `no_policy_or_review_can_publish_local_only_or_generated` e `review_unlocks_needs_review_only`). 17 testes passam. Caminhos absolutos, `..`, `\` e NUL são rejeitados (`validate_relative_path`).

**Limites declarados:** a trava olha o **caminho**, não o conteúdo. Um arquivo `Shareable` ainda deve passar em `tools/acervo/src/validar.mjs` (RA, CPF, e-mail, senha, cookie, caminho absoluto). Não está ligada a nenhum fluxo de commit ou release: hoje é uma biblioteca testada, não um gate ativo. A lista de prefixos duplica a de `separar-privado.mjs` e precisa de uma fonte única quando o `estuda-cli` existir (`estuda check-publish`).

## 2. Manifesto de publicação (proposto)

Um arquivo `biblioteca/MANIFEST.json` por snapshot, gerado e **revisado antes** de ser assinado:

```json
{
  "snapshot": "2026.11.1",
  "entries": [
    { "path": "biblioteca/conteudos/calculo/limites.md", "sha256": "…",
      "source_id": "…", "license": "CC-BY-4.0",
      "rights_checked": true, "privacy_checked": true, "reviewed_by": "…", "reviewed_at": "…" }
  ]
}
```

O gerador só inclui entradas com `rights_checked` e `privacy_checked` verdadeiros e que passem em `check_publishable`. Por decisão do dono, materiais do Portal/Canvas e dados pessoais podem entrar; o manifesto registra a origem e a licença conhecida (`unknown` é aceito, mas visível). Fica registrado como risco, sem bloqueio: texto de terceiros (UVA, e-books) pode ter direitos autorais; a responsabilidade pela publicação é do dono.

## 3. Controles gerais

- **Credenciais:** nunca em log, JSON, Markdown ou Git; `settings` do app não guarda token. O login do Portal continua manual na janela Chromium.
- **Diretórios privados:** `0700`/`0600`; logs técnicos em arquivo separado, sem conteúdo acadêmico e sem URL com query.
- **Entrada externa:** JSON, HTML e Markdown importados são dados. Nenhum código de documento é executado; HTML passa por sanitização antes de virar texto; caminhos vindos de documento passam por `validate_relative_path` e por canonicalização contra a raiz antes de abrir.
- **Fixtures:** sem nome, RA, nota ou docente reais. Os testes novos usam apenas IDs sintéticos de formato.

## 4. Decisões que dependem de você

1. ~~`privado/` dentro do repositório~~ **Resolvido pela política de 6/11/2026:** fica no repositório, separado do catálogo; não há reescrita de histórico.
2. **Quem executou `separar-privado.mjs --aplicar`.** O working tree tem 273 mudanças não commitadas. Confirme se foi intencional antes de qualquer commit.
3. **Qual banco é o real** (`user_version` 0 em app data, 5 no legado da raiz).
4. ~~Direitos do conteúdo de Portal/Canvas~~ **Resolvido:** tudo pode ir (04 §2). Pendente só decidir se RA, CPF e RG podem entrar; seguem excluídos.
5. **Aprovação do SQL** de 03 §7 e, depois, da cópia de dados.
6. **Teste do ciclo Node↔Rust** em clone temporário (03 §5.7).
7. **Workspace Cargo na raiz** (02 §4).

## 5. Riscos e limitações

- Dado pessoal público inclui nomes de docentes (terceiros) e notas; é decisão do dono e fica registrada aqui.
- Se o app abrir o banco legado (v5), **não o migra**: move o arquivo para `*.v5-<epoch>.bak.sqlite3` na mesma pasta e cria um banco novo v8 (regenerável pela sincronização). Nada é apagado, mas é mudança de arquivo: não abra o app apontando para ele antes de decidir.
- A classificação por caminho produz falso negativo para dado pessoal dentro de arquivo `Shareable`; a defesa é o validador de conteúdo.
- `Confidence::reconcile` é política mínima; casos como notas parciais exigem regra por campo.
- Nenhuma captura autenticada do Portal/Canvas foi executada nem testada nesta etapa.
