ARMGNU ?= arm-none-eabi
BUILD = build
TARGET = kernel.img

SRCS = $(shell find -name main.rs -or -name *.S)
DIRS = $(dir $(SRCS))
BUILDDIRS = $(addprefix $(BUILD)/, $(DIRS))

OBJS := $(SRCS:.rs=.o)
OBJS := $(OBJS:.S=.o)
OBJS := $(addprefix $(BUILD)/, $(OBJS))

.PHONY: default all clean

default:
	@[ -d $(BUILD) ] || mkdir -p $(BUILD)
	@[ -d "$(BUILDDIRS)" ] || mkdir -p $(BUILDDIRS)
	@make all --no-print-directory

all: $(TARGET)

$(TARGET): $(BUILD)/output.elf
	$(ARMGNU)-objcopy $< -O binary $@

$(BUILD)/output.elf: $(OBJS)
	$(ARMGNU)-ld -static -nostdlib -T kernel.ld -o $@ $^ `$(ARMGNU)-gcc -print-libgcc-file-name`

$(BUILD)/%.o: %.S
	$(ARMGNU)-as -mfpu=neon-vfpv4 -mfloat-abi=hard -march=armv7-a -o $@ $<

$(BUILD)/%.o: %.rs  $(shell find -name *.rs)
	rustc -O --target armv7-unknown-linux-gnueabihf --emit=obj $< -o $@

clean:
	rm -rf build
	rm -rf $(TARGET)
