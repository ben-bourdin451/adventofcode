DAYS := $(wildcard day*/.)
SOURCES := $(shell find . -type f -name "*.scala")
TARGET := $(notdir $(CURDIR))

$(TARGET): $(SOURCES)
	@scalac $(SOURCES)
	@scala Main

all: $(DAYS)
$(DAYS):
	$(MAKE) -C $@

clean: 
	@find . -type f -name "*.class" -delete

.PHONY: all $(DAYS)

