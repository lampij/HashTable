#[derive(Debug)]
pub struct HashTable {
	pub size: u8,
	internal_collection: Vec<String>
}

impl HashTable {
	pub fn add(&mut self, key: &String, value: &String) {
		self.internal_collection[hash_fun(&key)] = value.clone();
		//self.internal_collection.push(value.clone());
		self.size += 1;
	}

	pub fn get(&self, key: &String) -> String {
		self.internal_collection[hash_fun(&key)].clone()
	}

	pub fn new() -> HashTable {
		HashTable {size: 0, internal_collection: vec!(String::from(""); 10000)}
	}

}

pub fn hash_fun(mut key: &str) -> usize {
	let mut table_location: u64 = 0;
	let mut key_as_byte_array = key.as_bytes();
	for byte in key_as_byte_array {
		let tmp_tab_loc = table_location + u64::from(*byte);
		table_location = tmp_tab_loc.clone();
	}

	table_location as usize
}