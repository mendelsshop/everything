use std::{
    collections::HashSet,
    fmt::{self, Formatter},
};

use crate::{
    ast::{ast4::Ast4, Arg},
    interior_mut::RC,
};

type Label = String;

// https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd45a
macro_rules! hashset {
    ($($element:expr),*) => {{
        // check that count is const
        const C: usize = $crate::count![@COUNT; $($element),*];

        #[allow(unused_mut)]
        let mut vs = HashSet::with_capacity(C);
        $(vs.insert ($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
        $crate::hashset![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };
    (@SUBST; $_element:expr) => { () };
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Linkage {
    Return,
    Next,
    Label(Label),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Register {
    Env,
    Argl,
    Val,
    Proc,
    Continue,
    Thunk,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    LookupVariableValue,
    CompiledProcedureEnv,
    CompiledProcedureEntry,
    DefineVariable,
    ApplyPrimitiveProcedure,
    ExtendEnvironment,
    Cons,
    SetVariableValue,
    False,
    RandomBool,
    MakeCompiledProcedure,
    PrimitiveProcedure,
    VariadiacProcedure,
    MakeThunk,
    ThunkEntry,
    ThunkEnv,
    NotThunk,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Env => write!(f, "env"),
            Self::Argl => write!(f, "argl"),
            Self::Val => write!(f, "val"),
            Self::Proc => write!(f, "proc"),
            Self::Continue => write!(f, "continue"),
            Self::Thunk => write!(f, "thunk"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Const {
    Empty,
    String(String),
    Symbol(String),
    Number(f64),
    Boolean(bool),
    List(Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Const(Const),
    Label(Label),
    Register(Register),
    Op(Perform),
}
impl fmt::Display for Const {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::List(e1, e2) => {
                let mut e2 = *e2.clone();
                write!(f, "{e1}")?;
                while let Expr::Const(Self::List(ne1, ne2)) = e2.clone() {
                    write!(f, " {ne1}",)?;
                    e2 = *ne2;
                }
                if e2 == Expr::Const(Self::Empty) {
                    write!(f, ")")
                } else {
                    write!(f, " . {e2}")
                }
            }
            Self::String(s) => write!(f, "{s}"),
            Self::Symbol(s) => write!(f, "{s}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::Empty => write!(f, "()"),
            Self::Boolean(b) => write!(f, "{b}"),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Const(s) => write!(f, "(const {s})",),
            Self::Label(l) => write!(f, "(label {l})",),
            Self::Register(r) => write!(f, "(reg {r})",),
            Self::Op(p) => write!(f, "{p}",),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Goto {
    Label(Label),
    Register(Register),
}

impl fmt::Display for Goto {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Label(l) => write!(f, "(label {l})"),
            Self::Register(r) => write!(f, "(reg {r})"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
// TODO: consider combining the Operation enum with this so that each operation can declare its arity
pub struct Perform {
    op: Operation,
    args: Vec<Expr>,
}

impl Perform {
    pub fn op(&self) -> &Operation {
        &self.op
    }

    pub fn args(&self) -> &[Expr] {
        self.args.as_ref()
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let decamel = |str: String| {
            str.chars().fold(String::new(), |str, c| {
                if c.is_ascii_uppercase() && !str.is_empty() {
                    str + &format!("-{}", c.to_ascii_lowercase())
                } else {
                    str + &c.to_ascii_lowercase().to_string()
                }
            })
        };
        let kebabified = decamel(format!("{self:?}"));
        match self {
            Self::False | Self::PrimitiveProcedure | Self::NotThunk => write!(f, "{kebabified}?"),
            _ => write!(f, "{kebabified}"),
        }
    }
}
impl fmt::Display for Perform {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(op {}) {}",
            self.op,
            self.args
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Assign(Register, Expr),
    Test(Perform),
    Branch(Label),
    Goto(Goto),
    Save(Register),
    Restore(Register),
    Perform(Perform),
    Label(Label),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(r, e) => write!(f, " (assign {r} {e})"),
            Self::Test(p) => write!(f, " (test {p})",),
            Self::Branch(l) => write!(f, " (branch (label {l}))",),
            Self::Goto(g) => write!(f, " (goto {g})",),
            Self::Save(r) => write!(f, " (save {r})",),
            Self::Restore(r) => write!(f, " (restore {r})",),
            Self::Perform(r) => write!(f, " (perform {r})",),
            Self::Label(l) => write!(f, "{l}",),
        }
    }
}

#[derive(Debug)]
pub struct InstructionSequnce {
    needs: HashSet<Register>,
    modifiers: HashSet<Register>,
    instructions: Vec<Instruction>,
}

impl InstructionSequnce {
    fn new(
        needs: HashSet<Register>,
        modifiers: HashSet<Register>,
        instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            needs,
            modifiers,
            instructions,
        }
    }

    pub fn instructions(&self) -> &[Instruction] {
        self.instructions.as_ref()
    }
}

pub fn compile(
    exp: Ast4,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    match exp {
        Ast4::Ident(i) => compile_variable(i.to_string(), target, linkage),
        Ast4::Application(a) => {
            let compile_application = compile_application(a, target, linkage, lambda_linkage);
            // eprintln!("{compile_application:#?}");
            compile_application
        }
        Ast4::FnParam(i) => compile_variable(format!("'{i}'"), target, linkage),
        Ast4::Begin(b) => compile_seq(b, target, linkage, lambda_linkage),
        Ast4::Define(s, exp) => compile_defeninition((s, *exp), target, linkage, lambda_linkage),
        Ast4::Set(s, exp) => compile_assignment((s, *exp), target, linkage, lambda_linkage),
        Ast4::Lambda(arg, body) => compile_lambda((arg, *body), target, linkage),
        Ast4::If(cond, cons, alt) => {
            compile_if((*cond, *cons, *alt), target, linkage, lambda_linkage)
        }
        Ast4::Quote(q) => compile_quoted(*q, target, linkage),
        Ast4::Label(l) => end_with_linkage(linkage, make_label_instruction(l.to_string())),
        Ast4::Goto(l) => end_with_linkage(
            linkage,
            make_intsruction_sequnce(
                hashset!(),
                hashset!(),
                vec![Instruction::Goto(Goto::Label(l.to_string()))],
            ),
        ),
        Ast4::Stop(s) => append_instruction_sequnce(
            // this only works when we don't modify the continue register, aka any type of call
            // should we even allow return in function calls
            s.map_or_else(
                // default break is hempty, another way to do this is to make stop not an option
                // and when parsing it default to empty
                || {
                    make_intsruction_sequnce(
                        hashset!(),
                        hashset!(target),
                        vec![Instruction::Assign(target, Expr::Const(Const::Empty))],
                    )
                },
                |s| compile(*s, target, Linkage::Next, lambda_linkage),
            ),
            compile_linkage(Linkage::Return),
        ),
        Ast4::Loop(l) => todo!("compile loop"),
        exp => compile_self_evaluating(exp.into(), target, linkage),
    }
}

fn empty_instruction_seqeunce() -> InstructionSequnce {
    InstructionSequnce::new(hashset![], hashset![], vec![])
}

fn compile_linkage(linkage: Linkage) -> InstructionSequnce {
    match linkage {
        Linkage::Return => InstructionSequnce::new(
            hashset![Register::Continue],
            hashset![],
            vec![Instruction::Goto(Goto::Register(Register::Continue))],
        ),
        Linkage::Next => empty_instruction_seqeunce(),
        Linkage::Label(label) => InstructionSequnce::new(
            hashset![],
            hashset![],
            vec![Instruction::Goto(Goto::Label(label))],
        ),
    }
}

fn parallel_instruction_sequnce(
    instruction_seq1: InstructionSequnce,
    instruction_seq2: InstructionSequnce,
) -> InstructionSequnce {
    let mut needs = instruction_seq1.needs;
    needs.extend(instruction_seq2.needs);
    let mut modifiers = instruction_seq1.modifiers;
    modifiers.extend(instruction_seq2.modifiers);
    let mut instructions = instruction_seq1.instructions;
    instructions.extend(instruction_seq2.instructions);
    InstructionSequnce::new(needs, modifiers, instructions)
}

fn append_instruction_sequnce(
    mut seq1: InstructionSequnce,
    seq2: InstructionSequnce,
) -> InstructionSequnce {
    seq1.instructions.extend(seq2.instructions);
    seq1.needs.extend(seq2.needs);
    seq1.modifiers.extend(seq2.modifiers);
    seq1
}

fn preserving(
    register: HashSet<Register>,
    instruction_seq1: InstructionSequnce,
    instruction_seq2: InstructionSequnce,
) -> InstructionSequnce {
    if register.is_empty() {
        append_instruction_sequnce(instruction_seq1, instruction_seq2)
    } else {
        // we collect used registers into vec instead of hashset b/c vec can be reversed so the saves and restores happen in the opposite order
        // ie:
        // save val
        // save env
        // ...
        // restore env
        // restore val
        // instead:
        // save val
        // save env
        // ...
        // restore val
        // restore env
        let used_registers = instruction_seq1
            .modifiers
            .intersection(&instruction_seq2.needs)
            .collect::<Vec<_>>()
            .into_iter();
        let used_registers = used_registers.filter(|r| register.contains(r));

        let mut instructions = vec![];
        instructions.extend(used_registers.clone().map(|r| Instruction::Save(*r)));
        instructions.extend(instruction_seq1.instructions);
        instructions.extend(used_registers.rev().map(|r| Instruction::Restore(*r)));
        instructions.extend(instruction_seq2.instructions);
        let mut needs = instruction_seq1.needs;
        needs.extend(instruction_seq2.needs);
        let mut modifiers = instruction_seq1.modifiers;
        modifiers.extend(instruction_seq2.modifiers);
        InstructionSequnce::new(
            needs.into_iter().collect(),
            modifiers.into_iter().collect(),
            instructions,
        )
    }
}

fn end_with_linkage(
    linkage: Linkage,
    instruction_sequnce: InstructionSequnce,
) -> InstructionSequnce {
    preserving(
        hashset![Register::Continue],
        instruction_sequnce,
        compile_linkage(linkage),
    )
}

fn compile_self_evaluating(exp: Expr, target: Register, linkage: Linkage) -> InstructionSequnce {
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![],
            hashset![target],
            vec![Instruction::Assign(target, exp)],
        ),
    )
}

fn compile_quoted(quoted: Ast4, target: Register, linkage: Linkage) -> InstructionSequnce {
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![],
            hashset![target],
            vec![Instruction::Assign(target, quoted.into())],
        ),
    )
}

static mut LABEL_COUNTER: usize = 0;

fn label_counter() -> usize {
    unsafe {
        LABEL_COUNTER += 1;
        LABEL_COUNTER
    }
}

fn make_label_name(label: String) -> Label {
    format!("{}{}", label, label_counter())
}

fn compile_variable(exp: String, target: Register, linkage: Linkage) -> InstructionSequnce {
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![Register::Env],
            hashset![target],
            vec![Instruction::Assign(
                target,
                Expr::Op(Perform {
                    op: Operation::LookupVariableValue,
                    args: vec![
                        Expr::Const(Const::Symbol(exp)),
                        Expr::Register(Register::Env),
                    ],
                }),
            )],
        ),
    )
}

fn compile_assignment(
    exp: (RC<str>, Ast4),
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    let var = exp.0.to_string();
    let get_value_code = compile(exp.1, Register::Val, Linkage::Next, lambda_linkage);

    end_with_linkage(
        linkage,
        preserving(
            hashset![Register::Env],
            get_value_code,
            InstructionSequnce::new(
                hashset![Register::Env, Register::Val],
                hashset![target],
                vec![
                    Instruction::Perform(Perform {
                        op: Operation::SetVariableValue,
                        args: vec![
                            Expr::Const(Const::Symbol(var)),
                            Expr::Register(Register::Val),
                            Expr::Register(Register::Env),
                        ],
                    }),
                    Instruction::Assign(target, Expr::Const(Const::Symbol("ok".to_string()))),
                ],
            ),
        ),
    )
}

fn compile_defeninition(
    exp: (RC<str>, Ast4),
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    let var = exp.0.to_string();
    let val = compile(exp.1, Register::Val, Linkage::Next, lambda_linkage);

    end_with_linkage(
        linkage,
        preserving(
            hashset![Register::Env],
            val,
            InstructionSequnce::new(
                hashset![Register::Env, Register::Val],
                hashset![target],
                vec![
                    Instruction::Perform(Perform {
                        op: Operation::DefineVariable,
                        args: vec![
                            Expr::Const(Const::Symbol(var)),
                            Expr::Register(Register::Val),
                            Expr::Register(Register::Env),
                        ],
                    }),
                    Instruction::Assign(target, Expr::Const(Const::Symbol("ok".to_string()))),
                ],
            ),
        ),
    )
}

fn compile_if(
    exp: (Ast4, Ast4, Ast4),
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    let t_branch = make_label_name("true-branch".to_string());
    let f_branch = make_label_name("false-branch".to_string());
    let after_if = make_label_name("after-if".to_string());
    let consequent_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_if.clone())
    } else {
        linkage.clone()
    };

    #[cfg(feature = "lazy")]
    let p_code = force_it(exp.0, Register::Val, Linkage::Next, lambda_linkage.clone());
    #[cfg(not(feature = "lazy"))]
    let p_code = compile(exp.0, Register::Val, Linkage::Next, lambda_linkage.clone());

    let c_code = compile(exp.1, target, linkage, lambda_linkage.clone());
    let a_code = compile(exp.2, target, consequent_linkage, lambda_linkage);

    preserving(
        hashset!(Register::Env, Register::Continue),
        p_code,
        append_instruction_sequnce(
            InstructionSequnce::new(
                hashset!(Register::Val),
                hashset!(),
                vec![
                    Instruction::Test(Perform {
                        op: Operation::False,
                        args: vec![Expr::Register(Register::Val)],
                    }),
                    Instruction::Branch(f_branch.clone()),
                ],
            ),
            append_instruction_sequnce(
                parallel_instruction_sequnce(
                    append_instruction_sequnce(make_label_instruction(t_branch), c_code),
                    append_instruction_sequnce(make_label_instruction(f_branch), a_code),
                ),
                make_label_instruction(after_if),
            ),
        ),
    )
}

fn compile_seq(
    seq: Vec<Ast4>,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    let size = seq.len();
    seq.into_iter()
        .enumerate()
        .map(move |(i, exp)| {
            if i == size - 1 {
                compile(exp, target, linkage.clone(), lambda_linkage.clone())
            } else {
                compile(exp, target, Linkage::Next, lambda_linkage.clone())
            }
        })
        .reduce(|a, b| preserving(hashset!(Register::Env, Register::Continue), a, b))
        .unwrap_or_else(empty_instruction_seqeunce)
}

fn tack_on_instruction_seq(
    seq1: InstructionSequnce,
    seq2: InstructionSequnce,
) -> InstructionSequnce {
    make_intsruction_sequnce(
        seq1.needs,
        seq1.modifiers,
        seq1.instructions
            .into_iter()
            .chain(seq2.instructions)
            .collect(),
    )
}

fn compile_lambda(lambda: (Arg, Ast4), target: Register, linkage: Linkage) -> InstructionSequnce {
    let proc_entry = make_label_name("entry".to_string());
    let after_lambda = make_label_name("after-lambda".to_string());
    let lambda_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_lambda.clone())
    } else {
        linkage
    };
    append_instruction_sequnce(
        tack_on_instruction_seq(
            end_with_linkage(
                lambda_linkage.clone(),
                InstructionSequnce::new(
                    hashset!(Register::Env),
                    hashset!(target),
                    vec![Instruction::Assign(
                        target,
                        Expr::Op(Perform {
                            op: Operation::MakeCompiledProcedure,
                            args: vec![
                                Expr::Label(proc_entry.clone()),
                                Expr::Register(Register::Env),
                                Expr::Const(Const::Number(
                                    <Arg as Into<usize>>::into(lambda.0) as f64
                                )),
                            ],
                        }),
                    )],
                ),
            ),
            compile_lambda_body(lambda, proc_entry, lambda_linkage),
        ),
        make_label_instruction(after_lambda),
    )
}

