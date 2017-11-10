#!/bin/bash

set -e

./old/alpha-03/bin/ppc.out src/main.ppl bin/main.c -d
./old/alpha-03/bin/ppc.out src/preprocessor.ppl bin/preprocessor.c -d
./old/alpha-03/bin/ppc.out src/lexer.ppl bin/lexer.c -d
./old/alpha-03/bin/ppc.out src/parser.ppl bin/parser.c -d

gcc -std=c11 -g bin/main.c bin/preprocessor.c bin/lexer.c bin/parser.c -o bin/ppc.out

rm bin/main.c
rm bin/preprocessor.c
rm bin/lexer.c
rm bin/parser.c