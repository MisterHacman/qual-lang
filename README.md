# The Qual programming language

Qual is a low level, performance based programming language. 

## Functional features
It's designed to be a fast, safe and testable language.
Featuring classical functional features including:
* Closures
* First class functions
* Immutable state
* Pattern matching
* Type polymorphism
* Lazy evaluation
* Monads

These all help to create readable, testable and maintainable code.
Lazy evaluation is great for handling big data and AI.
Also, mutable state allows for safe threading which is very important in subjects like webservers.

## Procedural features
Qual also uses a few procedural features which are used for handling effects:
* Function procedures
* Compiler macros
* Borrow checking
* Asyncronous procedures
* Multithreading

Which help creating testable code with effects and memory safety. They help with simple tasks.

## Functional examples
Closures are anonymous functions
```
id = :x -> x            // id takes an input and returns it

drop = :x -> :y -> x    // drop takes two inputs and drops the second one and returns the first
```
