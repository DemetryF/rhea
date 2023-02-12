# Welcome to Rhea!
Rhea is simple programming language, created for example language interpreter.

# About the name of the language
Like the rest of my project names Rhea associated with Saturn - she is second largest moon of Saturn.

# About syntax
You can see the grammar of the language in detail [here](https://github.com/DemetryF/rhea/blob/main/frontend/grammary.ebnf).
Rhea has 4 syntax statements:

## Function Declaration Statement
You can declare a function with the `fn` keyword like rust:
```rs
fn name(arg1, arg2) = arg1 + arg2;
```

## Variable Declaration Statement
In rhea you are required to declare your variables before using. You can do it with `let` keyword like rust or js:
```rs
let a = 2;
```
you can not initialize your variables, then they will be initialized to zero:
```rs
let a;
```

## Assignment Statement
After the declaration you can assign another value to your variable:
```rs
a = 3;
```

## Call Statement
You can call the function, for example you can call the built-in function `print`:
```rs
print(2);
```
If your function takes more than one argument, arguments are separated by a comma:
```rs
fn mul(a, b) = a * b;
print(mul(2, 3));
```

## Expressions
Rhea has 4 operators: `+`, `-`, `*`, `/`.

## Semicolons
All statements in Rhea must end with a semicolon.