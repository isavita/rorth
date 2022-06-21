#!/bin/sh

set -ex

as output.asm -o output.o
ld output.o -o output -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path`
