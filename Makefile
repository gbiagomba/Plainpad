# Makefile (LITE)
# Satisfies RULE 2 - maintain Makefile
# Satisfies RULE 1 - test before delivery

APP_NAME := program_name
CARGO := cargo

.PHONY: all build release install run test clean

all: release

build:
	$(CARGO) build --release

release: build

install:
	$(CARGO) install --path .

run:
	$(CARGO) run --release -- $(ARGS)

test:
	$(CARGO) test

clean:
	$(CARGO) clean
