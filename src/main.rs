#![no_std]
#![no_main]
use core::panic::PanicInfo;

// 这个函数在panic时被调用
// 函数参数包含了panic发生的文件名、代码行数和可选的错误信息
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
/*
大多数语言都拥有一个运行时系统（runtime system）
它通常为垃圾回收（garbage collection）或绿色线程（software threads，或green threads）服务，如Java的GC或Go语言的协程（goroutine）
这个运行时系统需要在main函数前启动，因为它需要让程序初始化。

一个典型的使用标准库的Rust程序，它的运行将从名为crt0的运行时库开始。
crt0意为C runtime zero，它能建立一个适合运行C语言程序的环境，这包含了栈的创建和可执行程序参数的传入。
这之后，这个运行时库会调用Rust的运行时入口点，这个入口点被称作start语言项（"start" language item）。
Rust只拥有一个极小的运行时，它只拥有较少的功能，如爆栈检测和打印堆栈轨迹（stack trace）。
这之后，运行时将会调用main函数。
crt0 -> start -> main


我们的独立式可执行程序并不能访问Rust运行时或crt0库，所以我们需要定义自己的入口点。
实现一个start语言项并不能解决问题，因为这之后程序依然要求crt0库。
所以，我们要做的是，直接重写整个crt0库和它定义的入口点。
*/


// 禁用名称重整（name mangling）
// 这确保Rust编译器输出一个名为_start的函数
// 将函数标记为extern "C"，告诉编译器这个函数应当使用C语言的调用约定，而不是Rust语言的调用约定。
// 函数名为_start，是因为大多数系统默认使用这个名字作为入口点名称。
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> !{
    let vga_buffer = 0xb8000 as *mut u8;
    
    for(i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            // 0xb 表示淡青色
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

