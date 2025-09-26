# Gramáticas Formais e Hierarquia de Chomsky (Semana 3)

## **1. Classificação na Hierarquia de Chomsky**

A gramática da Symplia é classificada como **Tipo 2 - Livre de Contexto** na hierarquia de Chomsky.

**Justificativa:**
- As produções seguem o formato A → α, onde A é um único não-terminal
- Permite expressar a estrutura hierárquica e aninhamento típicos de linguagens de programação
- É suficiente para expressar constructs como escopo de blocos, expressões aritméticas com precedência, e estruturas de controle aninhadas

## **2. Gramática Formal da Symplia (BNF Estendido)**

### **2.1 Estrutura Básica do Programa**

```
<programa> ::= <declaracao-funcao>+

<declaracao-funcao> ::= "funcao" <tipo>? <identificador> "(" <parametros>? ")" <bloco> "fimfuncao"

<parametros> ::= <parametro> ("," <parametro>)*
<parametro> ::= <tipo> <identificador>

<bloco> ::= "{" <comando>* "}"
```

### **2.2 Comandos e Declarações**

```
<comando> ::= <declaracao-variavel>
            | <atribuicao>
            | <estrutura-controle>
            | <chamada-funcao>
            | <comando-escreva>
            | <comando-leia>
            | <retorno>

<declaracao-variavel> ::= <tipo> <identificador> ("=" <expressao>)? ";"

<atribuicao> ::= <identificador> "=" <expressao> ";"

<retorno> ::= "retorne" <expressao>? ";"
```

### **2.3 Estruturas de Controle**

```
<estrutura-controle> ::= <condicional> | <loop>

<condicional> ::= "se" <expressao> "entao" <bloco> ("senao" <bloco>)? "fimse"

<loop> ::= "enquanto" <expressao> "faca" <bloco> "fimenquanto"
          | "para" <identificador> "de" <expressao> "ate" <expressao> "faca" <bloco> "fimpara"
```

### **2.4 Expressões (com Precedência)**

```
<expressao> ::= <expressao-logica>

<expressao-logica> ::= <expressao-logica> ("&&" | "||") <expressao-relacional>
                     | <expressao-relacional>
                     | "!" <expressao-logica>

<expressao-relacional> ::= <expressao-aritmetica> ("==" | "!=" | "<" | ">" | "<=" | ">=") <expressao-aritmetica>
                         | <expressao-aritmetica>

<expressao-aritmetica> ::= <expressao-aritmetica> ("+" | "-") <termo>
                         | <termo>

<termo> ::= <termo> ("*" | "/" | "%") <fator>
          | <fator>

<fator> ::= <literal>
          | <identificador>
          | <chamada-funcao>
          | "(" <expressao> ")"
          | ("+" | "-") <fator>

<chamada-funcao> ::= <identificador> "(" <argumentos>? ")"
<argumentos> ::= <expressao> ("," <expressao>)*
```

### **2.5 Tipos e Literais**

```
<tipo> ::= "inteiro" | "decimal" | "texto" | "logico"

<literal> ::= <inteiro> | <decimal> | <string> | <booleano>

<inteiro> ::= [0-9]+
<decimal> ::= [0-9]+ "." [0-9]+
<string> ::= '"' ([^"] | '\\"')* '"'
<booleano> ::= "verdadeiro" | "falso"
```

## **3. Exemplos de Derivações**

### **3.1 Derivação de uma Expressão Aritmética**
```
a + b * 3

<expressao>
→ <expressao-aritmetica>
→ <expressao-aritmetica> + <termo>
→ <termo> + <termo>
→ <fator> + <termo>
→ <identificador> + <termo>
→ a + <termo>
→ a + <termo> * <fator>
→ a + <fator> * <fator>
→ a + <identificador> * <fator>
→ a + b * <fator>
→ a + b * <inteiro>
→ a + b * 3
```

### **3.2 Derivação de um Condicional**
```
se x > 0 entao { escreva("Positivo") } fimse

<comando>
→ <estrutura-controle>
→ <condicional>
→ "se" <expressao> "entao" <bloco> "fimse"
→ "se" <expressao-relacional> "entao" <bloco> "fimse"
→ "se" <expressao-aritmetica> ">" <expressao-aritmetica> "entao" <bloco> "fimse"
→ "se" <identificador> ">" <inteiro> "entao" <bloco> "fimse"
→ "se" x ">" 0 "entao" "{" <comando> "}" "fimse"
→ "se" x ">" 0 "entao" "{" <comando-escreva> "}" "fimse"
→ "se" x ">" 0 "entao" "{" "escreva" "(" <argumentos> ")" ";" "}" "fimse"
→ "se" x ">" 0 "entao" "{" "escreva" "(" <string> ")" ";" "}" "fimse"
→ "se" x > 0 entao { escreva("Positivo"); } fimse
```

