pub struct Storage {
    store: web_sys::Storage,
}

impl Storage {
    pub fn new() -> Result<Storage, &'static str> {
        let window = web_sys::window();
        let window = match window {
            Some(value) => value,
            None => return Err("no-window"),
        };

        let store = window.local_storage();
        let store = match store {
            Ok(value) => value,
            Err(_) => return Err("no-local-storage"),
        };
        let store = match store {
            Some(value) => value,
            None => return Err("no-local-storage"),
        };
        Ok(Storage { store })
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let item = self.store.get_item(key);
        match item {
            Ok(value) => value,
            Err(_) => return None,
        }
    }

    pub fn set(&self, key: &str, value: &String) {
        let action = self.store.set_item(key, value);
        // The error is not useful to know, if it fails it fails.
        match action {
            _ => (),
        }
    }
}
