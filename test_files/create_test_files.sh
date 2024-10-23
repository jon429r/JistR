#!/bin/bash

# Create and fill test_files

# Variable reassignment
cat << 'VAR_REASSIGN' > test_files/variable_reassignment.jist
let a: int = 5;
a = 10;
print("Variable Name: a\nVariable Type: Int\nVariable Value: " + a);
VAR_REASSIGN

# Arithmetic operations
cat << 'ARITH_OPS' > test_files/arithmetic_operations.jist
let a: int = 3;
let b: int = 2;
let add: int = a + b;
let sub: int = a - b;
let mul: int = a * b;
let div: int = a / b;
print("Result of addition: " + add);
print("Result of subtraction: " + sub);
print("Result of multiplication: " + mul);
print("Result of division: " + div);
ARITH_OPS

# If-else conditions
cat << 'IF_ELSE' > test_files/if_else_conditions.jist
let a: int = 5;
let b: int = 3;
if a > b {
    print("Condition met: a is greater than b");
} else {
    print("Condition not met");
}
IF_ELSE

# While loop
cat << 'WHILE_LOOP' > test_files/while_loop.jist
let i: int = 1;
while i <= 3 {
    print("Looping: " + i);
    i = i + 1;
}
print("Loop ended");
WHILE_LOOP

# For loop
cat << 'FOR_LOOP' > test_files/for_loop.jist
for let i: int = 1 to 3 {
    print("Iterating: " + i);
}
FOR_LOOP

# Function declaration and call
cat << 'FUNC_DECL' > test_files/function_declaration.jist
fn add(a: int, b: int) -> int {
    return a + b;
}
let result: int = add(10, 5);
print("Function Result: " + result);
FUNC_DECL

# Nested function calls
cat << 'NESTED_FUNC' > test_files/nested_function_calls.jist
fn outer(x: int) -> int {
    return x * 2;
}
fn inner(y: int) -> int {
    return y + 5;
}
let outer_result: int = outer(inner(5));
print("Outer function result: " + outer_result);
print("Inner function result: " + inner(5));
NESTED_FUNC

# Recursive function
cat << 'RECURSIVE_FUNC' > test_files/recursive_function.jist
fn factorial(n: int) -> int {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
let result: int = factorial(5);
print("Factorial of 5 is " + result);
RECURSIVE_FUNC

# Logical operations
cat << 'LOGICAL_OPS' > test_files/logical_operations.jist
let a: boolean = true;
let b: boolean = false;
print("Logical AND: " + (a && b));
print("Logical OR: " + (a || b));
print("Logical NOT: " + (!a));
LOGICAL_OPS

# File read (assuming you have a file named hello.txt)
cat << 'FILE_READ' > test_files/file_read.jist
let file_content: string = read("hello.txt");
print("File content: " + file_content);
FILE_READ

# File write
cat << 'FILE_WRITE' > test_files/file_write.jist
write("output.txt", "Hello, world!");
print("File successfully written");
FILE_WRITE

# Single-line comments
cat << 'SINGLE_LINE_COMMENTS' > test_files/single_line_comments.jist
// This is a comment
let a: int = 5;
print("Execution completed without processing the commented line");
SINGLE_LINE_COMMENTS

# Multi-line comments
cat << 'MULTI_LINE_COMMENTS' > test_files/multi_line_comments.jist
/* This is a
multi-line comment */
let a: int = 10;
print("Execution completed without processing the commented block");
MULTI_LINE_COMMENTS

# Complex expressions
cat << 'COMPLEX_EXPRESSIONS' > test_files/complex_expressions.jist
let a: int = 10;
let b: int = 5;
let c: int = (a * b) + (a / b);
print("Expression result: " + c);
COMPLEX_EXPRESSIONS

# Type mismatch (expect error)
cat << 'TYPE_MISMATCH' > test_files/type_mismatch.jist
let a: int = 10;
let b: string = "string";
let result = a + b; // This should cause a type mismatch error
TYPE_MISMATCH

echo "Test files created successfully."
