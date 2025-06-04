use std::{
    collections::HashSet,
    fmt::{self, Formatter},
    rc::Rc,
};

use itertools::Itertools;
use log::info;

use crate::ast::{ast2::Ast2, Ast, Param, Symbol};

type Label = String;

// https://gist.github.com/jonhoo/ec57882a976a2d2a92b3138ea25cd25a
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
    // true mean that expects one value, maybe instead of boolean have optional label, to jump to
    Next { expect_single: bool },
    Label { place: Label, expect_single: bool },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Register {
    Env,
    Argl,
    Val,
    Proc,
    Continue,
    ContinueMulti,
    Values,
    Thunk,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    // some of the operations inputs are "static" so they could be put out of the perform and into
    // the operation of adt
    LookupVariableValue,
    CompiledProcedureEnv,
    CompiledProcedureEntry,
    DefineVariable(Vec<Rc<str>>),
    ApplyPrimitiveProcedure,
    ExtendEnvironment,
    Cons,
    SetVariableValue,
    False,
    MakeCompiledProcedure,
    PrimitiveProcedure,
    VariadiacProcedure,
    MakeThunk,
    ThunkEntry,
    ThunkEnv,
    NotThunk,
    NotStop,
    ResetStop,
    SetStop,
    // maybe combine with define variables - rn only place it is used
    SetSingleMultiValueHandler,
    // maybe combine with extendenv
    NewEnvironment,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Env => write!(f, "env"),
            Self::Argl => write!(f, "argl"),
            Self::Val => write!(f, "val"),
            Self::Proc => write!(f, "proc"),
            Self::Continue => write!(f, "continue"),
            Self::ContinueMulti => write!(f, "continue-multi"),
            Self::Thunk => write!(f, "thunk"),
            Self::Values => write!(f, "values"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    // techincally should be ast - function (and maybe - label)
    Const(Ast),
    Label(Label),
    Register(Register),
    Op(Perform),
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
    pub const fn op(&self) -> &Operation {
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
            Self::False | Self::PrimitiveProcedure | Self::NotThunk | Self::NotStop => {
                write!(f, "{kebabified}?")
            }
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
    AssignError(Register, &'static str),
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign(r, e) => write!(f, " (assign {r} {e})"),
            Self::AssignError(r, e) => write!(f, " (assign error to {r} with message {e})"),
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
    const fn new(
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

pub fn compile(exp: Ast2, target: Register, linkage: Linkage) -> InstructionSequnce {
    match exp {
        Ast2::Basic(Ast::Symbol(i)) => compile_variable(i, target, linkage),
        Ast2::Application(f, a) => {
            // eprintln!("{compile_application:#?}");
            compile_application(*f, a, target, linkage)
        }
        Ast2::Begin(b) => compile_seq(b, target, linkage),
        Ast2::DefineValues(s, exp) => compile_defeninition(s, *exp, target, linkage),
        Ast2::Set(s, exp) => compile_assignment((s, *exp), target, linkage),
        Ast2::Lambda(arg, body) => compile_lambda((arg, *body), target, linkage),
        Ast2::If(cond, cons, alt) => compile_if((*cond, *cons, *alt), target, linkage),
        Ast2::Quote(q) => compile_quoted(q, target, linkage),
        Ast2::Basic(Ast::Label(l)) => {
            end_with_linkage(linkage, make_label_instruction(l.to_string()))
        }
        Ast2::Goto(l) => end_with_linkage(
            linkage,
            make_intsruction_sequnce(
                hashset!(),
                hashset!(),
                vec![Instruction::Goto(Goto::Label(l.0.to_string()))],
            ),
        ),
        Ast2::Stop(s) => append_instruction_sequnce(
            make_intsruction_sequnce(
                hashset!(),
                hashset!(),
                vec![Instruction::Perform(Perform {
                    op: Operation::SetStop,
                    args: vec![],
                })],
            ),
            append_instruction_sequnce(
                // this only works when we don't modify the continue register, aka any type of call
                // should we even allow return in function calls
                s.map_or_else(
                    // default break is hempty, another way to do this is to make stop not an option
                    // and when parsing it default to empty
                    || {
                        make_intsruction_sequnce(
                            hashset!(),
                            hashset!(target),
                            vec![Instruction::Assign(target, Expr::Const(Ast::TheEmptyList))],
                        )
                    },
                    |s| {
                        compile(
                            *s,
                            target,
                            Linkage::Next {
                                expect_single: true,
                            },
                        )
                    },
                ),
                compile_linkage(Linkage::Return),
            ),
        ),
        Ast2::Loop(loop_function) => compile_loop(loop_function, target, linkage),
        Ast2::Module(name, kind) => todo!("compile module refrence"),
        Ast2::Basic(exp) => compile_self_evaluating(exp.into(), target, linkage),
        Ast2::LetValues(items, ast2) => todo!(),
        Ast2::LetRecValues(variables, body) => compile_let_rec(variables, body, target, linkage),

        Ast2::Expression(ast2) => todo!(),
        Ast2::Begin0(ast2s) => todo!(),
        Ast2::Skip => todo!(),
    }
}

fn compile_let_rec(
    variables: Vec<(Vec<Rc<str>>, Ast2)>,
    body: Box<Ast2>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    // make new envoirment

    // go through list of variables, for each set of variables:
    // eval current variable into multi value register (need to use SetSingleMultiValueHanlder)
    // define current list of variables with multi value register in new envoirment
    let env = make_intsruction_sequnce(
        hashset!(Register::Env),
        hashset!(Register::Values, Register::Env),
        vec![Instruction::Assign(
            Register::Env,
            Expr::Op(Perform {
                op: Operation::NewEnvironment,
                args: vec![Expr::Register(Register::Env)],
            }),
        )],
    );
    let variables = variables
        .into_iter()
        .map(|(v, values)| compile_defeninition_let(v, values))
        .fold(env, |a, b| preserving(hashset!(), a, b));

    preserving(
        hashset!(Register::Continue, Register::ContinueMulti),
        variables,
        // eval body in new env
        // go back to original env
        compile(*body, target, linkage),
    )
}

fn compile_loop(
    loop_function: Box<Ast2>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    // TODO: should we use an itermidiate register to hold loop_function
    info!(
        "generating ir for loop {loop_function:?}, with register {target}, with linkage {linkage:?}",

    );
    let loop_function = compile(
        *loop_function,
        Register::Proc,
        Linkage::Next {
            expect_single: true,
        },
    );
    let loop_label = make_label_name("loop".to_owned());
    let after_loop_label = make_label_name("after_loop".to_owned());
    let done_label = make_label_name("done".to_owned());
    end_with_linkage(
        linkage,
        append_instruction_sequnce(
            loop_function,
            append_instruction_sequnce(
                make_intsruction_sequnce(
                    hashset!(Register::Continue),
                    hashset!(Register::Continue),
                    vec![
                        Instruction::Assign(
                            Register::Continue,
                            Expr::Label(after_loop_label.clone()),
                        ),
                        Instruction::Label(loop_label.clone()),
                    ],
                ),
                preserving(
                    hashset!(Register::Continue, Register::Proc),
                    // we don't actually care about result of application so we throw it in target
                    // which is overwritten when loop is done
                    compile_procedure_call_loop(
                        target,
                        Linkage::Next {
                            expect_single: false,
                        },
                    ),
                    make_intsruction_sequnce(
                        hashset!(Register::Continue, Register::Proc),
                        hashset!(target),
                        vec![
                            Instruction::Label(after_loop_label),
                            Instruction::Test(Perform {
                                op: Operation::NotStop,
                                args: vec![],
                            }),
                            Instruction::Branch(loop_label),
                            Instruction::Label(done_label),
                            // TODO: do we need to save and restore stop in case of recursion
                            // in that case delegate to register
                            Instruction::Perform(Perform {
                                op: Operation::ResetStop,
                                args: vec![],
                            }),
                            Instruction::Assign(target, Expr::Register(Register::Val)),
                        ],
                    ),
                ),
            ),
        ),
    )
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
        Linkage::Next { expect_single: _ } => empty_instruction_seqeunce(),
        // we are assuming this is only being used from single value context
        Linkage::Label {
            place: label,
            expect_single: _,
        } => InstructionSequnce::new(
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
        hashset![Register::Continue, Register::ContinueMulti],
        instruction_sequnce,
        compile_linkage(linkage),
    )
}

fn compile_self_evaluating(exp: Expr, target: Register, linkage: Linkage) -> InstructionSequnce {
    info!("generating ir for primitive expression {exp:?}, with register {target}, with linkage {linkage:?}");
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![],
            hashset![target],
            vec![Instruction::Assign(target, exp)],
        ),
    )
}

fn compile_quoted(quoted: Ast, target: Register, linkage: Linkage) -> InstructionSequnce {
    info!("generating ir for quoted expression {quoted:?}, with register {target}, with linkage {linkage:?}");
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

fn compile_variable(exp: Symbol, target: Register, linkage: Linkage) -> InstructionSequnce {
    info!("generating ir for looking up variable {exp:?}, with register {target}, with linkage {linkage:?}");
    end_with_linkage(
        linkage,
        InstructionSequnce::new(
            hashset![Register::Env],
            hashset![target],
            vec![Instruction::Assign(
                target,
                Expr::Op(Perform {
                    op: Operation::LookupVariableValue,
                    args: vec![Expr::Const(Ast::Symbol(exp)), Expr::Register(Register::Env)],
                }),
            )],
        ),
    )
}

fn compile_assignment(
    exp: (Rc<str>, Ast2),
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    info!(
        "generating ir for assigning {:?} to {}, with register {target}, with linkage {linkage:?}",
        exp.1, exp.0
    );
    let var = exp.0;
    let get_value_code = compile(
        exp.1,
        Register::Val,
        Linkage::Next {
            expect_single: true,
        },
    );

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
                            Expr::Const(Ast::Symbol(Symbol(var))),
                            Expr::Register(Register::Val),
                            Expr::Register(Register::Env),
                        ],
                    }),
                    // TODO: make the a zero value
                    Instruction::Assign(target, Expr::Const(Ast::Symbol("ok".into()))),
                ],
            ),
        ),
    )
}

