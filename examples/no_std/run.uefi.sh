mkdir -p target/esp/efi/boot
cp $1 target/esp/efi/boot/bootx64.efi
qemu-system-x86_64 -device isa-debug-exit,iobase=0xf4,iosize=0x04 \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE_4M.fd \
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_VARS_4M.fd \
    -drive format=raw,file=fat:rw:target/esp \
    --nographic
if [ $? -ne 123 ]; then
    exit $?
else
    exit 0
fi
