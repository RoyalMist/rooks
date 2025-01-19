use extism::convert::Json;
use extism::{Manifest, Plugin, Wasm};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    name: String,
    age: i32,
    happy: bool,
}

fn main() {
    let input = Data {
        name: "John".to_string(),
        age: 30,
        happy: true,
    };

    let wasm_path = std::env::var("WASM_PATH").unwrap();
    let path = Wasm::File {
        path: wasm_path.parse().unwrap(),
        meta: Default::default(),
    };

    let manifest = Manifest::new([path]).with_config_key("redact", "xxxxx");
    let mut plugin = Plugin::new(&manifest, [], false).unwrap();
    let res = plugin
        .call::<Json<Data>, Json<Data>>("redact", Json::from(input))
        .unwrap();

    println!("{:?}", res);
}
