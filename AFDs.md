# AFDs SimplesCOde

Este documento contém os diagramas de Autômatos Finitos Determinísticos (AFDs) para os tokens da linguagem SimplesCode, baseados nas expressões regulares definidas na Semana 4. Cada AFD é representado usando a sintaxe Mermaid para visualização.
Diagramas dos AFDs
A minimização de AFDs será aplicada para otimizar o desempenho do analisador léxico.

## AFD para Identificadores (ID)

Regex: [a-zA-Z][a-zA-Z0-9_]*
```mermaid
flowchart TD
    q0((q0)) -- a-zA-Z --> q1((q1))
    q1 -- a-zA-Z0-9_ --> q1
    q1 -- outro --> Fim
    style q1 fill:#00ff00
```
## AFD para Números Inteiros (INT)

Regex: [0-9]+
```mermaid
flowchart TD
    q0((q0)) -- 0-9 --> q1((q1))
    q1 -- 0-9 --> q1
    q1 -- outro --> Fim
    style q1 fill:#00ff00
```
## AFD para Números Decimais (DECIMAL)

Regex: [0-9]+\.[0-9]+
```mermaid
flowchart TD
    q0((q0)) -- 0-9 --> q1((q1))
    q1 -- 0-9 --> q1
    q1 -- . --> q2((q2))
    q2 -- 0-9 --> q3((q3))
    q3 -- 0-9 --> q3
    q3 -- outro --> Fim
    style q3 fill:#00ff00
```
## AFD para Strings (STR)

Regex: "([^"\\]|\\.)*"
```mermaid
flowchart TD
    q0((q0)) --> q1
    q1 --> q1
    q1 --> q2
    q2 --> q1
    q1 --> q3((q3))
    style q3 fill:#00ff00
```
## AFD para Operadores

Regex: \+|\-|\*|\/|=
```mermaid
flowchart TD
    q0((q0)) -- +, -, *, /, = --> q1((q1))
    style q1 fill:#00ff00
```
## AFD para Comentários

Regex: \#.*
```mermaid
flowchart TD
    q0((q0)) -- # --> q1((q1))
    q1 -- [^\n] --> q1
    q1 -- \n --> Fim
```
## AFD para Espaços em Branco

Regex: [ \t\n\r]+
```mermaid
flowchart TD
    q0((q0)) -- espaço, tab, \n, \r --> q1((q1))
    q1 -- espaço, tab, \n, \r --> q1
    q1 -- outro --> Fim
```

