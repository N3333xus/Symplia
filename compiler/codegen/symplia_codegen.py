#!/usr/bin/env python3
"""
Ponto de entrada principal para o codegen Symplia
Gera LLVM IR a partir de arquivos JSON da AST
"""

import sys
import os
import argparse
import json
import traceback
from pathlib import Path
from typing import Optional

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from ast_nodes import ASTDeserializer, SerializableProgram, print_ast_summary
from llvm_builder import LLVMBuilder, generate_llvm_ir

class SympliaCodeGen:
    
    def __init__(self, verbose: bool = False, optimize: bool = False):
        self.verbose = verbose
        self.optimize = optimize
        self.errors = []
        self.warnings = []
    
    def log(self, message: str):
        if self.verbose:
            print(f"[INFO] {message}")
    
    def error(self, message: str):
        self.errors.append(message)
        print(f"❌ ERRO: {message}", file=sys.stderr)
    
    def warning(self, message: str):
        self.warnings.append(message)
        print(f"⚠️  AVISO: {message}", file=sys.stderr)
    
    def generate_from_json_file(self, json_file: str, output_file: Optional[str] = None) -> bool:
        """Gera LLVM IR a partir de um arquivo JSON da AST"""
        try:
            self.log(f"Lendo arquivo JSON: {json_file}")
            
            if not os.path.exists(json_file):
                self.error(f"Arquivo não encontrado: {json_file}")
                return False
            
            self.log("Desserializando AST...")
            program = ASTDeserializer.from_json_file(json_file)
            
            if self.verbose:
                print_ast_summary(program)
            
            if not self._validate_program(program):
                return False
            
            self.log("Gerando código LLVM IR...")
            llvm_ir = generate_llvm_ir(program)
            
            if self.optimize:
                self.log("Aplicando otimizações...")
                llvm_ir = self._apply_optimizations(llvm_ir)
            
            if output_file is None:
                output_file = json_file.replace('.ast.json', '.ll')
                if output_file == json_file:  # Se não tinha .ast.json
                    output_file = json_file + '.ll'
            
            # Salvar arquivo
            self.log(f"Salvando LLVM IR em: {output_file}")
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(llvm_ir)
            
            # Mostrar estatísticas
            self._print_statistics(program, llvm_ir, output_file)
            
            return True
            
        except Exception as e:
            self.error(f"Erro durante a geração de código: {e}")
            if self.verbose:
                traceback.print_exc()
            return False
    
    def generate_from_json_string(self, json_str: str, module_name: str = "symplia_module") -> Optional[str]:
        """Gera LLVM IR a partir de uma string JSON (para testes)"""
        try:
            self.log("Desserializando AST from string...")
            program = ASTDeserializer.from_json_string(json_str)
            
            # Validar AST
            if not self._validate_program(program):
                return None
            
            # Gerar LLVM IR
            self.log("Gerando código LLVM IR...")
            llvm_ir = generate_llvm_ir(program, module_name)
            
            return llvm_ir
            
        except Exception as e:
            self.error(f"Erro durante a geração de código: {e}")
            if self.verbose:
                traceback.print_exc()
            return None
    
    def _validate_program(self, program: SerializableProgram) -> bool:
        """Valida o programa antes da geração de código"""
        self.log("Validando programa...")
        
        if not program.functions and not program.global_statements:
            self.warning("Programa vazio - nenhuma função ou statement global")
        
        # Verificar se há função main para statements globais
        if program.global_statements:
            has_main = any(func.name == "main" for func in program.functions)
            if not has_main:
                self.log("Criando função main implícita para statements globais")
        
        # Validar cada função
        for func in program.functions:
            if not self._validate_function(func):
                return False
        
        return True
    
    def _validate_function(self, function) -> bool:
        """Valida uma função individual"""
        if not function.name:
            self.error("Função sem nome")
            return False
        
        # Verificar se há return em funções não-void
        if function.return_type and function.return_type != "Void":
            # Esta é uma validação básica  poderia ser mais sofisticada
            # analisando a AST para verificar se todos os caminhos retornam valor
            pass
        
        return True
    
    def _apply_optimizations(self, llvm_ir: str) -> str:
        """Aplica otimizações básicas ao LLVM IR"""
        # Esta é uma implementação simplificada
        # Em uma versão mais avançada, usaríamos as passes de otimização do LLVM
        
        # Otimizações simples em nível de texto (para demonstração)
        optimized_ir = llvm_ir
        
        # Remover instruções não utilizadas (pattern matching simples)
        lines = optimized_ir.split('\n')
        optimized_lines = []
        
        i = 0
        while i < len(lines):
            line = lines[i].strip()
            
            # Pular alocações não utilizadas (análise muito básica)
            if line.startswith('%') and ' = alloca ' in line and i + 1 < len(lines):
                next_line = lines[i + 1].strip()
                if not next_line.startswith('store ') and not next_line.startswith('call '):
                    # Provavelmente não utilizado - pular
                    i += 1
                    continue
            
            optimized_lines.append(lines[i])
            i += 1
        
        optimized_ir = '\n'.join(optimized_lines)
        
        self.log(f"Aplicadas otimizações básicas")
        return optimized_ir
    
    def _print_statistics(self, program: SerializableProgram, llvm_ir: str, output_file: str):
        """Imprime estatísticas da geração de código"""
        num_functions = len(program.functions)
        num_global_stmts = len(program.global_statements)
        num_llvm_lines = llvm_ir.count('\n') + 1
        
        print("✅ Geração de código concluída com sucesso!")
        print(f"📊 Estatísticas:")
        print(f"   • Funções: {num_functions}")
        print(f"   • Statements globais: {num_global_stmts}")
        print(f"   • Linhas de LLVM IR: {num_llvm_lines}")
        print(f"   • Arquivo gerado: {output_file}")
        
        if self.warnings:
            print(f"   • Avisos: {len(self.warnings)}")
        
        # Mostrar preview do IR
        if self.verbose:
            print(f"\n📄 Preview do LLVM IR:")
            lines = llvm_ir.split('\n')[:20] 
            for line in lines:
                print(f"   {line}")
            if len(llvm_ir.split('\n')) > 20:
                print("   ...")

