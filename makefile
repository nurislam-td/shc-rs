
include .env
export


.PHONY: run migrate migration downgrade


run:
	cargo run -q


migrate:
	diesel migration run


# make migration m=some-name
migration:
	diesel migration generate $(m)

downgrade:
	diesel migration revert