SRC = $(wildcard src/*.rs)

all: $(SRC) psp.json
	cargo build --target psp
	./gen-eboot.sh


psp.json: psp.json.in
	./gen-target.sh
