use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Read, Write},
};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct Record {
    key: String,
    value: String,
}

struct DiveStore {
    kv_store: HashMap<String, String>,
}

impl DiveStore {
    fn new() -> Self {
        DiveStore {
            kv_store: HashMap::new(),
        }
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.kv_store.get(key)
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.kv_store.insert(key, value)
    }

    fn persist(&mut self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;

        self.kv_store.iter().for_each(|(k, v)| {
            let _ = wtr.serialize(Record {
                key: k.to_string(),
                value: v.to_string(),
            });
        });

        let _ = wtr.flush();

        Ok(())
    }

    // read from file and create new HashMap
    fn load_store(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut kv_store = DiveStore {
            kv_store: HashMap::new(),
        };
        let mut rdr = csv::Reader::from_path(filename).expect("Couldn't load file.");

        rdr.deserialize().for_each(|value| {
            let record: Record = value.expect("File data cannot be deserialized to Record.");
            kv_store.set(record.key, record.value);
        });

        return Ok(kv_store);
    }

    fn reset_inmem(&mut self) {
        self.kv_store.clear();
    }
}

fn main() {
    let mut store2 = DiveStore::load_store("hello.csv".to_owned());

    match store2 {
        Ok(mut store) => {
            println!(
                "first value is for store2: {}",
                store
                    .get("1")
                    .unwrap_or(&"Couldn't find key in store".to_string())
            );

            store.set("5".to_owned(), "Yin".to_owned());
            store.set("6".to_owned(), "Yang".to_owned());
            store.set("7".to_owned(), "Me".to_owned());

            println!(
                "first and sixth values for store2 is: {}, {}",
                store
                    .get("1")
                    .unwrap_or(&"Couldn't find key in store".to_string()),
                store
                    .get("6")
                    .unwrap_or(&"Couldn't find key in store".to_string())
            );

            let _ = store.persist("hello.csv".to_owned());
            // let mut store = DiveStore::new();
            // let _ = store.persist("hello.csv".to_owned());
        }
        Err(e) => eprint!("An error occurred while attempting to load store: {} ", e),
    }
}
