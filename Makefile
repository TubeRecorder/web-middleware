
all:
	make clean
	make build
	make test
	make run

clean:
	cargo clean

build:
	cargo build

test:
	cargo test

run:
	cargo run --bin server -- \
	--stdout-log
