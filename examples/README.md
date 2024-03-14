## Formal syntax (incomplete, as of 2024-03-14)

The formal syntax will be specified with a metalanguage similar to Go's:
```EBNF
Syntax     = [Definition]…
Definition = def_name "=" Expression ["\n" "|" Expression]… "\n"
Expression = Term ["|" Term]…
Term       = Factor [Factor]…
Factor     = def_name | tok["-"tok] | Group | Option
Group      = "("Expression")"
Option     = "["Expression"]"["…"]
```

Lexical tokens are enclosed in "" or \`\` (i.e. "let", "=", "``" or \`""\`).
Terminal tokens are lowercase. Non-terminal tokens are CamelCase.
- `[a-b]` = characters a through b

Note that one or more consecutive newlines - potentially intertwined with any
other whitespace, which is ignored - together form a `\n` token. All other
whitespace is ignored, apart from serving to separate non-newline tokens.

-----------

### General
```EBNF
Syntax     = [Expression]
Expression = CmdCall|FuncUse|block

CmdCall = (BlockDef|GroupDef|TokenDef|Lambda) [Expression]
FuncUse = (IdentToken|Argument) [IdentToken|Argument]…
```

Note that `FuncUse` must always contain *at least* one `IdentToken`.

### Block Definitions
```EBNF
BlockDef = "defblock" BVariant BStart BEnd BEsc
BVariant = "comment"|"string"|"cstring"|"custom"
BStart   = regex_with_var_def_and_interpolation
BEnd     = regex_with_var_def_and_interpolation
BEsc     = regex_with_var_def_and_interpolation | "(" ")"
```

### Group/Token Definitions
```EBNF
GroupDef = "defgroup"  "(" [token]… ")"
TokenDef = "deftokens" "(" [token]… ")"
```

### λλλλλ
```EBNF
Lambda    = "λ" ("transform"|"function") ParamList FuncBody [ArgList]
ParamList = "(" [Parameter]… ")"
FuncBody  = "(" [Expression] ")"
ArgList   = "(" [Argument]…  ")"

Parameter = "(" ParName [Type] ")" | "'" Type "'"
ParName   = "(" [AttrList] FuncPart [FuncPart]… ")" | IdentToken
FuncPart  = Parameter|IdentToken

Argument = Expression | Builtin | "$" [IdentToken]
```

### Type System
```EBNF
Type = "(" [Type] ")"
     | "<" [Type] ">"
     | TypeOp
     | MatPat
     | Single

TypeOp = [Type] "&" [Type]
       | [Type] "|" [Type]
       |        "~" [Type]

MatPat = ("#" Parameter | IdentToken) ["#" Parameter | IdentToken]…
Single = Builtin|block
```

### Attributes
```EBNF
AttrList  = "##" "(" [Attribute]… ")"
Attribute = "(" (PreceAttr|AssocAttr) ")"

PreceAttr = Builtin "precedes" Builtin
AssocAttr = "assoc" ("none"|"left"|"right")
```

Precedence relations form a DAG - see figure 1 in the following paper:
https://www.cse.chalmers.se/~nad/publications/danielsson-norell-mixfix.pdf

Default associativity should probably be "none" or "left".

### Foreign Function Interface
```EBNF
RunForeign = "runforeign" "rust" block
```

### Misc.
```EBNF
Builtin    = "this"|"..."|"_"
IdentToken = token
```

Comments and whitespace can be inserted anywhere.
Note however that newlines count as tokens and are sometimes used.
