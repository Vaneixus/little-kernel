build:
	xargo +nightly build -Z build-std --target armv7a-none-eabihf --release

run: build


clean:
	rm -rf ./target
