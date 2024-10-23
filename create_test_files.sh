#!/bin/bash

# Create and fill test_files

# Variable reassignment
cat << 'VAR_REASSIGN' > test_files/variable_reassignment.jist
var a = 5;
a = 10;
print("Variable Name: a\nVariable Type: Int\nVariable Value: " + a);
VAR_REASSIGN

# Arithmetic operations
cat << 'ARITH_OPS' > test_files/arithmetic_operations.jist
var a = 3;
var b = 2;
var add = a + b;
var sub = a - b;
var mul = a * b;
var div = a / b;
print("Result of addition: " + add);
print("Result of subtraction: " + sub);
print("Result of multiplication: " + mul);
print("Result of division: " + div);
ARITH_OPS

# If-else conditions
cat << 'IF_ELSE' > test_files/if_else_conditions.jist
var a = 5;
var b = 3;
if a > b {
    print("Condition met: a is greater than b");
} else {
    print("Condition not met");
}
IF_ELSE

# While loop
cat << 'WHILE_LOOP' > test_files/while_loop.jist
var i = 1;
while i <= 3 {
    print("Looping: " + i);
    i = i + 1;
}
print("Loop ended");
WHILE_LOOP

# For loop
cat << 'FOR_LOOP' > test_files/for_loop.jist
for i = 1 to 3 {
    print("Iterating: " + i);
}
FOR_LOOP

# Function declaration and call
cat << 'FUNC_DECL' > test_files/function_declaration.jist
fn add(a, b) {
    return a + b;
}
var result = add(10, 5);
print("Function Result: " + result);
FUNC_DECL

# Nested function calls
cat << 'NESTED_FUNC' > test_files/nested_function_calls.jist
fn outer(x) {
    return x * 2;
}
fn inner(y) {
    return y + 5;
}
var outer_result = outer(inner(5));
print("Outer function result: " + outer_result);
print("Inner function result: " + inner(5));
NESTED_FUNC

# Recursive function
cat << 'RECURSIVE_FUNC' > test_files/recursive_function.jist
fn factorial(n) {
    if n <= 1 {
        return 1;
    } else {
        return n * factorial(n - 1);
    }
}
var result = factorial(5);
print("Factorial of 5 is " + result);
RECURSIVE_FUNC

# Logical operations
cat << 'LOGICAL_OPS' > test_files/logical_operations.jist
var a = true;
var b = false;
print("Logical AND: " + (a && b));
print("Logical OR: " + (a || b));
print("Logical NOT: " + (!a));
LOGICAL_OPS

# File read (assuming you have a file named hello.txt)
cat << 'FILE_READ' > test_files/file_read.jist
var file_content = read("hello.txt");
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
var a = 5;
print("Execution completed without processing the commented line");
SINGLE_LINE_COMMENTS

# Multi-line comments
cat << 'MULTI_LINE_COMMENTS' > test_files/multi_line_comments.jist
/* This is a
multi-line comment */
var a = 10;
print("Execution completed without processing the commented block");
MULTI_LINE_COMMENTS

# Empty file
touch test_files/empty_file.jist

# Complex expressions
cat << 'COMPLEX_EXPRESSIONS' > test_files/complex_expressions.jist
var a = 10;
var b = 5;
var c = (a * b) + (a / b);
print("Expression result: " + c);
COMPLEX_EXPRESSIONS

# Type mismatch (expect error)
cat << 'TYPE_MISMATCH' > test_files/type_mismatch.jist
var a = 10;
var b = "string";
var result = a + b; // This should cause a type mismatch error
TYPE_MISMATCH

echo "Test files created successfully."
