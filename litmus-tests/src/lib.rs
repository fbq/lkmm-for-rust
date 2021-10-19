#[cfg(test)]
mod tests {
    #[test]
    fn test_corr_poonceonce_once() {
        use std::sync::atomic::{AtomicI32, Ordering};
        use std::sync::Arc;
        use std::thread;

        let x = Arc::new(AtomicI32::new(0));
        let x_in_p0 = x.clone();
        let x_in_p1 = x.clone();

        let p0 = thread::spawn(move || {
            x_in_p0.store(1, Ordering::Relaxed);
        });

        let p1 = thread::spawn(move || -> (i32, i32) {
            let r0 = x_in_p1.load(Ordering::Relaxed);
            let r1 = x_in_p1.load(Ordering::Relaxed);

            (r0, r1)
        });

        p0.join().unwrap();
        let (r0, r1) = p1.join().unwrap();

        assert!(!(r0 == 1 && r1 ==0));
    }

}
