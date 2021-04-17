use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("No key was not there");
    let value = arguments.next().unwrap();
    println!("The key is {} and the value is {}", &key, &value);
    let mut database = Database::new().expect("Failed to init Database");
    database.insert(key, value);
}

#[derive(Debug)]
struct Database {
    map: HashMap<String, String>,
    is_flushed: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key");
            let value = chunks.next().expect("No value");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map,
            is_flushed: false,
        })
    }

    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key.clone(), value.clone());
    }

    fn flush(&mut self) -> std::io::Result<String> {
        self.is_flushed = true;
        let contents = do_flush(&self).unwrap();
        Ok(contents)
    }
}

fn do_flush(database: &Database) -> Result<String, std::io::Error> {
    let mut contents = String::new();
    for (key, value) in database.map.iter() {
        let kvpair = format!("{}\t{}\n", key, value);
        contents.push_str(&kvpair)
    }
    Ok(contents)
}

impl Drop for Database {
    fn drop(&mut self) {
        // if not flushed database then do flush
        if !self.is_flushed {
            let contents = self.flush().expect("Failed to flush database");
            std::fs::write("kv.db", contents).unwrap();
        }
    }
}