fn compile_lambda_body(
    lambda: (Arg, Ast4),
    proc_entry: String,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    // TODO: do aritty checks by either going through argl and getting the length, having a register that contains the length of the arguments, or combine the 2 together and argl could be a pair of the length of the arguements and the arguements
    append_instruction_sequnce(
        if let Arg::One(i) | Arg::AtLeast0(i) | Arg::AtLeast1(i) = lambda.0 {
            InstructionSequnce::new(
                hashset!(Register::Env, Register::Proc, Register::Argl),
                hashset!(Register::Env),
                vec![
                    Instruction::Label(proc_entry),
                    Instruction::Assign(
                        Register::Env,
                        Expr::Op(Perform {
                            op: Operation::CompiledProcedureEnv,
                            args: vec![Expr::Register(Register::Proc)],
                        }),
                    ),
                    Instruction::Assign(
                        Register::Env,
                        Expr::Op(Perform {
                            op: Operation::ExtendEnvironment,
                            args: vec![
                                Expr::Const(Const::Symbol(format!("'{i}'"))),
                                Expr::Register(Register::Argl),
                                Expr::Register(Register::Env),
                            ],
                        }),
                    ),
                ],
            )
        } else {
            InstructionSequnce::new(
                hashset!(Register::Env, Register::Proc),
                hashset!(Register::Env),
                vec![
                    Instruction::Label(proc_entry),
                    Instruction::Assign(
                        Register::Env,
                        Expr::Op(Perform {
                            op: Operation::CompiledProcedureEnv,
                            args: vec![Expr::Register(Register::Proc)],
                        }),
                    ),
                ],
            )
        },
        compile(lambda.1, Register::Val, Linkage::Return, lambda_linkage),
    )
}

