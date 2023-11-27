# Fors
Forth Interpreter in Rust.
Forth is a stack-based programming language.

## Quick Start
```sh
git clone https://github.com/CobbCoding1/fors
cargo build
```

## How It Works

### Push onto stack
```Forth
0 ( pushes 0 )
1 ( pushes 1 )
```

### Arithmetic Operations on stack
```Forth
0 1 + ( sums the top 2 elements of the stack )
```
Valid symbols: + - * / mod

### Stack Movement Operations
```Forth
1 dup ( duplicates the top of the stack )
1 2 swap ( swaps the top two elements of the stack )
1 2 3 over ( duplicates the second element from the top of the stack )
1 2 3 rot ( takes the third element from the top of the stack, and moves it to the top )
```

### Words
Words are like functions in any other language
```Forth
: name 1 2 + ; ( define a word, starting at : until ; )
name ( call it with its name )
```

### Output
Two kinds of output in Forth, numbers, strings, characters, and newlines.
```Forth
1 2 + . ( dot takes the top of the stack and outputs it, no newline )
." hello, world " ( ." will output every character until the closing " )
48 emit ( will output the ascii value of the top of the stack )
cr ( prints a newline )
```

### Input
```Forth
key ( reads input from the user and pushes the keycode onto the stack )
```

### Comparisons
```Forth
10 10 = ( pushes -1 if equal, 0 if not equal )
11 10 > ( pushes -1 if greater, 0 if not greater )
10 11 < ( pushes -1 if greater, 0 if not greater )
```

### Bitwise Operations
```Forth
0 0 and ( outputs -1 if AND or 0 if not )
0 0 or ( outputs -1 if OR or 0 if not )
0 invert ( inverts the output, i.e. 0 -> -1. -1 -> 0, 5 -> -6 )
```

### Conditions and Loops
Conditions and loops must be in words
#### If Else
```Forth
: is-zero 0 = if ." That is zero " else "That is not zero " then ;
1 is-zero ( That is not zero )
0 is-zero ( That is zero )
```
#### Loops
Do Loop
```Forth
: loop-10-times 10 0 do i . loop ;
loop-10-times ( outputs 0-9, the i is a special keyword which prints the current iteration of the loop )
```
Begin Until
```Forth
: until-zero begin 1 - 0 = until ;
10 until-zero ( go until top of stack is zero )
```

### Variables and Constants
Variables
```Forth
variable var ( declare a variable called `var` into memory )
var ( push the variable location in memory to the stack )
15 var ! ( change the value in memory of the variable )
var @ ( get the value stored at the memory of variable )
var ? ( get the value stored and output it )
```
Constants
```Forth
50 constant size ( declares a constant with the value at the top of the stack )
size . ( outputs the value stored at constant size )
```

### Allocate Memory Space
```
variable space ( declare variable for start of memory )
15 cells allot ( allocate 15 spaces to the memory, starting at the most recently declared variable )
```
