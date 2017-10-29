#!/bin/bash

set -e

./old/bin/ppc.out src/main.ppl bin/main.c
./old/bin/ppc.out src/preprocessor.ppl bin/preprocessor.c
./old/bin/ppc.out src/lexer.ppl bin/lexer.c
./old/bin/ppc.out src/parser.ppl bin/parser.c
./old/bin/ppc.out src/def.h bin/def.h

gcc -std=c11 bin/main.c bin/preprocessor.c bin/lexer.c bin/parser.c -o bin/ppc.out

rm bin/main.c
rm bin/preprocessor.c
rm bin/lexer.c
rm bin/parser.c
rm bin/def.h