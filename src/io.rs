extern crate gpw;
extern crate linked_hash_map;
extern crate owning_ref;
extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use linked_hash_map::LinkedHashMap;

use owning_ref::OwningRef;

use std::cell::RefCell;
use std::env;

use tokio::sync::{RwLock, RwLockReadGuard};

type RwLockReadGuardRef<'a, T, U = T> = OwningRef<Box<RwLockReadGuard<'a, T>>, U>;

lazy_static! {
    static ref ENTRIES: RwLock<LinkedHashMap<String, String>> = RwLock::new(LinkedHashMap::new());
    static ref BUFFER_SIZE: usize = env::var("BIN_BUFFER_SIZE")
        .map(|f| f
            .parse::<usize>()
            .expect("Failed to parse value of BIN_BUFFER_SIZE"))
        .unwrap_or(1000usize);
}

/// Ensures `ENTRIES` is less than the size of `BIN_BUFFER_SIZE`. If it isn't then
/// `ENTRIES.len() - BIN_BUFFER_SIZE` elements will be popped off the front of the map.
///
/// During the purge, `ENTRIES` is locked and the current thread will block.
async fn purge_old() {
    let entries_len = ENTRIES.read().await.len();

    if entries_len > *BUFFER_SIZE {
        let to_remove = entries_len - *BUFFER_SIZE;

        let mut entries = ENTRIES.write().await;

        for _ in 0..to_remove {
            entries.pop_front();
        }
    }
}

/// Generates a 'pronounceable' random ID using gpw
pub fn generate_id() -> String {
    thread_local!(static KEYGEN: RefCell<gpw::PasswordGenerator> = RefCell::new(gpw::PasswordGenerator::default()));

    KEYGEN.with(|k| k.borrow_mut().next()).unwrap_or_else(|| {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .collect::<String>()
    })
}

/// Stores a paste under the given id
pub async fn store_paste(id: String, content: String) {
    purge_old().await;

    ENTRIES
        .write()
        .await
        .insert(id, content);
}

/// Get a paste by id.
///
/// Returns `None` if the paste doesn't exist.
pub async fn get_paste(id: &str) -> Option<RwLockReadGuardRef<'_, LinkedHashMap<String, String>, String>> {
    // need to box the guard until owning_ref understands Pin is a stable address
    let or = RwLockReadGuardRef::new(Box::new(ENTRIES.read().await));

    if or.contains_key(id) {
        Some(or.map(|x| x.get(id).unwrap()))
    } else {
        None
    }
}
