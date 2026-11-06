# Projeto e Análise de Autômato Finito Determinístico para Reconhecimento de Cadeias Binárias Terminadas em 1

> Linguagens Formais e Autômatos (E10300019 / Curso Canvas: 57904) · AVA 1. Conversão textual do trabalho acadêmico; nome completo e matrícula removidos para versionamento no repositório.

---

## 1 INTRODUÇÃO E CONTEXTUALIZAÇÃO TEÓRICA

A teoria da computação e a teoria dos autômatos constituem a pedra angular da Ciência da Computação moderna, fornecendo os fundamentos matemáticos necessários para modelar, compreender e projetar dispositivos de hardware, sistemas operacionais, compiladores e protocolos de comunicação (SIPSER, 2007; HOPCROFT; MOTWANI; ULLMAN, 2001).

No nível mais elementar da Hierarquia de Chomsky situam-se as **linguagens regulares**, cuja classe de reconhecedores abstratos equivale exatamente aos **Autômatos Finitos Determinísticos (AFDs)** e aos **Autômatos Finitos Não-Determinísticos (AFNDs)**. Um autômato finito é um modelo matemático idealizado para sistemas computacionais que possuem uma quantidade finita e estritamente delimitada de memória interna (RAM ou registradores). Ao contrário de dispositivos com memória irrestrita (como as Máquinas de Turing com sua fita infinita) ou memória estruturada em pilha (como os Autômatos com Pilha para linguagens livres de contexto), o autômato finito opera inspecionando uma cadeia de entrada da esquerda para a direita, símbolo a símbolo, em sentido único, alterando seu estado interno a cada transição (RAMOS; NETO; VEGA, 2009).

Muitos problemas do mundo real reduzem-se à verificação de padrões regulares em fluxos sequenciais de bits ou caracteres. Reconhecer se uma cadeia termina com determinado caractere ou prefixo/sufixo específico é um requisito onipresente em:
1. **Análise Léxica de Compiladores (Scanners):** identificação de delimitadores de comandos, literais inteiros e pontuação de sintaxe.
2. **Circuitos Lógicos e Máquinas de Estados Sequenciais (FSM):** implementação física em circuitos integrados (FPGA / ASIC) de detectores de paridade e controladores de barramento.
3. **Protocolos de Rede e Comunicação:** detecção de flags de finalização de quadro (*frame delimiters*), como na camada de enlace do protocolo HDLC.

O presente trabalho tem como objetivo desenvolver, de maneira analítica, rigorosa e detalhada, a resolução da Atividade 1 (AVA 1), projetando o autômato finito determinístico capaz de reconhecer todas as cadeias do alfabeto binário $\Sigma = \{0, 1\}$ que terminam com o símbolo $1$. Para atender plenamente à rubrica avaliativa (Acerto: 60 pts; Desenvolvimento: 30 pts; Interpretação: 10 pts), são apresentadas a definição formal da 5-tupla, a tabela de transição de estados, a representação gráfica do diagrama de estados, a simulação e rastreamento passo a passo de computações em cadeias de teste, a prova indutiva de corretude e a demonstração formal de minimalidade de estados baseada no Teorema de Myhill-Nerode.

---

## 2 OBJETIVOS E ESPECIFICAÇÃO FORMAL DA MÁQUINA

### 2.1 Definição da Linguagem Formal

Dado o alfabeto de entrada binário:
$$\Sigma = \{0, 1\}$$

Define-se $\Sigma^*$ como o conjunto de todas as palavras possíveis formadas sobre $\Sigma$, incluindo a palavra vazia $\varepsilon$. A linguagem regular solicitada pela atividade, denotada por $L$, é o conjunto de todas as palavras $w \in \Sigma^*$ cujo último símbolo é $1$:

$$L = \{ w \in \{0, 1\}^* \mid w \text{ termina com '1'} \}$$

Em termos de expressões regulares, a linguagem pode ser expressa sucintamente por:
$$R = (0 \cup 1)^* 1$$

Isso implica as seguintes propriedades fundamentais para a linguagem:
- A palavra vazia $\varepsilon$ não possui símbolos; logo, não termina com $1$ e não pertence à linguagem ($\varepsilon \notin L$).
- Nenhuma palavra cujo último caractere seja $0$ pertence à linguagem ($w0 \notin L, \, \forall w \in \Sigma^*$).
- Toda palavra cujo último caractere seja $1$ pertence à linguagem ($w1 \in L, \, \forall w \in \Sigma^*$).

### 2.2 Definição Matemática da 5-Tupla

Segundo a definição formal consagrada na literatura (SIPSER, 2007, p. 35; HOPCROFT; MOTWANI; ULLMAN, 2001, p. 45), um Autômato Finito Determinístico (AFD) é caracterizado matematicamente por uma 5-tupla:

