use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Read, Write},
};

struct DiveStore {
    kv_store: HashMap<String, String>,
}

impl DiveStore {
    fn new() -> Self {
        DiveStore {
            kv_store: HashMap::new(),
        }
    }

    fn get(&self, key: String) -> Option<&String> {
        self.kv_store.get(&key)
    }

    fn set(&mut self, key: String, value: String) -> Option<String> {
        self.kv_store.insert(key, value)
    }

    fn persist(&mut self, filename: String) {
        let file = File::create(&filename).unwrap_or_else(|err| {
            panic!("Failed to create or open file '{}': {}", filename, err);
        });

        let mut data_file = BufWriter::new(file); //creates a file write stream

        self.kv_store.iter().for_each(|(k, v)| {
            let value = format!("{}, {}\n", k, v);
            data_file.write_all(value.as_bytes()).expect("Write failed");
        });

        data_file.flush().expect("Failed to flush buffer"); //ensures all burfered content reaches their destination
    }

    // read from file and create new HashMap
    fn load_store(filename: String) -> Self {
        let mut kv_store = DiveStore {
            kv_store: HashMap::new(),
        };
        let mut data_file = File::open(filename).unwrap();
        let mut file_string = String::new();

        let _ = data_file.read_to_string(&mut file_string);

        file_string.lines().for_each(|line| {
            let data_vec: Vec<&str> = line.split(",").collect();
            kv_store.set(data_vec[0].to_owned(), data_vec[1].to_owned());
        });

        return kv_store;
    }

    fn reset_inmem(&mut self) {
        self.kv_store.clear();
    }
}

fn main() {
    // let mut data_file = File::open("hello.csv").unwrap_or_else(|err| {
    //     println!("Failed to open file: {}", err);
    //     println!("Creating new file with name: 'hello.csv'");
    //     File::create("hello.csv").unwrap()
    // });
    // let mut file_string = String::new();

    // let mut store = DiveStore::new();
    // store.set("1".to_owned(), "Divine".to_owned());
    // store.set("2".to_owned(), "Cedar".to_owned());
    // store.set("3".to_owned(), "Grace".to_owned());
    // store.set("4".to_owned(), "Levi".to_owned());

    // println!("first value is: {}", store.get("1".to_owned()).unwrap());

    // store.persist("hello.csv".to_owned());

    let mut store2 = DiveStore::load_store("hello.csv".to_owned());

    println!(
        "first value is for store2: {}",
        store2.get("1".to_owned()).unwrap()
    );

    store2.set("5".to_owned(), "Yin".to_owned());
    store2.set("6".to_owned(), "Yang".to_owned());

    println!(
        "first and sixth values for store2 is: {}, {}",
        store2.get("1".to_owned()).unwrap(),
        store2.get("6".to_owned()).unwrap(),
    );

    store2.persist("hello.csv".to_owned());
}
