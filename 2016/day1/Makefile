SOURCES := $(shell find . -type f -name "*.scala")
TARGET := $(notdir $(CURDIR))

$(TARGET): $(SOURCES)
	@scalac $(SOURCES)
	@scala Main

