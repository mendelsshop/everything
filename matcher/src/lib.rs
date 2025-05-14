#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(
    clippy::use_self,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::missing_panics_doc
)]
#![doc = include_str!("./../README.md")]
use core::fmt;
use std::collections::HashSet;

use crate::custom::DotDotPlus;
use custom::{id, sym};
use proc_macro::TokenStream;
use quote::{ToTokens, TokenStreamExt, quote};
use rand::random;
use syn::{
    Ident, Token, ext::IdentExt, parenthesized, parse::Parse, parse_macro_input, spanned::Spanned,
};

#[derive(Clone, Debug, PartialEq)]
struct MatchStruct {
    binders: HashSet<Ident>,
}
mod custom {
    use syn::{custom_keyword, custom_punctuation};

    custom_punctuation!(DotDotPlus, ..+);
    custom_keyword!(id);
    custom_keyword!(sym);
}
struct NameAsSExpr(Ident, SExpr);
impl Parse for NameAsSExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        input.parse::<Token![as]>()?;
        let sexpr = input.parse()?;
        Ok(Self(ident, sexpr))
    }
}
#[derive(Debug, PartialEq, Clone)]
enum SExpr {
    Many(Box<Self>, MatchStruct),
    ManyOne(Box<Self>, MatchStruct),
    Any(Ident),
    // only matches identifiers
    Identifier(Ident),
    Symbol(Ident),
    Empty,
    Pair {
        car: Box<Self>,
        cdr: Box<Self>,
        binders: MatchStruct,
    },
}
impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Many(sexpr, _) => write!(f, "{sexpr} ..."),
            Self::ManyOne(sexpr, _) => write!(f, "{sexpr} ..+"),
            Self::Any(ident) => write!(f, "{ident}"),
            Self::Symbol(ident) => write!(f, "{ident}"),
            Self::Identifier(ident) => write!(f, "{ident}:id"),
            Self::Empty => write!(f, "()"),
            Self::Pair {
                car,
                cdr,
                binders: _,
            } => {
                write!(f, "({car}")?;
                let mut second: Self = *(cdr.clone());
                while let Self::Pair {
                    car,
                    cdr,
                    binders: _,
                } = second
                {
                    write!(f, " {car}")?;
                    second = *cdr;
                }
                if second != Self::Empty {
                    write!(f, " . {second}")?;
                }
                write!(f, ")")
            }
        }
    }
}
impl SExpr {
    fn binders(&self) -> MatchStruct {
        match self {
            Self::Many(_, match_struct) | Self::ManyOne(_, match_struct) => match_struct.clone(),
            Self::Any(ident) | Self::Identifier(ident) | Self::Symbol(ident) => MatchStruct {
                binders: HashSet::from([ident.clone()]),
            },
            Self::Empty => MatchStruct {
                binders: HashSet::new(),
            },
            Self::Pair {
                car: _,
                cdr: _,
                binders,
            } => binders.clone(),
        }
    }
}
impl Parse for SExpr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let sexpr = if input.peek(Ident::peek_any) {
            let ident = Ident::parse_any(input)?;
            if input.peek(Token![:]) {
                input.parse::<Token![:]>()?;
                if input.peek(id) {
                    input.parse::<id>()?;
                    let ident = Ident::new(&(ident.to_string() + "_id"), ident.span());
                    Self::Identifier(ident)
                } else if input.peek(sym) {
                    input.parse::<sym>()?;
                    let ident = Ident::new(&(ident.to_string() + "_sym"), ident.span());
                    Self::Symbol(ident)
                } else {
                    return Err(input.error("unkown syntax expected `id` or `sym` after `:`"));
                }
            } else if ident == "id" {
                Self::Identifier(ident)
            } else {
                Self::Any(ident)
            }
        } else {
            let paren_input;
            parenthesized!(paren_input in input);
            parse_paren(&paren_input)?
        };
        if input.peek(Token![...]) {
            input.parse::<Token![...]>()?;

            let binders = sexpr.binders();
            Ok(Self::Many(Box::new(sexpr), binders))
        } else if input.peek(DotDotPlus) {
            input.parse::<DotDotPlus>()?;
            let binders = sexpr.binders();
            Ok(Self::ManyOne(Box::new(sexpr), binders))
        } else {
            Ok(sexpr)
        }
    }
}

fn parse_paren(input: &syn::parse::ParseBuffer<'_>) -> syn::Result<SExpr> {
    if input.is_empty() {
        Ok(SExpr::Empty)
    } else {
        let current = input
            .parse::<SExpr>()
            .map_err(|e| input.error(format!("unterminated sexpr pair {e}")))?;
        let mut current_binders = current.binders();
        if input.peek(Token![.]) && !(input.peek(Token![...]) || input.peek(DotDotPlus)) {
            input.parse::<Token![.]>()?;
            let end = input.parse::<SExpr>().map_err(|_| {
                input.error("expected expression after improper list dots".to_string())
            })?;
            if input.is_empty() {
                check_duplicates(input, &mut current_binders, &end)?;
                Ok(SExpr::Pair {
                    car: Box::new(current),
                    cdr: Box::new(end),
                    binders: current_binders,
                })
            } else {
                Err(input.error("expected nothing after last expression in improper list"))
            }
        } else if input.is_empty() && matches!(current, |SExpr::Many(_, _)| SExpr::ManyOne(_, _)) {
            Ok(current)
        } else {
            let next = parse_paren(input)?;
            check_duplicates(input, &mut current_binders, &next)?;
            Ok(SExpr::Pair {
                car: Box::new(current),
                cdr: Box::new(next),
                binders: current_binders,
            })
        }
    }
}

