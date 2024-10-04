# Jist Programming Language

Welcome to **Jist**, a lightweight and efficient programming language built in **Rust**. Jist is designed for simplicity and flexibility, offering fast performance with a clean and readable syntax. This README provides an overview of the language, its features, syntax, and installation instructions.

---

## Table of Contents
1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Syntax Overview](#syntax-overview)
4. [Data Types](#data-types)
5. [Control Structures](#control-structures)
6. [Functions](#functions)
7. [Error Handling](#error-handling)
8. [Examples](#examples)
9. [Contributing](#contributing)
10. [License](#license)

---

## Introduction

Jist is built using **Rust**, taking advantage of its safety and performance. With a focus on minimalism, Jist is suited for both small scripting tasks and larger, more complex applications. Rust's ownership system ensures memory safety, and Jist inherits these properties, making it a secure and performant language.

### Key Features:
- Simple and expressive syntax
- Safe and fast, leveraging Rust's ownership model
- Flexible typing system with both static and dynamic capabilities
- Built-in concurrency support using Rust's async features
- Cross-platform support, compiling to efficient binaries

---

## Installation

**Requirements:**
- [Jist Compiler (JistR)](https://example.com/download)
- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Standard Library](https://example.com/stdlib)

```bash
# Install the Jist compiler (requires Rust installed)
# Currently, the compiler is not available via a package manager. Please clone the repository to use.

# If on Windows
./setup.ps1

# If on Linux/macOS
./setup.sh

# Alternatively, you can manually build and install:
$ cargo install --path .
## Syntax Overview

### Hello World
```jist
echo("Hello, World!")
```
### Variables
```jist
//Copy code
let name: string = "Jist"
let version: float = 1.0
```
### Comments
```jist
//Copy code
// This is a single-line comment

/* 
   This is a 
   multi-line comment 
*/
```
### Data Types

Jist supports both primitive and complex data types:

Primitive Types:
Integer
Float
String
Boolean
Complex Types:
Arrays
Dictionaries
Custom Structs
```jist
//Copy code
let age: int = 25        // Integer
let pi: float = 3.14       // Float
let greeting: string = "Hi" // String
let firstInital: char = 'J' // Char, notice singe quotes for chars and double quotes for strings 
let isValid: boolean = true  // Boolean
```
### Control Structures

If-Else
```jist
Copy code
if (condition) {
    // do something
} else {
    // do something else
}
```
Loops
```jist
//Copy code
for (i in 0..10) {
    echo(i)
}

while (condition) {
    // do something
```
### Functions

```jist
//Copy code
func add(a: int, b: int) -> int {
    return a + b;
```
### Error Handling

Jist uses try-catch blocks for error handling, inspired by Rust’s result and error types.

```jist
//Copy code
try {
    // code that may fail
} catch error {
    // handle error
```

## Contributing

We welcome contributions! Please follow these steps if you would like to contribute:

Fork the repository.
Create a new branch (git checkout -b feature-branch).
Commit your changes (git commit -m 'Add new feature').
Push to the branch (git push origin feature-branch).
Open a Pull Request.
Please see CONTRIBUTING.md for more guidelines on contributing.

## License

Jist is licensed under the MIT License

