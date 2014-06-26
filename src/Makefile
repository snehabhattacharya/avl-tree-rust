all: tree.rs avl_tree.rs test.rs
	rustc --crate-type=lib tree.rs
	rustc --test test.rs -L .

check:
	./test

clean:
	rm *.rlib
