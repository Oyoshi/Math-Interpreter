# math-interpreter

Prototype of a math interpreter. Because this project was created only for the learning purpose so there are some limitations.

TODO - list limitations

## Demo
TODO

## Build & Run :worker:
TODO

## Flow

Input string is being converted by the **lexer** (**lexical analyzer**, **scanner** or **tokenizer**) into stream of **tokens**.This process is being called __lexical analysis__.  Then these **tokens** is being passed to the **parser** (**semantic analyzer**).**Parser** converts them into **AST**. This process is being called __syntax analysis__ or __parsing__. Finally **interpreter** runs a visitor mechanism (tree walker) on the **AST** using **postorder traversal** algorithm and finally evaluate a result.

## Grammar

Typical algebraic expression (might) consists of **Terms**, **Factors**, **Powers**, **Coefficients** and **Constants**.In this case I've made little simplification. It's easily visible in the grammar (for the pragmatic reason I'm using EBNF notation):

```
expr   : power ((PLUS | MINUS) power)*
power  : term (POW term)*
term   : factor ((MUL | DIV) power)*
factor : (PLUS | MINUS) factor | INTEGER | LPAREN expr RPAREN
```

## references:

1. [Ruslans Blog](https://ruslanspivak.com/)
2. [EBNF](https://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_form)
3. [Exponentiation By Squaring](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
