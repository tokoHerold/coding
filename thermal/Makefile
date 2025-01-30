CC = gcc
#No optimization
CFLAGS = -O0

.PHONY: all clean

BUILD_DIR := .build
OUT_DIR := out
DIRS = $(BUILD_DIR) $(OUT_DIR)
CACHE := $(patsubst src/cache/%.c,$(OUT_DIR)/cache/%,$(wildcard src/cache/*.c)) $(OUT_DIR)/temp
PAGE := $(patsubst src/page/%.c,$(OUT_DIR)/page/%,$(wildcard src/page/*.c)) $(OUT_DIR)/temp

print:
	echo print $(CACHE)

all: cache pagemap

cache: $(CACHE)
page: $(PAGE)

clean:
	rm -rf $(BUILD_DIR) $(OUT_DIR)

# Create all objects in the .buid directory
$(BUILD_DIR)/%.o: src/%.c $(BUILD_DIR)
	$(CC) $(CFLAGS) -c $< -o $(BUILD_DIR)/$(@F)

# Create the executables from the object files in the build directory
$(OUT_DIR)/%: $(BUILD_DIR)/%.o $(OUT_DIR)
	$(CC) $(BUILD_DIR)/$(@F).o -o $(OUT_DIR)/$(@F)

$(DIRS):
	mkdir -p $@