use std::iter::FusedIterator;

#[derive(Clone)]
pub struct MapWithSelf<I, F> {
    iter: I,
    f: F,
}

impl<I: fmt::Debug, F> fmt::Debug for MapWithSelf<I, F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapWithSelf")
            .field("iter", &self.iter)
            .finish()
    }
}

impl<B, I: Iterator + Clone, F> Iterator for MapWithSelf<I, F>
where
    F: FnMut(&I, I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        let iter = self.iter.clone();
        self.iter.next().map(|n| (self.f)(&iter, n))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<B, I: DoubleEndedIterator + Clone, F> DoubleEndedIterator for MapWithSelf<I, F>
where
    F: FnMut(&I, I::Item) -> B,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        self.iter.next_back().map(|n| (self.f)(&self.iter, n))
    }
}

impl<B, I: ExactSizeIterator + Clone, F> ExactSizeIterator for MapWithSelf<I, F>
where
    F: FnMut(&I, I::Item) -> B,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<B, I: FusedIterator + Clone, F> FusedIterator for MapWithSelf<I, F> where
    F: FnMut(&I, I::Item) -> B
{
}

pub trait MapWithSelfExt: Iterator {
    fn map_with_self<F, B>(self, f: F) -> MapWithSelf<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self, Self::Item) -> B,
    {
        MapWithSelf { iter: self, f }
    }
}

