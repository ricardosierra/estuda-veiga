# Especificação: organização do acervo acadêmico

**Data:** 2026-10-08

**Pasta principal:** `/Users/sierra/Documents/Faculdade`

## Objetivo

Manter no histórico Git documentos acadêmicos em formatos textuais, organizados por matéria e atividade. Separar os PDFs, documentos Word e outros originais binários para consulta local, sem perder nem reescrever as fontes.

## Escopo aprovado

- Os arquivos atualmente em `Faculdade`, que abrangem Bases de Programação e Paradigmas de Programação.
- O arquivo Pages `../trabalho-paradigmas-ricardo.pages`, cuja prévia confirma o mesmo trabalho CSU01 – Realizar Saque Bancário e hoje está localizado na pasta pai. A capa chama a disciplina de “Paradigmas de Linguagens de Programação”, enquanto o DOCX local usa “Paradigmas de Programação”; os dois originais serão preservados sem forçar uma equivalência de versão.
- O PDF U2 de Desenvolvimento Frontend I aberto na plataforma UVA. A tentativa de obter esse arquivo retornou HTTP 403; seu conteúdo não será representado por texto até que a fonte esteja acessível.

A implementação não deve inferir nem inventar conteúdo de materiais indisponíveis.

## Decisões de organização

1. Iniciar um repositório Git local em `Faculdade`.
2. Versionar um `README.md` de índice e instruções, documentos Markdown por atividade, o algoritmo VisuAlg como arquivo `.alg`, o texto alternativo existente de Paradigmas e os três diagramas PNG.
3. Guardar DOCX, PDF e Pages sob `originais/`, com essa pasta e essas extensões ignoradas pelo Git.
4. Usar o DOCX como fonte de conversão de cada trabalho que também tenha PDF. A extração dos pares Bases de Programação AVA 2 e Paradigmas AVA 2 mostrou texto idêntico; não criar uma segunda cópia textual do mesmo conteúdo por causa do PDF.
5. Manter separado o texto atual de Paradigmas AVA 2, pois ele difere materialmente do par DOCX/PDF. Preservar o código de Bases AVA 2 como material complementar, em vez de tratá-lo como conversão integral do relatório.
6. Os diagramas PNG, com 25–43 KB cada, permanecem versionados e são referenciados pelo Markdown de Paradigmas AVA 2. Seus conteúdos são idênticos às imagens incorporadas no DOCX.
7. Registrar Desenvolvimento Frontend I U2 no índice como fonte pendente, usando o endereço `https://ead.uva.br/disciplinas/grad/publica/cont/def/dfe1/re/u2/documents/Ilumno_Desenvolvimento-Frontend-I_U2.pdf`. Não produzir Markdown de conteúdo sem conseguir obter o PDF.

## Estrutura proposta

```text
Faculdade/
├── README.md
├── .gitignore
├── docs/superpowers/specs/2026-10-08-acervo-academico-design.md
├── materias/
│   ├── bases-de-programacao/
│   │   ├── ava1-apuracao-de-votos.md
│   │   ├── ava2-monitoramento-de-esteira.md
│   │   └── codigo/monitoramento-de-esteira.alg
│   ├── paradigmas-de-programacao/
│   │   ├── ava1-caso-de-uso-saque.md
│   │   ├── ava2-modelagem-uml-biblioteca.md
│   │   ├── versoes/modelagem-uml-texto-alternativo.txt
│   │   └── diagramas/*.png
└── originais/  # ignorada pelo Git
    ├── bases-de-programacao/*.{docx,pdf}
    ├── paradigmas-de-programacao/*.{docx,pdf,pages}
    └── desenvolvimento-frontend-i/u2/Ilumno_Desenvolvimento-Frontend-I_U2.pdf  # somente se acessível
```

O caminho do arquivo Pages será ajustado durante a implementação para preservar o nome original e deixar clara a relação com o trabalho de Paradigmas.

## Conversão e preservação

- Converter DOCX para Markdown com Pandoc e ajustar títulos numerados para cabeçalhos Markdown.
- Extrair o texto dos PDFs com `pdftotext` para comparação com o Markdown equivalente; os PDFs locais não exigem conteúdo duplicado quando o DOCX correspondente é idêntico.
- Conferir que o algoritmo extraído do `.txt` complementar continua legível e funcional como texto antes de renomeá-lo para `.alg`.
- Preservar a versão textual divergente de Paradigmas com indicação explícita de que é uma versão paralela, sem alegar qual versão é mais recente.
- Preservar localmente os originais binários em `originais/`; não removê-los depois da conversão.

## Verificação de aceite

- Todos os arquivos textuais de atividades estão sob `materias/<disciplina>/` e aparecem no índice raiz.
- O algoritmo e a versão paralela de Paradigmas permanecem identificáveis e pesquisáveis.
- As três imagens são referenciadas na atividade UML.
- DOCX, PDF, Pages e `originais/` aparecem como ignorados e não fazem parte da lista de arquivos rastreados.
- Os textos derivados correspondem às fontes locais; pares equivalentes não geram duplicação desnecessária.
- Desenvolvimento Frontend I U2 fica marcado como pendente enquanto o servidor devolver 403; nenhum conteúdo é fabricado.
- O repositório local registra os documentos textuais e o índice em commits revisáveis.

## Limitações conhecidas

A UVA respondeu HTTP 403 ao pedido do PDF U2 e exibiu uma página padrão do IIS no endereço raiz. O PDF não pode ser convertido até que o arquivo esteja acessível no ambiente local.
