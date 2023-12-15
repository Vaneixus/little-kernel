all: run

run: build
	bash desktop-lite-debian.sh
	/usr/local/share/desktop-init.sh
	qemu-system-arm -M raspi2b -kernel build/kernel.elf -serial stdio

build: clean
	mkdir -p build/dep
	arm-none-eabi-gcc -mfloat-abi=hard -mcpu=cortex-a7 -fpic -ffreestanding -c src/boot.s -o build/dep/boot.o
	xargo build  --target armv7a-none-eabihf --release
	arm-none-eabi-gcc -T src/linker.ld -o build/kernel.elf -ffreestanding -O2 -nostdlib build/dep/boot.o target/armv7a-none-eabihf/release/liblittlekernel.a -lgcc

clean:
	rm -rf build