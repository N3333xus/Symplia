# Reformulação da Especificação Léxica da Semana 2 Usando Expressões Regulares e Não Ambiguas (Semana 4)

## **1. Especificação Completa com Expressões Regulares**

### **1.1 Categorias Léxicas e Expressões Regulares**

| Categoria | Expressão Regular | Exemplos | Observações |
|-----------|-------------------|----------|-------------|
| **Palavras-chave** | `\b(se|entao|senao|fimse|enquanto|faca|fimenquanto|para|de|ate|fimpara|funcao|retorne|fimfuncao|inteiro|decimal|texto|logico|verdadeiro|falso|escreva|leia)\b` | `se`, `funcao` | `\b` para word boundaries |
| **Identificadores** | `[a-zA-Z_][a-zA-Z0-9_]*` | `idade`, `soma_total` | Não começa com dígito |
| **Inteiros** | `[0-9]+` | `42`, `0`, `1000` | |
| **Decimais** | `[0-9]+\.[0-9]+` | `3.14`, `0.5` | Ponto decimal obrigatório |
| **Strings** | `"([^"\\]|\\.)*"` | `"texto"`, `"linha\n"` | Suporte a escape sequences |
| **Booleanos** | `\b(verdadeiro|falso)\b` | `verdadeiro`, `falso` | |
| **Operadores** | `(\+\+|--|==|!=|<=|>=|&&|\|\||[+\-*/%=<>!])` | `+`, `==`, `&&` | Maximal munch aplicado |
| **Delimitadores** | `([(){}\[\],;:.])` | `(`, `)`, `;` | |
| **Comentários linha** | `//[^\n]*` | `// comentário` | Ignorado pelo lexer |
| **Comentários bloco** | `/\*[\s\S]*?\*/` | `/* comentário */` | Não-aninhados |
| **Espaços branco** | `[ \t\r\n]+` | espaços, tabs, newlines | Ignorados |

### **1.2 Expressões Regulares Detalhadas por Grupo**

#### **Números:**
```regex
# Ordem CRÍTICA: tentar decimais primeiro, depois inteiros
NUMERO_DECIMAL: [0-9]+\.[0-9]+
NUMERO_INTEIRO: [0-9]+
```

#### **Operadores com Maximal Munch:**
```regex
# Ordem por comprimento (mais longo primeiro)
OPERADOR_DUPLO: (==|!=|<=|>=|\+\+|--|&&|\|\|)
OPERADOR_SIMPLES: ([+\-*/%=<>!])
```

#### **Strings com Escape Sequences:**
```regex
STRING: "(
    [^"\\] |        # Qualquer caractere exceto " e \
    \\.             # Sequência de escape: \", \\, \n, \t
)*"
```

## **2. Análise de Ambiguidades e Regras de Resolução**

### **2.1 Ambiguidades Identificadas**

#### **Caso 1: Palavras-chave vs Identificadores**
- **Problema:** `se` (palavra-chave) vs `secao` (identificador)
- **Solução:** Word boundaries (`\b`) garantem match exato
- **Regra:** Palavras-chave têm precedência sobre identificadores

#### **Caso 2: Operadores Compostos vs Simples**
- **Problema:** `a++` vs `a + +b`
- **Solução:** Maximal munch - operador mais longo possível
- **Ordem de matching:** `++` antes de `+`

#### **Caso 3: Decimais vs Inteiros + Ponto**
- **Problema:** `3.14` (decimal) vs `3 . 14` (três tokens)
- **Solução:** Tentar decimal primeiro na ordem de regex
- **Regra:** Números decimais têm precedência sobre inteiros

#### **Caso 4: Comentários vs Operadores de Divisão**
- **Problema:** `//` (comentário) vs `/ /` (dois operadores)
- **Solução:** Comentário tem precedência máxima
- **Ordem:** Comentários antes de operadores

### **2.2 Ordem de Precedência no Analisador Léxico**

**Ordem CRÍTICA de aplicação das expressões regulares:**