fn compile_defeninition_let(variables: Vec<Rc<str>>, value: Ast2) -> InstructionSequnce {
    let val = compile(
        value,
        Register::Val,
        Linkage::Next {
            expect_single: false,
        },
    );

    let label = make_label_name("mv".to_string());
    preserving(
        hashset![Register::Env],
        append_instruction_sequnce(
            make_intsruction_sequnce(
                hashset!(),
                hashset!(Register::Continue, Register::ContinueMulti),
                vec![
                    Instruction::Assign(Register::ContinueMulti, Expr::Label(label.clone())),
                    Instruction::Perform(Perform {
                        op: Operation::SetSingleMultiValueHandler,
                        args: vec![],
                    }),
                ],
            ),
            // TODO: might need to presereve single or multi continue register
            val,
        ),
        InstructionSequnce::new(
            hashset![
                Register::Env,
                Register::Val,
                Register::Continue,
                Register::ContinueMulti
            ],
            hashset![],
            vec![
                Instruction::Label(label),
                Instruction::Perform(Perform {
                    op: Operation::DefineVariable(variables),
                    args: vec![Expr::Register(Register::Val), Expr::Register(Register::Env)],
                }),
            ],
        ),
    )
}
fn compile_defeninition(
    variables: Vec<Rc<str>>,
    value: Ast2,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    info!(
        "generating ir for assigning {value} to {variables:?}, with register {target}, with linkage {linkage:?}"
    );
    let val = compile(
        value,
        Register::Val,
        Linkage::Next {
            expect_single: false,
        },
    );

    let label = make_label_name("mv".to_string());
    end_with_linkage(
        linkage,
        preserving(
            hashset![Register::Env],
            append_instruction_sequnce(
                make_intsruction_sequnce(
                    hashset!(),
                    hashset!(),
                    vec![Instruction::Perform(Perform {
                        op: Operation::SetSingleMultiValueHandler,
                        args: vec![Expr::Label(label.clone())],
                    })],
                ),
                val,
            ),
            InstructionSequnce::new(
                hashset![Register::Env, Register::Val],
                hashset![target],
                vec![
                    Instruction::Label(label),
                    Instruction::Perform(Perform {
                        op: Operation::DefineVariable(variables),
                        args: vec![Expr::Register(Register::Val), Expr::Register(Register::Env)],
                    }),
                    Instruction::Assign(target, Expr::Const(Ast::Symbol("ok".into()))),
                ],
            ),
        ),
    )
}

