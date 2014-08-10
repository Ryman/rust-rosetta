// Implements http://rosettacode.org/wiki/JSON

extern crate serialize;

use serialize::json;

#[deriving(Show, Encodable, Decodable, PartialEq, Eq)]
pub struct Contact {
    name: String,
    city: String
}

#[cfg(not(test))]
fn main() {
    // Encode contact to json
    let c = Contact { name: "John".to_string(), city: "Paris".to_string() };
    let json = json::encode(&c);
    println!("Encoded: {}", json.as_slice());

    // Decode json to contact
    let json_str = r#"{
        "name": "Alan",
        "city": "Tokyo"
    }"#;
    let contact = json::decode::<Contact>(json_str).unwrap();
    println!("Decoded: {}", contact);
}

#[test]
fn test_coherence() {
    use serialize::json::{decode, encode};
    let c = Contact { name: "John".to_string(), city: "Paris".to_string() };
    assert_eq!(decode::<Contact>(encode(&c).as_slice()).unwrap(), c);
}

#[test]
fn test_decode() {
    let json_str = r#"{
        "name": "Alan",
        "city": "Tokyo"
    }"#;

    let contact = json::decode::<Contact>(json_str).unwrap();
    assert_eq!(contact, Contact { name: "Alan".to_string(), city: "Tokyo".to_string() });
}
