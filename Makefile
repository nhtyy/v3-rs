install:
	cd bindings && cargo run
	cd bindings && mv src/lib.txt src/lib.rs