impl<I: Iterator> MapWithSelfExt for I {}

fn compile_application(
    exp: Vec<Ast4>,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    #[cfg(feature = "lazy")]
    let proc_code = force_it(
        exp[0].clone(),
        Register::Proc,
        Linkage::Next,
        lambda_linkage.clone(),
    );
    #[cfg(not(feature = "lazy"))]
    let proc_code = compile(
        exp[0].clone(),
        Register::Proc,
        Linkage::Next,
        lambda_linkage.clone(),
    );
    // TODO: make it non strict by essentially turning each argument into zero parameter function and then when we need to unthunk the parameter we just call the function with the env
    let operand_codes_primitive = {
        exp[1..].iter().map(|exp| {
            force_it(
                exp.clone(),
                Register::Val,
                Linkage::Next,
                lambda_linkage.clone(),
            )
        })
        // .map(|exp| {
        //     compile(
        //         exp.clone(),
        //         Register::Val,
        //         Linkage::Next,
        //         lambda_linkage.clone(),
        //     )
        // })
        // .collect()
    };
    #[cfg(feature = "lazy")]
    let operand_codes_compiled = {
        exp[1..].iter().map(|exp| {
            delay_it(
                exp.clone(),
                Register::Val,
                Linkage::Next,
                lambda_linkage.clone(),
            )
        })
        // .map(|exp| {
        //     compile(
        //         exp.clone(),
        //         Register::Val,
        //         Linkage::Next,
        //         lambda_linkage.clone(),
        //     )
        // })
        // .collect()
    };
    operand_codes_primitive
        .zip(operand_codes_compiled)
        .map_with_self(|args, (_, arg)| (args.clone().unzip(), arg))
        .enumerate()
        .fold(
            proc_code,
            |proc, (i, ((primitive_args, compiled_args), arg))| {
                let (comiled_linkage, target_compiled) = if i == exp.len() - 2 {
                    // eprintln!("last arg {:?}", arg);
                    (linkage.clone(), target)
                } else {
                    (Linkage::Next, Register::Proc)
                };
                preserving(
                    hashset!(Register::Continue, Register::Env),
                    // hashset!(Register::Proc, Register::Continue),
                    proc,
                    compile_procedure_call(
                        target,
                        linkage.clone(),
                        primitive_args,
                        #[cfg(feature = "lazy")]
                        compiled_args,
                        #[cfg(feature = "lazy")]
                        arg,
                        target_compiled,
                        comiled_linkage,
                    ),
                )
            },
        )
    // preserving(
    //     hashset!(Register::Continue, Register::Env),
    //     // hashset!(Register::Proc, Register::Continue),
    //     proc_code,
    //     compile_procedure_call(
    //             target,
    //             linkage,
    //             operands.0.into_iter().map(|(fst, _)| fst),
    //             #[cfg(feature = "lazy")]
    //             operand_codes_compiled,
    //         ),
    //     )
}

