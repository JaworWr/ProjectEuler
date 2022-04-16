#!/bin/bash
set -eEuo pipefail

CFLAGS=$(pkg-config --cflags fortran_stdlib)
LIBS=$(pkg-config --libs fortran_stdlib)

gfortran $CFLAGS -O2 -o $1 $1.f90 $LIBS
