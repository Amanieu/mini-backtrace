use mini_backtrace::Backtrace;

fn main() {
    let bt = Backtrace::<16>::capture();
    println!("Backtrace:");
    for frame in bt.frames {
        println!("  {:#x}", adjust_for_pic(frame));
    }
    if bt.frames_omitted {
        println!(" ... <frames omitted>");
    }
}

// For position-independent code, convert the addresses to be relative to the
// executable base address.
//
// This should *only* be done for position-independent binaries, not statically
// linked ones.
fn adjust_for_pic(ip: usize) -> usize {
    extern "C" {
        // Symbol defined by the linker
        static __executable_start: [u8; 0];
    }
    let base = unsafe { __executable_start.as_ptr() as usize };
    ip - base
}
