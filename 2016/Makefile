YEAR := all $(notdir $(CURDIR))
DAYS := $(wildcard day*)

$(YEAR): $(DAYS)
$(DAYS):
	@$(MAKE) -C $@

clean:
	@find . -type f -name "*.class" -delete

.PHONY: $(DAYS) $(YEAR) clean

