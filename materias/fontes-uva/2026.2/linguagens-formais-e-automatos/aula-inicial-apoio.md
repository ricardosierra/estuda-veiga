# Linguagens Formais e Autômatos — aula inicial de apoio

> **Material suplementar, não oficial da UVA.** Em 14 de outubro de 2026, o curso Canvas 57904 informava “Nenhum módulo foi definido para este curso”; a página Programa não exibia plano ou conteúdo. Esta aula introduz um tópico comum de teoria da computação e não afirma reproduzir a ementa, sequência ou avaliação da disciplina UVA. Confira o escopo oficial com a professora ou o professor quando estiver disponível.
>
> Elaborada para estudo local a partir de [MIT OpenCourseWare 18.404J, Lecture 1: Finite Automata, Regular Expressions](https://ocw.mit.edu/courses/18-404j-theory-of-computation-fall-2020/resources/lecture-1-introduction-finite-automata-regular-expressions/) (curso de graduação/pós, outono de 2020; consultado em 14/10/2026). O PDF da aula aparece como recurso externo e não foi copiado para o acervo UVA.

## 1. Alfabeto, palavras e linguagens

**Alfabeto** é um conjunto finito de símbolos. Se `Σ = {0, 1}`, cada símbolo é `0` ou `1`.

**Palavra** é uma sequência finita de símbolos do alfabeto. `0101` é uma palavra de `Σ`; a palavra vazia costuma ser escrita `ε` e tem comprimento zero.

**Linguagem** é qualquer conjunto de palavras sobre um alfabeto: `L ⊆ Σ*`. Por exemplo, sobre `{0,1}`, a linguagem das palavras que terminam em `01` é `{01, 001, 101, 0001, ...}`.

## 2. Autômato finito determinístico

**Definição:** um AFD é uma máquina com um número finito de estados que lê uma palavra símbolo por símbolo e decide se a aceita. Podemos representá-lo por `(Q, Σ, δ, q₀, F)`: estados `Q`, alfabeto `Σ`, transição `δ`, estado inicial `q₀` e estados finais `F`.

Em um AFD, cada par de estado e símbolo determina exatamente um próximo estado. A execução começa em `q₀`, consome todos os símbolos e aceita a palavra se terminar em um estado de `F`.

### Exemplo: palavras que terminam em `01`

Use os estados:

- `q₀`: até agora não há sufixo útil;
- `q₁`: a palavra lida termina em `0`;
- `q₂`: a palavra lida termina em `01` (estado aceitador).

| Estado atual | Entrada `0` | Entrada `1` |
|---|---|---|
| `q₀` (inicial) | `q₁` | `q₀` |
| `q₁` | `q₁` | `q₂` |
| `q₂` (aceitador) | `q₁` | `q₀` |

Rastreando `1101`: `q₀ → q₀ → q₀ → q₁ → q₂`; a palavra é aceita. Rastreando `1100`, termina em `q₁`, então é rejeitada.

## 3. Reconhecer uma palavra e descrever a linguagem

Um autômato reconhece uma linguagem inteira: aceitar ou rejeitar uma palavra é uma consulta ao conjunto definido pela máquina. Isso é diferente de dizer que o autômato “gera” todas as palavras aceitas.

Uma expressão regular oferece outra forma de descrever linguagens regulares. Expressões e autômatos ajudam a especificar padrões, mas a descrição precisa respeitar o alfabeto e a definição do padrão. Para strings que terminam em `01`, o AFD acima torna explícito como o estado acompanha o sufixo relevante.

## 4. Pratique rastreando o AFD

Sem olhar a seção de respostas, percorra os estados do AFD para `0001`, `0100`, `10101` e `ε`. Anote o estado final e, depois, classifique cada palavra como aceita ou rejeitada.

**Resposta para conferência:** `0001` termina em `q₂` e é aceita; `0100` termina em `q₁` e é rejeitada; `10101` termina em `q₂` e é aceita; `ε` permanece em `q₀` e é rejeitada.

## Limites desta aula

Esta aula cobre apenas uma introdução a linguagens e AFDs. Ela não confirma que o material UVA do semestre seguirá este exemplo ou cobrirá esses tópicos nessa ordem. O plano didático oficial continua desconhecido no Canvas consultado.
