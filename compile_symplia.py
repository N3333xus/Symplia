#!/usr/bin/env python3
"""
Script de integração do compilador Symplia
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
        self.errors = []
        
        self.project_root = Path(__file__).parent
        self.compiler_dir = self.project_root / "compiler"      # Código Rust
        self.codegen_dir = self.compiler_dir / "codegen"        # Código Python
        self.programas_dir = self.project_root / "programas"    # Programas .sym
        self.build_dir = self.project_root / "build"            # diretório de build
        
        self.build_dir.mkdir(exist_ok=True)

        self._validate_project_structure()
    
    def _validate_project_structure(self):
        """Valida se a estrutura do projeto está correta"""
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
        
        for dir_path in required_dirs:
            if not dir_path.exists():
                raise RuntimeError(f"Diretório não encontrado: {dir_path}")
        
        for file_path in required_files:
            if not file_path.exists():
                raise RuntimeError(f"Arquivo não encontrado: {file_path}")
        
        if not self.programas_dir.exists():
            self.log(f"AVISO: Diretório 'programas/' não encontrado. Crie para organizar seus arquivos .sym")
        
        self.log("Estrutura do projeto validada")
    
    def _find_source_file(self, source_file: str) -> Optional[Path]:
        """Encontra o arquivo fonte, procurando em programas/ se necessário"""
        source_path = Path(source_file)
        
        # Se o caminho existe diretamente, usar
        if source_path.exists():
            return source_path
        
        # Se não, tentar na pasta programas/
        programas_path = self.programas_dir / source_path.name
        if programas_path.exists():
            self.log(f"Arquivo encontrado em: {programas_path}")
            return programas_path
        
        # Tentar com extensão .sym se não tiver
        if not source_path.suffix:
            source_path = source_path.with_suffix('.sym')
            if source_path.exists():
                return source_path
            
            programas_path = self.programas_dir / source_path.name
            if programas_path.exists():
                self.log(f"Arquivo encontrado em: {programas_path}")
                return programas_path
        
        return None
    
    def log(self, message: str):
        if self.verbose:
            print(f"[SYMPLIA] {message}")
    
    def error(self, message: str):
        self.errors.append(message)
        print(f"ERRO: {message}", file=sys.stderr)
    
    def compile(self, source_file: str, output_file: Optional[str] = None, 
                optimize: bool = False, target: str = "executable") -> bool:
        """Compila um arquivo .sym para o target especificado"""
        try:
            source_path = self._find_source_file(source_file)
            if source_path is None:
                self.error(f"Arquivo fonte não encontrado: {source_file}")
                self.error(f"Procurei em: {Path(source_file).absolute()} e {self.programas_dir}")
                return False
            
            if source_path.suffix != '.sym':
                self.error(f"Arquivo deve ter extensão .sym: {source_path}")
                return False
            
            base_name = source_path.stem
            
            json_file = self.build_dir / f"{base_name}.ast.json"
            llvm_file = self.build_dir / f"{base_name}.ll"
            
            if output_file is None:
                output_file = base_name
            
            final_output = self.build_dir / output_file
            
            self.log(f"Iniciando compilação de: {source_path}")
            self.log(f"Arquivo AST JSON: {json_file}")
            self.log(f"Arquivo LLVM IR: {llvm_file}")
            
            # PASSO 1: Compilar com Rust (frontend) - gerar JSON na pasta build
            self.log("=== FASE 1: Frontend Rust (Análise) ===")
            if not self._run_rust_compiler(str(source_path), json_file):
                return False
            
            # PASSO 2: Gerar LLVM IR com Python (codegen) - ler JSON da pasta build
            self.log("=== FASE 2: Backend Python (Geração de Código) ===")
            if not self._run_python_codegen(json_file, llvm_file, optimize):
                return False
            
            # PASSO 3: Compilar para target final
            self.log("=== FASE 3: Geração do Target Final ===")
            if target == "executable":
                return self._generate_executable(llvm_file, str(final_output))
            elif target == "llvm-ir":
                final_ll_file = final_output.with_suffix('.ll')
                shutil.copy2(llvm_file, final_ll_file)
                self.log(f"✅ Arquivo LLVM IR salvo: {final_ll_file}")
                return True
            elif target == "assembly":
                return self._generate_assembly(llvm_file, str(final_output))
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
            # Limpar arquivos intermediários se não for keep_temp
            if not self.keep_temp:
                self._cleanup_intermediate_files(json_file, llvm_file)
    
    '''def _cleanup_intermediate_files(self, json_file: Path, llvm_file: Path):
        """Remove arquivos intermediários da pasta build se existirem"""
        try:
            if json_file.exists():
                self.log(f"Removendo arquivo intermediário: {json_file}")
                json_file.unlink()
            if llvm_file.exists():
                self.log(f"Removendo arquivo intermediário: {llvm_file}")
                llvm_file.unlink()
        except Exception as e:
            self.log(f"AVISO: Não foi possível remover alguns arquivos intermediários: {e}")'''
    
    def _run_rust_compiler(self, source_file: str, json_output: Path) -> bool:
        """Executa o compilador Rust para análise do código fonte"""
        try:
            self.log("Executando compilador Rust...")
            
            source_path = Path(source_file)
            if not source_path.is_absolute():
                source_path = source_path.absolute()
            
            self.log(f"Caminho absoluto do arquivo: {source_path}")
            
            cmd = [
                "cargo", "run", "--release", "--quiet", "--",
                str(source_path)
            ]
            
            env = os.environ.copy()
            
            self.log(f"Comando: {' '.join(cmd)}")
            self.log(f"Diretório: {self.compiler_dir}")
            
            result = subprocess.run(
                cmd,
                cwd=self.compiler_dir,
                capture_output=True,
                text=True,
                env=env
            )
            
            # verifica se o arquivo existe no local esperado
            if json_output.exists():
                self.log(f"Análise Rust concluída: {json_output}")
                return True
            else:
                # Mostrar erros do Rust se houver
                if result.stderr:
                    error_lines = result.stderr.strip().split('\n')
                    for line in error_lines:
                        if line.strip() and "Compiling" not in line and "Finished" not in line:
                            self.error(f"Rust: {line}")
                if result.stdout:
                    output_lines = result.stdout.strip().split('\n')
                    for line in output_lines:
                        if line.strip() and "Compiling" not in line and "Finished" not in line:
                            self.log(f"Rust: {line}")
                
                self.error(f"Compilador Rust não gerou arquivo JSON esperado: {json_output}")
                return False
                
        except subprocess.CalledProcessError as e:
            self.error(f"Erro na execução do compilador Rust: {e}")
            if hasattr(e, 'stderr') and e.stderr:
                self.error(f"Stderr: {e.stderr.decode()}")
            return False
        except FileNotFoundError:
            self.error("Compilador Rust não encontrado. Certifique-se de que o Rust está instalado.")
            return False
        except Exception as e:
            self.error(f"Erro inesperado no compilador Rust: {e}")
            return False
    
    def _run_python_codegen(self, json_file: Path, llvm_file: Path, optimize: bool) -> bool:
        """Executa o codegen Python para gerar LLVM IR"""
        try:
            self.log("Executando codegen Python...")
            
            cmd = [
                sys.executable,
                str(self.codegen_dir / "symplia_codegen.py"),
                str(json_file),
                "-o", str(llvm_file)
            ]
            
            if optimize:
                cmd.append("--optimize")
            
            if self.verbose:
                cmd.append("-v")
            
            self.log(f"Comando: {' '.join(cmd)}")
            
            result = subprocess.run(
                cmd,
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                if result.stderr:
                    self.error(f"Codegen Python stderr: {result.stderr}")
                self.error("Falha no codegen Python")
                return False
            
            if llvm_file.exists():
                self.log(f"Codegen Python concluído: {llvm_file}")
                return True
            else:
                self.error(f"Codegen Python não gerou arquivo LLVM: {llvm_file}")
                return False
                
        except Exception as e:
            self.error(f"Erro inesperado no codegen Python: {e}")
            return False
    
    def _generate_executable(self, llvm_file: Path, output_file: str) -> bool:
        """Compila LLVM IR para executável usando clang"""
        try:
            self.log("Compilando LLVM IR para executável...")
            
            output_path = Path(output_file).with_suffix('')
            if os.name == 'nt':
                output_path = output_path.with_suffix('.exe')
            
            cmd = [
                "clang", 
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
                self.error("Falha ao compilar para executável")
                return False
            
            if output_path.exists():
                self.log(f"Executável gerado: {output_path}")
                print(f"Compilação concluída! Executável: {output_path}")
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
            
            cmd = [
                "clang",
                "-S",
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
                self.log(f"Assembly gerado: {output_path}")
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
    parser = argparse.ArgumentParser(
        description='Compilador Symplia - Compila arquivos .sym para executáveis',
        epilog='''
Exemplos:
  %(prog)s exemplo3.sym                       # Busca automaticamente em programas/
  %(prog)s programas/exemplo3.sym             # Caminho explícito
  %(prog)s exemplo3.sym -o meu_programa       # Nome personalizado  
  %(prog)s exemplo3.sym --target llvm-ir      # Gera apenas LLVM IR
  %(prog)s exemplo3.sym -v                    # Modo verboso
        '''
    )
    
    parser.add_argument(
        'source_file',
        help='Arquivo fonte Symplia (.sym) - busca em programas/ se não encontrado'
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
        help='Mantém arquivos intermediários (.ast.json, .ll) na pasta build'
    )
    
    parser.add_argument(
        '--version',
        action='version',
        version='Symplia Compiler 0.1.0'
    )
    
    args = parser.parse_args()
    
    print("=== 🚀 Compilador Symplia 🚀 ===")
    print(f"Arquivo: {args.source_file}")
    print(f"Target: {args.target}")
    print("=" * 30)
    
    compiler = SympliaCompiler(verbose=args.verbose, keep_temp=args.keep_temp)
    
    success = compiler.compile(
        source_file=args.source_file,
        output_file=args.output_file,
        optimize=args.optimize,
        target=args.target
    )
    
    if success:
        print("Compilação bem-sucedida!")
    else:
        print("Compilação falhou!")
        if compiler.errors:
            print("\nErros encontrados:")
            for error in compiler.errors:
                print(f"  • {error}")
        sys.exit(1)

if __name__ == "__main__":
    main()