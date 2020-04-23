pub mod storage {
    use std::{collections, io};
    use std::sync::{Arc, RwLock};

    pub fn new() -> Memory {
        Memory { data: Arc::new(RwLock::new(collections::HashMap::new())) }
    }

    pub struct Memory {
        pub data: Arc<RwLock<collections::HashMap<String, String>>>
    }

    impl Memory {
        pub fn set(&mut self, key: &String, val: String) {
            // if a poisoned mutex is encountered just panic because the programme is already
            // broken.
            self.data.write().unwrap().insert(key.clone(), val);
        }

        pub fn get(&self, key: &String) -> Result<String, io::ErrorKind> {
            self.data.read().unwrap().get(key).cloned().ok_or(io::ErrorKind::NotFound)
        }
    }
}
