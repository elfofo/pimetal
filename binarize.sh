cd obj

~/gcc-arm-eabi/bin/arm-none-eabi-gcc -O0 -mfpu=vfp -mfloat-abi=hard -march=armv6zk -mtune=arm1176jzf-s -nostartfiles kernel.o -o kernel.elf

~/gcc-arm-eabi/bin/arm-none-eabi-objcopy kernel.elf -O binary kernel.img

cp kernel.img /run/media/elfofo/NOUVEAU\ NOM/kernel.img

