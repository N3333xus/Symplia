"""
Classes AST Node para codegen Symplia em Python 
Espelha as estruturas serializáveis de Rust
"""

from dataclasses import dataclass, field
from typing import List, Optional, Union, Dict, Any
import json
from pathlib import Path
from enum import Enum

class SerializableType(Enum):
    Inteiro = "Inteiro"
    Decimal = "Decimal" 
    Texto = "Texto"
    Logico = "Logico"
    Void = "Void"

class SerializableBinaryOperator(Enum):
    Add = "Add"
    Subtract = "Subtract"
    Multiply = "Multiply"
    Divide = "Divide"
    Modulo = "Modulo"
    Equal = "Equal"
    NotEqual = "NotEqual"
    Less = "Less"
    LessEqual = "LessEqual"
    Greater = "Greater"
    GreaterEqual = "GreaterEqual"
    And = "And"
    Or = "Or"

class SerializableUnaryOperator(Enum):
    Negate = "Negate"
    Plus = "Plus"
    Not = "Not"

@dataclass
class ProgramMetadata:
    source_file: str
    timestamp: str
    version: str
    entry_point: Optional[str] = None

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'ProgramMetadata':
        return cls(
            source_file=data.get("source_file", "unknown"),
            timestamp=data.get("timestamp", ""),
            version=data.get("version", "0.1.0"),
            entry_point=data.get("entry_point")
        )

@dataclass
class SerializableLiteral:
    value: Union[int, float, str, bool]
    literal_type: str  # "Inteiro", "Decimal", "Texto", "Logico"

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableLiteral':
        if "Inteiro" in data:
            return cls(value=data["Inteiro"], literal_type="Inteiro")
        elif "Decimal" in data:
            return cls(value=data["Decimal"], literal_type="Decimal")
        elif "Texto" in data:
            return cls(value=data["Texto"], literal_type="Texto")
        elif "Logico" in data:
            return cls(value=data["Logico"], literal_type="Logico")
        else:
            raise ValueError(f"Invalid literal data: {data}")

@dataclass
class SerializableCallExpr:
    function: str
    arguments: List['SerializableExpr']

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableCallExpr':
        return cls(
            function=data["function"],
            arguments=[SerializableExpr.from_dict(arg) for arg in data["arguments"]]
        )

@dataclass
class SerializableExpr:

    expr_type: str  # "Literal", "Variable", "Call", "BinaryOp", "UnaryOp"
    data: Dict[str, Any]
    expr_type_annotation: SerializableType

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableExpr':
        # Extrair anotação de tipo
        type_annotation = SerializableType(data.get("expr_type", "Inteiro"))
        
        return cls(
            expr_type=data["type"],
            data=data,
            expr_type_annotation=type_annotation
        )

    def get_literal_value(self) -> SerializableLiteral:
        if self.expr_type != "Literal":
            raise ValueError(f"Not a literal expression: {self.expr_type}")
        return SerializableLiteral.from_dict(self.data["value"])

    def get_variable_name(self) -> str:
        if self.expr_type != "Variable":
            raise ValueError(f"Not a variable expression: {self.expr_type}")
        return self.data["name"]

    def get_call_expr(self) -> SerializableCallExpr:
        if self.expr_type != "Call":
            raise ValueError(f"Not a call expression: {self.expr_type}")
        return SerializableCallExpr.from_dict(self.data["call"])

    def get_binary_op(self) -> tuple:
        if self.expr_type != "BinaryOp":
            raise ValueError(f"Not a binary operation: {self.expr_type}")
        return (
            SerializableBinaryOperator(self.data["op"]),
            SerializableExpr.from_dict(self.data["left"]),
            SerializableExpr.from_dict(self.data["right"])
        )

    def get_unary_op(self) -> tuple:
        if self.expr_type != "UnaryOp":
            raise ValueError(f"Not a unary operation: {self.expr_type}")
        return (
            SerializableUnaryOperator(self.data["op"]),
            SerializableExpr.from_dict(self.data["operand"])
        )

@dataclass
class SerializableParameter:
    param_type: SerializableType
    name: str

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableParameter':
        return cls(
            param_type=SerializableType(data["param_type"]),
            name=data["name"]
        )

