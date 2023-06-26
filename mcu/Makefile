#
# SPDX-License-Identifier: GPL-2.0
# Copyright (c) 向阳, all rights reserved.
#

TARGET = fraduino

CONFIGURE_FILE = Configure.mk

include $(CONFIGURE_FILE)

# 编译时显示的内容
CC_DISPLAY = CC:

# 链接时显示的内容
LD_DISPLAY = Linker output :

# Table generator
TBGEN_DISPLAY = TB:

# Build path
BUILD_DIR = build

# target
OUTPUT_DIR = target

# C sources
C_SOURCES =  \
$(wildcard ./src/*.c) \
$(wildcard ./src/**/*.c) \
$(wildcard ./mcu/Core/Src/*.c) \
$(wildcard ./mcu/Drivers/STM32L4xx_HAL_Driver/Src/*.c)

# ASM sources
ASM_SOURCES =  \
./mcu/startup_stm32l432xx.s

# linker script
LINK_SCRIPT = \
./mcu/STM32L432KBUx_FLASH.ld

# tbgen
TBGEN_INPUTS = \
$(wildcard ./src/*.tbgen.yml)

PREFIX = arm-none-eabi-
# The gcc compiler bin path can be either defined in make command via GCC_PATH variable (> make GCC_PATH=xxx)
# either it can be added to the PATH environment variable.
ifdef GCC_PATH
CC = $(GCC_PATH)/$(PREFIX)gcc
AS = $(GCC_PATH)/$(PREFIX)gcc -x assembler-with-cpp
CP = $(GCC_PATH)/$(PREFIX)objcopy
SZ = $(GCC_PATH)/$(PREFIX)size
DUMP = $(GCC_PATH)/$(PREFIX)objdump
else
CC = $(PREFIX)gcc
AS = $(PREFIX)gcc -x assembler-with-cpp
CP = $(PREFIX)objcopy
SZ = $(PREFIX)size
DUMP = $(PREFIX)objdump
endif
# Python
ifdef PYTHON_PATH
PY = $(PYTHON_PATH)/python
else
PY = python
endif
# 格式转换
HEX = $(CP) -O ihex
BIN = $(CP) -O binary -S

# cpu
CPU = -mcpu=cortex-m4

# fpu
FPU = -mfpu=fpv4-sp-d16

# float-abi
FLOAT-ABI = -mfloat-abi=hard

# mcu
MCU = $(CPU) -mthumb $(FPU) $(FLOAT-ABI)

# macros for gcc
# AS defines
AS_DEFS = \
$(C_CONFIG)

# C defines
C_DEFS =  \
-DUSE_FULL_LL_DRIVER \
-DUSE_HAL_DRIVER \
-DSTM32L432xx\
$(C_CONFIG)


# AS includes
AS_INCLUDES = 

# C includes
C_INCLUDES =  \
-Imcu/Core/Inc \
-Imcu/Drivers/STM32L4xx_HAL_Driver/Inc \
-Imcu/Drivers/STM32L4xx_HAL_Driver/Inc/Legacy \
-Imcu/Drivers/CMSIS/Device/ST/STM32L4xx/Include \
-Imcu/Drivers/CMSIS/Include \
-Iinc

# Standard
C_STANDARD = --std=c11

# compile gcc flags
ASFLAGS = $(MCU) $(AS_DEFS) $(AS_INCLUDES) $(OPT) -Wall -fdata-sections -ffunction-sections

ifeq ($(DEBUG), 1)
CFLAGS = $(MCU) $(C_STANDARD) $(C_DEFS) $(C_INCLUDES) $(OPT) -D_DEBUG -Wall -fdata-sections -ffunction-sections -g -gdwarf-2
else
CFLAGS = $(MCU) $(C_STANDARD) $(C_DEFS) $(C_INCLUDES) -flto $(OPT) -Wall -fdata-sections -ffunction-sections
endif

# Generate dependency information
CFLAGS += -MMD -MP -MF"$(@:%.o=%.d)"

# libraries
LIBS = -lc -lm
LIBDIR =
LDFLAGS = $(MCU) -specs=nano.specs -T$(LINK_SCRIPT) $(LIBDIR) $(LIBS) -Wl,-Map=$(OUTPUT_DIR)/$(TARGET).map,--cref -Wl,--no-warn-rwx-segments,--gc-sections
ifeq ($(DEBUG), 0)
LDFLAGS += -flto
endif

# output files

OBJECTS = $(addprefix $(BUILD_DIR)/,$(notdir $(C_SOURCES:.c=.o)))
vpath %.c $(sort $(dir $(C_SOURCES)))
# list of ASM program objects
OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(ASM_SOURCES:.s=.o)))
vpath %.s $(sort $(dir $(ASM_SOURCES)))
# list of auto generated program objects
OBJECTS += $(addprefix $(BUILD_DIR)/,$(notdir $(TBGEN_INPUTS:.tbgen.yml=.o)))
vpath %.tbgen.yml $(sort $(dir $(TBGEN_INPUTS)))

# build all
all: $(OUTPUT_DIR)/$(TARGET).elf $(OUTPUT_DIR)/$(TARGET).hex $(OUTPUT_DIR)/$(TARGET).bin
	@echo build completed.

$(BUILD_DIR)/%.o: %.c Makefile $(CONFIGURE_FILE) | $(BUILD_DIR)
	@echo $(CC_DISPLAY) $<
	@$(CC) -c $(CFLAGS) $< -o $@

$(BUILD_DIR)/%.o: %.s Makefile $(CONFIGURE_FILE) | $(BUILD_DIR)
	@echo $(AS_DISPLAY) $<
	@$(AS) -c $(ASFLAGS) $< -o $@

$(BUILD_DIR)/%.o: %.tbgen.yml Makefile $(CONFIGURE_FILE) | $(BUILD_DIR)
	@echo $(TBGEN_DISPLAY) $<
	@$(PY) tbgen --input $< --output "$(@:%.o=%.c)"
	@echo $(CC_DISPLAY) $(@:%.o=%.c)
	@$(CC) -c $(CFLAGS) "$(@:%.o=%.c)" -o $@

$(OUTPUT_DIR)/$(TARGET).elf: $(OBJECTS) Makefile $(CONFIGURE_FILE) | $(OUTPUT_DIR)
	@echo $(LD_DISPLAY) $@
	@$(CC) $(OBJECTS) $(LDFLAGS) -o $@
	@$(DUMP) -d $@ > $(OUTPUT_DIR)/$(TARGET).s
	@$(SZ) $@

$(OUTPUT_DIR)/%.hex: $(OUTPUT_DIR)/%.elf | $(OUTPUT_DIR)
	@$(HEX) $< $@
	
$(OUTPUT_DIR)/%.bin: $(OUTPUT_DIR)/%.elf | $(OUTPUT_DIR)
	@$(BIN) $< $@
	
$(BUILD_DIR):
	mkdir $@

$(OUTPUT_DIR):
	mkdir $@

-include $(wildcard $(BUILD_DIR)/*.d)
