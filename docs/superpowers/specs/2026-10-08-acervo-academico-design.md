# Especificação: organização do acervo acadêmico

**Data:** 2026-10-08

**Pasta principal:** `/Users/sierra/Documents/Faculdade`

## Objetivo

Manter no histórico Git documentos acadêmicos em formatos textuais, organizados por matéria e atividade. Preservar fontes editáveis e complementares locais sem versionar originais binários redundantes; usar a Lixeira para remover cópias PDF recuperáveis quando há texto equivalente verificado.

## Escopo aprovado

- Os arquivos atualmente em `Faculdade`, incluindo os trabalhos locais de Bases de Programação e Paradigmas de Programação e a documentação já versionada para o semestre UVA 2026.1 (`README.md`, `CURSOS_2026.1.md`, `MATERIAIS_2026.1.md` e `PLANEJAMENTO.md`). Os documentos existentes de 2026.1 serão preservados e complementados.
- O arquivo Pages `../trabalho-paradigmas-ricardo.pages`, cuja prévia confirma o mesmo trabalho CSU01 – Realizar Saque Bancário e hoje está localizado na pasta pai. A capa chama a disciplina de “Paradigmas de Linguagens de Programação”, enquanto o DOCX local usa “Paradigmas de Programação”; os dois originais serão preservados sem forçar uma equivalência de versão.
- As cinco disciplinas UVA de 2026.1, com links oficiais e notas autorais das 20 unidades em `NOTAS_DE_ESTUDO_2026.1.md`. Os PDFs de aula não serão adicionados ao repositório; cópias locais temporárias podem ser usadas para extrair e conferir texto.
- Os PDFs U1 e U2 de Desenvolvimento Frontend I tiveram o texto extraído de cópias encontradas em Downloads para conferência. Depois da extração, essas cópias foram movidas para a Lixeira, onde podem ser recuperadas; os PDFs e a transcrição bruta não serão armazenados no repositório.

A implementação não deve inferir nem inventar conteúdo de materiais indisponíveis.

## Decisões de organização

1. Iniciar um repositório Git local em `Faculdade`.
2. Versionar um `README.md` de índice e instruções, documentos Markdown por atividade, o algoritmo VisuAlg como arquivo `.alg`, o texto alternativo existente de Paradigmas e os três diagramas PNG.
3. Guardar DOCX, os dois arquivos `.txt` de origem e Pages sob `originais/`, com essa pasta e essas extensões ignoradas pelo Git. Não reter cópias locais dos PDFs de aula ou PDFs redundantes de trabalhos.
4. Usar o DOCX como fonte de conversão de cada trabalho que também tenha PDF. A comparação dos pares Bases de Programação AVA 2 e Paradigmas AVA 2 produziu similaridade normalizada 1,000; não criar uma segunda cópia textual do mesmo trabalho por causa do PDF e mover essas cópias PDF para a Lixeira.
5. Manter separado o texto atual de Paradigmas AVA 2, pois ele difere materialmente do par DOCX/PDF. Preservar o código de Bases AVA 2 como material complementar, em vez de tratá-lo como conversão integral do relatório.
6. Os diagramas PNG, com 25–43 KB cada, permanecem versionados e são referenciados pelo Markdown de Paradigmas AVA 2. Seus conteúdos são idênticos às imagens incorporadas no DOCX.
7. Manter os endereços oficiais dos PDFs e as notas autorais; não adicionar PDFs ou transcrições integrais desses materiais ao repositório.
8. Não versionar nomes completos nem matrículas presentes nos trabalhos. Gerar versões textuais sem esses identificadores e preservar as fontes DOCX, TXT e Pages em `originais/`, fora do Git, conforme a regra já registrada no `README.md` atual.

## Estrutura proposta

```text
Faculdade/
├── README.md
├── .gitignore
├── CURSOS_2026.1.md
├── MATERIAIS_2026.1.md
├── PLANEJAMENTO.md
├── NOTAS_DE_ESTUDO_2026.1.md
├── docs/superpowers/specs/2026-10-08-acervo-academico-design.md
├── materias/
│   ├── README.md
│   ├── bases-de-programacao/
│   │   ├── ava1-apuracao-de-votos.md
│   │   ├── ava2-monitoramento-de-esteira.md
│   │   └── codigo/monitoramento-de-esteira.alg
│   ├── paradigmas-de-programacao/
│   │   ├── ava1-caso-de-uso-saque.md
│   │   ├── ava2-modelagem-uml-biblioteca.md
│   │   ├── versoes/modelagem-uml-texto-alternativo.md
│   │   └── diagramas/*.png
└── originais/  # ignorada pelo Git
    ├── bases-de-programacao/*.{docx,txt}
    ├── paradigmas-de-programacao/*.{docx,txt,pages}
```