fn make_label_instruction(label: Label) -> InstructionSequnce {
    InstructionSequnce::new(hashset!(), hashset!(), vec![Instruction::Label(label)])
}

fn make_intsruction_sequnce(
    needs: HashSet<Register>,
    modifies: HashSet<Register>,
    instructions: Vec<Instruction>,
) -> InstructionSequnce {
    InstructionSequnce::new(needs, modifies, instructions)
}
#[cfg(not(feature = "lazy"))]
fn compile_procedure_call(
    target: Register,
    linkage: Linkage,
    operand_codes: Vec<InstructionSequnce>,
) -> InstructionSequnce {
    // TODO: make cfg for lazy so when its not we can just have one set of operand codes
    let primitive_branch = make_label_name("primitive-branch".to_string());
    let compiled_branch = make_label_name("compiled-branch".to_string());
    let after_call = make_label_name("after-call".to_string());
    let compiled_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_call.clone())
    } else {
        linkage.clone()
    };
    preserving(
        hashset!(Register::Proc, Register::Continue),
        construct_arg_list(operand_codes),
        append_instruction_sequnce(
            InstructionSequnce::new(
                hashset!(Register::Proc),
                hashset!(),
                vec![
                    Instruction::Test(Perform {
                        op: Operation::PrimitiveProcedure,
                        args: vec![Expr::Register(Register::Proc)],
                    }),
                    Instruction::Branch(primitive_branch.clone()),
                ],
                // ),
            ),
            parallel_instruction_sequnce(
                append_instruction_sequnce(
                    make_label_instruction(compiled_branch),
                    compile_proc_appl::<Procedure>(target, compiled_linkage),
                ),
                append_instruction_sequnce(
                    make_label_instruction(primitive_branch),
                    // preserving(
                    // hashset!(Register::Proc, Register::Continue),
                    // construct_arg_list(operand_codes_primitive),
                    append_instruction_sequnce(
                        end_with_linkage(
                            linkage,
                            make_intsruction_sequnce(
                                hashset!(Register::Proc, Register::Argl),
                                hashset!(target),
                                vec![Instruction::Assign(
                                    target,
                                    Expr::Op(Perform {
                                        op: Operation::ApplyPrimitiveProcedure,
                                        args: vec![
                                            Expr::Register(Register::Proc),
                                            Expr::Register(Register::Argl),
                                        ],
                                    }),
                                )],
                            ),
                        ),
                        make_label_instruction(after_call),
                        // ),
                    ),
                ),
            ),
        ),
    )
}
#[cfg(feature = "lazy")]
fn compile_procedure_call(
    target: Register,
    linkage: Linkage,
    operand_codes_primitive: Vec<InstructionSequnce>,
    operand_codes_compiled: Vec<InstructionSequnce>,
    arg: InstructionSequnce,
    target_compiled: Register,
    comiled_linkage: Linkage,
) -> InstructionSequnce {
    // TODO: make cfg for lazy so when its not we can just have one set of operand codes
    let primitive_branch = make_label_name("primitive-branch".to_string());
    let normal_branch = make_label_name("normal-branch".to_string());
    let variadiac_branch = make_label_name("variadiac-branch".to_string());
    let compiled_branch = make_label_name("compiled-branch".to_string());
    let after_call = make_label_name("after-call".to_string());
    let compiled_linkage = if comiled_linkage == Linkage::Next {
        Linkage::Label(after_call.clone())
    } else {
        comiled_linkage
    };
    let linkage = if linkage == Linkage::Next {
        Linkage::Label(after_call.clone())
    } else {
        linkage
    };
    // preserving(
    //     hashset!(Register::Proc, Register::Continue),
    //     construct_arg_list(operand_codes_primitive),
    append_instruction_sequnce(
        InstructionSequnce::new(
            hashset!(Register::Proc),
            hashset!(),
            vec![
                Instruction::Test(Perform {
                    op: Operation::PrimitiveProcedure,
                    args: vec![Expr::Register(Register::Proc)],
                }),
                Instruction::Branch(primitive_branch.clone()),
            ],
            // ),
        ),
        parallel_instruction_sequnce(
            append_instruction_sequnce(
                InstructionSequnce::new(
                    hashset!(Register::Proc),
                    hashset!(),
                    vec![
                        Instruction::Label(compiled_branch.clone()),
                        Instruction::Test(Perform {
                            op: Operation::VariadiacProcedure,
                            args: vec![Expr::Register(Register::Proc)],
                        }),
                        Instruction::Branch(variadiac_branch.clone()),
                    ],
                    // ),
                ),
                parallel_instruction_sequnce(
                    append_instruction_sequnce(
                        make_label_instruction(normal_branch),
                        preserving(
                            hashset!(Register::Proc, Register::Continue),
                            preserving(
                                hashset!(Register::Argl),
                                arg,
                                InstructionSequnce::new(
                                    hashset!(Register::Val, Register::Argl),
                                    hashset!(Register::Argl),
                                    vec![Instruction::Assign(
                                        Register::Argl,
                                        Expr::Register(Register::Val),
                                    )],
                                ),
                            ),
                            compile_proc_appl::<Procedure>(target_compiled, compiled_linkage),
                        ),
                    ),
                    append_instruction_sequnce(
                        make_label_instruction(variadiac_branch),
                        preserving(
                            hashset!(Register::Proc, Register::Continue),
                            preserving(
                                hashset!(Register::Argl),
                                construct_arg_list(operand_codes_compiled),
                                InstructionSequnce::new(
                                    hashset!(Register::Val, Register::Argl),
                                    hashset!(Register::Argl),
                                    vec![Instruction::Assign(
                                        Register::Argl,
                                        Expr::Register(Register::Val),
                                    )],
                                ),
                            ),
                            compile_proc_appl::<Procedure>(target, linkage.clone()),
                        ),
                    ),
                ),
            ),
            append_instruction_sequnce(
                make_label_instruction(primitive_branch),
                preserving(
                    hashset!(Register::Proc, Register::Continue),
                    construct_arg_list(operand_codes_primitive),
                    append_instruction_sequnce(
                        end_with_linkage(
                            linkage,
                            make_intsruction_sequnce(
                                hashset!(Register::Proc, Register::Argl),
                                hashset!(target),
                                vec![Instruction::Assign(
                                    target,
                                    Expr::Op(Perform {
                                        op: Operation::ApplyPrimitiveProcedure,
                                        args: vec![
                                            Expr::Register(Register::Proc),
                                            Expr::Register(Register::Argl),
                                        ],
                                    }),
                                )],
                            ),
                        ),
                        make_label_instruction(after_call),
                    ),
                ),
            ),
        ),
        // ),
    )
}

