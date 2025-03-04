use extism::convert::Json;
use extism::{Error, Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct In {
    name: String,
    age: i32,
    happy: bool,
    school: String,
}

impl In {
    pub fn new(name: String, age: i32, happy: bool, school: String) -> Self {
        In {
            name,
            age,
            happy,
            school,
        }
    }
}

pub fn call(input: In) -> Result<String, Error> {
    let wasm_path = std::env::var("WASM_PATH")?;
    let path = Wasm::File {
        path: wasm_path.parse()?,
        meta: Default::default(),
    };

    let manifest = Manifest::new([path])
        .with_config_key("redact", "Y")
        .with_allowed_host("api.chucknorris.io");

    let mut plugin = Plugin::new(&manifest, [], false)?;
    plugin.call::<Json<In>, String>("call", Json::from(input))
}
