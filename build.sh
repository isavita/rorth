#!/bin/sh

set -ex

as hello.asm -o hello.o
ld hello.o -o hello -lSystem -syslibroot `xcrun -sdk macosx --show-sdk-path`
