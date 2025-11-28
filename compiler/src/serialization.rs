use serde::{Serialize, Deserialize};
use crate::parser::ast::*;
use crate::semantic::semantic::{SemanticAnalysisResult, AnnotatedExpr, AnnotatedStatement};
use std::fs;
use chrono::Utc;

// ==================== ESTRUTURAS DE METADADOS ====================

#[derive(Serialize, Deserialize)]
pub struct ProgramMetadata {
    pub source_file: String,
    pub timestamp: String,
    pub version: String,
    pub entry_point: Option<String>,
}

// ==================== ESTRUTURAS DE TIPOS ====================

#[derive(Serialize, Deserialize)]
pub enum SerializableType {
    Inteiro,
    Decimal,
    Texto,
    Logico,
    Void,
}

impl From<&Type> for SerializableType {
    fn from(ty: &Type) -> Self {
        match ty {
            Type::Inteiro => SerializableType::Inteiro,
            Type::Decimal => SerializableType::Decimal,
            Type::Texto => SerializableType::Texto,
            Type::Logico => SerializableType::Logico,
        }
    }
}

impl From<Type> for SerializableType {
    fn from(ty: Type) -> Self {
        SerializableType::from(&ty)
    }
}

// ==================== ESTRUTURAS DE EXPRESSÕES ====================

#[derive(Serialize, Deserialize)]
pub enum SerializableLiteral {
    Inteiro(i64),
    Decimal(f64),
    Texto(String),
    Logico(bool),
}

