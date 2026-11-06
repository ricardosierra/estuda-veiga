# Estudo de Funções Custo, Custo Marginal, Pontos Críticos e Segunda Derivada

> Cálculo de Uma Variável (57352 / E10100002) · AVA 1. Conversão textual do trabalho acadêmico; nome completo e matrícula removidos para versionamento no repositório.

---

## 1 INTRODUÇÃO

Nas últimas décadas, a matemática consolidou-se como instrumento indispensável para o suporte à tomada de decisões em cenários econômicos e operacionais de alta complexidade. Em um contexto dominado por massas massivas de dados (*Big Data*), com centenas ou milhares de variáveis simultâneas (taxas de juros, flutuações cambiais, demanda estocástica, restrições orçamentárias e custos logísticos), métodos quantitativos e o Cálculo Diferencial e Integral tornaram-se o alicerce para modelar comportamentos, prever resultados e otimizar a alocação de recursos escassos (STEWART, 2013; THOMAS et al., 2012).

No âmbito da engenharia de produção e da microeconomia, a análise do comportamento dos custos de fabricação é primordial. Uma organização precisa compreender não apenas o custo total incorrido para uma dada escala de produção, mas principalmente a taxa na qual esse custo se altera quando o volume produzido varia em uma unidade infinitesimal. Essa taxa de variação instantânea é denominada **Custo Marginal**, obtida matematicamente pela primeira derivada da função custo total em relação à quantidade produzida.

Além disso, a identificação dos **pontos críticos** de uma função custo permite reconhecer patamares de estabilização da taxa de variação, enquanto a análise da **segunda derivada** revela a concavidade da curva de custos. Essa curvatura fornece a distinção fundamental entre regimes de rendimentos marginais crescentes (ganhos de escala e eficiência produtiva) e regimes de rendimentos marginais decrescentes (deseconomias de escala, gargalos e saturação fabril), identificando com precisão pontos de máximo, mínimo ou pontos de inflexão (LEITHOLD, 1994).

O presente trabalho tem como objetivo desenvolver, de maneira analítica, rigorosa e detalhada, a resolução da Atividade 1 (AVA 1), aplicando as regras de derivação de primeira e segunda ordens, a investigação de pontos críticos e a análise de concavidade e extremos para as funções de custo propostas para quatro fabricantes.

---

## 2 FUNDAMENTAÇÃO TEÓRICA E DIRETRIZES DE DERIVAÇÃO

### 2.1 Conceito e Definição de Derivada

Dada uma função $C: \mathbb{R} \to \mathbb{R}$ diferenciável em um ponto $x$ do seu domínio, a derivada $C'(x)$ representa o limite da taxa média de variação da função quando o incremento $\Delta x$ tende a zero:

$$C'(x) = \lim_{\Delta x \to 0} \frac{C(x + \Delta x) - C(x)}{\Delta x} = \lim_{h \to 0} \frac{C(x + h) - C(x)}{h}$$

Geometricamente, $C'(x)$ expressa a inclinação da reta tangente à curva no ponto $(x, C(x))$. Economicamente, se $C(x)$ representa o custo total para produzir $x$ unidades, $C'(x)$ é o **Custo Marginal**, que aproxima o custo adicional necessário para fabricar a $(x+1)$-ésima unidade.

### 2.2 Regras Básicas de Derivação Empregadas

Com base nas propriedades discutidas nas Unidades 1 e 2 da disciplina:

1. **Regra da Constante**: A derivada de uma função constante é nula:
   $$\frac{d}{dx}[c] = 0, \quad \forall c \in \mathbb{R}$$

2. **Regra da Potência**: Para qualquer expoente real $n$:
   $$\frac{d}{dx}[x^n] = n \cdot x^{n-1}$$

3. **Multiplicação por Escalar**: Para qualquer constante real $k$:
   $$\frac{d}{dx}[k \cdot f(x)] = k \cdot \frac{d}{dx}[f(x)]$$

