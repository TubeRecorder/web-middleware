
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
	--stdout-log \
	--database-name test\
  --database-username test_user \
  --database-password test_pass

fmt:
	/usr/bin/find ./ -type f -name "*.rs" -exec rustfmt {} \;
