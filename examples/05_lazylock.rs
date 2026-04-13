use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

static _MY_LIST: Vec<i32> = Vec::new();

// static _MY_MAP: HashMap<i32, i32> = HashMap::new();

static MY_MAP: LazyLock<HashMap<i32, i32>> = LazyLock::new(|| {
    let mut my_map = HashMap::new();
    my_map.insert(42, 99);
    my_map
});

static MY_MAP2: LazyLock<Mutex<HashMap<i32, i32>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

fn main() {
    assert_eq!(MY_MAP.get(&42), Some(&99));
    // MY_MAP.insert(1, 2);
    MY_MAP2.lock().unwrap().insert(1, 2);
}