1. **Espaços em branco** (ignorar primeiro)
2. **Comentários** (linha e bloco - ignorar)
3. **Palavras-chave** (precedência sobre identificadores)
4. **Booleanos** (precedência sobre identificadores)
5. **Strings** (delimitadas claramente)
6. **Números decimais** (antes de inteiros)
7. **Números inteiros**
8. **Operadores compostos** (maximal munch)
9. **Operadores simples**
10. **Delimitadores**
11. **Identificadores** (últimos - catch-all)

### **2.3 Tabela de Precedência e Resolução**

| Situação | Padrão 1 | Padrão 2 | Resolução | Exemplo |
|----------|----------|----------|-----------|---------|
| palavra-chave vs id | `\bse\b` | `[a-z]+` | Palavra-chave | `se` ≠ `secao` |
| decimal vs inteiro | `\d+\.\d+` | `\d+` | Decimal primeiro | `3.14` ≠ `3` + `.` + `14` |
| operador composto vs simples | `==` | `=` | Maximal munch | `==` ≠ `=` + `=` |
| comentário vs divisão | `//` | `/` | Comentário primeiro | `//` ≠ `/` + `/` |

## **3. Estratégia para Tratamento de Erros Léxicos**

### **3.1 Tipos de Erros Léxicos**

```regex
# Padrões para erros comuns
ERRO_STRING_NAO_FECHADA: "([^"\\]|\\.)*$  # String sem fechamento
ERRO_COMENTARIO_NAO_FECHADO: /\*[\s\S]*$  # Comentário de bloco aberto
ERRO_CARACTERE_INVALIDO: [^a-zA-Z0-9_ \t\r\n+\-*/%=<>!(){}[\],;:.""]  # Caracteres fora do alfabeto
```

### **3.2 Algoritmo de Recuperação de Erros**

```python
def tokenize(source_code):
    tokens = []
    line = 1
    column = 1
    i = 0
    
    while i < len(source_code):
        # Tentar match com padrões válidos (ordem de precedência)
        matched = False
        
        # 1. Espaços em branco
        if match_whitespace(source_code, i):
            # Atualizar contadores de linha/coluna
            i, line, column = update_position(source_code, i, line, column)
            continue
            
        # 2. Comentários
        if match := match_comment(source_code, i):
            i, line, column = update_position(source_code, i + len(match), line, column)
            continue
            
        # 3-11. Demais tokens...
        
        # 12. Tratamento de erros
        if not matched:
            # Caractere inválido encontrado
            error_char = source_code[i]
            error_msg = criar_mensagem_erro(line, column, error_char)
            reportar_erro(error_msg)
            
            # Estratégia: pular caractere inválido e continuar
            i += 1
            column += 1
    
    return tokens
```

### **3.3 Estratégias de Recuperação**

| Tipo de Erro | Estratégia | Exemplo |
|--------------|------------|---------|
| **Caractere inválido** | Pular caractere e continuar | `a @ b` → pular `@`, tokens: `a`, `b` |
| **String não fechada** | Fechar string no final da linha | `"texto sem fechamento` → erro + token string até \n |
| **Comentário não fechado** | Fechar no final do arquivo | `/* comentário` → erro + ignorar até EOF |
| **Número malformado** | Ler até caractere inválido | `123.` → erro + token inteiro `123` |

## **4. Mensagens de Erro para Usuários**

### **4.1 Esboços de Mensagens de Erro**

#### **Erro 001: Caractere Inválido**
```
Erro Léxico [001] - Linha 5, Coluna 12:
    Caractere '�' não é válido na linguagem Symplia.
    
Contexto:
    4:     escreva("Olá mundo!")
    5:     resultado = x @ y  // ERRO AQUI
                 ^
    
Sugestão: Verifique se o caractere foi digitado corretamente.
Caracteres válidos: letras, números, operadores (+-*/=) e delimitadores ({}();).
```

#### **Erro 002: String Não Fechada**
```
Erro Léxico [002] - Linha 3:
    String iniciada na linha 3 não foi fechada.
    
Contexto:
    2: funcao main() {
    3:     texto msg = "Esta string não foi fechada  // ERRO
                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    
Sugestão: Adicione aspas duplas (") para fechar a string.
```

