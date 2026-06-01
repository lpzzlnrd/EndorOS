use core::panic::PanicInfo;

/// Bare-metal panic handler. In a real kernel this would print to a serial port
/// and halt the CPU. Here we loop forever after signalling the fault.
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    // In a real implementation we would write to a memory-mapped UART.
    // For the conceptual kernel we simply spin.
    let _ = info;
    loop {
        // Hint to the CPU that we are in a spin-wait to reduce power.
        core::hint::spin_loop();
    }
}
