SHELL := /bin/bash

db_url := postgres://postgres:postgres@192.168.68.102:5432/sadhana_pro

run_server:
	DATABASE_URL=$(db_url) \
		RUST_BACKTRACE=full \
		cargo run --bin server

run: 
	cd frontend; trunk build
	DATABASE_URL=$(db_url) \
		RUST_BACKTRACE=full \
		cargo run --bin server

create_migration:
	DATABASE_URL=$(db_url) diesel migration generate $(name) --migration-dir=migrations

migrate:
	DATABASE_URL=$(db_url) diesel migration run --migration-dir=migrations

redo_migrate:
	DATABASE_URL=$(db_url) diesel migration redo --migration-dir=migrations

reset_db:
	DATABASE_URL=$(db_url) diesel database reset --migration-dir=migrations

gen_schema:
	DATABASE_URL=$(db_url) diesel print-schema > server/src/schema.rs

test:
	DATABASE_URL=$(db_url) \
		cargo test $(T) -- --nocapture --test-threads=1

lint:
	@rustup component add clippy 2> /dev/null
	cargo clippy --all-targets --all-features -- -D warnings

# non-file target for make
.PHONY: run_server run create_migration migrate redo_migrate reset_db gen_schema test lint
