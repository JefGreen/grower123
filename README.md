# grower123
Is an embeded project written in Rust for a raspberry pi 4 model B.


# Get started using
<!-- TODO: update read me file -->
```bash
cargo build
```

```bash
cargo rustc -- -C link-arg=--script=./linker.ld
```

# create Raspbery pie image
```bash
file target/armv7a-none-eabi/debug/grower123
```

<!-- TODO: fix this command because I currently get command not found: arm-none-eabi-objcopy -->
<!-- /Applications/ArmGNUToolchain/12.3.rel1/arm-none-eabi/arm-none-eabi/bin/objcopy -->
```bash
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/grower123 ./kernel7.img
```

# View built file using
```bash
arm-none-eabi-objdump -D target/armv7a-none-eabi/debug/grower123 | less
```

Install target
```
rustup target add armv7a-none-eabi
```
