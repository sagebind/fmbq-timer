TSC_FLAGS := --target ES2015 --module system
STATIC_FILES := $(shell find static -type f)
TS_FILES := $(shell find src -type f)


.PHONY: build
build: dist/app.js dist/service-worker.js $(STATIC_FILES:static/%=dist/%)

.PHONY: open
open: build
	open dist/index.html

.PHONY: clean
clean:
	-rm -r dist

dist/%.js: src/%.ts $(TS_FILES)
	@mkdir -p $(dir $@)
	tsc $(TSC_FLAGS) --outFile $@ $<

dist/%: static/%
	@mkdir -p $(dir $@)
	cp $< $@