@dataclass
class SerializableBlock:
    statements: List['SerializableStatement']

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableBlock':
        return cls(
            statements=[SerializableStatement.from_dict(stmt) for stmt in data["statements"]]
        )

@dataclass
class SerializableVariableDecl:
    var_type: SerializableType
    name: str
    initializer: Optional[SerializableExpr]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableVariableDecl':
        initializer = None
        if data.get("initializer"):
            initializer = SerializableExpr.from_dict(data["initializer"])
        
        return cls(
            var_type=SerializableType(data["var_type"]),
            name=data["name"],
            initializer=initializer
        )

@dataclass
class SerializableExprStmt:
    expr: SerializableExpr

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableExprStmt':
        return cls(
            expr=SerializableExpr.from_dict(data["expr"])
        )

@dataclass
class SerializableIfStmt:
    condition: SerializableExpr
    then_branch: SerializableBlock
    else_branch: Optional[SerializableBlock]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableIfStmt':
        else_branch = None
        if data.get("else_branch"):
            else_branch = SerializableBlock.from_dict(data["else_branch"])
        
        return cls(
            condition=SerializableExpr.from_dict(data["condition"]),
            then_branch=SerializableBlock.from_dict(data["then_branch"]),
            else_branch=else_branch
        )

@dataclass
class SerializableWhileStmt:
    condition: SerializableExpr
    body: SerializableBlock

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableWhileStmt':
        return cls(
            condition=SerializableExpr.from_dict(data["condition"]),
            body=SerializableBlock.from_dict(data["body"])
        )

@dataclass
class SerializableForStmt:
    variable: str
    start: SerializableExpr
    end: SerializableExpr
    body: SerializableBlock

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableForStmt':
        return cls(
            variable=data["variable"],
            start=SerializableExpr.from_dict(data["start"]),
            end=SerializableExpr.from_dict(data["end"]),
            body=SerializableBlock.from_dict(data["body"])
        )

@dataclass
class SerializableReturnStmt:
    value: Optional[SerializableExpr]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableReturnStmt':
        value = None
        if data.get("value"):
            value = SerializableExpr.from_dict(data["value"])
        
        return cls(value=value)

@dataclass
class SerializableWriteStmt:
    arguments: List[SerializableExpr]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableWriteStmt':
        return cls(
            arguments=[SerializableExpr.from_dict(arg) for arg in data["arguments"]]
        )

@dataclass
class SerializableReadStmt:
    target: SerializableExpr

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableReadStmt':
        return cls(
            target=SerializableExpr.from_dict(data["target"])
        )

@dataclass
class SerializableStatement:
    statement_type: str  # "VariableDecl", "ExprStmt", "IfStmt", etc.
    data: Dict[str, Any]

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableStatement':
        return cls(
            statement_type=data["type"],
            data=data
        )

    def get_variable_decl(self) -> SerializableVariableDecl:
        if self.statement_type != "VariableDecl":
            raise ValueError(f"Not a variable declaration: {self.statement_type}")
        return SerializableVariableDecl.from_dict(self.data)

    def get_expr_stmt(self) -> SerializableExprStmt:
        if self.statement_type != "ExprStmt":
            raise ValueError(f"Not an expression statement: {self.statement_type}")
        return SerializableExprStmt.from_dict(self.data)

    def get_if_stmt(self) -> SerializableIfStmt:
        if self.statement_type != "IfStmt":
            raise ValueError(f"Not an if statement: {self.statement_type}")
        return SerializableIfStmt.from_dict(self.data)

    def get_while_stmt(self) -> SerializableWhileStmt:
        if self.statement_type != "WhileStmt":
            raise ValueError(f"Not a while statement: {self.statement_type}")
        return SerializableWhileStmt.from_dict(self.data)

    def get_for_stmt(self) -> SerializableForStmt:
        if self.statement_type != "ForStmt":
            raise ValueError(f"Not a for statement: {self.statement_type}")
        return SerializableForStmt.from_dict(self.data)

    def get_return_stmt(self) -> SerializableReturnStmt:
        if self.statement_type != "ReturnStmt":
            raise ValueError(f"Not a return statement: {self.statement_type}")
        return SerializableReturnStmt.from_dict(self.data)

    def get_write_stmt(self) -> SerializableWriteStmt:
        if self.statement_type != "WriteStmt":
            raise ValueError(f"Not a write statement: {self.statement_type}")
        return SerializableWriteStmt.from_dict(self.data)

    def get_read_stmt(self) -> SerializableReadStmt:
        if self.statement_type != "ReadStmt":
            raise ValueError(f"Not a read statement: {self.statement_type}")
        return SerializableReadStmt.from_dict(self.data)

