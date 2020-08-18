use std::thread;
use std::sync::mpsc;

use ds_13::sync::list::{foldl, List};
use ds_13::list_synced;

#[test]
fn test_clone_in_thread() {
    let list = list_synced!(3, 4);

    let list_clone = list.clone();
    let handle = thread::spawn(move || {

        assert_eq!(foldl(|a, b| a + b, 0, &list_clone), 7);
    });
    assert_eq!(foldl(|a, b| a + b, 0, &list), 7);
    handle.join().unwrap();
}

#[test]
fn send_list() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let list = list_synced!(1, 2, 3, 4);

        tx.send(list).unwrap();
    });

    let list = rx.recv().unwrap();

    assert_eq!(list, list_synced!(1, 2, 3, 4));
}
