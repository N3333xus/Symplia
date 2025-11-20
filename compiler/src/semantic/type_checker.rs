use crate::parser::ast::{Expr, Type, Literal, BinaryOperator, UnaryOperator};
use crate::semantic::symbol_table::SymbolTable;

#[derive(Debug)]
pub struct TypeChecker;

impl TypeChecker {
    pub fn infer_expression_type(expr: &Expr, symbol_table: &SymbolTable) -> Result<Type, String> {
        match expr {
            Expr::Literal(literal) => Self::infer_literal_type(literal),
            Expr::Variable(name) => Self::infer_variable_type(name, symbol_table),
            Expr::BinaryOp(op, left, right) => Self::infer_binary_op_type(op, left, right, symbol_table),
            Expr::UnaryOp(op, operand) => Self::infer_unary_op_type(op, operand, symbol_table),
            Expr::Call(call_expr) => Self::infer_call_type(call_expr, symbol_table),
        }
    }

    fn infer_literal_type(literal: &Literal) -> Result<Type, String> {
        match literal {
            Literal::Inteiro(_) => Ok(Type::Inteiro),
            Literal::Decimal(_) => Ok(Type::Decimal),
            Literal::Texto(_) => Ok(Type::Texto),
            Literal::Logico(_) => Ok(Type::Logico),
        }
    }

    fn infer_variable_type(name: &str, symbol_table: &SymbolTable) -> Result<Type, String> {
        match symbol_table.lookup(name) {
            Some(symbol) => match symbol {
                crate::semantic::symbol_table::Symbol::Variable { type_, .. } => Ok(type_.clone()),
                crate::semantic::symbol_table::Symbol::Function { declaration } => {
                    if let Some(return_type) = &declaration.return_type {
                        Ok(return_type.clone())
                    } else {
                        Err(format!("Função '{}' não tem tipo de retorno", name))
                    }
                }
            },
            None => Err(format!("Variável '{}' não declarada", name)),
        }
    }

    fn infer_binary_op_type(
        op: &BinaryOperator, 
        left: &Expr, 
        right: &Expr,
        symbol_table: &SymbolTable
    ) -> Result<Type, String> {
        let left_type = Self::infer_expression_type(left, symbol_table)?;
        let right_type = Self::infer_expression_type(right, symbol_table)?;

        match op {
            BinaryOperator::Add | BinaryOperator::Subtract | BinaryOperator::Multiply | BinaryOperator::Divide => {
                Self::check_arithmetic_operation(&left_type, &right_type, op)
            }
            BinaryOperator::Modulo => {
                if left_type == Type::Inteiro && right_type == Type::Inteiro {
                    Ok(Type::Inteiro)
                } else {
                    Err(format!("Operador '{}' requer operandos inteiros", op))
                }
            }
            BinaryOperator::Equal | BinaryOperator::NotEqual => {
                if left_type == right_type {
                    Ok(Type::Logico)
                } else {
                    Err(format!("Operador '{}' requer operandos do mesmo tipo: {} != {}", op, left_type, right_type))
                }
            }
            BinaryOperator::Less | BinaryOperator::LessEqual | BinaryOperator::Greater | BinaryOperator::GreaterEqual => {
                Self::check_comparison_operation(&left_type, &right_type, op)
            }
            BinaryOperator::And | BinaryOperator::Or => {
                if left_type == Type::Logico && right_type == Type::Logico {
                    Ok(Type::Logico)
                } else {
                    Err(format!("Operador '{}' requer operandos lógicos", op))
                }
            }
        }
    }

    fn infer_unary_op_type(op: &UnaryOperator, operand: &Expr, symbol_table: &SymbolTable) -> Result<Type, String> {
        let operand_type = Self::infer_expression_type(operand, symbol_table)?;

        match op {
            UnaryOperator::Negate | UnaryOperator::Plus => {
                if operand_type == Type::Inteiro || operand_type == Type::Decimal {
                    Ok(operand_type)
                } else {
                    Err(format!("Operador unário '{:?}' requer operando numérico", op))
                }
            }
            UnaryOperator::Not => {
                if operand_type == Type::Logico {
                    Ok(Type::Logico)
                } else {
                    Err(format!("Operador '!' requer operando lógico"))
                }
            }
        }
    }

    fn infer_call_type(call_expr: &crate::parser::ast::CallExpr, symbol_table: &SymbolTable) -> Result<Type, String> {
        let func_info = Self::get_function_info(&call_expr.function, symbol_table)?;

        if call_expr.arguments.len() != func_info.parameters.len() {
            return Err(format!(
                "Função '{}' espera {} argumentos, mas {} foram fornecidos",
                call_expr.function,
                func_info.parameters.len(),
                call_expr.arguments.len()
            ));
        }

        for (i, (arg, param)) in call_expr.arguments.iter().zip(func_info.parameters.iter()).enumerate() {
            let arg_type = Self::infer_expression_type(arg, symbol_table)?;
            if arg_type != param.param_type {
                return Err(format!(
                    "Argumento {} da função '{}': tipo esperado {}, encontrado {}",
                    i + 1,
                    call_expr.function,
                    param.param_type,
                    arg_type
                ));
            }
        }

        func_info.return_type
            .clone()
            .ok_or_else(|| format!("Função '{}' não tem tipo de retorno", call_expr.function))
    }

    fn get_function_info(function_name: &str, symbol_table: &SymbolTable) -> Result<FunctionInfo, String> {
        match symbol_table.lookup(function_name) {
            Some(symbol) => {
                if let crate::semantic::symbol_table::Symbol::Function { declaration } = symbol {
                    Ok(FunctionInfo {
                        parameters: declaration.parameters.clone(),
                        return_type: declaration.return_type.clone(),
                    })
                } else {
                    Err(format!("'{}' não é uma função", function_name))
                }
            }
            None => Err(format!("Função '{}' não declarada", function_name)),
        }
    }

    fn check_arithmetic_operation(left: &Type, right: &Type, op: &BinaryOperator) -> Result<Type, String> {
        match (left, right) {
            (Type::Inteiro, Type::Inteiro) => Ok(Type::Inteiro),
            (Type::Decimal, Type::Decimal) => Ok(Type::Decimal),
            (Type::Inteiro, Type::Decimal) | (Type::Decimal, Type::Inteiro) => Ok(Type::Decimal),
            _ => Err(format!("Operador '{}' não suportado para tipos {} e {}", op, left, right)),
        }
    }

    fn check_comparison_operation(left: &Type, right: &Type, op: &BinaryOperator) -> Result<Type, String> {
        match (left, right) {
            (Type::Inteiro, Type::Inteiro)
            | (Type::Decimal, Type::Decimal)
            | (Type::Inteiro, Type::Decimal)
            | (Type::Decimal, Type::Inteiro) => Ok(Type::Logico),
            (Type::Texto, Type::Texto) => Ok(Type::Logico),
            _ => Err(format!("Operador '{}' não suportado para tipos {} e {}", op, left, right)),
        }
    }
}

#[derive(Debug)]
struct FunctionInfo {
    parameters: Vec<crate::parser::ast::Parameter>,
    return_type: Option<Type>,
}