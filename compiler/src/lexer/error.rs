#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub linha: usize,
    pub coluna: usize,
}

impl LexerError {
    pub fn new(message: String, linha: usize, coluna: usize) -> Self {
        Self { message, linha, coluna }
    }

    pub fn invalid_char(c: char, linha: usize, coluna: usize) -> Self {
        Self::new(format!("Caractere inválido: '{}'", c), linha, coluna)
    }

    pub fn unterminated_string(linha: usize, coluna: usize) -> Self {
        Self::new("String não foi fechada".to_string(), linha, coluna)
    }

    pub fn invalid_number(lexema: &str, linha: usize, coluna: usize) -> Self {
        Self::new(format!("Número inválido: '{}'", lexema), linha, coluna)
    }

    pub fn invalid_escape(c: char, linha: usize, coluna: usize) -> Self {
        Self::new(format!("Sequência de escape inválida: '\\{}'", c), linha, coluna)
    }
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ERRO LÉXICO: Linha {}, Coluna {} - {}", self.linha, self.coluna, self.message)
    }
}

impl std::error::Error for LexerError {}