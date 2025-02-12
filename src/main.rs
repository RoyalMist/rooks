use extism::convert::Json;
use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct In {
    name: String,
    age: i32,
    happy: bool,
    school: String,
}

fn main() {
    let input = In {
        name: "Thibault Fouache".to_string(),
        age: 30,
        happy: true,
        school: "High One".to_string(),
    };

    let wasm_path = std::env::var("WASM_PATH").unwrap();
    let path = Wasm::File {
        path: wasm_path.parse().unwrap(),
        meta: Default::default(),
    };

    let manifest = Manifest::new([path])
        .with_config_key("redact", "Francesco")
        .with_allowed_host("api.chucknorris.io");

    let mut plugin = Plugin::new(&manifest, [], false).unwrap();
    let res = plugin
        .call::<Json<In>, String>("call", Json::from(input))
        .unwrap();

    println!("{:?}", res);
}
