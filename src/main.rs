mod store;

use std::collections::HashMap;

use store::DiveStore;

fn main() {
    let mut store: HashMap<String, String> = HashMap::new();

    let a: Vec<i32> = vec![];

    let value = store.get("divine");

    println!("vec len: {}", a.len());

    let mut store2 = DiveStore::load_store("hello.csv".to_owned());

    store2
        .as_mut()
        .unwrap()
        .set("1".to_owned(), "Divine".to_owned());
    store2
        .as_mut()
        .unwrap()
        .set("6".to_owned(), "Yin".to_owned());
    store2
        .as_mut()
        .unwrap()
        .set("7".to_owned(), "Yang".to_owned());

    let val1 = store2.as_mut().unwrap().get("7").unwrap();
    let val2 = store2.as_mut().unwrap().get("7").unwrap();
    let val3 = store2.as_mut().unwrap().get("6").unwrap();
    // let val4 = store2.as_mut().unwrap().get("6").unwrap();

    println!("Recoil map:\n {:?}", store2.as_ref().unwrap().recoil);
    store2.as_mut().unwrap().lfu_evict();
    println!("KVstore map:\n {:?}", store2.as_ref().unwrap().kv_store);
    println!("Recoil map:\n {:?}", store2.as_ref().unwrap().recoil);

    let _ = store2.unwrap().persist("hello.csv".to_owned());

    // match store2 {
    //     Ok(mut store) => {
    //         println!(
    //             "first value is for store2: {}",
    //             store
    //                 .get("1")
    //                 .unwrap_or(&"Couldn't find key in store".to_string())
    //         );

    //         store.set("5".to_owned(), "Yin".to_owned());
    //         store.set("6".to_owned(), "Yang".to_owned());
    //         store.set("7".to_owned(), "Me".to_owned());

    //         println!(
    //             "first and sixth values for store2 is: {}, {}",
    //             store
    //                 .get("1")
    //                 .unwrap_or(&"Couldn't find key in store".to_string()),
    //             store
    //                 .get("6")
    //                 .unwrap_or(&"Couldn't find key in store".to_string())
    //         );

    //         let _ = store.persist("hello.csv".to_owned());
    //         // let mut store = DiveStore::new();
    //         // let _ = store.persist("hello.csv".to_owned());
    //     }
    //     Err(e) => eprint!("An error occurred while attempting to load store: {} ", e),
    // }
}
