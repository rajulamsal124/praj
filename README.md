# RCode Interpreter

RCode is a simple programming language interpreter built in Rust. This project showcases the implementation of a basic interpreter with lexical analysis, parsing, and evaluation.

## Table of Contents

- [RCode Interpreter](#rcode-interpreter)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
  - [Usage](#usage)
  - [Example](#example)
    - [`examples/add.pr`](#examplesaddpr)

## Introduction

This project is a Rust-based interpreter for a custom language, RCode. It includes components for tokenization, parsing, and evaluating expressions. The interpreter can read source code from `.pr` files, parse them into an abstract syntax tree (AST), and evaluate the expressions.

## Features

- Lexical analysis and tokenization
- Expression parsing with basic arithmetic operations
- Evaluation of expressions with runtime error handling

## Getting Started

To get started with RCode, you need to have Rust installed on your machine. You can download and install Rust from [rust-lang.org](https://www.rust-lang.org/).

### Prerequisites

- Rust (installed via [rustup](https://rustup.rs/))
- A text editor or IDE (optional, but recommended)

## Usage

1. **Clone the repository:**

   ```bash
   git clone https://github.com/yourusername/rcode-interpreter.git
   cd rcode-interpreter
   ```

2. **Build the project:**

   ```bash
   cargo build
   ```

3. **Run the interpreter with a `.pr` file:**

   ```bash
   cargo run <path to .pr file>
   ```

   Example:

   ```bash
   cargo run examples/add.pr
   ```

## Example

Here is an example `.pr` file to test the interpreter:

### `examples/add.pr`

```pr
let x = 5;
let y = 10;
print(x + y);
```