## **4. Análise de Ambiguidades e Estratégias de Resolução**

### **4.1 Ambiguidades Identificadas**

#### **Problema 1: "if-else" pendente**
```portugol
se cond1 entao se cond2 entao cmd1 senao cmd2 fimse
```
**Ambiguidade:** O `senao` pertence ao primeiro ou segundo `se`?

**Solução:** Regra de associatividade - `senao` associa com o `se` mais próximo:
```
se cond1 entao 
    se cond2 entao 
        cmd1 
    senao  // pertence ao segundo se
        cmd2 
    fimse
fimse
```

#### **Problema 2: Precedência de operadores**
```
a + b * c
```
**Ambiguidade:** `(a + b) * c` ou `a + (b * c)`?

**Solução:** Hierarquia de não-terminais define precedência:
- Multiplicação (*, /, %) tem precedência sobre adição (+, -)

#### **Problema 3: Expressões de atribuição**
```
a = b = c
```
**Ambiguidade:** `(a = b) = c` ou `a = (b = c)`?

**Solução:** A linguagem não permite atribuições múltiplas - apenas `a = b`

### **4.2 Estratégias de Resolução**

1. **Gramática não-ambígua por construção:**
   - Hierarquia de expressões com níveis de precedência explícitos
   - Delimitadores obrigatórios (`entao`, `faca`, `fimse`)

2. **Regras de associatividade:**
   - Operadores aritméticos: associatividade à esquerda
   - Operadores de comparação: não-associativos
   - Operadores lógicos: associatividade à esquerda

3. **Palavras-chave de fechamento:**
   - `fimse`, `fimenquanto`, `fimpara` eliminam ambiguidade de aninhamento

## **5. Paradigma da Linguagem**

### **5.1 Paradigma Principal: Imperativo**
- Programas como sequência de comandos que alteram estado
- Variáveis mutáveis
- Estruturas de controle tradicionais (if, while, for)

### **5.2 Influências Funcionais (Futuro)**
- Funções como cidadãos de primeira classe (planejado para versão 2.0)
- Imutabilidade opcional

### **5.3 Justificativa das Escolhas**
- **Simplicidade para iniciantes:** Paradigma imperativo é mais intuitivo
- **Progressão natural:** Base sólida antes de introduzir conceitos funcionais
- **Expressividade controlada:** Recursos limitados para evitar sobrecarga cognitiva

## **6. Precedência e Associatividade (Resumo)**

| Nível | Operadores | Associatividade |
|-------|------------|-----------------|
| 1 | () (chamada função) [] | Esquerda |
| 2 | ! + - (unários) | Direita |
| 3 | * / % | Esquerda |
| 4 | + - (binários) | Esquerda |
| 5 | < > <= >= == != | Não associativo |
| 6 | && | Esquerda |
| 7 | \|\| | Esquerda |
| 8 | = | Direita (não permitido) |

## **7. Exemplo Completo com Análise Sintática**

```portugol
// Programa exemplo
funcao inteiro fatorial(inteiro n) {
    se n <= 1 entao {
        retorne 1
    } senao {
        retorne n * fatorial(n - 1)
    }
    fimse
}

funcao principal() {
    inteiro numero = 5
    inteiro resultado = fatorial(numero)
    escreva("Fatorial de ", numero, " é ", resultado)
}
```

**Árvore sintática abstrata (partial) do fatorial:**
```
Programa
├── Função: fatorial
│   ├── Parâmetro: n (inteiro)
│   └── Bloco
│       └── Condicional
│           ├── Condição: <=(n, 1)
│           ├── Bloco then: Retorne 1
│           └── Bloco else: Retorne *(n, Chamada(fatorial, -(n, 1)))
└── Função: principal
    ├── Declaração: numero = 5
    ├── Declaração: resultado = Chamada(fatorial, numero)
    └── Escreva: "Fatorial de ", numero, " é ", resultado
```

## **8. Considerações para Implementação**

### **8.1 Gramática LL(1)**
- A gramática foi desenhada para ser (majoritariamente) LL(1)
- Facilita implementação com parser recursivo descendente
- Poucos casos requerem lookahead maior

### **8.2 Tokens de sincronização**
- Palavras-chave de fechamento (`fimse`, `fimfuncao`) ajudam na recuperação de erros
- Ponto e vírgula como terminadores explícitos

### **8.3 Expansões futuras**
- Gramática preparada para adição de arrays e estruturas
- Espaço para operador ternário condicional (versão futura)

---
