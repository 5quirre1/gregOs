#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::fmt::Write;
mod vga_buffer;
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut writer: vga_buffer::Writer = vga_buffer::Writer::new();
    write!(writer, "this took way too friggin' long..\ngregOs").unwrap();
    loop {}
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
