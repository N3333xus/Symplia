#!/usr/bin/env python3
"""
Script de integração completo do compilador Symplia
Orquestra todo o processo: Rust (frontend) → Python (codegen) → Executável
"""

import os
import sys
import subprocess
import argparse
import tempfile
import shutil
from pathlib import Path
from typing import Optional, List

class SympliaCompiler:
    
    def __init__(self, verbose: bool = False, keep_temp: bool = False):
        self.verbose = verbose
        self.keep_temp = keep_temp
        self.temp_dir = None
        self.errors = []
        
        self.project_root = Path(__file__).parent
        self.compiler_dir = self.project_root / "compiler"      # Código Rust
        self.codegen_dir = self.compiler_dir / "codegen"        # Código Python
        self.programas_dir = self.project_root / "programas"    # Programas usuário
        
        self._validate_project_structure()
    
    def _validate_project_structure(self):
        """Valida se a estrutura do projeto está correta - NOVA ESTRUTURA"""
        required_dirs = [
            self.compiler_dir,
            self.codegen_dir
        ]
        
        required_files = [
            self.compiler_dir / "Cargo.toml",
            self.compiler_dir / "src" / "main.rs",
            self.codegen_dir / "symplia_codegen.py",
            self.codegen_dir / "ast_nodes.py", 
            self.codegen_dir / "llvm_builder.py"
        ]
        
        # Verificar diretórios
        for dir_path in required_dirs:
            if not dir_path.exists():
                raise RuntimeError(f"Diretório não encontrado: {dir_path}")
        
        # Verificar arquivos
        for file_path in required_files:
            if not file_path.exists():
                raise RuntimeError(f"Arquivo não encontrado: {file_path}")
        
        # Verificar se programas/ existe (opcional, mas recomendado)
        if not self.programas_dir.exists():
            self.log(f"⚠️  Diretório 'programas/' não encontrado. Crie para organizar seus arquivos .sym")
        
        self.log("✅ Estrutura do projeto validada")
    
    def log(self, message: str):
        """Log de mensagens no modo verbose"""
        if self.verbose:
            print(f"[SYMPLIA] {message}")
    
    def error(self, message: str):
        """Registra e exibe um erro"""
        self.errors.append(message)
        print(f"❌ ERRO: {message}", file=sys.stderr)
    
    def compile(self, source_file: str, output_file: Optional[str] = None, 
                optimize: bool = False, target: str = "executable") -> bool:
        """Compila um arquivo .sym para o target especificado"""
        try:
            source_path = Path(source_file)

            if not source_path.exists():
                self.error(f"Arquivo fonte não encontrado: {source_file}")
                return False
            
            if source_path.suffix != '.sym':
                self.error(f"Arquivo deve ter extensão .sym: {source_file}")
                return False
            
            if not self.keep_temp:
                self.temp_dir = tempfile.mkdtemp(prefix="symplia_")
                self.log(f"Diretório temporário criado: {self.temp_dir}")
            else:
                self.temp_dir = source_path.parent
            
            base_name = source_path.stem
            if output_file is None:
                output_file = base_name
            
            json_file = Path(self.temp_dir) / f"{base_name}.ast.json"
            llvm_file = Path(self.temp_dir) / f"{base_name}.ll"
            
            self.log(f"Iniciando compilação de: {source_file}")
            self.log(f"Arquivos temporários: {json_file}, {llvm_file}")
            
            # PASSO 1: Compilar com Rust (frontend)
            self.log("=== FASE 1: Frontend Rust (Análise) ===")
            if not self._run_rust_compiler(source_file, json_file):
                return False
            
            # PASSO 2: Gerar LLVM IR com Python (codegen)
            self.log("=== FASE 2: Backend Python (Geração de Código) ===")
            if not self._run_python_codegen(json_file, llvm_file, optimize):
                return False
            
            # PASSO 3: Compilar para target final
            self.log("=== FASE 3: Geração do Target Final ===")
            if target == "executable":
                return self._generate_executable(llvm_file, output_file)
            elif target == "llvm-ir":
                # Para target LLVM IR, apenas copiar o arquivo .ll
                final_ll_file = Path(output_file).with_suffix('.ll')
                shutil.copy2(llvm_file, final_ll_file)
                self.log(f"✅ Arquivo LLVM IR salvo: {final_ll_file}")
                return True
            elif target == "assembly":
                return self._generate_assembly(llvm_file, output_file)
            else:
                self.error(f"Target não suportado: {target}")
                return False
            
        except Exception as e:
            self.error(f"Erro durante a compilação: {e}")
            if self.verbose:
                import traceback
                traceback.print_exc()
            return False
        
        finally:
            # Limpar arquivos temporários
            if not self.keep_temp and self.temp_dir and Path(self.temp_dir).exists():
                self.log(f"Limpando diretório temporário: {self.temp_dir}")
                shutil.rmtree(self.temp_dir)
    
    def _run_rust_compiler(self, source_file: str, json_output: Path) -> bool:
        """Executa o compilador Rust para análise do código fonte"""
        try:
            self.log("Executando compilador Rust...")
            
            # Construir comando
            cmd = [
                "cargo", "run", "--release", "--quiet", "--",
                source_file
            ]
            
            # Executar em modo release para melhor performance
            env = os.environ.copy()
            
            self.log(f"Comando: {' '.join(cmd)}")
            self.log(f"Diretório: {self.compiler_dir}")
            
            # Executar compilador Rust
            result = subprocess.run(
                cmd,
                cwd=self.compiler_dir,
                capture_output=True,
                text=True,
                env=env
            )
            
            # Verificar se o Rust gerou o arquivo JSON
            expected_json = Path(source_file).with_suffix('.ast.json')
            if expected_json.exists():
                shutil.move(expected_json, json_output)
                self.log(f"✅ Análise Rust concluída: {json_output}")
                return True
            else:
                # Tentar obter mensagens de erro
                if result.stderr:
                    self.error(f"Rust compiler stderr: {result.stderr}")
                if result.stdout:
                    self.log(f"Rust compiler stdout: {result.stdout}")
                
                self.error("Compilador Rust não gerou arquivo JSON esperado")
                return False
                
        except subprocess.CalledProcessError as e:
            self.error(f"Erro na execução do compilador Rust: {e}")
            if e.stderr:
                self.error(f"Stderr: {e.stderr.decode()}")
            return False
        except FileNotFoundError:
            self.error("Compilador Rust não encontrado. Certifique-se de que o Rust está instalado.")
            return False
        except Exception as e:
            self.error(f"Erro inesperado no compilador Rust: {e}")
            return False
    
    def _run_python_codegen(self, json_file: Path, llvm_output: Path, optimize: bool) -> bool:
        """Executa o codegen Python para gerar LLVM IR"""
        try:
            self.log("Executando codegen Python...")
            
            # Construir comando - IMPORTANTE: agora codegen está em compiler/codegen/
            cmd = [
                sys.executable, "-m", "codegen.symplia_codegen",
                str(json_file),
                "-o", str(llvm_output)
            ]
            
            if optimize:
                cmd.append("--optimize")
            if self.verbose:
                cmd.append("-v")
            
            self.log(f"Comando: {' '.join(cmd)}")
            self.log(f"Diretório de trabalho: {self.compiler_dir}")
            
            # Executar codegen Python a partir do diretório compiler/
            result = subprocess.run(
                cmd,
                cwd=self.compiler_dir,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                if result.stderr:
                    self.error(f"Python codegen stderr: {result.stderr}")
                if result.stdout:
                    self.log(f"Python codegen stdout: {result.stdout}")
                self.error("Codegen Python falhou")
                return False
            
            if not llvm_output.exists():
                self.error(f"Codegen Python não gerou arquivo: {llvm_output}")
                return False
            
            self.log(f"✅ Codegen Python concluído: {llvm_output}")
            return True
            
        except subprocess.CalledProcessError as e:
            self.error(f"Erro na execução do codegen Python: {e}")
            return False
        except Exception as e:
            self.error(f"Erro inesperado no codegen Python: {e}")
            return False
    
    def _generate_executable(self, llvm_file: Path, output_file: str) -> bool:
        """Compila LLVM IR para executável usando clang"""
        try:
            self.log("Compilando LLVM IR para executável...")
            
            output_path = Path(output_file).with_suffix('')
            if os.name == 'nt':  # Windows
                output_path = output_path.with_suffix('.exe')
            
            cmd = [
                "clang", 
                "-O2",  # Otimização
                str(llvm_file),
                "-o", str(output_path)
            ]
            
            self.log(f"Comando: {' '.join(cmd)}")
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                if result.stderr:
                    self.error(f"clang stderr: {result.stderr}")
                self.error("Falha ao compilar para executável")
                return False
            
            # Verificar se o executável foi criado
            if output_path.exists():
                self.log(f"✅ Executável gerado: {output_path}")
                print(f"🎉 Compilação concluída! Executável: {output_path}")
                return True
            else:
                self.error(f"Executável não foi criado: {output_path}")
                return False
                
        except FileNotFoundError:
            self.error("clang não encontrado. Instale o LLVM/clang para gerar executáveis.")
            self.error("Como alternativa, use --target llvm-ir para gerar apenas o LLVM IR.")
            return False
        except Exception as e:
            self.error(f"Erro ao gerar executável: {e}")
            return False
    
    def _generate_assembly(self, llvm_file: Path, output_file: str) -> bool:
        """Compila LLVM IR para assembly"""
        try:
            self.log("Compilando LLVM IR para assembly...")
            
            output_path = Path(output_file).with_suffix('.s')
            
            # Usar clang para gerar assembly
            cmd = [
                "clang",
                "-S",  # Gerar assembly
                "-O2",
                str(llvm_file),
                "-o", str(output_path)
            ]
            
            self.log(f"Comando: {' '.join(cmd)}")
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                if result.stderr:
                    self.error(f"clang stderr: {result.stderr}")
                self.error("Falha ao gerar assembly")
                return False
            
            if output_path.exists():
                self.log(f"✅ Assembly gerado: {output_path}")
                print(f"🎉 Compilação concluída! Assembly: {output_path}")
                return True
            else:
                self.error(f"Arquivo assembly não foi criado: {output_path}")
                return False
                
        except FileNotFoundError:
            self.error("clang não encontrado. Instale o LLVM/clang.")
            return False
        except Exception as e:
            self.error(f"Erro ao gerar assembly: {e}")
            return False

def main():
    """Função principal do script de compilação"""
    parser = argparse.ArgumentParser(
        description='Compilador Symplia - Compila arquivos .sym para executáveis',
        epilog='''
Exemplos:
  %(prog)s programas/exemplo.sym              # Compila para executável
  %(prog)s exemplo.sym -o meu_programa        # Nome personalizado  
  %(prog)s exemplo.sym --target llvm-ir       # Gera apenas LLVM IR
  %(prog)s exemplo.sym -v                     # Modo verboso

💡 Dica: Crie uma pasta 'programas/' para organizar seus arquivos .sym
        '''
    )
    
    parser.add_argument(
        'source_file',
        help='Arquivo fonte Symplia (.sym)'
    )
    
    parser.add_argument(
        '-o', '--output',
        dest='output_file',
        help='Arquivo de saída (sem extensão para executável)'
    )
    
    parser.add_argument(
        '-t', '--target',
        choices=['executable', 'llvm-ir', 'assembly'],
        default='executable',
        help='Target da compilação (padrão: executable)'
    )
    
    parser.add_argument(
        '-O', '--optimize',
        action='store_true',
        help='Ativa otimizações no codegen'
    )
    
    parser.add_argument(
        '-v', '--verbose',
        action='store_true',
        help='Modo verboso - mostra informações detalhadas'
    )
    
    parser.add_argument(
        '-k', '--keep-temp',
        action='store_true',
        help='Mantém arquivos temporários (.ast.json, .ll)'
    )
    
    parser.add_argument(
        '--version',
        action='version',
        version='Symplia Compiler 0.1.0'
    )
    
    args = parser.parse_args()
    
    print("=== 🚀 Compilador Symplia ===")
    print(f"Arquivo: {args.source_file}")
    print(f"Target: {args.target}")
    print("=" * 30)
    
    # Criar e executar compilador
    compiler = SympliaCompiler(verbose=args.verbose, keep_temp=args.keep_temp)
    
    success = compiler.compile(
        source_file=args.source_file,
        output_file=args.output_file,
        optimize=args.optimize,
        target=args.target
    )
    
    if success:
        print("✅ Compilação bem-sucedida!")
    else:
        print("❌ Compilação falhou!")
        if compiler.errors:
            print("\nErros encontrados:")
            for error in compiler.errors:
                print(f"  • {error}")
        sys.exit(1)

if __name__ == "__main__":
    main()