$$M = (Q, \Sigma, \delta, q_0, F)$$

Para o problema proposto, os elementos da 5-tupla são rigorosamente especificados como:

1. **$Q = \{q_0, q_1\}$**: Conjunto finito de estados da máquina.
2. **$\Sigma = \{0, 1\}$**: Alfabeto finito dos símbolos de entrada aceitos pelo autômato.
3. **$\delta : Q \times \Sigma \to Q$**: Função total de transição determinística, que mapeia cada par ordenado (estado atual, símbolo lido) em exatamente um único próximo estado.
4. **$q_0 \in Q$**: Estado inicial do autômato, no qual o processamento de qualquer palavra necessariamente se inicia antes da leitura do primeiro símbolo.
5. **$F = \{q_1\} \subseteq Q$**: Conjunto finito de estados de aceitação (ou estados finais). Caso a computação termine com o autômato no estado $q_1$, a cadeia de entrada é aceita; caso contrário, é rejeitada.

### 2.3 Semântica e Papel dos Estados

A memória do autômato reside exclusivamente no estado ativo. Cada estado possui uma invariante semântica bem definida:

- **Estado $q_0$ (Estado Inicial de Não-Aceitação / Rejeição):**
  - **Condição Representada:** A palavra consumida até o momento é vazia ($\varepsilon$) ou tem como último símbolo o dígito $0$.
  - **Status:** $q_0 \notin F$. Se a cadeia de entrada encerrar quando a máquina estiver em $q_0$, a cadeia é categoricamente **rejeitada**.

- **Estado $q_1$ (Estado Final de Aceitação):**
  - **Condição Representada:** A palavra consumida até o momento tem como último símbolo o dígito $1$.
  - **Status:** $q_1 \in F$. Se a cadeia de entrada encerrar quando a máquina estiver em $q_1$, a cadeia é formalmente **aceita**.

### 2.4 Tabela de Transição de Estados ($\delta$)

A função de transição determinística $\delta$ pode ser representada de maneira exaustiva por meio da matriz tabular apresentada na Tabela 1:

**Tabela 1: Tabela de transição de estados do AFD $M$**

| Estado Atual ($q$) | Símbolo de Entrada `0` ($\delta(q, 0)$) | Símbolo de Entrada `1` ($\delta(q, 1)$) | Condição no Encerramento |
| :---: | :---: | :---: | :---: |
| $\to q_0$ (Inicial) | $q_0$ | $q_1$ | Rejeição ($q_0 \notin F$) |
| $* q_1$ (Final) | $q_0$ | $q_1$ | **Aceitação** ($q_1 \in F$) |

*Convenção notacional clássica: A seta ($\to$) indica o estado inicial; o asterisco ($*$) identifica o estado final de aceitação.*

#### Justificativa Analítica das Transições:
- **$\delta(q_0, 0) = q_0$:** Se a máquina está em $q_0$ (histórico vazio ou terminado em $0$) e consome o símbolo $0$, o novo último caractere continua sendo $0$. Portanto, o autômato deve permanecer em $q_0$.
- **$\delta(q_0, 1) = q_1$:** Se a máquina está em $q_0$ e consome o símbolo $1$, o novo último caractere passa a ser $1$. Portanto, o autômato deve transitar para o estado aceitador $q_1$.
- **$\delta(q_1, 0) = q_0$:** Se a máquina está em $q_1$ (a cadeia até então terminava em $1$) e consome o símbolo $0$, o novo último caractere passa a ser $0$, revogando imediatamente a condição de aceitação. Portanto, o autômato deve transitar de volta para o estado de rejeição $q_0$.
- **$\delta(q_1, 1) = q_1$:** Se a máquina está em $q_1$ e consome mais um símbolo $1$, o último caractere continua sendo $1$. Portanto, o autômato permanece no estado de aceitação $q_1$ por meio de um laço reflexivo (*self-loop*).

---

## 3 DIAGRAMA DE ESTADOS DO AUTÔMATO FINITO DETERMINÍSTICO

### 3.1 Representação Gráfica

O diagrama de estados descreve visualmente a estrutura de grafo direcionado do autômato finito:

```
            +---+ 0
            |   |
            v   |
   Início   +------+       1       +======+ <---+ 1
 ---------> |  q0  | ------------> |  q1  |     |
            +------+ <------------ +======+ ----+
                           0
```

- **Estado Inicial ($q_0$):** Círculo simples com seta de entrada "Início".
- **Estado de Aceitação ($q_1$):** Círculo concêntrico duplo indicando estado final pertencente a $F$.
- **Transições:** Arcos orientados rotulados com os símbolos consumidos do alfabeto $\{0, 1\}$.

*(O arquivo gráfico de alta resolução para postagem no AVA encontra-se na pasta local de originais).*

