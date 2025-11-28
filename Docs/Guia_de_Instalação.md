# Guia de Instalação
---
## 1 - Pré-Requisitos

### Rust (1.80+)
É altamente recomendável que a instalação seja feita por meio do **Rustup**, que está disponível em: https://rustup.rs/

**Windows**
```
Acesse https://rustup.rs/
Vá em: display all supported installers
Baixe e Execute o arquivo rustup-init.exe
```

**Linux (Debian)**
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version # Verifique a instalação
```

**Linux (Arch)**
```
sudo pacman -S rustup
```

### Python (3.8+)

**Windows**
```
# Via winget (Windows 10+)
winget install Python.Python.3.11

# Ou baixe do site oficial: https://www.python.org/downloads/
```
**Linux (Debian)**
```
sudo apt install python3 python3-pip python3-venv
```

**Linux (Arch)**
```
sudo pacman -S python python-pip
```
---

## 2 - Instalação

### Clone o Repositório

```
git clone https://github.com/N3333xus/Symplia
cd Symplia
```

### Crie e Ative o Venv Python
```
python -m venv my_env
source my_env/bin/activate  # Caso voce esteja utilizando Linux/MacOS
my_env\Scripts\activate     # Caso voce esteja utilizando Windows
```
### Instale as Dependencias Python com pip
```
pip install -r compiler/requirements.txt
```

### Compilar o Código Rust

```
cd compiler
cargo build --release
cd ..
```

Pronto! Tudo esta Configurado.

---

## 3 - Compilando e Executando um Programa Symplia

No diretório **programas/** voce encontrará alguns exemplos de programas escritos em Symplia.<br>
(Veja mais detalhes na documentação da linguagem [>>Aqui<<](Documentacao-Symplia-pt-br.md))

### Compilando um Programa
Primeiro garanta que voce está na raiz do projeto:
```
~$ pwd
> /home/usuario/Documents/Symplia
```
Para compilar os programas que estão no diretório **programas/** use:

```
python compile_symplia.py programas/exemplo3.sym
```
### Executando o Binário
O comando que vimos acíma ira criar um diretório **build/**, aonde é gerado o binário executável.<br>
Para executar o binário, use:
```
./build/exemplo3
```
### Opções Avançadas de Compilação
É possível também ver um output mais verboso de compilação com o parâmetro **-v**
```
python compile_symplia.py programas/exemplo3.sym -v
```

---




