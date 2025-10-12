# Custom Compiler Project

This project is a from-scratch implementation of a modern compiler for a C-like programming language.  
It is designed as a modular and educational compiler architecture, written entirely in Rust.

---

## 🧩 Overview

The compiler is built around a classic multi-stage design, where each stage is responsible for transforming the source program closer to executable machine code.  
Each stage communicates through well-defined data structures to ensure modularity, clarity, and extensibility.

---
## 🧱 Compiler Architecture

### 1. **Lexical Analysis (Lexer)**
The lexer converts raw source code into a stream of tokens — the smallest meaningful units of the language.  
It identifies identifiers, keywords, literals, symbols, and operators.

Tokens are **not stored in a collection**.  
Instead, they are retrieved **one by one** through the `get_next_token()` method, which returns `Option<Token>` until the end of the source is reached.

**Output:** `Option<Token>`

---

### 2. **Syntax Analysis (Parser)**
The parser consumes tokens from the lexer and constructs an **Abstract Syntax Tree (AST)** based on the grammar rules of the language.  
This stage uses a **bottom-up parsing** technique for efficient handling of complex grammars.

Output: `AST`

---

### 3. **Semantic Analysis**
The semantic analysis phase verifies that the AST follows the logical and type-related rules of the language.  
This includes:
- Type checking  
- Scope resolution  
- Variable/function declarations and usages  
- Constant evaluation  

This stage enriches the AST with additional information, such as symbol table links and inferred types.

---

### 4. **Code Generation**
The code generation phase transforms the semantically valid AST into an intermediate or target representation (IR).  
This stage is designed to be modular, allowing future backends (such as x86-64, LLVM IR, or custom bytecode).  

Responsibilities include:
- Instruction selection  
- Register or stack allocation  
- Expression evaluation  
- Control flow translation  

Output: target instructions or intermediate code.

---

## 🧠 Goals

- Build a complete compiler pipeline from source text to executable representation  
- Implement modular and reusable components  
- Serve as a platform for exploring language design, optimization, and runtime systems  

---

## 🛠️ Technologies

- **Language:** Rust  
- **Paradigm:** Modular, multi-stage compiler architecture  
- **Parsing:** Bottom-up parser (handwritten)  
- **Error Handling:** Structured diagnostics  
- **Target:** Custom code generation layer  

---

