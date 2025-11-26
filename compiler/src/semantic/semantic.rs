use crate::parser::ast::*;
use crate::semantic::symbol_table::{SymbolTable, Symbol};
use crate::semantic::type_checker::TypeChecker;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotatedExpr {
    pub expr: Expr,
    pub type_: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnnotatedStatement {
    pub statement: Statement,
    pub expr_annotations: Vec<AnnotatedExpr>,
}

#[derive(Debug)]
pub struct SemanticAnalysisResult {
    pub annotated_ast: Program,
    pub symbol_table: SymbolTable,
    pub errors: Vec<SemanticError>,
}

#[derive(Debug, Clone)]
pub struct SemanticError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    errors: Vec<SemanticError>,
    current_function_return: Option<Type>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            errors: Vec::new(),
            current_function_return: None,
        }
    }

    pub fn analyze(&mut self, program: Program) -> SemanticAnalysisResult {

        self.collect_declarations(&program);

        let annotated_ast = self.check_and_annotate_program(program);

        SemanticAnalysisResult {
            annotated_ast,
            symbol_table: self.symbol_table.clone(),
            errors: self.errors.clone(),
        }
    }

    fn collect_declarations(&mut self, program: &Program) {

        for function in &program.functions {
            let symbol = Symbol::Function {
                declaration: function.clone(),
            };
            
            if let Err(e) = self.symbol_table.insert_symbol(function.name.clone(), symbol) {
                self.report_error(&e, 0, 0);
            }
        }

        for statement in &program.statements {
            if let Statement::VariableDecl(decl) = statement {
                let symbol = Symbol::Variable {
                    name: decl.name.clone(),
                    type_: decl.var_type.clone(),
                    defined: decl.initializer.is_some(),
                };
                
                if let Err(e) = self.symbol_table.insert_symbol(decl.name.clone(), symbol) {
                    self.report_error(&e, 0, 0);
                }
            }
        }
    }

    fn check_and_annotate_program(&mut self, program: Program) -> Program {

        let mut annotated_global_statements = Vec::new();
        
        for statement in program.statements {
            match self.check_and_annotate_statement(statement) {
                Ok(annotated) => annotated_global_statements.push(annotated.statement),
                Err(_) => continue,
            }
        }

        let mut annotated_functions = Vec::new();
        
        for function in program.functions {
            match self.check_and_annotate_function(function) {
                Ok(annotated) => annotated_functions.push(annotated),
                Err(_) => continue,
            }
        }

        Program {
            functions: annotated_functions,
            statements: annotated_global_statements,
        }
    }

    fn check_and_annotate_function(&mut self, mut function: FunctionDecl) -> Result<FunctionDecl, ()> {
        self.symbol_table.enter_scope();
        
        for param in &function.parameters {
            let symbol = Symbol::Variable {
                name: param.name.clone(),
                type_: param.param_type.clone(),
                defined: true,
            };
            
            if let Err(e) = self.symbol_table.insert_symbol(param.name.clone(), symbol) {
                self.report_error(&e, 0, 0);
                self.symbol_table.exit_scope();
                return Err(());
            }
        }

        let old_return_context = self.current_function_return.take();
        self.current_function_return = function.return_type.clone();

        let annotated_body = self.check_and_annotate_block(function.body)?;

        self.current_function_return = old_return_context;
        self.symbol_table.exit_scope();

        function.body = annotated_body;
        Ok(function)
    }

    fn check_and_annotate_block(&mut self, block: Block) -> Result<Block, ()> {
        self.symbol_table.enter_scope();
        
        let mut annotated_statements = Vec::new();
        
        for statement in block.statements {
            match self.check_and_annotate_statement(statement) {
                Ok(annotated) => annotated_statements.push(annotated.statement),
                Err(_) => continue,
            }
        }

        self.symbol_table.exit_scope();
        
        Ok(Block {
            statements: annotated_statements,
        })
    }

    fn check_and_annotate_statement(&mut self, statement: Statement) -> Result<AnnotatedStatement, ()> {
        let mut expr_annotations = Vec::new();
        let annotated_statement = match statement {
            Statement::VariableDecl(decl) => {
                self.check_variable_declaration(decl, &mut expr_annotations)?
            }
            Statement::ExprStmt(expr_stmt) => {
                self.check_expression_statement(expr_stmt, &mut expr_annotations)?
            }
            Statement::IfStmt(if_stmt) => {
                self.check_if_statement(if_stmt, &mut expr_annotations)?
            }
            Statement::WhileStmt(while_stmt) => {
                self.check_while_statement(while_stmt, &mut expr_annotations)?
            }
            Statement::ForStmt(for_stmt) => {
                self.check_for_statement(for_stmt, &mut expr_annotations)?
            }
            Statement::ReturnStmt(return_stmt) => {
                self.check_return_statement(return_stmt, &mut expr_annotations)?
            }
            Statement::WriteStmt(write_stmt) => {
                self.check_write_statement(write_stmt, &mut expr_annotations)?
            }
            Statement::ReadStmt(read_stmt) => {
                self.check_read_statement(read_stmt, &mut expr_annotations)?
            }
        };

        Ok(AnnotatedStatement {
            statement: annotated_statement,
            expr_annotations,
        })
    }

    fn check_variable_declaration(
        &mut self,
        decl: VariableDecl,
        annotations: &mut Vec<AnnotatedExpr>,
    ) -> Result<Statement, ()> {

        let symbol = Symbol::Variable {
            name: decl.name.clone(),
            type_: decl.var_type.clone(),
            defined: decl.initializer.is_some(),
        };
        
        if let Err(e) = self.symbol_table.insert_symbol(decl.name.clone(), symbol) {
            self.report_error(&e, 0, 0);
            return Err(());
        }

        if let Some(initializer) = decl.initializer {
            let (annotated_expr, expr_type) = self.check_and_annotate_expression(initializer)?;
            
            if expr_type != decl.var_type {
                self.report_error(
                    &format!(
                        "Tipo do inicializador ({}) não corresponde ao tipo da variável ({})",
                        expr_type, decl.var_type
                    ),
                    0, 0
                );
                return Err(());
            }
            
            let new_annotated_expr = AnnotatedExpr {
                expr: annotated_expr.expr.clone(),
                type_: annotated_expr.type_.clone(),
            };
            
            annotations.push(new_annotated_expr);
            
            Ok(Statement::VariableDecl(VariableDecl {
                var_type: decl.var_type,
                name: decl.name,
                initializer: Some(annotated_expr.expr),
            }))
        } else {
            Ok(Statement::VariableDecl(decl))
        }
    }

    fn check_expression_statement(
        &mut self,
        expr_stmt: ExprStmt,
        annotations: &mut Vec<AnnotatedExpr>,
    ) -> Result<Statement, ()> {
        let (annotated_expr, _) = self.check_and_annotate_expression(expr_stmt.expr)?;
        
        let new_annotated_expr = AnnotatedExpr {
            expr: annotated_expr.expr.clone(),
            type_: annotated_expr.type_.clone(),
        };
        
        annotations.push(new_annotated_expr);
        
        Ok(Statement::ExprStmt(ExprStmt {
            expr: annotated_expr.expr,
        }))
    }

    fn check_if_statement(
        &mut self,
        if_stmt: IfStmt,
        annotations: &mut Vec<AnnotatedExpr>,
    ) -> Result<Statement, ()> {
        let (annotated_condition, condition_type) = self.check_and_annotate_expression(if_stmt.condition)?;
        
        if condition_type != Type::Logico {
            self.report_error("Condição do if deve ser do tipo lógico", 0, 0);
            return Err(());
        }
        
        let new_annotated_condition = AnnotatedExpr {
            expr: annotated_condition.expr.clone(),
            type_: annotated_condition.type_.clone(),
        };
        
        annotations.push(new_annotated_condition);
        
        let then_branch = self.check_and_annotate_block(if_stmt.then_branch)?;
        let else_branch = if_stmt.else_branch.map(|b| self.check_and_annotate_block(b)).transpose()?;
        
        Ok(Statement::IfStmt(IfStmt {
            condition: annotated_condition.expr,
            then_branch,
            else_branch,
        }))
    }

    fn check_return_statement(
        &mut self,
        return_stmt: ReturnStmt,
        annotations: &mut Vec<AnnotatedExpr>,
    ) -> Result<Statement, ()> {

        let current_return = self.current_function_return.clone();
        
        match (return_stmt.value, current_return) {
            (Some(value), Some(expected_type)) => {
                let (annotated_value, actual_type) = self.check_and_annotate_expression(value)?;
                
                if actual_type != expected_type {
                    self.report_error(
                        &format!("Tipo de retorno esperado: {}, encontrado: {}", expected_type, actual_type),
                        0, 0
                    );
                    return Err(());
                }
                
                let new_annotated_value = AnnotatedExpr {
                    expr: annotated_value.expr.clone(),
                    type_: annotated_value.type_.clone(),
                };
                
                annotations.push(new_annotated_value);
                
                Ok(Statement::ReturnStmt(ReturnStmt {
                    value: Some(annotated_value.expr),
                }))
            }
            (Some(_), None) => {
                self.report_error("Retorno com valor em função sem tipo de retorno", 0, 0);
                Err(())
            }
            (None, Some(_)) => {
                self.report_error("Retorno sem valor em função com tipo de retorno", 0, 0);
                Err(())
            }
            (None, None) => Ok(Statement::ReturnStmt(ReturnStmt { value: None })),
        }
    }

    fn check_and_annotate_expression(&mut self, expr: Expr) -> Result<(AnnotatedExpr, Type), ()> {
        match TypeChecker::infer_expression_type(&expr, &self.symbol_table) {
            Ok(type_) => {
                let annotated_expr = AnnotatedExpr { expr, type_: type_.clone() };
                Ok((annotated_expr, type_))
            }
            Err(e) => {
                self.report_error(&e, 0, 0);
                Err(())
            }
        }
    }

    fn report_error(&mut self, message: &str, line: usize, column: usize) {
        self.errors.push(SemanticError {
            message: message.to_string(),
            line,
            column,
        });
    }

    fn check_while_statement(
        &mut self,
        while_stmt: WhileStmt,
        annotations: &mut Vec<AnnotatedExpr>,
    ) -> Result<Statement, ()> {

        let (annotated_condition, condition_type) = self.check_and_annotate_expression(while_stmt.condition)?;
        
        if condition_type != Type::Logico {
            self.report_error("Condição do while deve ser do tipo lógico", 0, 0);
            return Err(());
        }
        
        let new_annotated_condition = AnnotatedExpr {
            expr: annotated_condition.expr.clone(),
            type_: annotated_condition.type_.clone(),
        };
        annotations.push(new_annotated_condition);
        
        let body = self.check_and_annotate_block(while_stmt.body)?;
        
        Ok(Statement::WhileStmt(WhileStmt {
            condition: annotated_condition.expr,
            body,
        }))
    }

    fn check_for_statement(&mut self, for_stmt: ForStmt, annotations: &mut Vec<AnnotatedExpr>) -> Result<Statement, ()> {

        let (annotated_start, _) = self.check_and_annotate_expression(for_stmt.start.clone())?;
        let (annotated_end, _) = self.check_and_annotate_expression(for_stmt.end.clone())?;
        
        let new_annotated_start = AnnotatedExpr {
            expr: annotated_start.expr.clone(),
            type_: annotated_start.type_.clone(),
        };
        let new_annotated_end = AnnotatedExpr {
            expr: annotated_end.expr.clone(),
            type_: annotated_end.type_.clone(),
        };
        
        annotations.push(new_annotated_start);
        annotations.push(new_annotated_end);
        
        let body = self.check_and_annotate_block(for_stmt.body)?;
        Ok(Statement::ForStmt(ForStmt {
            variable: for_stmt.variable,
            start: for_stmt.start,
            end: for_stmt.end,
            body,
        }))
    }

    fn check_write_statement(
        &mut self,
        write_stmt: WriteStmt,
        annotations: &mut Vec<AnnotatedExpr>,
    ) -> Result<Statement, ()> {
        let mut checked_arguments = Vec::new();
        
        for arg in write_stmt.arguments {
            let (annotated_arg, _arg_type) = self.check_and_annotate_expression(arg)?; 
            
            let new_annotated_arg = AnnotatedExpr {
                expr: annotated_arg.expr.clone(),
                type_: annotated_arg.type_.clone(),
            };
            annotations.push(new_annotated_arg);
            checked_arguments.push(annotated_arg.expr);
        }
        
        Ok(Statement::WriteStmt(WriteStmt {
            arguments: checked_arguments,
        }))
    }

    fn check_read_statement(&mut self, read_stmt: ReadStmt, annotations: &mut Vec<AnnotatedExpr>) -> Result<Statement, ()> {
        let (annotated_target, _target_type) = self.check_and_annotate_expression(read_stmt.target)?;
        
        match &annotated_target.expr {
            Expr::Variable(_) => {
            }
            _ => {
                self.report_error("Comando 'leia' só pode ser usado com variáveis", 0, 0);
                return Err(());
            }
        }
        
        let new_annotated_target = AnnotatedExpr {
            expr: annotated_target.expr.clone(),
            type_: annotated_target.type_.clone(),
        };
        annotations.push(new_annotated_target);
        
        Ok(Statement::ReadStmt(ReadStmt {
            target: annotated_target.expr,
        }))
    }
}