# P+ compiler written in C

## Implemented Features
#### Definitions
<sup>Keywords surrounded by</sup>
* <sub>brackets (`[]`) are *optional*</sub>
* <sub>angle brackets (`<>`) **must** be replaced by a name of your choice</sub>
* <sub>backticks (`` ` ``) are **required** and escapes these definitions (i.e. `` `<type>` `` means you must literally type `<type>`)</sub>
* <sub>parentheses (`()`) **and** seperated by bars (`|`) are part of a list of mutually exclusive **required** keywords</sub>
* <sub>brackets (`[]`) **and** seperated by bars (`|`) are part of a list of mutually exclusive *optional* keywords</sub>

<sub>Everything else is **required.**</sub>

--------

### Data types
#### Basic
* `char`

#### Special
* `void`

--------

### Operators
#### Arithmetic
* `+`
* `-`
* `*`
* `/`
* `%`

#### Bitwise
* `&`
* `|`
* `~`
* `^`
* `<<`
* `>>`

#### Compound Assignment
* `+=`
* `-=`
* `*=`
* `/=`
* `%=`
* `&=`
* `|=`
* `^=`
* `<<=`
* `>>=`

#### Logical
* `!`
* `&&`
* `||`

#### Relational
* `==`
* `!=`
* `>`
* `<`
* `>=`
* `<=`

#### Misc.
* `? :` (**NOTE:** Might get replaced by `if` in the future, and the `?` might be used for exception handling instead)
* `->`
* `[]`
* `@`
* `>>>`

--------

### Lists
* `str[>>>] == "Test"`
* `str[start >>> stop]`
* `str == address`
* `str[when <condition> >>> until <condition>]`

--------

### Conditionals
* `if(<condition>) { <code> } [else if(<condition>) { <code> } else if...] [else { <code> }]`
* `match(<var>) { case <val1>: <code> [case <val2>: <code>...] [default: <code>] }` (equivalent of C's `switch`)

--------

### Strings
* `"null terminated string"`
* `'string size determined by <size> property'`
* `'null terminated string, but <size> property can still be used to get size\0'`
* `"null terminated string" == 'null terminated string\0'`

--------

### Loops
* `while(<condition>) { <code> }`
* `repeat(<n times>) { <code> }`
* `break`
* `continue`

--------

### Defining
* `#def '<code>' as '<code>'` (will support regex in the future using `#{<regex>}`, as well as `%{(property|properties|var)}`)
* `#ifdef <const>`
* `#ifndef <const>`
* `#endif`

--------

### Special
* `goto <label>`
* ``#import (('|")<path>('|")|`<`<std lib path>`>`) [as <name>]`` (**NOTE:** Currently pretty buggy)
* `#export <function1>[, <function2>...]`

--------

### Built-in global variables
* `__OS`
* `__path`
* `__args`
* `__argc`
* `__line`
* `__item`

--------

### Comments
* `// <One line comment>`

```
/* <Multi
line
comment> */
```

--------