#### **Erro 003: Comentário Não Fechado**
```
Erro Léxico [003] - Linha 7:
    Comentário de bloco iniciado na linha 7 não foi fechado.
    
Contexto:
    6:     /* Este é um comentário
    7:     que continua mas não termina  // ERRO
         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    
Sugestão: Adicione '*/' para fechar o comentário.
```

#### **Erro 004: Número Decimal Malformado**
```
Erro Léxico [004] - Linha 8, Coluna 15:
    Número decimal inválido.
    
Contexto:
    7:     decimal a = 3.14  // Correto
    8:     decimal b = 42.    // ERRO
                      ^^
    
Sugestão: Números decimais devem ter dígitos após o ponto (ex: 42.0).
```

### **4.2 Template de Mensagem de Erro**
```python
def criar_mensagem_erro(codigo, mensagem, linha, coluna, contexto, sugestao):
    return f"""
Erro Léxico [{codigo:03d}] - Linha {linha}, Coluna {coluna}:
    {mensagem}
    
Contexto:
    {contexto}
    
Sugestão: {sugestao}
"""
```

## **5. Implementação do Analisador Léxico**

### **5.1 Estrutura do Lexer**
```python
import re

class SympliaLexer:
    def __init__(self):
        # Compilar expressões regulares em ordem de precedência
        self.patterns = [
            ('ESPACO_BRANCO', r'[ \t\r\n]+', True),  # Ignorar
            ('COMENTARIO_LINHA', r'//[^\n]*', True),
            ('COMENTARIO_BLOCO', r'/\*[\s\S]*?\*/', True),
            ('PALAVRA_CHAVE', r'\b(se|entao|senao|fimse|enquanto|faca|fimenquanto|para|de|ate|fimpara|funcao|retorne|fimfuncao|inteiro|decimal|texto|logico|verdadeiro|falso|escreva|leia)\b', False),
            ('BOOLEANO', r'\b(verdadeiro|falso)\b', False),
            ('STRING', r'"([^"\\]|\\.)*"', False),
            ('NUMERO_DECIMAL', r'[0-9]+\.[0-9]+', False),
            ('NUMERO_INTEIRO', r'[0-9]+', False),
            ('OPERADOR_DUPLO', r'==|!=|<=|>=|\+\+|--|&&|\|\|', False),
            ('OPERADOR_SIMPLES', r'[+\-*/%=<>!]', False),
            ('DELIMITADOR', r'[(){}\[\],;:.]', False),
            ('IDENTIFICADOR', r'[a-zA-Z_][a-zA-Z0-9_]*', False),
        ]
        
        self.compiled_patterns = [
            (name, re.compile(pattern), ignore) 
            for name, pattern, ignore in self.patterns
        ]
    
    def tokenize(self, source_code):
        tokens = []
        pos = 0
        line = 1
        col = 1
        
        while pos < len(source_code):
            matched = False
            
            for token_name, pattern, ignore in self.compiled_patterns:
                match = pattern.match(source_code, pos)
                if match:
                    if not ignore:
                        tokens.append((token_name, match.group(), line, col))
                    
                    # Atualizar contadores de posição
                    matched_text = match.group()
                    line_breaks = matched_text.count('\n')
                    if line_breaks > 0:
                        line += line_breaks
                        col = len(matched_text) - matched_text.rfind('\n')
                    else:
                        col += len(matched_text)
                    
                    pos = match.end()
                    matched = True
                    break
            
            if not matched:
                # Erro léxico - caractere inválido
                self.report_error(line, col, source_code[pos])
                pos += 1
                col += 1
        
        return tokens
```

## **6. Considerações sobre Internacionalização**

### **6.1 Suporte a Caracteres Não-ASCII (Futuro)**
```regex
# Para versão futura com suporte a Unicode
IDENTIFICADOR_UNICODE: [\p{L}_][\p{L}0-9_]*
```

### **6.2 Decisão Atual:**
- **Não suportar** caracteres acentuados em identificadores na v1.0
- **Motivo:** Simplificar a gramática e evitar complexidades de Unicode
- **Alternativa:** Usar underscore (`idade_aluno`, `preço_medio`)

---

