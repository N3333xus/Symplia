# SimplesCode — Semana 1: Proposta Inicial

## Objetivos
- Definir a visão geral da linguagem.
- Estabelecer público-alvo e objetivos pedagógicos.
- Criar o Diário de Desenvolvimento.

## Visão Geral
O **SimplesCode** é uma linguagem de programação projetada para ensinar lógica a iniciantes, especialmente crianças e adolescentes. Sua sintaxe é inspirada em linguagem natural (português simplificado), tornando o aprendizado mais acessível.

## Público-Alvo
- Estudantes do ensino fundamental e médio.
- Pessoas que nunca programaram antes.
- Professores que desejam introduzir conceitos de lógica e algoritmos.

## Objetivos da Linguagem
- Ser simples e intuitiva.
- Reduzir a barreira de entrada para iniciantes.
- Ajudar no raciocínio lógico sem a complexidade de linguagens tradicionais.

## Diário de Desenvolvimento (Semana 1)
- Formação da equipe.
- Definição do nome da linguagem: **SimplesCode**.
- Estabelecimento do repositório no GitHub.
- Primeiras ideias para comandos básicos (`guarde`, `mostre`, `se`, `repita`).

---

# SimplesCode — Semana 2: Especificação Léxica (Versão Inicial)

## Objetivos
- Definir o alfabeto básico da linguagem.
- Especificar os tipos de tokens.
- Criar exemplos de programas válidos.

## Alfabeto da Linguagem
- Letras: `a-z`, `A-Z`
- Dígitos: `0-9`
- Símbolos: `+ - * / = ( ) " :`
- Espaço em branco e quebra de linha.

## Tokens Definidos
| Token | Descrição | Exemplo |
|-------|-----------|---------|
| **Identificadores** | Nomes de variáveis | `idade`, `nome1` |
| **Números** | Inteiros e decimais | `10`, `3.14` |
| **Palavras-chave** | Comandos reservados | `guarde`, `mostre`, `se`, `repita` |
| **Operadores** | Operações matemáticas | `+ - * /` |
| **Strings** | Texto entre aspas | `"Olá"` |

## Exemplos de Programas Válidos
```
guarde 10 em x
mostre x
```

```
repita 3 vezes:
    mostre "Oi"
fim
```

## Reflexões
- Identificadores são **case-insensitive**.
- Espaços e quebras de linha servem como separadores.
- Ambiguidade evitada não permitindo identificadores começarem com dígito.

---

# SimplesCode — Semana 3: Gramática Formal

## Classificação na Hierarquia de Chomsky
- **Tipo:** Gramática Livre de Contexto (GLC).
- **Nível:** 2 da Hierarquia de Chomsky.

## Gramática Formal
### Terminais
- Palavras-chave: `guarde`, `em`, `se`, `então`, `senão`, `fim`, `repita`, `vezes`, `mostre`, `pergunte`
- Operadores: `+ - * / =`
- Delimitadores: `(` `)` `:"`
- Literais: `NUM`, `STR`
- Identificadores: `ID`

### Não-Terminais
`Prog, ListaCmd, Cmd, Atrib, If, Loop, Print, Perg, Bloco, Expr, ExprAdd, ExprMul, Atom`

### Produções
```
Prog      → ListaCmd
ListaCmd  → Cmd ListaCmd | ε
Cmd       → Atrib | If | Loop | Print | Perg
Atrib     → "guarde" Expr "em" ID
If        → "se" Expr "então" ":" Bloco "fim"
          | "se" Expr "então" ":" Bloco "senão" ":" Bloco "fim"
Loop      → "repita" Expr "vezes" ":" Bloco "fim"
Print     → "mostre" Expr | "mostre" STR
Perg      → "pergunte" STR "e" "guarde" "em" ID
Bloco     → Cmd ListaCmd
Expr      → ExprAdd
ExprAdd   → ExprAdd "+" ExprMul | ExprAdd "-" ExprMul | ExprMul
ExprMul   → ExprMul "*" Atom | ExprMul "/" Atom | Atom
Atom      → NUM | ID | "(" Expr ")"
```

## Exemplos de Derivação
```
guarde 10 em x
```
```
Prog ⇒ ListaCmd ⇒ Cmd ⇒ Atrib ⇒ "guarde" Expr "em" ID ⇒ "guarde" NUM "em" ID
```

## Análise de Ambiguidades
- **Expressões Aritméticas:** resolvidas com fatoração (`ExprAdd`, `ExprMul`).
- **Dangling else:** resolvido com uso de `:` e `fim` delimitando blocos.

---

# SimplesCode — Semana 4: Especificação Léxica com Expressões Regulares

## Classificação na Hierarquia de Chomsky
- **Tipo:** Gramática Livre de Contexto (GLC) para sintaxe.
- **Tipo:** Linguagem Regular para análise léxica.

## Especificação Léxica com Regex
| Token | Regex | Observações |
|-------|-------|-------------|
| **ID** | `[a-zA-Z][a-zA-Z0-9_]*` | Identificadores. |
| **Palavras-chave** | `mostre|guarde|em|se|então|senão|fim|repita|vezes|pergunte` | Precedência sobre ID. |
| **Inteiros** | `[0-9]+` | |
| **Decimais** | `[0-9]+\.[0-9]+` | |
| **String (STR)** | `"([^"\\]|\\.)*"` | Suporta escapes. |
| **Operadores** | `\+|\-|\*|\/|=` | |
| **Comentários** | `\#.*` | Até o fim da linha. |
| **Espaços** | `[ \t\n\r]+` | Ignorados. |

## Ambiguidades e Regras
- Palavras-chave > Identificadores.
- Identificadores não iniciam com dígito.
- Strings mal fechadas geram erro.

## Tratamento de Erros
- Caractere inválido → erro com posição.
- Strings não fechadas → mensagem específica.
- Números mal formados → mensagem clara.

### Exemplos de Erros
- `@x = 10` → *Erro léxico: caractere inválido '@'.*
- `guarde "Olá` → *Erro léxico: string não fechada.*
- `12.34.56` → *Erro léxico: número decimal mal formado.*

## Exemplos de Programas
```
guarde 10 em x
guarde 20 em y
guarde x + y * 2 em z
mostre z
```

```
repita 3 vezes:
    mostre "Oi"
fim
```

---
*Arquivo atualizado para a entrega das Semanas 1 a 4 — Projeto **SimplesCode***

