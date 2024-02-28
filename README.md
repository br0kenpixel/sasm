# SASM Lang
*Simple Assembly* is a simple high-level assembly-like interpreted scripting language. It's designed for educational purposes.

## Basics
SASM uses "instructions" to perform operations. Each instruction is a single line of code. It does __not__ support functions or procedures. It also doesn't support any kind of modules or libraries. You cannot import any external code.

SASM implements the following data types:
- Numbers
- Strings

Variables are partially statically typed. Their type is determined by the first value assigned to them. Once a variable is assigned a value, it's type cannot be changed.

## Syntax
A single instruction is made up of 2 parts:
1. The operation
2. The arguments

Example:
```
VAR x
```

Here, `VAR` is the operation and `x` is the argument.

2nd Example:
```
MOV x,13
```

Here, `MOV` is the operation and `x,13` are the arguments. __Arguments are separated by commas, with no spaces.__

## Instructions

### VAR
Creates a variable. By default, it's value is not set (is `null`). You can set it's value __(and type)__ by assigning a value to it.

```
VAR x
MOV x,1
```

### MOV
Moves a value to a variable. The first argument is the destination variable and the second argument is the value to be moved. The source value can be a variable or a literal. If the source value is a variable, it's value is copied to the destination variable.

```
VAR x
VAR y

MOV x,1
MOV y,9
```

To copy the value of `x` to `y`:
```
VAR x
VAR y

MOV x,9
MOV y,x
```

### INC
Increments a variable by 1. If the variable is not a number (or `null`), an error is thrown.

```
VAR x

MOV x,1
INC x
```

### DEC
Decrements a variable by 1. If the variable is not a number (or `null`), an error is thrown.

```
VAR x

MOV x,1
DEC x
```

### DMP
Prints the value of a variable to the console. It also accepts variables with no value (`null`).

```
VAR x
MOV x,1
DMP x
```

### ADD
Adds a value to a variable. The first argument is the destination variable and the second argument is the value to be added. The source value can be a variable or a literal. If the source value is a variable, it's value is added to the destination variable.

```
VAR x
VAR y

MOV x,1
MOV y,9
ADD x,y

DMP x
```

### SUB
Subtracts a value from a variable. The first argument is the destination variable and the second argument is the value to be subtracted. The source value can be a variable or a literal. If the source value is a variable, it's value is subtracted from the destination variable.

```
VAR x
VAR y

MOV x,1
MOV y,9
SUB x,y

DMP x
```

### MUL
Multiplies a variable by a value. The first argument is the destination variable and the second argument is the value to be multiplied. The source value can be a variable or a literal.

```
VAR x
VAR y

MOV x,1
MOV y,9

MUL x,y
DMP x
```

### DIV
Divides a variable by a value. The first argument is the destination variable and the second argument is the value to be divided. The source value can be a variable or a literal.

```
VAR x
VAR y

MOV x,9
MOV y,3

DIV x,y
DMP x
```

### POW
Raises a variable to a power. The first argument is the destination variable and the second argument is the value to be raised to. The source value can be a variable or a literal.

```
VAR x
VAR y

MOV x,2
MOV y,3

POW x,y
DMP x
```

### CMP
Compares 2 values. The first argument must be a variable and the second argument can be a variable or a literal. 

```
VAR x
VAR y

MOV x,1
MOV y,9
CMP x,y
```

### JNE
Jumps to the specified line offset if the last comparison was not equal. The offset must be relative to the current line. If the offset is negative, it jumps backwards. If the offset is positive, it jumps forwards.

```
VAR x
VAR y

MOV x,1
MOV y,9
CMP x,y

JNE 3
DMP "They are equal"
DIE
DMP "They are not equal"
```

### JEQ
Jumps to the specified line offset if the last comparison was equal. The offset must be relative to the current line. If the offset is negative, it jumps backwards. If the offset is positive, it jumps forwards.

```
VAR x
VAR y

MOV x,1
MOV y,9
CMP x,y

JEQ 3
DMP "They are not equal"
DIE
DMP "They are equal"
```

### DIE
Exits the program with a status code. If no status code is provided, it defaults to 0.

```
DIE 1
```

Or to exit with a status code of 0:
```
DIE
```