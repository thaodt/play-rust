use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct A {
    x: u64,
    y: String,
}
fn main() {
    let a = A {
        x: 3301,
        y: "liber primus".to_string(),
    };
    let encoded_a = a.try_to_vec().unwrap();
    let decoded_a = A::try_from_slice(&encoded_a).unwrap();
    println!("{:#?}", a);
    println!("{:#?}", decoded_a);
}


#[test]
fn test_simple_struct() {
    let a = A {
        x: 3301,
        y: "liber primus".to_string(),
    };
    let encoded_a = a.try_to_vec().unwrap();
    let decoded_a = A::try_from_slice(&encoded_a).unwrap();
    println!("{:#?}", encoded_a);
    println!("{:#?}", decoded_a);
    assert_eq!(a, decoded_a);
}