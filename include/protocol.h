#pragma once
// CPU state when entering kernel:
// Virtual memory: lower 1GiB identity mapped
// Selectors are flat
// TODO: CR0? (spec. if NX, PAE, PSE are enabled)
// From UEFI specification:
//  * CR0.EM = 0
//  * CR0.TS = 0
// Stack:
//  * At least 120KiB available
// FPU state:
//  * CW = 0x037F
// MMX state:
//  * CW = 0x1F80
// Interrupts are disabled

#define YB_KERNEL_MAGIC_V1          0xA197A9B007B007UL
#define YB_LOADER_MAGIC_V1          0x700B700B9A791AUL

#define YB_CMDLINE_SIZE             256

#define YB_VIDEO_FORMAT_RGB32       0
#define YB_VIDEO_FORMAT_BGR32       1

#if !defined(__ASM__)
#include <stdint.h>

struct yboot_header {
    uint64_t kernel_magic;
    uint64_t loader_magic;
};

struct yboot_v1 {
    struct yboot_header header;

    uint64_t memory_map_data;                   // R
    uint32_t memory_map_size;                   // RW
    uint32_t memory_map_entsize;                // W

    // Video mode settings
    uint32_t video_width;                       // RW
    uint32_t video_height;                      // RW
    uint32_t video_format;                      // RW
    uint32_t __pad0;                            // --
    uint64_t video_framebuffer;                 // W
    uint64_t video_pitch;                       // W

    uint64_t elf_symtab_hdr;                    // W
    uint64_t elf_symtab_data;                   // W
    uint64_t elf_strtab_hdr;                    // W
    uint64_t elf_strtab_data;                   // W

    uint64_t initrd_base;                       // W
    uint64_t initrd_size;                       // W

    uint64_t rsdp;                              // W

    char cmdline[YB_CMDLINE_SIZE];              // W
};
#endif

// * W - fields written by this loader
// * R - fields with kernel-specified values
