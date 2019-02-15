extern crate gpw;
extern crate linked_hash_map;

use linked_hash_map::LinkedHashMap;

use std::sync::RwLock;
use std::env;
use std::cell::RefCell;

lazy_static! {
    static ref ENTRIES: RwLock<LinkedHashMap<String, String>> = RwLock::new(LinkedHashMap::new());
    static ref BUFFER_SIZE: usize = env::var("BIN_BUFFER_SIZE").map(|f| f.parse::<usize>().unwrap()).unwrap_or(1000usize);
}

/// Ensures `ENTRIES` is less than the size of `BIN_BUFFER_SIZE`. If it isn't then
/// `ENTRIES.len() - BIN_BUFFER_SIZE` elements will be popped off the front of the map.
///
/// During the purge, `ENTRIES` is locked and the current thread will block.
fn purge_old() {
    let entries_len = ENTRIES.read().unwrap().len();

    if entries_len > *BUFFER_SIZE {
        let to_remove = entries_len - *BUFFER_SIZE;

        let mut entries = ENTRIES.write().unwrap();

        for _ in 0..to_remove {
            entries.pop_front();
        }
    }
}

/// Generates a 'pronounceable' random ID using gpw
pub fn generate_id() -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));
    KEYGEN.with(|k| k.borrow_mut().next().unwrap())
}

/// Stores a paste under the given id
pub fn store_paste(id: String, content: String) {
    purge_old();
    ENTRIES.write().unwrap().insert(id, content);
}

/// Get a paste by id.
///
/// Returns `None` if the paste doesn't exist.
pub fn get_paste(id: &str) -> Option<String> {
    ENTRIES.read().unwrap().get(id).cloned()
}