impl From<&Literal> for SerializableLiteral {
    fn from(literal: &Literal) -> Self {
        match literal {
            Literal::Inteiro(n) => SerializableLiteral::Inteiro(*n),
            Literal::Decimal(n) => SerializableLiteral::Decimal(*n),
            Literal::Texto(s) => SerializableLiteral::Texto(s.clone()),
            Literal::Logico(b) => SerializableLiteral::Logico(*b),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SerializableBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

impl From<&BinaryOperator> for SerializableBinaryOperator {
    fn from(op: &BinaryOperator) -> Self {
        match op {
            BinaryOperator::Add => SerializableBinaryOperator::Add,
            BinaryOperator::Subtract => SerializableBinaryOperator::Subtract,
            BinaryOperator::Multiply => SerializableBinaryOperator::Multiply,
            BinaryOperator::Divide => SerializableBinaryOperator::Divide,
            BinaryOperator::Modulo => SerializableBinaryOperator::Modulo,
            BinaryOperator::Equal => SerializableBinaryOperator::Equal,
            BinaryOperator::NotEqual => SerializableBinaryOperator::NotEqual,
            BinaryOperator::Less => SerializableBinaryOperator::Less,
            BinaryOperator::LessEqual => SerializableBinaryOperator::LessEqual,
            BinaryOperator::Greater => SerializableBinaryOperator::Greater,
            BinaryOperator::GreaterEqual => SerializableBinaryOperator::GreaterEqual,
            BinaryOperator::And => SerializableBinaryOperator::And,
            BinaryOperator::Or => SerializableBinaryOperator::Or,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum SerializableUnaryOperator {
    Negate,
    Plus,
    Not,
}

impl From<&UnaryOperator> for SerializableUnaryOperator {
    fn from(op: &UnaryOperator) -> Self {
        match op {
            UnaryOperator::Negate => SerializableUnaryOperator::Negate,
            UnaryOperator::Plus => SerializableUnaryOperator::Plus,
            UnaryOperator::Not => SerializableUnaryOperator::Not,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableCallExpr {
    pub function: String,
    pub arguments: Vec<SerializableExpr>,
}

impl From<&CallExpr> for SerializableCallExpr {
    fn from(call: &CallExpr) -> Self {
        SerializableCallExpr {
            function: call.function.clone(),
            arguments: call.arguments.iter().map(|expr| SerializableExpr::from_expr(expr)).collect(),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SerializableExpr {
    Literal {
        value: SerializableLiteral,
        expr_type: SerializableType,
    },
    Variable {
        name: String,
        expr_type: SerializableType,
    },
    Call {
        call: SerializableCallExpr,
        expr_type: SerializableType,
    },
    BinaryOp {
        op: SerializableBinaryOperator,
        left: Box<SerializableExpr>,
        right: Box<SerializableExpr>,
        expr_type: SerializableType,
    },
    UnaryOp {
        op: SerializableUnaryOperator,
        operand: Box<SerializableExpr>,
        expr_type: SerializableType,
    },
}

impl SerializableExpr {
    pub fn from_expr(expr: &Expr) -> Self {
        match expr {
            Expr::Literal(literal) => SerializableExpr::Literal {
                value: SerializableLiteral::from(literal),
                expr_type: match literal {
                    Literal::Inteiro(_) => SerializableType::Inteiro,
                    Literal::Decimal(_) => SerializableType::Decimal,
                    Literal::Texto(_) => SerializableType::Texto,
                    Literal::Logico(_) => SerializableType::Logico,
                },
            },
            Expr::Variable(name) => SerializableExpr::Variable {
                name: name.clone(),
                expr_type: SerializableType::Inteiro, // Tipo padrão
            },
            Expr::Call(call_expr) => SerializableExpr::Call {
                call: SerializableCallExpr::from(call_expr),
                expr_type: SerializableType::Inteiro, // Tipo padrão
            },
            Expr::BinaryOp(op, left, right) => SerializableExpr::BinaryOp {
                op: SerializableBinaryOperator::from(op),
                left: Box::new(Self::from_expr(left)),
                right: Box::new(Self::from_expr(right)),
                expr_type: SerializableType::Inteiro, // Tipo padrão
            },
            Expr::UnaryOp(op, operand) => SerializableExpr::UnaryOp {
                op: SerializableUnaryOperator::from(op),
                operand: Box::new(Self::from_expr(operand)),
                expr_type: SerializableType::Inteiro, // Tipo padrão
            },
        }
    }
}

impl From<&AnnotatedExpr> for SerializableExpr {
    fn from(annotated: &AnnotatedExpr) -> Self {
        let base_expr = Self::from_expr(&annotated.expr);
        
        match base_expr {
            SerializableExpr::Literal { value, .. } => SerializableExpr::Literal {
                value,
                expr_type: SerializableType::from(&annotated.type_),
            },
            SerializableExpr::Variable { name, .. } => SerializableExpr::Variable {
                name,
                expr_type: SerializableType::from(&annotated.type_),
            },
            SerializableExpr::Call { call, .. } => SerializableExpr::Call {
                call,
                expr_type: SerializableType::from(&annotated.type_),
            },
            SerializableExpr::BinaryOp { op, left, right, .. } => SerializableExpr::BinaryOp {
                op,
                left,
                right,
                expr_type: SerializableType::from(&annotated.type_),
            },
            SerializableExpr::UnaryOp { op, operand, .. } => SerializableExpr::UnaryOp {
                op,
                operand,
                expr_type: SerializableType::from(&annotated.type_),
            },
        }
    }
}

// ==================== ESTRUTURAS DE PARÂMETROS ====================

#[derive(Serialize, Deserialize)]
pub struct SerializableParameter {
    pub param_type: SerializableType,
    pub name: String,
}

impl From<&Parameter> for SerializableParameter {
    fn from(param: &Parameter) -> Self {
        SerializableParameter {
            param_type: SerializableType::from(&param.param_type),
            name: param.name.clone(),
        }
    }
}

// ==================== ESTRUTURAS DE BLOCO ====================

#[derive(Serialize, Deserialize)]
pub struct SerializableBlock {
    pub statements: Vec<SerializableStatement>,
}

impl From<&Block> for SerializableBlock {
    fn from(block: &Block) -> Self {
        SerializableBlock {
            statements: block.statements.iter().map(SerializableStatement::from_stmt).collect(),
        }
    }
}

// ==================== ESTRUTURAS DE STATEMENTS ====================

#[derive(Serialize, Deserialize)]
pub struct SerializableVariableDecl {
    pub var_type: SerializableType,
    pub name: String,
    pub initializer: Option<SerializableExpr>,
}

impl From<&VariableDecl> for SerializableVariableDecl {
    fn from(decl: &VariableDecl) -> Self {
        SerializableVariableDecl {
            var_type: SerializableType::from(&decl.var_type),
            name: decl.name.clone(),
            initializer: decl.initializer.as_ref().map(SerializableExpr::from_expr),
        }
    }
}

impl From<&AnnotatedStatement> for SerializableVariableDecl {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::VariableDecl(decl) = &annotated.statement {
            let initializer = if let Some(expr_annotation) = annotated.expr_annotations.get(0) {
                Some(SerializableExpr::from(expr_annotation))
            } else {
                decl.initializer.as_ref().map(SerializableExpr::from_expr)
            };
            
            SerializableVariableDecl {
                var_type: SerializableType::from(&decl.var_type),
                name: decl.name.clone(),
                initializer,
            }
        } else {
            panic!("Expected VariableDecl statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableExprStmt {
    pub expr: SerializableExpr,
}

#[derive(Serialize, Deserialize)]
pub struct SerializableIfStmt {
    pub condition: SerializableExpr,
    pub then_branch: SerializableBlock,
    pub else_branch: Option<SerializableBlock>,
}

impl From<&IfStmt> for SerializableIfStmt {
    fn from(stmt: &IfStmt) -> Self {
        SerializableIfStmt {
            condition: SerializableExpr::from_expr(&stmt.condition),
            then_branch: SerializableBlock::from(&stmt.then_branch),
            else_branch: stmt.else_branch.as_ref().map(SerializableBlock::from),
        }
    }
}

impl From<&AnnotatedStatement> for SerializableIfStmt {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::IfStmt(if_stmt) = &annotated.statement {
            let condition = if let Some(expr_annotation) = annotated.expr_annotations.get(0) {
                SerializableExpr::from(expr_annotation)
            } else {
                SerializableExpr::from_expr(&if_stmt.condition)
            };
            
            SerializableIfStmt {
                condition,
                then_branch: SerializableBlock::from(&if_stmt.then_branch),
                else_branch: if_stmt.else_branch.as_ref().map(SerializableBlock::from),
            }
        } else {
            panic!("Expected IfStmt statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableWhileStmt {
    pub condition: SerializableExpr,
    pub body: SerializableBlock,
}

impl From<&WhileStmt> for SerializableWhileStmt {
    fn from(stmt: &WhileStmt) -> Self {
        SerializableWhileStmt {
            condition: SerializableExpr::from_expr(&stmt.condition),
            body: SerializableBlock::from(&stmt.body),
        }
    }
}

impl From<&AnnotatedStatement> for SerializableWhileStmt {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::WhileStmt(while_stmt) = &annotated.statement {
            let condition = if let Some(expr_annotation) = annotated.expr_annotations.get(0) {
                SerializableExpr::from(expr_annotation)
            } else {
                SerializableExpr::from_expr(&while_stmt.condition)
            };
            
            SerializableWhileStmt {
                condition,
                body: SerializableBlock::from(&while_stmt.body),
            }
        } else {
            panic!("Expected WhileStmt statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableForStmt {
    pub variable: String,
    pub start: SerializableExpr,
    pub end: SerializableExpr,
    pub body: SerializableBlock,
}

impl From<&ForStmt> for SerializableForStmt {
    fn from(stmt: &ForStmt) -> Self {
        SerializableForStmt {
            variable: stmt.variable.clone(),
            start: SerializableExpr::from_expr(&stmt.start),
            end: SerializableExpr::from_expr(&stmt.end),
            body: SerializableBlock::from(&stmt.body),
        }
    }
}

impl From<&AnnotatedStatement> for SerializableForStmt {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::ForStmt(for_stmt) = &annotated.statement {
            let start = if let Some(expr_annotation) = annotated.expr_annotations.get(0) {
                SerializableExpr::from(expr_annotation)
            } else {
                SerializableExpr::from_expr(&for_stmt.start)
            };
            
            let end = if let Some(expr_annotation) = annotated.expr_annotations.get(1) {
                SerializableExpr::from(expr_annotation)
            } else {
                SerializableExpr::from_expr(&for_stmt.end)
            };
            
            SerializableForStmt {
                variable: for_stmt.variable.clone(),
                start,
                end,
                body: SerializableBlock::from(&for_stmt.body),
            }
        } else {
            panic!("Expected ForStmt statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableReturnStmt {
    pub value: Option<SerializableExpr>,
}

impl From<&ReturnStmt> for SerializableReturnStmt {
    fn from(stmt: &ReturnStmt) -> Self {
        SerializableReturnStmt {
            value: stmt.value.as_ref().map(SerializableExpr::from_expr),
        }
    }
}

// CORREÇÃO: Implementação simplificada para ReturnStmt anotado
impl From<&AnnotatedStatement> for SerializableReturnStmt {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::ReturnStmt(return_stmt) = &annotated.statement {
            let value = if let Some(expr_annotation) = annotated.expr_annotations.get(0) {
                Some(SerializableExpr::from(expr_annotation))
            } else {
                return_stmt.value.as_ref().map(SerializableExpr::from_expr)
            };
            
            SerializableReturnStmt {
                value,
            }
        } else {
            panic!("Expected ReturnStmt statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableWriteStmt {
    pub arguments: Vec<SerializableExpr>,
}

impl From<&WriteStmt> for SerializableWriteStmt {
    fn from(stmt: &WriteStmt) -> Self {
        SerializableWriteStmt {
            arguments: stmt.arguments.iter().map(SerializableExpr::from_expr).collect(),
        }
    }
}

impl From<&AnnotatedStatement> for SerializableWriteStmt {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::WriteStmt(write_stmt) = &annotated.statement {
            let arguments = if !annotated.expr_annotations.is_empty() {
                annotated.expr_annotations.iter().map(SerializableExpr::from).collect()
            } else {
                write_stmt.arguments.iter().map(SerializableExpr::from_expr).collect()
            };
            
            SerializableWriteStmt {
                arguments,
            }
        } else {
            panic!("Expected WriteStmt statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SerializableReadStmt {
    pub target: SerializableExpr,
}

impl From<&ReadStmt> for SerializableReadStmt {
    fn from(stmt: &ReadStmt) -> Self {
        SerializableReadStmt {
            target: SerializableExpr::from_expr(&stmt.target),
        }
    }
}

impl From<&AnnotatedStatement> for SerializableReadStmt {
    fn from(annotated: &AnnotatedStatement) -> Self {
        if let Statement::ReadStmt(read_stmt) = &annotated.statement {
            let target = if let Some(expr_annotation) = annotated.expr_annotations.get(0) {
                SerializableExpr::from(expr_annotation)
            } else {
                SerializableExpr::from_expr(&read_stmt.target)
            };
            
            SerializableReadStmt {
                target,
            }
        } else {
            panic!("Expected ReadStmt statement");
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SerializableStatement {
    VariableDecl(SerializableVariableDecl),
    ExprStmt(SerializableExprStmt),
    IfStmt(SerializableIfStmt),
    WhileStmt(SerializableWhileStmt),
    ForStmt(SerializableForStmt),
    ReturnStmt(SerializableReturnStmt),
    WriteStmt(SerializableWriteStmt),
    ReadStmt(SerializableReadStmt),
}

impl SerializableStatement {
    pub fn from_stmt(stmt: &Statement) -> Self {
        match stmt {
            Statement::VariableDecl(decl) => {
                SerializableStatement::VariableDecl(SerializableVariableDecl::from(decl))
            }
            Statement::ExprStmt(expr_stmt) => {
                SerializableStatement::ExprStmt(SerializableExprStmt {
                    expr: SerializableExpr::from_expr(&expr_stmt.expr),
                })
            }
            Statement::IfStmt(if_stmt) => {
                SerializableStatement::IfStmt(SerializableIfStmt::from(if_stmt))
            }
            Statement::WhileStmt(while_stmt) => {
                SerializableStatement::WhileStmt(SerializableWhileStmt::from(while_stmt))
            }
            Statement::ForStmt(for_stmt) => {
                SerializableStatement::ForStmt(SerializableForStmt::from(for_stmt))
            }
            Statement::ReturnStmt(return_stmt) => {
                SerializableStatement::ReturnStmt(SerializableReturnStmt::from(return_stmt))
            }
            Statement::WriteStmt(write_stmt) => {
                SerializableStatement::WriteStmt(SerializableWriteStmt::from(write_stmt))
            }
            Statement::ReadStmt(read_stmt) => {
                SerializableStatement::ReadStmt(SerializableReadStmt::from(read_stmt))
            }
        }
    }
}

impl From<&Statement> for SerializableStatement {
    fn from(stmt: &Statement) -> Self {
        SerializableStatement::from_stmt(stmt)
    }
}

// CORREÇÃO: Implementação simplificada para AnnotatedStatement
impl From<&AnnotatedStatement> for SerializableStatement {
    fn from(annotated: &AnnotatedStatement) -> Self {
        match &annotated.statement {
            Statement::VariableDecl(_) => {
                SerializableStatement::VariableDecl(SerializableVariableDecl::from(annotated))
            }
            Statement::ExprStmt(_) => {
                if let Some(annotated_expr) = annotated.expr_annotations.get(0) {
                    SerializableStatement::ExprStmt(SerializableExprStmt {
                        expr: SerializableExpr::from(annotated_expr),
                    })
                } else {
                    Self::from_stmt(&annotated.statement)
                }
            }
            Statement::IfStmt(_) => {
                SerializableStatement::IfStmt(SerializableIfStmt::from(annotated))
            }
            Statement::WhileStmt(_) => {
                SerializableStatement::WhileStmt(SerializableWhileStmt::from(annotated))
            }
            Statement::ForStmt(_) => {
                SerializableStatement::ForStmt(SerializableForStmt::from(annotated))
            }
            Statement::ReturnStmt(_) => {
                SerializableStatement::ReturnStmt(SerializableReturnStmt::from(annotated))
            }
            Statement::WriteStmt(_) => {
                SerializableStatement::WriteStmt(SerializableWriteStmt::from(annotated))
            }
            Statement::ReadStmt(_) => {
                SerializableStatement::ReadStmt(SerializableReadStmt::from(annotated))
            }
        }
    }
}

// ==================== ESTRUTURAS DE FUNÇÕES ====================

#[derive(Serialize, Deserialize)]
pub struct SerializableFunction {
    pub name: String,
    pub return_type: Option<SerializableType>,
    pub parameters: Vec<SerializableParameter>,
    pub body: SerializableBlock,
}

impl From<&FunctionDecl> for SerializableFunction {
    fn from(func: &FunctionDecl) -> Self {
        SerializableFunction {
            name: func.name.clone(),
            return_type: func.return_type.as_ref().map(SerializableType::from),
            parameters: func.parameters.iter().map(SerializableParameter::from).collect(),
            body: SerializableBlock::from(&func.body),
        }
    }
}

// ==================== ESTRUTURA PRINCIPAL DO PROGRAMA ====================

#[derive(Serialize, Deserialize)]
pub struct SerializableProgram {
    pub functions: Vec<SerializableFunction>,
    pub global_statements: Vec<SerializableStatement>,
    pub metadata: ProgramMetadata,
}

impl From<&Program> for SerializableProgram {
    fn from(program: &Program) -> Self {
        SerializableProgram {
            functions: program.functions.iter().map(SerializableFunction::from).collect(),
            global_statements: program.statements.iter().map(SerializableStatement::from_stmt).collect(),
            metadata: ProgramMetadata {
                source_file: "unknown".to_string(),
                timestamp: Utc::now().to_rfc3339(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                entry_point: Some("main".to_string()),
            },
        }
    }
}

impl From<&SemanticAnalysisResult> for SerializableProgram {
    fn from(result: &SemanticAnalysisResult) -> Self {
        SerializableProgram {
            functions: result.annotated_ast.functions.iter().map(SerializableFunction::from).collect(),
            global_statements: result.annotated_ast.statements.iter().map(SerializableStatement::from).collect(),
            metadata: ProgramMetadata {
                source_file: "unknown".to_string(), 
                timestamp: Utc::now().to_rfc3339(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                entry_point: Some("principal".to_string()),
            },
        }
    }
}

// ==================== FUNÇÕES PÚBLICAS ====================

pub fn save_program_to_json(program: &Program, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let serializable_program = SerializableProgram::from(program);
    let json = serde_json::to_string_pretty(&serializable_program)?;
    fs::write(filename, json)?;
    Ok(())
}

pub fn save_semantic_result_to_json(
    result: &SemanticAnalysisResult, 
    filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let serializable_program = SerializableProgram::from(result);
    let json = serde_json::to_string_pretty(&serializable_program)?;
    fs::write(filename, json)?;
    Ok(())
}

pub fn load_program_from_json(filename: &str) -> Result<SerializableProgram, Box<dyn std::error::Error>> {
    let json = fs::read_to_string(filename)?;
    let program: SerializableProgram = serde_json::from_str(&json)?;
    Ok(program)
}

// ==================== TESTES ====================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Literal, Type};

    #[test]
    fn test_serialize_literal() {
        let literal = Literal::Inteiro(42);
        let serializable = SerializableLiteral::from(&literal);
        
        let json = serde_json::to_string(&serializable).unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("Inteiro"));
    }

    #[test]
    fn test_serialize_type() {
        let ty = Type::Decimal;
        let serializable = SerializableType::from(&ty);
        
        let json = serde_json::to_string(&serializable).unwrap();
        assert!(json.contains("Decimal"));
    }

    #[test]
    fn test_serialize_empty_program() {
        let program = Program {
            functions: vec![],
            statements: vec![],
        };
        
        let serializable = SerializableProgram::from(&program);
        let json = serde_json::to_string_pretty(&serializable).unwrap();
        
        assert!(json.contains("\"functions\""));
        assert!(json.contains("\"global_statements\""));
        assert!(json.contains("\"metadata\""));
    }

    #[test]
    fn test_save_and_load_json() {
        let program = Program {
            functions: vec![],
            statements: vec![],
        };
        
        let test_file = "test_program.ast.json";
        save_program_to_json(&program, test_file).unwrap();
        
        let loaded = load_program_from_json(test_file).unwrap();
        assert!(loaded.functions.is_empty());
        assert!(loaded.global_statements.is_empty());
        
        // Limpar
        let _ = fs::remove_file(test_file);
    }
}