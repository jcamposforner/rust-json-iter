extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Error;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Foo {
    data: String,
}

fn typed_example() -> Result<Vec<Foo>, Error> {
    let data = r#"[
        {
            "data": "value1"
        },
        {
            "data": "value2"
        },
        {
            "data": "value3"
        }
    ]"#;

    let array: Vec<Foo> = serde_json::from_str(data)?;
    let b = array
        .iter()
        .filter(|x| x.data == "value2")
        .cloned()
        .collect();

    Ok(b)
}

fn main() {
    let t = typed_example().unwrap();

    println!("{:?}", t);
}