fn compile_if(exp: (Ast2, Ast2, Ast2), target: Register, linkage: Linkage) -> InstructionSequnce {
    info!("generating ir for if expresion (condition) {:?} (consequent) {:?} (alternative) {:?}, with register {target}, with linkage {linkage:?}", exp.0, exp.1, exp.2);
    let t_branch = make_label_name("true-branch".to_string());
    let f_branch = make_label_name("false-branch".to_string());
    let after_if = make_label_name("after-if".to_string());
    let consequent_linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: after_if.clone(),
            expect_single,
        }
    } else {
        linkage.clone()
    };

    #[cfg(feature = "lazy")]
    let p_code = force_it(
        exp.0,
        Register::Val,
        Linkage::Next {
            expect_single: true,
        },
    );
    #[cfg(not(feature = "lazy"))]
    let p_code = compile(
        exp.0,
        Register::Val,
        Linkage::Next {
            expect_single: true,
        },
    );

    let c_code = compile(exp.1, target, linkage);
    let a_code = compile(exp.2, target, consequent_linkage);

    preserving(
        hashset!(Register::Env, Register::Continue, Register::ContinueMulti),
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

fn compile_seq(seq: Vec<Ast2>, target: Register, linkage: Linkage) -> InstructionSequnce {
    info!(
        "generating ir for begin with expressions {:?}, with register {target}, with linkage {linkage:?}",
        seq.iter().map(|e|format!("{e:?}")).join(" ")
    );
    let size = seq.len();
    seq.into_iter()
        .enumerate()
        .map(move |(i, exp)| {
            if i == size - 1 {
                compile(exp, target, linkage.clone())
            } else {
                compile(
                    exp,
                    target,
                    Linkage::Next {
                        expect_single: false,
                    },
                )
            }
        })
        .reduce(|a, b| {
            preserving(
                hashset!(Register::Env, Register::Continue, Register::ContinueMulti),
                a,
                b,
            )
        })
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

fn compile_lambda(lambda: (Param, Ast2), target: Register, linkage: Linkage) -> InstructionSequnce {
    info!(
        "generating ir for lambda with parameter {} and body {:?}, with register {target}, with linkage {linkage:?}",
        lambda.0, lambda.1
    );
    let proc_entry = make_label_name("entry".to_string());
    let after_lambda = make_label_name("after-lambda".to_string());
    let lambda_linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: after_lambda.clone(),
            expect_single,
        }
    } else {
        linkage
    };
    append_instruction_sequnce(
        tack_on_instruction_seq(
            end_with_linkage(
                lambda_linkage,
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
                                // TODO: update llvm generation code to use symbols as opposed to
                                // numbers
                                Expr::Const(Ast::Number(
                                    <&Param as Into<usize>>::into(&lambda.0) as f64
                                )),
                            ],
                        }),
                    )],
                ),
            ),
            compile_lambda_body(lambda, proc_entry),
        ),
        make_label_instruction(after_lambda),
    )
}

