// main.rs
#![no_std]  // 不链接rust标准库
#![no_main] // 禁用所有rust层级的入口点

use core::panic::PanicInfo;

mod vga_buffer;

/// 这个函数将在panic时候被调用
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 因为编译器会寻找一个名为 ‘_start’ 的函数，所有这个函数就是入口点
    // 默认命名为 ‘_start’

    print!("hello world {}", 123);

    loop {}
}



