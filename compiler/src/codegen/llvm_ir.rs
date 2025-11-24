use inkwell::context::Context;
use inkwell::values::{FunctionValue, BasicValueEnum, PointerValue, BasicValue, BasicMetadataValueEnum /*, GlobalValue*/};
//use inkwell::values::AnyValue;
use inkwell::types::{BasicType /*, BasicTypeEnum*/, BasicMetadataTypeEnum}; // Adicionar BasicMetadataTypeEnum
use crate::parser::ast::*;
use crate::semantic::SemanticAnalysisResult;
use super::context::CodeGenContext;
use super::builder::IRBuilder;
//use either::Either;

pub struct LLVMCodeGenerator<'ctx> {
    context: &'ctx Context,
    ir_builder: IRBuilder<'ctx>,
}

impl<'ctx> LLVMCodeGenerator<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let ir_builder = IRBuilder::new(context);
        Self { context, ir_builder }
    }

    pub fn generate_ir(&self, semantic_result: &SemanticAnalysisResult, module_name: &str) -> Result<String, String> {
        let mut codegen_context = CodeGenContext::new(self.context, module_name);
        
        for function in &semantic_result.annotated_ast.functions {
            self.declare_function(&mut codegen_context, function)?;
        }

        for function in &semantic_result.annotated_ast.functions {
            self.define_function(&mut codegen_context, function)?;
        }

        for statement in &semantic_result.annotated_ast.statements {
            self.generate_statement(&mut codegen_context, statement)?;
        }

        Ok(codegen_context.module.print_to_string().to_string())
    }

    fn declare_function(&self, context: &mut CodeGenContext<'ctx>, function: &FunctionDecl) -> Result<(), String> {
        let param_types: Vec<BasicMetadataTypeEnum<'ctx>> = function.parameters
            .iter()
            .map(|param| self.ir_builder.get_llvm_type(&param.param_type).into())
            .collect();

        let function_type = if let Some(ref ret_type) = function.return_type {
            let return_type = self.ir_builder.get_llvm_type(ret_type);
            return_type.fn_type(&param_types, false)
        } else {
            self.context.void_type().fn_type(&param_types, false)
        };

        let function_value = context.module.add_function(&function.name, function_type, None);

        context.named_values.insert(function.name.clone(), function_value.as_global_value().as_basic_value_enum());

        Ok(())
    }

    fn define_function(&self, context: &mut CodeGenContext<'ctx>, function: &FunctionDecl) -> Result<(), String> {
        let function_value = context.module.get_function(&function.name)
            .ok_or_else(|| format!("Função '{}' não declarada", function.name))?;

        let entry_block = context.context.append_basic_block(function_value, "entry");
        context.builder.position_at_end(entry_block);

        context.named_values.clear();

        for (i, param) in function_value.get_param_iter().enumerate() {
            let param_name = &function.parameters[i].name;
            let alloca = self.create_entry_block_alloca(context, function_value, param_name, &function.parameters[i].param_type)?;
            context.builder.build_store(alloca, param).map_err(|e| e.to_string())?;
            context.named_values.insert(param_name.clone(), alloca.into());
        }

        for statement in &function.body.statements {
            self.generate_statement(context, statement)?;
        }

        if function.return_type.is_none() && function_value.get_type().get_return_type().is_none() {
            context.builder.build_return(None).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    fn generate_statement(&self, context: &mut CodeGenContext<'ctx>, statement: &Statement) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        match statement {
            Statement::VariableDecl(decl) => self.generate_variable_decl(context, decl),
            Statement::ExprStmt(expr_stmt) => self.generate_expression_stmt(context, expr_stmt),
            Statement::ReturnStmt(return_stmt) => self.generate_return_stmt(context, return_stmt),
            Statement::IfStmt(if_stmt) => self.generate_if_stmt(context, if_stmt),
            Statement::WhileStmt(while_stmt) => self.generate_while_stmt(context, while_stmt),
            Statement::ForStmt(for_stmt) => self.generate_for_stmt(context, for_stmt),
            Statement::WriteStmt(write_stmt) => self.generate_write_stmt(context, write_stmt),
            Statement::ReadStmt(read_stmt) => self.generate_read_stmt(context, read_stmt),
        }
    }

    fn generate_variable_decl(&self, context: &mut CodeGenContext<'ctx>, decl: &VariableDecl) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        let function = context.builder.get_insert_block()
            .and_then(|block| block.get_parent())
            .ok_or("Não está dentro de uma função")?;

        let alloca = self.create_entry_block_alloca(context, function, &decl.name, &decl.var_type)?;

        if let Some(initializer) = &decl.initializer {
            let init_value = self.generate_expression(context, initializer)?;
            context.builder.build_store(alloca, init_value).map_err(|e| e.to_string())?;
        }

        context.named_values.insert(decl.name.clone(), alloca.into());
        Ok(None)
    }

    fn generate_expression_stmt(&self, context: &mut CodeGenContext<'ctx>, expr_stmt: &ExprStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        self.generate_expression(context, &expr_stmt.expr).map(Some)
    }

    fn generate_return_stmt(&self, context: &mut CodeGenContext<'ctx>, return_stmt: &ReturnStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        if let Some(value) = &return_stmt.value {
            let ret_value = self.generate_expression(context, value)?;
            context.builder.build_return(Some(&ret_value)).map_err(|e| e.to_string())?;
        } else {
            context.builder.build_return(None).map_err(|e| e.to_string())?;
        }
        Ok(None)
    }

    fn generate_expression(&self, context: &mut CodeGenContext<'ctx>, expr: &Expr) -> Result<BasicValueEnum<'ctx>, String> {
        match expr {
            Expr::Literal(literal) => Ok(self.ir_builder.build_literal(literal)),
            Expr::Variable(name) => {
                context.named_values.get(name)
                    .cloned()
                    .ok_or_else(|| format!("Variável '{}' não encontrada", name))
            }
            Expr::BinaryOp(op, left, right) => {
                let left_val = self.generate_expression(context, left)?;
                let right_val = self.generate_expression(context, right)?;
                
                let left_type = Type::Inteiro; // Placeholder
                let right_type = Type::Inteiro; // Placeholder
                
                self.ir_builder.build_binary_op(&context.builder, op, left_val, right_val, &left_type, &right_type)
            }
            Expr::UnaryOp(op, operand) => {
                let operand_val = self.generate_expression(context, operand)?;
                let operand_type = Type::Inteiro; // Placeholder
                self.ir_builder.build_unary_op(&context.builder, op, operand_val, &operand_type)
            }
            Expr::Call(call_expr) => self.generate_call_expr(context, call_expr),
        }
    }

    fn create_entry_block_alloca(
        &self,
        context: &mut CodeGenContext<'ctx>,
        function: FunctionValue<'ctx>,
        name: &str,
        var_type: &Type,
    ) -> Result<PointerValue<'ctx>, String> {
        let builder = context.context.create_builder();
        let entry_block = function.get_first_basic_block().unwrap();
        
        if let Some(first_instr) = entry_block.get_first_instruction() {
            builder.position_before(&first_instr);
        } else {
            builder.position_at_end(entry_block);
        }

        let llvm_type = self.ir_builder.get_llvm_type(var_type);
        builder.build_alloca(llvm_type, name).map_err(|e| e.to_string())
    }

    // Placeholders para estruturas mais complexas
    fn generate_if_stmt(&self, _context: &mut CodeGenContext<'ctx>, _if_stmt: &IfStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        // Implementação do if
        Ok(None)
    }

    fn generate_while_stmt(&self, _context: &mut CodeGenContext<'ctx>, _while_stmt: &WhileStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        // Implementação do while
        Ok(None)
    }

    fn generate_for_stmt(&self, _context: &mut CodeGenContext<'ctx>, _for_stmt: &ForStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        // Implementação do for
        Ok(None)
    }

    fn generate_write_stmt(&self, _context: &mut CodeGenContext<'ctx>, _write_stmt: &WriteStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        // Implementação do escreva
        Ok(None)
    }

    fn generate_read_stmt(&self, _context: &mut CodeGenContext<'ctx>, _read_stmt: &ReadStmt) -> Result<Option<BasicValueEnum<'ctx>>, String> {
        // Implementação do leia
        Ok(None)
    }

    fn generate_call_expr(
        &self,
        context: &mut CodeGenContext<'ctx>,
        call_expr: &CallExpr,
    ) -> Result<BasicValueEnum<'ctx>, String> {
        use inkwell::values::{BasicMetadataValueEnum, BasicValueEnum, CallSiteValue};

        // Busca a função no módulo
        let function = context
            .module
            .get_function(&call_expr.function)
            .ok_or_else(|| format!("Função '{}' não definida", call_expr.function))?;

        // Gera os argumentos
        let args: Result<Vec<BasicValueEnum<'ctx>>, String> = call_expr
            .arguments
            .iter()
            .map(|arg| self.generate_expression(context, arg))
            .collect();

        let args = args?;
        let args_metadata: Vec<BasicMetadataValueEnum<'ctx>> =
            args.into_iter().map(Into::into).collect();

        let call_site: CallSiteValue<'ctx> = context
            .builder
            .build_call(function, &args_metadata, "calltmp")
            .map_err(|e| e.to_string())?;

        // Ajuste do erro: try_as_basic_value() retorna Either<BasicValueEnum, CallSiteValue>
        // Usar `.left()` para obter Option<BasicValueEnum>
        match call_site.try_as_basic_value().left() {
            Some(value) => Ok(value),
            None => Ok(self.context.i32_type().const_int(0, false).into()),
        }
    }


}