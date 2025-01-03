# macro

Reimplementation of rackets macro sysstem in rust for my compiler.
Following this [code](https://github.com/mflatt/expander) (currently on mini).

## differences
we do not have a concept of values so we use define/define-syntax instead of define-values/define-syntaxes
