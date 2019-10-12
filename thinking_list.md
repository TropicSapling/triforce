# Things that need more thinking
## frozen
- Lazy?

## impl box

## purity
- Proving equality?

## == box
- How will this "non-strict" equality work?

## concurrency

## functions
- getting function without calling it?
- anonymous functions?
- could spaces in names be problematic?

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

## lists
- memory layout
- should lists without `[]` (`a, b, ...`) exist and be similar to tuples?

## pattern matching
- `pattern | pattern | ...` - should this be allowed? i.e. `(0) + (1) | (1) + (0) => ...`
- NOTE: using such a syntax `[0, x] | [y, 0]` would *not* be allowed; instead use `[0, x] | [x, 0]`

## boxes as functions, Church numerals, etc.
- seems rather difficult
- functions returning multiple things?
- functions returning either one thing or another?
- nested functions