def main():
    """Função principal - ponto de entrada da linha de comando"""
    parser = argparse.ArgumentParser(
        description='Symplia CodeGen - Gera LLVM IR a partir da AST JSON',
        epilog='Exemplo: python symplia_codegen.py programa.ast.json'
    )
    
    parser.add_argument(
        'input_file',
        help='Arquivo JSON da AST (.ast.json)'
    )
    
    parser.add_argument(
        '-o', '--output',
        dest='output_file',
        help='Arquivo de saída LLVM IR (.ll)'
    )
    
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Modo verboso - mostra informações detalhadas'
    )
    
    parser.add_argument(
        '--optimize',
        action='store_true',
        help='Aplica otimizações básicas ao código gerado'
    )
    
    parser.add_argument(
        '--version',
        action='version',
        version='Symplia CodeGen 0.1.0'
    )
    
    args = parser.parse_args()
    
    # Verificar extensão do arquivo
    if not args.input_file.endswith('.ast.json'):
        print("⚠️  Aviso: Arquivo de entrada não tem extensão .ast.json")
    
    # Executar codegen
    codegen = SympliaCodeGen(verbose=args.verbose, optimize=args.optimize)
    success = codegen.generate_from_json_file(args.input_file, args.output_file)
    
    # Retornar código de saída apropriado
    sys.exit(0 if success else 1)

def interactive_mode():
    """Modo interativo para testes rápidos"""
    print("=== Symplia CodeGen - Modo Interativo ===")
    print("Digite 'quit' para sair")
    print("Cole o JSON da AST abaixo:\n")
    
    codegen = SympliaCodeGen(verbose=True)
    
    while True:
        try:
            print("> ", end='')
            line = input().strip()
            
            if line.lower() in ['quit', 'exit', 'q']:
                break
            
            if not line:
                continue
            
            # Tentar processar como JSON
            llvm_ir = codegen.generate_from_json_string(line)
            if llvm_ir:
                print("\n✅ LLVM IR Gerado:")
                print(llvm_ir)
                print()
            else:
                print("❌ Falha na geração de código\n")
                
        except EOFError:
            break
        except KeyboardInterrupt:
            print("\nSaindo...")
            break
        except Exception as e:
            print(f"❌ Erro: {e}\n")

if __name__ == "__main__":

    if len(sys.argv) == 1:
        # Modo interativo
        interactive_mode()
    else:
        # Modo linha de comando
        main()