#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_rust::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::{panic::PanicInfo, arch::asm};
use alloc::{boxed::Box, vec::{Vec, self}, rc::Rc};
use bootloader::{BootInfo, entry_point};
use os_rust::{println, memory::{translate_addr, BootInfoFrameAllocator}, allocator, task::simple_executor::SimpleExecutor};
use x86_64::structures::paging::Page;
use os_rust::task::{Task};

static HELLO: &[u8] = b"Hello world!err";

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> !{
    use os_rust::memory;
    use x86_64::{structures::paging::Translate, VirtAddr};

    println!("Hello World{}", "!");
    os_rust::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)    
    };
    // memory::EmptyFrameAllocator

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let heap_value =Box::new(41);
    println!("heap value at: {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i)
    }
    println!("vec at {:p}", vec.as_slice());
    
    let reference_counted = Rc::new(alloc::vec![1,2,3]);
    let cloned_refence = reference_counted.clone();
    println!("current refence count is {}", Rc::strong_count(&cloned_refence));
    core::mem::drop(reference_counted);
    println!("reference count is now {}", Rc::strong_count(&cloned_refence));
    
    // waker
    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.run();
    
    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

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

    // let ptr = 0x204d4a as *mut u8;
    // unsafe { let x = *ptr; }
    // println!("read worked");
    // // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");
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

async fn asnyc_number() -> u32 {
    42
}

async fn example_task() {
    let number = asnyc_number().await;
    println!("async number: {:?}", number)
}


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



