# Welcome to **Jist**, a lightweight and efficient programming language built in **Rust**. Jist is designed for simplicity and flexibility, offering fast performance with a clean and readable syntax. This README provides an overview of the language, its features, syntax, and installation instructions.

---

## Table of Contents
1. [Introduction](#introduction)
2. [Installation](#installation)
3. [Syntax Overview](#syntax-overview)
4. [Data Types](#data-types)
5. [Control Structures](#control-structures)
6. [Functions](#functions)
7. [Error Handling](#error-handling)
8. [Standard Library](#standard-library)
9. [Examples](#examples)
10. [Contributing](#contributing)
11. [License](#license)

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

Jist will also try it's best to avoid type conflicts, meaning that the following

```jist
let a: float = 3.1;
let b: int = a;
```

Will result in b having a value of 3 instead of throwing an error

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

### Standard Library

Jist has an extessive built in library which does even need an import.
These are called by using standard function call syntax

IE
```jist
let a: string = to_uppercase("hello world");
echo(a);
```
Output:
HELLO WORLD

```rust
fn max(a: f64, b: f64) -> f64  
fn min(a: f64, b: f64) -> f64  
fn rand() -> f64  
fn floor(a: f64) -> f64  
fn ceil(a: f64) -> f64  
fn round(a: f64) -> f64  
fn add(a: f64, b: f64) -> f64  
fn sub(a: f64, b: f64) -> f64  
fn mult(a: f64, b: f64) -> f64  
fn divide(a: f64, b: f64) -> f64  
fn echo(a: String)  
fn echoln(a: String)  
fn abs(a: f64) -> f64  
fn pow(a: f64, b: f64) -> f64  
fn sqrt(a: f64) -> f64  
fn log(a: f64, base: f64) -> f64  
fn sin(a: f64) -> f64  
fn cos(a: f64) -> f64  
fn tan(a: f64) -> f64  
fn concat(a: String, b: String) -> String  
fn len(s: String) -> usize  
fn to_uppercase(s: String) -> String  
fn to_lowercase(s: String) -> String  
fn trim(s: String) -> String  
fn input(s: String) -> String
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

