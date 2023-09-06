
all: main.js
	node $<

main.js: main.ts node_modules
	npx tsc

node_modules:
	npm install
