# Acervo acadêmico — Faculdade

Repositório local para organizar planejamento, cursos, notas e materiais acadêmicos em formatos de texto. Os PDFs são consultados por seus links oficiais; quando acessíveis, sua camada textual é extraída para arquivos versionáveis. Após conferir a extração, a cópia PDF local é descartada para economizar espaço.

## Endereços oficiais

- [Portal do Aluno UVA](https://portalaluno.uva.br/Aluno/PortalAluno)
- [Canvas UVA](https://uva.instructure.com/)
- [Como navegar no Portal e nos seus 5 subsistemas (Lyceum, Canvas, Sistema de Provas, Zeev, Pagamento)](docs/NAVEGACAO_PORTAL_UVA.md)

As credenciais devem ser digitadas diretamente no Portal. Não são armazenadas neste repositório.

## Índices e planejamento

- [Planejamento e passo a passo para acompanhar as aulas](PLANEJAMENTO.md)
- [Registro acadêmico e frequência](REGISTRO_ACADEMICO.md)
- [Cursos e materiais anteriores](CURSOS_HISTORICOS.md) · [Engenharia de Computação — vínculo evadido](ENGENHARIA_DE_COMPUTACAO.md) · [inventário histórico](MATERIAIS_HISTORICOS.md) · [notas históricas adicionais](NOTAS_DE_ESTUDO_HISTORICOS.md)
- [Disciplinas matriculadas em 2026.2](CURSOS_2026.2.md) · [inventário de materiais](MATERIAIS_2026.2.md) · [notas de estudo](NOTAS_DE_ESTUDO_2026.2.md) · [textos extraídos](materias/fontes-uva/2026.2/README.md)
- [Disciplinas, unidades e links de 2026.1](CURSOS_2026.1.md) · [inventário de materiais](MATERIAIS_2026.1.md) · [notas de estudo](NOTAS_DE_ESTUDO_2026.1.md) · [textos extraídos dos 28 PDFs](materias/fontes-uva/2026.1/README.md)
- [Semestres futuros](FUTUROS_SEMESTRES.md)
- [Calendário acadêmico transcrito de 2026.1](CALENDARIO_2026.1.md)
- [Trabalhos em texto por matéria](materias/README.md)
- [Dados estruturados do acervo (um registro por arquivo) e como contribuir](dados/LEIA-ME.md) · [gerador e visualizações](tools/acervo/README.md)
- [Aplicativo local Estuda UVA e passo a passo](app/README.md) · [estado de implementação e verificações](docs/ESTADO_IMPLEMENTACAO_ESTUDA_UVA.md)

## Git e arquivos

Repositório Git local, na branch master, sem remoto configurado. Documentos Markdown, notas autorais e extrações textuais são versionados. PDFs e arquivos grandes ficam fora do histórico Git; todo PDF ou documento vira Markdown com a página e a URL oficial, e todo diagrama vira Mermaid ou descrição textual. Binários ficam fora do Git (`node tools/acervo/build.mjs --verificar-texto` aponta os que restam). Arquivos Word e Pages-fonte de trabalhos ficam em originais, ignorado pelo Git. Credenciais e identificadores pessoais não devem ser adicionados ao repositório. Binários acadêmicos e de mídia são ignorados pelo Git; os diagramas UML foram convertidos para Mermaid.
