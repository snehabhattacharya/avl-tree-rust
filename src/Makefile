all: avl_tree.rs test.rs
	rustc --crate-type=lib avl_tree.rs
	rustc --test test.rs -L .

check:
	./test

clean:
	rm *.rlib
