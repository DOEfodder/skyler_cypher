use std::io::stdin;
use std::fs::File;
use std::io::{self, Write};

//Inserts an ASCII salt into the cyphered vector.
pub fn salt_insertation(mut vector_bytes: Vec<u8>) -> Vec<u8> {
    println!("Enter the salt to be added in plaintext.");
    let mut salt = String::new();

    stdin()
        .read_line(&mut salt)
        .expect("Failed to read line");
    
    let salt = salt.trim();

    store_to_file(&salt);

    let salt_bytes: &[u8] = salt.as_bytes();
    let mut salt_bytes_vector: Vec<u8> = Vec::new();

    for &byte in salt_bytes {
        salt_bytes_vector.push(byte);
    }
    
    let middle = (vector_bytes.len() / 2) as usize;
    vector_bytes.splice(middle..middle, salt_bytes_vector);

    return vector_bytes;
}

//Stores the salt string to a .txt file.
pub fn store_to_file(salt_string: &str) -> io::Result<()> {
    let mut file = File::create("salt")?;

    file.write_all(salt_string.as_bytes())?;
    Ok(())
}

//Performs the main cyphering operations. Outputs a vector.
pub fn skyler_cypher(input_str: &str) -> Vec<u8> {

    let input_str_bytes: &[u8] = input_str.as_bytes();
    let mut bytes_vector: Vec<u8> = Vec::new();

    for &byte in input_str_bytes {
        bytes_vector.push(byte);
    }

    for element in &mut bytes_vector {
        let mut element_temp: String = element.to_string();
        element_temp = element_temp
            .chars().rev().collect::<String>();

        let parsed_temp: u32 = element_temp
            .parse().unwrap();

        *element = ( (parsed_temp % 95) + 32 ) as u8;
    }

    let bytes_vector: Vec<u8> = salt_insertation(bytes_vector.clone());

    return bytes_vector;
}

fn main() -> io::Result<()> {
    println!("Enter the plaintext to be ciphered.");
    
    let mut start_string = String::new();

    stdin()
        .read_line(&mut start_string)
        .expect("Failed to input string");

    let start_string = start_string.trim();

    println!("{:?}", skyler_cypher(&start_string));
    Ok(())
}
