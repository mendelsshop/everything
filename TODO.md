- [ ] Have a REPL.
- [/] Support loops.
Base syntax done(rest done through macros).
Would this just be mapped to something like a foreach function? (What about early returns?)
- [/] Support early returns from loops and functions.
During (sicp.rs), we need to pass around a the linkage that the function was compiled with, and then we get to return we use compile linkage, or we could have a label at the end of each function and just remember that and do a goto to it on return, similiar logic can be applied to loops (depends on how loops work).
But this might not work with thunks which do set their continue register.
- [ ] Have a Class system.
How should this be implmented?
- [ ] Create a module system.
How does modules work with gotos, can a module define a label that a user of that module can jump to?
How to implement at comilation, compile each module seperatly, but when compiling each thing in module with module path.
What about empty modules?
- [ ] Create and document a standard library (should be its own module).
- [ ] Remove all explicit panics (unwraps, excepts, ...), and replace them with results.
- [ ] Use proper error types for reuslts (not String).
- [x] Make trait for ast transformations, to clean up ast transformations.
- [/] Have better parser errors (and have better errors in general).
- [ ] Test parser.
- [ ] Test macro expander.
- [ ] Test ast transformers.
- [ ] Test compiler.
- [/] Adding logging.
- [/] Different syntax for lisp and not lisp.
Maybe in parser we should in application check for "reserved" idents and parse them as idents rather then expressions
- [ ] Combine ast1 to ast2 transformation, and macro expander and link finder as they all do somewhat similiar things, and all need to know what variables are already defined
