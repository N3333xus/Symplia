#[derive(Debug, Clone, PartialEq)]
pub enum ErrorSeverity {
    Warning,    // continua a analise
    Error,      // parar a análise
    Fatal,      // erro irrecuperável
}

#[derive(Debug, Clone)]
pub struct LexerError {
    pub message: String,
    pub linha: usize,
    pub coluna: usize,
    pub severity: ErrorSeverity,
    pub recovery_suggestion: String,
}

impl LexerError {
    pub fn new(message: String, linha: usize, coluna: usize) -> Self {
        Self {
            message,
            linha,
            coluna,
            severity: ErrorSeverity::Error,
            recovery_suggestion: String::new(),
        }
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

    pub fn unclosed_comment(linha: usize, coluna: usize) -> Self {
        Self::new("Comentário de bloco não foi fechado".to_string(), linha, coluna)
    }

    pub fn with_recovery_suggestion(
        message: String, 
        linha: usize, 
        coluna: usize,
        suggestion: String
    ) -> Self {
        Self {
            message,
            linha,
            coluna,
            severity: ErrorSeverity::Error,
            recovery_suggestion: suggestion,
        }
    }

}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "ERRO LÉXICO: Linha {}, Coluna {} - {}", self.linha, self.coluna, self.message)
    }
}

impl std::error::Error for LexerError {}