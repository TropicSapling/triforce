# Things that need more thinking
## pointers
- Pointers should be clearly distinguished from other values
  - Idk how many times I’ve made the mistake of accidentally copying a pointer to the data instead of the actual data
  - No `Array` type secretly being a pointer to an array
    - How to make this work though? Arrays rely on pointers

## frozen
- Lazy evaluation? Optional like in Idris?
- Should it be renamed to `lazy` or similar?
	- `frozen` makes little sense when pattern matching still requires some partial evaluation
		- i.e. `$x as frozen any Bool` may require some evaluation in order to determine if `$x` is `any Bool`

## impl box

## purity
- Proving equality?
- Probably better with a different definition though: rather than no outside/free variables, it should be no mutable outside/free patterns/functions
  - Because you're almost always going to use some functions defined outside, like `mod` or `+`

## == box
- How will this "non-strict" equality work?
- `Integer` or `Integer {*}` -> `Integer {...|-1|0|1|...}`?
- `A {(B C {D})|E}` or `A {(B {*} C {D})|(E {*})}` -> `A {(B {...} C {D})|(E {...})}`?
- `*` -> `Bool|Integer|String|all_types_etc...`?
- This syntax sugar may be implemented if ``(`code`)`` args are added
  - i.e. ``(`Integer`) => Integer {...|-1|0|1|...}``
  - Hard to make it work for every single type though

## concurrency
- https://vorpus.org/blog/notes-on-structured-concurrency-or-go-statement-considered-harmful/

## functions
- getting function without calling it?
  - Haskell is able to infer this by looking at surrounding code and types
- anonymous functions?
- could spaces in names be problematic?
  - better to only allow Agda-style? (`this (arg) is (arg2) a (arg3) function ...`)

## memory
- rust borrow system? changes?
- memory layouts (of boxes)?
- How to deal with 8/16+ byte boxes?
  - Built-in arrays?

## mutability
- Dependent types makes mutability harder
- Like, `let n = 123` says type is (Integer) 123 rather than just Integer
- Maybe immutability by default, then specify what ranges of mutability are allowed?

## syntax / parsing
- Line breaks, semicolons?
- How will parsing work?
- Allow running functions inside args?
  - i.e. `f ([1, 2] length)` = `f (2)`
- Allow indentation as replacement for parentheses `()`?
- Mixfix parsing: http://www.cse.chalmers.se/~nad/publications/danielsson-norell-mixfix.pdf
  - Here it says even parentheses `()` can be defined as a mixfix function!
    - Not applicable for Triforce though, since parentheses are required to define any function
- https://www.reddit.com/r/ProgrammingLanguages/comments/hl04eq/this_talk_by_nicholas_matsakis_is_the_best/
  - Better compiler structure; instead of lex-parse-compile, only compile small parts at a time

## lists
- memory layout
- should lists without `[]` (`a, b, ...`) exist and be similar to tuples?
  - how would that work though? how to differentiate between `a` and list containing `a`?
- `,` as a function of its own like in Haskell?

## pattern matching
- `pattern | pattern | ...` - should this be allowed? i.e. `(0) + (1) | (1) + (0) => ...`
- NOTE: using such a syntax `[0, x] | [y, 0]` would *not* be allowed; instead use `[0, x] | [x, 0]`

## boxes as functions, Church encoding, etc.
- i.e. Church numerals, bools, etc.
  - `true (x) (_) => x; false (_) (y) => y`
  - `zero (f) (x) => x; one (f) (x) => f x; two (f) (x) => f (f x); ...`
- seems rather difficult
- functions returning multiple things?
- functions returning either one thing or another?
- nested functions
- https://www.youtube.com/watch?v=XrNdvWqxBvA
- needs something to group function args like you could using `{}` with boxes

## raw code input
- ``f (`some raw code`) => ...`` (you can also return raw code same way)
- `f (raw raw_code) => ...` - here `raw` specifies that input is raw and `raw_code` contains the raw code
- would this be necessary or could it as well work with `frozen`?
  - i.e. `f (frozen raw_code)` and `f (frozen (some raw code))`

## special symbols
- Repellors
  - i.e. digits of numbers
- Operators

## Scratch-ish GUI
- Not totally related to the language, but would be nice to have some GUI for testing programs easily
  - kinda like in Scratch
- Example of a similar thing but more text-ish: https://marketplace.visualstudio.com/items?itemName=fraser.live-coder

## Left- or right-associative function calls?
- Operators are typically left, but currently normal functions are right
- In Haskell normal functions are also left
  - However, this causes strange stuff like `print 1 + 2` becoming `(print 1) + 2`

## modules
- https://news.ycombinator.com/item?id=16458732
- https://ocaml.org/learn/tutorials/modules.html

## refinement types
- define types using predicates/conditions
  - i.e. `Nat = n >= 0`

## error handling
- https://www.reddit.com/r/ProgrammingLanguages/comments/drr3ri/comment/f6kn8a5

## static/dynamic linking
- https://gankra.github.io/blah/swift-abi/

## optimisation
- http://www.lihaoyi.com/post/HowanOptimizingCompilerWorks.html
- The dependent typing system "automatically" optimises some code

## other
- should `map` be called `apply <function> for each in <list>` or maybe `apply_all <function> <list>`?
- `map` -> `lmap` (list map)? Fits better together with `fmap`.
- code readability: https://dmitripavlutin.com/coding-like-shakespeare-practical-function-naming-conventions/
- box defs with commas? `A {B C ...}` -> `A {B, C, ...}`
- `id` / `identity` (function) actually has a few uses
  - ex: `bimap id (\w -> Tetris (startPosition,nextShape) w rest) (clearLines (well `combine` place player))`
- `any (T) => T _ _ ... _`
  - `_` = all possible values
  - i.e. `x == any Nat` <=> `x == Nat _` <=> `x == Nat (Zero|(Nat (PlusOne Zero))|...)`
- `a|b|c|...` as sort-of lists? probably necessary in order to implement some functions
- some way to get amount of args a function takes? probably necessary for `any` function
  - a way to specify functions taking `n` args is also necessary
    - i.e. perhaps `f (g _ _ _)` and `f (_ infix _ function _)` specifies functions taking 3 args?
- some way of differentiating between naming a function and matching on a function name
  - i.e. does `f (g x)` mean "only take the specific function `g` as input" or "take all (1-arg) functions as input and name them `g`?
- https://en.wikipedia.org/wiki/Aspect-oriented_programming
- https://pike.lysator.liu.se/about/
- http://www.cs.ox.ac.uk/jeremy.gibbons/publications/progorn.pf (Section 2.1, p. 3: Index-first)
- https://dev.to/myterminal/recursion-memoization-and-y-combinator-174l
  - easy memoization with only anonymous functions
  - alt: https://stackoverflow.com/questions/93526/what-is-a-y-combinator/6713431#6713431