trait Application {
    fn register() -> Register;
    fn entry() -> Operation;
    fn name() -> &'static str;
    fn return_register() -> Register;
}

struct Thunk;
impl Application for Thunk {
    fn entry() -> Operation {
        Operation::ThunkEntry
    }

    fn register() -> Register {
        Register::Thunk
    }

    fn name() -> &'static str {
        "thunk"
    }

    fn return_register() -> Register {
        Register::Thunk
    }
}
struct Procedure;
impl Application for Procedure {
    fn entry() -> Operation {
        Operation::CompiledProcedureEntry
    }

    fn register() -> Register {
        Register::Proc
    }

    fn name() -> &'static str {
        "procedure"
    }

    fn return_register() -> Register {
        Register::Val
    }
}

// the way we differentiate between thunk and procedure is with the T generic we could also do this with const generics
fn compile_proc_appl<T: Application>(
    target: Register,
    compiled_linkage: Linkage,
) -> InstructionSequnce {
    match compiled_linkage {
        Linkage::Return
            // if target == T::return_register() 
 => make_intsruction_sequnce(
            hashset!(T::register(), Register::Continue),
            all_regs(),
            vec![
                Instruction::Assign(
                    Register::Val,
                    Expr::Op(Perform {
                        op: T::entry(),
                        args: vec![Expr::Register(T::register())],
                    }),
                ),
                Instruction::Goto(Goto::Register(Register::Val)),
            ],
        ),
        Linkage::Label(l) if target == T::return_register() => make_intsruction_sequnce(
            hashset!(T::register()),
            all_regs(),
            vec![
                Instruction::Assign(Register::Continue, Expr::Label(l)),
                Instruction::Assign(
                    Register::Val,
                    Expr::Op(Perform {
                        op: T::entry(),
                        args: vec![Expr::Register(T::register())],
                    }),
                ),
                Instruction::Goto(Goto::Register(Register::Val)),
            ],
        ),
        Linkage::Next => unreachable!(),
        Linkage::Return => panic!(
            "return linkage, target not {} -- COMPILE {target}",
            T::return_register()
        ),
        Linkage::Label(l) => {
            let proc_return = make_label_name(format!("{}-return", T::name()));
            make_intsruction_sequnce(
                hashset!(T::register()),
                all_regs(),
                vec![
                    Instruction::Assign(Register::Continue, Expr::Label(proc_return.clone())),
                    Instruction::Assign(
                        Register::Val,
                        Expr::Op(Perform {
                            op: T::entry(),
                            args: vec![Expr::Register(T::register())],
                        }),
                    ),
                    Instruction::Goto(Goto::Register(Register::Val)),
                    Instruction::Label(proc_return),
                    Instruction::Assign(target, Expr::Register(T::return_register())),
                    Instruction::Goto(Goto::Label(l)),
                ],
            )
        }
    }
}

