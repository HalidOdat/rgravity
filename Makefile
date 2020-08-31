COMPILER_DIR = src/compiler/
RUNTIME_DIR = src/runtime/
SHARED_DIR = src/shared/
UTILS_DIR = src/utils/
OPT_DIR = src/optionals/
GRAVITY_SRC = src/cli/gravity.c
EXAMPLE_SRC = examples/example.c

CC ?= gcc
INCLUDE = -I$(COMPILER_DIR) -I$(RUNTIME_DIR) -I$(SHARED_DIR) -I$(UTILS_DIR) -I$(OPT_DIR)
CFLAGS = $(INCLUDE) -std=gnu99 -fgnu89-inline -fPIC -DBUILD_GRAVITY_API

ifeq ($(mode),debug)
	MODE = ""
	LIB_GRAVITY = "target/debug/"
else
	MODE = "--release"
	LIB_GRAVITY = "target/release/"
endif

all: gravity example

gravity: $(GRAVITY_SRC)
	cargo build $(MODE)
	$(CC) $(CFLAGS) -o $@ $^ -L $(LIB_GRAVITY) -lgravity_sys -lm


example: $(EXAMPLE_SRC)
	cargo build $(MODE)
	$(CC) $(CFLAGS) -o $@ $^ -L $(LIB_GRAVITY) -lgravity_sys -lm

clean:
	cargo clean
	rm -f gravity example

.PHONY: all clean gravity example