### 3.2 Arquivo de Projeto JFLAP (`automato_termina_em_1.jff`)

Em atendimento aos procedimentos recomendados na atividade, o autômato foi implementado na ferramenta de simulação acadêmica JFLAP. O modelo estrutural XML compatível está disponível no arquivo anexo `automato_termina_em_1.jff`.

---

## 4 DESENVOLVIMENTO, RASTREAMENTO PASSO A PASSO E SIMULAÇÃO

### 4.1 Função de Transição Estendida ($\hat{\delta}$)

Para avaliar cadeias completas $w \in \Sigma^*$, utiliza-se a função de transição estendida $\hat{\delta} : Q \times \Sigma^* \to Q$, definida indutivamente por:

1. **Base:** Para a palavra vazia $\varepsilon$:
   $$\hat{\delta}(q, \varepsilon) = q, \quad \forall q \in Q$$

2. **Passo Indutivo:** Para qualquer palavra $w = xa$, com $x \in \Sigma^*$ e $a \in \Sigma$:
   $$\hat{\delta}(q, xa) = \delta(\hat{\delta}(q, x), a)$$

Uma palavra $w$ é aceita pelo autômato $M$ se e somente se:
$$\hat{\delta}(q_0, w) \in F$$

### 4.2 Bateria de Testes com Rastreamento de Estados

A Tabela 2 apresenta o teste e rastreamento de estados para um conjunto representativo de entradas binárias:

**Tabela 2: Rastreamento da computação de cadeias no AFD $M$**

| ID | Cadeia ($w$) | Sequência de Estados Percorrida no AFD | $\hat{\delta}(q_0, w)$ | Pertence a $F$? | Classificação | Critério Teórico |
|:--:|:---:|:---|:---:|:---:|:---:|:---:|
| T01 | $\varepsilon$ | $q_0$ (nenhum símbolo processado) | $q_0$ | Não ($q_0 \notin F$) | **Rejeitada** | Não termina em 1 |
| T02 | `0` | $q_0 \xrightarrow{0} q_0$ | $q_0$ | Não ($q_0 \notin F$) | **Rejeitada** | Termina em 0 |
| T03 | `1` | $q_0 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |
| T04 | `00` | $q_0 \xrightarrow{0} q_0 \xrightarrow{0} q_0$ | $q_0$ | Não ($q_0 \notin F$) | **Rejeitada** | Termina em 0 |
| T05 | `01` | $q_0 \xrightarrow{0} q_0 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |
| T06 | `10` | $q_0 \xrightarrow{1} q_1 \xrightarrow{0} q_0$ | $q_0$ | Não ($q_0 \notin F$) | **Rejeitada** | Termina em 0 |
| T07 | `11` | $q_0 \xrightarrow{1} q_1 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |
| T08 | `001` | $q_0 \xrightarrow{0} q_0 \xrightarrow{0} q_0 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |
| T09 | `010` | $q_0 \xrightarrow{0} q_0 \xrightarrow{1} q_1 \xrightarrow{0} q_0$ | $q_0$ | Não ($q_0 \notin F$) | **Rejeitada** | Termina em 0 |
| T10 | `101` | $q_0 \xrightarrow{1} q_1 \xrightarrow{0} q_0 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |
| T11 | `110` | $q_0 \xrightarrow{1} q_1 \xrightarrow{1} q_1 \xrightarrow{0} q_0$ | $q_0$ | Não ($q_0 \notin F$) | **Rejeitada** | Termina em 0 |
| T12 | `111` | $q_0 \xrightarrow{1} q_1 \xrightarrow{1} q_1 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |
| T13 | `01001011` | $q_0 \xrightarrow{0} q_0 \xrightarrow{1} q_1 \xrightarrow{0} q_0 \xrightarrow{0} q_0 \xrightarrow{1} q_1 \xrightarrow{0} q_0 \xrightarrow{1} q_1 \xrightarrow{1} q_1$ | $q_1$ | **Sim** ($q_1 \in F$) | **Aceita** | Termina em 1 |

### 4.3 Prova Indutiva de Corretude

> **Teorema:** Para qualquer cadeia não-vazia $w \in \Sigma^+$, onde $w = u \cdot a$ ($u \in \Sigma^*$ e $a \in \Sigma$):
> $$\hat{\delta}(q_0, w) = \begin{cases} q_1, & \text{se } a = 1 \\ q_0, & \text{se } a = 0 \end{cases}$$

**Demonstração por Indução no comprimento da cadeia $|w| = n$:**

- **Caso Base ($n = 1$):**
  - Se $w = 0$: $\hat{\delta}(q_0, 0) = \delta(q_0, 0) = q_0 \notin F$ (Rejeita).
  - Se $w = 1$: $\hat{\delta}(q_0, 1) = \delta(q_0, 1) = q_1 \in F$ (Aceita).
  Verificado.

