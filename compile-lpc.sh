#!/bin/sh

arm-none-eabi-gcc $1 -mcpu=cortex-m3 -nostdlib -Wl,-Ttext=0x0,-emain -o "elf-im"
arm-none-eabi-objcopy --dump-section .text=obj elf-im 