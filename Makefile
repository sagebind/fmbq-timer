.PHONY: build
build: app.js

.PHONY: clean
clean:
	rm app.js

app.js: $(wildcard src/*.ts)
	tsc --target ES2015 --module system --outFile $@ src/app.ts
