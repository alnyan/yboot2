use core::convert::TryInto;
use yboot2_proto::{
    ProtoV1,
    LoadProtocol,
    PixelFormat,
    MemoryMapInfo,
    VideoInfo,
    VideoRequest
};

pub struct V1 {
    proto: ProtoV1
}

impl LoadProtocol for V1 {
    const KERNEL_MAGIC: [u8; 8] = [
        0x07, 0xB0, 0x07, 0xB0, 0xA9, 0x97, 0xA1, 0x00
    ];

    fn set_loader_magic(&mut self) {
        const LOADER_MAGIC: [u8; 8] = [
            0x1A, 0x79, 0x9A, 0x0B, 0x70, 0x0B, 0x70, 0x00
        ];
        self.proto.hdr.loader_magic = LOADER_MAGIC;
    }

    fn set_mmap(&mut self, map: &MemoryMapInfo) {
        let ptr = map.address as *const u8;
        if ptr >= 0x100000000 as *const _ {
            panic!("Memory map pointer crosses 4GiB");
        }
        if map.size > self.proto.memory_map_size {
            panic!("Can't fit memory map");
        }

        unsafe {
            extern "C" {
                fn memcpy(dst: *mut u8, src: *const u8, count: usize) -> *mut u8;
            }
            memcpy(self.proto.memory_map_data as *mut _, ptr, map.size as usize);
        }

        self.proto.memory_map_size = map.size;
        self.proto.memory_map_entsize = map.entsize;
    }

    fn set_initrd(&mut self, base: usize, size: usize) {
        if self.proto.initrd_base + self.proto.initrd_size >= 0x100000000 {
            panic!("Initrd crosses 4GiB");
        }
        self.proto.initrd_base = base.try_into().unwrap();
        self.proto.initrd_size = size.try_into().unwrap();
    }

    fn set_acpi_rsdp(&mut self, rsdp: usize) {
        self.proto.rsdp = rsdp.try_into().unwrap();
    }

    fn get_video_request(&self) -> VideoRequest {
        use core::convert::TryFrom;
        VideoRequest {
            width: self.proto.video_width,
            height: self.proto.video_height,
            format: PixelFormat::try_from(self.proto.video_format).unwrap()
        }
    }

    fn set_video_info(&mut self, info: &VideoInfo) {
        if info.framebuffer >= 0x100000000 {
            panic!("Video framebuffer address is above 4GiB");
        }

        self.proto.video_width = info.width;
        self.proto.video_height = info.height;
        self.proto.video_format = info.format as u32;
        self.proto.video_framebuffer = info.framebuffer as u64;
        self.proto.video_pitch = info.pitch as u64;
    }
}

impl V1 {
    pub fn set_efi_mmap(&mut self, mmap: &efi::MemoryMap) {
        self.set_mmap(&MemoryMapInfo {
            address:    mmap.storage_ref.as_ptr() as u64,
            entsize:    mmap.descriptor_size.try_into().unwrap(),
            size:       mmap.size.try_into().unwrap()
        });
    }
}
