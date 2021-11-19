TARGET = aarch64-unknown-none-softfloat
KERNEL_BIN = kernel8.img
LINKER_FILE = src/pi/link.ld

KERNEL_ELF = target/$(TARGET)/release/myPiOs

COMPILER_ARGS = --target=$(TARGET)

RUSTFLAGS = -C link-arg=$(LINKER_FILE) -C debuginfo=2

RUSTC_CMD = cargo rustc $(COMPILER_ARGS) --release
OBJCOPY_CMD = rust-objcopy --strip-all -O binary

OBJDUMP_CMD = rust-objdump -d --print-imm-hex

.PHONY: all $(KERNEL_BIN) $(KERNEL_ELF)

all: $(KERNEL_BIN)

$(KERNEL_ELF):
	@RUSTFLAGS="$(RUSTFLAGS)" $(RUSTC_CMD)

$(KERNEL_BIN): $(KERNEL_ELF)
	@$(OBJCOPY_CMD) $(KERNEL_ELF) $(KERNEL_BIN)


objdump:
	@$(OBJDUMP_CMD) $(KERNEL_ELF)

install:
	ttywrite -i $(KERNEL_BIN) /dev/$(USB)