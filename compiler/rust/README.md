# P+ compiler written in Rust

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
* `bool`
* `func`

#### Extras
* `[unsigned] int` (**NOTE:** corresponds to Rust's i64/u64 for now)

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
* `?` (**NOTE:** Rust version)
* `->` (**NOTE:** Used for both pointers and specifying return types for functions)
* `[]`
* `@`

--------

### Conditionals
* `if <condition> { <code> } [else if <condition> { <code> } else if...] [else { <code> }]`
* `match <var> { <val> [as <var>] => <statement>, [<val> [as <var>] => <statement>...] }`

--------

### Strings
* `"Rust string"`

--------

### Loops
* `while <condition> { <code> }`
* `foreach <item> in <list> { <code> }`
* `break`
* `continue`

--------

### Special
* `import <module>` (**NOTE:** Only basic Rust-like functionality for now)

--------

### Comments
* `// <One line comment>`

```
/* <Multi
line
comment> */
```

--------