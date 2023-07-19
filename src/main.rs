#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{panic::PanicInfo, arch::asm};
use os_rust::println;

static HELLO: &[u8] = b"Hello world!err";

#[no_mangle]
pub extern "C" fn _start() -> !{
    println!("Hello World{}", "!");

    os_rust::init(); 
    
    // divide_by_zero();
    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); 

    // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u8) = 42;
    // };
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }
    // stack_overflow();
    // as before
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    os_rust::hlt_loop();
}
// fn divide_by_zero() {
//     unsafe {
//         asm!("mov dx, 0; div dx")
//     }
// }


#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_rust::test_panic_handler(info)
}
// #[cfg(test)]
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     serial_println!("[failed]\n");
//     serial_println!("Error: {}\n", info);
//     exit_qemu(QemuExitCode::Failed);
//     loop {}
// }

// #[cfg(test)]
// fn test_runner(tests: &[&dyn Testable]) {
//     serial_println!("Running {} tests", tests.len());
//     for test in tests {
//         test.run();
//     }
//     exit_qemu(QemuExitCode::Success);
// }

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}



