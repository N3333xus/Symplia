# Guia de Instalação e Uso

## 1 - Instalação de Dependencias

Necessário Rust 1.90, é interresante que seja instalado por meio do Rustup

Windows

Linux (Debian/Arch)

## 2 - Clone o Repositório

- Necessario rust 1.90.0 e cargo 1.90.0
- necessário cmake

rustup component add llvm-tools
cargo install cargo-binutils

export LLVM_SYS_210_PREFIX=$(llvm-config --prefix)


```
git clone https://github.com/N3333xus/Symplia
cd Symplia
cd compiler/tests

Execução Completa(main.rs): cargo run -- exemplo3.sym 

```
aonde em /tests:
- exemplo1.sym : programa com erros
- exemplo2.sym : programa com erros
- exemplo3.sym : programa maior e sem erros 
- exemplo4.sym : programa curto e sem erros


Saida detalhada para analise léxica: FALTA
Saida detalhada para analise sintatica: FALTA
Saída somente da geração da AST: FALTA   

