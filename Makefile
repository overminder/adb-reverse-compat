ANDROID_TARGET := arm-linux-androideabi
ANDROID_STRIP = $(ANDROID_TARGET)-strip
RUST_SRCS := $(wildcard src/*.rs)
CARGO := cargo
STRIP := strip

BIN_CLIENT := adb-reverse-client
BIN_SERVER := adb-reverse-server
BINS := $(BIN_CLIENT) $(BIN_SERVER)

all : $(BINS)

$(BIN_CLIENT) : $(RUST_SRCS)
	$(CARGO) rustc --bin $@ --target=$(ANDROID_TARGET) --release
	cp target/$(ANDROID_TARGET)/release/$@ ./
	$(ANDROID_STRIP) $@

$(BIN_SERVER) : $(RUST_SRCS)
	$(CARGO) rustc --bin $@ --release
	cp target/release/$@ ./
	$(STRIP) $@

test :
	$(CARGO) test --lib

clean :
	rm $(BINS)
	$(CARGO) clean

