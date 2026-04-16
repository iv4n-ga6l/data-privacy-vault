use std::collections::HashMap;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref STORAGE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

pub fn store_tokenized_data(token: String, original_value: String) {
    let mut storage = STORAGE.lock().unwrap();
    storage.insert(token, original_value);
}

pub fn retrieve_original_data(token: &str) -> Option<String> {
    let storage = STORAGE.lock().unwrap();
    storage.get(token).cloned()
}
