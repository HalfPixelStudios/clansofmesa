use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

type ResourceMap<P: DeserializeOwned> = HashMap<String, P>;

pub struct PrefabResource<P: DeserializeOwned> {
    map: ResourceMap<P>,
}

impl<P: DeserializeOwned> PrefabResource<P> {
    pub fn new(filepath: &str) -> Self {
        let contents = fs::read_to_string(Path::new(&filepath)).unwrap();
        let map: ResourceMap<P> = ron::from_str(&contents).unwrap();

        PrefabResource { map }
    }

    pub fn get(&self, id: &str) -> Option<&P> {
        self.map.get(id)
    }
}
