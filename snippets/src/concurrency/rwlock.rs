use std::sync::{RwLock, Arc};
use std::thread;


#[derive(Debug)]
struct Config {
    nightly_enabled: bool,
    logging_enabled: bool
}

#[test]
fn rwlock_test() {

    let initial_config = Config {
        nightly_enabled : true,
        logging_enabled : false,
    };

    let arc_rwlock = Arc::new(RwLock::new(initial_config));

    for i in 1..5 {

        //Rwlock can have several readers, following rust borrowing rules -> 1 mut and several non-mut access

        let ref_lock = arc_rwlock.clone();
        thread::spawn(move ||  {
            let data = ref_lock.read().unwrap();
            println!("{:?}", data);
        });
    }

    let write_lock = arc_rwlock.clone();
    let handle = thread::spawn(move || {
        let mut data = write_lock.write().unwrap();
        data.logging_enabled = true;
    });

    handle.join().unwrap();

    let read_lock = arc_rwlock.read().unwrap();
    assert_eq!(read_lock.logging_enabled, true);
    assert_eq!(read_lock.nightly_enabled, true);


}