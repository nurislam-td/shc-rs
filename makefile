.PHONY: run


run:
	DATABASE_URL=./shc.db cargo run -q
