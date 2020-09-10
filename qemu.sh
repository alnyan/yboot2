#!/bin/sh

BIOS=/usr/share/edk2-ovmf/OVMF_CODE.fd
IMAGE=target/x86_64-unknown-uefi/debug/image.fat32

make

qemu-system-x86_64 \
    -drive format=raw,file=$BIOS,readonly=on,if=pflash \
    -drive format=raw,file=$IMAGE \
    -net none \
    -enable-kvm \
    -M q35 \
    -cpu host
