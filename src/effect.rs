use std::sync;
use std::collections;

static AFFECTED_CHRINS_TABLE: sync::OnceLock<sync::RwLock<collections::HashMap<usize, f32>>> = sync::OnceLock::new();

pub fn get_affected_chrinses() -> sync::RwLockReadGuard<'static, collections::HashMap<usize, f32>> {
    AFFECTED_CHRINS_TABLE.get_or_init(|| sync::RwLock::new(collections::HashMap::default()))
        .read()
        .unwrap()
}

pub fn add_affected_chrins(chrins: usize, mult: f32) {
    AFFECTED_CHRINS_TABLE.get_or_init(|| sync::RwLock::new(collections::HashMap::default()))
        .write()
        .unwrap()
        .insert(chrins, mult);
}

pub fn remove_affected_chrins(chrins: usize) {
    AFFECTED_CHRINS_TABLE.get_or_init(|| sync::RwLock::new(collections::HashMap::default()))
        .write()
        .unwrap()
        .remove(&chrins);
}