O caminho do arquivo Pages será ajustado durante a implementação para preservar o nome original e deixar clara a relação com o trabalho de Paradigmas.

## Conversão e preservação

- Converter os quatro DOCX de trabalhos para Markdown com Pandoc, remover identificadores pessoais e ajustar títulos numerados para cabeçalhos Markdown.
- Comparar os PDFs de AVA 2 com seus DOCX usando texto normalizado, sem gerar arquivos textuais duplicados para os mesmos trabalhos.
- Manter os PDFs das unidades UVA somente nos links oficiais; as notas de estudo autorais em Markdown não são transcrições integrais dos PDFs.
- Conferir que o algoritmo extraído do `.txt` complementar continua legível e funcional como texto antes de renomeá-lo para `.alg`.
- Preservar a versão textual divergente de Paradigmas com indicação explícita de que é uma versão paralela, sem alegar qual versão é mais recente.
- Preservar localmente os originais binários em `originais/`; não removê-los depois da conversão.

## Verificação de aceite

- Todos os arquivos textuais de atividades estão sob `materias/<disciplina>/` e aparecem no índice raiz.
- O algoritmo e a versão paralela de Paradigmas permanecem identificáveis e pesquisáveis.
- Os arquivos de curso, planejamento, inventário e notas autorais de 2026.1 permanecem preservados e indexados.
- As três imagens são referenciadas na atividade UML.
- Nomes completos e matrículas estão ausentes do conteúdo rastreado.
- DOCX, PDFs, Pages, textos-fonte originais e `originais/` aparecem como ignorados e não fazem parte da lista de arquivos rastreados; PDFs de aula UVA e cópias PDF redundantes dos trabalhos não ficam no repositório ou em `originais/`.
- Os textos derivados correspondem às fontes locais; pares equivalentes não geram duplicação desnecessária.
- Os PDFs de Desenvolvimento Frontend I U1 e U2 tiveram texto extraído para conferir as notas; nenhum PDF ou texto integral é versionado.
- O repositório local registra os documentos textuais e o índice em commits revisáveis.

## Limitações conhecidas

Na primeira tentativa, o acesso direto ao PDF U2 respondeu HTTP 403 e o visualizador não permitiu copiar texto. Em uma etapa posterior, cópias de U1 e U2 encontradas em Downloads puderam ser processadas com extração local de texto; depois foram movidas para a Lixeira. A prévia de um PDF de critérios apareceu preta. Não se deve preencher o texto dos demais arquivos por suposição.

## Decisão posterior: material didático das disciplinas UVA

Em 8 de outubro de 2026, o estudante decidiu que o acervo deve guardar apenas notas em texto e links oficiais, sem cópias locais de PDFs das disciplinas. A revisão posterior dos 75 itens de módulos encontrou 34 PDFs únicos: 16 materiais de unidade, 10 documentos de apoio, 2 documentos institucionais e 6 PDFs anexados a trabalhos enviados. Os 16 PDFs de unidade incluem Leitura e Produção Acadêmica U2, que não aparecia no primeiro inventário. A lista atualizada está em `MATERIAIS_2026.1.md` e as notas das 20 unidades estão em `NOTAS_DE_ESTUDO_2026.1.md`.

As notas são sínteses autorais, não transcrições integrais. Em 8 de outubro, o texto dos PDFs de Desenvolvimento Web U1 (29 páginas) e U2 (31 páginas) foi extraído para conferência a partir de cópias encontradas em Downloads; depois, essas cópias foram movidas para a Lixeira. Dois PDFs locais de trabalhos pessoais também foram comparados aos DOCX correspondentes (similaridade normalizada 1,000) e movidos para a Lixeira; os DOCX, TXT e Pages permanecem sob `originais/`. Apenas notas e links foram adicionados ao repositório. A página HTML de U2 também apresenta os quatro temas e um estudo de caso. A prévia visual do PDF U2 permitiu ler as páginas 12–13. Os outros 14 PDFs de unidade, 10 documentos de apoio, 2 documentos institucionais e 6 anexos de trabalhos permanecem catalogados sem extração textual integral. A prévia de um documento de critérios ficou preta. O inventário registra materiais, links e limites sem afirmar a extração de todos os PDFs.
