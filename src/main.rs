use plugin::In;
mod plugin;

fn main() {
    let input = In::new("John".to_string(), 25, true, "MIT".to_string());
    let res = plugin::call(input);
    match res {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err.root_cause()),
    }
}
