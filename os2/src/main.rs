
#![no_std]
#![no_main]

#![feature(panic_info_message)]


mod lang_items;
mod sbi;
#[macro_use]
mod console;

// fn main() {
//     //println!("Hello, world!");
// }
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    println!("Hello, world555!");
    panic!("Shutdown machine!");
}

//尝试从其他地方找到全局符号 sbss 和 ebss ，它们由链接脚本 linker.ld 给出，并分别指出需要被清零的 .bss 段的起始和终止地址
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}


