use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type PrefabMap<P> = HashMap<String, P>;

pub struct PrefabLib<P: DeserializeOwned> {
    map: PrefabMap<P>,
}

impl<P: DeserializeOwned> PrefabLib<P> {
    pub fn new(filepath: &str) -> Self {
        let contents = fs::read_to_string(Path::new(&filepath)).unwrap();
        let map: PrefabMap<P> = ron::from_str(&contents).unwrap();

        PrefabLib { map }
    }

    pub fn get(&self, id: &str) -> Option<&P> {
        self.map.get(id)
    }
}
