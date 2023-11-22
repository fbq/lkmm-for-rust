use core::ptr::write_volatile;
use core::cell::UnsafeCell;
use core::arch::asm;

// Simulates shared memory used for C <-> Rust communication.
pub struct SharedMem(UnsafeCell<usize>);

impl SharedMem {
    /// C version: READ_ONCE()
    pub fn read_once(&self) -> usize {
        unsafe { self.0.get().read_volatile() }
    }

    /// C version: WRITE_ONCE()
    pub fn write_once(&self, val: usize) {
        unsafe { write_volatile(self.0.get(), val); }
    }

    /// Full barrier.
    ///
    /// C version: smp_mb()
    #[cfg(target_arch = "aarch64")]
    pub fn smp_mb() {
        // C version: asm volatile("dmbish": : : "memory");
        unsafe { asm!("dmb ish"); }
    }
    #[cfg(target_arch = "x86_64")]
    pub fn smp_mb() {
        // C version: asm volatile("dmbish": : : "memory");
        unsafe { asm!("mfence"); }
    }

    pub fn new(val: usize) -> Self {
        SharedMem(UnsafeCell::new(val))
    }
}


// According to LKMM, read_once and write_once are volatile atomic.
unsafe impl Sync for SharedMem {}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    #[test]
    fn test_corr_poonceonce_once() {
        // C litmus test:
        // tools/memory-model/litmus-tests/CoRR+poonceonce+Once.litmus

        let x_in_mem = SharedMem::new(0);

        thread::scope(|scope| {
            let x = &x_in_mem;

            let p0 = scope.spawn(move || {
                x.write_once(1);
            });

            let p1 = scope.spawn(move || -> (usize, usize) {
                let r0 = x.read_once();
                let r1 = x.read_once();

                (r0, r1)
            });

            p0.join().unwrap();
            let (r0, r1) = p1.join().unwrap();

            // Result: Never
            // exists (1:r0=1 /\ 1:r1=0)
            //
            // expect r0 == 1 && r1 == 0 never happens
            assert!(!(r0 == 1 && r1 == 0));
        });
    }

    #[test]
    fn lb_fencembonceonce_ctrlonceonce() {
        // C litmus test:
        // tools/memory-model/litmus-tests/LB+fencembonceonce+ctrlonceonce.litmus

        let x_in_mem = SharedMem::new(0);
        let y_in_mem = SharedMem::new(0);

        thread::scope(|scope| {
            let x = &x_in_mem;
            let y = &y_in_mem;

            let p0 = scope.spawn(move || -> usize {
                let r0 = x.read_once();

                if r0 != 0 {
                    y.write_once(1);
                }

                r0
            });

            let p1 = scope.spawn(move || -> usize {
                let r0 = y.read_once();

                SharedMem::smp_mb();
                x.write_once(1);

                r0
            });

            let p0_r0 = p0.join().unwrap();
            let p1_r0 = p1.join().unwrap();

            // Result: Never
            // exists (0:r0=1 /\ 1:r0=1)
            //
            // expect p0_r0 == 1 && p1_r0 == 0 never happens
            assert!(!(p0_r0 == 1 && p1_r0 == 0));
        });
    }

}
