# About Hier Language
Hier is my second attempt at making a programming language (previous was GoldByte, but it had bad architecture, so it was easier to start from the  beginning). This time I decided to do actual research (Thanks for Crafting Interpreters). It runs code by interpreting AST, so it is not very fast, but it currently works and writing a bytecode VM is a possibility. There are issues with it, and it is not perfect, but with time it will improve.

# Usage
Libhier is a library implementing Hier. You can use it in your own projects. If you want to access it from CLI, you can check [hier](https://github.com/wiktorwojcik112/hier).

# Design
Hier uses a Lisp-like syntax of putting everything in brackets - ( and ). Hier contains only 5 constructs: lists (using ( and ) ), blocks (using { and }), numbers (number with optional ., for example: 1.0, 2.5, -1.5, 5, -3), strings (using " and ", they can contain all characters except ", because there is no interpolation and they can be multiline (new lines are included in string)), subscripts (expression with [ ]), properties (expression with . and its property (including functions)), identifiers (any characters, except it can't begin with " and must not contain spaces, :, (, ), ., new lines, [ and ]) and directives (which begin with #).

The language is functional (no classes) and everything in it (except directives) is some kind an expression. Here is an example of a program which adds numbers 1 2 3 and result of subtracting 2 from 1 and prints it:

```
#main
(print (+ 1 2 3 (- 1 2)))
```

Almost all of Hier is value-based. That means that operation creates a copy of a value. For example, by using insert, remove or replace functions on an array, you don't change the original array, but create a new array with specified changes.

# Importing
In Hier you can import files using import function which accepts a string with a path to a hier file (./ at the beginning is automatically prepended and .hier is added at the end). It returns a special object which you can assign to a variable and use it by prepending an identifier with this variables name and # (object#identifier). Here is an example showing how importing of an example library (library.hier) and another one in a folder (./math/constants.hier).

./library.hier
```
(@hello (| name) { (println "Hello, " name) })
```

./math/constants.hier
```
(@pi 3.14159265359)
(@golden_ratio 1.61803398875)
```

./main.hier
```
(@mylibrary (import "library"))
(library#hello "World")
(@constants (import "math/constants"))
(print "Pi is equal " constants#pi)
(print "Golden ratio is equal " constants#golden_ratio)
```

# Lists
Lists are fundamental element of Hier. They are made of expressions between ( and ). The list may be a function call depending on circumstances. If first expression is an identifier, it will work like a function call, for example (print 1 2 3). If first expression is a property it will work like a syntactic sugar for function call on object, for example, it will convert (array.insert 1) to (insert array 1), allowing clearer syntax. If first expression evaluates to function arguments (for example: (| a b c)), list will evaluate to an anonymous function. Otherwise, it will generate an array (in Hier, arrays are collective name for lists and arrays). Lists (arrays) can also be created using (& value1 value2) function call or (list value1 value2) function call.

An example:
```
(print 1)
(@array (1 2 3))
(print array[0])
(@function ((| a b c) { (print a) }))
(print (array.length))
```
# Table
Tables are created using (# value1 value2) or (table value1 value2) function calls. Tables contain many key-values (all arguments to calls must be key-values (look: key-value section)). Fields of table can be accessed using property and specifying key of a key-value pair. Here is an example of the usage:
```
(@some (# key:"value"))
(print some.key)
```

At this moment, adding fields is unsupported.

# Blocks
Blocks are made of expressions between curly brackets - { and }. The difference between them and lists is that blocks do not evaluate immediately. This means, that to run a block, you will need to put it as a expression for a function, that executes blocks, for example if, run or while. Blocks also have this property, that they evaluate to the last expression in it, so { (+ 1 2) (+ 5 6) } will evaluate to a value 11. All blocks always create new scopes.

# Properties
Properties can mean either "methods" or a property of a structure. If it appears at the beginning of a list, it will convert to a function with object as the first argument (look: lists). If it appears later, it will convert to a get function call, for example, (print a.b) will convert to (print (get a b)), which will get property b from object a.

# Variables
In Hier, you declare a variable using (@variable_name value_expression) syntax. Two variables with the same name can't exist in the same scope (block). You can get value of the variable by just using its identifier in a list further than first argument, for example, (print variable_name). If variable doesn't exist, Hier returns null. You can use (=name value) to assign a new value to variable.

# Subscripts
Subscript is made by appending [value] to an expression. The value can be an identifier, a number, a block, a list and a string. Subscripts are converted like properties to a get function call, for example, (print array[0]) gets converted to (print (get array 0)).

# Key-value
Key-value is created using identifier(expression) syntax. You can access key using its key property and value using its value property, like this:
(@some key("value"))
(print some.key some.value)

# Identifiers
Some identifiers get converted into values, like true, false and null. All other refer to variables.

# Operators
Operators are just functions called like other functions. There are operators for addition (+; also acts as a string concatenation operation), subtraction (-), multiplication (*), division (/), modulo (%; because all numbers are floats, it rounds all numbers down and then performs modulo), logical negation (!; the only operator that only accepts one argument) logical and (&&),logical or (||), null-coalescing (??; if left is null, returns right, and if left isn't null, return left), (non-)equality (!= and ==) and comparison (<, >, <= and >=). +, -, *, /, && and || accept many arguments. 

# Piping
Pipe is represented using > symbol. When pipe is used, the previous list is placed inside the next list as first argument.
This allows for more readable chaining of long commands. For example, instead of
(print (map (1 2 3) { (+ element 1 }))
you can write
(1 2 3) > (map { (+ element 1) }) > (print)
The piping syntax is converted into the first example, so it has the same effect.

# Functions
Functions are declared using (@function_name (| first_argument second_argument) { (print first_argument) }) syntax. Function | returns function arguments - a special value that just contains identifiers that are passed as arguments. The block is the code that will get executed when function is called. You call such function using normal syntax: (function_name 1 2). Hier checks arity (number of arguments) of functions and errors when it doesn't match. 

# Control flow
Hier has 6 control flow functions - try, run, if, while, repeat, break and for.

## Run
(run expression...)
Run evaluates all (run accepts any number of arguments) of its arguments (including execution of passed blocks) and returns value of the last expression, so this code would return 2:
(run { (print "test") } 2)

## If
(if condition block block?)
If executes first block when condition is true and returns the result of its execution (the last expression). If there is a second block, it gets executed when condition is false and its value gets returned.

## While
(while block block)
While first executes first block in the new scope. Then it checks the value of the second block (must evaluate to a bool) and if its true, then it executes third block. Then it repeats check and execution as long as check evaluates to true. Th evalue of while evaluation is null, but it may be changed to the value of the last iteration (same for for and repeat loops).

## Try
(try expression block)
Try checks if expression returns error (you can make an error using error function which accepts string as the only argument) and runs the block if it is an error. It returns value of the expression if it's not an error, and value of the block, if expression evaluates to an error.

## Repeat
(repeat number? block)
Repeat runs the block specified number of times. If there is only a block, it runs the block infinitely. 

## Break
(break)
Break creates a "LoopExit" error which exits currently running loop.

## For
(for array block)
For runs the block for every element of a array (must be a list or a string). In every iteration the current element is passed as a variable named "element".

# Other functions
Hier contains many functions like print (print all values), println (print all values and a new line at the end), cmd (run a shell command), eval (evaluate Hier code string), string and number conversion, operations on arrays (insert, remove, length and replace) and a few more. You can find all of them in native_functions.rs file (they will be split to separate files in the future). All the functions will soon be documented. Some of them are only accessible from a client (example: [hier](https://github.com/wiktorwojcik112/hier) - CLI client)