fn compile_lambda_body(lambda: (Param, Ast2), proc_entry: String) -> InstructionSequnce {
    // TODO: do aritty checks by either going through argl and getting the length, having a register that contains the length of the arguments, or combine the 2 together and argl could be a pair of the length of the arguements and the arguements
    append_instruction_sequnce(
        if let Param::One(i) | Param::AtLeast0(i) | Param::AtLeast1(i) = lambda.0 {
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
                                Expr::Const(Ast::Symbol(Symbol(i))),
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
                            // TODO: probably need to create new scope in env
                        }),
                    ),
                ],
            )
        },
        compile(lambda.1, Register::Val, Linkage::Return),
    )
}
// map with self is used compiling applications:
// because when we compiling applications we do not know if we have to fully apply
// or not if we have to fully apply then we want whatever is left (the self part) (this is for
// variadic and primitives now)
// otherwise in standard application (and maybe eventually primitives that are non variadic) we
// just need the current element like a normal iterator
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
    F: FnMut(I, I::Item) -> B,
{
    type Item = B;

    #[inline]
    fn next(&mut self) -> Option<B> {
        let this = self.iter.clone();
        self.iter.next().map(|n| (self.f)(this, n))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<B, I: DoubleEndedIterator + Clone, F> DoubleEndedIterator for MapWithSelf<I, F>
where
    F: FnMut(I, I::Item) -> B,
{
    #[inline]
    fn next_back(&mut self) -> Option<B> {
        let this = self.iter.clone();
        self.iter.next_back().map(|n| (self.f)(this, n))
    }
}

impl<B, I: ExactSizeIterator + Clone, F> ExactSizeIterator for MapWithSelf<I, F>
where
    F: FnMut(I, I::Item) -> B,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<B, I: FusedIterator + Clone, F> FusedIterator for MapWithSelf<I, F> where
    F: FnMut(I, I::Item) -> B
{
}

pub trait MapWithSelfExt: Iterator {
    fn map_with_self<F, B>(self, f: F) -> MapWithSelf<Self, F>
    where
        Self: Sized,
        F: FnMut(Self, Self::Item) -> B,
    {
        MapWithSelf { iter: self, f }
    }
}
impl<I: Iterator> MapWithSelfExt for I {}
#[cfg(not(feature = "lazy"))]
fn compile_application(
    f: Ast2,
    exp: Vec<Ast2>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    let proc_code = compile(
        f,
        Register::Proc,
        Linkage::Next {
            expect_single: true,
        },
    );
    let arg_count = exp.len();
    let operand_codes = exp.into_iter();

    let after_full_call = make_label_name("after-full-call".to_string());
    append_instruction_sequnce(
        operand_codes
            .map_with_self(|this, rest| (this, rest))
            .enumerate()
            .fold(proc_code, |proc, (i, (args, arg))| {
                let (compiled_linkage, target_compiled) = if i + 1 == arg_count {
                    (linkage.clone(), target)
                } else {
                    (
                        Linkage::Next {
                            expect_single: true,
                        },
                        Register::Proc,
                    )
                };
                preserving(
                    hashset!(Register::Continue, Register::ContinueMulti, Register::Env),
                    // hashset!(Register::Proc, Register::Continue),
                    proc,
                    compile_procedure_call(
                        target,
                        linkage.clone(),
                        target_compiled,
                        compiled_linkage,
                        arg,
                        args,
                        |e| {
                            compile(
                                e,
                                Register::Val,
                                Linkage::Next {
                                    expect_single: true,
                                },
                            )
                        },
                        after_full_call.clone(),
                    ),
                )
            }),
        make_label_instruction(after_full_call),
    )
}

#[cfg(feature = "lazy")]
fn compile_application(
    f: Ast2,
    exp: Vec<Ast2>,
    target: Register,
    linkage: Linkage,
) -> InstructionSequnce {
    #[cfg(feature = "lazy")]
    info!(
        "generating ir for application with expressions {:?}, with register {target}, with linkage {linkage:?}",
        exp.iter().map(|e|format!("{e:?}")).join(" ")
    );
    info!("generating ir for applications' function");
    let proc_code = force_it(
        f,
        Register::Proc,
        Linkage::Next {
            expect_single: true,
        },
    );

    let len = exp.len();
    // TODO: make it non strict by essentially turning each argument into zero parameter function and then when we need to unthunk the parameter we just call the function with the env
    let operands = exp.into_iter();

    let after_full_call = make_label_name("after-full-call".to_string());
    append_instruction_sequnce(
        operands
            .map_with_self(|this, rest| (this, rest))
            .enumerate()
            .fold(proc_code, |proc, (i, (args, arg))| {
                let (comiled_linkage, target_compiled) = if i + 1 == len {
                    // eprintln!("last arg {:?}", arg);
                    (linkage.clone(), target)
                } else {
                    (
                        Linkage::Next {
                            expect_single: true,
                        },
                        Register::Proc,
                    )
                };
                preserving(
                    hashset!(Register::Continue, Register::ContinueMulti, Register::Env),
                    // hashset!(Register::Proc, Register::Continue),
                    proc,
                    compile_procedure_call(
                        target,
                        linkage.clone(),
                        args,
                        arg,
                        target_compiled,
                        comiled_linkage,
                        |e| {
                            force_it(
                                e,
                                Register::Val,
                                Linkage::Next {
                                    expect_single: true,
                                },
                            )
                        },
                        |e| {
                            delay_it(
                                e,
                                Register::Val,
                                Linkage::Next {
                                    expect_single: true,
                                },
                            )
                        },
                        after_full_call.clone(),
                    ),
                )
            }),
        make_label_instruction(after_full_call),
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

const fn make_intsruction_sequnce(
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
    intermediary_target: Register,
    intermediary_linkage: Linkage,
    intermediary_arg: Ast2,
    args: impl DoubleEndedIterator<Item = Ast2> + Clone,
    compile: fn(Ast2) -> InstructionSequnce,
    after_full_call: Label,
) -> InstructionSequnce {
    // TODO: make cfg for lazy so when its not we can just have one set of operand codes
    let primitive_branch = make_label_name("primitive-branch".to_string());
    let normal_branch = make_label_name("normal-branch".to_string());
    let compiled_branch = make_label_name("compiled-branch".to_string());
    let variadiac_branch = make_label_name("variadiac-branch".to_string());
    let after_call = make_label_name("after-call".to_string());
    let compiled_linkage = if let Linkage::Next { expect_single } = intermediary_linkage {
        Linkage::Label {
            place: after_call.clone(),
            expect_single,
        }
    } else {
        linkage.clone()
    };
    let linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: after_full_call,
            expect_single,
        }
    } else {
        linkage
    };

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
        ),
        parallel_instruction_sequnce(
            append_instruction_sequnce(
                InstructionSequnce::new(
                    hashset!(Register::Proc),
                    hashset!(),
                    vec![
                        Instruction::Label(compiled_branch),
                        Instruction::Test(Perform {
                            op: Operation::VariadiacProcedure,
                            args: vec![Expr::Register(Register::Proc)],
                        }),
                        Instruction::Branch(variadiac_branch.clone()),
                    ],
                ),
                parallel_instruction_sequnce(
                    append_instruction_sequnce(
                        make_label_instruction(normal_branch),
                        preserving(
                            hashset!(Register::Proc, Register::Continue, Register::ContinueMulti),
                            preserving(
                                hashset!(Register::Argl),
                                compile(intermediary_arg),
                                InstructionSequnce::new(
                                    hashset!(Register::Val, Register::Argl),
                                    hashset!(Register::Argl),
                                    // we do not need to do any consing because we only take on
                                    // a single argument, and the way extend env works is that it
                                    // only expects one value and one symbol because user defined
                                    // functions only take on arguement, the only difference for
                                    // variadiacs is at the call site
                                    vec![Instruction::Assign(
                                        Register::Argl,
                                        Expr::Register(Register::Val),
                                    )],
                                ),
                            ),
                            compile_proc_appl::<Procedure>(intermediary_target, compiled_linkage),
                        ),
                    ),
                    append_instruction_sequnce(
                        make_label_instruction(variadiac_branch),
                        preserving(
                            hashset!(Register::Proc, Register::Continue, Register::ContinueMulti),
                            construct_arg_list(args.clone().map(compile)),
                            compile_proc_appl::<Procedure>(target, linkage.clone()),
                        ),
                    ),
                ),
            ),
            append_instruction_sequnce(
                // primitive branch uses end with linkage which assumes single values
                // we have to inform the primitve about the two jumping place (through the registes
                // continue/continue-multi or by providing apply primitive procedure two registers)
                // instead of implicitly jumping to a single value, point
                // it would be up to the primtive to decide which one to use
                // similar to what we do in compile_proc_appl, speciically setting the two
                // register, and letting the called function decide where to jump back to
                make_label_instruction(primitive_branch),
                preserving(
                    hashset!(Register::Proc, Register::Continue, Register::ContinueMulti),
                    construct_arg_list(args.map(compile)),
                    append_instruction_sequnce(
                        compile_proc_appl_end::<Procedure>(
                            Procedure::return_register(),
                            linkage,
                            make_intsruction_sequnce(
                                hashset!(Register::Proc, Register::Argl),
                                // calls could modify the continue(-multi) registers
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
#[cfg(feature = "lazy")]
fn compile_procedure_call(
    target: Register,
    linkage: Linkage,
    operands: impl DoubleEndedIterator<Item = Ast2> + Clone,
    arg: Ast2,
    target_compiled: Register,
    comiled_linkage: Linkage,
    compile_primitive: fn(Ast2) -> InstructionSequnce,
    compile_compiled: fn(Ast2) -> InstructionSequnce,
    after_full_call: Label,
) -> InstructionSequnce {
    // TODO: make cfg for lazy so when its not we can just have one set of operand codes
    let primitive_branch = make_label_name("primitive-branch".to_string());
    let normal_branch = make_label_name("normal-branch".to_string());
    let variadiac_branch = make_label_name("variadiac-branch".to_string());
    let compiled_branch = make_label_name("compiled-branch".to_string());
    let after_call = make_label_name("after-call".to_string());
    let compiled_linkage = if let Linkage::Next { expect_single } = comiled_linkage {
        Linkage::Label {
            place: after_call.clone(),
            expect_single,
        }
    } else {
        linkage.clone()
    };
    let linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: after_full_call,
            expect_single,
        }
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
                        Instruction::Label(compiled_branch),
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
                            hashset!(Register::Proc, Register::Continue, Register::ContinueMulti),
                            preserving(
                                hashset!(Register::Argl),
                                compile_compiled(arg),
                                InstructionSequnce::new(
                                    hashset!(Register::Val, Register::Argl),
                                    hashset!(Register::Argl),
                                    // we do not need to do any consing because we only take on
                                    // a single argument, and the way extend env works is that it
                                    // only expects one value and one symbol because user defined
                                    // functions only take on arguement, the only difference for
                                    // variadiacs is at the call site
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
                            hashset!(Register::Proc, Register::Continue, Register::ContinueMulti),
                            construct_arg_list(operands.clone().map(compile_compiled)),
                            compile_proc_appl::<Procedure>(target, linkage.clone()),
                        ),
                    ),
                ),
            ),
            append_instruction_sequnce(
                make_label_instruction(primitive_branch),
                preserving(
                    hashset!(Register::Proc, Register::Continue, Register::ContinueMulti),
                    construct_arg_list(operands.map(compile_primitive)),
                    append_instruction_sequnce(
                        end_with_linkage(
                            linkage.clone(),
                            compile_proc_appl_end::<Procedure>(
                                Procedure::register(),
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
                        ),
                        make_label_instruction(after_call),
                    ),
                ),
            ),
        ),
        // ),
    )
}
// this is special function caller for loops where functions for loop are zero arg
fn compile_procedure_call_loop(target: Register, linkage: Linkage) -> InstructionSequnce {
    info!("generating ir for loop body function call");
    // TODO: make cfg for lazy so when its not we can just have one set of operand codes
    let primitive_branch = make_label_name("primitive-branch".to_string());
    let compiled_branch = make_label_name("compiled-branch".to_string());
    let after_call = make_label_name("after-call".to_string());
    let linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: after_call.clone(),
            expect_single,
        }
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
                    vec![Instruction::Label(compiled_branch)],
                    // ),
                ),
                compile_proc_appl::<Procedure>(target, linkage.clone()),
            ),
            append_instruction_sequnce(
                make_label_instruction(primitive_branch),
                append_instruction_sequnce(
                    // maybe don't need to end with linkage as it should be up to the primitive
                    // procedure to do the jumping
                    compile_proc_appl_end::<Procedure>(
                        Procedure::register(),
                        linkage,
                        make_intsruction_sequnce(
                            hashset!(Register::Proc, Register::Argl),
                            // calls could modify the continue(-multi) registers
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
    application_code: InstructionSequnce,
) -> InstructionSequnce {
    match compiled_linkage {
        // if target == T::return_register()
        Linkage::Return => append_instruction_sequnce(
            make_intsruction_sequnce(
                hashset!(Register::ContinueMulti, Register::Continue),
                hashset!(),
                vec![],
            ),
            application_code,
        ),
        Linkage::Label {
            place: l,
            expect_single,
        } if target == T::return_register() => append_instruction_sequnce(
            make_intsruction_sequnce(
                hashset!(),
                hashset!(Register::ContinueMulti, Register::Continue),
                vec![
                    Instruction::Assign(Register::Continue, Expr::Label(l.clone())),
                    if expect_single {
                        Instruction::AssignError(
                            Register::ContinueMulti,
                            "not expecting multiple values",
                        )
                    } else {
                        Instruction::Assign(Register::ContinueMulti, Expr::Label(l))
                    },
                ],
            ),
            application_code,
        ),
        Linkage::Return => panic!(
            "return linkage, target not {} -- COMPILE {target}",
            T::return_register()
        ),
        Linkage::Label {
            place: l,
            expect_single,
        } => {
            let proc_return = make_label_name(format!("{}-return", T::name()));
            append_instruction_sequnce(
                make_intsruction_sequnce(
                    hashset!(),
                    hashset!(Register::ContinueMulti, Register::Continue),
                    vec![
                        Instruction::Assign(Register::Continue, Expr::Label(proc_return.clone())),
                        if expect_single {
                            Instruction::AssignError(
                                Register::ContinueMulti,
                                "not expecting multiple values",
                            )
                        } else {
                            Instruction::Assign(
                                Register::ContinueMulti,
                                Expr::Label(proc_return.clone()),
                            )
                        },
                    ],
                ),
                append_instruction_sequnce(
                    application_code,
                    make_intsruction_sequnce(
                        hashset!(),
                        hashset!(target),
                        vec![
                            Instruction::Label(proc_return),
                            Instruction::Assign(target, Expr::Register(T::return_register())),
                            Instruction::Goto(Goto::Label(l)),
                        ],
                    ),
                ),
            )
        }
        Linkage::Next { expect_single } => unreachable!(),
    }
}
// the way we differentiate between thunk and procedure is with the T generic we could also do this with const generics
fn compile_proc_appl<T: Application>(
    target: Register,
    compiled_linkage: Linkage,
) -> InstructionSequnce {
    let application_code = make_intsruction_sequnce(
        hashset!(T::register()),
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
    );
    compile_proc_appl_end::<T>(target, compiled_linkage, application_code)
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
fn force_it(exp: Ast2, target: Register, linkage: Linkage) -> InstructionSequnce {
    info!(
        "generating ir for forcing expression {exp:?}, with register {target}, with linkage {linkage:?}"
    );
    // TODO: when a thunk early returns were do we early return from?
    // it should early return from the last known function: (lambda linkage), so we need to
    // remember the what was in the continue register of before unthunking it (which will modify the
    // continue register, if we have an actual thunk)
    let actual_value_label = make_label_name("actual-value".to_string());
    let force_label = make_label_name("force".to_string());
    let done = make_label_name("done".to_string());

    let thunk = compile(
        exp,
        Register::Thunk,
        Linkage::Next {
            expect_single: true,
        },
    );
    let thunk_linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: actual_value_label.clone(),
            expect_single,
        }
    } else {
        linkage
    };
    preserving(
        hashset!(Register::Env, Register::Continue),
        append_instruction_sequnce(
            thunk,
            make_intsruction_sequnce(
                hashset!(Register::Thunk),
                hashset!(),
                vec![
                    Instruction::Label(actual_value_label),
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
fn delay_it(exp: Ast2, target: Register, linkage: Linkage) -> InstructionSequnce {
    info!(
        "generating ir for delaying expression {exp:?}, with register {target}, with linkage {linkage:?}"
    );
    // TODO: when a thunk early returns were do we early return from?
    // it should early return from the last known function: (lambda linkage), so we need to
    // remember the what was in the continue register of before unthunking it (which will modify the
    // continue register, if we have an actual thunk)
    let thunk_label = make_label_name("thunk".to_string());
    let after_thunk = make_label_name("after-label".to_string());
    let thunk_linkage = if let Linkage::Next { expect_single } = linkage {
        Linkage::Label {
            place: after_thunk.clone(),
            expect_single,
        }
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
        .extend(compile_thunk_body(exp, thunk_label).instructions);
    inst.instructions.push(Instruction::Label(after_thunk));
    inst
}

#[cfg(feature = "lazy")]
fn compile_thunk_body(thunk: Ast2, thunk_entry: Label) -> InstructionSequnce {
    append_instruction_sequnce(
        InstructionSequnce::new(
            hashset!(Register::Env, Register::Thunk),
            hashset!(Register::Env),
            vec![
                Instruction::Label(thunk_entry),
                Instruction::Assign(
                    Register::Env,
                    Expr::Op(Perform {
                        op: Operation::ThunkEnv,
                        args: vec![Expr::Register(Register::Thunk)],
                    }),
                ),
            ],
        ),
        compile(thunk, Register::Thunk, Linkage::Return),
    )
}
// #[cfg(feature = "lazy")]
#[cfg(not(feature = "lazy"))]
fn delay_it(exp: Ast2, target: Register, linkage: Linkage) -> InstructionSequnce {
    compile(exp, target, linkage)
}

// #[cfg(feature = "lazy")]
#[cfg(not(feature = "lazy"))]
fn force_it(exp: Ast2, target: Register, linkage: Linkage) -> InstructionSequnce {
    compile(exp, target, linkage)
}
fn construct_arg_list(
    operand_codes: impl DoubleEndedIterator<Item = InstructionSequnce>,
) -> InstructionSequnce {
    operand_codes
        // .map(delay_it)
        .map(add_to_argl)
        .rev()
        .fold(
            InstructionSequnce::new(
                hashset!(),
                hashset!(Register::Argl),
                vec![Instruction::Assign(
                    Register::Argl,
                    Expr::Const(Ast::TheEmptyList),
                )],
            ),
            |a, b| preserving(hashset!(Register::Env), a, b),
        )
}

impl From<Ast> for Expr {
    fn from(value: Ast) -> Self {
        Self::Const(value)
    }
}
