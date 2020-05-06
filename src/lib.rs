extern crate libc;

use libc::{_exit, c_int, signal, SIGUSR1, SIGSYS};
use std::io;

static mut SIGNAL_HANDLER_CALLED: bool = false;

extern "C" fn dummy_handler(signum: c_int) {
    if signum != SIGUSR1 {
        eprintln!("Got unexpected signal {}", signum);
        unsafe { _exit(1i32) };
    }

    unsafe { SIGNAL_HANDLER_CALLED = true };
}

pub type SignalHandler = extern "C" fn(signum: c_int) -> ();

pub fn register_signal_handler(num: c_int, handler: SignalHandler) -> io::Result<()> {
    match unsafe { signal(num, handler as *const () as usize) } {
        0 => Ok(()),
        _ => Err(io::Error::last_os_error()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::raise;

    #[test]
    fn it_works() {
        register_signal_handler(SIGSYS, dummy_handler);
        assert!(!unsafe { SIGNAL_HANDLER_CALLED });
        unsafe { raise(SIGSYS) };
        assert!(unsafe { SIGNAL_HANDLER_CALLED });
    }
}
