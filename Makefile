##
# Comet
#
# @file
# @version 0.1

.PHONY: help
help:
	@echo "Available targets:"
	@echo "    build:   build the project via docker buildx"
	@echo "    run:     run the project"
	@echo "    clean:   remove all the project dependency files"
	@echo "    prepare: prepares the SQL queries for offline compilation"
	@echo "    help:    shows this message"

.PHONY: build
build:
	docker buildx build -t comet .


.PHONY: run
run:
	$(MAKE) build
	docker run comet


.PHONY: prepare
prepare:
	cargo sqlx prepare --database-url='sqlite://data.db' -- --all-targets --all-features


.PHONY: clean
clean:
	cargo clean
	rm -rf ui/node_modules ui/dist

# end
