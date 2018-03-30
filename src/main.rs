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

#[test]
fn multi_add(){
	let mut test_hash = hash_table::HashTable::new();

	test_hash.add(&String::from("1"), &String::from("Bananas"));
	test_hash.add(&String::from("2"), &String::from("Apples"));
	test_hash.add(&String::from("3"), &String::from("Oranges"));
	test_hash.add(&String::from("4"), &String::from("Grapes"));

	assert_eq!(test_hash.get(&String::from("1")), "Bananas");
	assert_eq!(test_hash.get(&String::from("2")), "Apples");
	assert_eq!(test_hash.get(&String::from("3")), "Oranges");
	assert_eq!(test_hash.get(&String::from("4")), "Grapes");
}

#[test]
fn collision_testing() {
    let a = hash_table::hash_fun("test");
    let b = hash_table::hash_fun("rest");
    let c = hash_table::hash_fun("chortle");
    let d = hash_table::hash_fun("orange");
    let e = hash_table::hash_fun("poplar");

    //Probably the ugliest test ever
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_ne!(a, d);
    assert_ne!(a, e);

    assert_ne!(b, c);
    assert_ne!(b, d);
    assert_ne!(b, e);

    assert_ne!(c, d);
    assert_ne!(c, e);

    assert_ne!(d, e);


}