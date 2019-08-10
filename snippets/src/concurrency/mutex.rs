use std::sync::{Arc, Mutex};
use std::thread;


#[test]
fn mutex_test() {

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 1..10 + 1 {

        let counter_ref = counter.clone();

        handles.push(thread::spawn(move || {
            let mut c = counter_ref.lock().unwrap();
            *c += 1;
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(*counter.lock().unwrap(), 10);
}