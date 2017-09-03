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
* `number`
* `bool`
* `(array|list|pointer)[*<n>]`

##### Extras
* `[volatile] only (array|list|pointer)[*<n>]`
* `[const|volatile] [unsigned|signed|fraction] number [(array|list|pointer)[*<n>]]`
* `[volatile] only [unsigned|signed|fraction] number (array|list|pointer)[*<n>]`
* `[const|volatile] [unsigned|signed] (int|char) [(array|list|pointer)[*<n>]]`
* `[volatile] only [unsigned|signed] (int|char) (array|list|pointer)[*<n>]`

##### Special
* `chan`
* `void`
* `type <custom type> = <type1>[|<type2>|<type3>...]`
* `func`
* `noscope [<more types>...]`
* `clang <type> <function name>([<parameters>]) { <C code> }`
* Not specifying a type for a function parameter allows the parameter to be of any type.

##### Properties
* `` var`<type>` ``
* `` var`<size>` ``
* `` var`<alignment>` ``
* `` some_fraction`<precision>` `` \[**NOTE:** The precision value is the number of bits for the exponent, **not** the number of decimals\]
* `` pointer_to_list`<length>` `` \[**NOTE:** `` some_list`<`<property>`>` `` will **not** access the property of the whole list, but the properties of each item of the list.\* Use `` ->some_list`<`<property>`>` `` instead. \]
* `` channel`<buffer>` ``
* `var>bit<`
* You can assign properties at variable creation: ``<type> [`<`<property1>`>`=<property1 value> `<`<property2>`>`=<property2 value>...] var``

<sup>\*This is because `` some_list`<`<property>`>` `` decays into `` pointer_to_list[>>>]`<`<property>`>` ``</sup>

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

##### Compound Assignment
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
* `[]`
* `@`
* `<<<`
* `>>>`

--------

#### Lists
* `pointer sublist -> some_list[start >>> stop]`
* `pointer sublist2 -> some_list[when <condition> >>> until <condition>]`
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
* P+ allows ad-hoc polymorphism; you can create multiple functions with the same name but with different parameters.

--------

#### Loops
* `foreach <item> in <list>`
* `while(<condition>)`
* `repeat(<n times>)`
* `break`

--------

#### Defining
* `#redef`
* `#def` (supports regex using `#{<regex>}`, as well as `%{(property|properties|var)}`)
* `#ifdef`
* `#else`
* `#endif`

--------

#### Special
* `async`
* `eval`
* `import`

--------

#### Built-in global variables
* `__path`
* `__app`
* `__line`
* `__item`

--------
