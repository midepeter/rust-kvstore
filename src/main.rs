use::std::fs::File;
use::std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    let val: &str = "Hello, world!";
   // let _: Result<String, std::io::Error>;
    let _ = create_store();
    add_value();
    let content = String::new();
    read_value(content);
    delete_table(val);
   // println!("pointVal {}", Store::new(val));
}

pub struct Store {
    pub key: String,
    pub value: String
}

impl Store {
    fn new(val: &str) -> Store {
        return Store { 
            key:  String::from(val), 
            value: String::from("The value is value")
        }
    }
}   

fn create_store() -> std::io::Result<()> {
    let mut new_file = File::create("cache.db")?;    
    new_file.write_all(b"Hello world")?;
    Ok(())
}

fn add_value() {}

fn read_value(read: String) {
    println!("The content is {}", read);
}

fn delete_table(name: &str) -> bool {
    println!("The name in the func {}", name);
    return false
}
