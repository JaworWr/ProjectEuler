OPT = -O2

CFLAGS := $(shell pkg-config --cflags fortran_stdlib)
LIBS := $(shell pkg-config --libs fortran_stdlib)

%: %.f90
	gfortran $(CFLAGS) $(OPT) -o $@ $< $(LIBS)

modclean:
	rm -f *.mod *.smod
