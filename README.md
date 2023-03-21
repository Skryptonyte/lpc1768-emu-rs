# LPC1768-emu-rs

This is an experimental emulator which intends to eventually fully replicate the microcontroller LPC1768 developed by NXP and potentially other microcontrollers using the same CPU as well.

# What works?

As of present, there's only a bare implementation of a Cortex M3 cpu with a small subset of thumb1 instructions supported. There can and will be incorrect behaviour!

# How to test

There are three example assembly files provided which are tested to work as expected on this emulator. I have provided a wrapper script compile-lpc.sh which will compile an asm file and create obj files in cwd which will be used by the program. 

Note: You will require arm-none-eabi-gcc toolchain to compile the assembly files.

# References:

1. https://developer.arm.com/documentation/ddi0406/c/Application-Level-Architecture/Thumb-Instruction-Set-Encoding?lang=en