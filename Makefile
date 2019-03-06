TSC_FLAGS := --target ES2015 --module system


.PHONY: build
build: dist/app.js dist/service-worker.js
	cp timer.webmanifest dist/
	cp app.css dist/
	cp index.html dist/
	cp -r images dist/
	cp -r sounds dist/

.PHONY: clean
clean:
	-rm -r dist

dist/%.js: src/%.ts
	tsc $(TSC_FLAGS) --outFile $@ $^
