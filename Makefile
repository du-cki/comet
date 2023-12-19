##
# Comet
#
# @file
# @version 0.1

build:
	pnpm -C ui/ build
	cargo build --release

run:
	$(MAKE) build
	RUST_LOG=none,comet=debug cargo run --release

dev:
	RUST_LOG=none,comet=debug cargo run &
	pnpm -C ui/ dev --host --clearScreen false 

install:
	SQLX_OFFLINE=true cargo build
	pnpm -C ui/ install

prepare:
	cargo sqlx prepare --database-url='sqlite://data.db'

clean:
	cargo clean
	rm -rf ui/node_modules


# end
