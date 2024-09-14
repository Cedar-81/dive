use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Value {
    value: String,
    freq: i32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Record {
    key: String,
    value: String,
    freq: i32,
}

#[derive(Debug)]
pub struct Recoil {
    pub freq_store: HashMap<i32, Vec<String>>,
}

#[derive(Debug)]
pub struct DiveStore {
    pub kv_store: HashMap<String, Value>,
    //include a recoil here
    pub recoil: Recoil,
}

impl Recoil {
    pub fn new() -> Self {
        Recoil {
            freq_store: HashMap::new(),
        }
    }

    pub fn init_insert(&mut self, value: &mut Value, key: String) {
        let freq_val = self.freq_store.get_mut(&value.freq);

        match freq_val {
            Some(val) => {
                val.push(key.clone());
            }
            None => {
                //create key value pair for new freq
                let mut some_val: Vec<String> = vec![];
                some_val.push(key);
                self.freq_store.insert(value.freq, some_val);
            }
        }
    }

    pub fn check_inc_insert(&mut self, value: &mut Value, key: String) {
        let freq_val = self.freq_store.get_mut(&value.freq); //get freq to retrieve key

        match freq_val {
            Some(val) => {
                val.retain(|x| x != &key); //remove the key from the arr
                if (val.len() == 0) {
                    // remove freq if arr is empty
                    self.freq_store.remove(&value.freq);
                }

                let new_freq = value.freq.clone() + 1;
                let existing_freq_val = self.freq_store.get_mut(&new_freq);

                match existing_freq_val {
                    Some(val) => {
                        val.push(key.clone()); // add if new freq exits
                    }
                    None => {
                        //create key value pair for new freq
                        let mut some_val: Vec<String> = vec![];
                        some_val.push(key);
                        self.freq_store.insert(new_freq, some_val);
                    }
                }

                value.freq = new_freq; //modify the Values freq to the updated freq
            }
            None => {
                let mut some_val: Vec<String> = vec![];
                some_val.push(key);
                self.freq_store.insert(value.freq + 1, some_val);
                value.freq += 1; //modify the Values freq to the updated freq
            }
        }
    }
}

impl DiveStore {
    pub fn new() -> Self {
        DiveStore {
            kv_store: HashMap::new(),
            recoil: Recoil::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<&String> {
        if let Some(store_val) = self.kv_store.get_mut(key) {
            self.recoil.check_inc_insert(store_val, key.to_string());
            let value = &store_val.value;
            return Some(value);
        }

        eprintln!("Key does not exist in store");
        None
    }

    //used for initial key value set where there is no previous freq
    //ideally used when inserting new data to kv_store
    pub fn set(&mut self, key: String, value: String) {
        let store_val = Value { value, freq: 0 };
        self.kv_store.insert(key.clone(), store_val);

        //set initial freq for new value
        let s_val = self.kv_store.get_mut(&key);
        if let Some(val) = s_val {
            self.recoil.init_insert(val, key);
        }

        println!(
            "Key-Value inserted successfully, {:?}",
            self.recoil.freq_store
        );
    }

    //used to load stored value from storage to memory
    pub fn set_with_store_val(&mut self, key: String, value: Value) {
        self.kv_store.insert(key.clone(), value);

        //set initial freq for new value
        let s_val = self.kv_store.get_mut(&key);
        if let Some(val) = s_val {
            self.recoil.init_insert(val, key);
        }

        println!("Key-Value inserted successfully");
    }

    pub fn persist(&mut self, filename: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;

        self.kv_store.iter().for_each(|(k, v)| {
            let _ = wtr.serialize(Record {
                key: k.to_string(),
                value: v.value.clone(),
                freq: v.freq,
            });
        });

        let _ = wtr.flush();

        Ok(())
    }

    // read from file and create new HashMap
    pub fn load_store(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut kv_store = DiveStore {
            kv_store: HashMap::new(),
            recoil: Recoil::new(),
        };
        let mut rdr = csv::Reader::from_path(filename).expect("Couldn't load file.");

        rdr.deserialize().for_each(|value| {
            let record: Record = value.expect("File data cannot be deserialized to Record.");
            kv_store.set_with_store_val(
                record.key,
                Value {
                    value: record.value,
                    freq: record.freq,
                },
            );
        });

        return Ok(kv_store);
    }

    pub fn lfu_evict(&mut self) {
        let mut freqs: Vec<i32> = self.recoil.freq_store.keys().copied().collect(); //retrieve all freq in freq_store to a freq arr
        freqs.sort(); //sort freq arr in asc order

        let keys = self.recoil.freq_store.get(&freqs[0]); //retrieve least frequent keys arr

        if let Some(keys) = keys {
            //remove least freq keys from kv_store map
            keys.iter().for_each(|key| {
                self.kv_store.remove(key);
            });

            //remove least freq from freq_store
            self.recoil.freq_store.remove(&freqs[0]);

            println!("LFU eviction successful");
        } else {
            eprintln!(
                "LFU eviction unsuccessful due to inability to retrieve least frequently used key"
            );
        }
    }

    pub fn reset_inmem(&mut self) {
        self.kv_store.clear();
    }
}
