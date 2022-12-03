SOURCES := $(shell find . -type f -name "*.ts")
TARGET := $(notdir $(CURDIR))

$(TARGET): $(SOURCES)
	@npm run build
	@node dist/index.js

test:
	@npm run test

.PHONY: test
