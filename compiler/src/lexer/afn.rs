use std::collections::{HashMap, HashSet};
use super::token::TokenType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Estado(pub usize);

#[derive(Debug, Clone)]
pub struct TransicaoAFN {
    pub destino: Estado,
    pub simbolo: Option<char>, // None representa transição ε
}

#[derive(Debug, Clone)]
pub struct AFN {
    pub estados: HashSet<Estado>,
    pub estado_inicial: Estado,
    pub estados_finais: HashMap<Estado, TokenType>,
    pub transicoes: HashMap<Estado, Vec<TransicaoAFN>>,
    pub alfabeto: HashSet<char>,
}

impl AFN {
    pub fn new() -> Self {
        Self {
            estados: HashSet::new(),
            estado_inicial: Estado(0),
            estados_finais: HashMap::new(),
            transicoes: HashMap::new(),
            alfabeto: HashSet::new(),
        }
    }

    pub fn adicionar_estado(&mut self, estado: Estado) {
        self.estados.insert(estado);
    }

    pub fn adicionar_transicao(&mut self, origem: Estado, destino: Estado, simbolo: Option<char>) {
        if let Some(simbolo) = simbolo {
            self.alfabeto.insert(simbolo);
        }
        
        self.transicoes
            .entry(origem)
            .or_insert_with(Vec::new)
            .push(TransicaoAFN { destino, simbolo });
    }

    pub fn definir_estado_final(&mut self, estado: Estado, token_type: TokenType) {
        self.estados_finais.insert(estado, token_type);
    }

    // Fecho-ε: conjunto de estados alcançáveis via transições ε
    pub fn fecho_epsilon(&self, estados: &HashSet<Estado>) -> HashSet<Estado> {
        let mut fecho = estados.clone();
        let mut pilha: Vec<Estado> = estados.iter().cloned().collect();

        while let Some(estado) = pilha.pop() {
            if let Some(transicoes) = self.transicoes.get(&estado) {
                for transicao in transicoes {
                    if transicao.simbolo.is_none() && !fecho.contains(&transicao.destino) {
                        fecho.insert(transicao.destino.clone());
                        pilha.push(transicao.destino.clone());
                    }
                }
            }
        }

        fecho
    }

    // Movimento: estados alcançáveis a partir de um conjunto de estados com um símbolo
    pub fn movimento(&self, estados: &HashSet<Estado>, simbolo: char) -> HashSet<Estado> {
        let mut resultado = HashSet::new();

        for estado in estados {
            if let Some(transicoes) = self.transicoes.get(estado) {
                for transicao in transicoes {
                    if transicao.simbolo == Some(simbolo) {
                        resultado.insert(transicao.destino.clone());
                    }
                }
            }
        }

        resultado
    }
}

// Construção do AFN para a linguagem Symplia
impl AFN {
    pub fn construir_afn_symplia() -> Self {
        let mut afn = AFN::new();
        let mut contador_estado = 0;

        // Função auxiliar para criar novo estado
        let mut novo_estado = || {
            let estado = Estado(contador_estado);
            contador_estado += 1;
            afn.adicionar_estado(estado.clone());
            estado
        };

        let estado_inicial = novo_estado();
        afn.estado_inicial = estado_inicial.clone();

        // Construir AFNs para cada categoria de token e conectá-los com transições ε
        let afns_tokens = vec![
            Self::construir_afn_palavras_chave(&mut novo_estado),
            Self::construir_afn_identificadores(&mut novo_estado),
            Self::construir_afn_numeros(&mut novo_estado),
            Self::construir_afn_strings(&mut novo_estado),
            Self::construir_afn_operadores(&mut novo_estado),
            Self::construir_afn_delimitadores(&mut novo_estado),
        ];

        // Conectar estado inicial a todos os AFNs de tokens com transições ε
        for (_, estado_inicial_token) in afns_tokens {
            afn.adicionar_transicao(estado_inicial.clone(), estado_inicial_token, None);
        }

        afn
    }

    fn construir_afn_palavras_chave(novo_estado: &mut dyn FnMut() -> Estado) -> (HashMap<Estado, TokenType>, Estado) {
        let mut estados_finais = HashMap::new();
        let estado_inicial = novo_estado();

        let palavras_chave = vec![
            ("se", TokenType::Se),
            ("entao", TokenType::Entao),
            ("senao", TokenType::Senao),
            ("fimse", TokenType::FimSe),
            ("enquanto", TokenType::Enquanto),
            ("faca", TokenType::Faca),
            ("fimenquanto", TokenType::FimEnquanto),
            ("para", TokenType::Para),
            ("de", TokenType::De),
            ("ate", TokenType::Ate),
            ("fimpara", TokenType::FimPara),
            ("funcao", TokenType::Funcao),
            ("retorne", TokenType::Retorne),
            ("fimfuncao", TokenType::FimFuncao),
            ("inteiro", TokenType::Inteiro),
            ("decimal", TokenType::Decimal),
            ("texto", TokenType::Texto),
            ("logico", TokenType::Logico),
            ("verdadeiro", TokenType::Verdadeiro),
            ("falso", TokenType::Falso),
            ("escreva", TokenType::Escreva),
            ("leia", TokenType::Leia),
            ("principal", TokenType::Principal),
        ];

        for (palavra, token_type) in palavras_chave {
            let mut estado_atual = estado_inicial.clone();
            
            for (i, char) in palavra.chars().enumerate() {
                let proximo_estado = novo_estado();
                // Aqui seria necessário adicionar a transição ao AFN principal
                if i == palavra.len() - 1 {
                    estados_finais.insert(proximo_estado.clone(), token_type.clone());
                }
                estado_atual = proximo_estado;
            }
        }

        (estados_finais, estado_inicial)
    }

