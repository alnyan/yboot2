#![feature(asm,const_fn)]
#![no_main]
#![no_std]

extern crate efi;
extern crate core_rt;
extern crate char16_literal;
extern crate yboot2_proto;
pub (crate) use char16_literal::cstr16;

use efi::{
    CStr16,
    Status,
    ConfigurationTableEntry,
    ImageHandle,
    SystemTable,
    system_table,
    image_handle,
};
use yboot2_proto::LoadProtocol;

#[macro_use]
mod println;
mod initrd;
mod proto;
mod video;
mod elf;

fn main() -> efi::Result<()> {
    let mut desc_array = [0u8; 16384];
    let mut mmap = efi::MemoryMap::new(&mut desc_array);
    let bs = &system_table().boot_services;

    println!("Getting memory map");
    bs.get_memory_map(&mut mmap)?;

    println!("Getting RSDP");
    let rsdp = system_table()
        .config_iter()
        .find(|x| matches!(x, ConfigurationTableEntry::Acpi10Table(_)))
        .map(|x| match x {
            ConfigurationTableEntry::Acpi10Table(ptr)   => ptr,
            _                                           => panic!()
        });

    let mut root = image_handle().get_boot_path()?.open_partition()?;

    // Load kernel
    let mut obj = elf::Object::open(&mut root, CStr16::from_literal(cstr16!(r"\kernel.elf")))?;
    let entry = obj.load(&mmap)?;
    let data = obj.locate_protocol_data::<proto::V1>()?;

    // Load initrd
    let (initrd_base, initrd_size) = initrd::load_somewhere(&mut root,
        CStr16::from_literal(cstr16!(r"\initrd.img")),
        &mmap,
        &obj)?;

    // Set video mode
    data.set_initrd(initrd_base, initrd_size);
    data.set_acpi_rsdp(rsdp.unwrap_or(core::ptr::null_mut()) as usize);
    data.set_loader_magic();

    // Get the new memory map and terminate boot services
    bs.get_memory_map(&mut mmap)?;
    data.set_efi_mmap(&mmap);
    video::set_mode(bs, data)?;

    bs.exit_boot_services(mmap.key)?;

    unsafe {
        let entry_fn: unsafe fn () -> ! = core::mem::transmute(entry);
        entry_fn();
    }
}

#[no_mangle]
extern "C" fn efi_main(ih: *mut ImageHandle, st: *mut SystemTable) -> u64 {
    efi::init(ih, st);
    let res = &main();
    println!("result -> {:?}", res);

    efi::Termination::to_efi(res)
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(panic: &PanicInfo<'_>) -> ! {
    // TODO: check if BS are available
    println!("Panic: {}!", panic);
    system_table().boot_services.exit(Status::Err);
}
