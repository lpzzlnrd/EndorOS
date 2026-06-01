#![no_std]
#![no_main]
// Allow the use of alloc types (Vec, String, Box, …) in the kernel.
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod boot;
mod panic;

use core::fmt;
use core::fmt::Write as FmtWrite;

// ---------------------------------------------------------------------------
// Minimal serial-port writer (conceptual — no real UART I/O in no_std demo)
// ---------------------------------------------------------------------------

/// A zero-size writer that conceptually represents the first serial port
/// (COM1, I/O port 0x3F8). In a real kernel we would write bytes directly to
/// the UART data register using an `outb` instruction.
struct SerialWriter;

impl fmt::Write for SerialWriter {
    fn write_str(&mut self, _s: &str) -> fmt::Result {
        // In a real implementation:
        //   for byte in s.bytes() { unsafe { outb(0x3F8, byte); } }
        Ok(())
    }
}

/// Helper to write a formatted string to the conceptual serial port.
macro_rules! serial_println {
    ($($arg:tt)*) => {{
        let mut w = SerialWriter;
        let _ = writeln!(w, $($arg)*);
    }};
}

// ---------------------------------------------------------------------------
// Kernel entry point
// ---------------------------------------------------------------------------

/// Kernel entry point called by the bootloader after the CPU is in 64-bit
/// protected mode and the stack pointer has been set up.
///
/// The `#[no_mangle]` attribute ensures the linker can find this symbol, and
/// the calling convention matches what the bootloader expects (System V AMD64
/// ABI on x86_64).
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Print the kernel boot banner over the serial port.
    serial_println!("┌─────────────────────────────────────────────┐");
    serial_println!("│   EndorOS Kernel  v0.1.0  — starting…       │");
    serial_println!("│   Architecture  : x86_64 (bare-metal)       │");
    serial_println!("│   Allocator     : BumpAllocator (2 MiB)      │");
    serial_println!("│   Design        : Hexagonal / Ports&Adapters │");
    serial_println!("└─────────────────────────────────────────────┘");

    // Run the full boot sequence.
    //
    // SAFETY: `_start` is called exactly once by the bootloader; no kernel
    // subsystem has been touched before this point.
    unsafe { boot::init(); }

    serial_println!("[kernel] Boot complete. Handing off to init process.");

    // The real kernel would never return here — it would either transfer
    // control to the scheduler (which enables interrupts and context-switches
    // to the first process) or park the BSP in a halt loop while APs run.
    loop {
        core::hint::spin_loop();
    }
}

// ---------------------------------------------------------------------------
// Allocation error handler (required when alloc_error_handler is enabled)
// ---------------------------------------------------------------------------

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    serial_println!(
        "[kernel] FATAL: allocation failed — size={}, align={}",
        layout.size(),
        layout.align()
    );
    loop {
        core::hint::spin_loop();
    }
}
