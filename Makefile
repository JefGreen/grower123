kernel7.img:
	/Applications/ArmGNUToolchain/12.3.rel1/arm-none-eabi/arm-none-eabi/bin/objcopy -O binary target/armv7a-none-eabi/debug/grower123 ./kernel7.img

clean:
	rm -f kernel7.img
