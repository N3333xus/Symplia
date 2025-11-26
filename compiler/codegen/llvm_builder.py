"""
LLVM IR Builder para compilador Symplia 
Gera LLVM IR a partir da AST desserializada usando llvmlite
"""

from llvmlite import ir, binding
from typing import List, Dict, Optional #, Union, Any
from dataclasses import dataclass
from .ast_nodes import (
    SerializableProgram, SerializableFunction, SerializableStatement,
    SerializableExpr, SerializableType, SerializableBinaryOperator,
    SerializableUnaryOperator, SerializableLiteral, SerializableCallExpr,
    SerializableVariableDecl, SerializableBlock
)

binding.initialize()
binding.initialize_native_target()
binding.initialize_native_asmprinter()

@dataclass
class VariableInfo:
    """Informação sobre variáveis no escopo atual"""
    ir_value: ir.Value
    var_type: SerializableType
    is_initialized: bool = True

class LLVMBuilder:
    """Construtor de código LLVM IR para Symplia"""
    
    def __init__(self, module_name: str = "symplia_module"):
        self.module = ir.Module(module_name)
        self.builder: Optional[ir.IRBuilder] = None
        self.current_function: Optional[ir.Function] = None
        
        self.symbol_tables: List[Dict[str, VariableInfo]] = [{}]
        
        self._declare_runtime_functions()
    
    def _declare_runtime_functions(self):
        """Declara funções de runtime (I/O, etc.)"""

        printf_ty = ir.FunctionType(ir.IntType(32), [ir.PointerType(ir.IntType(8))], var_arg=True)
        self.printf = ir.Function(self.module, printf_ty, name="printf")
        
        scanf_ty = ir.FunctionType(ir.IntType(32), [ir.PointerType(ir.IntType(8))], var_arg=True)
        self.scanf = ir.Function(self.module, scanf_ty, name="scanf")
        
        malloc_ty = ir.FunctionType(ir.PointerType(ir.IntType(8)), [ir.IntType(64)])
        self.malloc = ir.Function(self.module, malloc_ty, name="malloc")
    
    def generate_ir(self, program: SerializableProgram) -> str:
        """Gera código LLVM IR para um programa completo"""
        try:

            for function in program.functions:
                self._generate_function_declaration(function)
            
            for function in program.functions:
                self._generate_function_definition(function)
            
            if program.global_statements:
                self._generate_global_statements(program.global_statements)
            
            return str(self.module)
        
        except Exception as e:
            raise RuntimeError(f"Erro na geração de código LLVM: {e}")
    
    def _generate_function_declaration(self, function: SerializableFunction):
        """Declara uma função (sem gerar corpo)"""
        return_type = self._map_type(function.return_type or SerializableType.Void)
        param_types = [self._map_type(param.param_type) for param in function.parameters]
        
        func_type = ir.FunctionType(return_type, param_types)
        ir_function = ir.Function(self.module, func_type, name=function.name)
        
        self.symbol_tables[0][function.name] = VariableInfo(
            ir_value=ir_function,
            var_type=function.return_type or SerializableType.Void
        )
    
    def _generate_function_definition(self, function: SerializableFunction):
        """Gera o corpo de uma função"""
        ir_function = self.symbol_tables[0][function.name].ir_value
        
        entry_block = ir_function.append_basic_block(name="entry")
        self.builder = ir.IRBuilder(entry_block)
        self.current_function = ir_function
        
        self.symbol_tables.append({})
        
        for i, (param, ir_param) in enumerate(zip(function.parameters, ir_function.args)):
            param_name = param.name
            ir_param.name = param_name
            
            alloca = self.builder.alloca(self._map_type(param.param_type), name=param_name)
            self.builder.store(ir_param, alloca)
            
            self.symbol_tables[-1][param_name] = VariableInfo(
                ir_value=alloca,
                var_type=param.param_type
            )
        
        self._generate_block(function.body)
        
        if not self.builder.block.is_terminated:
            if function.return_type is None or function.return_type == SerializableType.Void:
                self.builder.ret_void()
            else:

                default_value = self._get_default_value(function.return_type)
                self.builder.ret(default_value)
        
        self.symbol_tables.pop()
        self.current_function = None
    
    def _generate_block(self, block: SerializableBlock):
        """Gera código para um bloco de statements"""
        for statement in block.statements:
            if self.builder.block.is_terminated:
                break  # Não gerar código após retorno
            self._generate_statement(statement)
    
    def _generate_statement(self, statement: SerializableStatement):
        """Gera código para um statement"""
        try:
            stmt_type = statement.statement_type
            
            if stmt_type == "VariableDecl":
                self._generate_variable_decl(statement.get_variable_decl())
            elif stmt_type == "ExprStmt":
                self._generate_expr_stmt(statement.get_expr_stmt())
            elif stmt_type == "IfStmt":
                self._generate_if_stmt(statement.get_if_stmt())
            elif stmt_type == "WhileStmt":
                self._generate_while_stmt(statement.get_while_stmt())
            elif stmt_type == "ForStmt":
                self._generate_for_stmt(statement.get_for_stmt())
            elif stmt_type == "ReturnStmt":
                self._generate_return_stmt(statement.get_return_stmt())
            elif stmt_type == "WriteStmt":
                self._generate_write_stmt(statement.get_write_stmt())
            elif stmt_type == "ReadStmt":
                self._generate_read_stmt(statement.get_read_stmt())
            else:
                raise ValueError(f"Tipo de statement não suportado: {stmt_type}")
                
        except Exception as e:
            raise RuntimeError(f"Erro ao gerar statement {statement.statement_type}: {e}")
    
    def _generate_variable_decl(self, decl: SerializableVariableDecl):
        """Gera código para declaração de variável"""

        var_type = self._map_type(decl.var_type)
        alloca = self.builder.alloca(var_type, name=decl.name)
        
        self.symbol_tables[-1][decl.name] = VariableInfo(
            ir_value=alloca,
            var_type=decl.var_type
        )
        
        if decl.initializer:
            init_value = self._generate_expression(decl.initializer)
            self.builder.store(init_value, alloca)
    
    def _generate_expr_stmt(self, stmt):
        """Gera código para statement de expressão"""
        self._generate_expression(stmt.expr)
    
    def _generate_if_stmt(self, stmt):
        """Gera código para if statement"""

        cond_value = self._generate_expression(stmt.condition)
        
        then_block = self.current_function.append_basic_block("then")
        else_block = self.current_function.append_basic_block("else")
        merge_block = self.current_function.append_basic_block("if_merge")
        
        self.builder.cbranch(cond_value, then_block, else_block)

        self.builder.position_at_end(then_block)
        self.symbol_tables.append({})
        self._generate_block(stmt.then_branch)
        if not self.builder.block.is_terminated:
            self.builder.branch(merge_block)
        self.symbol_tables.pop()
        
        self.builder.position_at_end(else_block)
        if stmt.else_branch:
            self.symbol_tables.append({})
            self._generate_block(stmt.else_branch)
            self.symbol_tables.pop()
        if not self.builder.block.is_terminated:
            self.builder.branch(merge_block)
        
        # Continuar no merge block
        self.builder.position_at_end(merge_block)
    
    def _generate_while_stmt(self, stmt):
        """Gera código para while statement"""
        # Criar blocos
        cond_block = self.current_function.append_basic_block("while_cond")
        body_block = self.current_function.append_basic_block("while_body")
        end_block = self.current_function.append_basic_block("while_end")
        
        # Branch para condição
        self.builder.branch(cond_block)
        
        # Gerar condição
        self.builder.position_at_end(cond_block)
        cond_value = self._generate_expression(stmt.condition)
        self.builder.cbranch(cond_value, body_block, end_block)
        
        # Gerar corpo
        self.builder.position_at_end(body_block)
        self.symbol_tables.append({})
        self._generate_block(stmt.body)
        if not self.builder.block.is_terminated:
            self.builder.branch(cond_block)  # Loop
        self.symbol_tables.pop()
        
        # Continuar no end block
        self.builder.position_at_end(end_block)
    
    def _generate_for_stmt(self, stmt):
        """Gera código for statement (simulado como while)"""
        # Alocar variável do loop
        var_type = self._map_type(SerializableType.Inteiro)
        alloca = self.builder.alloca(var_type, name=stmt.variable)
        
        # Inicializar variável
        start_value = self._generate_expression(stmt.start)
        self.builder.store(start_value, alloca)
        
        # Adicionar à tabela de símbolos
        self.symbol_tables[-1][stmt.variable] = VariableInfo(
            ir_value=alloca,
            var_type=SerializableType.Inteiro
        )
        
        # Criar blocos
        cond_block = self.current_function.append_basic_block("for_cond")
        body_block = self.current_function.append_basic_block("for_body")
        end_block = self.current_function.append_basic_block("for_end")
        
        # Branch para condição
        self.builder.branch(cond_block)
        
        # Gerar condição
        self.builder.position_at_end(cond_block)
        current_value = self.builder.load(alloca, stmt.variable)
        end_value = self._generate_expression(stmt.end)
        cond_value = self.builder.icmp_signed('<=', current_value, end_value)
        self.builder.cbranch(cond_value, body_block, end_block)
        
        # Gerar corpo
        self.builder.position_at_end(body_block)
        self.symbol_tables.append({})
        self._generate_block(stmt.body)
        
        # Incrementar variável
        current_value = self.builder.load(alloca, stmt.variable)
        inc_value = self.builder.add(current_value, ir.Constant(ir.IntType(32), 1))
        self.builder.store(inc_value, alloca)
        
        if not self.builder.block.is_terminated:
            self.builder.branch(cond_block)  # Loop
        self.symbol_tables.pop()
        
        # Continuar no end block
        self.builder.position_at_end(end_block)
    
    def _generate_return_stmt(self, stmt):
        """Gera código para return statement"""
        if stmt.value:
            ret_value = self._generate_expression(stmt.value)
            self.builder.ret(ret_value)
        else:
            self.builder.ret_void()
    
    def _generate_write_stmt(self, stmt):
        """Gera código para escreva statement"""
        for arg in stmt.arguments:
            arg_value = self._generate_expression(arg)
            arg_type = arg.expr_type_annotation
            
            if arg_type == SerializableType.Inteiro:
                # printf("%d\n", value)
                format_str = self._create_global_string("%d\n")
                self.builder.call(self.printf, [format_str, arg_value])
            elif arg_type == SerializableType.Decimal:
                # printf("%f\n", value)
                format_str = self._create_global_string("%f\n")
                self.builder.call(self.printf, [format_str, arg_value])
            elif arg_type == SerializableType.Texto:
                # printf("%s\n", value)
                format_str = self._create_global_string("%s\n")
                self.builder.call(self.printf, [format_str, arg_value])
            elif arg_type == SerializableType.Logico:
                # Converter booleano para string
                true_str = self._create_global_string("verdadeiro\n")
                false_str = self._create_global_string("falso\n")
                
                cond = self.builder.icmp_unsigned('!=', arg_value, ir.Constant(ir.IntType(1), 0))
                self.builder.call(self.printf, [self.builder.select(cond, true_str, false_str)])
    
    def _generate_read_stmt(self, stmt):
        """Gera código para leia statement"""
        target = stmt.target
        
        if target.expr_type != "Variable":
            raise RuntimeError("leia só pode ser usado com variáveis")
        
        var_name = target.get_variable_name()
        var_info = self._lookup_variable(var_name)
        
        if var_info.var_type == SerializableType.Inteiro:
            # scanf("%d", &var)
            format_str = self._create_global_string("%d")
            self.builder.call(self.scanf, [format_str, var_info.ir_value])
        elif var_info.var_type == SerializableType.Decimal:
            # scanf("%lf", &var)
            format_str = self._create_global_string("%lf")
            self.builder.call(self.scanf, [format_str, var_info.ir_value])
        else:
            raise RuntimeError(f"Tipo não suportado para leia: {var_info.var_type}")
    
    def _generate_expression(self, expr: SerializableExpr) -> ir.Value:
        """Gera código para uma expressão e retorna o valor LLVM"""
        expr_type = expr.expr_type
        
        if expr_type == "Literal":
            return self._generate_literal(expr.get_literal_value())
        elif expr_type == "Variable":
            return self._generate_variable_access(expr.get_variable_name())
        elif expr_type == "Call":
            return self._generate_call(expr.get_call_expr())
        elif expr_type == "BinaryOp":
            op, left, right = expr.get_binary_op()
            return self._generate_binary_op(op, left, right)
        elif expr_type == "UnaryOp":
            op, operand = expr.get_unary_op()
            return self._generate_unary_op(op, operand)
        else:
            raise RuntimeError(f"Tipo de expressão não suportado: {expr_type}")
    
    def _generate_literal(self, literal: SerializableLiteral) -> ir.Value:
        """Gera código para literal"""
        if literal.literal_type == "Inteiro":
            return ir.Constant(ir.IntType(32), literal.value)
        elif literal.literal_type == "Decimal":
            return ir.Constant(ir.DoubleType(), literal.value)
        elif literal.literal_type == "Texto":
            # Criar string global
            return self._create_global_string(literal.value)
        elif literal.literal_type == "Logico":
            return ir.Constant(ir.IntType(1), 1 if literal.value else 0)
        else:
            raise RuntimeError(f"Tipo de literal não suportado: {literal.literal_type}")
    
    def _generate_variable_access(self, var_name: str) -> ir.Value:
        """Gera código para acesso a variável"""
        var_info = self._lookup_variable(var_name)
        return self.builder.load(var_info.ir_value, var_name)
    
    def _generate_call(self, call: SerializableCallExpr) -> ir.Value:
        """Gera código para chamada de função"""
        # Buscar função
        if call.function not in self.symbol_tables[0]:
            raise RuntimeError(f"Função não encontrada: {call.function}")
        
        func_info = self.symbol_tables[0][call.function]
        ir_function = func_info.ir_value
        
        # Gerar argumentos
        args = [self._generate_expression(arg) for arg in call.arguments]
        
        # Chamar função
        return self.builder.call(ir_function, args)
    
    def _generate_binary_op(self, op: SerializableBinaryOperator, 
                          left: SerializableExpr, right: SerializableExpr) -> ir.Value:
        """Gera código para operação binária"""
        left_val = self._generate_expression(left)
        right_val = self._generate_expression(right)
        
        left_type = left.expr_type_annotation
        right_type = right.expr_type_annotation
        
        # Operações aritméticas
        if op in [SerializableBinaryOperator.Add, SerializableBinaryOperator.Subtract,
                 SerializableBinaryOperator.Multiply, SerializableBinaryOperator.Divide]:
            
            if left_type in [SerializableType.Inteiro, SerializableType.Logico]:
                if op == SerializableBinaryOperator.Add:
                    return self.builder.add(left_val, right_val)
                elif op == SerializableBinaryOperator.Subtract:
                    return self.builder.sub(left_val, right_val)
                elif op == SerializableBinaryOperator.Multiply:
                    return self.builder.mul(left_val, right_val)
                elif op == SerializableBinaryOperator.Divide:
                    return self.builder.sdiv(left_val, right_val)
            
            elif left_type == SerializableType.Decimal:
                if op == SerializableBinaryOperator.Add:
                    return self.builder.fadd(left_val, right_val)
                elif op == SerializableBinaryOperator.Subtract:
                    return self.builder.fsub(left_val, right_val)
                elif op == SerializableBinaryOperator.Multiply:
                    return self.builder.fmul(left_val, right_val)
                elif op == SerializableBinaryOperator.Divide:
                    return self.builder.fdiv(left_val, right_val)
        
        # Operações de comparação
        elif op in [SerializableBinaryOperator.Equal, SerializableBinaryOperator.NotEqual,
                   SerializableBinaryOperator.Less, SerializableBinaryOperator.LessEqual,
                   SerializableBinaryOperator.Greater, SerializableBinaryOperator.GreaterEqual]:
            
            if left_type in [SerializableType.Inteiro, SerializableType.Logico]:
                cmp_op = {
                    SerializableBinaryOperator.Equal: '==',
                    SerializableBinaryOperator.NotEqual: '!=',
                    SerializableBinaryOperator.Less: '<',
                    SerializableBinaryOperator.LessEqual: '<=',
                    SerializableBinaryOperator.Greater: '>',
                    SerializableBinaryOperator.GreaterEqual: '>=',
                }[op]
                return self.builder.icmp_signed(cmp_op, left_val, right_val)
            
            elif left_type == SerializableType.Decimal:
                cmp_op = {
                    SerializableBinaryOperator.Equal: '==',
                    SerializableBinaryOperator.NotEqual: '!=',
                    SerializableBinaryOperator.Less: '<',
                    SerializableBinaryOperator.LessEqual: '<=',
                    SerializableBinaryOperator.Greater: '>',
                    SerializableBinaryOperator.GreaterEqual: '>=',
                }[op]
                return self.builder.fcmp_ordered(cmp_op, left_val, right_val)
        
        # Operações lógicas
        elif op in [SerializableBinaryOperator.And, SerializableBinaryOperator.Or]:
            if op == SerializableBinaryOperator.And:
                return self.builder.and_(left_val, right_val)
            else:  # Or
                return self.builder.or_(left_val, right_val)
        
        else:
            raise RuntimeError(f"Operador binário não suportado: {op}")
    
    def _generate_unary_op(self, op: SerializableUnaryOperator, 
                          operand: SerializableExpr) -> ir.Value:
        """Gera código para operação unária"""
        operand_val = self._generate_expression(operand)
        operand_type = operand.expr_type_annotation
        
        if op == SerializableUnaryOperator.Negate:
            if operand_type == SerializableType.Inteiro:
                return self.builder.neg(operand_val)
            elif operand_type == SerializableType.Decimal:
                return self.builder.fneg(operand_val)
        elif op == SerializableUnaryOperator.Not:
            if operand_type == SerializableType.Logico:
                return self.builder.not_(operand_val)
        
        raise RuntimeError(f"Operador unário {op} não suportado para tipo {operand_type}")
    
    def _generate_global_statements(self, statements: List[SerializableStatement]):
        """Gera código para statements globais (em função main implícita)"""

        main_type = ir.FunctionType(ir.IntType(32), [])
        main_func = ir.Function(self.module, main_type, name="main")
        
        # Gerar corpo da main
        entry_block = main_func.append_basic_block(name="entry")
        builder = ir.IRBuilder(entry_block)
        
        old_builder = self.builder
        self.builder = builder
        self.current_function = main_func
        
        self.symbol_tables.append({})
        
        for statement in statements:
            self._generate_statement(statement)
        
        # Retornar 0 (sucesso)
        if not self.builder.block.is_terminated:
            self.builder.ret(ir.Constant(ir.IntType(32), 0))
        
        self.symbol_tables.pop()
        self.builder = old_builder
        self.current_function = None
    
    def _lookup_variable(self, name: str) -> VariableInfo:
        """Busca variável na tabela de símbolos (do escopo atual para os pais)"""
        for scope in reversed(self.symbol_tables):
            if name in scope:
                return scope[name]
        raise RuntimeError(f"Variável não encontrada: {name}")
    
    def _map_type(self, symplia_type: SerializableType) -> ir.Type:
        """Mapeia tipo Symplia para tipo LLVM"""
        type_map = {
            SerializableType.Inteiro: ir.IntType(32),
            SerializableType.Decimal: ir.DoubleType(),
            SerializableType.Texto: ir.PointerType(ir.IntType(8)),
            SerializableType.Logico: ir.IntType(1),
            SerializableType.Void: ir.VoidType(),
        }
        return type_map[symplia_type]
    
    def _get_default_value(self, symplia_type: SerializableType) -> ir.Constant:
        """Retorna valor padrão para um tipo"""
        type_map = {
            SerializableType.Inteiro: ir.Constant(ir.IntType(32), 0),
            SerializableType.Decimal: ir.Constant(ir.DoubleType(), 0.0),
            SerializableType.Texto: ir.Constant(ir.PointerType(ir.IntType(8)), None),
            SerializableType.Logico: ir.Constant(ir.IntType(1), 0),
        }
        return type_map.get(symplia_type, ir.Constant(ir.IntType(32), 0))
    
    def _create_global_string(self, text: str) -> ir.Value:
        """Cria uma string global constante"""

        text_bytes = text.encode('utf-8') + b'\x00'
        
        global_str = ir.GlobalVariable(self.module, 
                                     ir.ArrayType(ir.IntType(8), len(text_bytes)),
                                     name=".str")
        global_str.linkage = 'internal'
        global_str.global_constant = True
        global_str.initializer = ir.Constant(ir.ArrayType(ir.IntType(8), len(text_bytes)),
                                         [ir.Constant(ir.IntType(8), b) for b in text_bytes])
        
        zero = ir.Constant(ir.IntType(32), 0)
        return self.builder.gep(global_str, [zero, zero], inbounds=True)

def generate_llvm_ir(program: SerializableProgram, module_name: str = "symplia_module") -> str:
    """Função utilitária para gerar LLVM IR rapidamente"""
    builder = LLVMBuilder(module_name)
    return builder.generate_ir(program)