fn check_duplicates(
    input: &syn::parse::ParseBuffer<'_>,
    current_binders: &mut MatchStruct,
    next: &SExpr,
) -> Result<(), syn::Error> {
    next.binders().binders.into_iter().try_for_each(|binder| {
        let message = format!("duplicate binder {binder}");
        if current_binders.binders.insert(binder) {
            Ok(())
        } else {
            Err(input.error(message))
        }
    })
}

impl ToTokens for SExpr {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Many(sexpr, match_struct) => {
                let binders = match_struct.binders.clone().into_iter();
                let binders1 = match_struct.binders.clone().into_iter();
                let token = quote! {
                    let res = s.fold_to_syntax_list::<Self, String>(
                        &mut |s, mut current| {
                            let mut this = Self::default();
                            #sexpr;
                            #(  current.#binders = crate::ast::Ast::Pair(Box::new(crate::ast::Pair(this.#binders, current.#binders))); )*
                            Ok(current)
                        },
                        Self::default()
                    )?;

                    #(  this.#binders1 = res.#binders1; )*
                };
                tokens.append_all(token);
            }
            Self::ManyOne(sexpr, match_struct) => {
                let sexpr_string = format!("{sexpr}");
                let binders = match_struct.binders.clone().into_iter();
                let binders1 = match_struct.binders.clone().into_iter();
                let token = quote! {
                    let sexpr = #sexpr_string;
                    let error = format!("expected at least one of {sexpr} {s}");
                    let res = s.fold_to_syntax_list::<(usize, Self), String>(
                        &mut |s, (i, mut current)| {
                            let next_i = if s == crate::ast::Ast::TheEmptyList { 0 } else  { 1 } + i;
                            let mut this = Self::default();
                            #sexpr;
                            #(  current.#binders = crate::ast::Ast::Pair(Box::new(Pair(this.#binders, current.#binders))); )*
                            Ok((next_i, current))
                        },
                        (0, Self::default())
                    )?;

                    if res.0 == 0 {
                        return Err(error)
                    }
                    #(  this.#binders1 = res.1.#binders1; )*
                };
                tokens.append_all(token);
            }
            Self::Any(ident) => {
                let token = quote! {
                    this.#ident = s;
                };
                tokens.append_all(token);
            }
            Self::Symbol(ident) => {
                let ident_string = ident.to_string();
                let token = quote! {
                    let ident = #ident_string;
                    if !matches!(s, crate::ast::Ast::Symbol(_)) {
                        return Err(format!("not an symbol {s} when matching symbol: {ident}"))
                    }
                    this.#ident = s;
                };
                tokens.append_all(token);
            }
            Self::Identifier(ident) => {
                let ident_string = ident.to_string();
                let token = quote! {
                    let ident = #ident_string;
                    if !s.identifier() {
                        return Err(format!("not an identifier {s} when matching identifier: {ident}"))
                    }
                    this.#ident = s;
                };
                tokens.append_all(token);
            }
            Self::Empty => {
                let token = quote! {
                    let s = if let Ast::Syntax(s) = s { s.0 } else { s };
                    if s != crate::ast::Ast::TheEmptyList {
                        return Err(format!("bad syntax expected expected null {s}"))
                    }
                };
                tokens.append_all(token);
            }
            Self::Pair {
                car,
                cdr,
                binders: _,
            } => {
                let this_string = self.to_string();
                let token = quote! {
                    let s = if let Ast::Syntax(s) = s { s.0 } else { s };
                    let this_string = #this_string;
                    if let crate::ast::Ast::Pair(p) = s {
                        let crate::ast::Pair(car, cdr) = *p;
                        {
                            let s = car;
                            #car
                        }
                        {
                            let s = cdr;
                            #cdr
                        }
                    } else {
                        let this_string = #this_string;
                        return Err(format!("not a pair {s} when matching pair: {this_string}"))
                    }
                };
                tokens.append_all(token);
            }
        }
    }
}
#[proc_macro]
pub fn match_syntax_as(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as NameAsSExpr);
    let name = input.0;
    let input = input.1;
    let binders = input.binders().binders.into_iter();
    let binders1 = input.binders().binders.into_iter();
    quote! {
        #[derive(Clone)]
        struct #name {
            #(  #binders: crate::ast::Ast, )*
        }
        impl Default for #name {
            fn default() -> Self {
                Self {
                    #(  #binders1: crate::ast::Ast::TheEmptyList, )*
                }
            }
        }
        impl #name {
            fn matches(s: Ast) -> Result<Self, String> {
                let mut this = Self::default();
                #input
                Ok(this)
            }
        }
        // TODO: somehow just return the type (#name), but doesn't seem to be usable in a type context

    }
    .into()
}
#[proc_macro]
pub fn match_syntax(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as SExpr);
    let binders = input.binders().binders.into_iter();
    let binders1 = input.binders().binders.into_iter();
    let name = syn::Ident::new(&format!("Matcher{}", random::<u64>()), input.span());
    quote! {
        {
            #[derive(Clone)]
            struct #name {
                #(  #binders: crate::ast::Ast, )*
            }
            impl Default for #name {
                fn default() -> Self {
                    Self {
                        #(  #binders1: crate::ast::Ast::TheEmptyList, )*
                    }
                }
            }
            impl #name {
                fn matches(s: Ast) -> Result<Self, String> {
                    let mut this = Self::default();
                    #input
                    Ok(this)
                }
            }
            // TODO: somehow just return the type (#name), but doesn't seem to be usable in a type context
            (#name::matches)
        }
    }
    .into()
}
