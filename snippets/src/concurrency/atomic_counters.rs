use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn atomic_bool_test() {

    let atomic_cancel = Arc::new(AtomicBool::new(false));

    for i in 1..8 {

        let atomic_ref = atomic_cancel.clone();

        thread::spawn(move || {
            long_running_task(i, &atomic_ref);
        });
    }

    thread::sleep(Duration::from_millis(1000));
    atomic_cancel.swap(true, Ordering::SeqCst);
}

fn long_running_task(id: usize, cancellation : &Arc<AtomicBool>) {

    loop {

        if cancellation.load(Ordering::SeqCst) {
            return;
        }

        println!("Executing job in thread {}", id);
    }
}