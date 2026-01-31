# Makefile (LITE)
# Satisfies RULE 2 - maintain Makefile
# Satisfies RULE 1 - test before delivery

APP_NAME := plainpad
CARGO := cargo

.PHONY: all build release install run test fmt clippy ci clean

all: release

build:
	$(CARGO) build --release

release: build

install:
	$(CARGO) install --path .

run:
	$(CARGO) run --release

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt --all

clippy:
	$(CARGO) clippy --all-targets -- -D warnings

ci: fmt clippy test build

clean:
	$(CARGO) clean
