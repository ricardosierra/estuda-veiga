# Paradigmas de Linguagens de Programação — aula inicial de apoio

> **Material suplementar, não oficial da UVA.** Em 14 de outubro de 2026, o curso Canvas 57159 ainda exibia um módulo identificado como ASI1, e os PDFs associados tratam de análise e projeto de sistemas, não de paradigmas. Esta aula não usa esses PDFs como conteúdo da disciplina e não afirma reproduzir a ementa, sequência ou avaliação UVA. Confirme o programa correto com a UVA.
>
> Elaborada para estudo local a partir de materiais acadêmicos da Carnegie Mellon University: [15-312 Foundations of Programming Languages](https://www.cs.cmu.edu/~crary/312/index.html) (descrição inclui paradigmas funcionais, imperativos, concorrentes e orientados a objetos; página de 2005) e [17-363 Programming Language Pragmatics, introdução](https://www.cs.cmu.edu/~aldrich/courses/17-363-fa21/slides/lecture1-introduction.pdf) (slides de 2021 sobre linguagens, projeto e implementação; consultados em 14/10/2026). Esses materiais externos não foram arquivados como PDFs UVA.

## 1. O que é um paradigma?

**Paradigma de programação** é uma maneira de organizar e expressar a solução de um problema. Ele influencia como descrevemos dados, passos, funções e interações. Paradigmas ajudam a comparar ideias; não significam que toda linguagem pertença a uma única categoria exclusiva.

O material da CMU enfatiza que línguas e estilos podem se sobrepor: uma mesma linguagem pode permitir código imperativo, funcional e orientado a objetos. É mais útil reconhecer as construções usadas no programa do que classificar a linguagem com um único rótulo.

## 2. Três maneiras de transformar uma lista

Considere `numeros = [1, 2, 3]`. Queremos produzir `[1, 4, 9]`.

### Imperativo

Descrevemos comandos e atualizações de estado passo a passo:

```python
quadrados = []
for numero in numeros:
    quadrados.append(numero * numero)
```

O foco está na sequência: criar uma lista, percorrer os valores e acrescentar cada resultado.

### Funcional

Descrevemos uma transformação de valores usando uma função:

```python
quadrados = list(map(lambda numero: numero * numero, numeros))
```

O foco está em aplicar a mesma transformação a cada elemento. Em estilo funcional, costuma-se privilegiar expressões, composição de funções e menor dependência de alterações compartilhadas de estado.

### Orientado a objetos

Organizamos dados e operações em objetos que colaboram. Por exemplo, uma classe `Medida` poderia guardar um valor e oferecer um método para calcular seu quadrado. O foco seria a responsabilidade do objeto e a interação por métodos; para uma transformação simples de lista, isso pode acrescentar estrutura sem benefício.

## 3. Como comparar as soluções

Ao ler um programa, pergunte:

1. Que estado é guardado ou alterado?
2. A solução é uma sequência de comandos ou uma composição de transformações?
3. Que dados e operações pertencem juntos a cada objeto?
4. O estilo deixa claro o comportamento que queremos e facilita testar ou mudar o programa?

Uma linguagem pode oferecer suporte a mais de um estilo. A escolha deve considerar o problema, a clareza e as garantias necessárias, não apenas o nome do paradigma.

## 4. Pratique identificando o estilo

Um programa percorre uma lista, calcula cada valor com a mesma função e monta uma nova lista sem alterar a original. Qual ideia aparece com mais força? Explique em uma frase.

**Resposta para conferência:** o estilo funcional aparece com mais força, pois o foco está em transformar cada elemento e produzir um novo resultado. A presença dessa operação, sozinha, não torna toda a linguagem ou o programa “puramente funcional”.

## Limites desta aula

Este material apresenta conceitos introdutórios e não confirma os tópicos oficiais de Paradigmas da UVA. A disciplina ainda precisa de plano e materiais corretos da instituição; os PDFs de ASI1 continuam separados e pendentes de esclarecimento.
