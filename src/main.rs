mod hash_table;

fn main() {
    println!("Hello, world!");
}


#[test]
fn add_to_hashtable(){

    let key = String::from("testKey");
    let val = String::from("testVal");

    let mut a = hash_table::HashTable::new();
    a.add(&key, &val);
    assert_eq!(a.size,1);
}

#[test]
fn get_from_hashtable(){

    let key = String::from("testKey");
    let val = String::from("testVal");

    let mut a = hash_table::HashTable::new();
    a.add(&key, &val);
    let value = a.get(&key);
    assert_eq!(&value, &val);
}