- **Hipótese Indutiva:**
  Assuma que para qualquer cadeia $u$ de comprimento $k \ge 1$, $\hat{\delta}(q_0, u) = q \in \{q_0, q_1\}$.

- **Passo Indutivo ($n = k + 1$):**
  Para $w = u \cdot a$ ($a \in \{0, 1\}$), temos pela definição de $\hat{\delta}$:
  $$\hat{\delta}(q_0, w) = \delta(\hat{\delta}(q_0, u), a) = \delta(q, a)$$
  1. Se $a = 0$: independentemente de $q = q_0$ ou $q = q_1$, $\delta(q, 0) = q_0 \notin F$ (Rejeita).
  2. Se $a = 1$: independentemente de $q = q_0$ ou $q = q_1$, $\delta(q, 1) = q_1 \in F$ (Aceita).

Logo, o estado final depende exclusivamente de o último caractere ser $1$, provando formalmente a corretude do autômato. $\blacksquare$

---

## 5 INTERPRETAÇÃO, MINIMALIDADE E APLICAÇÕES PRÁTICAS

### 5.1 Invariante de Estados e Memória Finita

Um autômato finito determinístico possui memória estritamente limitada ao estado interno corrente. Para reconhecer se uma palavra arbitrária termina em $1$, a máquina precisa reter unicamente um bit de informação: o último símbolo consumido. O autômato resume todo o histórico de entrada com complexidade temporal $\mathcal{O}(n)$ e espacial $\mathcal{O}(1)$.

### 5.2 Prova de Minimalidade pelo Teorema de Myhill-Nerode

Pelo Teorema de Myhill-Nerode, o número de estados do AFD mínimo para uma linguagem $L$ é igual ao índice da relação de indistinguibilidade por sufixo $R_L$:
$$x \equiv_L y \iff (\forall z \in \Sigma^*, \, xz \in L \iff yz \in L)$$

A linguagem $L$ divide $\Sigma^*$ em exatamente duas classes de equivalência:
1. $[0] = \{ w \in \Sigma^* \mid w = \varepsilon \text{ ou } w \text{ termina em } 0 \}$;
2. $[1] = \{ w \in \Sigma^* \mid w \text{ termina em } 1 \}$.

Tomando $x = 0 \in [0]$ e $y = 1 \in [1]$, com o sufixo distinguidor $z = \varepsilon$:
- $xz = 0 \notin L$
- $yz = 1 \in L$

Como as duas classes são mutuamente distinguíveis por sufixo, o índice de Myhill-Nerode é 2. Logo, qualquer AFD que reconheça $L$ necessita de **no mínimo 2 estados**. O autômato desenvolvido atinge essa cota inferior, sendo comprovadamente **canonicamente mínimo**.

### 5.3 Aplicações Práticas no Mundo Real

1. **Compiladores e Análise Léxica:** Modelagem de analisadores de literais e operadores em scanners léxicos (como Lex/Flex).
2. **Circuitos Digitais (FSM Hardware):** Implementação física com um único flip-flop do tipo D em sistemas embarcados.
3. **Protocolos de Comunicação:** Detecção de delimitadores de pacotes seriais e flags de término de transmissão de dados.

---

## 6 CONSIDERAÇÕES FINAIS

O trabalho atende integralmente às exigências da AVA 1 de Linguagens Formais e Autômatos, fornecendo a modelagem conceitual, o diagrama estrutural no JFLAP, o desenvolvimento passo a passo com testes exaustivos e a fundamentação teórica completa com prova de minimalidade.

---

## REFERÊNCIAS BIBLIOGRÁFICAS

1. HOPCROFT, John E.; MOTWANI, Rajeev; ULLMAN, Jeffrey D. **Introdução à Teoria de Autômatos, Linguagens e Computação**. 2. ed. Rio de Janeiro: Campus/Elsevier, 2001. ISBN: 9788535210729.
2. RAMOS, Marcus Vinícius Midena; NETO, João José; VEGA, Ítalo Santiago. **Linguagens Formais: Teoria, Modelagem e Implementação**. Porto Alegre: Bookman, 2009. ISBN: 9788577804474.
3. SIPSER, Michael. **Introdução à Teoria da Computação**. 2. ed. São Paulo: Cengage Learning, 2007. Capítulo 1: Linguagens Regulares (Autômatos Finitos). ISBN: 9788522104994. Minha Biblioteca.
4. UNIVERSIDADE VEIGA DE ALMEIDA. **Linguagens Formais e Autômatos: Unidade 1 – Autômatos Finitos e Linguagens Regulares**. Rio de Janeiro: UVA, 2026.
