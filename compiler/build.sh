#!/bin/bash

set -e

./old/alpha-03/bin/ppc.out src/main.ppl bin/main.c

gcc -std=c11 bin/main.c -o bin/ppc.out

rm bin/main.c