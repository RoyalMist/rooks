use extism::convert::Json;
use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct In {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) happy: bool,
    pub(crate) school: String,
}

pub fn call(input: In) -> String {
    let wasm_path = std::env::var("WASM_PATH").unwrap();
    let path = Wasm::File {
        path: wasm_path.parse().unwrap(),
        meta: Default::default(),
    };

    let manifest = Manifest::new([path])
        .with_config_key("redact", "XXXX")
        .with_allowed_host("api.chucknorris.io");

    let mut plugin = Plugin::new(&manifest, [], false).unwrap();
    plugin
        .call::<Json<In>, String>("call", Json::from(input))
        .unwrap()
}
