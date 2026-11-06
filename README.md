# Acervo acadêmico — Faculdade

Repositório local para organizar cursos, períodos, matérias e materiais acadêmicos em texto. Cada conteúdo fica em um único lugar, e os demais arquivos apontam para ele por link. PDFs e documentos ficam fora do Git; o que é versionado é o texto extraído, com a página e a fonte oficial.

## Endereços oficiais

- [Portal do Aluno UVA](https://portalaluno.uva.br/Aluno/PortalAluno)
- [Canvas UVA](https://uva.instructure.com/)
- [Como navegar no Portal e nos seus subsistemas](docs/NAVEGACAO_PORTAL_UVA.md)

As credenciais são digitadas somente no Portal e não são armazenadas aqui.

## Mapa do acervo

| Pasta | O que guarda | Entrada |
|---|---|---|
| `cursos/` | uma pasta por programa: nome, código, currículo | [cursos](privado/cursos/README.md) |
| `periodos/` | uma pasta por semestre, ligada a cursos e matérias | [períodos](periodos/README.md) |
| `materias/` | uma pasta por matéria: atividades, aulas e notas | [matérias](materias/README.md) |
| `fontes-uva/` | textos extraídos de PDFs oficiais, por período e matéria | [fontes UVA](fontes-uva/README.md) |
| `dados/` | registros estruturados, um por arquivo (JSON) | [dados](dados/LEIA-ME.md) |
| `docs/` | arquitetura, estado do aplicativo e guias | [arquitetura](docs/ARQUITETURA_ACERVO.md) |
| `app/` | aplicativo local Estuda UVA | [app](app/README.md) |
| `tools/acervo/` | gerador e validador de `dados/` | [gerador](tools/acervo/README.md) |

Fora do Git: `originais/` (documentos Word e Pages) e `build/` (saída do gerador).

## Índices e planejamento

- [Planejamento e rotina de aulas](privado/PLANEJAMENTO.md)
- [Registro acadêmico (pessoal)](privado/REGISTRO_ACADEMICO.md)
- [Engenharia de Computação — matrícula anterior](privado/cursos/engenharia-de-computacao/README.md)
- [Histórico de cursos e materiais](privado/periodos/historicos/README.md)
- [2026.2: disciplinas, materiais e notas de estudo](privado/periodos/2026.2/README.md)
- [2026.1: disciplinas, materiais, notas de estudo e calendário](privado/periodos/2026.1/README.md)
- [Semestres futuros](periodos/futuros.md)
- [Trabalhos por matéria](materias/README.md)
- [Estuda UVA: estado de implementação](docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md)

## Git e arquivos

O que é do aluno fica em `privado/`, dentro deste mesmo repositório, separado de matérias e instituição. Tudo é versionado como texto. Documentos e imagens locais ficam em `originais/binarios/`, ignorado pelo Git.

Repositório local na branch `master`, sem remoto configurado. Versionamos Markdown, JSON e textos extraídos. PDFs, imagens e documentos Word ou Pages ficam fora do Git; quando um PDF é útil, o texto dele vai para `fontes-uva/`, com a página e o link oficial. Diagramas viram Mermaid ou descrição textual. Credenciais, cookies e identificadores pessoais não entram no repositório. Para listar binários que ainda restam, use `node tools/acervo/build.mjs --verificar-texto`.
