use core::convert::TryInto;
use efi::{MemoryMap, gop::PixelFormat};

const CMDLINE_SIZE: usize = 256;

pub trait LoadProtocol: Sized {
    const KERNEL_MAGIC: [u8; 8];

    fn set_loader_magic(&mut self);
    fn set_mmap(&mut self, map: &MemoryMap);
    fn set_initrd(&mut self, base: usize, size: usize);
    fn set_acpi_rsdp(&mut self, rsdp: usize);
    fn set_video_info(&mut self, info: &VideoInfo);
    fn get_video_request(&self) -> VideoRequest;
}

pub struct VideoRequest {
    pub width:      u32,
    pub height:     u32,
    pub format:     PixelFormat
}

pub struct VideoInfo {
    pub width:          u32,
    pub height:         u32,
    pub format:         PixelFormat,
    pub framebuffer:    usize,
    pub pitch:          usize
}

#[repr(C)]
pub struct Header {
    kernel_magic: [u8; 8],
    loader_magic: [u8; 8]
}

#[repr(C)]
pub struct V1 {
    hdr:                Header,

    memory_map_data:    u64,
    memory_map_size:    u32,
    memory_map_entsize: u32,

    video_width:        u32,
    video_height:       u32,
    video_format:       u32,
    _pad0:              u32,
    video_framebuffer:  u64,
    video_pitch:        u64,

    elf_symtab_hdr:     u64,
    elf_symtab_data:    u64,
    elf_strtab_hdr:     u64,
    elf_strtab_data:    u64,

    initrd_base:        u64,
    initrd_size:        u64,

    rsdp:               u64,

    cmdline:            [u8; CMDLINE_SIZE]
}

impl LoadProtocol for V1 {
    const KERNEL_MAGIC: [u8; 8] = [
        0x07, 0xB0, 0x07, 0xB0, 0xA9, 0x97, 0xA1, 0x00
    ];

    fn set_loader_magic(&mut self) {
        const LOADER_MAGIC: [u8; 8] = [
            0x1A, 0x79, 0x9A, 0x0B, 0x70, 0x0B, 0x70, 0x00
        ];
        self.hdr.loader_magic = LOADER_MAGIC;
    }

    fn set_mmap(&mut self, map: &MemoryMap) {
        self.memory_map_data = (map.storage_ref.as_ptr() as usize).try_into().unwrap();
        self.memory_map_size = map.size.try_into().unwrap();
        self.memory_map_entsize = map.descriptor_size.try_into().unwrap();
    }

    fn set_initrd(&mut self, base: usize, size: usize) {
        self.initrd_base = base.try_into().unwrap();
        self.initrd_size = size.try_into().unwrap();
    }

    fn set_acpi_rsdp(&mut self, rsdp: usize) {
        self.rsdp = rsdp.try_into().unwrap();
    }

    fn get_video_request(&self) -> VideoRequest {
        use core::convert::TryFrom;
        VideoRequest {
            width: self.video_width,
            height: self.video_height,
            format: PixelFormat::try_from(self.video_format).unwrap()
        }
    }

    fn set_video_info(&mut self, info: &VideoInfo) {
        self.video_width = info.width;
        self.video_height = info.height;
        self.video_format = info.format as u32;
        self.video_framebuffer = info.framebuffer as u64;
        self.video_pitch = info.pitch as u64;
    }
}
