build:
	xargo +nightly build -Z build-std --target aarch64-unknown-none -emit=obj --release

run:
	qemu-system-arm -M raspi2b -serial stdio -kernel "./target/aarch64-unknown-none/release/little-kernel"

clean:
	rm -rf ./target
