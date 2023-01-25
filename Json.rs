use serde_json::{Result, Value, Map};

type JsonMap = Map<String, serde_json::Value>;

fn untyped_parse(data: &str) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data)?;

    // Access parts of the data by indexing with square brackets.
    println!("Please call {} at the number {}", v["name"], v["phones"][0]);

    Ok(())
}

fn untyped_print(data: &str) -> Result<()> {
    let map: JsonMap = serde_json::from_str(data)?;
    
    untyped_print_rec(&map)
}

fn untyped_print_rec(map: &JsonMap) -> Result<()> {
    for (key, value) in map {
        match value {
            Value::Null => println!("<>"),
            Value::Bool(v) => println!("{key} = {v}"),
            Value::Number(v) => println!("{key} = {v}"),
            Value::String(v) => println!("{key} = \"{v}\""),
            Value::Object(v) => {
                untyped_print_rec(v)?;
            },
            Value::Array(v) => {
                for item in v {
                    if item.is_object() || item.is_array() {
                        let s = serde_json::to_string(&item)?;
                        untyped_print(&s)?;
                    } else {
                        println!("{key} = {item}");
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn main() {
    // Some JSON input data as a &str. Maybe this comes from the user.

    let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678",
                {
                    "backup": "+44 xxxxxxx"
                }
            ]
        }"#;
    untyped_parse(data).expect("Malformatted json");
    untyped_print(data).expect("Malformatted json");
}
