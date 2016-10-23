ARMGNU ?= arm-none-eabi
BUILD = build
TARGET = kernel.img
RUST_OBJ = ./target/armv7-unknown-linux-gnueabihf/release/libruspberry_os.a

SRCS = $(shell find -name *.S)
DIRS = $(dir $(SRCS))
BUILDDIRS = $(addprefix $(BUILD)/, $(DIRS))

OBJS := $(SRCS:.S=.o)
OBJS := $(addprefix $(BUILD)/, $(OBJS))

.PHONY: default all clean

default:
	@[ -d $(BUILD) ] || mkdir -p $(BUILD)
	@[ -d "$(BUILDDIRS)" ] || mkdir -p $(BUILDDIRS)
	@make all --no-print-directory

all: $(TARGET)

$(TARGET): $(BUILD)/output.elf
	$(ARMGNU)-objcopy $< -O binary $@

$(BUILD)/output.elf: $(OBJS) $(RUST_OBJ)
	$(ARMGNU)-ld -static -nostdlib -T kernel.ld -o $@ $^ `$(ARMGNU)-gcc -print-libgcc-file-name`

$(BUILD)/%.o: %.S
	$(ARMGNU)-as -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -o $@ $<

$(RUST_OBJ): $(shell find -name *.rs)
	@cargo build --release --verbose

clean:
	rm -rf $(TARGET)
	rm -rf build
	cargo clean
