# Compiler-in-Rust
Compiler written from scratch in the Rust programming language
The goal is to write the compiler without using any external modules,
only referencing the Rust standard library. This involves writing things
like Memory Maps from scratch. 

In this compiler the lexical analysis process happens based on ASCII character codes
as opposed to traditional strings. The point of this is to minimize memory footprint,
making the compiler as lightweight as possible. The main difference is that ASCII 
codes can be stored as a single byte each, whereas UTF-8 characters can take more than 
a byte.

This means that the read source code of a file like hello.txt would be
84, 72, 73, 83, 32, 73, 83, 32, 65, 32, 84, 69, 83, 84, 32, 70, 73, 76, 69
which reads: THIS IS A TEST FILE

Currently, this project is behaving as though I'm writing a new language. 
The end goal is to create a compiler for an existing language.