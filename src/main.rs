mod plugin;

fn main() {
    let input = plugin::In {
        name: "Thibault Fouache".to_string(),
        age: 30,
        happy: true,
        school: "High One".to_string(),
    };

    let res = plugin::call(input);
    match res {
        Ok(res) => println!("{}", res),
        Err(err) => println!("Error: {}", err.root_cause()),
    }
}
