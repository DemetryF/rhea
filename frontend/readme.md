## What is language Frontend?
frontend - part of the compiler or interpreter of the language that builds the ast

Frontend has two stages: lexical analysis (tokenization) and syntactic analysis (parser).
Lexical analysis transforms source code to tokens - pair of value and type like (`+`, operator) or (`15`, number);
Parser collect AST (Abstract Syntax Tree) from tokens and transmits it to language backend.