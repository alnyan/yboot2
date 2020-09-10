TARGET=x86_64-unknown-uefi
RUST_SRC=$(shell find src -type f -name "*.rs") \
		 Cargo.toml
O=target/$(TARGET)/debug

all: $(O)/image.fat32

clean:
	cargo clean

$(O)/yboot2.efi: $(RUST_SRC)
	cargo build -Z build-std=core

$(O)/image.fat32: $(O)/yboot2.efi
	dd if=/dev/zero of=$@ bs=1M count=64
	mkfs.vfat -F32 $@
	mcopy -i $@ $(O)/yboot2.efi ::app.efi
