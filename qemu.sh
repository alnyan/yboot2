#!/bin/sh

BIOS=/usr/share/edk2-ovmf/OVMF_CODE.fd
IMAGE=target/x86_64-unknown-uefi/debug/image.fat32

set -e

cargo build -Z build-std=core

dd if=/dev/zero of=${IMAGE} bs=1M count=64
mkfs.vfat -F32 ${IMAGE}
mcopy -i ${IMAGE} target/x86_64-unknown-uefi/debug/yboot2.efi ::app.efi

qemu-system-x86_64 \
    -drive format=raw,file=$BIOS,readonly=on,if=pflash \
    -drive format=raw,file=$IMAGE \
    -net none \
    -enable-kvm \
    -M q35 \
    -cpu host
