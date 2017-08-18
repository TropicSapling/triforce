# P+ programming language
P+ is for...
* Performance
* Power
* Productivity

... and most importantly:
* **It makes sense.**

## Features
### Current

--------

#### Comments
`// One line comment`

--------

### Planned

--------

#### Comments
```
/* Multi
line
comment */
```

--------

#### Data types
##### Basic
* `var [array]`
* `array [array]`
* `pointer [array]`

##### Extras
* `[unsigned|signed|decimal] number [array|pointer]`
* `[unsigned|signed] (int|char|string) [array|pointer]`
* `(posnum|posint) [array|pointer]`

##### Special
* `void`
* `clang <type> <function name>(<parameters>) { <C code> }`
* Not specifying a type for a function parameter allows the parameter to be of any type.

##### Properties
* `var<type>`
* `var<size>`
* `arr<length>`

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
* `@`
* `<<<`
* `>>>`

--------

#### Lists
* `pointer subarr -> arr[start >>> stop]`
* `pointer subarr2 -> arr[when <condition> >>> until <condition>`
* `str[>>>] == "Test"`
* `str[<<<] == "tseT"`
* `str[start >>> stop]`
* `str[stop <<< start]`
* `str[when <condition> >>> until <condition>]`
* `str == address`

--------

#### Functions
* `<return type> <function name>([parameters]) { <code> }`
* `<function name>([parameters])`

--------

#### IO
* `cout "Print something to console"`

--------

#### Special
* `eval`
* `import`

--------

#### Built-in global variables
* `__path`
* `__line`
* `__item`
* `__itemID`

--------
