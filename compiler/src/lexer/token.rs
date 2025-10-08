#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Palavras-chave
    Se,
    Entao,
    Senao,
    FimSe,
    Enquanto,
    Faca,
    FimEnquanto,
    Para,
    De,
    Ate,
    FimPara,
    Funcao,
    Retorne,
    FimFuncao,
    Inteiro,
    Decimal,
    Texto,
    Logico,
    Verdadeiro,
    Falso,
    Escreva,
    Leia,
    Principal,

    // Identificadores e literais
    Identificador(String),
    InteiroLiteral(i64),
    DecimalLiteral(f64),
    StringLiteral(String),

    // Operadores
    Mais,
    Menos,
    Multiplicacao,
    Divisao,
    Modulo,
    Atribuicao,
    Igual,
    Diferente,
    Menor,
    Maior,
    MenorIgual,
    MaiorIgual,
    ELogico,
    OuLogico,
    NaoLogico,

    // Delimitadoress
    ParenteseEsquerdo,
    ParenteseDireito,
    ChaveEsquerda,
    ChaveDireita,
    ColcheteEsquerdo,
    ColcheteDireito,
    Ponto,
    Virgula,
    PontoEVirgula,
    DoisPontos,

    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub linha: usize,
    pub coluna: usize,
    pub lexema: String,
}

impl Token {
    pub fn new(token_type: TokenType, linha: usize, coluna: usize, lexema: String) -> Self {
        Self {
            token_type,
            linha,
            coluna,
            lexema,
        }
    }

    pub fn eof(linha: usize, coluna: usize) -> Self {
        Self::new(TokenType::EOF, linha, coluna, "".to_string())
    }
}