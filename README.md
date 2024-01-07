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