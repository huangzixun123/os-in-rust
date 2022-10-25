build:
	cargo build
image:
	cargo bootimage
run:
	cargo run

all:build image run