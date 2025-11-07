#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {

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

    Identificador(String),
    InteiroLiteral(i64),
    DecimalLiteral(f64),
    StringLiteral(String),

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
    pub lexema: String,
    pub linha: usize,
    pub coluna: usize,
    pub comprimento: usize,
}

// Adicione esta implementação no arquivo lexer/token.rs
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f, 
            "Token({:?}, linha: {}, coluna: {})", 
            self.token_type, self.linha, self.coluna
        )
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexema: String, linha: usize, coluna: usize) -> Self {
        let comprimento = lexema.chars().count();
        Self {
            token_type,
            lexema,
            linha,
            coluna,
            comprimento,
        }
    }

    pub fn eof(linha: usize, coluna: usize) -> Self {
        Self::new(TokenType::EOF, "".to_string(), linha, coluna)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self.token_type, TokenType::EOF)
    }

    pub fn valor_numerico(&self) -> Option<f64> {
        match &self.token_type {
            TokenType::InteiroLiteral(n) => Some(*n as f64),
            TokenType::DecimalLiteral(n) => Some(*n),
            _ => None,
        }
    }
}

