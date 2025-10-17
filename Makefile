BUILD_CMD := maturin develop

.DEFAULT_GOAL := help
.PHONY: help build-dev build-prod test fmt

help:
	@echo "RapidQuery Project Management"
	@echo ""
	@echo -e "    build-dev     build source"
	@echo -e "    build-prod    build source (release mode)"
	@echo -e "    test          clippy and test rust code"
	@echo -e "    fmt           format rust and python code"

build-dev:
	$(BUILD_CMD) --uv

build-prod:
	$(BUILD_CMD) --uv --release

test:
	cargo clippy
	pytest -s -vv
	-rm -rf .pytest_cache
	-ruff check .
	ruff clean

fmt:
	cargo fmt
	ruff format --line-length=100 .
	ruff clean
