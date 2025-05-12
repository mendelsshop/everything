# Everything(-lang) Documentation

# Anonymous Functions

# Calling Functions

Any user defined function that is not variadic is curried.
When you apply a curried function to a list of arguments like `(f 1 2 3)` it is evaluated or compiled like `(((f 1) 2) 3)`.
This unless at some point it evaluates to a builtin or variadic function let us say `(f 1)` resulted in a variadic function then full evaluation would be `((f 1) 2 3)`.
In other word applications of variadic or builtin functions are applied immediately to the rest of the original argument list.
