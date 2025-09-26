# Especificação de Alfabetos e Tokens (Semana 2)

## **1. Alfabeto da Linguagem Symplia**

O alfabeto Σ da Symplia é definido como a união dos seguintes conjuntos:

### **Conjuntos Básicos:**
- **Letras** = {A, B, C, ..., Z, a, b, c, ..., z}
- **Dígitos** = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
- **Símbolos Especiais** = {_, @, #, $, ?} (limitados para simplicidade)

### **Operadores:**
- **Aritméticos** = {+, -, *, /, %}
- **Comparação** = {=, <, >, !}
- **Lógicos** = {&, |}

### **Delimitadores:**
- **Agrupamento** = {(, ), {, }, [, ]}
- **Pontuação** = {., ,, ;, :, ", '}

### **Espaços em Branco:**
- **Separadores** = {espaço, tab, nova-linha}

### **Alfabeto Completo:**
Σ = Letras ∪ Dígitos ∪ Símbolos Especiais ∪ Operadores ∪ Delimitadores ∪ Separadores

## **2. Especificação Formal dos Tokens**

### **2.1 Palavras-Chave (Keywords)**
```
L_keywords = {se, entao, senao, fimse, 
             enquanto, faca, fimenquanto,
             para, de, ate, fimpara,
             funcao, retorne, fimfuncao,
             inteiro, decimal, texto, logico,
             verdadeiro, falso, escreva, leia}
```

### **2.2 Identificadores**
Usando operações com linguagens formais:
- **Letra** = a | b | c | ... | z | A | B | ... | Z
- **Digito** = 0 | 1 | 2 | ... | 9
- **Identificador** = Letra (Letra | Digito | '_')*

**Definição Formal:** L_identificadores = Letra · (Letra ∪ Digito ∪ {'_'})*

### **2.3 Literais Numéricos**

#### **Inteiros:**
- L_inteiros = Digito+
- Exemplos: 0, 123, 9999

#### **Decimais:**
- L_decimais = Digito+ '.' Digito+
- Exemplos: 3.14, 0.5, 123.45

#### **Notação Científica (Opcional - para versão futura):**
- L_cientifica = Digito+ '.' Digito+ ('e' | 'E') ('+' | '-')? Digito+

### **2.4 Literais de Texto**
#### **Strings:**
- L_string = '"' (Caractere - {'"'})* '"'
- **Caractere escapado**: \\" | \\\\ | \\n | \\t
- Exemplo: "Olá mundo!", "Texto com \"aspas\""

#### **Caracteres (para versão futura):**
- L_char = "'" Caractere "'"

### **2.5 Operadores**
```
L_operadores = {+, -, *, /, %, =, ==, !=, <, >, <=, >=, &&, ||, !}
```

### **2.6 Delimitadores**
```
L_delimitadores = {(, ), {, }, [, ], ,, ;, :, ., ::}
```

### **2.7 Comentários**
#### **Comentário de Linha:**
- L_comentario_linha = '//' (Σ - {nova-linha})* nova-linha

#### **Comentário de Bloco (usando Fechamento de Kleene):**
- L_comentario_bloco = '/*' (Σ* - ('*' Σ* '/'))* '*/'

## **3. Decisões de Design Léxico**

### **Case Sensitivity:**
- **Identificadores**: Case-sensitive (`variavel ≠ Variavel ≠ VARIAVEL`)
- **Palavras-chave**: Apenas minúsculas (`se` é válido, `SE` é identificador)

### **Espaços em Branco:**
- Ignorados entre tokens, exceto como separadores
- **Indentação significativa**: Sim (como Python), para facilitar aprendizado

### **Regras de Desambiguação:**
1. **Máxima coincidência**: `seja` ≠ `sejaa` (identificadores distintos)
2. **Prioridade de tokens**: Palavras-chave > Operadores > Identificadores
3. **Numbers vs Identifiers**: Identificadores não podem começar com dígitos

### **Caracteres Válidos em Identificadores:**
- Permitido: letras, dígitos, underscore (`_`)
- **Não permitido**: caracteres especiais (@, #, $) em identificadores
- **Restrição**: Não pode começar com dígito (`123var` é inválido)

## **4. Exemplos de Programas Válidos**

### **Exemplo 1 - Hello World:**
```portugol
funcao principal() {
    escreva("Olá, mundo Symplia!")
}
```

### **Exemplo 2 - Operações Aritméticas:**
```portugol
funcao calcula_media() {
    decimal nota1 = 8.5
    decimal nota2 = 7.8
    decimal media = (nota1 + nota2) / 2
    escreva("Média: ", media)
}
```

### **Exemplo 3 - Estruturas de Controle:**
```portugol
funcao verifica_idade() {
    inteiro idade = leia("Digite sua idade: ")
    
    se idade >= 18 entao {
        escreva("Maior de idade")
    } senao {
        escreva("Menor de idade")
    }
}
```

### **Exemplo 4 - Demonstrando Tokens Complexos:**
```portugol
// Comentário de linha
funcao exemplos_tokens() {
    // Identificadores
    inteiro contador123 = 10
    texto mensagem_final = "Fim do programa"
    
    // Operadores compostos
    logico resultado = (x > 0) && (y <= 100)
    
    /* Comentário de bloco
       com múltiplas linhas */
}
```

## **5. Gramática Léxica (Resumo)**

| Categoria | Padrão | Exemplos |
|-----------|--------|----------|
| Palavra-chave | Lista fixa | `se`, `enquanto`, `funcao` |
| Identificador | `[a-zA-Z][a-zA-Z0-9_]*` | `idade`, `soma_total`, `var1` |
| Inteiro | `[0-9]+` | `0`, `42`, `1000` |
| Decimal | `[0-9]+\.[0-9]+` | `3.14`, `0.5`, `99.99` |
| String | `"(\.|[^"])*"` | `"texto"`, `"linha1\nlinha2"` |
| Operador | Símbolos específicos | `+`, `==`, `&&`, `!=` |
| Delimitador | Símbolos específicos | `(`, `)`, `{`, `}`, `;` |

## **6. Considerações para a Análise Léxica**

### **Problemas Potenciais Identificados:**
1. **`seja` vs `sejaa`**: Máxima coincidência resolve
2. **`10` vs `10.5`**: Prioridade para padrão mais específico
3. **Strings não fechadas**: Error handling específico
4. **Comentários aninhados**: Não permitidos na versão inicial

### **Decisões para Implementação:**
- **Tabela de símbolos** para palavras-chave e identificadores
- **Autômato finito determinístico** para reconhecimento de tokens
- **Mensagens de erro em português** para facilitar debugging

---

