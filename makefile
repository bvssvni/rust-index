
LIB = src/lib.rs
TESTS = src/tests.rs

all: lib

test: lib $(TESTS)
	rustc --test $(TESTS) -o bin/tests -L bin/ && ./bin/tests

lib: $(LIB) bin
	rustc $(LIB) --out-dir bin/

bin:
	mkdir bin

doc: $(LIB)
	rustdoc $(LIB)

clean:
	rm bin/*

