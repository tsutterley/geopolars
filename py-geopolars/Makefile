.DEFAULT_GOAL := help

SHELL=/bin/bash
PYTHON=venv/bin/python
PYTHON_BIN=venv/bin

venv:  ## Set up virtual environment
	@python -m venv venv
	@venv/bin/pip install -U pip
	@venv/bin/pip install -r build.requirements.txt
	@unset CONDA_PREFIX && source venv/bin/activate && maturin develop

.PHONY: clean
clean:  ## Clean up caches and build artifacts
	@rm -rf venv/
	@rm -rf docs/build/
	@rm -rf docs/source/reference/api/
	@rm -rf .hypothesis/
	@rm -rf .mypy_cache/
	@rm -rf .pytest_cache/
	@rm -f .coverage
	@rm -f coverage.xml
	@rm -f geopolars.data/data/proj_data
	@find -type f -name '*.py[co]' -delete -or -type d -name __pycache__ -delete
	@cargo clean

# We use pre-commit separately
# .PHONY: pre-commit
# pre-commit: venv  ## Run autoformatting and linting
# 	$(PYTHON_BIN)/isort .
# 	$(PYTHON_BIN)/black .
# 	$(PYTHON_BIN)/blackdoc .
# 	$(PYTHON_BIN)/pyupgrade --py37-plus `find geopolars/ tests/ -name "*.py" -type f`
# 	$(PYTHON_BIN)/mypy
# 	$(PYTHON_BIN)/flake8
# 	make -C .. fmt_toml
# 	cargo fmt --all

.PHONY: clippy
clippy:  ## Run clippy
	cargo clippy -- -D warnings -A clippy::borrow_deref_ref

.PHONY: test
test: venv  ## Run fast unittests
	$(PYTHON_BIN)/maturin develop
	$(PYTHON) -m pytest tests/unit/

.PHONY: test-all
test-all: venv  ## Run all tests
	$(PYTHON_BIN)/maturin develop
	$(PYTHON) -m pytest
	$(PYTHON) tests/docs/run_doc_examples.py

.PHONY: test-with-cov
test-with-cov: venv  ## Run tests and report coverage
	$(PYTHON) -m pytest --cov=geopolars --cov-report xml

.PHONY: doctest
doctest:  ## Run doctests
	$(PYTHON) tests/docs/run_doc_examples.py

.PHONY: install-wheel
install-wheel:
	pip install --force-reinstall -U wheels/geopolars-*.whl

.PHONY: copy-proj-data
copy-proj-data:  ## Copy PROJ data into the working tree for bundling with the wheel
	python copy-proj-data.py

.PHONY: build-no-venv
build-no-venv: copy-proj-data
	maturin build -o wheels --features "proj"

.PHONY: build-no-venv-release
build-no-venv-release: copy-proj-data
	maturin build -o wheels --release --features "proj"

.PHONY: build-and-test-no-venv
build-and-test-no-venv: copy-proj-data
	maturin build -o wheels
	pip install --force-reinstall -U wheels/geopolars-*.whl
	pytest tests

.PHONY: install-no-venv
install-no-venv: build-no-venv install-wheel

.PHONY: install-no-venv-release
install-no-venv-release: build-no-venv-release install-wheel

.PHONY: help
help:  ## Display this help screen
	@echo -e '\033[1mAvailable commands:\033[0m'
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-18s\033[0m %s\n", $$1, $$2}'
