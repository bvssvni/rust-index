
LIB = src/lib.rs
TESTS = src/tests.rs

all: lib

test: lib $(TESTS)
	rustc --opt-level=3 --test $(TESTS) -o bin/tests -L bin/ && ./bin/tests

bench: lib $(TESTS)
	rustc --opt-level=3 --test $(TESTS) -o bin/tests -L bin/ && ./bin/tests --bench

lib: $(LIB) bin
	rustc $(LIB) --out-dir bin/

bin:
	mkdir bin

doc: $(LIB)
	rustdoc $(LIB)

clean:
	rm bin/*

