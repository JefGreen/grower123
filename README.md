# grower123

# Get started using
```bash
cargo build
```

```bash
cargo rustc -- -C link-arg=--script=./linker.ld
```

# View built file using
```bash
objdump -D target/armv7a-none-eabi/debug/grower123 | less
```

Install target
```
rustup target add armv7a-none-eabi
```

 objdump -D target/armv7a-none-eabi/debug/grower123 | less