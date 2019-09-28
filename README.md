# P+ programming language
P+ is for...
* Performance
* Power
* Productivity

... and most importantly:
* **It makes sense.**

## <s>Features [OUTDATED]
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
* `number`
* `bool`
* `(array|list|pointer)[*<n>]`
* `func`

#### Extras
* `only [register|stack|heap] [volatile] [unique] [func] (array|list|pointer)[*<n>] [chan]`
* `[register|stack|heap] [const|volatile] [unsigned|signed|fraction] number [func] [(array|list|pointer)[*<n>]] [chan]`
* `only [register|stack|heap] [volatile] [unsigned|signed|fraction] number [func] (array|list|pointer)[*<n>] [chan]`
* `[register|stack|heap] [const|volatile] [unsigned|signed] (int|char) [func] [(array|list|pointer)[*<n>]] [chan]`
* `only [register|stack|heap] [volatile] [unsigned|signed] (int|char) [func] (array|list|pointer)[*<n>] [chan]`

#### Special
* `void`
* ``type <custom type> extends <type1>[`|`<type2>`|`<type3>...]``
* `clang <type> <function name>([<parameters>]) { <C code> }`
* Not specifying a type for a function parameter allows the parameter to be of any type.

#### Properties
* `` var`<type>` ``
* `` var`<size>` ``
* `` var`<memsize>` ``
* `` var`<alignment>` ``
* `` var`<scope>` `` (default: `1`)
* `` some_fraction`<precision>` `` \[**NOTE:** The precision value is the number of bits for the exponent, **not** the number of decimals\]
* `` pointer_to_list`<length>` `` \[**NOTE:** `` some_list`<`<property>`>` `` will **not** access the property of the whole list, but the properties of each item of the list.\* Use `` ->some_list`<`<property>`>` `` instead. \]
* `` pointer_to_list`<separator>` ``
* `` str`<encoding>` `` (default: `'utf-8'`)
* `` channel`<buffer>` `` (default: `1`)
* `var>bit<`
* You can assign properties at variable creation: ``<type> [`<`<property1>`>`=<property1 value> `<`<property2>`>`=<property2 value>...] var``

<sup>\*This is because `` some_list`<`<property>`>` `` decays into `` pointer_to_list[>>>]`<`<property>`>` ``</sup>

--------

### Operators
#### Arithmetic
* `+`
* `-`
* `*`
* `/`
* `%`
* `**`

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
* `<<<`
* `in` (example: `if(item in arr) ...`)

--------

### Lists & arrays
* `str[>>>] == "Test"`
* `str[start >>> stop]`
* `str == address`
* `str[when <condition> >>> until <condition>]`
* `pointer sublist -> some_list[start >>> stop]`
* `pointer sublist2 -> some_list[when <condition> >>> until <condition>]`
* `pointer new_sublist -> [1, 2, 3]`
* `pointer new_subarr -> {1, 2, 3}`
* `str[<<<] == "tseT"`
* `str[stop <<< start]`

--------

### Conditionals
* `if <condition> { <code> } [else if <condition> { <code> } else if...] [else { <code> }]`
* `match <var> { case <val1>: <code> [case <val2>: <code>...] [default: <code>] }` (equivalent of C's `switch`)

--------

### Strings
* `"null terminated string"`
* `'string size determined by <size> property'`
* `'null terminated string, but <size> property can still be used to get size\0'`
* `"null terminated string" == 'null terminated string\0'`

--------

### Functions
* `func <function>([<parameters>]) [-> <return type>] { <code> }`
* `<function>([parameters])` or `<function> <parameter>` or `<parameter> <function> <parameter>`
* `return [from <function>] <value>` (**NOTE:** You can't return from just any function, it needs to call the function you're currently in either directly or indirectly)
* Functions return the result of the last statement by default; no need to use `return` unless you want to return from somewhere else.
* P+ allows ad-hoc polymorphism; you can create multiple functions with the same name but with different parameters.
* Operator overloading is supported; for example, doing `func +(...)` would overload the `+` operator.

--------

### Loops
* `while <condition> { <code> }`
* `foreach <item> in <list> { <code> }`
* `repeat <n times> { <code> }`
* `break [<value>] [from <function>]`
* `continue`

--------

### Concurrency
* `async { <code> }`
* `select { <cases> }`
* `send <data> to <channel>`
* `<type> <var> = receive from <channel>`

--------

### Defining
* `#def '<code>' as '<code>'` (will support regex in the future using `#{<regex>}`, as well as `%{(property|properties|var)}`)
* `#if <condition>`
* `#else`
* `#elif <condition>`
* `#ifdef <const>`
* `#ifndef <const>`
* `#endif`

--------

### Special
* `goto <label>`
* ``#import (('|")<path>('|")|`<`<std lib path>`>`) [as <name>]``
* `#export <function1>[, <function2>...]`
* `eval '<code>'`

--------

### Built-in global variables
* `__OS`
* `__path`
* `__args`
* `__argc`
* `__line`
* `__item`
* `__app`

--------

### Comments
* `// <One line comment>`

```
/* <Multi
line
comment> */
```

--------

### Precedence
1. `()`, `[]`, `.`, `++`, `--`
2. `!`, `~`, `(<type>)`, `@`, `->`, `**`
3. `*`, `/`, `%`
4. `+`, `-`
5. `>>`, `<<`
6. `<`, `<=`, `>`, `>=`
7. `==`, `!=`
8. `&`
9. `^`
10. `|`
11. `&&`
12. `||`
13. `?:`
14. `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `**=`, `>>=`, `<<=`, `&=`, `^=`, `|=`
15. `>>>`, `<<<`, `,`, `in`

--------
</s>
