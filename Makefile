precommit_init:
	pre-commit install

doc:
	cargo doc --no-deps

docs_build: doc
docs_rebuild: doc

test:
	cargo test
