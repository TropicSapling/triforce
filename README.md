# P+ programming language
P+ is for...
* Performance
* Power
* Productivity

... and most importantly:
* **It makes sense.**

## Features
#### Definitions
<sup>Keywords surrounded by</sup>
* <sub>brackets (`[]`) are *optional*</sub>
* <sub>angle brackets (`<>`) **must** be replaced by a name of your choice</sub>
* <sub>backticks (`` ` ``) are **required** and escapes these definitions (i.e. `` `<type>` `` means you must literally type `<type>`)</sub>
* <sub>parentheses (`()`) **and** seperated by bars (`|`) are part of a list of mutually exclusive **required** keywords</sub>
* <sub>brackets (`[]`) **and** seperated by bars (`|`) are part of a list of mutually exclusive *optional* keywords</sub>

<sub>Dots (`...`) mean essentially what they do in mathematics.</sub>

<sub>Everything else is **required.**</sub>

--------

### Spec
#### Anonymous functions
1. Structure: `(<input pars> => <function body>) <input args>`.
2. `<input pars>` = `(<par1>) [(<par2>) ...]`
3. `<input args>` = `<arg1> [<arg2> ...]`
4. Every parameter is a *pattern def*.
5. If not enough input args are given, the function is partially applied.

#### Patterns (variables but better)
1. `<pattern def>` = `($(<pattern to define>) as <pattern to match>)` where
	- `<pattern to define>` = `<name start> [(<pattern def>)|<name continuation> ...] [<name end>]` where
		- `<name start>`, `name continuation`, `<name end>` are allowed to contain any symbols, including whitespace (TODO: exception for ops)
2. Call structure for ex. def. `$(example pattern taking (_ as _) and (_ as _)) as _`:
	- mixfix ex: `example pattern taking (x) and (123)`
	- prefix ex: `(example pattern taking $_ and $_) x 123`
3. Patterns are defined within the scope described by the *pattern parsing algorithm*.
4. Patterns can only be defined within `<input pars>`.
	- if it looks like a pattern is defined outside, it's actually part of a call to a defined pattern
5. Patterns, like functions, can be partially applied.

`$(add (4) to ($(a) as 7)) as #a #0`

#### Pattern parsing algorithm
1. Choose a `$(...)` and move to its outside.
2. If inside another `$(...)`, move to its outside, and then keep leaving scopes until you find `as`.
   If not, keep leaving scopes until you find `=>`.
3. Your pattern is defined after this `as` or `=>`.

#### Syntax sugar
1. `$(<pattern to define>)` <=> `($(<pattern to define>) as _)` <=> `($(<pattern to define>) as #0 [#1 ...])`
	- Note that this allows the input to be any kind of function, which you can call like `<defined pattern> [<arg1>] [<arg2> ...]`
2. `(<pattern to match>)`   <=> `(_ as <pattern to match>)`

#### Misc
1. `_` is a special built-in symbol meaning different things in different contexts, but typically it means "anything".

### [OLD] Syntax
1. Functions are defined using `<input> => <output>`.

2. `(<expr>)` *always* has higher precedence than `<expr>`.

3. Functions can have 1 or more args.
    - (define `f _ => ...` and call with `f _` to emulate 0 args)

4. Functions can have almost any structure (mixfix with additions).

5. Function names can only contain *either* characters *or* operators.

6. Variable function input is denoted by `$<var>`.

7. Non-variable (specific functions) input (except literals) or input with both non-variables and variables require surrounding `()`.

8. Number literals, char literals and string literals are built-in and bound to library implementations similarly to Agda.

9. Precedence can be overriden using `#precedence (below|above) <function> <your function>`.

10. `(<expr>)` returns whatever is left of `<expr>` after evaluation to the outer scope.

11. Functions return themselves and can be called "anonymously".

12. Functions return *partially* if passed as args to a non-evaluating function.
    - I.e. `f (g $x => x)` partially returns `(g $x => x)`.
    - **NOTE:** This does *not* apply to anonymous functions. I.e. `f ($x => x)` does *not* partially return `($x => x)`.

13. Functions are *only* defined in the scope they were created and scopes in which they (possibly partially) have been returned to.
    - **NOTE:** Functions are *not* defined inside functions they are passed to (except inside the variable). This means that `let f = g;` is different from `g;` in that the latter returns and therefore defines the function `g` in the scope while the former does not.

14. `_` is a special built-in symbol meaning different things in different contexts, but typically it means "anything".

15. Function input works using pattern matching of function names and their args.

16. `continue from <function> or alt <expr>` continues pattern matching if possible, else evaluates `<expr>`.

17. `caller` is a reserved keyword for the caller of the current function.

18. Functions which are passed fewer args than required are called *partially applied* and return the partially applied versions of themselves.

19. `` a`|`b`|`...`|`z `` is an or-pattern.

20. `` `...` `` are used in (or-)patterns to let the compiler figure out the rest.

21. The compiler will try to run as much as possible during compilation unless otherwise specified.

22. `prerun <expr>` ensures `<expr>` runs during compilation.

23. `run <expr>` ensures `<expr>` runs during runtime.

24. `stringify <expr>` turns the code of `<expr>` into a string.

25. `op <operator>[\n op <operator>...]` defines operators, which are defined to be characters placeable right next to separate functions.
    - I.e. `op ;` allows `($expr; =>);`.

26. Single-line `//` and multi-line `/* */` comments are built-in (to avoid issues with nested strings).

27. Passing all required args to a function will run it.

28. `ALL_ARGS <function>` returns all possible args that can be applied to the function. `length >= 1`.

29. `APPLIED_ARGS <function>` returns the args that have been applied to the function. `length >= 0`.

30. `Maximal munch`/`Longest match` parsing is used to solve ambiguity (unless invalid; then context is used).

31. In case there's ambiguity between if a fully applied function or another partially applied function was intended, the compiler will assume the fully applied function was intended and give a warning about this.
    - I.e. `if True do_something` is assumed to mean the fully applied `if $cond $body` function rather than a partially applied `if $cond $expr else $expr`.

--------

### <s>Data types [OUTDATED]
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

### Operators [OUTDATED]
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

### Lists & arrays [OUTDATED]
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

### Conditionals [OUTDATED]
* `if <condition> { <code> } [else if <condition> { <code> } else if...] [else { <code> }]`
* `match <var> { case <val1>: <code> [case <val2>: <code>...] [default: <code>] }` (equivalent of C's `switch`)

--------

### Strings [OUTDATED]
* `"null terminated string"`
* `'string size determined by <size> property'`
* `'null terminated string, but <size> property can still be used to get size\0'`
* `"null terminated string" == 'null terminated string\0'`

--------

### Functions [OUTDATED]
* `func <function>([<parameters>]) [-> <return type>] { <code> }`
* `<function>([parameters])` or `<function> <parameter>` or `<parameter> <function> <parameter>`
* `return [from <function>] <value>` (**NOTE:** You can't return from just any function, it needs to call the function you're currently in either directly or indirectly)
* Functions return the result of the last statement by default; no need to use `return` unless you want to return from somewhere else.
* P+ allows ad-hoc polymorphism; you can create multiple functions with the same name but with different parameters.
* Operator overloading is supported; for example, doing `func +(...)` would overload the `+` operator.

--------

### Loops [OUTDATED]
* `while <condition> { <code> }`
* `foreach <item> in <list> { <code> }`
* `repeat <n times> { <code> }`
* `break [<value>] [from <function>]`
* `continue`

--------

### Concurrency [OUTDATED]
* `async { <code> }`
* `select { <cases> }`
* `send <data> to <channel>`
* `<type> <var> = receive from <channel>`

--------

### Defining [OUTDATED]
* `#def '<code>' as '<code>'` (will support regex in the future using `#{<regex>}`, as well as `%{(property|properties|var)}`)
* `#if <condition>`
* `#else`
* `#elif <condition>`
* `#ifdef <const>`
* `#ifndef <const>`
* `#endif`

--------

### Special [OUTDATED]
* `goto <label>`
* ``#import (('|")<path>('|")|`<`<std lib path>`>`) [as <name>]``
* `#export <function1>[, <function2>...]`
* `eval '<code>'`

--------

### Built-in global variables [OUTDATED]
* `__OS`
* `__path`
* `__args`
* `__argc`
* `__line`
* `__item`
* `__app`

--------

### </s>Comments
* `// <One line comment>`

```
/* <Multi
line
comment> */
```

--------

### <s>Precedence [OUTDATED]
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
