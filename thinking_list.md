# Things that need more thinking
## frozen
- Lazy evaluation? Optional like in Idris?

## impl box

## purity
- Proving equality?

## == box
- How will this "non-strict" equality work?
- `Integer` or `Integer {*}` -> `Integer {...|-1|0|1|...}`?
- `A {(B C {D})|E}` or `A {(B {*} C {D})|(E {*})}` -> `A {(B {...} C {D})|(E {...})}`?
- `*` -> `Bool|Integer|String|all_types_etc...`?
- This syntax sugar may be implemented if ``(`code`)`` args are added
  - i.e. ``(`Integer`) => Integer {...|-1|0|1|...}``
  - Hard to make it work for every single type though

## concurrency

## functions
- getting function without calling it?
  - Haskell is able to infer this by looking at surrounding code and types
- anonymous functions?
- could spaces in names be problematic?
  - better to only allow Agda-style? (`this (arg) is (arg2) a (arg3) function ...`)

## memory
- rust borrow system? changes?
- memory layouts (of boxes)?

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

## lists
- memory layout
- should lists without `[]` (`a, b, ...`) exist and be similar to tuples?

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

## other
- should `map` be called `apply <function> for each in <list>` or maybe `apply_all <function> <list>`?
- code readability: https://dmitripavlutin.com/coding-like-shakespeare-practical-function-naming-conventions/
- box defs with commas? `A {B C ...` -> `A {B, C, ...}`
- `id` / `identity` (function) actually has a few uses
  - ex: `bimap id (\w -> Tetris (startPosition,nextShape) w rest) (clearLines (well `combine` place player))`
- `any (T) => T _ _ ... _`
  - `_` = all possible values
  - i.e. `x == any Nat` <=> `x == Nat _` <=> `x == (Nat Zero)|(Nat (PlusOne Zero))|...`
