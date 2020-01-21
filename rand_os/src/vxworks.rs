use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use rand_core::{Error, ErrorKind};
use super::OsRngImpl;
use std::io;

#[derive(Clone, Debug)]
pub struct OsRng;

impl OsRngImpl for OsRng {
    fn new() -> Result<OsRng, Error> { Ok(OsRng) }

    fn fill_chunk(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        static RNG_INIT: AtomicBool = AtomicBool::new(false);
        while !RNG_INIT.load(Relaxed) {
            let ret = unsafe { libc::randSecure() };
            if ret < 0 {
                return Err(Error::with_cause(
                    ErrorKind::Unavailable,
                    "randSecure failed",
                    io::Error::last_os_error()));
            } else if ret > 0 {
                RNG_INIT.store(true, Relaxed);
                break;
            }
            unsafe { libc::usleep(10) };
        }
        let ret = unsafe {
            libc::randABytes(dest.as_mut_ptr() as *mut libc::c_uchar, dest.len() as libc::c_int)
        };
        if ret < 0 {
            Err(Error::with_cause(
                ErrorKind::Unavailable,
                "couldn't generate random bytes",
                io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    fn method_str(&self) -> &'static str { "randABytes" }
}