4. **Regra da Soma e Diferença**: A derivada da soma algébrica de funções é a soma algébrica das derivadas:
   $$\frac{d}{dx}[f(x) \pm g(x)] = f'(x) \pm g'(x)$$

### 2.3 Pontos Críticos

Um ponto $x = c$ pertencente ao domínio da função $C(x)$ é denominado **ponto crítico** se:
$$C'(c) = 0 \quad \text{ou} \quad C'(c) \text{ não existe}$$

Nos pontos em que $C'(c) = 0$, a reta tangente à curva é horizontal. Esses pontos são candidatos a pontos de extremo local (máximo ou mínimo relativo) ou podem representar patamares de inflexão com tangente horizontal.

### 2.4 Derivadas de Ordem Superior e Teste da Segunda Derivada

A segunda derivada, denotada por $C''(x)$, é a derivada de $C'(x)$:
$$C''(x) = \frac{d^2 C}{dx^2} = \frac{d}{dx}\left[C'(x)\right]$$

A segunda derivada mede a aceleração ou taxa de variação da inclinação da reta tangente, determinando a **concavidade** da função:
- Se $C''(x) > 0$ em um intervalo, o gráfico é **côncavo para cima** ($\cup$).
- Se $C''(x) < 0$ em um intervalo, o gráfico é **côncavo para baixo** ($\cap$).

O **Teste da Segunda Derivada** para classificação de pontos críticos estabelece:
- Se $C'(c) = 0$ e $C''(c) > 0$, então $x = c$ é um ponto de **mínimo relativo**.
- Se $C'(c) = 0$ e $C''(c) < 0$, então $x = c$ é um ponto de **máximo relativo**.
- Se $C'(c) = 0$ e $C''(c) = 0$, o teste é inconclusivo, devendo-se recorrer ao teste da primeira derivada ou a derivadas de ordens superiores.

### 2.5 Pontos de Inflexão

Um ponto $x = c$ no qual o gráfico de uma função contínua muda sua concavidade (passando de côncavo para cima a côncavo para baixo, ou vice-versa) é denominado **ponto de inflexão**. Uma condição necessária para a ocorrência de ponto de inflexão em funções duas vezes deriváveis é:
$$C''(c) = 0$$
acompanhada da efetiva alternância de sinal de $C''(x)$ à esquerda e à direita de $c$. No ponto de inflexão, a função custo marginal $C'(x)$ atinge um valor extremo local (máximo ou mínimo local da taxa de variação).

---

## 3 RESOLUÇÃO DETALHADA DAS FUNÇÕES PROPOSTAS

São apresentadas a seguir as análises completas para os quatro fabricantes propostos nas diretivas de execução.

---

### 3.1 FABRICANTE A

A função de custo total do Fabricante A é dada por:
$$C(x) = (10^{-6}) x^3 - 0,003 x^2 + 5x + 1000$$

onde $10^{-6} = 0,000001$ e $0,003 = 3 \cdot 10^{-3}$.

#### Item I: Cálculo da Função Custo Marginal ($C'(x)$)

Aplicando as propriedades da derivada da soma, da potência e da constante:

$$C'(x) = \frac{d}{dx}\left[ 10^{-6} x^3 - 0,003 x^2 + 5x + 1000 \right]$$

Passo a passo por termo:
1. $\frac{d}{dx}\left[ 10^{-6} x^3 \right] = 10^{-6} \cdot (3x^{3-1}) = 3 \cdot 10^{-6} x^2$
2. $\frac{d}{dx}\left[ -0,003 x^2 \right] = -0,003 \cdot (2x^{2-1}) = -0,006 x$
3. $\frac{d}{dx}\left[ 5x \right] = 5 \cdot (1 x^0) = 5$
4. $\frac{d}{dx}\left[ 1000 \right] = 0$ (derivada de constante)

Somando os termos:
$$C'(x) = 3 \cdot 10^{-6} x^2 - 0,006 x + 5$$

Ou, em representação decimal pura:
$$C'(x) = 0,000003 x^2 - 0,006 x + 5$$

#### Item II: Determinação dos Pontos Críticos

Como $C(x)$ é um polinômio, é diferenciável em toda a reta real ($\mathbb{R}$), inexistindo pontos onde a derivada deixe de existir. Logo, os pontos críticos ocorrem exclusivamente onde $C'(x) = 0$:

$$3 \cdot 10^{-6} x^2 - 0,006 x + 5 = 0$$

Para simplificar a álgebra, podemos multiplicar todos os coeficientes por $10^6$:
$$3x^2 - 6000x + 5.000.000 = 0$$

Dividindo a equação inteira por 3:
$$x^2 - 2000x + \frac{5.000.000}{3} = 0$$

Calculamos o discriminante ($\Delta = b^2 - 4ac$) com os coeficientes $a = 3 \cdot 10^{-6}$, $b = -0,006 = -6 \cdot 10^{-3}$ e $c = 5$:

$$\Delta = (-0,006)^2 - 4 \cdot (3 \cdot 10^{-6}) \cdot 5$$
$$\Delta = 0,000036 - 4 \cdot 15 \cdot 10^{-6}$$
$$\Delta = 3,6 \cdot 10^{-5} - 6,0 \cdot 10^{-5}$$
$$\Delta = -2,4 \cdot 10^{-5} < 0 \quad (\text{ou } \Delta = -24.000.000 \text{ na forma inteira})$$

**Conclusão sobre pontos críticos:**
Como o discriminante é estritamente negativo ($\Delta < 0$), a equação quadrática não admite raízes reais ($x \notin \mathbb{R}$). 

Além disso, como o coeficiente quadrático $a = 3 \cdot 10^{-6} > 0$, a parábola de $C'(x)$ está totalmente situada acima do eixo das abscissas, ou seja:
$$C'(x) > 0, \quad \forall x \in \mathbb{R}$$

Portanto, a função custo $C(x)$ é estritamente crescente em todo o seu domínio e **não possui pontos críticos reais** (nem máximos nem mínimos locais).

#### Item III: Segunda Derivada e Análise de Concavidade / Inflexão

Calculamos a segunda derivada derivando $C'(x)$:

$$C''(x) = \frac{d}{dx}\left[ 3 \cdot 10^{-6} x^2 - 0,006 x + 5 \right]$$
$$C''(x) = 3 \cdot 10^{-6} \cdot (2x) - 0,006 \cdot (1) + 0$$
$$C''(x) = 6 \cdot 10^{-6} x - 0,006$$

Embora não existam pontos críticos de primeira ordem para serem classificados como extremos via teste da segunda derivada, podemos investigar os pontos de anulação de $C''(x)$ e analisar a concavidade:

$$C''(x) = 0 \implies 6 \cdot 10^{-6} x - 0,006 = 0$$
$$6 \cdot 10^{-6} x = 6 \cdot 10^{-3}$$
$$x = \frac{6 \cdot 10^{-3}}{6 \cdot 10^{-6}} = 10^3 = 1000$$

Estudo do sinal de $C''(x)$:
- Para $x < 1000$: $C''(x) < 0 \implies$ o gráfico de $C(x)$ é côncavo para baixo ($\cap$).
- Para $x > 1000$: $C''(x) > 0 \implies$ o gráfico de $C(x)$ é côncavo para cima ($\cup$).

Houve inversão de sinal na concavidade através de $x = 1000$, logo $x = 1000$ é um **ponto de inflexão**.

Calculando o custo total nesse ponto:
$$C(1000) = 10^{-6}(1000)^3 - 0,003(1000)^2 + 5(1000) + 1000$$
$$C(1000) = 10^{-6}(10^9) - 0,003(10^6) + 5000 + 1000$$
$$C(1000) = 1000 - 3000 + 5000 + 1000 = 4000$$

O ponto de inflexão ocorre em $(1000, 4000)$. No ponto $x = 1000$, o custo marginal atinge seu valor mínimo:
$$C'(1000) = 3 \cdot 10^{-6}(1000)^2 - 0,006(1000) + 5 = 3 - 6 + 5 = 2 \text{ R\$/unidade}$$

---

### 3.2 FABRICANTE B

A função de custo total do Fabricante B é dada por:
$$C(x) = x^3 - 6x^2 + 13x + 15$$

#### Item I: Cálculo da Função Custo Marginal ($C'(x)$)

Derivando termo a termo:
$$C'(x) = \frac{d}{dx}\left[ x^3 - 6x^2 + 13x + 15 \right]$$
$$C'(x) = 3x^2 - 6 \cdot (2x) + 13 \cdot (1) + 0$$
$$C'(x) = 3x^2 - 12x + 13$$

A função custo marginal do Fabricante B é $C'(x) = 3x^2 - 12x + 13$.

#### Item II: Determinação dos Pontos Críticos

Igualando a primeira derivada a zero:
$$3x^2 - 12x + 13 = 0$$

Identificando os coeficientes: $a = 3$, $b = -12$, $c = 13$.
Calculando o discriminante ($\Delta$):
$$\Delta = b^2 - 4ac = (-12)^2 - 4 \cdot 3 \cdot 13$$
$$\Delta = 144 - 156$$
$$\Delta = -12 < 0$$

**Conclusão sobre pontos críticos:**
Como $\Delta = -12 < 0$, a equação não possui raízes reais. Como o coeficiente líder $a = 3 > 0$, a expressão $3x^2 - 12x + 13$ é estritamente positiva para todo $x \in \mathbb{R}$:
$$C'(x) > 0, \quad \forall x \in \mathbb{R}$$

Dessa forma, a função $C(x)$ é estritamente crescente e **não possui pontos críticos reais** (não há máximos nem mínimos locais).

#### Item III: Segunda Derivada e Análise de Concavidade / Inflexão

Calculando a segunda derivada:
$$C''(x) = \frac{d}{dx}\left[ 3x^2 - 12x + 13 \right] = 6x - 12$$

Verificando onde a segunda derivada se anula:
$$C''(x) = 0 \implies 6x - 12 = 0 \implies 6x = 12 \implies x = 2$$

Estudo do sinal de $C''(x)$:
- Para $x < 2$: $C''(x) = 6x - 12 < 0 \implies$ o gráfico é côncavo para baixo ($\cap$).
- Para $x > 2$: $C''(x) = 6x - 12 > 0 \implies$ o gráfico é côncavo para cima ($\cup$).

Como o sinal de $C''(x)$ varia ao passar por $x = 2$, trata-se de um **ponto de inflexão**.

Calculando a ordenada do ponto:
$$C(2) = (2)^3 - 6(2)^2 + 13(2) + 15$$
$$C(2) = 8 - 6(4) + 26 + 15 = 8 - 24 + 26 + 15 = 25$$

O ponto de inflexão do Fabricante B é $(2, 25)$. Em $x = 2$, o custo marginal atinge seu valor mínimo:
$$C'(2) = 3(2)^2 - 12(2) + 13 = 12 - 24 + 13 = 1 \text{ R\$/unidade}$$

---

### 3.3 FABRICANTE C

A função de custo total do Fabricante C é dada por:
$$C(x) = 7 \cdot 10^6 + 30x$$

onde $7 \cdot 10^6 = 7.000.000$ representa o custo fixo e $30$ representa o custo variável unitário.

#### Item I: Cálculo da Função Custo Marginal ($C'(x)$)

Derivando a função:
$$C'(x) = \frac{d}{dx}\left[ 7 \cdot 10^6 + 30x \right]$$
$$C'(x) = 0 + 30 \cdot (1) = 30$$

A função custo marginal do Fabricante C é constante e igual a:
$$C'(x) = 30$$

Isso significa que cada unidade adicional produzida tem um custo invariável de R$ 30,00.

#### Item II: Determinação dos Pontos Críticos

Buscando onde $C'(x) = 0$ ou onde $C'(x)$ não existe:
$$C'(x) = 30 \neq 0, \quad \forall x \in \mathbb{R}$$

Como a primeira derivada é constante, não nula e definida em toda a reta real, a igualdade $C'(x) = 0$ jamais é satisfeita.
Portanto, a função **não possui pontos críticos**.

#### Item III: Segunda Derivada e Classificação

Calculando a segunda derivada:
$$C''(x) = \frac{d}{dx}[30] = 0, \quad \forall x \in \mathbb{R}$$

Como $C''(x) = 0$ para todos os pontos do domínio, a função apresenta concavidade nula (trata-se de uma reta no plano cartesiano). Não há pontos críticos e não há mudança de concavidade. 
Portanto, a função **não possui pontos de máximo, mínimo ou de inflexão**.

---

### 3.4 FABRICANTE D

A função de custo total do Fabricante D é dada por:
$$C(x) = 75 + 80x - x^2$$

#### Item I: Cálculo da Função Custo Marginal ($C'(x)$)

Derivando termo a termo:
$$C'(x) = \frac{d}{dx}\left[ 75 + 80x - x^2 \right]$$
$$C'(x) = 0 + 80 \cdot (1) - 2x^{2-1}$$
$$C'(x) = 80 - 2x$$

A função custo marginal do Fabricante D é $C'(x) = 80 - 2x$.

#### Item II: Determinação dos Pontos Críticos

Igualando a primeira derivada a zero:
$$C'(x) = 0 \implies 80 - 2x = 0$$
$$2x = 80 \implies x = \frac{80}{2} = 40$$

Sendo uma função polinomial contínua e derivável em $\mathbb{R}$, o único ponto crítico encontrado é:
$$x = 40$$

Calculando o custo total correspondente a este ponto:
$$C(40) = 75 + 80(40) - (40)^2$$
$$C(40) = 75 + 3200 - 1600 = 1675$$

As coordenadas do ponto crítico são $(40, 1675)$.

#### Item III: Segunda Derivada e Classificação

Calculando a segunda derivada derivando $C'(x)$:
$$C''(x) = \frac{d}{dx}[80 - 2x] = 0 - 2 = -2$$

Avaliando a segunda derivada no ponto crítico $x = 40$:
$$C''(40) = -2 < 0$$

Pelo **Teste da Segunda Derivada**:
- Como $C'(40) = 0$ e $C''(40) < 0$, a concavidade da função é voltada para baixo ($\cap$) em toda a sua extensão.
- Consequentemente, o ponto crítico $x = 40$ é classificado categoricamente como um **ponto de MÁXIMO (relativo e absoluto)**.

O valor máximo assumido pela função custo é $C(40) = \text{R\$} \, 1.675,00$.

Quanto a pontos de inflexão: como $C''(x) = -2$ é uma constante não nula, $C''(x)$ nunca se anula nem troca de sinal, inexistindo pontos de inflexão.

*(Nota Econômica: Em modelagens teóricas de custo, funções quadráticas com coeficiente de $x^2$ negativo possuem validade operacional restrita ao trecho em que o custo marginal é não negativo, ou seja, para $0 \le x \le 40$. Acima de $x = 40$, o modelo geraria custos marginais negativos, o que sinaliza a fronteira física/econômica do modelo analisado).*

---

## 4 TABELA SÍNTESE COMPARATIVA

| Fabricante | Função Custo Total $C(x)$ | Custo Marginal $C'(x)$ | Pontos Críticos ($C'=0$) | Segunda Derivada $C''(x)$ | Classificação / Inflexão |
|:---|:---|:---|:---|:---|:---|
| **A** | $10^{-6}x^3 - 0,003x^2 + 5x + 1000$ | $3 \cdot 10^{-6}x^2 - 0,006x + 5$ | Nenhum ($\Delta = -2,4 \cdot 10^{-5} < 0$) | $6 \cdot 10^{-6}x - 0,006$ | Sem extremos locais; Ponto de inflexão em $x = 1000$ com $C(1000) = 4000$ |
| **B** | $x^3 - 6x^2 + 13x + 15$ | $3x^2 - 12x + 13$ | Nenhum ($\Delta = -12 < 0$) | $6x - 12$ | Sem extremos locais; Ponto de inflexão em $x = 2$ com $C(2) = 25$ |
| **C** | $7 \cdot 10^6 + 30x$ | $30$ | Nenhum ($C' = 30 \neq 0$) | $0$ | Sem extremos locais e sem pontos de inflexão (reta linear) |
| **D** | $75 + 80x - x^2$ | $80 - 2x$ | $x = 40$ | $-2$ | Ponto de **MÁXIMO** em $x = 40$ com $C(40) = 1675$ |

---

## 5 CONSIDERAÇÕES FINAIS

A resolução analítica das funções de custo dos quatro fabricantes evidencia a versatilidade e a precisão do Cálculo Diferencial na caracterização do comportamento produtivo e de custos:

1. **Taxas marginais estritamente positivas e crescimento monotônico (Fabricantes A e B):** Para os fabricantes A e B, embora as curvas cúbicas apresentem curvatura dinâmica, o cálculo do discriminante demonstrou que as derivadas primeiras não possuem raízes reais ($\Delta < 0$). Com isso, o custo marginal permanece estritamente positivo ($C'(x) > 0$), garantindo que os custos totais sejam funções estritamente crescentes sem picos ou vales locais.
2. **Importância dos pontos de inflexão:** Nos fabricantes A e B, a análise da segunda derivada revelou pontos de inflexão em $x = 1000$ e $x = 2$, respectivamente. Nesses pontos, a concavidade inverte de côncava para baixo para côncava para cima, sinalizando o exato volume produtivo onde a eficiência marginal é máxima (menor custo marginal por unidade produzida).
3. **Custo marginal constante (Fabricante C):** O modelo linear do fabricante C reflete uma estrutura com rendimentos de escala neutros e custo variável unitário perfeitamente linearizado (R$ 30,00 por peça), com segunda derivada identicamente nula.
4. **Otimização e ponto de máximo (Fabricante D):** O fabricante D ilustra uma estrutura quadrática côncava, cujo ponto crítico em $x = 40$ foi classificado rigorosamente pelo teste da segunda derivada ($C''(40) = -2 < 0$) como um ponto de máximo absoluto de custo.

O estudo cumpriu integralmente todos os requisitos da Atividade 1 da disciplina Cálculo de Uma Variável, detalhando cada passagem algébrica e conceitual exigida pela rubrica de avaliação.

---

## REFERÊNCIAS BIBLIOGRÁFICAS

- LEITHOLD, Louis. **O cálculo com geometria analítica**. 3. ed. São Paulo: Harbra, 1994. v. 1.
- STEWART, James. **Cálculo**. 7. ed. São Paulo: Cengage Learning, 2013. v. 1.
- THOMAS, George B. et al. **Cálculo**. 12. ed. São Paulo: Pearson Education do Brasil, 2012. v. 1.
- UNIVERSIDADE VEIGA DE ALMEIDA. **Cálculo de uma variável: Unidade 1 – Estudo dos limites e conceito de derivada**. Rio de Janeiro: UVA, 2026.
- UNIVERSIDADE VEIGA DE ALMEIDA. **Cálculo de uma variável: Unidade 2 – Técnicas de derivação e aplicações**. Rio de Janeiro: UVA, 2026.
