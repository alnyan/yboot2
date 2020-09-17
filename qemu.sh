#!/bin/sh

BIOS=/usr/share/edk2-ovmf/OVMF_CODE.fd
IMAGE=image/image.fat32
TARGET=x86_64-unknown-uefi
CONFIG=debug

set -e

mkdir -p image

cargo build -Z build-std=core

dd if=/dev/zero of=${IMAGE} bs=1M count=64
mkfs.vfat -F32 ${IMAGE}
mcopy -i ${IMAGE} target/${TARGET}/${CONFIG}/yboot2.efi ::app.efi
mcopy -i ${IMAGE} image/initrd.img ::initrd.img
mcopy -i ${IMAGE} image/kernel.elf ::kernel.elf

qemu-system-x86_64 \
    -s \
    -serial stdio \
    -m 256 \
    -drive format=raw,file=$BIOS,readonly=on,if=pflash \
    -drive format=raw,file=$IMAGE \
    -net none \
    -enable-kvm \
    -M q35 \
    -cpu host
