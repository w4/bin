extern crate gpw;
extern crate linked_hash_map;

use linked_hash_map::LinkedHashMap;

use std::sync::RwLock;
use std::env;
use std::cell::RefCell;

lazy_static! {
    static ref ENTRIES: RwLock<LinkedHashMap<String, String>> = RwLock::new(LinkedHashMap::new());
}

/// Ensures `ENTRIES` is less than the size of `BIN_BUFFER_SIZE`. If it isn't then
/// `ENTRIES.len() - BIN_BUFFER_SIZE` elements will be popped off the front of the map.
///
/// During the purge, `ENTRIES` is locked and the current thread will block.
fn purge_old() {
    let entries_len = ENTRIES.read().unwrap().len();
    let buffer_size = env::var("BIN_BUFFER_SIZE").map(|f| f.parse::<usize>().unwrap()).unwrap_or(1000usize);

    if entries_len > buffer_size {
        let to_remove = entries_len - buffer_size;

        let mut entries = ENTRIES.write().unwrap();

        for _ in 0..to_remove {
            entries.pop_front();
        }
    }
}

/// Generates a randomly generated id, stores the given paste under that id and then returns the id.
///
/// Uses gpw to generate a (most likely) pronounceable URL.
pub fn store_paste(content: String) -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));
    let id = KEYGEN.with(|k| k.borrow_mut().next().unwrap());

    purge_old();
    ENTRIES.write().unwrap().insert(id.clone(), content);

    id
}

/// Get a paste by id.
///
/// Returns `None` if the paste doesn't exist.
pub fn get_paste(id: &str) -> Option<String> {
    ENTRIES.read().unwrap().get(id).cloned()
}