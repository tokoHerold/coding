CC = gcc
#No optimization
CFLAGS = -O0

.PHONY: all clean 

BUILD_DIR := .build
OUT_DIR := out
CACHE := $(patsubst src/cache/%.c,$(OUT_DIR)/cache/%,$(wildcard src/cache/*.c)) src/temp

# print:
# 	echo print $(cache)

all: cache

cache: $(CACHE)

clean:
	rm -rf $(BUILD_DIR) $(OUT_DIR)

# $(BUILD_DIR)/%.o: src/%.c $(BUILD_DIR)
# 	$(CC) $(CFLAGS) -c $< -o $(BUILD_DIR)/$(@F).o

$(OUT_DIR)/%: src/%.c#src/$(*).c $(OUT_DIR)
	echo $(CC) $(BUILD_DIR)/$(@F).o -o $(OUT_DIR)/$(@F)

$(GENERATED_DIR):
	mkdir -p $@



