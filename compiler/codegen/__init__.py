"""
Compilador linguagem Symplia
Integração do frontend Rust com backend Python para geração de código LLVM
"""

__version__ = "0.1.0"
__author__ = "Eu, N3333xus!"

from pathlib import Path

PROJECT_ROOT = Path(__file__).parent.parent
COMPILER_DIR = PROJECT_ROOT / "compiler"
CODGEN_DIR = COMPILER_DIR / "codegen"

def get_version():
    return __version__