unsafe fn load_cr3(value: usize) {
    llvm_asm!("mov $0, %cr3"::"r"(value):"memory");
}

const PAGE_HUGE: u64 = 1 << 7;
const PAGE_USER: u64 = 1 << 2;
const PAGE_WRITE: u64 = 1 << 1;
const PAGE_PRESENT: u64 = 1 << 0;

// 1 PML4, 1 PDPT, 4 PD
// TODO: maybe setup this in compile-time?
static mut TABLES: [u64; 512 * 6] = [0; 512 * 6];

unsafe fn setup_tables() {
    for i in 0 .. 512 * 4 {
        // pd[i] = 2MiB block
        TABLES[i + 512 * 2] = (i << 21) as u64 | PAGE_HUGE | PAGE_WRITE | PAGE_PRESENT | PAGE_USER;
    }

    for i in 0 .. 4 {
        // pdpt[i] = pd_i
        TABLES[512 + i] = (&TABLES[i * 512 + 512 * 2]) as *const _ as u64 | PAGE_WRITE | PAGE_PRESENT | PAGE_USER;
    }

    // pml4[0] = PRESENT | pdpt
    TABLES[0] = (&TABLES[512]) as *const _ as u64 | PAGE_WRITE | PAGE_PRESENT | PAGE_USER;
    // pml4[510] = PRESENT | pdpt
    TABLES[510] = (&TABLES[512]) as *const _ as u64 | PAGE_WRITE | PAGE_PRESENT | PAGE_USER;
}

pub fn setup_upper() {
    unsafe {
        setup_tables();
        load_cr3(TABLES.as_ptr() as usize);
    }
}