fn all_regs() -> HashSet<Register> {
    hashset!(
        Register::Continue,
        Register::Argl,
        Register::Env,
        Register::Proc,
        Register::Val,
        Register::Thunk
    )
}

fn add_to_argl(inst: InstructionSequnce) -> InstructionSequnce {
    preserving(
        hashset!(Register::Argl),
        inst,
        InstructionSequnce::new(
            hashset!(Register::Val, Register::Argl),
            hashset!(Register::Argl),
            vec![Instruction::Assign(
                Register::Argl,
                Expr::Op(Perform {
                    op: Operation::Cons,
                    args: vec![
                        Expr::Register(Register::Val),
                        Expr::Register(Register::Argl),
                    ],
                }),
            )],
        ),
    )
}

#[cfg(feature = "lazy")]
// #[cfg(not(feature = "lazy"))]
fn force_it(
    exp: Ast4,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    // TODO: when a thunk early returns were do we early return from?
    // it should early return from the last known function: (lambda linkage), so we need to
    // remember the what was in the continue register of before unthunking it (which will modify the
    // continue register, if we have an actual thunk)
    let actual_value_label = make_label_name("actual-value".to_string());
    let force_label = make_label_name("force".to_string());
    let done = make_label_name("done".to_string());
    let thunk = compile(exp, Register::Thunk, Linkage::Next, lambda_linkage);
    let thunk_linkage = if linkage == Linkage::Next {
        Linkage::Label(actual_value_label.clone())
    } else {
        linkage.clone()
    };
    preserving(
        hashset!(Register::Env, Register::Continue),
        append_instruction_sequnce(
            thunk,
            make_intsruction_sequnce(
                hashset!(Register::Thunk),
                hashset!(),
                vec![
                    Instruction::Label(actual_value_label.clone()),
                    Instruction::Test(Perform {
                        op: Operation::NotThunk,
                        args: vec![Expr::Register(Register::Thunk)],
                    }),
                    Instruction::Branch(done.clone()),
                    Instruction::Label(force_label),
                ],
            ),
        ),
        append_instruction_sequnce(
            compile_proc_appl::<Thunk>(Register::Thunk, thunk_linkage),
            make_intsruction_sequnce(
                hashset!(),
                hashset!(),
                vec![
                    Instruction::Label(done),
                    Instruction::Assign(target, Expr::Register(Register::Thunk)),
                ],
            ),
        ),
    )
}

