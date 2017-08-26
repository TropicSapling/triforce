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

<sub>Everything else is **required.**</sub>

### Current

--------

#### Comments
* `// <One line comment>`

```
/* <Multi
line
comment> */
```

--------

### Planned

--------

#### Data types
##### Basic
* `var`
* `[only|const] (array|list|pointer)[*<n>]`

##### Extras
* `const [var]`
* `bool`
* `[const] [unsigned|signed|decimal] number [(array|list|pointer)[*<n>]]`
* `only [unsigned|signed|decimal] number (array|list|pointer)[*<n>]`
* `[const] [unsigned|signed] (int|char) [(array|list|pointer)[*<n>]]`
* `only [unsigned|signed] (int|char) (array|list|pointer)[*<n>]`

##### Special
* `chan`
* `void`
* `func`
* `noscope [<more types>...]`
* `clang <type> <function name>([<parameters>]) { <C code> }`
* Not specifying a type for a function parameter allows the parameter to be of any type.

##### Properties
* `` variable`<type>` ``
* `` variable`<size>` ``
* `` number`<precision>` `` \[**NOTE:** The precision value is the number of bits for the exponent, **not** the number of decimals\]
* `` arr`<length>` ``
* `` channel`<buffer>` ``
* `var>bit<`
* You can assign properties at variable creation: ``<type> variable [`<`<property1>`>`=<property1 value> `<`<property2>`>`=<property2 value>...] (=|->|<-) <variable value>``

--------

#### Operators
##### Arithmetic
* `+`
* `-`
* `*`
* `/`
* `%`

##### Bitwise
* `&`
* `|`
* `~`
* `^`
* `<<`
* `>>`

##### Assignment
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

##### Logical
* `!`
* `&&`
* `||`

##### Relational
* `==`
* `!=`
* `>`
* `<`
* `>=`
* `<=`

##### Misc.
* `? :`
* `->` (used for pointers)
* `<-` (used for channels)
* `@`
* `<<<`
* `>>>`

--------

#### Lists
* `pointer sublist -> list[start >>> stop]`
* `pointer sublist2 -> list[when <condition> >>> until <condition>`
* `str[>>>] == "Test"`
* `str[<<<] == "tseT"`
* `str[start >>> stop]`
* `str[stop <<< start]`
* `str[when <condition> >>> until <condition>]`
* `str == address`

--------

#### Strings
* `"null terminated string"`
* `'string size determined by <size> property'`
* `'null terminated string, but <size> property can still be used to get size\0'`
* `"null terminated string" == 'null terminated string\0'`

--------

#### Functions
* `<return type> <function name>([<parameters>]) { <code> }`
* `func <function name>([<parameters>]) { <code> }`
* `<function name>([parameters])`
* `return [from <function>] <value>`

--------

#### Loops
* `foreach <item> in <list>`
* `while(<condition>)`
* `repeat(<n times>)`
* `break`

--------

#### IO
* `cout "Print something to console"`

--------

#### Special
* `async`
* `eval`
* `pause <ms>`
* `import`
* `#redef`

--------

#### Built-in global variables
* `__path`
* `__app`
* `__line`
* `__item`

--------
