use rand::{thread_rng, Rng, distributions::Alphanumeric};

use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use owning_ref::OwningRef;
use tokio::sync::{RwLock, RwLockReadGuard};

use std::cell::RefCell;

type RwLockReadGuardRef<'a, T, U = T> = OwningRef<Box<RwLockReadGuard<'a, T>>, U>;

pub type PasteStore = RwLock<LinkedHashMap<String, String>>;

lazy_static! {
    static ref BUFFER_SIZE: usize = argh::from_env::<crate::BinArgs>().buffer_size;
}

/// Ensures `ENTRIES` is less than the size of `BIN_BUFFER_SIZE`. If it isn't then
/// `ENTRIES.len() - BIN_BUFFER_SIZE` elements will be popped off the front of the map.
///
/// During the purge, `ENTRIES` is locked and the current thread will block.
async fn purge_old(entries: &PasteStore) {
    let entries_len = entries.read().await.len();

    if entries_len > *BUFFER_SIZE {
        let to_remove = entries_len - *BUFFER_SIZE;

        let mut entries = entries.write().await;

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
pub async fn store_paste(entries: &PasteStore, id: String, content: String) {
    purge_old(&entries).await;

    entries.write()
        .await
        .insert(id, content);
}

/// Get a paste by id.
///
/// Returns `None` if the paste doesn't exist.
pub async fn get_paste<'a>(entries: &'a PasteStore, id: &str) -> Option<RwLockReadGuardRef<'a, LinkedHashMap<String, String>, String>> {
    // need to box the guard until owning_ref understands Pin is a stable address
    let or = RwLockReadGuardRef::new(Box::new(entries.read().await));

    if or.contains_key(id) {
        Some(or.map(|x| x.get(id).unwrap()))
    } else {
        None
    }
}