#[cfg(feature = "lazy")]
// #[cfg(not(feature = "lazy"))]
fn delay_it(
    exp: Ast4,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    // TODO: when a thunk early returns were do we early return from?
    // it should early return from the last known function: (lambda linkage), so we need to
    // remember the what was in the continue register of before unthunking it (which will modify the
    // continue register, if we have an actual thunk)
    let thunk_label = make_label_name("thunk".to_string());
    let after_thunk = make_label_name("after-label".to_string());
    let thunk_linkage = if linkage == Linkage::Next {
        Linkage::Label(after_thunk.clone())
    } else {
        linkage
    };
    let mut inst = end_with_linkage(
        thunk_linkage,
        make_intsruction_sequnce(
            hashset!(Register::Env),
            hashset!(target),
            vec![Instruction::Assign(
                target,
                Expr::Op(Perform {
                    op: Operation::MakeThunk,
                    args: vec![
                        Expr::Label(thunk_label.clone()),
                        Expr::Register(Register::Env),
                    ],
                }),
            )],
        ),
    );
    inst.instructions
        .extend(compile_thunk_body(exp, thunk_label, lambda_linkage).instructions);
    inst.instructions.push(Instruction::Label(after_thunk));
    inst
}

#[cfg(feature = "lazy")]
fn compile_thunk_body(
    thunk: Ast4,
    thunk_entry: Label,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    append_instruction_sequnce(
        InstructionSequnce::new(
            hashset!(Register::Env, Register::Thunk),
            hashset!(Register::Env),
            vec![
                Instruction::Label(thunk_entry.clone()),
                Instruction::Assign(
                    Register::Env,
                    Expr::Op(Perform {
                        op: Operation::ThunkEnv,
                        args: vec![Expr::Register(Register::Thunk)],
                    }),
                ),
            ],
        ),
        compile(thunk, Register::Thunk, Linkage::Return, lambda_linkage),
    )
}
// #[cfg(feature = "lazy")]
#[cfg(not(feature = "lazy"))]
fn delay_it(
    exp: Ast4,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    compile(exp, target, linkage, lambda_linkage)
}

// #[cfg(feature = "lazy")]
#[cfg(not(feature = "lazy"))]
fn force_it(
    exp: Ast4,
    target: Register,
    linkage: Linkage,
    lambda_linkage: Linkage,
) -> InstructionSequnce {
    compile(exp, target, linkage, lambda_linkage)
}
fn construct_arg_list(operand_codes: Vec<InstructionSequnce>) -> InstructionSequnce {
    operand_codes
        .into_iter()
        // .map(delay_it)
        .map(add_to_argl)
        .rev()
        .fold(
            InstructionSequnce::new(
                hashset!(),
                hashset!(Register::Argl),
                vec![Instruction::Assign(
                    Register::Argl,
                    Expr::Const(Const::Empty),
                )],
            ),
            |a, b| preserving(hashset!(Register::Env), a, b),
        )
}

impl From<Ast4> for Expr {
    fn from(value: Ast4) -> Self {
        match value {
            Ast4::Bool(b) => match b {
                crate::ast::Boolean::False => Self::Const(Const::Boolean(false)),
                crate::ast::Boolean::True => Self::Const(Const::Boolean(true)),
                crate::ast::Boolean::Maybee => Self::Op(Perform {
                    op: Operation::RandomBool,
                    args: vec![],
                }),
            },
            Ast4::Number(n) => Self::Const(Const::Number(n)),
            Ast4::String(s) => Self::Const(Const::String(s.to_string())),
            Ast4::Ident(i) => Self::Const(Const::Symbol(i.to_string())),
            Ast4::Application(a) => a
                .into_iter()
                .map(Into::into)
                .rfold(Self::Const(Const::Empty), |a, b| {
                    Self::Const(Const::List(Box::new(b), Box::new(a)))
                }),
            Ast4::FnParam(i) => Self::Const(Const::Symbol(format!("'{i}'"))),
            _ => unreachable!(),
        }
    }
}
