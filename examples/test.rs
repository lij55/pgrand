use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;



fn my_hash<T>(obj: T) -> u128
    where
        T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish() as u128
}

fn main() {
    println!("{}", my_hash("abcdefg"));
}