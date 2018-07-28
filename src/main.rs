extern crate bincode;

use bincode::{serialize, deserialize};

fn main() {
    for i in 0..(1<<30) {
        let vec = vec![0u8; i];
        assert_eq!(vec.len(), vec.capacity());
        let serialized = serialize(&vec).unwrap();
        let deserialized: Vec<u8> = deserialize(&serialized).unwrap();
        if deserialized.len() != deserialized.capacity() {
            println!("{} {}", deserialized.len(), deserialized.capacity());
        }
    }
}
