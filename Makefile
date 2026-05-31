.PHONY = all fmt lint test  # install

all: fmt lint test  # install

fmt:
	@echo "===> Formatting"
	@isort . 
	@black .

lint:
	@echo "===> Linting"
	@flake8 .
	@mypy --strict .

test:
	@echo "===> Testing"

# install:
# 	@echo "===> Installing"
# 	@pip install --editable .

