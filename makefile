.PHONY: run export


run:
	DATABASE_URL=./shc.db cargo run -q


export:
	. ./export.sh