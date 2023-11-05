# grower123

# Get started using
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
objdump -D target/armv7a-none-eabi/debug/grower123 | less
```

Install target
```
rustup target add armv7a-none-eabi
```
