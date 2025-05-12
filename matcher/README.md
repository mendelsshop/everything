[![Crates.io](https://img.shields.io/crates/v/matcher-proc-macro.svg)](https://crates.io/crates/matcher-proc-macro)
[![Docs.rs](https://docs.rs/matcher-proc-macro/badge.svg)](https://docs.rs/matcher-proc-macro)
[![license](https://img.shields.io/github/license/mendelsshop/everything)](https://github.com/mendelsshop/everything/blob/main/LICENSE)

# matcher proc macro

A proc macro to generate an SExpr pattern matcher - for [everything(-lang)](https://github.com/mendelsshop/everything) and [macro-lang](https://github.com/mendelsshop/macro-lang)
Provides two macros: `match_syntax` and `match_syntax_as`, which are pretty much the same, except the type the `match_syntax` generates is basically opaque.

Will only work in everything(-lang) or macro-lang as it relies on some data structures and functions that they both share, at a later point I might split out those data structures into another crate, in which case this would usuable without those crates.

## pattern syntax
```
sexpr => <symbol>
        | <symbol>":id"
        | "id"
        | "("<sexpr>+ "." <sexpr>")"
        | "("<sexpr>*")"
        | <sexpr> "..."
        | <sexpr> "..+"
symbol => ([a-zA-Z]|_)([a-zA-Z0-9]|_)*
```

## macro syntax

- `match_syntax!(sexpr)`
- `match_syntax_as(StructName as sexpr)`

Example: `let matcher = match_syntax((foo bar));`.

## Running a matcher

The result of `match_syntax` or `match_syntax_as` is the `matches` method of opaque struct or a the creation of a new struct with a `matches` method on it (in the case of `match_syntax_as`).

In the case of `match_syntax`: do `match_syntax(sexpr)(sexpr_to_run_matcher_on)`.
In the case of `match_syntax_as`: use the `matches` method on the newline generated struct like `StructName::matches(sexpr_to_run_matcher_on)`.

Note: `sexpr_to_run_matcher_on` has different syntax then `sexpr`. See note at the beggining.

Example: `let results = matcher(sexpr!((a (b c))))?`, `sexpr!` is a macro from everything(-lang) or macro-lang.

## Using the results

Assuming you ran the matcher and unrwapped the result, you just do access the field like you declared in the original `sexpr`.

Example: `results.foo`.

Note: if you used an `:id` postfix for your `symbol` pattern then to access from the struct it will be `<symbol>_id`.
