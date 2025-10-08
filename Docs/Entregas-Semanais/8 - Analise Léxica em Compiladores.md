# AFDs Finais

1. AFD para Identificadores e Palavras-Chave

```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_id
    q0_id --> q1_id : Letra ou "_"
    q1_id --> q1_id : Letra, Dígito ou "_"
    q1_id --> [*] : Qualquer outro caractere
```

2. AFD para Números (Inteiros e Decimais)
```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_num
    q0_num --> q1_int : Dígito (0-9)
    q1_int --> q1_int : Dígito (0-9)
    q1_int --> q2_dec : "."
    q1_int --> [*] : Qualquer outro (Final Inteiro)
    q2_dec --> q3_dec : Dígito (0-9)
    q3_dec --> q3_dec : Dígito (0-9)
    q3_dec --> [*] : Qualquer outro (Final Decimal)
```

3. AFD para Strings
```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_str
    q0_str --> q1_str : "\""
    q1_str --> q1_str : Qualquer caractere exceto "\"" e "\\"
    q1_str --> q2_esc : "\\"
    q2_esc --> q1_str : "n", "t", "r", "\"", "\\"
    q1_str --> q3_str : "\""
    q3_str --> [*]
```

4. AFD para Comentários de Linha
```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_comm
    q0_comm --> q1_comm : "/"
    q1_comm --> q2_comm : "/"
    q2_comm --> q2_comm : Qualquer caractere exceto "\\n"
    q2_comm --> q3_comm : "\\n"
    q3_comm --> [*]
```

5. AFD para Comentários de Bloco
```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_bcomm
    q0_bcomm --> q1_bcomm : "/"
    q1_bcomm --> q2_bcomm : "*"
    q2_bcomm --> q2_bcomm : Qualquer caractere exceto "*"
    q2_bcomm --> q3_bcomm : "*"
    q3_bcomm --> q2_bcomm : Qualquer caractere exceto "/"
    q3_bcomm --> q4_bcomm : "/"
    q4_bcomm --> [*]
```

6. AFD para Operadores Compostos
```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_op
    q0_op --> q1_eq : "="
    q1_eq --> q2_eq : "="
    q2_eq --> [*] : "=="
    q1_eq --> [*] : "="
    
    q0_op --> q3_neq : "!"
    q3_neq --> q4_neq : "="
    q4_neq --> [*] : "!="
    q3_neq --> [*] : "!"
    
    q0_op --> q5_lt : "<"
    q5_lt --> q6_lte : "="
    q6_lte --> [*] : "<="
    q5_lt --> [*] : "<"
    
    q0_op --> q7_gt : ">"
    q7_gt --> q8_gte : "="
    q8_gte --> [*] : ">="
    q7_gt --> [*] : ">"
    
    q0_op --> q9_and : "&"
    q9_and --> q10_and : "&"
    q10_and --> [*] : "&&"
    
    q0_op --> q11_or : "|"
    q11_or --> q12_or : "|"
    q12_or --> [*] : "||"
```
7. AFD Completo - Visão Geral dos Tokens
```mermaid
stateDiagram-v2
    direction LR
    [*] --> Inicio
    
    state Inicio {
        [*] --> Classificar
        Classificar --> Identificador : Letra ou "_"
        Classificar --> Numero : Dígito (0-9)
        Classificar --> String : "\""
        Classificar --> Comentario : "/"
        Classificar --> Operador : "+-*/%=!<>&|"
        Classificar --> Delimitador : "(){}[],;:."
        
        Identificador --> [*]
        Numero --> [*]
        String --> [*]
        Comentario --> [*]
        Operador --> [*]
        Delimitador --> [*]
    }
```
8. AFD para Sequência de Escape em Strings
```mermaid
stateDiagram-v2
    direction LR
    [*] --> q0_escape
    q0_escape --> q1_escape : "\\"
    q1_escape --> q2_valid : "n", "t", "r", "\"", "\\"
    q2_valid --> [*]
    q1_escape --> [*] : Caractere inválido (ERRO)
```

9. Diagrama de Transição entre Estados do Lexer
```mermaid
stateDiagram-v2
    [*] --> InicioToken
    InicioToken --> ProcessandoToken : Caractere válido
    ProcessandoToken --> ProcessandoToken : Continuação válida
    ProcessandoToken --> TokenReconhecido : Fim do token
    ProcessandoToken --> ErroLexico : Caractere inválido
    TokenReconhecido --> InicioToken : Próximo token
    ErroLexico --> [*] : Para execução
    InicioToken --> [*] : Fim do arquivo
```