    fn construir_afn_identificadores(novo_estado: &mut dyn FnMut() -> Estado) -> (HashMap<Estado, TokenType>, Estado) {
        let mut estados_finais = HashMap::new();
        let estado_inicial = novo_estado();
        let estado_final = novo_estado();

        estados_finais.insert(estado_final.clone(), TokenType::Identificador("".to_string()));

        // Transições para primeiro caractere (letra ou _)
        // letras: 'a'..='z', 'A'..='Z', '_'
        
        // Transições para caracteres subsequentes (letra, dígito ou _)
        // letras: 'a'..='z', 'A'..='Z'
        // digitos: '0'..='9'
        // '_'

        (estados_finais, estado_inicial)
    }

    fn construir_afn_numeros(novo_estado: &mut dyn FnMut() -> Estado) -> (HashMap<Estado, TokenType>, Estado) {
        let mut estados_finais = HashMap::new();
        let estado_inicial = novo_estado();
        let estado_inteiro = novo_estado();
        let estado_decimal = novo_estado();

        estados_finais.insert(estado_inteiro.clone(), TokenType::InteiroLiteral(0));
        estados_finais.insert(estado_decimal.clone(), TokenType::DecimalLiteral(0.0));

        // Transições para dígitos
        // '0'..='9' -> estado_inteiro (loop)
        // '.' -> estado_decimal
        // estado_decimal: '0'..='9' (loop)

        (estados_finais, estado_inicial)
    }

    // Implementações similares para strings, operadores, delimitadores...
    fn construir_afn_strings(novo_estado: &mut dyn FnMut() -> Estado) -> (HashMap<Estado, TokenType>, Estado) {
        let mut estados_finais = HashMap::new();
        let estado_inicial = novo_estado();
        let estado_final = novo_estado();

        estados_finais.insert(estado_final.clone(), TokenType::StringLiteral("".to_string()));

        (estados_finais, estado_inicial)
    }

    fn construir_afn_operadores(novo_estado: &mut dyn FnMut() -> Estado) -> (HashMap<Estado, TokenType>, Estado) {
        let mut estados_finais = HashMap::new();
        let estado_inicial = novo_estado();

        let operadores = vec![
            ('+', TokenType::Mais),
            ('-', TokenType::Menos),
            ('*', TokenType::Multiplicacao),
            ('/', TokenType::Divisao),
            ('%', TokenType::Modulo),
            ('=', TokenType::Atribuicao),
            ('!', TokenType::NaoLogico),
            ('<', TokenType::Menor),
            ('>', TokenType::Maior),
        ];

        for (char, token_type) in operadores {
            let estado_final = novo_estado();
            estados_finais.insert(estado_final.clone(), token_type);
            // Adicionar transição: estado_inicial --char--> estado_final
        }

        // Operadores compostos
        let operadores_compostos = vec![
            ("==", TokenType::Igual),
            ("!=", TokenType::Diferente),
            ("<=", TokenType::MenorIgual),
            (">=", TokenType::MaiorIgual),
            ("&&", TokenType::ELogico),
            ("||", TokenType::OuLogico),
        ];

        for (str, token_type) in operadores_compostos {
            let mut estado_atual = estado_inicial.clone();
            for char in str.chars() {
                let proximo_estado = novo_estado();
                // Adicionar transição
                estado_atual = proximo_estado;
            }
            estados_finais.insert(estado_atual, token_type);
        }

        (estados_finais, estado_inicial)
    }

    fn construir_afn_delimitadores(novo_estado: &mut dyn FnMut() -> Estado) -> (HashMap<Estado, TokenType>, Estado) {
        let mut estados_finais = HashMap::new();
        let estado_inicial = novo_estado();

        let delimitadores = vec![
            ('(', TokenType::ParenteseEsquerdo),
            (')', TokenType::ParenteseDireito),
            ('{', TokenType::ChaveEsquerda),
            ('}', TokenType::ChaveDireita),
            ('[', TokenType::ColcheteEsquerdo),
            (']', TokenType::ColcheteDireito),
            (',', TokenType::Virgula),
            (';', TokenType::PontoEVirgula),
            (':', TokenType::DoisPontos),
            ('.', TokenType::Ponto),
        ];

        for (char, token_type) in delimitadores {
            let estado_final = novo_estado();
            estados_finais.insert(estado_final.clone(), token_type);
            // Adicionar transição: estado_inicial --char--> estado_final
        }

        (estados_finais, estado_inicial)
    }
}