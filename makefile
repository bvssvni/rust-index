
COMPILER ?= rustc
SOURCES = index.rs

all: index

test: index
	./index

index: $(SOURCES)
	rustc --test $(SOURCES) -o index

clean:
	rm index