@dataclass
class SerializableFunction:
    name: str
    return_type: Optional[SerializableType]
    parameters: List[SerializableParameter]
    body: SerializableBlock

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableFunction':
        return_type = None
        if data.get("return_type"):
            return_type = SerializableType(data["return_type"])
        
        return cls(
            name=data["name"],
            return_type=return_type,
            parameters=[SerializableParameter.from_dict(param) for param in data["parameters"]],
            body=SerializableBlock.from_dict(data["body"])
        )

# ==================== PROGRAMA PRINCIPAL ====================

@dataclass
class SerializableProgram:
    functions: List[SerializableFunction]
    global_statements: List[SerializableStatement]
    metadata: ProgramMetadata

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> 'SerializableProgram':
        return cls(
            functions=[SerializableFunction.from_dict(func) for func in data["functions"]],
            global_statements=[SerializableStatement.from_dict(stmt) for stmt in data["global_statements"]],
            metadata=ProgramMetadata.from_dict(data["metadata"])
        )

# ==================== DESSERIALIZADOR PRINCIPAL ====================

class ASTDeserializer:
    """Desserializa JSON da AST Rust para objetos Python"""
    
    @staticmethod
    def from_json_string(json_str: str) -> SerializableProgram:
        """Desserializa a partir de uma string JSON"""
        data = json.loads(json_str)
        return ASTDeserializer._deserialize_program(data)
    
    @staticmethod
    def from_json_file(filename: str) -> SerializableProgram:
        """Desserializa a partir de um arquivo JSON"""
        file_path = Path(filename)
        if not file_path.exists():
            raise FileNotFoundError(f"Arquivo JSON não encontrado: {filename}")
            
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
        return ASTDeserializer._deserialize_program(data)
    
    @staticmethod
    def _deserialize_program(data: Dict[str, Any]) -> SerializableProgram:
        """Desserializa o programa principal"""
        return SerializableProgram.from_dict(data)

# ==================== UTILITÁRIO ====================

def print_ast_summary(program: SerializableProgram) -> None:
    """Imprime um resumo da AST para debugging"""
    print(f"=== RESUMO DA AST ===")
    print(f"Arquivo: {program.metadata.source_file}")
    print(f"Funções: {len(program.functions)}")
    print(f"Statements globais: {len(program.global_statements)}")
    
    for i, func in enumerate(program.functions):
        print(f"  Função {i+1}: {func.name}")
        print(f"    Retorno: {func.return_type}")
        print(f"    Parâmetros: {len(func.parameters)}")
        print(f"    Statements no corpo: {len(func.body.statements)}")
    
    print("======================")


''' 
# TESTE P DEBUGGING

if __name__ == "__main__":
    sample_json = {
        "functions": [
            {
                "name": "main",
                "return_type": "Inteiro",
                "parameters": [],
                "body": {
                    "statements": [
                        {
                            "type": "VariableDecl",
                            "var_type": "Inteiro",
                            "name": "x",
                            "initializer": {
                                "type": "Literal",
                                "value": {"Inteiro": 42},
                                "expr_type": "Inteiro"
                            }
                        }
                    ]
                }
            }
        ],
        "global_statements": [],
        "metadata": {
            "source_file": "test.sym",
            "timestamp": "2024-01-01T00:00:00Z",
            "version": "0.1.0",
            "entry_point": "main"
        }
    }
    
    # Testar desserialização
    json_str = json.dumps(sample_json)
    program = ASTDeserializer.from_json_string(json_str)
    print_ast_summary(program)
    
    # Testar acesso aos dados
    main_func = program.functions[0]
    first_stmt = main_func.body.statements[0]
    var_decl = first_stmt.get_variable_decl()
    
    print(f"Variável declarada: {var_decl.name} do tipo {var_decl.var_type}")
    if var_decl.initializer:
        literal = var_decl.initializer.get_literal_value()
        print(f"Valor inicial: {literal.value}")

'''