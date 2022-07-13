use std::collections::HashMap;

pub struct Layers(HashMap<LayerName, LayerData>);

impl Layers {
    pub fn new() -> Self {
        Layers(HashMap::from([
            (LayerName::Ground, LayerData { z_height: 0. }),
            (LayerName::Tower, LayerData { z_height: 100. }),
        ]))
    }
    pub fn get(&self, layer_name: LayerName) -> &LayerData {
        self.0.get(&layer_name).unwrap()
    }
}

#[derive(Eq, PartialEq, Hash)]
pub enum LayerName {
    Ground,
    Tower,
}

pub struct LayerData {
    pub z_height: f32,
}
