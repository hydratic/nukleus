use x86_64::structures::idt::ExceptionStackFrame;
use memory::MemoryController;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

pub fn init(memory_controller: &mut MemoryController) {
    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");

    IDT.load();
}

// our new double fault handler
extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

// Sample Usage
// in src/lib.rs

// #[no_mangle]
// pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ...
    // initialize our IDT
    // interrupts::init();

    // fn stack_overflow() {
        // stack_overflow(); // for each recursion, the return address is pushed
    // }

    // trigger a stack overflow
    // stack_overflow();

    // println!("It did not crash!");
    // loop {}
// }
