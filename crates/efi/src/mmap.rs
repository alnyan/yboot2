#[repr(C)]
#[derive(Copy, Clone)]
pub struct MemoryDescriptor {
    pub _type:              u32,
    pub physical_start:     usize,
    pub virtual_start:      usize,
    pub number_of_pages:    u64,
    pub attribute:          u64
}

impl MemoryDescriptor {
    pub fn begin(&self) -> usize {
        return self.physical_start;
    }
    pub fn end(&self) -> usize {
        return self.physical_start + (self.number_of_pages as usize) * 0x1000;
    }
}

pub struct MemoryMapIterator<'a> {
    mmap:       &'a MemoryMap<'a>,
    pos:        usize,
    count:      usize,
}

impl<'a> Iterator for MemoryMapIterator<'a> {
    type Item = &'a MemoryDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.count {
            return None;
        }

        let item_ptr = &self.mmap.storage_ref[self.pos * self.mmap.descriptor_size];
        self.pos += 1;

        unsafe {
            return ((item_ptr as *const u8) as *const MemoryDescriptor).as_ref();
        }
    }
}

pub struct MemoryMap<'a> {
    pub storage_ref:        &'a mut [u8],
    pub key:                usize,
    pub descriptor_size:    usize,
    pub size:               usize,
}

impl<'a> MemoryMap<'a> {
    pub fn new(storage: &mut [u8]) -> MemoryMap {
        return MemoryMap {
            storage_ref: storage,
            descriptor_size: 0,
            size: 0,
            key: 0
        };
    }

    pub fn iter(&self) -> Option<MemoryMapIterator> {
        if self.size == 0 || self.descriptor_size == 0 {
            return None;
        }

        return Some(MemoryMapIterator {
            mmap: self,
            pos: 0,
            count: self.size / self.descriptor_size
        });
    }
}
