# SASM Lang
*Simple Assembly* is a simple high-level assembly-like interpreted scripting language. It's designed for educational purposes.

## Basics
SASM uses "instructions" to perform operations. Each instruction is a single line of code. It does __not__ support functions or procedures. It also doesn't support any kind of modules or libraries. You cannot import any external code.

SASM implements the following data types:
- Numbers
- Strings
- Floats

Variables are partially statically typed. Their type is determined by the first value assigned to them. Once a variable is assigned a value, it's type cannot be changed.

## Wiki & Guides
Wiki home page can be found [here](https://github.com/br0kenpixel/sasm/wiki).

## Download and try
You can download the latest prebuilt binaries from the [releases](https://github.com/br0kenpixel/sasm/releases) page. There's no need to install anything, since all the binaries are statically linked.

Also check out some example programs in [examples](examples/).

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

## Loops
Since SASM does not allow defining code blocks, loops are implemented using `CMP` and `JNE`/`JEQ` instructions.
Here's an example of a simple loop that prints the numbers 1 to 10:

```
VAR x
MOV x,1

CMP x,11
JEQ 4
DMP x
INC x
JMP -4

DIE
```