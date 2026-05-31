.PHONY: fmt

fmt:
	@echo "===> Formatting"
	black .
	isort .

install:
	@echo "===> Installing"
	pip install -U .[dev]
